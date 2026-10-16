"""Record scoped approvals and verify proposal arithmetic; does not change gameplay."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import importlib.util
import json
import re
import struct
import sys

sys.dont_write_bytecode = True
DIRECTORY = Path(__file__).resolve().parent
ROOT = DIRECTORY.parents[1]


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def output(path, value):
    assert not path.exists(), f"Preserve prior decisions: {path.name}"
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def drone_records(source):
    matches = list(re.finditer(r"^### ([A-Z]+\d{2}) (.+)$", source, re.MULTILINE))
    assert [m[1] for m in matches] == ["D56", "D57", "D58", "D59", "D66", "DP01", "DP02"]
    result = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        block = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.MULTILINE)[0].strip()
        copy = "\n".join(line for line in block.splitlines() if line.startswith(">"))
        assert copy, match[1]
        mechanic = "\n".join(line for line in block.splitlines() if not line.startswith(">"))
        result.append({"id": match[1], "title": match[2], "revision": 1,
                       "block_sha256": digest(block), "text_sha256": digest(copy),
                       "mechanic_sha256": digest(mechanic),
                       "text_approval": "pending_user_review",
                       "mechanic_approval": "requested_structure_confirmed_values_and_behaviors_pending_review",
                       "visual_approval": "existing_sprite_accepted_new_actions_and_bindings_pending_review",
                       "runtime_integration": "not_integrated"})
    return result


def rows_at(n):
    r, step = n - 1, (n - 1) // 3
    guardian = Fraction(5) + Fraction(25 * r, n + 9)
    resistance = Fraction(10) + Fraction(40 * r, n + 9)
    assert guardian < 30 and resistance < 50
    mark = Fraction(115 + 2 * r, 100)
    chance = min(Fraction(95), max(Fraction(5), 100 * (100 * mark) / (100 * mark + 25)))
    return {"effective_level": n,
            "D56": {"base_hp": 16 + 3 * r, "vision": min(8, 4 + step), "duration": 8 + step, "energy": 10 + step},
            "D57": {"base_hp": 28 + 4 * r, "capacity": 40 + 8 * r, "duration": 10 + step, "energy": 10 + step},
            "D58": {"base_hp": 22 + 4 * r, "base_damage": 5 + 2 * r, "duration": 8 + step, "energy": 12 + step},
            "D59_O01": {"base_hp": 20 + 4 * r, "base_damage": 4 + r, "duration": 8 + step, "energy": 12 + step},
            "D66": {"base_hp": 30 + 5 * r, "base_damage": 3 + r, "protection_percent": round(float(guardian), 1), "duration": 8 + step, "energy": 14 + step},
            "DP01": {"added_turns": n + 1, "artillery_duration_with_same_rank": 8 + step + n + 1},
            "DP02": {"damage_reduction_percent": round(float(resistance), 1)},
            "O03": {"drone_damage_bonus_percent": 25 + 2 * r},
            "T01": {"accuracy_bonus_percent": 15 + 2 * r, "activation_turns": 0,
                    "reference_hit_chance_percent": round(float(chance), 1),
                    "duration": 4 + r // 5, "energy": 6 + step},
            "T03": {"personal_ranged_damage_bonus_percent": 10 + 2 * r, "requires_previous_movement": False},
            "M03": {"personal_transformed_damage_bonus_percent": 10 + 2 * r, "human_form_bonus": 0}}


def verify_curves():
    previous = None
    for n in range(1, 10001):
        row = rows_at(n)
        if previous:
            for model in ["D56", "D57", "D58", "D59_O01", "D66"]:
                assert row[model]["base_hp"] > previous[model]["base_hp"]
                assert row[model]["duration"] >= previous[model]["duration"]
                assert row[model]["energy"] >= previous[model]["energy"]
                if "base_damage" in row[model]:
                    assert row[model]["base_damage"] > previous[model]["base_damage"]
            assert row["D57"]["capacity"] > previous["D57"]["capacity"]
            assert row["DP01"]["added_turns"] > previous["DP01"]["added_turns"]
        previous = row
    assert rows_at(1)["T01"]["reference_hit_chance_percent"] == 82.1
    assert 100 * Fraction(100, 125) == 80
    assert int(4 * Fraction(106, 100) * Fraction(125, 100)) == 5
    assert [rows_at(n)["DP01"]["artillery_duration_with_same_rank"] for n in [1, 10, 12]] == [10, 22, 24]


def main():
    filenames = ["class-drone-corrections.json", "review-lot-02-revision-03.json",
                 "review-drones-01.json", "drone-class-correction-examples.json"]
    assert all(not (DIRECTORY / name).exists() for name in filenames)
    old_path = DIRECTORY / "review-lot-02.json"
    old = load_json(old_path)
    snapshot = DIRECTORY / "review-lot-02-revision-02.md"
    assert digest(snapshot.read_text(encoding="utf-8")) == old["document_sha256"]
    first = load_json(DIRECTORY / "review-lot-01-revision-03.json")
    assert digest((ROOT / first["document"]).read_text(encoding="utf-8")) == first["document_sha256"]
    bank_path = ROOT / "assets/sprites/bestiary/manifest.json"
    entries = {e["number"]: e for e in load_json(bank_path)["entries"]}
    bindings = []
    for number in [56, 57, 58, 59, 66]:
        entry = entries[number]
        png = (bank_path.parent / entry["living"]).read_bytes()
        assert png[:8] == b"\x89PNG\r\n\x1a\n"
        assert struct.unpack(">II", png[16:24]) == (32, 32)
        assert hashlib.sha256(png).hexdigest() == entry["living_sha256"]
        bindings.append({"skill_review_id": f"D{number}", "number": number,
                         "name": entry["name"], "art_id": entry["art_id"],
                         "sprite": (bank_path.parent / entry["living"]).relative_to(ROOT).as_posix(),
                         "sprite_sha256": entry["living_sha256"], "native_size": [32, 32],
                         "sprite_modified": False, "skill_binding_approval": "proposed_pending_user_review"})
    decision = {"date": "2026-10-06", "runtime_integration": "not_integrated",
                "user_corrections": {"operator": "One learned point in attack-drone invocation; prior passive accepted",
                                     "tracker": "Instant mark, +15% accuracy rating; permanent +10% ranged weapon and attack-skill damage",
                                     "arcanist": "One learned point in damage magic; prior passive accepted",
                                     "metamorph": "One learned point in a transformation; +10% damage while transformed",
                                     "drones": "Each existing model gets an upgradable invocation; common lifespan, percentage resistance and percentage damage passives"},
                "accuracy_reply": {"question": "accuracy rating times 1.15 or add 15 percentage points to hit chance", "answer": "la 1 .", "accepted_rating_multiplier": 1.15},
                "accepted_initial_effects": {"O03": {"drone_damage_bonus_percent": 25},
                                             "T01": {"activation_turns": 0, "accuracy_rating_multiplier": 1.15},
                                             "T03": {"ranged_weapon_and_attack_skill_damage_bonus_percent": 10, "permanent": True},
                                             "A03": {"magic_energy_cost_reduction_percent": 20},
                                             "M03": {"transformed_personal_damage_bonus_percent": 10}},
                "bastion": "Preserved as melee-survival class; prior B01-B03 approvals remain unchanged",
                "drones_structure": {"invocation_ids": ["D56", "D57", "D58", "D59", "D66"],
                                     "global_passive_ids": ["DP01", "DP02", "O03"],
                                     "D59_and_O01": "one proposed skill, not two purchases"},
                "pending": ["Choice versus fixed first spell/form at character creation",
                            "Artilleur 59 assignment as the operator starting attack drone",
                            "New names and texts, growth curves, costs, durations and targets",
                            "Interpret lifespan as activity turns; other drone roles, limits and cargo/map transitions",
                            "All new effects and action/interface visuals before gameplay integration"],
                "sprite_bindings": bindings}
    decision_path = DIRECTORY / filenames[0]
    decision_text = json.dumps(decision, ensure_ascii=False, indent=2) + "\n"
    common = {"date": "2026-10-06", "runtime_integration": "not_integrated",
              "latest_decisions": decision_path.relative_to(ROOT).as_posix(),
              "latest_decisions_sha256": digest(decision_text)}
    helper_spec = importlib.util.spec_from_file_location("prior_review_helper", DIRECTORY / "build_class_review_updates.py")
    helper = importlib.util.module_from_spec(helper_spec)
    helper_spec.loader.exec_module(helper)
    second_path = ROOT / "docs/REVUE_COMPETENCES_AGENT_LOT_02.md"
    second_source = second_path.read_text(encoding="utf-8")
    expected = [f"{p}{n:02d}" for p in "OTAM" for n in range(4)]
    records = helper.extract_records(second_source, expected, old["blocks"])
    for record in records:
        if record["id"] in decision["accepted_initial_effects"]:
            record["accepted_initial_effect"] = deepcopy(decision["accepted_initial_effects"][record["id"]])
            record["mechanic_approval"] = "initial_effect_accepted_other_details_pending_user_review"
        if record["id"] in ["O01", "A01", "M01"]:
            record["starter_approval"] = "skill_category_confirmed_specific_binding_pending_user_review"
    second_payload = {**common, "batch_revision": 3, "document": second_path.relative_to(ROOT).as_posix(),
                      "document_sha256": digest(second_source), "previous_manifest": old_path.relative_to(ROOT).as_posix(),
                      "previous_document_snapshot": snapshot.relative_to(ROOT).as_posix(),
                      "status": "user_corrections_recorded_new_details_pending_review", "blocks": records}
    drone_path = ROOT / "docs/REVUE_DRONES_AGENT.md"
    drone_source = drone_path.read_text(encoding="utf-8")
    drone_payload = {**common, "batch_revision": 1, "document": drone_path.relative_to(ROOT).as_posix(),
                     "document_sha256": digest(drone_source), "status": "requested_structure_confirmed_values_pending_review",
                     "blocks": drone_records(drone_source), "existing_damage_passive_reference": "O03",
                     "duplicate_skill_alias": {"D59": "O01"}, "sprite_bindings": bindings}
    verify_curves()
    example_payload = {**common, "status": "proposal_arithmetic_verified_not_runtime_tests",
                       "supersedes_for_changed_skills": "artifacts/refonte-agent-2026-10-06/attribute-and-skill-examples.json",
                       "rows": [rows_at(n) for n in [1, 10, 12, 100]],
                       "verified": "HP, armed-drone damage and cargo capacity improve through effective rank 10000; bounded reductions stay below their limits",
                       "combat_balance": "not_playtested"}
    for name, payload in zip(filenames, [decision, second_payload, drone_payload, example_payload]):
        output(DIRECTORY / name, payload)
    print("Recorded scoped class approvals; 16 revised class blocks and 7 drone review blocks.")
    print("Verified arithmetic at ranks 1, 10, 12, 100 and curves through 10000; five unchanged native sprites match the bank.")
    print("Preserved prior review hashes and snapshot; no gameplay integration or playtest.")


if __name__ == "__main__":
    main()
