"""Record review changes and calculate proposed examples, without gameplay integration."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json
import math
import re

ROOT = Path(__file__).resolve().parents[2]
DIRECTORY = Path(__file__).resolve().parent


def digest(source):
    return hashlib.sha256(source.encode("utf-8")).hexdigest()


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def write_new(path, payload):
    assert not path.exists(), f"Preserve existing review: {path.name}"
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def extract_records(source, expected, previous=()):
    old_records = {record["id"]: record for record in previous}
    matches = list(re.finditer(r"^### ([A-Z]\d{2}) (.+)$", source, re.MULTILINE))
    assert [match.group(1) for match in matches] == expected
    records = []
    for index, match in enumerate(matches):
        identifier, title = match.group(1), match.group(2)
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        block = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.MULTILINE)[0].strip()
        copy = "\n".join(line for line in block.splitlines() if line.startswith(">"))
        assert copy, identifier
        narrative = identifier.startswith("N")
        mechanic = "\n".join(line for line in block.splitlines() if not line.startswith(">"))
        hashes = {"block_sha256": digest(block), "text_sha256": digest(copy),
                  "mechanic_sha256": None if narrative else digest(mechanic)}
        old = old_records.get(identifier)
        record = deepcopy(old) if old else {"id": identifier, "revision": 0}
        changed = not old or old["block_sha256"] != hashes["block_sha256"] or old["title"] != title
        record.update({"title": title, **hashes, "runtime_integration": "not_integrated"})
        if changed:
            record["revision"] += 1
        if not old or old["text_sha256"] != hashes["text_sha256"]:
            record["text_approval"] = "pending_user_review"
        if not old or old["mechanic_sha256"] != hashes["mechanic_sha256"]:
            record["mechanic_approval"] = "not_applicable" if narrative else "pending_user_review"
        record.setdefault("visual_approval", "pending_preview_and_user_review")
        if record.get("accepted_user_requirements"):
            record["mechanic_approval"] = "user_requirements_recorded_details_pending_review"
        records.append(record)
    return records


def clamp(value, low, high):
    return min(high, max(low, value))


def attribute_examples():
    rows = []
    for value in [5, 100, 200, 1000]:
        higher = value + 5
        rows.append({
            "attribute_before": value, "attribute_after": higher,
            "force_melee_damage_bonus_percent": [2 * (value - 5), 2 * (higher - 5)],
            "agility_physical_damage_bonus_percent": [float(Fraction(value - 5, 2)), float(Fraction(higher - 5, 2))],
            "agility_accuracy_and_evasion_modifier": [2 * (value - 5), 2 * (higher - 5)],
            "vitality_max_hp_before_percentage_bonuses_at_level_10": [30 + 4 * (value - 5) + 18, 30 + 4 * (higher - 5) + 18],
            "perception_ranged_weapon_damage_bonus_percent": [2 * (value - 5), 2 * (higher - 5)],
            "perception_vision_distance": [clamp(8 + (value - 5) // 10, 4, 16), clamp(8 + (higher - 5) // 10, 4, 16)],
            "intelligence_drone_hp_and_damage_bonus_percent": [2 * (value - 5), 2 * (higher - 5)],
            "willpower_spell_damage_bonus_percent": [2 * (value - 5), 2 * (higher - 5)],
            "independent_max_energy_gain_from_five_intelligence_or_willpower_points": 10,
        })
    return rows


def skill_examples():
    rows = []
    for n in [1, 10, 12, 100]:
        step = (n - 1) // 3
        interference = Fraction(15) + Fraction(35 * (n - 1), n + 9)
        channel = Fraction(20) + Fraction(30 * (n - 1), n + 9)
        adaptation = Fraction(5) + Fraction(15 * (n - 1), n + 9)
        assert interference < 50 and channel < 50 and adaptation < 20
        rows.append({
            "effective_level": n,
            "O01": {"base_hp": 20 + 4 * (n - 1), "base_damage": 4 + n - 1, "base_duration": 8 + step, "base_energy_cost": 12 + step},
            "O02": {"damage_reduction_percent_display": round(float(interference), 1), "base_duration": 3 + (n - 1) // 5, "base_energy_cost": 8 + step},
            "O03": {"drone_damage_bonus_percent": 25 + 2 * (n - 1)},
            "T01": {"damage_bonus_percent": 40 + 10 * (n - 1), "mark_duration": 4 + (n - 1) // 5, "base_energy_cost": 6 + step},
            "T02": {"evasion_bonus": 15 + 3 * (n - 1), "duration": 2 + (n - 1) // 5, "base_energy_cost": 8 + step},
            "T03": {"ranged_damage_bonus_percent": 10 + 2 * (n - 1)},
            "A01": {"base_damage": 8 + 3 * (n - 1), "base_energy_cost": 6 + step},
            "A02": {"base_damage": 6 + 2 * (n - 1), "base_energy_cost": 10 + step},
            "A03": {"magic_cost_reduction_percent_display": round(float(channel), 1), "A01_energy_cost_with_same_level_passive_only": math.ceil((6 + step) * (1 - channel / 100))},
            "M01": {"base_claw_damage": 8 + 2 * (n - 1), "base_duration": 6 + step, "base_energy_cost": 12 + step},
            "M02": {"base_damage": 7 + 2 * (n - 1), "base_energy_cost": 6 + step},
            "M03": {"direct_damage_reduction_percent_display": round(float(adaptation), 1)},
        })
    return rows


def main():
    outputs = [DIRECTORY / filename for filename in ["review-lot-01-revision-03.json", "review-lot-02.json", "attribute-and-skill-examples.json"]]
    assert all(not output.exists() for output in outputs), "Preserve existing review outputs."
    previous_path = DIRECTORY / "review-lot-01-revision-02.json"
    previous = read_json(previous_path)
    snapshot = DIRECTORY / "review-lot-01-revision-02.md"
    assert digest(snapshot.read_text(encoding="utf-8")) == previous["document_sha256"]
    decision_path = DIRECTORY / "attribute-usefulness-decisions.json"
    decision_source = decision_path.read_text(encoding="utf-8")
    decision = json.loads(decision_source)
    common = {"date": "2026-10-06", "runtime_integration": "not_integrated",
              "latest_decisions": decision_path.relative_to(ROOT).as_posix(),
              "latest_decisions_sha256": digest(decision_source)}
    first_path = ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md"
    first_source = first_path.read_text(encoding="utf-8")
    first_expected = ["N01", "N02", "S01", "S02", "S03", "S04", "S05", "S06", "E01", "E02", "E03", "P01", "P02", "P03", "B00", "B01", "B02", "B03"]
    first = extract_records(first_source, first_expected, previous["blocks"])
    by_id = {record["id"]: record for record in first}
    for identifier, examples in decision["accepted_visible_effect_examples"].items():
        by_id[identifier]["accepted_visible_effect_examples"] = examples
        by_id[identifier]["mechanic_approval"] = "visible_examples_accepted_remaining_details_pending_review"
    by_id["B00"]["accepted_specific_changes"] = ["move_initial_hp_bonus_to_passive"]
    by_id["B00"]["mechanic_approval"] = "hp_transfer_accepted_remaining_details_pending_review"
    by_id["P02"]["accepted_interpretations"] = {"starter_learned_level": 1, "starter_upgrades_to_cap": 9}
    by_id["P03"]["accepted_bonus_scopes"] = decision["accepted_requirements"]["skill_bonus_scopes"]
    by_id["P03"]["visual_approval"] = "cyan_color_family_accepted_exact_render_pending_preview"
    first_payload = {**common, "batch_revision": 3,
                     "document": first_path.relative_to(ROOT).as_posix(),
                     "document_sha256": digest(first_source),
                     "previous_manifest": previous_path.relative_to(ROOT).as_posix(),
                     "previous_document_snapshot": snapshot.relative_to(ROOT).as_posix(),
                     "status": "prior_visible_proposals_accepted_added_attribute_effects_pending_review",
                     "resolution_conventions_approval": "pending_user_review", "blocks": first}
    second_path = ROOT / "docs/REVUE_COMPETENCES_AGENT_LOT_02.md"
    second_source = second_path.read_text(encoding="utf-8")
    second_expected = [f"{prefix}{number:02d}" for prefix in "OTAM" for number in range(4)]
    second = extract_records(second_source, second_expected)
    second_payload = {**common, "batch_revision": 1,
                      "document": second_path.relative_to(ROOT).as_posix(),
                      "document_sha256": digest(second_source),
                      "status": "pending_user_review", "blocks": second}
    example_payload = {**common, "status": "proposal_calculations_not_runtime_tests",
                       "attribute_examples": attribute_examples(), "skill_examples": skill_examples(),
                       "combat_balance": "not_playtested"}
    for output, payload in zip(outputs, [first_payload, second_payload, example_payload]):
        write_new(output, payload)
    print(f"Recorded {len(first)} revised blocks and {len(second)} new blocks; prior records preserved.")
    print("Calculated attribute benefits beyond 10 and skill examples at 1, 10, 12 and 100; no gameplay changes.")


if __name__ == "__main__":
    main()
