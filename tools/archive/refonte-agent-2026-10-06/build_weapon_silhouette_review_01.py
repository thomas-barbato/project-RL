"""Reusable code-native held layers for the five class references, review only.

No active assets, content IDs, game rules or renderer are changed. Existing
art layers are placed, mirrored, rotated, sampled and material-recolored.
"""
from collections import Counter
from pathlib import Path
import hashlib
import json
import math
import re
import runpy
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OUT = BASE / "weapon-silhouette-review-01"
ART = ROOT / "assets/sprites/cyberpunk"
H = runpy.run_path(str(BASE / "render_pistol_robe_class_review_04.py"), run_name="composition_helpers")
cell, copy_layer = H["cell"], H["copy_layer"]
COMPONENTS = H["HELP"]["components"]
ONE = [0, 1, 4, 5, 6, 7, 16]
GRAY = [[38, 43, 48], [161, 171, 181], [232, 239, 245]]
PINK = [[104, 28, 82], [221, 67, 156], [255, 160, 216]]
BLUE = [[19, 40, 102], [52, 107, 221], [134, 195, 255]]
RED = [[76, 8, 15], [224, 22, 32], [255, 58, 50]]


def binary_ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def save_png(name, image):
    path = OUT / name
    assert not path.exists(), path
    image.save(path)
    return {**binary_ref(path), "dimensions": list(image.size)}


def ramp_color(pixel, ramp):
    value = max(pixel[:3]) / 255
    if value < 0.53:
        low, high, t = ramp[0], ramp[1], min(1, max(0, (value - 0.12) / 0.41))
    else:
        low, high, t = ramp[1], ramp[2], min(1, max(0, (value - 0.53) / 0.47))
    return tuple(math.floor(low[n] * (1 - t) + high[n] * t + 0.5) for n in range(3)) + (pixel[3],)


def recolor_labels(image, mask, ramps):
    result = image.copy()
    for y in range(image.height):
        for x in range(image.width):
            label = mask.getpixel((x, y))[0]
            pixel = image.getpixel((x, y))
            if pixel[3] and label in ramps:
                result.putpixel((x, y), ramp_color(pixel, ramps[label]))
    assert result.getchannel("A").tobytes() == image.getchannel("A").tobytes()
    return result


def native_material_ramp(body, mask, label):
    colors = sorted({body.getpixel((x, y))[:3] for y in range(32) for x in range(32)
                     if body.getpixel((x, y))[3] and mask.getpixel((x, y))[0] == label}, key=lambda p: sum(p))
    assert colors
    return [list(colors[0]), list(colors[len(colors) // 2]), list(colors[-1])]


def profiles(bodies, body_maps):
    configs = [(1, "Bastion", 9, 1), (2, "Technomancien", 2, 2),
               (3, "Traqueur", None, None), (4, "Arcaniste", 11, 1), (5, "Métamorphe", 5, 1)]
    result = []
    for number, name, index, sleeve_material in configs:
        if number == 3:
            # Reuse the prepared unarmed source; a complete generated rifle pose
            # cannot silently claim fixed-pixel identity after weapon removal.
            source = BASE / "traqueur-body-native32-01.png"
            body = Image.open(source).convert("RGBA").resize((36, 36), Image.Resampling.NEAREST).crop((2, 2, 34, 34))
            info = {"source": binary_ref(source), "conditioning": "nearest36px_then_crop2_2_34_34;body_stays_fixed_in_all_configurations",
                    "historical_equipped_pose_pixel_identity": "not_claimed;unarmed_layer_candidate_requires_review_alongside_prior_accepted_rifle_pose"}
            left_glove, right_glove = (9, 18), (22, 18)
            left_elbow, right_elbow = (10, 15), (22, 15)
            left_cut, right_cut = (8, 16, 11, 20), (21, 16, 25, 20)
            sleeve_ramp = [[8, 36, 98], [0, 74, 201], [88, 129, 181]]
        else:
            body = cell(bodies, index, 32)
            mask = cell(body_maps, index, 32)
            if number == 4:
                body, _ = H["red_primary_material"](body, mask, RED)
            info = {"source_atlas_index": index, "geometry_changes": 0,
                    "palette": "accepted_vivid_red_primary_only" if number == 4 else "original"}
            left_glove, right_glove = (6, 19), (25, 19)
            left_elbow, right_elbow = (8, 16), (22, 16)
            left_cut, right_cut = (3, 17, 10, 23), (22, 17, 30, 23)
            sleeve_ramp = native_material_ramp(body, mask, sleeve_material)
        for point in (left_glove, right_glove, left_elbow, right_elbow):
            assert body.getpixel(point)[3], (name, point)
        body_ref = save_png(f"body-{number}-native32.png", body)
        result.append({"number": number, "class": name, "body": body, "body_reference": body_ref, "body_source": info,
                       "gloves": {"L": left_glove, "R": right_glove},
                       "elbows": {"L": left_elbow, "R": right_elbow},
                       "forearm_masks": [left_cut, right_cut], "sleeve_ramp": sleeve_ramp})
    return result


def one_hand(profile, family, side, weapon, grip):
    pivot = list(grip)
    if family in (6, 7):
        weapon = weapon.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
        pivot[1] = 47 - pivot[1]
        elbow = profile["elbows"]["R"]
        glove = profile["gloves"]["R"]
        angle = math.degrees(math.atan2(glove[0] - elbow[0], glove[1] - elbow[1]))
        scratch = Image.new("RGBA", (96, 96))
        copy_layer(scratch, weapon, (48 - pivot[0], 48 - pivot[1]))
        weapon = scratch.rotate(angle, resample=Image.Resampling.NEAREST, center=(48, 48))
        pivot = [48, 48]
    else:
        angle = 0
    if side == "L":
        weapon = weapon.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        pivot[0] = weapon.width - 1 - pivot[0]
    point = profile["gloves"][side]
    target = [16 + point[0], 16 + point[1]]
    layer = Image.new("RGBA", (64, 64))
    copy_layer(layer, weapon, (target[0] - pivot[0], target[1] - pivot[1]))
    assert sum(a > 0 for a in layer.getchannel("A").tobytes()) == sum(a > 0 for a in weapon.getchannel("A").tobytes())
    contact = sum(layer.getpixel((16 + x, 16 + y))[3] > 0 and profile["body"].getpixel((x, y))[3] > 0
                  for y in range(max(0, point[1] - 2), min(32, point[1] + 3))
                  for x in range(max(0, point[0] - 2), min(32, point[0] + 3)))
    assert contact >= 2, (profile["class"], family, side, contact)
    return layer, {"hand": side, "family": family, "canvas_glove": target, "handle_body_overlap_pixels": contact,
                   "barrel_rotation_from_vertical": angle, "new_forearm_or_finger_layers": 0, "clipped_weapon_pixels": 0}


def forearm_warp(part, start, end, target_start, target_end):
    sx, sy = end[0] - start[0], end[1] - start[1]
    tx, ty = target_end[0] - target_start[0], target_end[1] - target_start[1]
    ls, lt = math.hypot(sx, sy), math.hypot(tx, ty)
    assert ls > 0 and lt > 0
    ux, uy, vx, vy = sx / ls, sy / ls, tx / lt, ty / lt
    # Change limb angle/length while preserving its transverse thickness.
    scale = lt / ls
    a, b = scale * vx * ux + vy * uy, scale * vx * uy - vy * ux
    c, d = scale * vy * ux - vx * uy, scale * vy * uy + vx * ux
    determinant = a * d - b * c
    ia, ib, ic, id_ = d / determinant, -b / determinant, -c / determinant, a / determinant
    ox = start[0] - ia * target_start[0] - ib * target_start[1]
    oy = start[1] - ic * target_start[0] - id_ * target_start[1]
    return part.transform((64, 64), Image.Transform.AFFINE, (ia, ib, ox, ic, id_, oy), resample=Image.Resampling.NEAREST), {
        "source_elbow": list(start), "source_grip": list(end), "target_elbow": list(target_start), "target_grip": list(target_end),
        "length_scale": round(scale, 4), "transverse_scale": 1}


def two_hands(profile, family, weapon, arms, arm_map, fingers, grips):
    sleeve = recolor_labels(arms, arm_map, {2: profile["sleeve_ramp"]})
    posed = Image.new("RGBA", (64, 64))
    limb_records = []
    for hand, nominal in [("L", (14, 23)), ("R", (33, 24))]:
        part = Image.new("RGBA", (48, 48))
        for y in range(48):
            for x in range(48):
                if (x < 25) == (hand == "L"):
                    part.putpixel((x, y), sleeve.getpixel((x, y)))
        points = [(x, y) for y in range(48) for x in range(48) if part.getpixel((x, y))[3]]
        assert points
        start = min(points, key=lambda p: (p[0] - nominal[0]) ** 2 + (p[1] - nominal[1]) ** 2)
        end = grips[0 if hand == "L" else 1]
        if start == tuple(end):
            start = max(points, key=lambda p: (p[0] - end[0]) ** 2 + (p[1] - end[1]) ** 2)
        elbow = profile["elbows"][hand]
        transformed, record = forearm_warp(part, start, end, [16 + elbow[0], 16 + elbow[1]], [8 + end[0], 8 + end[1]])
        copy_layer(posed, transformed)
        limb_records.append({"hand": hand, **record})
    glove_layer = fingers.copy()
    for y in range(48):
        for x in range(48):
            pixel = fingers.getpixel((x, y))
            if pixel[3]:
                glove_layer.putpixel((x, y), ramp_color(pixel, [[0, 1, 1], [18, 20, 22], [45, 48, 51]]))
    weapon_layer, front_gloves = Image.new("RGBA", (64, 64)), Image.new("RGBA", (64, 64))
    copy_layer(weapon_layer, weapon, (8, 8))
    copy_layer(front_gloves, glove_layer, (8, 8))
    contacts = []
    for gx, gy in grips:
        contact = sum(weapon_layer.getpixel((8 + x, 8 + y))[3] > 0 and front_gloves.getpixel((8 + x, 8 + y))[3] > 0
                      for y in range(gy - 2, gy + 3) for x in range(gx - 2, gx + 3))
        assert contact >= 2
        contacts.append(contact)
    return posed, weapon_layer, front_gloves, {"family": family, "grips": [[8 + x, 8 + y] for x, y in grips],
                                              "finger_weapon_contacts": contacts, "forearms": limb_records,
                                              "sleeve_ramp_from_own_class_material": profile["sleeve_ramp"], "clipped_weapon_pixels": 0}


def compose(profile, left, right, weapon_tiles, arm_tiles, arm_maps, finger_tiles, families):
    body = profile["body"]
    base = Image.new("RGBA", (64, 64))
    copy_layer(base, body, (16, 16))
    details, occlusion = [], Image.new("RGBA", (64, 64))
    if right is not None and right not in ONE:
        assert left is None
        fixed = body.copy()
        for x0, y0, x1, y1 in profile["forearm_masks"]:
            for y in range(y0, y1):
                for x in range(x0, x1):
                    fixed.putpixel((x, y), (0, 0, 0, 0))
        base = Image.new("RGBA", (64, 64))
        copy_layer(base, fixed, (16, 16))
        sleeve, weapon, gloves, record = two_hands(profile, right, weapon_tiles[right], arm_tiles[right], arm_maps[right], finger_tiles[right], families[right]["hands"])
        for layer in [sleeve, weapon, gloves]:
            copy_layer(base, layer)
            copy_layer(occlusion, layer)
        details.append(record)
        check_body = fixed
    else:
        base = Image.new("RGBA", (64, 64))
        for side, family in [("L", left), ("R", right)]:
            if family is not None:
                assert family in ONE
                layer, record = one_hand(profile, family, side, weapon_tiles[family], families[family]["hands"][0])
                copy_layer(base, layer)
                details.append(record)
        copy_layer(base, body, (16, 16))
        check_body = body
    checked = 0
    for y in range(32):
        for x in range(32):
            pixel = check_body.getpixel((x, y))
            if pixel[3] and not occlusion.getpixel((16 + x, 16 + y))[3]:
                assert base.getpixel((16 + x, 16 + y)) == pixel
                checked += 1
    unexpected, removed_sampling_fragments = [], []
    for group in COMPONENTS(base)[1:]:
        if not all(16 <= x < 48 and 16 <= y < 48 and body.getpixel((x - 16, y - 16))[3] for x, y in group):
            if right is not None and right not in ONE and len(group) <= 2 and all(
                    sleeve.getpixel((x, y))[3] and not weapon.getpixel((x, y))[3] and not gloves.getpixel((x, y))[3]
                    and not (16 <= x < 48 and 16 <= y < 48 and body.getpixel((x - 16, y - 16))[3]) for x, y in group):
                # An isolated resampling speck on the warped sleeve cannot be a
                # grip, weapon or authored body pixel. Keep the source intact.
                for point in group:
                    base.putpixel(point, (0, 0, 0, 0))
                removed_sampling_fragments.append(sorted(group))
            else:
                unexpected.append(sorted(group))
    return base, {"grip_details": details, "fixed_pixels_checked": checked, "fixed_pixel_changes_outside_declared_masks_and_occlusion": 0,
                  "new_detached_components": unexpected, "removed_affine_sleeve_sampling_fragments": removed_sampling_fragments,
                  "canvas": [64, 64], "body_tile": [16, 16, 48, 48]}


def main(revision=1):
    global OUT
    OUT = BASE / f"weapon-silhouette-review-{revision:02d}"
    assert not OUT.exists(), "Create a new review revision rather than replacing an existing bank"
    OUT.mkdir()
    files = [ART / name for name in ["crawl-bodies-32.png", "crawl-body-materials-32.png", "crawl-weapons-48.png", "crawl-weapon-materials-48.png",
                                   "crawl-arms-48.png", "crawl-arm-materials-48.png", "crawl-fingers-48.png", "manifest.json"]]
    frozen = {p: hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
    source = [Image.open(p).convert("RGBA") for p in files[:-1]]
    catalog = json.loads(files[-1].read_text(encoding="utf-8"))
    bodies = profiles(source[0], source[1])
    weapon_tiles = [cell(source[2], i, 48) for i in range(18)]
    masks = [cell(source[3], i, 48) for i in range(18)]
    # Only the requested pistol family changes its default material palette.
    weapon_tiles[6] = recolor_labels(weapon_tiles[6], masks[6], {1: GRAY})
    if revision >= 3:
        # Preserve the accepted steel/anthracite rifle direction independently
        # of the armor color, using colors from the approved Traqueur pose.
        weapon_tiles[17] = recolor_labels(weapon_tiles[17], masks[17], {
            1: [[8, 9, 10], [56, 54, 58], [146, 157, 169]],
            2: [[0, 0, 1], [56, 54, 58], [67, 84, 118]]})
    arm_tiles, arm_maps, finger_tiles = [[cell(atlas, i, 48) for i in range(18)] for atlas in source[4:7]]
    for i, tile in enumerate(weapon_tiles):
        save_png(f"weapon-family-{i:02d}-native48.png", tile)
    frames, images, defects = [], [], []
    for profile in bodies:
        configs = [(None, None)] + [(None, f) for f in range(18)] + [(f, None) for f in ONE] + [(l, r) for l in ONE for r in ONE]
        for left, right in configs:
            image, qa = compose(profile, left, right, weapon_tiles, arm_tiles, arm_maps, finger_tiles, catalog["weapon_families"])
            assert set(image.getchannel("A").tobytes()) <= {0, 255}
            label = f"C{profile['number']}-L{'none' if left is None else f'{left:02d}'}-R{'none' if right is None else f'{right:02d}'}"
            record = {"review_id": "WEAPON-SIL01-" + label, "class_number": profile["number"], "left_family": left, "right_family": right,
                      "sprite_index": len(frames), "native_image": save_png(label + ".png", image), "qa": qa}
            if qa["new_detached_components"]:
                defects.append({"id": record["review_id"], "groups": qa["new_detached_components"]})
            frames.append(record)
            images.append(image)
    atlas = Image.new("RGBA", (25 * 64, math.ceil(len(images) / 25) * 64))
    for i, image in enumerate(images):
        copy_layer(atlas, image, (i % 25 * 64, i // 25 * 64))
    atlas_ref = save_png("class-weapons-review-atlas64.png", atlas)
    actual_ids = []
    for path in sorted((ROOT / "content/core/weapons").glob("*.json5")):
        found = re.findall(r'^  "?id"?\s*:\s*"([^"\n]+)"', path.read_text(encoding="utf-8"), re.MULTILINE)
        assert len(found) == 1, path
        actual_ids.append(found[0])
    assert set(actual_ids) == {r["id"] for r in catalog["held_weapons"]} and len(actual_ids) == 113
    serial_profiles = [{key: value for key, value in p.items() if key != "body"} for p in bodies]
    data = {"date": "2026-10-09", "review_id": "WEAPON-SIL01", "revision": revision,
            "status": "full_family_and_hand_configuration_bank_prepared;visual_review_before_integration",
            "classes": serial_profiles, "families": catalog["weapon_families"], "model_mappings": catalog["held_weapons"],
            "current_model_count": 113, "family_count": 18, "class_count": 5,
            "configuration_count": len(frames), "one_hand_families": ONE, "frames": frames, "atlas": atlas_ref,
            "pistol_material": {"requested": "light_gray", "ramp": GRAY, "orientation": "accepted_forearm_axis_rule_per_class;own_hand_anchors"},
            "rifle_material_direction": "steel_and_anthracite_from_accepted_Traqueur_pose;shared_family17_drawings_pending_review" if revision >= 3 else "original_family17",
            "pink_replacement": {"requested": "neutral_gray", "ramp": GRAY, "no_model_or_bonus_deletion": True, "pink_in_proposed_palette_pool": False},
            "sources": [binary_ref(p) for p in files], "source_asset_changes": 0,
            "traqueur_layer_review": "prepared_unarmed_reference_conditioned_to_native_anatomical_height;compare_with_accepted_complete_rifle_pose_before_integration",
            "qa": {"models_missing": [], "fixed_pixel_errors": 0, "clipped_weapon_pixels": 0, "new_detached_component_cases": defects},
            "mechanical_changes": "none", "visual_approval": "all_new_family_compositions_and_gray_palette_pending_explicit_review",
            "runtime_integration": "not_integrated"}
    assert frozen == {p: hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
    (OUT / f"review-weapon-silhouettes-WEAPON-SIL01-{revision:02d}.json").write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps({"models": 113, "families": 18, "classes": 5, "configurations": len(frames),
                      "detached_component_cases": len(defects), "source_asset_changes": 0, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
