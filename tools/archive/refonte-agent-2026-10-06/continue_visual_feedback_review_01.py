"""Record visual rejection and link corrected candidates, preserving all mechanics."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
H = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (H[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "equipped-class-visual-feedback-decisions-01.json"
VISUAL = "review-equipped-classes-START-VIS01-03.json"
BATCH = "review-class-starts-START-ALL01-05.json"
PAIRS = [("review-implementation-readiness-36.json", "review-implementation-readiness-37.json"),
         ("review-attribute-model-94.json", "review-attribute-model-95.json"),
         ("review-primary-catalogue-266.json", "review-primary-catalogue-267.json"),
         ("review-complete-skill-design-241.json", "review-complete-skill-design-242.json"),
         ("remaining-individual-skill-review-counts-135.json", "remaining-individual-skill-review-counts-136.json")]
NEW_FILES = [DECISION, "review-equipped-classes-START-VIS01-02.json", VISUAL, BATCH] + [v[1] for v in PAIRS]


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
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        assert new.get("active_rule_overrides") == old.get("active_rule_overrides")
        assert new["accepted_starting_equipment"] == old["accepted_starting_equipment"]
        assert new["accepted_heavy_weapon_family_direction"] == old["accepted_heavy_weapon_family_direction"]
        assert new["pending_equipped_class_visual_review"] == ref(VISUAL)
        assert new["runtime_integration"] == "not_integrated"
    before, batch = load("review-class-starts-START-ALL01-04.json"), load(BATCH)
    for key in ["profiles", "implant_conditions", "accepted_starting_equipment", "accepted_heavy_weapon_family_direction"]:
        assert batch[key] == before[key], key
    visual = load(VISUAL)
    assert visual["changed_candidate_numbers"] == [2, 3, 4, 5]
    for row in visual["candidates"]:
        im = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
        assert im.size == (48, 48)
        assert set(im.getchannel("A").tobytes()) <= {0, 255}
        if row["number"] in [2, 4, 5]:
            details = row["composition"]
            assert details["original_body_pixel_changes"] == details["added_forearm_or_finger_pixels"] == details["clipped_weapon_pixels"] == 0
            assert details["handle_glove_overlap_pixels"] >= 2
        elif row["number"] == 3:
            assert len({p[:3] for p in im.get_flattened_data() if p[3]}) == 12
            assert row["conditioning"]["anatomy_pixel_identity_with_prior_body"].startswith("not_claimed")
            assert row["independent_palette"]["rifle"] == "steel_and_anthracite"
    assert load(PAIRS[-1][1])["remaining_first_pass_total"] == 0
    return checked


def main():
    save(DECISION, {"date": "2026-10-09", "review_id": "START-VIS01-FEEDBACK01",
          "reviewed_proposal": ref("review-equipped-classes-START-VIS01-01.json"),
          "user_feedback": [
              "non le pistolet ca va pas , le pistolet du technomancien est mal tenu . Et en vrai meme les armes de mélée à une main j'ai l'impression que tu ajoutes un avant bras au bras du personnage là ou il tien son arme... et donc au lieu de faire simplement une boule noire en guise de main avec une lame, ben ca fait une forme blanche bizarre...",
              "et c'est pareil pour le technomancien",
              "aussi je trouve que le traqueur (et son fils) sont trop détaillés... ce qui fait que en version 30px on voit rien.",
              "aussi je sais pas pourquoi tu tiens à ce que les personnages aient la meme couleur d'armure que d'arme mais bon ..."],
          "reading_of_typo": "son_fils_understood_as_son_fusil_from_the_weapon_preview_context;no_new_character_created",
          "requested_corrections": ["Technomancien_pistol_grip", "no_extra_one_hand_forearm_or_pale_hand;simple_original_black_glove",
                                    "simplify_Traqueur_body_and_rifle_for_native_readability", "do_not_impose_matching_weapon_and_armor_palettes"],
          "visual_rejection_scope": "candidate2_pistol_and_one_hand_melee_compositions4_5;candidate3_native_detail;no_acceptance_of_other_candidates_inferred",
          "preserved": "accepted_START_GEAR01_numbers_names_and_consumables;class_attributes_skills_implants_reserves;heavy_family_series_request;existing_independent_palette_rule",
          "revised_candidates": ref(VISUAL), "visual_approval": "revisions_pending_explicit_user_review", "runtime_integration": "not_integrated"})
    old = load("review-class-starts-START-ALL01-04.json")
    data = deepcopy(old)
    data.update(revision=5, status="starting_gear_accepted;equipped_visuals_revised_after_feedback;numeric_profiles_unchanged",
                visual_approval="START_VIS01_revised_candidates2_3_4_5_pending;no_acceptance_of_unchanged1_inferred")
    data["previous_batch"] = ref("review-class-starts-START-ALL01-04.json")
    data["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
    data["pending_equipped_class_visual_review"] = ref(VISUAL)
    data["latest_equipped_visual_feedback"] = ref(DECISION)
    data["visual_integration_boundary"] = "one_hand_rule_is_a_prototype;Traqueur_simplified_complete_pose_requires_layer_preparation_and_group_review_before_runtime_use"
    preserve(old, data)
    save(BATCH, data)
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data["date"] = "2026-10-09"
        data["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
        data["pending_equipped_class_visual_review"] = ref(VISUAL)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["latest_equipped_visual_feedback"] = ref(DECISION)
        data["next_review_kind"] = "START_VIS01_revision3_native_grips_simplified_Traqueur_and_independent_palettes"
        if old_name.startswith("review-implementation"):
            data.update(revision=37, status="equipped_visual_feedback_recorded;corrected_native_review_proposed;mechanics_unchanged")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["previous_pending_equipped_compositions"] = deepcopy(group["pending_equipped_compositions"])
                    group["pending_equipped_compositions"] = ref(VISUAL)
                    group["native_review_requirements"] = "explicit_review_of_corrected_one_hand_grips_and_simplified_Traqueur;body_weapon_palette_independence;prepare_layers_then_game_capture_before_integration"
            data["recommendation"] = "review_START_VIS01_revision3_then_continue_unshown_implant_ammo_and_heavy_model_reviews"
        else:
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL92", status="equipped_visual_feedback_revisions;accepted_numbers_and_mechanics_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", VISUAL)
                data["latest_proposal_scope"] = "corrected_native_grips_simplified_Traqueur_and_neutral_rifle;no_numeric_or_gameplay_change"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "visual_grips_and_native_readability_revised;skill_rules_and_counts_unchanged"
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("equipped-class-visual-feedback-validation-01.json", {"date": "2026-10-09", "review_id": "START-VIS01-CHECK02",
         "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
         "accepted_rules_and_profile_numbers": "unchanged", "original_one_hand_body_pixel_changes": 0,
         "one_hand_added_forearm_pixels": 0, "Traqueur_opaque_colors": 12, "weapon_armor_palettes": "independent",
         "sources_and_prompts": "saved_to_workspace;built_in_imagegen_for_new_raster_art;native_conditioning_only",
         "verification_scope": "design_reference_integrity_and_native_prototypes;not_runtime_rendering_or_gameplay_validation",
         "runtime_integration": "not_integrated"})
    print(json.dumps({"files_checked": len(NEW_FILES), "hash_checks": checked, "errors": 0,
                      "mechanic_changes": 0, "visuals": "candidates2_3_4_5_revised_pending", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
