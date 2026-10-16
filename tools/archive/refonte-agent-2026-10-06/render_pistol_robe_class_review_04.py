"""Review composition/orientation and authored material-palette choices only.

Reuse existing weapon pixels, body materials and the engine's red ramp.
No new bitmap art is painted and no active asset or renderer is changed.
"""
from pathlib import Path
from copy import deepcopy
from ctypes import c_float
import hashlib
import json
import math
import runpy
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
ART = ROOT / "assets/sprites/cyberpunk"
HELP = runpy.run_path(str(OUT / "render_fixed_hand_class_review_02.py"), run_name="fixed_hand_helpers")
cell, copy_layer, save_image, font = (HELP[k] for k in ["cell", "copy_layer", "save_image", "font"])
RED_RAMP = [[101, 30, 56], [215, 71, 91], [255, 154, 139]]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def f32(value):
    return c_float(value).value


def red_primary_material(body, mask, ramp=RED_RAMP):
    colored = body.copy()
    changes = 0
    for y in range(32):
        for x in range(32):
            pixel = body.getpixel((x, y))
            if mask.getpixel((x, y))[0] != 1 or not pixel[3]:
                continue
            value = f32(max(pixel[:3]) / f32(255.0))
            if value < f32(0.53):
                low, high = ramp[:2]
                t = min(1.0, max(0.0, f32(f32(value - f32(0.12)) / f32(0.41))))
            else:
                low, high = ramp[1:]
                t = min(1.0, max(0.0, f32(f32(value - f32(0.53)) / f32(0.47))))
            rgb = [math.floor(f32(f32(low[n] * f32(1.0 - t)) + f32(high[n] * t)) + 0.5) for n in range(3)]
            colored.putpixel((x, y), tuple(rgb) + (pixel[3],))
            changes += colored.getpixel((x, y)) != pixel
    assert colored.getchannel("A").tobytes() == body.getchannel("A").tobytes()
    for y in range(32):
        for x in range(32):
            if mask.getpixel((x, y))[0] != 1:
                assert colored.getpixel((x, y)) == body.getpixel((x, y))
    assert changes > 0
    return colored, {"primary_material": "existing_Red_ramp" if ramp == RED_RAMP else "proposed_vivid_red_ramp", "red_ramp": ramp,
                     "changed_primary_material_pixels": changes, "geometry_changes": 0,
                     "secondary_ivory_and_protected_pixel_changes": 0,
                     "weapon_palette_changes": 0,
                     "selection": "authored_material_label1_only;label2_ivory_and_label0_unchanged",
                     "scope": "review_prototype;existing_full_Red_palette_also_changes_secondary_material_so_not_a_runtime_capture"}


def compose(body, weapon, anchor):
    target_anchor = (33, 27)
    dx, dy = target_anchor[0] - anchor[0], target_anchor[1] - anchor[1]
    placed = Image.new("RGBA", (48, 48))
    copy_layer(placed, weapon, (dx, dy))
    assert sum(a > 0 for a in placed.getchannel("A").tobytes()) == sum(a > 0 for a in weapon.getchannel("A").tobytes())
    result = placed.copy()
    copy_layer(result, body, (8, 8))
    overlap = []
    for y in range(17, 21):
        for x in range(23, 27):
            pixel = body.getpixel((x, y))
            if pixel[3] and max(pixel[:3]) < 65 and placed.getpixel((x + 8, y + 8))[3]:
                overlap.append([x + 8, y + 8])
    assert len(overlap) >= 2
    for y in range(32):
        for x in range(32):
            if body.getpixel((x, y))[3]:
                assert result.getpixel((x + 8, y + 8)) == body.getpixel((x, y))
    for group in HELP["components"](result)[1:]:
        assert all(8 <= x < 40 and 8 <= y < 40 and body.getpixel((x - 8, y - 8))[3] for x, y in group)
    return result, {"native_glove_anchor": [25, 19], "canvas_glove_anchor": list(target_anchor),
                    "transformed_weapon_anchor": list(anchor), "weapon_translation": [dx, dy],
                    "handle_glove_overlap_pixels": len(overlap), "handle_glove_overlap_coordinates": overlap,
                    "composition_body_pixel_changes": 0, "added_forearm_or_finger_pixels": 0,
                    "clipped_weapon_pixels": 0, "draw_order": ["weapon", "entire_body_and_original_glove"]}


def main(revision=4, red_ramp=RED_RAMP):
    old = json.loads((OUT / "review-equipped-classes-START-VIS01-03.json").read_text(encoding="utf-8"))
    catalog = json.loads((ART / "manifest.json").read_text(encoding="utf-8"))
    sources = [ART / name for name in ["crawl-bodies-32.png", "crawl-body-materials-32.png", "crawl-weapons-48.png", "terrain-materials-32.png", "manifest.json"]]
    frozen = {path: sha(path) for path in sources}
    bodies = Image.open(sources[0]).convert("RGBA")
    materials = Image.open(sources[1]).convert("RGBA")
    weapons = Image.open(sources[2]).convert("RGBA")
    floors = Image.open(sources[3]).convert("RGBA")
    rows = deepcopy(old["candidates"])
    for row in rows:
        if row["number"] == 2:
            body = cell(bodies, 2, 32)
            original_weapon = cell(weapons, 6, 48)
            weapon = original_weapon.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
            source_anchor = catalog["weapon_families"][6]["hands"][0]
            anchor = (source_anchor[0], 47 - source_anchor[1])
            image, details = compose(body, weapon, anchor)
            details.update(original_body_pixel_changes=0, source_weapon_anchor=source_anchor,
                           orientation="muzzle_down;original_lowered_arm_and_glove_preserved",
                           weapon_transform="reflect_Y_on_48px_canvas;align_transformed_handle_to_original_glove",
                           source_weapon_color_or_pixel_count_changes=0)
            bbox = weapon.getbbox()
            details["placed_weapon_bounds"] = [bbox[0] + details["weapon_translation"][0], bbox[1] + details["weapon_translation"][1], bbox[2] + details["weapon_translation"][0], bbox[3] + details["weapon_translation"][1]]
            assert details["placed_weapon_bounds"][3] - 1 > 27
            row.update(previous_image=deepcopy(row["native_image"]),
                       native_image=save_image(f"classe-equipee-2-START-VIS01-{revision:02d}.png", image), composition=details,
                       visual_approval="downward_pistol_orientation_pending;prior_body_and_black_glove_acceptance_preserved")
        elif row["number"] == 4:
            body, color_details = red_primary_material(cell(bodies, 11, 32), cell(materials, 11, 32), red_ramp)
            image, details = compose(body, cell(weapons, 0, 48), catalog["weapon_families"][0]["hands"][0])
            row.update(previous_image=deepcopy(row["native_image"]),
                       native_image=save_image(f"classe-equipee-4-START-VIS01-{revision:02d}.png", image), composition=details,
                       material_palette_proposal=color_details,
                       visual_approval="red_robe_primary_material_pending;prior_shape_and_knife_grip_acceptance_preserved")
        elif row["number"] in (3, 5):
            row["visual_approval"] = "shown_revision3_accepted_by_user_je_valide;unchanged_in_revision4"
    board = Image.new("RGB", (740, 540), "#0c131b")
    draw = ImageDraw.Draw(board)
    draw.text((18, 10), f"START-VIS01 · Révision {revision} · Nouveaux aperçus", font=font(22), fill="#eef0f2")
    draw.text((18, 43), "Corps natifs 32 × 32 · armes en calques · propositions avant intégration", font=font(14), fill="#c2ced5")
    for column, index in enumerate((1, 3)):
        row, before = rows[index], old["candidates"][index]
        x = 18 + column * 368
        prior = Image.open(ROOT / before["native_image"]["path"]).convert("RGBA")
        current = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
        draw.rectangle((x, 78, x + 348, 399), fill="#182430", outline="#3b4b59")
        draw.text((x + 10, 88), f"{row['number']}. {row['class']}", font=font(18), fill="#eef0f2")
        draw.text((x + 10, 120), "Précédent · ×3", font=font(13), fill="#bbc7d0")
        draw.text((x + 185, 120), "Canon vers le bas · ×3" if index == 1 else "Robe rouge vif · ×3" if revision == 5 else "Robe rouge · ×3", font=font(13), fill="#aed9c4")
        for image, offset in ((prior, 10), (current, 185)):
            zoom = image.resize((144, 144), Image.Resampling.NEAREST)
            board.paste(zoom, (x + offset, 147), zoom)
        draw.text((x + 10, 303), "Avant / après · 1:1", font=font(12), fill="#bbc7d0")
        board.paste(prior, (x + 10, 330), prior)
        board.paste(current, (x + 75, 330), current)
        for j, color in enumerate(["#0c1117", "#c7c4b9", "#1e272b"]):
            patch = Image.new("RGBA", (48, 48), color)
            if j == 2:
                for yy in range(2):
                    for xx in range(2):
                        copy_layer(patch, cell(floors, 2 + (xx + yy) % 2, 32), (xx * 32, yy * 32))
            copy_layer(patch, current)
            board.paste(patch.convert("RGB"), (x + 181 + j * 55, 330))
    draw.text((18, 416), "Ensemble · 1:1 sur le sol du jeu", font=font(14), fill="#c2ced5")
    for yy in range(2):
        for xx in range(23):
            tile = cell(floors, 4 + (xx * 3 + yy) % 4, 32)
            board.paste(tile.convert("RGB"), (xx * 32, 454 + yy * 32))
    for i, row in enumerate(rows):
        image = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
        x = 49 + i * 142
        board.paste(image, (x, 458), image)
        draw.text((x + 16, 442), str(row["number"]), font=font(12), fill="#ffffff")
    board_ref = save_image(f"classes-equipees-START-VIS01-review-{revision:02d}.png", board)
    scene = Image.new("RGBA", (576, 384), "#0c131b")
    sd = ImageDraw.Draw(scene)
    sd.text((16, 10), f"START-VIS01 · Révision {revision} · Taille native", font=font(18), fill="#eef0f2")
    for section, family in enumerate([1, 2, 0]):
        top = 52 + section * 104
        for yy in range(3):
            for xx in range(18):
                variant = (xx * 17 + yy * 23 + xx * yy % 7) % 4
                copy_layer(scene, cell(floors, family * 4 + variant, 32), (xx * 32, top + yy * 32))
        for i, row in enumerate(rows):
            image = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
            cx, cy = 40 + i * 112, top + 42
            copy_layer(scene, image, (cx - 8, cy - 8))
            sd.text((cx + 8, top + 4), str(row["number"]), font=font(13), fill="#ffffff", stroke_width=1, stroke_fill="#0c131b")
    sd.text((16, 367), "Montage de revue sur les sols du jeu · avant intégration", font=font(12), fill="#bbc7d0")
    scene_ref = save_image(f"classes-equipees-START-VIS01-scene-native-{revision:02d}.png", scene.convert("RGB"))
    assert frozen == {path: sha(path) for path in sources}
    data = {"date": "2026-10-09", "review_id": "START-VIS01", "revision": revision,
            "status": "downward_pistol_and_red_robe_palette_proposed;not_integrated",
            "previous_visual_review_path": f"artifacts/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-{revision - 1:02d}.json",
            "comparison_images_source_revision": 3,
            "changed_candidate_numbers": [2, 4], "unchanged_candidate_numbers": [1, 3, 5],
            "candidates": rows, "board": board_ref, "native_scene": scene_ref,
            "method": "existing_code_native_layer_transform_and_authored_material_map_palette_selection;no_new_art_painting",
            "source_asset_checks": [{"path": p.relative_to(ROOT).as_posix(), "binary_sha256": h} for p, h in frozen.items()],
            "active_source_asset_changes": 0, "mechanical_changes": "none",
            "visual_approval": "new_downward_orientation2_and_red_primary_material4_pending_explicit_review",
            "approval_scope": "prior_acceptance_of_shown_revision3_compositions_is_preserved_except_pistol_orientation2_and_new_robe_palette4;unchanged1_and_unshown_weapon_configurations_not_inferred_approved",
            "runtime_integration": "not_integrated"}
    if revision == 5:
        previous = json.loads((OUT / "review-equipped-classes-START-VIS01-04.json").read_text(encoding="utf-8"))
        data["changed_candidate_numbers"] = [4]
        data["unchanged_candidate_numbers"] = [1, 2, 3, 5]
        data["carried_pending_candidate_numbers"] = [2]
        data["user_palette_feedback"] = "le rouge n'est pas assez petant :)"
        data["status"] = "vivid_red_robe_revision;downward_pistol_proposal_preserved;not_integrated"
        rows[3]["previous_image"] = previous["candidates"][3]["native_image"]
        rows[3]["visual_approval"] = "vivid_red_robe_primary_material_pending;previous_muted_red_superseded"
        assert Image.open(ROOT / previous["candidates"][1]["native_image"]["path"]).tobytes() == Image.open(ROOT / rows[1]["native_image"]["path"]).tobytes()
    target = OUT / f"review-equipped-classes-START-VIS01-{revision:02d}.json"
    assert not target.exists()
    target.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"changed_candidates": [2, 4], "pistol_body_pixel_changes": 0,
                      "red_robe_pixel_changes": rows[3]["material_palette_proposal"]["changed_primary_material_pixels"],
                      "grip_contacts": [rows[i]["composition"]["handle_glove_overlap_pixels"] for i in (1, 3)],
                      "clipped_weapon_pixels": 0, "active_asset_changes": 0, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
