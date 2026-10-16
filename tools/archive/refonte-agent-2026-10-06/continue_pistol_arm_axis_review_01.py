"""Link the arm-axis pistol proposal without changing accepted game rules."""
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
VISUAL = "review-equipped-classes-START-VIS01-06.json"
DECISION = "pistol-arm-axis-visual-feedback-decisions-01.json"
BATCH = "review-class-starts-START-ALL01-07.json"
PAIRS = [("review-implementation-readiness-38.json", "review-implementation-readiness-39.json"),
         ("review-attribute-model-96.json", "review-attribute-model-97.json"),
         ("review-primary-catalogue-268.json", "review-primary-catalogue-269.json"),
         ("review-complete-skill-design-243.json", "review-complete-skill-design-244.json"),
         ("remaining-individual-skill-review-counts-137.json", "remaining-individual-skill-review-counts-138.json")]
NEW_FILES = [VISUAL, DECISION, BATCH] + [b for a, b in PAIRS]


def hashes(value):
    checked = 0
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
        checked += sum(hashes(child) for child in value.values())
    elif isinstance(value, list):
        checked += sum(hashes(child) for child in value)
    return checked


def validate():
    checked = sum(hashes(load(name)) for name in NEW_FILES)
    for a, b in PAIRS:
        old, new = load(a), load(b)
        preserve(old, new)
        assert old.get("active_rule_overrides") == new.get("active_rule_overrides")
        assert new["pending_equipped_class_visual_review"] == ref(VISUAL)
        assert new["runtime_integration"] == "not_integrated"
    previous, current = load("review-class-starts-START-ALL01-06.json"), load(BATCH)
    for key in ["profiles", "implant_conditions", "accepted_starting_equipment", "accepted_heavy_weapon_family_direction", "accepted_arcaniste_red_robe_visual"]:
        assert previous[key] == current[key], key
    old_visual, visual = load("review-equipped-classes-START-VIS01-05.json"), load(VISUAL)
    assert visual["changed_candidate_numbers"] == [2]
    for old, new in zip(old_visual["candidates"], visual["candidates"]):
        if old["number"] != 2:
            assert old["native_image"] == new["native_image"]
    tech = visual["candidates"][1]
    image = Image.open(ROOT / tech["native_image"]["path"]).convert("RGBA")
    assert image.size == (48, 48) and set(image.getchannel("A").tobytes()) <= {0, 255}
    details = tech["composition"]
    assert details["rotation_degrees_from_vertical"] == 45
    assert details["original_body_pixel_changes"] == details["added_forearm_or_finger_pixels"] == details["clipped_weapon_pixels"] == 0
    assert details["handle_glove_overlap_pixels"] >= 2 and details["right_overflow_pixels"] > 0
    return checked


def main():
    save(DECISION, {"date": "2026-10-09", "review_id": "START-VIS01-FEEDBACK03",
         "previous_proposal": ref("review-equipped-classes-START-VIS01-05.json"),
         "user_feedback": ["elle suit pas vraiment l'orientation du bras et de la main, c'est à dire que l'arme doit etre dans la meme direction que la bras :) mais si tu n'y arrive pas alors ca n'est pas grave comme ca c'est déjà bien.",
                           "ca n'est pas grave si ca dépasse des 30px , un peu comme les armes à deux mains :)"],
         "requested_direction": "weapon_axis_continues_original_forearm_and_hand_direction;weapon_overflow_allowed",
         "fallback": "previous_vertical_downward_proposal_acceptable_if_alignment_cannot_be_achieved;not_used_as_approval_of_unshown_diagonal_candidate",
         "native_reference": "established32px_body_tile;48px_weapon_composition_canvas;unchanged_collision_and_body_size",
         "revised_proposal": ref(VISUAL), "preserved_red_robe_acceptance": ref("red-robe-visual-acceptance-decisions-01.json"),
         "approval_boundary": "only_new_pistol_orientation2_pending;accepted_red_robe_and_prior_unchanged_visual_parts_preserved",
         "mechanical_changes": "none", "runtime_integration": "not_integrated"})
    old = load("review-class-starts-START-ALL01-06.json")
    batch = deepcopy(old)
    batch.update(revision=7, status="pistol_arm_axis_proposal;vivid_red_robe_accepted;numeric_profiles_unchanged")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-06.json")
    batch["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
    batch["pending_equipped_class_visual_review"] = ref(VISUAL)
    batch["latest_equipped_visual_feedback"] = ref(DECISION)
    preserve(old, batch)
    save(BATCH, batch)
    for a, b in PAIRS:
        old, data = load(a), deepcopy(load(a))
        data["date"] = "2026-10-09"
        data["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
        data["pending_equipped_class_visual_review"] = ref(VISUAL)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["latest_equipped_visual_feedback"] = ref(DECISION)
        data["next_review_kind"] = "START_VIS01_pistol2_parallel_to_original_forearm;vivid_red_robe_already_accepted"
        if a.startswith("review-implementation"):
            data.update(revision=39, status="pistol_arm_axis_review;accepted_red_robe_and_gameplay_preserved")
            data["previous_inventory"] = ref(a)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["previous_pending_equipped_compositions"] = deepcopy(group["pending_equipped_compositions"])
                    group["pending_equipped_compositions"] = ref(VISUAL)
                    group["native_review_requirements"] = "pending_pistol2_parallel_to_forearm;weapon_overflow_allowed;vivid_red_robe4_accepted;other_family_side_group_reviews_and_game_capture_still_required"
            data["recommendation"] = "review_diagonal_pistol2_then_continue_unshown_implant_ammo_and_heavy_model_reviews"
        else:
            if a.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL94", status="pistol_arm_axis_visual_review;accepted_numbers_and_mechanics_preserved")
                pair(data, "previous_model", a)
            elif a.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", a)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", VISUAL)
                data["latest_proposal_scope"] = "pistol_axis_aligned_with_original_forearm;weapon_overflow_allowed;no_gameplay_change"
            else:
                pair(data, "previous_count", a)
                data["latest_feedback"] = "diagonal_pistol2_proposed;accepted_red_robe_and_skill_rules_counts_preserved"
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if not a.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        save(b, data)
    checked = validate()
    save("pistol-arm-axis-visual-validation-01.json", {"date": "2026-10-09", "review_id": "START-VIS01-CHECK04",
         "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
         "body_pixel_changes": 0, "added_forearm_or_finger_pixels": 0, "clipped_weapon_pixels": 0,
         "right_overflow_pixels": 7, "rotation_degrees_from_vertical": 45,
         "accepted_red_robe_and_gameplay": "unchanged", "source_asset_changes": 0,
         "verification_scope": "native_review_composition_and_reference_integrity;not_active_runtime_rendering",
         "runtime_integration": "not_integrated"})
    print(json.dumps({"files_checked": len(NEW_FILES), "hash_checks": checked, "errors": 0,
                      "mechanical_changes": 0, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
