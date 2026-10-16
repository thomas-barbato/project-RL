"""Append explicit visual acceptance; keep every approved gameplay rule intact."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
H = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (H[k] for k in ["load", "ref", "pair", "save", "preserve"])
RECEIPT = "weapon-silhouette-visual-acceptance-01.json"
BATCH = "review-class-starts-START-ALL01-09.json"
PAIRS = [("review-implementation-readiness-40.json", "review-implementation-readiness-41.json"),
         ("review-attribute-model-98.json", "review-attribute-model-99.json"),
         ("review-primary-catalogue-270.json", "review-primary-catalogue-271.json"),
         ("review-complete-skill-design-245.json", "review-complete-skill-design-246.json"),
         ("remaining-individual-skill-review-counts-139.json", "remaining-individual-skill-review-counts-140.json")]


def main():
    manifest = ROOT / "assets/sprites/cyberpunk/agent-rigs/manifest.json"
    active = {"reference": manifest.relative_to(ROOT).as_posix(),
              "reference_sha256": hashlib.sha256(manifest.read_text(encoding="utf-8").encode("utf-8")).hexdigest()}
    save(RECEIPT, {"date": "2026-10-09", "review_id": "WEAPON-SIL01-ACCEPT01", "user_acceptance": "oui je valide :)",
         "question_answered": "Tu valides ces rendus gris et les nouvelles prises présentées dans la galerie, y compris la base du Traqueur ?",
         "accepted_bank": ref("weapon-silhouette-review-03/review-weapon-silhouettes-WEAPON-SIL01-03.json"),
         "accepted_presentation": ref("weapon-silhouette-review-03/weapon-silhouette-review-presentation-02.json"),
         "accepted_start_compositions": ref("review-equipped-classes-START-VIS01-07.json"),
         "approved_visual_scope": {"models": 113, "families": 18, "silhouettes": 5, "configurations": 375,
             "parts": "fixed_native_bodies_including_Traqueur_unarmed_layer;family_weapon_hand_attachments;two_hand_forearms;left_right_and_dual_compositions",
             "materials": "light_gray_pistol;steel_anthracite_rifle;neutral_gray_replacing_Pink_equipment_palette;accepted_red_robe_and_other_unchanged_parts_preserved"},
         "active_visual_manifest": active,
         "approved_visual_integration": "authorized_and_installed_in_graphics_pipeline;validation_report_under_artifacts/weapon-integration-2026-10-09",
         "gameplay_scope": "no_change_to_class_catalog_ids_starting_stats_skills_equipment_model_ids_bonuses_collisions_turns_or_rng;remaining_refonte_systems_separate"})
    old = load("review-class-starts-START-ALL01-08.json")
    batch = deepcopy(old)
    batch.update(revision=9, status="all_presented_class_weapon_visuals_accepted;graphics_integrated;gameplay_profiles_preserved")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-08.json")
    batch["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
    for key in ["pending_equipped_class_visual_review", "pending_full_weapon_visual_review", "pending_weapon_review_presentation"]:
        batch[key] = None
    batch["accepted_class_and_weapon_visuals"] = ref(RECEIPT)
    batch["active_weapon_visual_manifest"] = active
    batch["latest_equipped_visual_feedback"] = ref(RECEIPT)
    preserve(old, batch)
    save(BATCH, batch)
    for a, b in PAIRS:
        old, data = load(a), deepcopy(load(a))
        data["date"] = "2026-10-09"
        data["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
        for key in ["pending_equipped_class_visual_review", "pending_full_weapon_visual_review", "pending_weapon_review_presentation"]:
            data[key] = None
        data["accepted_class_and_weapon_visuals"] = ref(RECEIPT)
        data["active_weapon_visual_manifest"] = active
        data["prepared_five_class_starts"] = ref(BATCH)
        data["latest_equipped_visual_feedback"] = ref(RECEIPT)
        data["next_review_kind"] = "accepted_class_weapon_graphics_integrated;continue_remaining_refonte_implant_ammo_and_heavy_model_designs"
        # This status describes the whole refonte, not just the approved graphics subset.
        data["runtime_integration"] = "not_integrated"
        data["runtime_visual_integration"] = "approved_native_class_weapon_layers_and_gray_palette_active"
        if a.startswith("review-implementation"):
            data.update(revision=41, status="approved_class_weapon_graphics_active;remaining_refonte_systems_separate")
            data["previous_inventory"] = ref(a)
            data["latest_user_decisions"] = ref(RECEIPT)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["previous_pending_equipped_compositions"] = deepcopy(group["pending_equipped_compositions"])
                    group["pending_equipped_compositions"] = None
                    group["accepted_visual_review"] = ref(RECEIPT)
                    group["native_review_requirements"] = "presented_visuals_accepted;active_pipeline_checks_and_captures_recorded_in_weapon_integration_2026_10_09;new_future_shapes_still_require_review"
            data["recommendation"] = "continue_pending_implant_ammo_and_heavy_model_designs_and_accepted_refonte_gameplay_implementation;do_not_request_identical_visual_approval_again"
        else:
            if a.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL96", status="class_weapon_visuals_accepted;approved_stats_and_mechanics_preserved")
                pair(data, "previous_model", a)
            elif a.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", a)
                pair(data, "latest_decisions", RECEIPT)
                data["latest_proposal_scope"] = "previous_full_weapon_and_gray_visual_review_now_accepted_and_integrated;no_gameplay_change"
            else:
                pair(data, "previous_count", a)
                data["latest_feedback"] = "full_weapon_gray_visuals_explicitly_accepted;skill_review_counts_preserved"
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if not a.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        assert old.get("active_rule_overrides") == data.get("active_rule_overrides")
        save(b, data)
    hashes = runpy.run_path(str(BASE / "continue_pistol_arm_axis_review_01.py"), run_name="hash_helpers")["hashes"]
    checked = sum(hashes(load(name)) for name in [RECEIPT, BATCH] + [b for _, b in PAIRS])
    print(json.dumps({"explicit_visual_acceptance": True, "hash_checks": checked, "preserved_gameplay_rules": True}))


if __name__ == "__main__":
    main()
