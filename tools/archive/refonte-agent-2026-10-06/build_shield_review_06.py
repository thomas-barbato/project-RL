"""Package generated shield sources as native, hand-centered review layers.

No game assets or content are written. New shield artwork comes from imagegen;
this script only conditions alpha/palette/scale and composes existing rig frames.
"""
from pathlib import Path
import base64
import hashlib
import json
import shutil

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OUT = BASE / "shield-review-06"
BANK = BASE / "weapon-silhouette-review-03"
RIG = ROOT / "assets/sprites/cyberpunk/agent-rigs/manifest.json"
NAMES = ["Bastion", "Technomancien", "Traqueur", "Arcaniste", "Métamorphe"]
HELD_SIZES = [(11, 15), (11, 15), (11, 17), (13, 17), (13, 15), (13, 17)]
BG = (12, 19, 27)
FG = (229, 236, 240)
SUB = (160, 182, 196)


def font(size):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", size)


def ref(path):
    return {"path": path.relative_to(ROOT).as_posix(),
            "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def json_ref(path):
    return {"reference": path.relative_to(ROOT).as_posix(),
            "reference_sha256": hashlib.sha256(path.read_text(encoding="utf-8").encode()).hexdigest()}


def save_png(path, im):
    path.parent.mkdir(parents=True, exist_ok=True)
    im.save(path)
    return {**ref(path), "dimensions": list(im.size)}


def write_json(path, obj):
    path.write_text(json.dumps(obj, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")


def colors(im):
    return sorted({p[:3] for p in im.get_flattened_data() if p[3]})


def condition(im, size, palette):
    small = im.resize(size, Image.Resampling.NEAREST)
    alpha = small.getchannel("A").point(lambda v: 255 if v >= 128 else 0)
    mapped = small.convert("RGB").quantize(palette=palette, dither=Image.Dither.NONE).convert("RGBA")
    mapped.putalpha(alpha)
    return mapped


def tiled_floor(source, variant):
    if variant == "dark":
        return Image.new("RGBA", (64, 64), (16, 23, 31, 255))
    if variant == "light":
        return Image.new("RGBA", (64, 64), (173, 182, 184, 255))
    im = Image.new("RGBA", (64, 64))
    for y in (0, 32):
        for x in (0, 32):
            im.alpha_composite(source, (x, y))
    return im


def render_on_floor(frame, floor):
    im = floor.copy()
    im.alpha_composite(frame)
    return im


def sheet_title(draw, title, subtitle):
    draw.text((18, 12), title, font=font(23), fill=FG)
    draw.text((18, 46), subtitle, font=font(13), fill=SUB)


def main():
    entries = json.loads((OUT / "prompts.json").read_text(encoding="utf-8"))["assets"]
    profiles = json.loads(RIG.read_text(encoding="utf-8"))["profiles"]
    floor_path = BANK / "review-floor-native32.png"
    source_floor = Image.open(floor_path).convert("RGBA")
    floors = {k: tiled_floor(source_floor, k) for k in ("dark", "light", "terrain")}
    records, compositions = [], []
    images = {}
    for i, entry in enumerate(entries):
        cid = entry["candidate_id"]
        source = OUT / "sources" / (cid + ".png")
        source.parent.mkdir(parents=True, exist_ok=True)
        if not source.exists():
            shutil.copy2(entry["source_path"], source)
        raw = Image.open(source).convert("RGBA")
        binary_alpha = raw.getchannel("A").point(lambda v: 255 if v >= 128 else 0)
        bbox = binary_alpha.getbbox()
        assert bbox
        cropped = raw.crop(bbox)
        hw, hh = HELD_SIZES[i]
        inv_h = 27
        inv_w = min(27, round(cropped.width * inv_h / cropped.height))
        # Palette is derived from source samples, without introducing new colors.
        sample = cropped.resize((inv_w, inv_h), Image.Resampling.NEAREST)
        opaque_colors = [p[:3] for p in sample.get_flattened_data() if p[3] >= 128]
        strip = Image.new("RGB", (len(opaque_colors), 1))
        strip.putdata(opaque_colors)
        palette = strip.quantize(colors=10, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
        held = condition(cropped, (hw, hh), palette)
        inv = condition(cropped, (inv_w, inv_h), palette)
        inventory = Image.new("RGBA", (32, 32))
        inventory.alpha_composite(inv, ((32 - inv_w) // 2, (32 - inv_h) // 2))
        pivot = (hw // 2, hh // 2)
        assert held.getpixel(pivot)[3] == 255
        assert {p[3] for p in held.get_flattened_data()} <= {0, 255}
        assert len(colors(held)) <= 10
        held_ref = save_png(OUT / (cid + "-held.png"), held)
        inv_ref = save_png(OUT / (cid + "-inventory-native32.png"), inventory)
        record = {"candidate_id": cid, "review_number": i + 1,
                  "visual_approval": "pending_explicit_user_review",
                  "source": ref(source), "source_generated_path": entry["source_path"],
                  "source_crop": list(bbox), "source_dimensions": list(raw.size),
                  "held_layer": held_ref, "inventory_tile": inv_ref,
                  "held_pivot": list(pivot), "held_face": "flat_front_no_rotation",
                  "opaque_color_count": len(colors(held)), "palette": colors(held),
                  "conditioning": "source_crop;nearest_resize;10_color_source_palette_no_dither;binary_alpha_128",
                  "geometry_and_colors_are_proposals": True, "models_or_rules_added_to_game": False}
        records.append(record)
        images[(cid, "inventory")] = inventory
        for profile in profiles:
            n = profile["number"]
            for side in ("L", "R"):
                hand = profile["gloves"][side]
                elbow = profile["elbows"][side]
                anchor = (16 + hand[0], 16 + hand[1])
                offset = (anchor[0] - pivot[0], anchor[1] - pivot[1])
                layer = Image.new("RGBA", (64, 64))
                layer.alpha_composite(held, offset)
                mask = layer.getchannel("A")
                assert mask.getpixel(anchor) == 255
                for dx in (-1, 0, 1):
                    for dy in (-1, 0, 1):
                        assert mask.getpixel((anchor[0] + dx, anchor[1] + dy)) == 255
                mid = (16 + round((hand[0] + elbow[0]) / 2), 16 + round((hand[1] + elbow[1]) / 2))
                assert mask.getpixel(mid) == 255
                layer_ref = save_png(OUT / "layers" / f"{cid}-C{n}-{side}.png", layer)
                for weapon in ("none", "knife", "pistol"):
                    fam = {"none": "none", "knife": "00", "pistol": "06"}[weapon]
                    filename = f"C{n}-Lnone-R{fam}.png" if side == "L" else f"C{n}-L{fam}-Rnone.png"
                    baseline_path = BANK / filename
                    baseline = Image.open(baseline_path).convert("RGBA")
                    composite = baseline.copy()
                    composite.alpha_composite(layer)
                    outside_changes = sum(1 for a, b, m in zip(baseline.get_flattened_data(), composite.get_flattened_data(), mask.get_flattened_data()) if a != b and not m)
                    assert outside_changes == 0
                    file = OUT / "frames" / f"{cid}-C{n}-{side}-{weapon}.png"
                    composed_ref = save_png(file, composite)
                    composed_ref.update({"candidate_id": cid, "class_number": n, "class": NAMES[n - 1],
                                         "shield_hand": side, "other_hand_weapon": weapon,
                                         "native_hand_anchor": hand, "layer_offset": list(offset),
                                         "source_frame": ref(baseline_path), "shield_layer": layer_ref,
                                         "qa": {"outside_shield_pixel_changes": outside_changes,
                                                "hand_center_opaque": True, "hand_center_3x3_covered": True,
                                                "mid_forearm_covered": True, "center_error_pixels": [0, 0]}})
                    compositions.append(composed_ref)
                    images[(cid, n, side, weapon)] = composite

    # Overview: each candidate's inventory tile and five unmodified bodies with
    # shield + pistol. All native versions retain one screen pixel per art pixel.
    overview = Image.new("RGB", (1110, 705), BG)
    d = ImageDraw.Draw(overview)
    sheet_title(d, "Boucliers · six candidats", "SHIELD-VIS01 · bouclier à gauche, pistolet à droite · vue native et zoom ×2")
    d.text((35, 85), "Objet", font=font(13), fill=SUB)
    for n, name in enumerate(NAMES, 1):
        d.text((190 + (n - 1) * 181, 85), name, font=font(13), fill=SUB)
    for i, r in enumerate(records):
        cid = r["candidate_id"]
        y = 110 + i * 96
        d.line((18, y - 7, 1090, y - 7), fill=(37, 50, 61))
        d.text((18, y + 4), f"{i+1:02d}", font=font(16), fill=FG)
        overview.paste(images[(cid, "inventory")], (53, y + 8), images[(cid, "inventory")])
        inv_zoom = images[(cid, "inventory")].resize((64, 64), Image.Resampling.NEAREST)
        overview.paste(inv_zoom, (93, y), inv_zoom)
        for n in range(1, 6):
            frame = render_on_floor(images[(cid, n, "L", "pistol")], floors["terrain"])
            x = 178 + (n - 1) * 181
            overview.paste(frame, (x, y + 2))
            overview.paste(frame.resize((128, 128), Image.Resampling.NEAREST).crop((16, 16, 112, 112)), (x + 64, y - 10))
    overview_ref = save_png(OUT / "shields-native-and-zoom.png", overview)

    # Both hands with no secondary weapon: sixty native placements, mirrored
    # only by the anchor. Shield lighting is not flipped with the carrier.
    both = Image.new("RGB", (810, 625), BG)
    bd = ImageDraw.Draw(both)
    sheet_title(bd, "Boucliers · les deux mains", "60 placements · chaque personnage à taille native · gauche puis droite")
    for n, name in enumerate(NAMES, 1):
        bd.text((156 + (n - 1) * 128, 82), name, font=font(12), fill=SUB)
    for i, r in enumerate(records):
        y = 110 + i * 82
        cid = r["candidate_id"]
        bd.text((18, y + 20), f"{i+1:02d} · {cid}", font=font(13), fill=FG)
        for n in range(1, 6):
            for s, side in enumerate(("L", "R")):
                frame = render_on_floor(images[(cid, n, side, "none")], floors["terrain"])
                both.paste(frame, (145 + (n-1)*128 + s*64, y))
    both_ref = save_png(OUT / "shields-both-hands-native.png", both)

    contrast = Image.new("RGB", (710, 565), BG)
    cd = ImageDraw.Draw(contrast)
    sheet_title(cd, "Boucliers · contraste", "Même couche sur fond sombre, clair et sol du jeu · taille native 1:1")
    for k, label in enumerate(("Sombre", "Clair", "Sol du jeu")):
        cd.text((220 + k*150, 83), label, font=font(13), fill=SUB)
    for i, r in enumerate(records):
        y = 112 + i*71
        cid = r["candidate_id"]
        cd.text((18, y + 21), f"{i+1:02d} · {cid}", font=font(13), fill=FG)
        for k, key in enumerate(("dark", "light", "terrain")):
            contrast.paste(render_on_floor(images[(cid, 2, "L", "knife")], floors[key]), (214 + k*150, y))
    contrast_ref = save_png(OUT / "shields-contrast-native.png", contrast)

    all_frame_data = [{**r, "url": "data:image/png;base64," + base64.b64encode((ROOT/r["path"]).read_bytes()).decode()} for r in compositions]
    floor_data = {k: "data:image/png;base64," + base64.b64encode(floor_image_bytes(im)).decode() for k, im in floors.items()}
    payload = {"candidates": [{"id":r["candidate_id"], "number":r["review_number"], "inventory":"data:image/png;base64,"+base64.b64encode((ROOT/r["inventory_tile"]["path"]).read_bytes()).decode()} for r in records],
               "frames": all_frame_data, "floors": floor_data, "classes":NAMES}
    html = (BASE / "shield_gallery_template_01.html").read_text(encoding="utf-8").replace("__PAYLOAD__", json.dumps(payload, ensure_ascii=False))
    gallery = OUT / "shield-gallery-06.html"
    gallery.write_text(html, encoding="utf-8", newline="\n")
    manifest = {"date":"2026-10-10", "review_id":"SHIELD-VIS01", "revision":6,
                "status":"native_readability_review;EQ-S02_lower_steel_frame_thickened_to_match_upper;other_five_unchanged;pending_user_pixel_approval",
                "previous_review":json_ref(BASE / "shield-review-05/review-shields-SHIELD-VIS01-05.json"),
                "feedback_review_base":json_ref(BASE / "shield-review-04/review-shields-SHIELD-VIS01-04.json"),
                "direction":json_ref(BASE / "shield-flat-hand-centered-direction-01.json"),
                "series":json_ref(BASE / "review-equipment-series-EQUIP-SERIES01-03.json"),
                "approved_source_frames":json_ref(BANK / "review-weapon-silhouettes-WEAPON-SIL01-03.json"),
                "prompts":json_ref(OUT / "prompts.json"), "generator":ref(Path(__file__)),
                "compositor_template":ref(BASE / "shield_gallery_template_01.html"),
                "native_body_dimensions":[32,32], "composition_canvas":[64,64],
                "layer_order":"existing_body_arm_weapon_fingers_frame_then_opaque_shield_face;no_fingers_over_shield",
                "candidates":records, "compositions":compositions,
                "overview":overview_ref, "both_hands_native":both_ref, "contrast_native":contrast_ref,
                "gallery":ref(gallery), "floor_source":ref(floor_path),
                "qa":{"candidate_count":6, "compositions":len(compositions), "hand_placements":60,
                      "outside_shield_pixel_changes":0, "center_offset_errors":0,
                      "all_hands_and_mid_forearms_covered":True, "alpha":"binary",
                      "opaque_colors_per_held_layer_max":max(r["opaque_color_count"] for r in records),
                      "active_assets_modified":False, "gameplay_changes":False},
                "approval":{"pixels":"EQ-S02_pending_revised_visual_review;other_five_unchanged_from_revision_4;no_additional_acceptance_inferred", "pose_direction":"accepted SHIELD-POSE01;concrete_placement_accepted_by_user_on_2026-10-10",
                            "item_names":"not_proposed_here", "stats":"not_defined_here"},
                "runtime_integration":"not_integrated;review_only"}
    write_json(OUT / "review-shields-SHIELD-VIS01-06.json", manifest)
    print(json.dumps(manifest["qa"]))


def floor_image_bytes(im):
    from io import BytesIO
    buffer = BytesIO()
    im.save(buffer, format="PNG")
    return buffer.getvalue()


if __name__ == "__main__":
    main()
