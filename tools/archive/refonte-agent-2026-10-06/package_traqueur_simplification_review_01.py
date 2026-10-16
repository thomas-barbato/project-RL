"""Condition generated review sources and assemble numbered native comparisons.

Nearest sampling, binary-alpha conditioning and bounded palette packaging only.
No active asset or renderer is changed; the new Traqueur is a pose prototype.
"""
from pathlib import Path
from copy import deepcopy
import hashlib
import json
import runpy
import shutil
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
ART = ROOT / "assets/sprites/cyberpunk"
GEN = Path("C:/Users/User/.codex/generated_images/01a11010-433f-7fc2-ac29-ffb696dec5ea")
HELP = runpy.run_path(str(OUT / "render_fixed_hand_class_review_02.py"), run_name="fixed_hand_helpers")
cell, copy_layer, save_image, font = (HELP[k] for k in ["cell", "copy_layer", "save_image", "font"])


def binary_ref(path):
    return {"path": path.relative_to(ROOT).as_posix(),
            "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def condition(source, side, colors=12):
    im = Image.open(source).convert("RGBA").resize((side, side), Image.Resampling.NEAREST)
    alpha = im.getchannel("A").point(lambda value: 255 if value >= 128 else 0)
    data = [(r, g, b, 255) if a else (0, 0, 0, 0)
            for (r, g, b, _), a in zip(im.get_flattened_data(), alpha.get_flattened_data())]
    im.putdata(data)
    # The generated artwork has the simple material masses; packing fixes the
    # solid alpha and limits its sampled colors without inventing new shapes.
    result = im.convert("RGB").quantize(colors, method=Image.Quantize.MEDIANCUT,
                                        dither=Image.Dither.NONE).convert("RGBA")
    result.putalpha(alpha)
    result.putdata([p if p[3] else (0, 0, 0, 0) for p in result.get_flattened_data()])
    assert set(result.getchannel("A").tobytes()) <= {0, 255}
    assert len({p[:3] for p in result.get_flattened_data() if p[3]}) <= colors
    assert result.getbbox()
    return result


def main():
    for source, target in [("exec-5662285b-d184-45c3-97fa-1c58c285aed3.png", "traqueur-equipped-source-03.png"),
                           ("exec-a0e87202-9125-4022-b086-5f4e782f72bb.png", "traqueur-equipped-dark-gloves-source-04.png")]:
        path = OUT / target
        assert not path.exists(), target
        shutil.copyfile(GEN / source, path)
    old = json.loads((OUT / "review-equipped-classes-START-VIS01-01.json").read_text(encoding="utf-8"))
    previous = json.loads((OUT / "review-equipped-classes-START-VIS01-02.json").read_text(encoding="utf-8"))
    source = OUT / "traqueur-equipped-dark-gloves-source-04.png"
    # Match the observed human head/boot baseline to the existing native size.
    # The generated square is sampled to 44px, then put on the 48px canvas;
    # this keeps the figure about 29px tall rather than filling the weapon box.
    pose = condition(source, 44)
    traqueur = Image.new("RGBA", (48, 48))
    copy_layer(traqueur, pose, (3, 0))
    assert all(not traqueur.getpixel((x, y))[3] for x in range(48) for y in (0, 47))
    assert all(not traqueur.getpixel((x, y))[3] for y in range(48) for x in (0, 47))
    native = save_image("classe-equipee-3-START-VIS01-03.png", traqueur)
    body_native = save_image("traqueur-body-native32-01.png", condition(OUT / "traqueur-body-source-01.png", 32))
    rifle_native = save_image("traqueur-rifle-steel-native48-01.png", condition(OUT / "traqueur-rifle-steel-source-02.png", 48, 8))
    records = deepcopy(previous["candidates"])
    row = records[2]
    row.update(native_image=native, body_size=[32, 32], canvas_size=[48, 48],
               visual_approval="simplified_Traqueur_and_neutral_rifle_prototype_pending_explicit_review")
    row["previous_image"] = old["candidates"][2]["native_image"]
    # A generated complete pose cannot inherit fixed-pixel preservation checks
    # from the old layered composition. Its identity and anatomy need review.
    for key in ["fixed_body_pixels_checked_outside_declared_hand_cut_and_layer_occlusion", "fixed_body_pixel_changes", "grips", "new_detached_fragments", "static_idle_pose", "palette"]:
        row.pop(key, None)
    row["composition_kind"] = "generated_equipped_pose_prototype;separate_fixed_body_arm_weapon_layers_required_before_active_integration"
    row["source"] = binary_ref(source)
    row["conditioning"] = {"sampled_square": [44, 44], "canvas_offset": [3, 0], "filter": "nearest",
                            "solid_alpha_threshold": 128, "maximum_opaque_colors": 12,
                            "native_opaque_colors": len({p[:3] for p in traqueur.get_flattened_data() if p[3]}),
                            "native_bounds": list(traqueur.getbbox()), "dither": "none",
                            "anatomy_pixel_identity_with_prior_body": "not_claimed;new_simplified_body_proposal"}
    row["independent_palette"] = {"body": "blue_and_ivory", "rifle": "steel_and_anthracite",
                                  "future_instance_cosmetics": "existing_independent_weapon_and_armor_palette_rule_preserved"}
    row["supporting_unarmed_body"] = body_native
    row["supporting_weapon_only"] = rifle_native
    figures = [(Image.open(ROOT / before["native_image"]["path"]).convert("RGBA"),
                Image.open(ROOT / after["native_image"]["path"]).convert("RGBA"))
               for before, after in zip(old["candidates"], records)]
    floors = Image.open(ART / "terrain-materials-32.png").convert("RGBA")
    board = Image.new("RGB", (1000, 676), "#0c131b")
    draw = ImageDraw.Draw(board)
    draw.text((20, 12), "START-VIS01 · Révision 3 · Prises et lisibilité", font=font(22), fill="#eef0f2")
    draw.text((20, 48), "Gants noirs · Traqueur simplifié · palettes d’armure et d’arme indépendantes · propositions", font=font(14), fill="#c2ced5")
    for i, ((before, after), item) in enumerate(zip(figures, records)):
        x = 20 + i * 196
        draw.rectangle((x, 82, x + 184, 650), fill="#182430", outline="#3b4b59")
        draw.text((x + 9, 92), f"{i + 1}. {item['class']}", font=font(17), fill="#eef0f2")
        label = "Inchangé" if i == 0 else "Silhouette simplifiée" if i == 2 else "Prise corrigée"
        draw.text((x + 9, 122), label, font=font(12), fill="#a9d9ca" if i else "#bac7d0")
        draw.text((x + 9, 154), "Avant · zoom ×3", font=font(13), fill="#bac7d0")
        zoom = before.resize((144, 144), Image.Resampling.NEAREST)
        board.paste(zoom, (x + 20, 180), zoom)
        draw.text((x + 9, 335), "Nouvel aperçu · zoom ×3", font=font(13), fill="#d9e5df")
        zoom = after.resize((144, 144), Image.Resampling.NEAREST)
        board.paste(zoom, (x + 20, 361), zoom)
        draw.text((x + 9, 516), "Avant / nouvel aperçu · 1:1", font=font(12), fill="#bac7d0")
        board.paste(before, (x + 25, 540), before)
        board.paste(after, (x + 112, 540), after)
        for j, color in enumerate(["#0c1117", "#c7c4b9", "#1e272b"]):
            patch = Image.new("RGBA", (48, 48), color)
            if j == 2:
                for yy in range(2):
                    for xx in range(2):
                        copy_layer(patch, cell(floors, 2 + (xx + yy) % 2, 32), (xx * 32, yy * 32))
            copy_layer(patch, after)
            board.paste(patch.convert("RGB"), (x + 9 + j * 55, 594))
    board_ref = save_image("classes-equipees-START-VIS01-review-03.png", board)
    scene = Image.new("RGBA", (576, 384), "#0c131b")
    sd = ImageDraw.Draw(scene)
    sd.text((16, 10), "START-VIS01 · Révision 3 · Taille native", font=font(18), fill="#eef0f2")
    for section, family in enumerate([1, 2, 0]):
        top = 52 + section * 104
        for yy in range(3):
            for xx in range(18):
                variant = (xx * 17 + yy * 23 + xx * yy % 7) % 4
                copy_layer(scene, cell(floors, family * 4 + variant, 32), (xx * 32, top + yy * 32))
        for i, (_, figure) in enumerate(figures):
            cx, cy = 40 + i * 112, top + 42
            copy_layer(scene, figure, (cx - 8, cy - 8))
            sd.text((cx + 8, top + 4), str(i + 1), font=font(13), fill="#ffffff", stroke_width=1, stroke_fill="#0c131b")
    sd.text((16, 367), "Montage de revue sur les sols du jeu · avant intégration", font=font(12), fill="#bbc7d0")
    scene_ref = save_image("classes-equipees-START-VIS01-scene-native-03.png", scene.convert("RGB"))
    prompts = [OUT / name for name in ["traqueur-simplification-body-prompt-01.txt", "traqueur-simplification-rifle-prompt-01.txt",
                                       "traqueur-simplification-rifle-palette-prompt-02.txt", "traqueur-simplification-equipped-prompt-03.txt",
                                       "traqueur-simplification-equipped-gloves-prompt-04.txt"]]
    sources = [OUT / name for name in ["traqueur-body-source-01.png", "traqueur-rifle-blue-source-01.png",
                                       "traqueur-rifle-steel-source-02.png", "traqueur-equipped-source-03.png",
                                       "traqueur-equipped-dark-gloves-source-04.png"]]
    result = {"date": "2026-10-09", "review_id": "START-VIS01", "revision": 3,
              "status": "corrected_grips_and_simplified_Traqueur_proposed;not_integrated",
              "previous_visual_review_path": "artifacts/refonte-agent-2026-10-06/review-equipped-classes-START-VIS01-02.json",
              "changed_candidate_numbers": [2, 3, 4, 5], "unchanged_candidate_numbers": [1],
              "candidates": records, "board": board_ref, "native_scene": scene_ref,
              "art_generation": {"method": "built_in_image_gen_tool", "prompts": [binary_ref(p) for p in prompts],
                                 "generated_sources": [binary_ref(p) for p in sources],
                                 "conditioning": "nearest_sampling_alpha_threshold_and_bounded_palette_only;no_python_drawing_or_repainting"},
              "source_asset_checks": old["sources"], "active_source_asset_changes": 0,
              "mechanical_changes": "none", "visual_approval": "pending_explicit_user_review",
              "approval_scope": "new_native_candidate3_and_one_hand_compositions2_4_5;no_acceptance_of_unchanged1_or_unshown_future_weapon_groups",
              "integration_boundary": "Traqueur_complete_pose_is_a_review_prototype;split_and_validate_fixed_body_arms_weapon_grips_and_palettes_before_runtime_use",
              "runtime_integration": "not_integrated"}
    target = OUT / "review-equipped-classes-START-VIS01-03.json"
    assert not target.exists()
    target.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"native_opaque_colors": row["conditioning"]["native_opaque_colors"],
                      "native_bounds": row["conditioning"]["native_bounds"], "native_review": board_ref["path"],
                      "source_asset_changes": 0, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
