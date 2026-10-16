"""Record melee approval and calculate the proposed survival review."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
IDS = ["B03", "B02", "SU03", "SU04", "SU05", "SU06"]


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def digest(source):
    return hashlib.sha256(source.encode("utf-8")).hexdigest()


def load(name):
    return json.loads((OUT / name).read_text(encoding="utf-8"))


def save(name, payload):
    target = OUT / name
    assert not target.exists(), name
    target.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def ceil(value):
    value = Fraction(value)
    return -(-value.numerator // value.denominator)


def floor(value):
    value = Fraction(value)
    return value.numerator // value.denominator


def effects(identifier, rank, max_hp):
    assert rank >= 1 and max_hp > 0
    step = rank - 1
    if identifier == "B03":
        percent = 10 + 2 * step
        return {"maximum_hp_bonus_percent": percent, "maximum_hp_with_base_100": floor(100 * (1 + Fraction(percent, 100))), "energy": 0}
    if identifier == "B02":
        return {"direct_reduction_percent": 25 + Fraction(50 * step, rank + 9), "energy": 10 + step // 3,
                "duration_turns": 4, "cooldown_turns": 8}
    if identifier == "SU03":
        percent = 5 + Fraction(25 * step, rank + 19)
        return {"absorption_percent_max_hp": percent, "absorbed_hp": ceil(max_hp * percent / 100),
                "calm_turns": 4, "energy": 0}
    if identifier == "SU04":
        percent = 4 + Fraction(16 * step, rank + 9)
        healing = ceil(max_hp * percent / 100)
        energy = 4 + step // 4
        return {"heal_percent_max_hp_per_turn": percent, "healed_hp_per_turn": healing,
                "maximum_channel_turns": 3, "max_total_healed_hp": healing * 3,
                "energy_per_turn": energy, "max_total_energy": energy * 3, "cooldown_turns": 8}
    if identifier == "SU05":
        return {"maximum_cleansed_statuses": 1 + step // 5,
                "energy": ceil(10 * (1 - Fraction(2 * step, 5 * (rank + 9)))), "cooldown_turns": 6}
    if identifier == "SU06":
        return {"surviving_hp": 1, "cooldown_turns": ceil(30 + Fraction(700, rank + 9)), "energy": 0}
    raise ValueError(identifier)


def serialize(values):
    return {key: {"numerator": value.numerator, "denominator": value.denominator,
                  "display": round(float(value), 1)} if isinstance(value, Fraction) else value
            for key, value in values.items()}


def sections(source):
    matches = list(re.finditer(r"^### ([A-Z]+\d{2}) (.+)$", source, re.M))
    rows = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        body = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.M)[0].strip()
        text = "\n".join(line for line in body.splitlines() if line.startswith(">"))
        mechanic = "\n".join(line for line in body.splitlines() if not line.startswith(">"))
        rows.append({"id": match.group(1), "title": match.group(2), "block_sha256": digest(body),
                     "text_sha256": digest(text), "mechanic_sha256": digest(mechanic)})
    return rows


def main():
    decisions_name = "melee-acceptance-survival-and-test-decisions.json"
    decisions = {
        "date": "2026-10-06", "runtime_integration": "not_integrated",
        "user_melee_reply": "ok oui tu peux continuer ; nous ferons les équilibrages petit à petit plus tard",
        "accepted_melee_scope": "six roleplay texts, tables displayed at rank 1 and 12, and conditions stated in the response; accepted provisionally for progressive balancing",
        "accepted_melee_ids": ["B01", "ME02", "ME03", "ME04", "ME05", "ME06"],
        "hidden_document_details": "unshown formulas and additional conditions remain proposed",
        "default_reference": "rank 1 before other bonuses, as displayed in the accepted first review",
        "balancing_timing": "progressively later; revisions still reviewed under stable identifiers",
        "user_test_menu_reply": "2 mais on s'en occupera quand on aura implementer toutes les compétences et toutes les animations",
        "test_character": {"configuration": "menu", "repeatable_respecialization": True,
            "level_gains_at_will": True, "location": "test room", "implementation": "explicitly_deferred_until_all_skills_and_animations_implemented"},
        "survival_review_status": "new_roleplay_texts_numeric_parameters_and_detailed_conditions_pending_user_review"
    }
    save(decisions_name, decisions)
    decision_path = f"artifacts/refonte-agent-2026-10-06/{decisions_name}"
    common = {"date": "2026-10-06", "latest_decisions": decision_path,
              "latest_decisions_sha256": digest(read(decision_path)), "runtime_integration": "not_integrated"}

    melee_old = load("review-melee-01.json")
    melee_snapshot = "artifacts/refonte-agent-2026-10-06/review-melee-revision-01.md"
    assert digest(read(melee_snapshot)) == melee_old["document_sha256"]
    melee_source = read(melee_old["document"])
    assert [line for line in melee_source.splitlines() if line.startswith("|")] == [line for line in read(melee_snapshot).splitlines() if line.startswith("|")]
    melee = deepcopy(melee_old)
    hashes = {row["id"]: row for row in sections(melee_source)}
    for row in melee["blocks"]:
        current = hashes[row["id"]]
        assert current["text_sha256"] == row["text_sha256"]
        if current["block_sha256"] != row["block_sha256"]:
            assert row["id"] == "B01"
            row["revision"] += 1
        row.update(current, text_approval="accepted_as_presented", numeric_rules_approval="displayed_tables_accepted_provisional_balance_unshown_formulas_pending",
                   presented_conditions_approval="accepted_as_presented", detailed_conditions_approval="additional_unshown_details_pending")
    melee.update(common, batch_revision=2, document_sha256=digest(melee_source),
                 previous_manifest="artifacts/refonte-agent-2026-10-06/review-melee-01.json",
                 previous_document_snapshot=melee_snapshot, default_reference_approval="accepted_as_displayed_reference")
    save("review-melee-02.json", melee)

    sample_rows = []
    for identifier in IDS:
        samples = []
        previous = None
        for rank in [1, 5, 10, 12, 100, 1000]:
            values = effects(identifier, rank, 100)
            if identifier == "B02":
                assert 25 <= values["direct_reduction_percent"] < 75
            if identifier == "SU03":
                assert 5 <= values["absorption_percent_max_hp"] < 30
            if identifier == "SU04":
                assert 4 <= values["heal_percent_max_hp_per_turn"] < 20
            if identifier == "SU05":
                assert values["energy"] > 0 and values["maximum_cleansed_statuses"] >= 1
                if previous:
                    assert values["energy"] <= previous["energy"] and values["maximum_cleansed_statuses"] >= previous["maximum_cleansed_statuses"]
            if identifier == "SU06":
                assert values["cooldown_turns"] > 30 and values["surviving_hp"] == 1
                if previous:
                    assert values["cooldown_turns"] <= previous["cooldown_turns"]
            previous = values
            samples.append({"effective_rank": rank, **serialize(values)})
        sample_rows.append({"id": identifier, "rank_samples_with_100_max_hp": samples,
                           "default": serialize(effects(identifier, 1, 100)),
                           "current_example": serialize(effects(identifier, 12, 132))})
    assert effects("B03", 12, 100)["maximum_hp_with_base_100"] == 132
    assert effects("SU03", 12, 132)["absorbed_hp"] == 19
    assert effects("SU04", 12, 132)["healed_hp_per_turn"] == 17
    assert effects("SU04", 12, 132)["max_total_healed_hp"] == 51
    assert effects("SU05", 12, 132)["energy"] == 8
    assert effects("SU06", 12, 132)["cooldown_turns"] == 64
    examples_name = "survival-description-examples.json"
    save(examples_name, {"date": "2026-10-06", "status": "proposed_design_calculations_not_engine_tests",
         "default_max_hp": 100, "current_max_hp_before_Endurance": 100, "current_max_hp": 132,
         "current_learned_rank": 10, "current_equipment_rank_bonus": 2, "current_effective_rank": 12,
         "actual_user_save": False, "rows": sample_rows})

    survival_document = "docs/REVUE_SURVIE_AGENT_LOT_04.md"
    prior_first_lot = {row["id"]: row for row in load("review-lot-01-revision-04.json")["blocks"]}
    survival_rows = sections(read(survival_document))
    assert [row["id"] for row in survival_rows] == IDS
    for row in survival_rows:
        old = prior_first_lot.get(row["id"])
        row.update(revision=old["revision"] + 1 if old else 1,
                   name_approval="accepted_as_presented", catalogue_effect_approval="accepted_as_presented",
                   text_approval="pending_individual_user_review", numeric_rules_approval="new_details_pending_user_review",
                   visual_intention_approval="accepted_as_presented", visual_approval="pending_native_preview_and_user_review",
                   runtime_integration="not_integrated")
        if old:
            row["accepted_visible_effect_examples"] = deepcopy(old["accepted_visible_effect_examples"])
            row["previous_block_manifest"] = "artifacts/refonte-agent-2026-10-06/review-lot-01-revision-04.json"
            row["numeric_rules_approval"] = "previous_visible_examples_preserved_new_details_pending"
    save("review-survival-01.json", {**common, "batch_revision": 1, "document": survival_document,
         "document_sha256": digest(read(survival_document)), "blocks": survival_rows,
         "calculation_examples": f"artifacts/refonte-agent-2026-10-06/{examples_name}",
         "calculation_examples_sha256": digest(read(f"artifacts/refonte-agent-2026-10-06/{examples_name}"))})

    format_old = load("review-description-format-02.json")
    format_snapshot = "artifacts/refonte-agent-2026-10-06/review-description-format-revision-02.md"
    assert digest(read(format_snapshot)) == format_old["document_sha256"]
    format_source = read(format_old["document"])
    format_new = deepcopy(format_old)
    matches = list(re.finditer(r"^### (CD\d{2}) (.+)$", format_source, re.M))
    for index, (match, row) in enumerate(zip(matches, format_new["blocks"])):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(format_source)
        body = format_source[match.end():end].strip()
        if digest(body) != row["block_sha256"]:
            assert row["id"] in ("CD02", "CD03")
            row.update(revision=row["revision"] + 1, block_sha256=digest(body))
    format_new.update(common, batch_revision=3, document_sha256=digest(format_source),
                      previous_manifest="artifacts/refonte-agent-2026-10-06/review-description-format-02.json",
                      previous_document_snapshot=format_snapshot, survival_review=survival_document)
    format_new["accepted_requirements"]["default_reference_definition"] = decisions["default_reference"]
    format_new.pop("default_reference_working_assumption", None)
    save("review-description-format-03.json", format_new)

    test_document = "docs/REVUE_PERSONNAGE_TEST_AGENT.md"
    test_source = read(test_document)
    test_matches = list(re.finditer(r"^### (TEST\d{2}) (.+)$", test_source, re.M))
    test_rows = []
    for index, match in enumerate(test_matches):
        end = test_matches[index + 1].start() if index + 1 < len(test_matches) else len(test_source)
        test_rows.append({"id": match.group(1), "title": match.group(2), "revision": 1,
                          "block_sha256": digest(test_source[match.end():end].strip()),
                          "requirement_approval": "explicit_user_requirement_recorded_details_pending"})
    assert len(test_rows) == 3
    save("review-test-character-01.json", {**common, "batch_revision": 1, "document": test_document,
         "document_sha256": digest(test_source), "blocks": test_rows,
         "accepted_requirements": decisions["test_character"], "implementation_status": "explicitly_deferred_by_user"})
    print(json.dumps({"accepted_melee_texts_and_displayed_tables": 6, "survival_review_skills": 6,
                      "survival_rank_samples_checked": 36, "test_menu": "deferred_after_all_skills_and_animations"}))


if __name__ == "__main__":
    main()
