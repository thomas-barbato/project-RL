"""Review corrected one-hand placement using the original body glove.

No new sprite is painted: the body and weapon sources are copied unchanged.
The correction is a proposed composition rule, never an active asset edit.
"""
from pathlib import Path
from copy import deepcopy
import hashlib
import json
import runpy
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
ART = ROOT / "assets/sprites/cyberpunk"
HELP = runpy.run_path(str(OUT / "render_equipped_class_review_01.py"), run_name="old_review_helpers")
cell, components = HELP["cell"], HELP["components"]
FONT = "C:/Windows/Fonts/segoeui.ttf"
REVISION = 2
CHANGED_NUMBERS = {2, 4, 5}
# Native coordinates in the actual original right glove, not the old overlays.
RIGHT_ANCHORS = {2: (25, 19), 11: (25, 19), 5: (25, 19)}


def font(size):
    return ImageFont.truetype(FONT, size)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def copy_layer(target, layer, offset=(0, 0)):
    assert set(layer.getchannel("A").tobytes()) <= {0, 255}
    target.paste(layer, offset, layer.getchannel("A"))


def image_record(path):
    return {"path": path.relative_to(ROOT).as_posix(), "binary_sha256": sha(path),
            "dimensions": list(Image.open(path).size)}


def save_image(name, im):
    path = OUT / name
    assert not path.exists(), name
    im.save(path)
    return image_record(path)


def corrected(body, weapon, family, body_index, catalog):
    gx, gy = catalog["weapon_families"][family]["hands"][0]
    bx, by = RIGHT_ANCHORS[body_index]
    dx, dy = 8 + bx - gx, 8 + by - gy
    placed = Image.new("RGBA", (48, 48))
    copy_layer(placed, weapon, (dx, dy))
    assert sum(a > 0 for a in placed.getchannel("A").tobytes()) == sum(a > 0 for a in weapon.getchannel("A").tobytes())
    result = placed.copy()
    # The original hand covers the weapon handle, with no duplicated forearm,
    # no new finger sprite and no rectangular cut in the original body.
    copy_layer(result, body, (8, 8))
    hand = {(8 + x, 8 + y) for y in range(by - 2, by + 2) for x in range(bx - 2, bx + 2)
            if body.getpixel((x, y))[3] and max(body.getpixel((x, y))[:3]) < 65}
    overlap = sorted((x, y) for x, y in hand if placed.getpixel((x, y))[3])
    assert len(overlap) >= 2, (body_index, family, overlap)
    for y in range(32):
        for x in range(32):
            if body.getpixel((x, y))[3]:
                assert result.getpixel((x + 8, y + 8)) == body.getpixel((x, y))
    # No new isolated weapon fragments; original intentionally isolated body
    # pixels can remain, and must not be disguised as newly attached pixels.
    for group in components(result)[1:]:
        assert all(8 <= x < 40 and 8 <= y < 40 and body.getpixel((x - 8, y - 8))[3] for x, y in group), (body_index, group)
    return result, {"native_glove_anchor": [bx, by], "canvas_glove_anchor": [bx + 8, by + 8],
                    "source_weapon_anchor": [gx, gy], "weapon_translation": [dx, dy],
                    "handle_glove_overlap_pixels": len(overlap), "handle_glove_overlap_coordinates": overlap,
                    "original_body_pixel_changes": 0, "added_forearm_or_finger_pixels": 0,
                    "clipped_weapon_pixels": 0, "draw_order": ["weapon", "entire_original_body_and_glove"]}


def main():
    old = json.loads((OUT / "review-equipped-classes-START-VIS01-01.json").read_text(encoding="utf-8"))
    catalog = json.loads((ART / "manifest.json").read_text(encoding="utf-8"))
    bodies = Image.open(ART / "crawl-bodies-32.png").convert("RGBA")
    weapons = Image.open(ART / "crawl-weapons-48.png").convert("RGBA")
    floors = Image.open(ART / "terrain-materials-32.png").convert("RGBA")
    frozen_sources = {entry["path"]: sha(ROOT / entry["path"]) for entry in old["sources"]}
    figures, candidates, corrections = [], [], []
    for original in old["candidates"]:
        number = original["number"]
        body_index, family = original["body_atlas_index"], original["weapon_family_index"]
        prior = Image.open(ROOT / original["native_image"]["path"]).convert("RGBA")
        body, weapon = cell(bodies, body_index, 32), cell(weapons, family, 48)
        if number in CHANGED_NUMBERS:
            image, details = corrected(body, weapon, family, body_index, catalog)
            image_ref = save_image(f"classe-equipee-{number}-START-VIS01-02.png", image)
            entry = {"review_id": original["review_id"], "number": number, "class": original["class"],
                     "body_atlas_index": body_index, "starting_weapon": original["starting_weapon"],
                     "reference_model": original["reference_model"], "weapon_family_index": family,
                     "previous_image": original["native_image"], "native_image": image_ref,
                     "composition": details, "visual_approval": "corrected_candidate_pending_explicit_review",
                     "body_size": [32, 32], "canvas_size": [48, 48], "source_weapon_pixel_or_palette_edits": 0}
            corrections.append(entry)
        else:
            image = prior.copy()
            entry = deepcopy(original)
            entry["visual_approval"] = "unchanged_previous_candidate;no_new_approval_inferred"
        candidates.append(entry)
        figures.append((prior, image))
    sheet = Image.new("RGB", (1000, 676), "#0c131b")
    draw = ImageDraw.Draw(sheet)
    draw.text((20, 12), "START-VIS01 · Révision 2 · Prises à une main", font=font(22), fill="#eef0f2")
    draw.text((20, 48), "Gant noir et bras d’origine conservés · poignée placée dans la main · aucun membre ajouté", font=font(14), fill="#c2ced5")
    for i, ((prior, image), row) in enumerate(zip(figures, candidates)):
        x = 20 + i * 196
        changed = row["number"] in CHANGED_NUMBERS
        draw.rectangle((x, 82, x + 184, 650), fill="#182430", outline="#3b4b59")
        draw.text((x + 9, 92), f"{i + 1}. {row['class']}", font=font(17), fill="#eef0f2")
        draw.text((x + 9, 122), "Prise corrigée" if changed else "Composition inchangée", font=font(12), fill="#a9d9ca" if changed else "#bac7d0")
        draw.text((x + 9, 154), "Avant · zoom ×3", font=font(13), fill="#bac7d0")
        zoom = prior.resize((144, 144), Image.Resampling.NEAREST)
        sheet.paste(zoom, (x + 20, 180), zoom)
        draw.text((x + 9, 335), "Révision 2 · zoom ×3", font=font(13), fill="#d9e5df")
        zoom = image.resize((144, 144), Image.Resampling.NEAREST)
        sheet.paste(zoom, (x + 20, 361), zoom)
        draw.text((x + 9, 516), "Avant / révision 2 · 1:1", font=font(12), fill="#bac7d0")
        sheet.paste(prior, (x + 25, 540), prior)
        sheet.paste(image, (x + 112, 540), image)
        for j, color in enumerate(["#0c1117", "#c7c4b9", "#1e272b"]):
            patch = Image.new("RGBA", (48, 48), color)
            if j == 2:
                for yy in range(2):
                    for xx in range(2):
                        copy_layer(patch, cell(floors, 2 + (xx + yy) % 2, 32), (xx * 32, yy * 32))
            copy_layer(patch, image)
            sheet.paste(patch.convert("RGB"), (x + 9 + j * 55, 594))
    sheet_record = save_image("classes-equipees-START-VIS01-review-02.png", sheet)
    scene = Image.new("RGBA", (576, 384), "#0c131b")
    sd = ImageDraw.Draw(scene)
    sd.text((16, 10), "START-VIS01 · Révision 2 · Taille native", font=font(18), fill="#eef0f2")
    items = Image.open(ART / "crawl-items-32.png").convert("RGBA")
    for section, family in enumerate([1, 2, 0]):
        top = 52 + section * 104
        for yy in range(3):
            for xx in range(18):
                variant = (xx * 17 + yy * 23 + xx * yy % 7) % 4
                copy_layer(scene, cell(floors, family * 4 + variant, 32), (xx * 32, top + yy * 32))
        for i, (_, image) in enumerate(figures):
            cx, cy = 40 + i * 112, top + 42
            copy_layer(scene, image, (cx - 8, cy - 8))
            sd.text((cx + 8, top + 4), str(i + 1), font=font(13), fill="#ffffff", stroke_width=1, stroke_fill="#0c131b")
            icon = cell(items, old["candidates"][i]["item_native_icon_index"], 32)
            copy_layer(scene, icon, (cx + 40, cy + 22))
    sd.text((16, 367), "Montage de revue sur les sols du jeu · avant intégration", font=font(12), fill="#bbc7d0")
    scene_record = save_image("classes-equipees-START-VIS01-scene-native-02.png", scene.convert("RGB"))
    assert frozen_sources == {path: sha(ROOT / path) for path in frozen_sources}
    data = {"date": "2026-10-09", "review_id": "START-VIS01", "revision": REVISION,
            "status": "one_hand_composition_revision_after_user_rejection;not_integrated",
            "previous_visual_review_path": "artifacts/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-01.json",
            "feedback": "Technomancien_pistol_bad_grip;one_hand_melee_has_extra_forearm_and_bright_hand_instead_of_original_black_glove",
            "diagnosis": "old_compose_adds_generic_light_sleeve_and_finger_layers_after_rectangular_hand_cut;grip_contact_QA_did_not_establish_anatomical_or_material_coherence",
            "proposed_composition_rule": "for_one_hand_candidates_keep_complete_original_body_and_glove;translate_original_weapon_handle_to_its_native_glove_anchor;draw_weapon_then_body;do_not_paste_generic_sleeves_or_fingers",
            "changed_candidate_numbers": sorted(CHANGED_NUMBERS), "candidates": candidates,
            "corrected_candidates": corrections, "board": sheet_record, "native_scene": scene_record,
            "sources": old["sources"], "source_asset_changes": 0, "mechanical_changes": "none",
            "scope": "corrected_class_candidates2_4_5;unchanged_candidates1_3_are_not_implicitly_approved;broader_one_hand_family_and_side_review_before_renderer_integration",
            "visual_approval": "pending_explicit_user_review", "runtime_integration": "not_integrated"}
    path = OUT / "review-equipped-classes-START-VIS01-02.json"
    assert not path.exists()
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"changed_candidates": sorted(CHANGED_NUMBERS), "body_pixel_changes": 0,
                      "added_forearm_pixels": 0, "clipped_weapon_pixels": 0, "source_asset_changes": 0,
                      "native_glove_contact_counts": [r["composition"]["handle_glove_overlap_pixels"] for r in corrections]}))


if __name__ == "__main__":
    main()
