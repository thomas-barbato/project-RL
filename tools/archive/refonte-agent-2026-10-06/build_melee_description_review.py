"""Calculate proposed melee descriptions, preserving existing approvals."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
IDS = ["B01", "ME02", "ME03", "ME04", "ME05", "ME06"]
DOCUMENT = "docs/REVUE_MELEE_AGENT_LOT_03.md"


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def digest(source):
    return hashlib.sha256(source.encode("utf-8")).hexdigest()


def load(name):
    return json.loads((OUT / name).read_text(encoding="utf-8"))


def save(name, payload):
    target = OUT / name
    assert not target.exists(), f"Preserve prior review: {name}"
    target.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def effects(identifier, rank):
    assert rank >= 1
    step = rank - 1
    if identifier == "B01":
        return {"damage_percent": 140 + 10 * step, "extra_damage_percent": 40 + 10 * step,
                "energy": 6 + step // 3, "cooldown_turns": 3, "target_limit": 1}
    if identifier == "ME02":
        return {"damage_percent": 80 + 5 * step, "energy": 8 + step // 4,
                "cooldown_turns": 3, "adjacent_cells": 3, "target_limit": 3}
    if identifier == "ME03":
        return {"damage_percent": 100 + 8 * step, "energy": 8 + step // 3,
                "cooldown_turns": 4, "armor_ignored_percent": Fraction(20) + Fraction(60 * step, rank + 11),
                "target_limit": 1}
    if identifier == "ME04":
        return {"damage_percent": 90 + 6 * step, "energy": 7 + step // 4,
                "cooldown_turns": 3, "push_max_cells": min(3, 1 + step // 5), "target_limit": 1}
    if identifier == "ME05":
        return {"damage_percent": 110 + 8 * step, "energy": 10 + step // 3,
                "cooldown_turns": 5, "target_max_distance": min(6, 3 + step // 4),
                "target_min_distance": 2, "target_limit": 1}
    if identifier == "ME06":
        return {"damage_percent": 60 + 5 * step, "energy": 12 + step // 3,
                "cooldown_turns": 6, "adjacent_cells": 8, "target_limit": 8}
    raise ValueError(identifier)


def serialize_effects(values):
    return {key: {"numerator": value.numerator, "denominator": value.denominator,
                  "display": round(float(value), 1)} if isinstance(value, Fraction) else value
            for key, value in values.items()}


def main():
    source = read(DOCUMENT)
    catalogue = load("review-primary-catalogue-02.json")
    catalog_rows = {row["id"]: row for branch in catalogue["branches"] for row in branch["candidates"]}
    prior_lot = load("review-lot-01-revision-04.json")
    old_b01 = next(row for row in prior_lot["blocks"] if row["id"] == "B01")
    default_modifier = Fraction(1)
    current_modifier = Fraction(11, 10) * Fraction(41, 40)
    assert current_modifier == Fraction(451, 400)
    base_weapon_damage = 20
    expected_default = [28, 16, 20, 18, 22, 12]
    expected_current = [56, 30, 42, 35, 44, 25]
    rows = []
    for index, identifier in enumerate(IDS):
        previous = None
        samples = []
        for rank in [1, 5, 10, 12, 100, 1000]:
            values = effects(identifier, rank)
            if previous:
                assert values["damage_percent"] > previous["damage_percent"]
                assert values["energy"] >= previous["energy"]
            assert values["energy"] > 0 and values["cooldown_turns"] > 0
            if identifier == "ME03":
                assert 20 <= values["armor_ignored_percent"] < 80
            if identifier == "ME04":
                assert 1 <= values["push_max_cells"] <= 3
            if identifier == "ME05":
                assert 3 <= values["target_max_distance"] <= 6
            previous = values
            samples.append({"effective_rank": rank, **serialize_effects(values)})
        default = effects(identifier, 1)
        current = effects(identifier, 12)
        default_damage = base_weapon_damage * default_modifier * Fraction(default["damage_percent"], 100)
        current_damage = base_weapon_damage * current_modifier * Fraction(current["damage_percent"], 100)
        default_integer = default_damage.numerator // default_damage.denominator
        current_integer = current_damage.numerator // current_damage.denominator
        assert default_integer == expected_default[index]
        assert current_integer == expected_current[index]
        rows.append({"id": identifier, "samples": samples,
                     "default_damage_before_defenses": default_integer,
                     "current_example_damage_before_defenses": current_integer})
    for rank in [1, 10, 12]:
        assert effects("B01", rank)["extra_damage_percent"] == old_b01["accepted_visible_effect_examples"][str(rank)]["extra_damage_percent"]

    examples_name = "melee-description-examples.json"
    save(examples_name, {"date": "2026-10-06", "status": "proposed_design_calculations_not_engine_tests",
         "default_reference": {"effective_rank": 1, "force": 5, "agility": 5},
         "current_example": {"learned_rank": 10, "equipment_rank_bonus": 2, "effective_rank": 12,
             "force": 10, "agility": 10, "damage_attribute_multiplier": 1.1275,
             "attribute_coefficients_approval": "pending_user_review", "actual_user_save": False},
         "both_profiles": {"weapon_damage": 20, "weapon_range": 1, "other_cost_or_damage_modifiers": False},
         "rows": rows})

    matches = list(re.finditer(r"^### ([A-Z]+\d{2}) (.+)$", source, re.M))
    assert [match.group(1) for match in matches] == IDS
    records = []
    for index, match in enumerate(matches):
        identifier, title = match.groups()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        body = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.M)[0].strip()
        text = "\n".join(line for line in body.splitlines() if line.startswith(">"))
        mechanic = "\n".join(line for line in body.splitlines() if not line.startswith(">"))
        assert text and catalog_rows[identifier]["name_approval"] == "accepted_as_presented"
        row = {"id": identifier, "title": title, "revision": old_b01["revision"] + 1 if identifier == "B01" else 1,
               "block_sha256": digest(body), "text_sha256": digest(text), "mechanic_sha256": digest(mechanic),
               "name_approval": "accepted_as_presented", "catalogue_effect_approval": "accepted_as_presented",
               "text_approval": "pending_individual_user_review", "numeric_rules_approval": "proposed_pending_user_review",
               "visual_intention_approval": "accepted_as_presented", "visual_approval": "pending_native_preview_and_user_review",
               "runtime_integration": "not_integrated"}
        if identifier == "B01":
            row["accepted_visible_effect_examples"] = deepcopy(old_b01["accepted_visible_effect_examples"])
            row["numeric_rules_approval"] = "prior_damage_examples_preserved_other_details_pending_user_review"
            row["previous_block_manifest"] = "artifacts/refonte-agent-2026-10-06/review-lot-01-revision-04.json"
        records.append(row)
    save("review-melee-01.json", {"date": "2026-10-06", "batch_revision": 1, "document": DOCUMENT,
         "document_sha256": digest(source), "blocks": records,
         "accepted_catalogue_reference": "artifacts/refonte-agent-2026-10-06/review-primary-catalogue-02.json",
         "calculation_examples": f"artifacts/refonte-agent-2026-10-06/{examples_name}",
         "calculation_examples_sha256": digest(read(f"artifacts/refonte-agent-2026-10-06/{examples_name}")),
         "default_reference_approval": "working_rank_one_assumption_not_explicitly_confirmed",
         "current_profile_status": "illustrative_not_a_user_save", "runtime_integration": "not_integrated"})

    format_old = load("review-description-format-01.json")
    snapshot_path = "artifacts/refonte-agent-2026-10-06/review-description-format-revision-01.md"
    assert digest(read(snapshot_path)) == format_old["document_sha256"]
    format_new = deepcopy(format_old)
    format_new.update(batch_revision=2, document_sha256=digest(read(format_new["document"])),
                      previous_manifest="artifacts/refonte-agent-2026-10-06/review-description-format-01.json",
                      previous_document_snapshot=snapshot_path,
                      first_melee_review=DOCUMENT, default_reference_working_assumption="rank_one_for_this_review")
    format_source = read(format_new["document"])
    format_matches = list(re.finditer(r"^### (CD\d{2}) (.+)$", format_source, re.M))
    for index, (match, row) in enumerate(zip(format_matches, format_new["blocks"])):
        end = format_matches[index + 1].start() if index + 1 < len(format_matches) else len(format_source)
        body = format_source[match.end():end].strip()
        assert match.group(1) == row["id"]
        if digest(body) != row["block_sha256"]:
            assert row["id"] == "CD03"
            row.update(revision=row["revision"] + 1, block_sha256=digest(body))
    save("review-description-format-02.json", format_new)
    print(json.dumps({"skills": len(records), "ranks_checked_per_skill": 6,
                      "default_damage": expected_default, "current_example_damage": expected_current,
                      "B01_accepted_damage_examples_preserved": True}))


if __name__ == "__main__":
    main()
