"""Record accepted directions and the full weapon review, preserving game rules."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
BANK = "weapon-silhouette-review-03/review-weapon-silhouettes-WEAPON-SIL01-03.json"
PRESENTATION = "weapon-silhouette-review-03/weapon-silhouette-review-presentation-02.json"
DECISION = "weapon-gray-and-family-adaptation-decisions-01.json"
VISUAL = "review-equipped-classes-START-VIS01-07.json"
BATCH = "review-class-starts-START-ALL01-08.json"
PAIRS = [("review-implementation-readiness-39.json", "review-implementation-readiness-40.json"),
         ("review-attribute-model-97.json", "review-attribute-model-98.json"),
         ("review-primary-catalogue-269.json", "review-primary-catalogue-270.json"),
         ("review-complete-skill-design-244.json", "review-complete-skill-design-245.json"),
         ("remaining-individual-skill-review-counts-138.json", "remaining-individual-skill-review-counts-139.json")]
H = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (H[k] for k in ["load", "ref", "pair", "save", "preserve"])
ART = runpy.run_path(str(BASE / "render_equipped_class_review_01.py"), run_name="art_helpers")
font, cell, overlay = (ART[k] for k in ["font", "cell", "overlay"])
HASHES = runpy.run_path(str(BASE / "continue_pistol_arm_axis_review_01.py"), run_name="hash_helpers")["hashes"]
NEW_FILES = [VISUAL, DECISION, BATCH, BANK, PRESENTATION] + [b for _, b in PAIRS]


def save_png(name, image):
    path = BASE / name
    assert not path.exists(), name
    image.save(path)
    return {"path": path.relative_to(ROOT).as_posix(),
            "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "dimensions": list(image.size)}


def prepare_starts():
    bank, old = load(BANK), load("review-equipped-classes-START-VIS01-06.json")
    frames = {(row["class_number"], row["left_family"], row["right_family"]): row for row in bank["frames"]}
    candidates, images = [], []
    for prior in old["candidates"]:
        number = prior["number"]
        row = frames[(number, None, prior["weapon_family_index"])]
        candidate = {key: deepcopy(prior[key]) for key in ["review_id", "number", "class", "starting_weapon", "reference_model", "weapon_family_index"]}
        for key in ["weapon_hold", "item_native_icon_index"]:
            if key in prior:
                candidate[key] = prior[key]
        candidate.update(previous_image=deepcopy(prior["native_image"]), native_image=deepcopy(row["native_image"]),
                         body_layer_reference=deepcopy(bank["classes"][number - 1]["body_reference"]),
                         body_source=deepcopy(bank["classes"][number - 1]["body_source"]),
                         composition=deepcopy(row["qa"]), body_size=[32, 32], canvas_size=[64, 64])
        candidate["visual_approval"] = {
            1: "new_two_hand_arm_composition_pending;prior_body_identity_preserved",
            2: "prior_diagonal_pistol_orientation_accepted;gray_material_pending;original_arm_and_glove_preserved",
            3: "new_fixed_unarmed_layer_and_family_rifle_composition_pending;steel_anthracite_direction_preserved;historical_equipped_pixel_identity_not_claimed",
            4: "accepted_vivid_red_robe_and_knife_composition_preserved;only_transparent_canvas_padding_changed",
            5: "accepted_body_and_knife_composition_preserved;only_transparent_canvas_padding_changed"
        }[number]
        if number in (4, 5):
            previous = Image.open(ROOT / prior["native_image"]["path"]).convert("RGBA")
            padded = Image.new("RGBA", (64, 64))
            overlay(padded, previous, (8, 8))
            current = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
            assert padded.tobytes() == current.tobytes(), number
            candidate["approved_composition_pixel_changes_after_padding"] = 0
        candidates.append(candidate)
        images.append(Image.open(ROOT / row["native_image"]["path"]).convert("RGBA"))
    board = Image.new("RGB", (1000, 360), "#0c131b")
    draw = ImageDraw.Draw(board)
    draw.text((18, 12), "START-VIS01 · Prises de départ et pistolet gris", font=font(22), fill="#eef0f2")
    draw.text((18, 48), "Corps natifs 32 × 32 · compositions par couches · nouvelles prises proposées", font=font(14), fill="#c2ced5")
    for i, (candidate, image) in enumerate(zip(candidates, images)):
        x = 18 + 196 * i
        draw.rectangle((x, 82, x + 182, 342), fill="#182430", outline="#3b4b59")
        draw.text((x + 8, 92), f"{i + 1}. {candidate['class']}", font=font(16), fill="#eef0f2")
        board.paste(image, (x + 59, 130), image)
        zoom = image.resize((128, 128), Image.Resampling.NEAREST)
        board.paste(zoom, (x + 27, 198), zoom)
        draw.text((x + 8, 116), "Taille native / zoom ×2", font=font(11), fill="#aec1cc")
    board_ref = save_png("classes-equipees-START-VIS01-review-07.png", board)
    # Native scene, reusing unchanged ground tiles and item illustrations.
    floors = Image.open(ROOT / "assets/sprites/cyberpunk/terrain-materials-32.png").convert("RGBA")
    items = Image.open(ROOT / "assets/sprites/cyberpunk/crawl-items-32.png").convert("RGBA")
    item_indices = [row["item_native_icon_index"] for row in load("review-equipped-classes-START-VIS01-01.json")["candidates"]]
    scene = Image.new("RGBA", (576, 384), "#0c131b")
    sd = ImageDraw.Draw(scene)
    sd.text((16, 10), "START-VIS01 · Prises de départ à taille native", font=font(18), fill="#eef0f2")
    for section, family in enumerate([1, 2, 0]):
        top = 52 + section * 104
        for yy in range(3):
            for xx in range(18):
                variant = (xx * 17 + yy * 23 + xx * yy % 7) % 4
                overlay(scene, cell(floors, family * 4 + variant, 32), (xx * 32, top + yy * 32))
        for i, figure in enumerate(images):
            cx, cy = 40 + i * 112, top + 42
            overlay(scene, figure, (cx - 16, cy - 16))
            sd.text((cx + 8, top + 4), str(i + 1), font=font(13), fill="#ffffff", stroke_width=1, stroke_fill="#0c131b")
            overlay(scene, cell(items, item_indices[i], 32), (cx + 40, cy + 22))
    sd.text((16, 367), "Montage de revue sur les sols du jeu · aucune intégration", font=font(12), fill="#bbc7d0")
    scene_ref = save_png("classes-equipees-START-VIS01-scene-native-07.png", scene.convert("RGB"))
    save(VISUAL, {"date": "2026-10-09", "review_id": "START-VIS01", "revision": 7,
                  "previous_visual_review": ref("review-equipped-classes-START-VIS01-06.json"),
                  "status": "full_weapon_family_bank_linked;light_gray_pistol_and_new_rigs_pending",
                  "changed_candidate_numbers": [1, 2, 3], "unchanged_compositions_with_larger_padding": [4, 5],
                  "candidates": candidates, "board": board_ref, "native_scene": scene_ref,
                  "full_weapon_family_review": ref(BANK), "full_review_presentation": ref(PRESENTATION),
                  "previous_pistol_orientation_accepted": True, "vivid_red_robe_acceptance": ref("red-robe-visual-acceptance-decisions-01.json"),
                  "method": "existing_body_weapon_arm_material_layers_and_attachment_transforms;no_new_bitmap_painting",
                  "source_asset_checks": deepcopy(bank["sources"]), "active_source_asset_changes": 0,
                  "mechanical_changes": "none", "visual_approval": "new_compositions_and_gray_palette_pending;existing_accepted_parts_preserved",
                  "runtime_integration": "not_integrated"})


def main():
    prepare_starts()
    save(DECISION, {"date": "2026-10-09", "review_id": "WEAPON-SIL01-DIRECTION01",
                   "user_messages": ["oui c'est mieux, maintenant on peut adapter toutes les armes du jeu aux silhouettes :) (et si tu peux aussi refaire la couleur de ce pistolet (plutot gris clair)",
                                     "je sais qu'on a créé pleins d'arme de couleurs differentes (et d'armures de la meme facon) on devrait surement eviter le rose (et supprimer les armures / armes roses )",
                                     "Un gris neutre",
                                     "et bien-sur il faudra faire la meme chose (adapter les armes) pour toutes les armes :) mais je crois pas que ca sera un probleme grace aux regles mises en place :)"],
                   "accepted_diagonal_pistol_orientation": {"candidate": "START-VIS01-2", "source_proposal": ref("review-equipped-classes-START-VIS01-06.json"), "user_acceptance": "oui c'est mieux"},
                   "accepted_directions": {"weapon_coverage": "all_existing_and_future_weapons_use_compatible_family_layers_and_silhouette_attachments;new_shapes_reviewed_when_added",
                                           "pistol_material": "light_gray;approved_arm_glove_and_diagonal_orientation_preserved",
                                           "existing_pink_equipment": "neutral_gray_replaces_Pink_cosmetic_variant;models_ids_stats_bonuses_preserved",
                                           "violet": "distinct_existing_violet_palette_preserved"},
                   "prepared_weapon_review": ref(BANK), "presentation": ref(PRESENTATION),
                   "approval_boundary": "accepted_directions_and_prior_pistol_orientation_only;new_shown_gray_materials_family_rigs_and_Traqueur_unarmed_layer_require_explicit_visual_review",
                   "mechanical_changes": "none", "runtime_integration": "not_integrated"})
    old = load("review-class-starts-START-ALL01-07.json")
    batch = deepcopy(old)
    batch.update(revision=8, status="all_weapon_family_review_prepared;gray_directions_accepted;new_rendered_candidates_pending")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-07.json")
    batch["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
    batch["pending_equipped_class_visual_review"] = ref(VISUAL)
    batch["pending_full_weapon_visual_review"] = ref(BANK)
    batch["pending_weapon_review_presentation"] = ref(PRESENTATION)
    batch["latest_equipped_visual_feedback"] = ref(DECISION)
    batch["accepted_weapon_adaptation_and_gray_directions"] = ref(DECISION)
    preserve(old, batch)
    save(BATCH, batch)
    for a, b in PAIRS:
        old, data = load(a), deepcopy(load(a))
        data["date"] = "2026-10-09"
        data["previous_pending_equipped_class_visual_review"] = deepcopy(old["pending_equipped_class_visual_review"])
        data["pending_equipped_class_visual_review"] = ref(VISUAL)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["pending_full_weapon_visual_review"] = ref(BANK)
        data["pending_weapon_review_presentation"] = ref(PRESENTATION)
        data["accepted_weapon_adaptation_and_gray_directions"] = ref(DECISION)
        data["latest_equipped_visual_feedback"] = ref(DECISION)
        data["next_review_kind"] = "WEAPON_SIL01_full_family_and_gray_visual_review;prior_pistol_orientation_accepted"
        if a.startswith("review-implementation"):
            data.update(revision=40, status="all_weapon_visual_review_prepared;accepted_gameplay_preserved")
            data["previous_inventory"] = ref(a)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["previous_pending_equipped_compositions"] = deepcopy(group["pending_equipped_compositions"])
                    group["pending_equipped_compositions"] = ref(VISUAL)
                    group["full_weapon_family_review"] = ref(BANK)
                    group["native_review_requirements"] = "new_weapon_rigs_gray_materials_and_Traqueur_unarmed_layer_pending;113_models_18_families_5_classes_375_configurations_prepared;live_game_capture_after_acceptance"
            data["recommendation"] = "review_complete_weapon_gallery_and_neutral_gray_palette_then_continue_implementation_of_accepted_assets_and_pending_implant_ammo_heavy_model_designs"
        else:
            if a.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL95", status="full_weapon_visual_review;accepted_stats_and_mechanics_preserved")
                pair(data, "previous_model", a)
            elif a.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", a)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", VISUAL)
                data["latest_proposal_scope"] = "complete_held_weapon_families_on_five_silhouettes;gray_palette_replaces_Pink;no_gameplay_change"
            else:
                pair(data, "previous_count", a)
                data["latest_feedback"] = "full_weapon_adaptation_and_gray_directions_accepted;concrete_compositions_pending;skill_review_counts_preserved"
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if not a.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        save(b, data)
    checked = sum(HASHES(load(name)) for name in NEW_FILES)
    for a, b in PAIRS:
        old, current = load(a), load(b)
        preserve(old, current)
        assert old.get("active_rule_overrides") == current.get("active_rule_overrides")
    previous, current = load("review-class-starts-START-ALL01-07.json"), load(BATCH)
    for key in ["profiles", "implant_conditions", "accepted_starting_equipment", "accepted_heavy_weapon_family_direction", "accepted_arcaniste_red_robe_visual"]:
        assert previous[key] == current[key], key
    bank = load(BANK)
    assert len(bank["frames"]) == 375 and len(bank["model_mappings"]) == 113
    for row in bank["frames"]:
        image = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
        assert image.size == (64, 64) and set(image.getchannel("A").tobytes()) <= {0, 255}
    for source in load(PRESENTATION)["palette_code_patch_sources"]:
        assert hashlib.sha256((ROOT / source["path"]).read_text(encoding="utf-8").encode("utf-8")).hexdigest() == source["current_content_sha256"]
    save("weapon-silhouette-review-validation-01.json", {"date": "2026-10-09", "review_id": "WEAPON-SIL01-CHECK01",
         "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
         "models": 113, "families": 18, "classes": 5, "configurations": 375,
         "source_asset_or_palette_patch_target_changes": 0, "accepted_gameplay": "unchanged",
         "browser_validation": ref("weapon-silhouette-review-03/gallery-browser-validation-01.json"),
         "verification_scope": "native_proposal_and_browser_gallery;prepared_unapplied_palette_patch;not_live_game_capture",
         "runtime_integration": "not_integrated"})
    print(json.dumps({"files_checked": len(NEW_FILES), "hash_checks": checked, "errors": 0,
                      "models": 113, "configurations": 375, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
