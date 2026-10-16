"""Calculate pending drone descriptions and record the accepted five-model limit."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json
import re
import struct

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
MODELS = {
    "D56": (16, 3, 0, 0, 8, 10), "D57": (28, 4, 0, 0, 10, 10),
    "D58": (22, 4, 5, 2, 8, 12), "D59": (20, 4, 4, 1, 8, 12),
    "D66": (30, 5, 3, 1, 8, 14),
}
IDS = [*MODELS, "DP01", "DP02", "O03"]


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


def floor(value):
    value = Fraction(value)
    return value.numerator // value.denominator


def serialize(values):
    return {key: {"numerator": value.numerator, "denominator": value.denominator,
                  "display": round(float(value), 1)} if isinstance(value, Fraction) else value
            for key, value in values.items()}


def passives(rank):
    assert rank >= 1
    return {"duration_extra_turns": rank + 1,
            "damage_reduction_percent": 10 + Fraction(40 * (rank - 1), rank + 9),
            "damage_extra_percent": 25 + 2 * (rank - 1)}


def model(identifier, rank, intelligence=5, passive_rank=None):
    assert rank >= 1
    hp, hp_step, damage, damage_step, duration, energy = MODELS[identifier]
    step = rank - 1
    modifier = 1 + Fraction(intelligence - 5, 50)
    learned_passives = passives(passive_rank) if passive_rank else None
    damage_modifier = 1 + Fraction(learned_passives["damage_extra_percent"], 100) if learned_passives else 1
    result = {"maximum_hp": floor((hp + hp_step * step) * modifier),
              "damage_before_defenses": floor((damage + damage_step * step) * modifier * damage_modifier),
              "duration_turns": duration + step // 3 + (learned_passives["duration_extra_turns"] if learned_passives else 0),
              "energy": energy + step // 3, "cooldown_turns": 6}
    if identifier == "D56":
        result["vision_cells"] = min(8, 4 + step // 3)
    if identifier == "D57":
        result["cargo_capacity_grams"] = floor((40_000 + 8_000 * step) * modifier)
    if identifier == "D66":
        result["protection_percent"] = 5 + Fraction(25 * step, rank + 9)
        result["protection_radius_cells"] = 2
    if identifier in ("D58", "D59", "D66"):
        result["attack_range_cells"] = 4 if identifier == "D59" else 1
    return result


def sections(source):
    matches = list(re.finditer(r"^### ([A-Z]+\d{2}) (.+)$", source, re.M))
    result = []
    for index, match in enumerate(matches):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        body = re.split(r"^## ", source[match.end():end], maxsplit=1, flags=re.M)[0].strip()
        copy = "\n".join(line for line in body.splitlines() if line.startswith(">"))
        mechanic = "\n".join(line for line in body.splitlines() if not line.startswith(">"))
        result.append({"id": match.group(1), "title": match.group(2), "block_sha256": digest(body),
                       "text_sha256": digest(copy), "mechanic_sha256": digest(mechanic)})
    return result


def main():
    decisions_name = "drone-description-and-five-model-decisions.json"
    decisions = {
        "date": "2026-10-06", "runtime_integration": "not_integrated",
        "continuation_reply": "tu peux continuer",
        "continuation_scope": "continue skill descriptions; no additional explicit text or number approval recorded for the Survival lot",
        "user_drone_limit_reply": "5 drones, un par modèle",
        "accepted_drone_limit": {"maximum_active": 5, "maximum_each_model": 1, "five_models_together_allowed": True},
        "replaced_proposed_limit": "two drones total, one per model",
        "accepted_catalogue": "five invocations and three common passives with their presented names and roles",
        "pending_review": ["eight new roleplay texts", "default and illustrative current tables", "unapproved rank formulas and stat contributions", "Mule cargo capacity expressed in kilograms", "commands, transitions and detailed behavior", "new rendered effects and animations"],
        "test_menu_timing": "after all skills and animations, as explicitly requested"
    }
    save(decisions_name, decisions)
    decisions_path = f"artifacts/refonte-agent-2026-10-06/{decisions_name}"
    common = {"date": "2026-10-06", "latest_decisions": decisions_path,
              "latest_decisions_sha256": digest(read(decisions_path)), "runtime_integration": "not_integrated"}

    old = load("review-drones-01.json")
    snapshot = "artifacts/refonte-agent-2026-10-06/review-drones-revision-01.md"
    assert digest(read(snapshot)) == old["document_sha256"]
    new = deepcopy(old)
    new.update(common, batch_revision=2, document_sha256=digest(read(new["document"])),
               previous_manifest="artifacts/refonte-agent-2026-10-06/review-drones-01.json",
               previous_document_snapshot=snapshot,
               status="catalogue_roles_and_five_model_limit_accepted_other_details_pending",
               accepted_drone_limit=decisions["accepted_drone_limit"])
    hashes = {row["id"]: row for row in sections(read(new["document"]))}
    for row in new["blocks"]:
        assert row["block_sha256"] == hashes[row["id"]]["block_sha256"], row["id"]
        row["name_approval"] = "accepted_catalogue_name"
        row["catalogue_role_approval"] = "accepted_as_presented"
    for binding in new["sprite_bindings"]:
        content = (ROOT / binding["sprite"]).read_bytes()
        assert hashlib.sha256(content).hexdigest() == binding["sprite_sha256"]
        assert list(struct.unpack(">II", content[16:24])) == binding["native_size"] == [32, 32]
        binding["skill_binding_approval"] = "catalogue_role_association_accepted_runtime_render_pending"
    save("review-drones-02.json", new)

    expected = {"D56": (53, 0, 24, 13), "D57": (79, 0, 26, 13),
                "D58": (72, 43, 24, 15), "D59": (70, 24, 24, 15), "D66": (93, 22, 24, 17)}
    sample_rows = []
    for identifier in MODELS:
        samples = []
        previous = None
        for rank in [1, 5, 10, 12, 100, 1000]:
            value = model(identifier, rank)
            assert value["energy"] > 0 and value["duration_turns"] > 0
            if previous:
                assert value["maximum_hp"] > previous["maximum_hp"]
                assert value["damage_before_defenses"] >= previous["damage_before_defenses"]
            if identifier == "D56":
                assert value["vision_cells"] <= 8 and value["damage_before_defenses"] == 0
            if identifier == "D57":
                assert value["damage_before_defenses"] == 0
                if previous:
                    assert value["cargo_capacity_grams"] > previous["cargo_capacity_grams"]
            if identifier == "D66":
                assert 5 <= value["protection_percent"] < 30
            previous = value
            samples.append({"effective_rank": rank, **serialize(value)})
        current = model(identifier, 12, 10, 12)
        assert tuple(current[key] for key in ["maximum_hp", "damage_before_defenses", "duration_turns", "energy"]) == expected[identifier]
        sample_rows.append({"id": identifier, "rank_samples_without_modifiers": samples,
                           "default": serialize(model(identifier, 1)), "current_example": serialize(current)})
    assert model("D57", 12, 10, 12)["cargo_capacity_grams"] == 140_800
    p12 = passives(12)
    guardian12 = model("D66", 12)["protection_percent"]
    combined = 100 * (1 - (1 - p12["damage_reduction_percent"] / 100) * (1 - guardian12 / 100))
    assert round(float(combined), 1) == 43.4
    passives_samples = [{"effective_rank": n, **serialize(passives(n))} for n in [1, 5, 10, 12, 100, 1000]]
    assert all(10 <= passives(n)["damage_reduction_percent"] < 50 for n in [1, 5, 10, 12, 100, 1000])
    examples_name = "drone-description-examples.json"
    examples_path = f"artifacts/refonte-agent-2026-10-06/{examples_name}"
    save(examples_name, {"date": "2026-10-06", "status": "proposed_design_calculations_not_engine_tests",
         "default_profile": {"effective_rank": 1, "intelligence": 5, "other_passives": False},
         "current_profile": {"learned_rank": 10, "equipment_rank_bonus": 2, "effective_rank": 12,
                             "intelligence": 10, "common_passive_ranks": 12, "actual_user_save": False},
         "models": sample_rows, "common_passive_samples": passives_samples,
         "guardian_plus_blindage_current_reduction_percent": serialize({"value": combined})["value"]})

    document = "docs/REVUE_DRONES_AGENT_LOT_05.md"
    prior = {row["id"]: row for row in old["blocks"]}
    prior_o03 = next(row for row in load("review-lot-02-revision-04.json")["blocks"] if row["id"] == "O03")
    rows = sections(read(document))
    assert [row["id"] for row in rows] == IDS
    for row in rows:
        previous = prior.get(row["id"], prior_o03)
        row.update(revision=previous["revision"] + 1, name_approval="accepted_catalogue_name",
                   catalogue_effect_approval="accepted_as_presented", text_approval="pending_individual_user_review",
                   numeric_rules_approval="proposed_details_pending_user_review", visual_intention_approval="accepted_as_presented",
                   visual_approval="pending_native_preview_and_user_review", runtime_integration="not_integrated")
        if row["id"] == "O03":
            row["accepted_initial_effect"] = "+25% damage for armed drones"
    save("review-drone-descriptions-01.json", {**common, "batch_revision": 1, "document": document,
         "document_sha256": digest(read(document)), "blocks": rows,
         "accepted_drone_limit": decisions["accepted_drone_limit"], "D59_alias": "O01; one skill",
         "calculation_examples": examples_path, "calculation_examples_sha256": digest(read(examples_path))})

    format_old = load("review-description-format-03.json")
    format_snapshot = "artifacts/refonte-agent-2026-10-06/review-description-format-revision-03.md"
    assert digest(read(format_snapshot)) == format_old["document_sha256"]
    format_new = deepcopy(format_old)
    source = read(format_new["document"])
    matches = list(re.finditer(r"^### (CD\d{2}) (.+)$", source, re.M))
    for index, (match, row) in enumerate(zip(matches, format_new["blocks"])):
        end = matches[index + 1].start() if index + 1 < len(matches) else len(source)
        body = source[match.end():end].strip()
        if digest(body) != row["block_sha256"]:
            assert row["id"] == "CD03"
            row.update(revision=row["revision"] + 1, block_sha256=digest(body))
    format_new.update(common, batch_revision=4, document_sha256=digest(source),
                      previous_manifest="artifacts/refonte-agent-2026-10-06/review-description-format-03.json",
                      previous_document_snapshot=format_snapshot, drone_review=document)
    save("review-description-format-04.json", format_new)
    print(json.dumps({"drone_models": 5, "common_passives": 3, "rank_samples_checked": 48,
                      "maximum_active_drones_accepted": 5, "existing_32px_sprites_preserved": 5}))


if __name__ == "__main__":
    main()
