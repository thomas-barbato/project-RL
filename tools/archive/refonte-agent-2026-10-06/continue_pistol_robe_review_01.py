"""Record scoped art acceptance, revised proposals and current weapon coverage."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import re
import runpy
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
H = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (H[k] for k in ["load", "ref", "pair", "save", "preserve"])
ACCEPT = "equipped-class-visual-acceptance-decisions-01.json"
PALETTE_ACCEPT = "red-robe-visual-acceptance-decisions-01.json"
DECISION = "pistol-red-robe-visual-feedback-decisions-01.json"
COVERAGE = "weapon-held-family-coverage-2026-10-09.json"
VISUAL = "review-equipped-classes-START-VIS01-05.json"
BATCH = "review-class-starts-START-ALL01-06.json"
PAIRS = [("review-implementation-readiness-37.json", "review-implementation-readiness-38.json"),
         ("review-attribute-model-95.json", "review-attribute-model-96.json"),
         ("review-primary-catalogue-267.json", "review-primary-catalogue-268.json"),
         ("review-complete-skill-design-242.json", "review-complete-skill-design-243.json"),
         ("remaining-individual-skill-review-counts-136.json", "remaining-individual-skill-review-counts-137.json")]
NEW_FILES = [ACCEPT, PALETTE_ACCEPT, DECISION, COVERAGE, "review-equipped-classes-START-VIS01-04.json", VISUAL, BATCH] + [b for a, b in PAIRS]


def binary_ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def coverage():
    manifest = json.loads((ROOT / "assets/sprites/cyberpunk/manifest.json").read_text(encoding="utf-8"))
    models = []
    for path in sorted((ROOT / "content/core/weapons").glob("*.json5")):
        # Root definitions use two spaces. Nested ground effects also carry IDs.
        identity = re.findall(r'^  "?id"?\s*:\s*"([^"\n]+)"', path.read_text(encoding="utf-8"), re.MULTILINE)
        assert len(identity) == 1, path
        models.append(identity[0])
    rows = manifest["held_weapons"]
    held_ids = [row["id"] for row in rows]
    families = manifest["weapon_families"]
    assert len(models) == len(set(models)) == len(held_ids) == len(set(held_ids)) == 113
    assert set(models) == set(held_ids)
    assert len(families) == 18
    assert {row["family"] for row in rows} == set(range(18))
    atlas = Image.open(ROOT / "assets/sprites/cyberpunk/crawl-weapons-48.png").convert("RGBA")
    for family in families:
        index = family["index"]
        tile = atlas.crop((index % 16 * 48, index // 16 * 48, index % 16 * 48 + 48, index // 16 * 48 + 48))
        assert tile.getbbox() and set(tile.getchannel("A").tobytes()) <= {0, 255}
        assert family["hands"] and all(0 <= x < 48 and 0 <= y < 48 for x, y in family["hands"])
    code = (ROOT / "src/game_art_catalog.rs").read_text(encoding="utf-8")
    assert re.search(r"HELD_WEAPON_COUNT:\s*u16\s*=\s*113", code)
    assert re.search(r"WEAPON_FAMILY_COUNT:\s*u16\s*=\s*18", code)
    save(COVERAGE, {"date": "2026-10-09", "review_id": "WEAPON-VIS-COVERAGE01", "current_model_count": 113,
         "held_id_count": 113, "missing_models": [], "unknown_held_ids": [], "family_count": 18,
         "families": [{**row, "model_count": sum(entry["family"] == row["index"] for entry in rows)} for row in families],
         "mappings": rows, "native_family_cells": "nonempty_48px;binary_alpha;declared_grips_in_bounds",
         "sources": [binary_ref(ROOT / name) for name in ["assets/sprites/cyberpunk/manifest.json", "assets/sprites/cyberpunk/crawl-weapons-48.png", "src/game_art_catalog.rs", "src/game_art_worn.rs"]],
         "coverage_scope": "existing_catalogue_family_assignment;not_new_class_grip_validation_or_113_unique_drawings",
         "new_class_adaptation": "feasible_with_fixed_bodies_and_reusable_family_layers;all_humanoid_bases_sides_active_groups_and_dual_wielding_need_visual_review",
         "future_heavy_series": "accepted_flamethrower_rocket_Gatling_plasma_grenade_direction_preserved;unshown_new_models_and_visuals_not_inferred_approved",
         "runtime_changes": "none"})


def validate():
    checked = 0
    def walk(value):
        nonlocal checked
        if isinstance(value, dict):
            for key, expected in value.items():
                if key == "binary_sha256" and "path" in value:
                    assert hashlib.sha256((ROOT / value["path"]).read_bytes()).hexdigest() == expected
                    checked += 1
                elif key.endswith("_sha256") and key not in {"binary_sha256", "calculations_sha256"}:
                    source = value.get(key[:-7], value.get(key[:-7] + "_markdown"))
                    if not isinstance(source, str):
                        continue
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        path = ROOT / source
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/", "content/")):
                            raise AssertionError(("missing", source))
                    assert hashlib.sha256(content.encode("utf-8")).hexdigest() == expected, (key, source[:100])
                    checked += 1
            for child in value.values():
                walk(child)
        elif isinstance(value, list):
            for child in value:
                walk(child)
    for name in NEW_FILES:
        walk(load(name))
    for a, b in PAIRS:
        old, new = load(a), load(b)
        preserve(old, new)
        assert old.get("active_rule_overrides") == new.get("active_rule_overrides")
        assert new["pending_equipped_class_visual_review"] == ref(VISUAL)
        assert new["runtime_integration"] == "not_integrated"
    old, batch = load("review-class-starts-START-ALL01-05.json"), load(BATCH)
    for key in ("profiles", "implant_conditions", "accepted_starting_equipment", "accepted_heavy_weapon_family_direction"):
        assert old[key] == batch[key], key
    visual = load(VISUAL)
    assert visual["changed_candidate_numbers"] == [4]
    for row in visual["candidates"]:
        image = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
        assert image.size == (48, 48) and set(image.getchannel("A").tobytes()) <= {0, 255}
    tech, mage = visual["candidates"][1], visual["candidates"][3]
    assert tech["composition"]["orientation"].startswith("muzzle_down")
    assert tech["composition"]["original_body_pixel_changes"] == 0
    for row in (tech, mage):
        assert row["composition"]["handle_glove_overlap_pixels"] >= 2
        assert row["composition"]["added_forearm_or_finger_pixels"] == row["composition"]["clipped_weapon_pixels"] == 0
    assert mage["material_palette_proposal"]["secondary_ivory_and_protected_pixel_changes"] == 0
    assert mage["material_palette_proposal"]["red_ramp"] == [[76, 8, 15], [224, 22, 32], [255, 58, 50]]
    assert load(PAIRS[-1][1])["remaining_first_pass_total"] == 0
    return checked


def main():
    coverage()
    save(PALETTE_ACCEPT, {"date": "2026-10-09", "review_id": "START-VIS01-4-ACCEPT02",
         "shown_proposal": ref(VISUAL), "shown_candidate": load(VISUAL)["candidates"][3]["native_image"],
         "user_acceptance": "beaucoup mieux oui :)", "reply_context": "vivid_red_robe_after_muted_red_rejection",
         "accepted_visual_part": "candidate4_vivid_red_primary_robe_material;ivory_shape_black_glove_and_knife_preserved",
         "approval_scope": "shown_Arcaniste_robe_palette;not_downward_pistol2_or_unshown_weapon_configurations",
         "runtime_integration": "not_integrated"})
    save(ACCEPT, {"date": "2026-10-09", "review_id": "START-VIS01-ACCEPT01",
         "shown_proposal": ref("review-equipped-classes-START-VIS01-03.json"), "user_acceptance": "je valide",
         "subsequent_user_pause": "attends", "scope": "the_four_modified_candidates2_3_4_5_presented_in_revision3;not_unchanged1_or_unshown_weapon_configurations",
         "accepted_parts_preserved": {"START-VIS01-2": "original_arm_and_black_glove;upward_pistol_orientation_superseded_by_later_correction",
                                      "START-VIS01-3": "simplified_shown_pose_and_independent_rifle_palette",
                                      "START-VIS01-4": "original_shape_and_black_glove_knife_grip;new_red_palette_requires_separate_review",
                                      "START-VIS01-5": "shown_black_glove_knife_composition"},
         "new_downward_pistol_and_red_robe": "not_approved_by_this_historical_acceptance",
         "runtime_integration": "not_integrated"})
    save(DECISION, {"date": "2026-10-09", "review_id": "START-VIS01-FEEDBACK02", "prior_scoped_acceptance": ref(ACCEPT),
         "user_feedback": ["le pistolet du technomancien est toujours mal orienté, en faite l'arme devrait poitée vers le bas si tu mets le bras comme ca.",
                           "Aussi, penses tu qu'il serait possible d'adapter toutes les armes du jeu à l'affichage en jeu ? apres tout, on vient de le faire pour des armes de base :) .",
                           "Et peut-être changer la couleur de l'arcaniste pour une robe rouge.", "le rouge n'est pas assez petant :)", "beaucoup mieux oui :)"],
         "requested_direction": ["Technomancien_muzzle_down_with_existing_lowered_arm", "Arcaniste_vivid_saturated_red_primary_robe_material",
                                 "verify_feasibility_of_adapting_all_weapon_family_layers_to_new_character_rendering"],
         "rejected_or_superseded_parts": ["upward_pistol_orientation_from_revision3", "muted_red_palette_from_revision4"],
         "revised_proposal": ref(VISUAL), "superseded_muted_palette_proposal": ref("review-equipped-classes-START-VIS01-04.json"),
         "current_weapon_coverage": ref(COVERAGE),
         "accepted_vivid_red_palette": ref(PALETTE_ACCEPT),
         "approval_boundary": "vivid_red_palette4_accepted;downward_pistol2_remains_pending_separate_explicit_visual_acceptance;unshown_configurations_not_approved",
         "mechanical_changes": "none", "runtime_integration": "not_integrated"})
    old = load("review-class-starts-START-ALL01-05.json")
    batch = deepcopy(old)
    batch.update(revision=6, status="scoped_visual_acceptance_recorded;vivid_red_robe_accepted;downward_pistol_proposed;numeric_profiles_unchanged",
                 visual_approval="revision3_parts_and_vivid_red_palette4_accepted;revised_orientation2_pending;unchanged1_and_unshown_weapon_groups_not_inferred_approved")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-05.json")
    batch["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
    batch["pending_equipped_class_visual_review"] = ref(VISUAL)
    batch["equipped_visual_acceptance_history"] = ref(ACCEPT)
    batch["accepted_arcaniste_red_robe_visual"] = ref(PALETTE_ACCEPT)
    batch["latest_equipped_visual_feedback"] = ref(DECISION)
    batch["current_weapon_family_coverage_audit"] = ref(COVERAGE)
    preserve(old, batch)
    save(BATCH, batch)
    for a, b in PAIRS:
        old, data = load(a), deepcopy(load(a))
        data["date"] = "2026-10-09"
        data["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
        data["pending_equipped_class_visual_review"] = ref(VISUAL)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["equipped_visual_acceptance_history"] = ref(ACCEPT)
        data["accepted_arcaniste_red_robe_visual"] = ref(PALETTE_ACCEPT)
        data["latest_equipped_visual_feedback"] = ref(DECISION)
        data["current_weapon_family_coverage_audit"] = ref(COVERAGE)
        data["next_review_kind"] = "START_VIS01_downward_pistol_visual;vivid_red_robe_accepted"
        if a.startswith("review-implementation"):
            data.update(revision=38, status="scoped_art_acceptance_recorded;revised_pistol_robe_and_current_weapon_coverage_review;mechanics_unchanged")
            data["previous_inventory"] = ref(a)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["previous_pending_equipped_compositions"] = deepcopy(group["pending_equipped_compositions"])
                    group["pending_equipped_compositions"] = ref(VISUAL)
                    group["native_review_requirements"] = "pending_downward_pistol2;vivid_red_robe4_and_prior_shown_parts_accepted;layer_family_side_and_group_reviews_then_game_capture"
            data["recommendation"] = "review_downward_pistol2_then_continue_unshown_implant_ammo_and_heavy_model_reviews;vivid_red_robe4_already_accepted"
        else:
            if a.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL93", status="scoped_art_acceptance_and_revised_orientation_palette;accepted_numbers_and_mechanics_preserved")
                pair(data, "previous_model", a)
            elif a.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", a)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", VISUAL)
                data["latest_proposal_scope"] = "downward_pistol_and_vivid_red_robe;current113_models18_families_coverage;no_gameplay_change"
            else:
                pair(data, "previous_count", a)
                data["latest_feedback"] = "prior_shown_visual_parts_and_vivid_red_palette4_accepted;orientation2_pending;skill_rules_and_counts_unchanged"
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if not a.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        save(b, data)
    checked = validate()
    save("pistol-red-robe-visual-validation-01.json", {"date": "2026-10-09", "review_id": "START-VIS01-CHECK03",
         "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
         "accepted_rules_profiles_and_equipment": "unchanged", "pistol_body_geometry_and_glove": "unchanged",
         "red_robe_changes": "primary_authored_material_only;ivory_protected_pixels_weapon_and_alpha_preserved",
         "current_weapon_family_coverage": "113_models;18_families;0_missing", "active_source_asset_changes": 0,
         "verification_scope": "review_artifact_integrity_native_composition_and_catalogue_coverage;not_runtime_rendering_or_gameplay_validation",
         "runtime_integration": "not_integrated"})
    print(json.dumps({"files_checked": len(NEW_FILES), "hash_checks": checked, "errors": 0,
                      "weapon_models": 113, "families": 18, "mechanical_changes": 0, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
