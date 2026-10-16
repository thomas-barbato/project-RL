"""Calculate proposed skill values and record the approved HUD pulse separately."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import importlib.util
import json

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
DECISIONS = PREFIX + "blink-acceptance-and-afflictions-decisions.json"
DOCUMENT = "docs/REVUE_AFFAIBLISSEMENTS_AGENT_LOT_06.md"
IDS = ("O02", "AF02", "AF03", "AF04", "AF05", "AF06")
ACTIVE = {
    "O02": (3, 5, 8, 5), "AF02": (3, 5, 9, 5),
    "AF03": (2, 5, 12, 8), "AF04": (3, 4, 10, 6),
    "AF05": (2, 5, 10, 7),
}
spec = importlib.util.spec_from_file_location("prior_review", OUT / "build_drone_persistence_review.py")
prior_review = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prior_review)


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def digest(path):
    return sha(read(path))


def load(name):
    return json.loads((OUT / name).read_text(encoding="utf-8"))


def save(name, data):
    target = OUT / name
    assert not target.exists(), f"Existing output: {name}"
    target.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def floor(value):
    value = Fraction(value)
    return value.numerator // value.denominator


def serial(value):
    if isinstance(value, Fraction):
        return {"numerator": value.numerator, "denominator": value.denominator,
                "display": round(float(value), 3)}
    if isinstance(value, dict):
        return {key: serial(item) for key, item in value.items()}
    if isinstance(value, list):
        return [serial(item) for item in value]
    return value


def chance(power, resistance=25):
    return min(Fraction(95), max(Fraction(5), Fraction(100 * power, power + max(1, resistance))))


def skill(identifier, rank, intelligence=5):
    assert identifier in IDS and rank >= 1 and intelligence >= 1
    step = rank - 1
    if identifier == "AF06":
        retention = Fraction(1, 2) + Fraction(2 * step, 5 * (rank + 9))
        source = skill("O02", rank, intelligence)
        return {"retention_fraction": retention, "radius_cells": min(4, 2 + step // 5),
                "maximum_new_targets_per_death": 1, "energy": 0, "cooldown_turns": 0,
                "inherited_seals_can_spread": False,
                "full_duration_interference_example": transfer(source, source["duration_turns"], retention)}
    base_duration, duration_step, base_energy, cooldown = ACTIVE[identifier]
    duration_bonus = min(3, max(0, intelligence - 5) // 20)
    power = max(1, 100 + 5 * step + 2 * (intelligence - 5))
    result = {"duration_turns": base_duration + step // duration_step + duration_bonus,
              "energy": base_energy + step // 3, "cooldown_turns": cooldown,
              "target_range_cells": 5, "application_power": power,
              "chance_against_resistance_25_percent": chance(power)}
    if identifier == "O02":
        result["direct_damage_reduction_percent"] = 15 + Fraction(35 * step, rank + 9)
    elif identifier == "AF02":
        result["armor_reduction_percent"] = 20 + Fraction(40 * step, rank + 9)
    elif identifier == "AF03":
        result["new_active_skill_activations_blocked"] = True
    elif identifier == "AF04":
        result["health_restoration_allowed"] = 0
    elif identifier == "AF05":
        result["false_position_radius_cells"] = 3
        result["receiving_damage_ends_effect"] = True
    return result


def transfer(source, remaining, retention):
    assert remaining >= 1
    power = max(1, floor(source["application_power"] * retention))
    return {"source_remaining_turns": remaining,
            "duration_turns": min(remaining, max(1, floor(remaining * retention))),
            "direct_damage_reduction_percent": source["direct_damage_reduction_percent"] * retention,
            "application_power": power,
            "chance_against_resistance_25_percent": chance(power)}


def calculate_and_check():
    checks = []
    default = {identifier: skill(identifier, 1) for identifier in IDS}
    current = {identifier: skill(identifier, 12, 25) for identifier in IDS}
    expected = {"O02": (6, 11, 5), "AF02": (6, 12, 5), "AF03": (5, 15, 8),
                "AF04": (6, 13, 6), "AF05": (5, 13, 7)}
    for identifier, (duration, energy, cooldown) in expected.items():
        row = current[identifier]
        assert (row["duration_turns"], row["energy"], row["cooldown_turns"]) == (duration, energy, cooldown)
        assert row["application_power"] == 195
        assert default[identifier]["application_power"] == 100
    checks.append("all five current active tables agree with independently specified duration, energy, cooldown and power")
    assert default["O02"]["direct_damage_reduction_percent"] == 15
    assert current["O02"]["direct_damage_reduction_percent"] == Fraction(100, 3)
    assert current["AF02"]["armor_reduction_percent"] == Fraction(860, 21)
    assert current["AF06"]["retention_fraction"] == Fraction(149, 210)
    assert current["AF06"]["radius_cells"] == 4
    inherited = current["AF06"]["full_duration_interference_example"]
    assert (inherited["duration_turns"], inherited["application_power"]) == (4, 138)
    assert inherited["direct_damage_reduction_percent"] == Fraction(1490, 63)
    assert default["AF06"]["full_duration_interference_example"]["duration_turns"] == 1
    checks.append("displayed reference, current reduction and one-transmission examples retain exact fractions")
    for identifier in ACTIVE:
        for intelligence in (1, 5, 10, 25, 65, 1000):
            row = skill(identifier, 12, intelligence)
            assert row["application_power"] >= 1
            assert Fraction(5) <= row["chance_against_resistance_25_percent"] <= Fraction(95)
        assert skill(identifier, 12, 25)["duration_turns"] == skill(identifier, 12, 5)["duration_turns"] + 1
        assert skill(identifier, 12, 1000)["duration_turns"] == skill(identifier, 12, 5)["duration_turns"] + 3
        assert skill(identifier, 12, 1000)["application_power"] > skill(identifier, 12, 65)["application_power"]
    checks.append("Intelligence adds duration at its stated thresholds and retains application power beyond the duration cap")
    for rank in range(1, 2001):
        interference = skill("O02", rank)
        armor = skill("AF02", rank)
        contagion = skill("AF06", rank)
        assert 15 <= interference["direct_damage_reduction_percent"] < 50
        assert 20 <= armor["armor_reduction_percent"] < 60
        assert Fraction(1, 2) <= contagion["retention_fraction"] < Fraction(9, 10)
        assert 2 <= contagion["radius_cells"] <= 4
        if rank > 1:
            assert interference["direct_damage_reduction_percent"] > skill("O02", rank - 1)["direct_damage_reduction_percent"]
            assert armor["armor_reduction_percent"] > skill("AF02", rank - 1)["armor_reduction_percent"]
            assert contagion["retention_fraction"] > skill("AF06", rank - 1)["retention_fraction"]
        for remaining in (1, 2, 3, 6, 1000):
            inherited = transfer(interference, remaining, contagion["retention_fraction"])
            assert 1 <= inherited["duration_turns"] <= remaining
            assert inherited["application_power"] <= interference["application_power"]
            assert inherited["direct_damage_reduction_percent"] <= interference["direct_damage_reduction_percent"]
    checks.append("ranks 1 through 2000 remain monotonic, below percentage limits, and cannot extend a transmitted remaining duration")
    samples = [{"id": identifier, "effective_rank": rank, "intelligence": 5,
                "values": serial(skill(identifier, rank))}
               for identifier in IDS for rank in (1, 5, 10, 12, 100, 1000)]
    samples.extend({"id": identifier, "learned_rank": 10, "equipment_bonus": 2,
                    "effective_rank": 12, "intelligence": 25, "values": serial(current[identifier])}
                   for identifier in IDS)
    return {"status": "passed_for_design_calculations_not_game_runtime", "sample_count": len(samples),
            "checks": checks, "samples": samples}


def revise_manifest(old_name, new_name, snapshot_name, source, allowed):
    previous = load(old_name)
    original = read(previous["document"])
    assert sha(original) == previous["document_sha256"], previous["document"]
    (OUT / snapshot_name).write_text(original, encoding="utf-8")
    (ROOT / previous["document"]).write_text(source, encoding="utf-8")
    current = deepcopy(previous)
    old_blocks = {row["id"]: row for row in previous["blocks"]}
    rows = []
    for hashes in prior_review.sections(source):
        row = deepcopy(old_blocks[hashes["id"]])
        if row["block_sha256"] != hashes["block_sha256"]:
            assert row["id"] in allowed
            row["revision"] += 1
            for key in ("block_sha256", "text_sha256", "mechanic_sha256"):
                if key in row:
                    row[key] = hashes[key]
        if row["id"] == "HUD02":
            row["visual_intention_approval"] = "displayed_corner_marker_pulse_accepted"
            row["visual_approval"] = "schematic_animated_preview_accepted_native_render_pending"
        rows.append(row)
    assert set(old_blocks) == {row["id"] for row in rows}
    current.update(batch_revision=previous["batch_revision"] + 1, document_sha256=sha(source), blocks=rows,
                   previous_manifest=PREFIX + old_name, previous_document_snapshot=PREFIX + snapshot_name,
                   latest_decisions=DECISIONS, latest_decisions_sha256=digest(DECISIONS))
    if new_name == "review-interface-effects-04.json":
        current["status"] = "displayed_schematic_pulse_accepted_native_render_and_unshown_details_pending"
        current["visual_approval"] = "displayed_schematic_pulse_accepted_native_render_pending"
    else:
        current["afflictions_review"] = DOCUMENT
    save(new_name, current)


def main():
    outputs = ("blink-acceptance-and-afflictions-decisions.json", "affliction-description-examples-01.json",
               "review-affliction-descriptions-01.json", "review-interface-effects-04.json",
               "review-interface-effects-revision-03.md", "review-description-format-05.json",
               "review-description-format-revision-04.md")
    assert all(not (OUT / name).exists() for name in outputs), "Review outputs already exist"
    for old_name in ("review-interface-effects-03.json", "review-description-format-04.json"):
        manifest = load(old_name)
        assert digest(manifest["document"]) == manifest["document_sha256"]
    hud = read("docs/REVUE_INTERFACE_EFFETS_AGENT.md")
    old_pulse = "à pleine opacité. Cette animation constitue un nouveau rendu à valider."
    new_pulse = ("à pleine opacité. Le joueur accepte l'animation de l'aperçu en répondant\n"
                 "« parfait on peut continuer . ». Le cycle de 1,4 seconde et les cadres\n"
                 "toujours visibles sont retenus tels que présentés ; le rendu natif reste à revoir.")
    old_state = ("La nouvelle animation, le rendu natif dans le jeu, l'emplacement définitif\n"
                 "et les règles non présentées, notamment l'initialisation et les transitions,\n"
                 "attendent leur propre validation. Cet accord ne valide pas automatiquement\n"
                 "les pourcentages de réparation ou le texte roleplay de DP01.")
    new_state = ("La pulsation de l'aperçu révisé est ensuite acceptée par « parfait on peut\n"
                 "continuer . ». Le rendu natif dans le jeu, l'emplacement définitif et les\n"
                 "règles non présentées, notamment l'initialisation et les transitions,\n"
                 "attendent leur propre validation. Les détails d'accessibilité non présentés\n"
                 "gardent leur statut de proposition. Cet accord ne valide pas automatiquement\n"
                 "les pourcentages de réparation ou le texte roleplay de DP01.")
    assert hud.count(old_pulse) == hud.count(old_state) == 1
    hud = hud.replace(old_pulse, new_pulse).replace(old_state, new_state)
    descriptions = read("docs/REVUE_DESCRIPTIFS_COMPETENCES_AGENT.md")
    descriptions = descriptions.rstrip() + ("\nLa [revue des Affaiblissements](REVUE_AFFAIBLISSEMENTS_AGENT_LOT_06.md)\n"
                                            "continue avec cinq actifs et un passif. Leurs nouveaux textes, chiffres\n"
                                            "et cas détaillés attendent une validation individuelle.\n")
    examples = calculate_and_check()
    pulse = load("hostiles-calm-blink-decisions.json")
    assert digest(pulse["prototype"]) == pulse["prototype_sha256"]
    assert digest(pulse["prototype_checks"]) == pulse["prototype_checks_sha256"]
    save(outputs[0], {
        "date": "2026-10-06", "user_reply": "parfait on peut continuer .", "runtime_integration": "not_integrated",
        "accepted_scope": "animation as shown in the immediately preceding schematic hostile-highlight preview",
        "accepted_animation": {"cycle_seconds": 1.4, "minimum_opacity": 0.6, "maximum_opacity": 1.0,
                               "only_corner_markers_pulse": True, "markers_always_visible": True},
        "prototype": pulse["prototype"], "prototype_sha256": pulse["prototype_sha256"],
        "previous_decisions": PREFIX + "hostiles-calm-blink-decisions.json",
        "previous_decisions_sha256": digest(PREFIX + "hostiles-calm-blink-decisions.json"),
        "still_pending": ["native game rendering", "definitive HUD location", "unshown accessibility and phase details",
                          "Maintenance roleplay and numeric rates", "all new Affaiblissements roleplay, numbers and detailed conditions"],
        "continued_work": "six existing Affaiblissements skills with reference/current values, proposed for individual review",
        "skill_ids": list(IDS), "catalogue_count_change": 0,
        "test_character_menu": "after all skills and animations are implemented"
    })
    save(outputs[1], examples)
    revise_manifest("review-interface-effects-03.json", "review-interface-effects-04.json",
                    "review-interface-effects-revision-03.md", hud, {"HUD02"})
    revise_manifest("review-description-format-04.json", "review-description-format-05.json",
                    "review-description-format-revision-04.md", descriptions, {"CD03"})
    blocks = []
    for hashes in prior_review.sections(read(DOCUMENT)):
        blocks.append({**hashes, "revision": 1, "type": "passive" if hashes["id"] == "AF06" else "active",
                       "name_approval": "accepted_catalogue_name", "catalogue_effect_approval": "accepted_as_presented",
                       "text_approval": "pending_individual_user_review", "numeric_rules_approval": "pending_individual_user_review",
                       "detailed_conditions_approval": "pending_individual_user_review",
                       "visual_intention_approval": "accepted_catalogue_intention",
                       "visual_approval": "pending_native_animation_preview_and_user_review", "runtime_integration": "not_integrated"})
    assert tuple(row["id"] for row in blocks) == IDS
    save(outputs[2], {"date": "2026-10-06", "batch_revision": 1, "document": DOCUMENT,
                      "document_sha256": digest(DOCUMENT), "runtime_integration": "not_integrated",
                      "latest_decisions": DECISIONS, "latest_decisions_sha256": digest(DECISIONS),
                      "reference_profile": {"effective_rank": 1, "intelligence": 5},
                      "current_example_profile": {"learned_rank": 10, "equipment_bonus": 2, "effective_rank": 12, "intelligence": 25},
                      "blocks": blocks, "examples": PREFIX + outputs[1], "examples_sha256": digest(PREFIX + outputs[1]),
                      "existing_interference_definition": "docs/REVUE_COMPETENCES_AGENT_LOT_02.md#o02-interférence",
                      "existing_catalogue": "docs/PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md",
                      "catalogue_count_change": 0})
    print(json.dumps({"approved": "displayed schematic hostile corner pulse", "proposed_affliction_skills": len(blocks),
                      "calculated_samples": examples["sample_count"], "calculation_checks": len(examples["checks"]),
                      "runtime_changes": 0}, ensure_ascii=False))


if __name__ == "__main__":
    main()
