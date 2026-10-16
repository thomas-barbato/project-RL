"""Align the existing pistol layer with the original forearm; review only."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import math
import runpy
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
ART = ROOT / "assets/sprites/cyberpunk"
H = runpy.run_path(str(OUT / "render_pistol_robe_class_review_04.py"), run_name="composition_helpers")
cell, copy_layer, save_image, font = (H[k] for k in ["cell", "copy_layer", "save_image", "font"])


def main():
    old = json.loads((OUT / "review-equipped-classes-START-VIS01-05.json").read_text(encoding="utf-8"))
    sources = [ART / name for name in ["crawl-bodies-32.png", "crawl-weapons-48.png", "terrain-materials-32.png"]]
    frozen = {path: hashlib.sha256(path.read_bytes()).hexdigest() for path in sources}
    body = cell(Image.open(sources[0]).convert("RGBA"), 2, 32)
    weapon = cell(Image.open(sources[1]).convert("RGBA"), 6, 48).transpose(Image.Transpose.FLIP_TOP_BOTTOM)
    # Measure native blue forearm and dark glove, never move or redraw either.
    elbow, glove = (22, 16), (25, 19)
    assert body.getpixel(elbow)[3] and body.getpixel(glove)[3]
    assert max(body.getpixel(glove)[:3]) < 65
    angle = math.degrees(math.atan2(glove[0] - elbow[0], glove[1] - elbow[1]))
    # Rotate on a spacious scratch layer around the handle. Crop only after
    # placement, and check that no sampled weapon pixel is lost in the canvas.
    scratch = Image.new("RGBA", (96, 96))
    copy_layer(scratch, weapon, (48 - 35, 48 - 25))
    rotated = scratch.rotate(angle, resample=Image.Resampling.NEAREST, center=(48, 48))
    placed = Image.new("RGBA", (48, 48))
    copy_layer(placed, rotated, (33 - 48, 27 - 48))
    sampled_pixels = sum(a > 0 for a in rotated.getchannel("A").tobytes())
    assert sampled_pixels == sum(a > 0 for a in placed.getchannel("A").tobytes())
    assert {p[:3] for p in rotated.get_flattened_data() if p[3]} <= {p[:3] for p in weapon.get_flattened_data() if p[3]}
    image, details = H["compose"](body, placed, (33, 27))
    details.update(original_body_pixel_changes=0, orientation="down_and_outward;weapon_axis_parallel_to_measured_forearm",
                   native_elbow=elbow, native_glove=glove, forearm_direction=[3, 3],
                   rotation_degrees_from_vertical=angle, rotation_filter="nearest;binary_alpha;no_new_colors",
                   source_weapon_transform="vertical_reflection_then_handle_pivot_rotation",
                   source_weapon_pixel_count=sum(a > 0 for a in weapon.getchannel("A").tobytes()),
                   rotated_weapon_pixel_count=sampled_pixels, placed_weapon_bounds=list(placed.getbbox()),
                   native_body_rect=[8, 8, 40, 40], weapon_may_extend_outside_body_tile=True,
                   right_overflow_pixels=placed.getbbox()[2] - 40,
                   source_weapon_pixel_count_identity="not_claimed;nearest_rotation_resamples_existing_layer")
    assert details["right_overflow_pixels"] > 0
    assert image.getpixel((33, 27)) == body.getpixel(glove)
    assert set(image.getchannel("A").tobytes()) <= {0, 255}
    native_ref = save_image("classe-equipee-2-START-VIS01-06.png", image)
    prior = Image.open(ROOT / old["candidates"][1]["native_image"]["path"]).convert("RGBA")
    floors = Image.open(sources[2]).convert("RGBA")
    board = Image.new("RGB", (640, 470), "#0c131b")
    draw = ImageDraw.Draw(board)
    draw.text((18, 12), "2. Technomancien · Pistolet dans l’axe du bras", font=font(22), fill="#eef0f2")
    draw.text((18, 47), "Corps natif 32 × 32 · débordement de l’arme conservé · proposition", font=font(14), fill="#c2ced5")
    for x, figure, label in [(20, prior, "Précédent · vertical"), (340, image, "Nouvel aperçu · diagonale")]:
        draw.rectangle((x, 80, x + 278, 454), fill="#182430", outline="#3b4b59")
        draw.text((x + 14, 92), label, font=font(16), fill="#bbc7d0" if x == 20 else "#aed9c4")
        zoom = figure.resize((192, 192), Image.Resampling.NEAREST)
        board.paste(zoom, (x + 43, 123), zoom)
        draw.text((x + 14, 325), "Zoom ×4 · ci-dessous : taille native 1:1", font=font(12), fill="#bbc7d0")
        for j, color in enumerate(["#0c1117", "#c7c4b9", "#1e272b"]):
            patch = Image.new("RGBA", (64, 64), color)
            if j == 2:
                for yy in range(2):
                    for xx in range(2):
                        copy_layer(patch, cell(floors, 4 + (xx + yy) % 4, 32), (xx * 32, yy * 32))
            copy_layer(patch, figure, (8, 8))
            board.paste(patch.convert("RGB"), (x + 20 + j * 85, 361))
    board_ref = save_image("classes-equipees-START-VIS01-review-06.png", board)
    rows = deepcopy(old["candidates"])
    rows[1].update(previous_image=deepcopy(rows[1]["native_image"]), native_image=native_ref, composition=details,
                   visual_approval="arm_axis_pistol_orientation_pending_explicit_visual_review")
    rows[3]["visual_approval"] = "vivid_red_robe_accepted_by_user_beaucoup_mieux_oui;unchanged_in_revision6"
    scene = Image.new("RGBA", (576, 384), "#0c131b")
    sd = ImageDraw.Draw(scene)
    sd.text((16, 10), "START-VIS01 · Révision 6 · Taille native", font=font(18), fill="#eef0f2")
    for section, family in enumerate([1, 2, 0]):
        top = 52 + section * 104
        for yy in range(3):
            for xx in range(18):
                variant = (xx * 17 + yy * 23 + xx * yy % 7) % 4
                copy_layer(scene, cell(floors, family * 4 + variant, 32), (xx * 32, top + yy * 32))
        for i, row in enumerate(rows):
            actor = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
            cx, cy = 40 + i * 112, top + 42
            copy_layer(scene, actor, (cx - 8, cy - 8))
            sd.text((cx + 8, top + 4), str(row["number"]), font=font(13), fill="#ffffff", stroke_width=1, stroke_fill="#0c131b")
    sd.text((16, 367), "Montage de revue sur les sols du jeu · avant intégration", font=font(12), fill="#bbc7d0")
    scene_ref = save_image("classes-equipees-START-VIS01-scene-native-06.png", scene.convert("RGB"))
    assert frozen == {path: hashlib.sha256(path.read_bytes()).hexdigest() for path in sources}
    data = {"date": "2026-10-09", "review_id": "START-VIS01", "revision": 6,
            "previous_visual_review_path": "artifacts/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-05.json",
            "status": "pistol_aligned_to_original_forearm;accepted_vivid_red_robe_preserved;not_integrated",
            "changed_candidate_numbers": [2], "unchanged_candidate_numbers": [1, 3, 4, 5],
            "candidates": rows, "board": board_ref, "native_scene": scene_ref,
            "method": "code_native_weapon_layer_rotation_about_grip;authored_body_unchanged;no_new_bitmap_painting",
            "source_asset_checks": [{"path": path.relative_to(ROOT).as_posix(), "binary_sha256": sha} for path, sha in frozen.items()],
            "active_source_asset_changes": 0, "mechanical_changes": "none",
            "visual_approval": "revised_orientation2_pending;accepted_vivid_red_robe4_and_prior_unchanged_parts_preserved",
            "overflow_scope": "user_explicitly_accepts_weapon_overflow_beyond_body_tile;no_actor_collision_or_tile_size_change",
            "runtime_integration": "not_integrated"}
    target = OUT / "review-equipped-classes-START-VIS01-06.json"
    assert not target.exists()
    target.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"rotation_degrees": angle, "glove_contacts": details["handle_glove_overlap_pixels"],
                      "body_pixel_changes": 0, "right_overflow_pixels": details["right_overflow_pixels"],
                      "clipped_weapon_pixels": 0, "active_asset_changes": 0, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
