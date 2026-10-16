"""Record accepted persistence and calculate the pending Maintenance proposal."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
DECISIONS = "artifacts/refonte-agent-2026-10-06/drone-persistence-acceptance-decisions.json"
MODELS = {"D56": (16, 3, 0, 0, 10), "D57": (28, 4, 0, 0, 10),
          "D58": (22, 4, 5, 2, 12), "D59": (20, 4, 4, 1, 12),
          "D66": (30, 5, 3, 1, 14)}


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def load(name):
    return json.loads((OUT / name).read_text(encoding="utf-8"))


def save(name, value):
    target = OUT / name
    assert not target.exists(), name
    target.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def floor(value):
    value = Fraction(value)
    return value.numerator // value.denominator


def serial(values):
    return {key: {"numerator": value.numerator, "denominator": value.denominator,
                  "display": round(float(value), 3)} if isinstance(value, Fraction) else value
            for key, value in values.items()}


def passives(rank):
    assert rank >= 1
    return {"repair_percent_maximum_hp": 2 + Fraction(rank - 1, 2),
            "damage_reduction_percent": 10 + Fraction(40 * (rank - 1), rank + 9),
            "damage_extra_percent": 25 + 2 * (rank - 1)}


def model(identifier, rank, intelligence=5, passive_rank=None):
    hp, hp_step, damage, damage_step, energy = MODELS[identifier]
    step = rank - 1
    intelligence_factor = 1 + Fraction(intelligence - 5, 50)
    damage_factor = 1 + Fraction(passives(passive_rank)["damage_extra_percent"], 100) if passive_rank else 1
    value = {"maximum_hp": floor((hp + hp_step * step) * intelligence_factor),
             "damage_before_defenses": floor((damage + damage_step * step) * intelligence_factor * damage_factor),
             "duration_turns": None, "unlimited_activity": True,
             "energy": energy + step // 3, "cooldown_turns": 6}
    if identifier == "D56":
        value["vision_cells"] = min(8, 4 + step // 3)
    if identifier == "D57":
        value["cargo_capacity_grams"] = floor((40_000 + 8_000 * step) * intelligence_factor)
    if identifier == "D66":
        value["protection_percent"] = 5 + Fraction(25 * step, rank + 9)
        value["protection_radius_cells"] = 2
    if identifier in ("D58", "D59", "D66"):
        value["attack_range_cells"] = 4 if identifier == "D59" else 1
    return value


def sections(source):
    matches = list(re.finditer(r"^### ([A-Z]+\d{2}) (.+)$", source, re.M))
    rows = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        body = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.M)[0].strip()
        copy = "\n".join(line for line in body.splitlines() if line.startswith(">"))
        rules = "\n".join(line for line in body.splitlines() if not line.startswith(">"))
        rows.append({"id": match.group(1), "title": match.group(2),
                     "block_sha256": digest(body), "text_sha256": digest(copy),
                     "mechanic_sha256": digest(rules)})
    return rows


def revise(old_name, new_name, snapshot_name):
    old = load(old_name)
    assert digest(read(f"artifacts/refonte-agent-2026-10-06/{snapshot_name}")) == old["document_sha256"]
    current = deepcopy(old)
    current.update(batch_revision=old["batch_revision"] + 1,
                   document_sha256=digest(read(old["document"])),
                   previous_manifest=f"artifacts/refonte-agent-2026-10-06/{old_name}",
                   previous_document_snapshot=f"artifacts/refonte-agent-2026-10-06/{snapshot_name}",
                   latest_decisions=DECISIONS, latest_decisions_sha256=digest(read(DECISIONS)))
    if "blocks" in current:
        hashes = {row["id"]: row for row in sections(read(current["document"]))}
        for row in current["blocks"]:
            updated = hashes[row["id"]]
            if updated["block_sha256"] != row["block_sha256"]:
                row["revision"] += 1
            row.update(updated)
    return current


def main():
    plans = [
        ("review-drone-descriptions-01.json", "review-drone-descriptions-02.json", "review-drone-descriptions-revision-01.md"),
        ("review-drones-02.json", "review-drones-03.json", "review-drones-revision-02.md"),
        ("review-lot-02-revision-04.json", "review-lot-02-revision-05.json", "review-lot-02-revision-04.md"),
        ("review-primary-catalogue-02.json", "review-primary-catalogue-03.json", "review-primary-catalogue-revision-02.md")]
    outputs = [DECISIONS.split("/")[-1], "drone-description-examples-02.json", "review-drone-persistence-01.json",
               *[new for _, new, _ in plans]]
    assert all(not (OUT / name).exists() for name in outputs)
    for old_name, _, snapshot in plans:
        assert digest(read(f"artifacts/refonte-agent-2026-10-06/{snapshot}")) == load(old_name)["document_sha256"]

    save(outputs[0], {"date": "2026-10-06", "runtime_integration": "not_integrated",
         "user_reply": "oui ok tu peux continuer :)",
         "approval_scope": "the immediately presented persistence proposal, not unshown roleplay, numbers or rendered effects",
         "accepted_rules": {"unlimited_activity_models": list(MODELS), "drones_can_be_destroyed_at_zero_hp": True,
                            "maximum_active": 5, "maximum_per_model": 1,
                            "deployment_energy": True, "per_turn_upkeep_energy": False,
                            "recall_and_layer_transition_preserve_hp_and_cargo": True,
                            "no_free_heal_on_redeployment": True, "mule_destruction_drops_real_cargo": True,
                            "DP01_name": "Maintenance des drones", "DP01_role": "progressive drone repair outside combat"},
         "pending_review": ["new DP01 roleplay text", "repair percentages and rank curve", "three calm turns and exact eligibility",
                            "fractional repair handling and recall/transition counters", "new repair visual intention and native render",
                            "other previously unapproved drone numbers and texts", "command costs and interface"],
         "original_proposal_snapshot": "artifacts/refonte-agent-2026-10-06/drone-persistence-proposal-revision-01.md",
         "original_proposal_sha256": digest(read("artifacts/refonte-agent-2026-10-06/drone-persistence-proposal-revision-01.md")),
         "cadrage_document": "docs/REFONTE_AGENT_CRYOGENISE.md",
         "cadrage_sha256": digest(read("docs/REFONTE_AGENT_CRYOGENISE.md")),
         "test_menu_timing": "after all skills and animations"})

    samples = []
    old_examples = load("drone-description-examples.json")
    for identifier in MODELS:
        rows = [{"effective_rank": n, **serial(model(identifier, n))} for n in [1, 5, 10, 12, 100, 1000]]
        current = model(identifier, 12, 10, 12)
        previous = next(row for row in old_examples["models"] if row["id"] == identifier)
        for key in ["maximum_hp", "damage_before_defenses", "energy", "cooldown_turns"]:
            assert current[key] == previous["current_example"][key]
        samples.append({"id": identifier, "rank_samples_without_modifiers": rows,
                        "default": serial(model(identifier, 1)), "current_example": serial(current),
                        "current_repair_hp_per_eligible_turn": serial({"value": current["maximum_hp"] * passives(12)["repair_percent_maximum_hp"] / 100})["value"]})
    assert passives(1)["repair_percent_maximum_hp"] == 2
    assert passives(10)["repair_percent_maximum_hp"] == Fraction(13, 2)
    assert passives(12)["repair_percent_maximum_hp"] == Fraction(15, 2)
    amount = 70 * passives(12)["repair_percent_maximum_hp"] / 100
    carry = Fraction(0)
    repaired = []
    for _ in range(4):
        carry += amount
        whole = floor(carry)
        carry -= whole
        repaired.append(whole)
    assert repaired == [5, 5, 5, 6] and carry == 0
    guardian = model("D66", 12)["protection_percent"]
    reduction = 100 * (1 - (1 - passives(12)["damage_reduction_percent"] / 100) * (1 - guardian / 100))
    assert round(float(reduction), 1) == 43.4
    examples_path = "artifacts/refonte-agent-2026-10-06/drone-description-examples-02.json"
    save(examples_path.split("/")[-1], {"date": "2026-10-06", "status": "proposed_design_calculations_not_engine_tests",
         "default_profile": {"effective_rank": 1, "intelligence": 5, "other_passives": False},
         "current_profile": {"learned_rank": 10, "equipment_rank_bonus": 2, "effective_rank": 12,
                             "intelligence": 10, "common_passive_ranks": 12, "actual_user_save": False},
         "models": samples, "common_passive_samples": [{"effective_rank": n, **serial(passives(n))} for n in [1, 5, 10, 12, 100, 1000]],
         "fractional_repair_example": {"maximum_hp": 70, "effective_rank": 12, "eligible_turn_repairs": repaired},
         "guardian_plus_blindage_current_reduction_percent": serial({"value": reduction})["value"]})

    for old_name, new_name, snapshot in plans:
        current = revise(old_name, new_name, snapshot)
        if new_name == "review-drone-descriptions-02.json":
            current.update(calculation_examples=examples_path, calculation_examples_sha256=digest(read(examples_path)),
                           status="persistence_and_maintenance_role_accepted_new_details_pending")
            for row in current["blocks"]:
                if row["id"] in MODELS:
                    row["accepted_activity_duration"] = "unlimited"
                if row["id"] == "DP01":
                    row.update(name_approval="accepted_persistence_revision_name", catalogue_effect_approval="repair_outside_combat_role_accepted",
                               text_approval="pending_individual_user_review", numeric_rules_approval="proposed_details_pending_user_review",
                               visual_intention_approval="new_repair_intention_pending_user_review")
        if new_name == "review-drones-03.json":
            current.update(status="historical_expiry_and_autonomy_superseded", historical_document=True,
                           current_drone_review="docs/REVUE_DRONES_AGENT_LOT_05.md")
            for row in current["blocks"]:
                row["superseded_duration_reference"] = "artifacts/refonte-agent-2026-10-06/review-drone-descriptions-02.json"
        if new_name == "review-lot-02-revision-05.json":
            current.update(status="O01_persistence_and_DP01_role_updated_other_details_pending",
                           catalogue_approval_reference="artifacts/refonte-agent-2026-10-06/review-primary-catalogue-03.json")
            next(row for row in current["blocks"] if row["id"] == "O01")["accepted_activity_duration"] = "unlimited"
        if new_name == "review-primary-catalogue-03.json":
            source = read(current["document"])
            lines = {match.group(1): match.group(0) for match in re.finditer(r"^\| ([A-Z]+\d{2}) \|.*$", source, re.M)}
            assert len(lines) == current["candidate_count"] == 86
            for branch in current["branches"]:
                for row in branch["candidates"]:
                    if row["id"] != "DP01":
                        assert digest(lines[row["id"]]) == row["row_sha256"], row["id"]
                    else:
                        cells = [cell.strip() for cell in lines["DP01"].strip("|").split("|")]
                        row.update(title=cells[1], effect_proposal=cells[3], visual_proposal=cells[4], row_sha256=digest(lines["DP01"]),
                                   name_approval="accepted_persistence_revision_name", mechanic_approval="repair_outside_combat_role_accepted_unshown_details_pending",
                                   visual_intention_approval="new_repair_intention_pending_user_review", effect_revision=2)
            current["DP01_replaces"] = "Autonomie des drones; duration removed"
        save(new_name, current)

    proposal = "docs/PROPOSITION_DRONES_PERSISTANTS_AGENT.md"
    save("review-drone-persistence-01.json", {"date": "2026-10-06", "batch_revision": 1, "runtime_integration": "not_integrated",
         "latest_decisions": DECISIONS, "latest_decisions_sha256": digest(read(DECISIONS)),
         "document": proposal, "document_sha256": digest(read(proposal)),
         "blocks": [{**row, "revision": 2, "mechanic_approval": "presented_persistence_and_maintenance_role_accepted",
                     "roleplay_text_approval": "no_roleplay_text_in_this_decision_document", "numeric_repair_approval": "pending_separate_DP01_review",
                     "visual_approval": "pending_native_preview_and_user_review"} for row in sections(read(proposal))]})
    print(json.dumps({"current_manifests": 5, "drone_models": 5, "rank_samples_checked": 48,
                      "catalogue_skills_preserved": 86, "repair_rank_1_10_12_percent": [2, 6.5, 7.5],
                      "fractional_repair_example": repaired, "runtime_changes": 0}))


if __name__ == "__main__":
    main()
