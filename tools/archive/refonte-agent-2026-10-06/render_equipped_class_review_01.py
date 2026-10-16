"""Export review compositions from existing approved layers, without painting.

The 32px bodies retain native pixels. The renderer's 48px transparent canvas
allows held weapons to overflow, at one pixel per pixel. No active asset changes.
"""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
ART = ROOT / "assets/sprites/cyberpunk"
RID = "START-VIS01"
NAMES = ["Bastion", "Technomancien", "Traqueur", "Arcaniste", "Métamorphe"]
SOURCES = ["crawl-bodies-32.png", "crawl-arms-48.png", "crawl-weapons-48.png",
           "crawl-fingers-48.png", "terrain-materials-32.png", "crawl-items-32.png",
           "manifest.json"]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cell(atlas, index, side):
    columns = atlas.width // side
    x, y = index % columns * side, index // columns * side
    return atlas.crop((x, y, x + side, y + side))


def overlay(target, layer, offset=(0, 0)):
    # Rust paste() copies opaque layer bytes. These reviewed sources are binary alpha.
    assert set(layer.getchannel("A").getdata()) <= {0, 255}
    target.paste(layer, offset, layer.getchannel("A"))


def cut_hand(pose, x, y):
    if not 17 <= y < 23:
        return False
    left, right = 3 <= x < 10, 22 <= x < 30
    return left or right if pose >= 2 else y >= (19 if pose == 1 else 18) and right


def compose(body, sleeve, weapon, fingers, pose):
    fixed = body.copy()
    for y in range(32):
        for x in range(32):
            if cut_hand(pose, x, y):
                fixed.putpixel((x, y), (0, 0, 0, 0))
    canvas = Image.new("RGBA", (48, 48))
    overlay(canvas, fixed, (8, 8))
    for layer in (sleeve, weapon, fingers):
        overlay(canvas, layer)
    return canvas


def font(size):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", size)


def save_png(name, image):
    path = OUT / name
    assert not path.exists(), name
    image.save(path)
    return {"path": str(path.relative_to(ROOT)).replace("\\", "/"),
            "binary_sha256": digest(path), "dimensions": list(image.size)}


def components(image):
    points = {(x, y) for y in range(image.height) for x in range(image.width)
              if image.getpixel((x, y))[3]}
    groups = []
    while points:
        seed = min(points)
        points.remove(seed)
        stack, group = [seed], {seed}
        while stack:
            x, y = stack.pop()
            for yy in range(y - 1, y + 2):
                for xx in range(x - 1, x + 2):
                    if (xx, yy) in points:
                        points.remove((xx, yy)); group.add((xx, yy)); stack.append((xx, yy))
        groups.append(group)
    return sorted(groups, key=len, reverse=True)


def main():
    snapshots = {name: digest(ART / name) for name in SOURCES}
    images = {name: Image.open(ART / name).convert("RGBA") for name in SOURCES if name.endswith(".png")}
    silhouettes = json.loads((OUT / "class-candidates.json").read_text(encoding="utf-8"))
    assert snapshots["crawl-bodies-32.png"] == silhouettes["source_sha256"]
    gear = json.loads((OUT / "review-starting-equipment-START-GEAR01-01.json").read_text(encoding="utf-8"))
    catalog = json.loads((ART / "manifest.json").read_text(encoding="utf-8"))
    bindings = {row["id"]: row for row in catalog["held_weapons"]}
    items = {row["id"]: row["illustration"] for row in catalog["items"]}
    records, figures = [], []
    for original, loadout, name in zip(silhouettes["candidates"], gear["proposed_loadouts"], NAMES):
        number, index = original["number"], original["source_atlas_index"]
        binding = bindings[loadout["active_weapon"]["reference_model"]]
        family = binding["family"]
        pose = {"Side": 0, "Raised": 1, "TwoHanded": 2, "MeleeTwoHanded": 3}[binding["hold"]]
        body = cell(images["crawl-bodies-32.png"], index, 32)
        assert body.tobytes() == Image.open(OUT / original["native_tile"]).convert("RGBA").tobytes()
        sleeve, weapon, fingers = [cell(images[source], family, 48) for source in
                                   ["crawl-arms-48.png", "crawl-weapons-48.png", "crawl-fingers-48.png"]]
        figure = compose(body, sleeve, weapon, fingers, pose)
        occlusion = Image.new("RGBA", (48, 48))
        for layer in (sleeve, weapon, fingers):
            overlay(occlusion, layer)
        checked = 0
        for y in range(32):
            for x in range(32):
                if not cut_hand(pose, x, y) and not occlusion.getpixel((x + 8, y + 8))[3]:
                    assert figure.getpixel((x + 8, y + 8)) == body.getpixel((x, y))
                    checked += 1
        grip_checks = []
        for gx, gy in catalog["weapon_families"][family]["hands"]:
            contact = sum(bool(weapon.getpixel((x, y))[3] and fingers.getpixel((x, y))[3])
                          for y in range(gy - 2, gy + 3) for x in range(gx - 2, gx + 3))
            assert contact >= 2, (name, gx, gy, contact)
            grip_checks.append({"position": [gx, gy], "finger_weapon_contacts": contact})
        assert len(components(weapon)) == 1
        for group in components(figure)[1:]:
            assert all(8 <= x < 40 and 8 <= y < 40 and body.getpixel((x - 8, y - 8))[3] for x, y in group)
        assert all(not figure.getpixel((x, y))[3] for x in range(48) for y in (0, 47))
        assert all(not figure.getpixel((x, y))[3] for y in range(48) for x in (0, 47))
        native = save_png(f"classe-equipee-{number}-START-VIS01-01.png", figure)
        records.append({"review_id": RID + f"-{number}", "number": number, "class": name,
                        "body_atlas_index": index, "starting_weapon": loadout["active_weapon"]["draft_id"],
                        "reference_model": binding["id"], "weapon_family_index": family,
                        "weapon_hold": binding["hold"], "palette": "Original", "static_idle_pose": 0,
                        "native_image": native, "body_scale": "32px_no_resize",
                        "fixed_body_pixels_checked_outside_declared_hand_cut_and_layer_occlusion": checked,
                        "fixed_body_pixel_changes": 0, "grips": grip_checks,
                        "new_detached_fragments": 0, "visual_approval": "proposed_equipped_composition",
                        "item_native_icon_index": items[binding["id"]]})
        figures.append((body, figure))

    board = Image.new("RGB", (1000, 526), "#0c131b")
    draw = ImageDraw.Draw(board)
    draw.text((20, 12), "START-VIS01 · Aperçus équipés des cinq classes", font=font(22), fill="#eef0f2")
    draw.text((20, 47), "Dessins existants · corps 32 × 32 · composition avec débordement d’arme · propositions", font=font(14), fill="#bbc7d0")
    floors = images["terrain-materials-32.png"]
    for i, ((body, figure), row) in enumerate(zip(figures, records)):
        x = 20 + i * 196
        draw.rectangle((x, 79, x + 184, 500), fill="#192430", outline="#3b4b59")
        draw.text((x + 9, 89), f"{i + 1}. {NAMES[i]}", font=font(17), fill="#eef0f2")
        draw.text((x + 9, 116), gear["proposed_loadouts"][i]["active_weapon"]["display_name"], font=font(13), fill="#bac7d0")
        draw.text((x + 9, 146), "Avant       Équipé · 1:1", font=font(12), fill="#bac7d0")
        board.paste(body, (x + 24, 179), body)
        board.paste(figure, (x + 109, 171), figure)
        draw.text((x + 9, 232), "Même composition · zoom ×3", font=font(12), fill="#bac7d0")
        board.paste(figure.resize((144, 144), Image.Resampling.NEAREST), (x + 20, 259), figure.resize((144, 144), Image.Resampling.NEAREST))
        draw.text((x + 9, 419), "Sombre / clair / sol · 1:1", font=font(12), fill="#bac7d0")
        for j, color in enumerate(["#0c1117", "#c7c4b9", "#1e272b"]):
            patch = Image.new("RGBA", (48, 48), color)
            if j == 2:
                patch = cell(floors, 2, 32).resize((48, 48), Image.Resampling.NEAREST)
                # Only the background is enlarged; the character remains native.
            patch.alpha_composite(figure)
            board.paste(patch.convert("RGB"), (x + 9 + j * 55, 443))
    board_record = save_png("classes-equipees-START-VIS01-review-01.png", board)

    # Review scene assembled on actual game floors, at native scale, with stacked
    # visual examples of existing weapon icons. This is not a live game capture.
    scene = Image.new("RGBA", (576, 384), "#0c131b")
    sd = ImageDraw.Draw(scene)
    sd.text((16, 10), "START-VIS01 · Lecture en situation à taille native", font=font(18), fill="#eef0f2")
    for section, family in enumerate([1, 2, 0]):
        top = 52 + section * 104
        for yy in range(3):
            for xx in range(18):
                variant = (xx * 17 + yy * 23 + xx * yy % 7) % 4
                overlay(scene, cell(floors, family * 4 + variant, 32), (xx * 32, top + yy * 32))
        for i, (_, figure) in enumerate(figures):
            cx, cy = 40 + i * 112, top + 42
            overlay(scene, figure, (cx - 8, cy - 8))
            sd.text((cx + 8, top + 4), str(i + 1), font=font(13), fill="#ffffff", stroke_width=1, stroke_fill="#0c131b")
            item = cell(images["crawl-items-32.png"], records[i]["item_native_icon_index"], 32)
            overlay(scene, item, (cx + 40, cy + 22))
    sd.text((16, 367), "Montage de revue sur les sols du jeu · aucune intégration", font=font(12), fill="#bbc7d0")
    scene_record = save_png("classes-equipees-START-VIS01-scene-native-01.png", scene.convert("RGB"))
    assert snapshots == {name: digest(ART / name) for name in SOURCES}
    manifest = {"date": "2026-10-09", "review_id": RID, "revision": 1,
                "status": "numbered_equipped_compositions_proposed_before_integration",
                "sources": [{"path": str((ART / name).relative_to(ROOT)).replace("\\", "/"),
                             "binary_sha256": value} for name, value in snapshots.items()],
                "composition_method": "existing_game_art_worn_compose_pose;original_palette;pose0;no_new_art_pixels_or_source_changes",
                "native_body": [32, 32], "transparent_composition_canvas": [48, 48],
                "draw_order": ["fixed_body_with_declared_hand_cuts", "sleeves", "weapon", "fingers"],
                "candidates": records, "board": board_record, "native_scene": scene_record,
                "approval_scope": "five_numbered_equipped_compositions_only;creation_screen_layout_animation_and_item_icons_separate",
                "visual_approval": "pending_explicit_user_review", "mechanic_changes": "none",
                "runtime_integration": "not_integrated"}
    path = OUT / "review-equipped-classes-START-VIS01-01.json"
    assert not path.exists()
    path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"candidates": 5, "fixed_body_changes": 0, "source_changes": 0,
                      "grip_contacts_checked": sum(len(row["grips"]) for row in records),
                      "review_board": board_record["path"], "native_scene": scene_record["path"]}))


if __name__ == "__main__":
    main()
