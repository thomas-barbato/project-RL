"""Pack the redrawn worn sprites in the player's shared anatomical coordinates.

Sources are complete, posed figures made with imagegen, not inventory pictures.
Only framing, layer separation, alpha cleanup and atlas packing happen here.
The original class, bestiary and inventory banks are left intact.
"""
import argparse
import colorsys
import hashlib
import io
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageChops, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "artifacts/joueur-equipe-2026-10-04"
RANGED_SOURCE = ROOT / "artifacts/joueur-tir-deux-mains-2026-10-04"
CORRECTED_SOURCE = ROOT / "artifacts/joueur-calques-corriges-2026-10-04"
ANATOMY_SOURCE = ROOT / "artifacts/joueur-anatomie-2026-10-04"
OUTPUT = ROOT / "assets/sprites/cyberpunk"
SIDE = 128
SCALE = SIDE // 32


def posed_frames(name, columns, rows, count, source=SOURCE):
    sheet = Image.open(source / name).convert("RGBA")
    result = []
    for index in range(count):
        col, row = index % columns, index // columns
        frame = sheet.crop((round(col * sheet.width / columns),
                            round(row * sheet.height / rows),
                            round((col + 1) * sheet.width / columns),
                            round((row + 1) * sheet.height / rows)))
        alpha = frame.getchannel("A").point(lambda a: 255 if a >= 128 else 0)
        bounds = alpha.getbbox()
        if not bounds:
            raise ValueError(f"Empty figure: {name}/{index}")
        frame.putalpha(alpha)
        frame = frame.crop(bounds)
        factor = 120 / frame.height
        size = (round(frame.width * factor), 120)
        if size[0] > 120:
            raise ValueError(f"Unexpected anatomy: {name}/{index}/{size}")
        tile = Image.new("RGBA", (SIDE, SIDE))
        tile.paste(frame.resize(size, Image.Resampling.NEAREST),
                   ((SIDE - size[0]) // 2, 4))
        result.append(tile)
    return result


def polygon(points):
    mask = Image.new("L", (SIDE, SIDE))
    ImageDraw.Draw(mask).polygon([(round(x * SCALE), round(y * SCALE)) for x, y in points], fill=255)
    return mask


HEAD = polygon([(8, 0), (24, 0), (24, 8.5), (21, 11.75), (11, 11.75), (8, 8.5)])
HEAD_CLEAR = polygon([(8, 0), (24, 0), (24, 5.5), (21, 8.5),
                      (21, 11.75), (11, 11.75), (11, 8.5), (8, 5.5)])
HANDS = Image.new("L", (SIDE, SIDE))
draw = ImageDraw.Draw(HANDS)
for left, right in [(0, 11), (21, 32)]:
    draw.rectangle((left * SCALE, round(17.25 * SCALE), right * SCALE, 23 * SCALE), fill=255)
FEET = polygon([(0, 24), (32, 24), (32, 32), (0, 32)])
TORSO = polygon([(0, 8.25), (13, 8.25), (14, 9.75), (18, 9.75), (19, 8.25), (32, 8.25),
                 (32, 21), (21, 21), (21, 20), (11, 20), (11, 21), (0, 21)])
TORSO = ImageChops.subtract(TORSO, HANDS)
COAT = ImageChops.lighter(TORSO, polygon([(10, 19), (22, 19), (23, 24), (9, 24)]))
TWO_HANDS = polygon([(10, 14), (17, 14), (17, 19), (10, 19)])
TWO_HANDS = ImageChops.lighter(TWO_HANDS,
    polygon([(18, 11), (24, 11), (24, 17), (18, 17)]))
RAISED_HANDS = polygon([(0, 17.25), (11, 17.25), (11, 23), (0, 23)])
RAISED_HANDS = ImageChops.lighter(RAISED_HANDS,
    polygon([(21.5, 5.5), (27, 5.5), (27, 11.25), (21.5, 11.25)]))
TWO_FINGERS = polygon([(11, 15), (12, 14.25), (15.75, 14.25), (16.25, 15),
                       (16.25, 17.25), (15, 18.25), (11.5, 18.25), (11, 17.5)])
TWO_FINGERS = ImageChops.lighter(TWO_FINGERS,
    polygon([(19, 12.5), (20, 11.5), (22.5, 11.5), (23.25, 12.5),
             (23.25, 15.5), (22.25, 16.25), (19.5, 16.25), (19, 15.5)]))
RAISED_FINGERS = polygon([(0, 17.5), (11, 17.5), (11, 23), (0, 23)])
RAISED_FINGERS = ImageChops.lighter(RAISED_FINGERS,
    polygon([(22.5, 6), (25.5, 6), (26, 7.5), (26, 10), (25, 10.75),
             (22, 10.75), (21.75, 8), (21.75, 7)]))
TWO_PALMS = polygon([(9.25, 15), (11, 14.25), (16, 14.25), (16.5, 17.75),
                     (15, 18.25), (11, 18.25), (9.25, 17.5)])
TWO_PALMS = ImageChops.lighter(TWO_PALMS,
    polygon([(19, 11.5), (22.75, 11.5), (24.75, 13), (24.75, 15.5),
             (23, 16.25), (19, 16.25)]))


def part(frame, mask):
    frame = frame.copy()
    frame.putalpha(ImageChops.multiply(frame.getchannel("A"), mask))
    pixels = frame.get_flattened_data() if hasattr(frame, "get_flattened_data") else frame.getdata()
    frame.putdata([pixel if pixel[3] else (0, 0, 0, 0) for pixel in pixels])
    return frame


def hand_part(frame, mask, neutral=False):
    """Separate a worn layer without carrying the class shirt into it."""
    frame = part(frame, mask)
    data = []
    pixels = frame.get_flattened_data() if hasattr(frame, "get_flattened_data") else frame.getdata()
    for red, green, blue, alpha in pixels:
        hue, saturation, value = colorsys.rgb_to_hsv(red / 255, green / 255, blue / 255)
        shirt = ((hue < 0.040 or hue > 0.95) and saturation > 0.55 and value > 0.05)
        if alpha and (shirt or (neutral and saturation > 0.25)):
            data.append((0, 0, 0, 0))
        else:
            data.append((red, green, blue, alpha))
    frame.putdata(data)
    return frame


def raised_hand_frames():
    sheet = Image.open(ANATOMY_SOURCE / "mains-pistolet-profil.png").convert("RGBA")
    result = []
    for index in range(7):
        frame = sheet.crop((round(index * sheet.width / 7), 0,
                            round((index + 1) * sheet.width / 7), sheet.height))
        frame.putalpha(frame.getchannel("A").point(lambda a: 255 if a >= 128 else 0))
        # Ignore isolated artifacts or a few pixels from the neighboring cell.
        # Dilating only for the framing search groups the curled fingers with
        # their palm; the original alpha and artwork remain untouched.
        mask = frame.getchannel("A").filter(ImageFilter.MaxFilter(21))
        pending_pixels = {(x, y) for y in range(mask.height) for x in range(mask.width)
                          if mask.getpixel((x, y))}
        largest = set()
        while pending_pixels:
            pending = [pending_pixels.pop()]
            component = set(pending)
            while pending:
                x, y = pending.pop()
                for point in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
                    if point in pending_pixels:
                        pending_pixels.remove(point)
                        component.add(point)
                        pending.append(point)
            if len(component) > len(largest):
                largest = component
        opaque = [(x, y) for x, y in largest if frame.getpixel((x, y))[3]]
        frame = frame.crop((min(x for x, y in opaque), min(y for x, y in opaque),
                            max(x for x, y in opaque)+1, max(y for x, y in opaque)+1))
        width = min(12, round(frame.width * 16 / frame.height))
        frame = frame.resize((width, 16), Image.Resampling.NEAREST)
        tile = Image.new("RGBA", (SIDE, SIDE))
        tile.paste(frame, (24 * SCALE - width // 2, 6 * SCALE))
        result.append(hand_part(tile, polygon([(0, 0), (32, 0), (32, 32), (0, 32)])))
    return result


def cuff_center(frame, roi):
    """Locate the enclosed dark cuff opening in the actual worn drawing."""
    left, top, right, bottom = [round(value * SCALE) for value in roi]
    pending_pixels = {(x, y) for y in range(top, bottom) for x in range(left, right)
                      if frame.getpixel((x, y))[3] >= 128
                      and max(frame.getpixel((x, y))[:3]) < 20}
    # Some openings touch the dark sleeve outline by a single source pixel.
    # Remove that thin bridge before measuring the interior, not the outline.
    pending_pixels = {(x, y) for x, y in pending_pixels
                      if all((x + dx, y + dy) in pending_pixels
                             for dx in (-1, 0, 1) for dy in (-1, 0, 1))}
    components = []
    while pending_pixels:
        pending = [pending_pixels.pop()]
        component = []
        while pending:
            x, y = pending.pop()
            component.append((x, y))
            for nx, ny in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
                if (nx, ny) in pending_pixels:
                    pending_pixels.remove((nx, ny))
                    pending.append((nx, ny))
        if len(component) >= 4 and not any(
                x in (left, right-1) or y in (top, bottom-1) for x, y in component):
            components.append(component)
    if not components:
        raise ValueError(f"No isolated cuff opening in {roi}")
    component = max(components, key=len)
    return [round(sum(point[axis] + 0.5 for point in component) / len(component) / SCALE * 4) / 4
            for axis in (0, 1)]


def atlas(frames, side):
    image = Image.new("RGBA", (16 * side, ((len(frames) + 15) // 16) * side))
    for index, frame in enumerate(frames):
        if side != SIDE:
            frame = frame.resize((side, side), Image.Resampling.NEAREST)
        image.paste(frame, (index % 16 * side, index // 16 * side))
    buffer = io.BytesIO()
    image.save(buffer, "PNG", optimize=True)
    return buffer.getvalue()


def class_accents(frame, body):
    """Retain the class-colored lining visible between the new worn pieces."""
    if body == 0:
        return frame
    frame = frame.copy()
    data = []
    pixels = frame.get_flattened_data() if hasattr(frame, "get_flattened_data") else frame.getdata()
    for red, green, blue, alpha in pixels:
        hue, saturation, value = colorsys.rgb_to_hsv(red / 255, green / 255, blue / 255)
        # The reference's crimson lining is distinct from brown leather,
        # rust, skin and amber lamps. Only that class signal changes hue.
        if alpha and (hue < 0.020 or hue > 0.98) and saturation > 0.60 and value > 0.35:
            rgb = colorsys.hsv_to_rgb(0.62 if body == 1 else 0.39, saturation, value)
            red, green, blue = [round(channel * 255) for channel in rgb]
        data.append((red, green, blue, alpha))
    frame.putdata(data)
    return frame


def body_variants(bodies, hands, torso, coat, complete_hands=False):
    variants = []
    for body in bodies:
        for bits in range(16):
            mask = Image.new("L", (SIDE, SIDE)) if complete_hands else hands.copy()
            for flag, region in [(1, torso), (2, HEAD_CLEAR), (4, FEET), (8, coat)]:
                if bits & flag:
                    mask = ImageChops.lighter(mask, region)
            variants.append(part(body, ImageChops.invert(mask)))
    return variants + [hand_part(body, hands, neutral=True) if complete_hands else part(body, hands)
                       for body in bodies]


def ranged_layers(pose, bodies, heads, feet, gloves, raised_hands):
    hand_region = TWO_HANDS if pose == "two-handed" else RAISED_HANDS
    hands = TWO_FINGERS if pose == "two-handed" else RAISED_FINGERS
    palms = TWO_PALMS if pose == "two-handed" else RAISED_FINGERS
    name = "classes-deux-mains.png" if pose == "two-handed" else "classes-main-levee.png"
    old_bodies = posed_frames(name, 3, 1, 3, RANGED_SOURCE)
    source_bodies = posed_frames("classes-deux-mains-compactes.png" if pose == "two-handed"
                                else "classes-bras-leve-compactes.png", 3, 1, 3, ANATOMY_SOURCE)
    if pose == "two-handed":
        aligned = []
        for frame in source_bodies:
            tile = Image.new("RGBA", (SIDE, SIDE))
            tile.paste(frame, (0, -SCALE))
            aligned.append(tile)
        source_bodies = aligned
    # Replace only the upper body. The accepted face and lower body are copied
    # unchanged from the standing sprite; the original hanging arms are erased.
    upper = ImageChops.lighter(ImageChops.lighter(TORSO, HANDS), hand_region)
    posed = []
    for original, changed in zip(bodies, source_bodies):
        frame = part(original, ImageChops.invert(upper))
        frame.alpha_composite(part(changed, upper))
        posed.append(frame)
    # Clothing is continuous behind the hands and weapon. The handless source
    # reveals the chest instead of punching rectangular holes through it.
    torso = upper
    coat = ImageChops.lighter(torso, polygon([(10, 19), (22, 19), (23, 24), (9, 24)]))
    source_torso = posed_frames("torses-deux-mains-compacts.png" if pose == "two-handed"
                               else "torses-bras-leve-compacts.png", 4, 4, 14, ANATOMY_SOURCE)
    if pose == "two-handed":
        # This source sheet has a taller head/shorter legs than the base.
        # Its shoulder-to-wrist span matches after this framing adjustment.
        aligned = []
        for frame in source_torso:
            tile = Image.new("RGBA", (SIDE, SIDE))
            tile.paste(frame, (0, -2 * SCALE))
            aligned.append(tile)
        source_torso = aligned
    rois = [(8.5, 12.5, 15, 19), (19, 12.5, 26, 19)] if pose == "two-handed" else [
        (4.5, 16, 11, 21.5), (21, 7, 28, 14)]
    wrists = []
    for index, frame in enumerate(source_torso):
        local_rois = list(rois)
        if pose == "two-handed" and index == 7:
            # This model's dark waist pouch sits directly below the cuff.
            local_rois[1] = (19, 12.5, 23, 17)
        wrists.append([cuff_center(frame, roi) for roi in local_rois])
    base_rois = rois if pose == "two-handed" else [(4.5, 15, 11, 21.5), (20.5, 6.5, 25, 10.75)]
    base_wrists = [[cuff_center(frame, roi) for roi in base_rois] for frame in source_bodies]
    worn = [hand_part(frame, coat if index in (4, 5, 8) else torso)
            for index, frame in enumerate(source_torso)]
    worn += [hand_part(frame, HEAD) for frame in heads]
    def hand_layer(frame, mask, index=None, neutral=False):
        old = hand_part(frame, mask, neutral=neutral)
        # Isolated hand pieces share one anatomical size, independently of the
        # source sheet's larger gauntlet plates or cuff padding.
        fitted = Image.new("RGBA", (SIDE, SIDE))
        centers = [(13, 16), (21, 14)] if pose == "two-handed" else [(8, 20)]
        for hand, (cx, cy) in enumerate(centers):
            piece = old.crop((0 if hand == 0 else 17 * SCALE, 0,
                              17 * SCALE if hand == 0 else SIDE, SIDE))
            piece = piece.crop(piece.getchannel("A").getbbox())
            height = 14 if pose == "two-handed" else 16
            width = min(16, round(piece.width * height / piece.height))
            piece = piece.resize((width, height), Image.Resampling.NEAREST)
            fitted.alpha_composite(piece, (cx * SCALE - width // 2,
                                           cy * SCALE - height // 2))
        if pose == "raised":
            fitted.alpha_composite(raised_hands[6 if index is None else index])
        return fitted
    worn += [hand_layer(frame, hands, index) for index, frame in enumerate(gloves)]
    worn += [part(frame, FEET) for frame in feet]
    # Wrists/palms go behind the weapon; only the smaller finger cutout goes
    # in front. This keeps sleeve joins while preserving the gun silhouette.
    worn += [hand_layer(frame, palms, index) for index, frame in enumerate(gloves)]
    variants = body_variants(posed, hands, torso, coat, complete_hands=True)
    variants[48:51] = [hand_layer(body, hands, neutral=True) for body in old_bodies]
    variants += [hand_layer(body, palms, neutral=True) for body in old_bodies]
    return posed, worn, variants, wrists, base_wrists


def build():
    bodies = posed_frames("classes-visages-v4.png", 3, 1, 3)
    torso = posed_frames("corps-portes.png", 4, 4, 14)
    heads = posed_frames("tetes-portees.png", 3, 2, 6)
    hands = posed_frames("mains-portees.png", 3, 2, 6)
    feet = posed_frames("pieds-portes.png", 3, 2, 6)
    # The longer coats replace the old thigh outline too, rather than floating
    # over the colored class plates. Other chest pieces retain the base legs.
    torso_masks = [COAT if index in (4, 5, 8) else TORSO for index in range(14)]
    worn = [hand_part(frame, mask) for frame, mask in zip(torso, torso_masks)]
    worn += [hand_part(frame, HEAD) for frame in heads]
    worn += [hand_part(frame, HANDS) for frame in hands]
    worn += [hand_part(frame, FEET) for frame in feet]
    masked_bodies = body_variants(bodies, HANDS, TORSO, COAT)
    outputs = {}
    for side in (32, 128):
        outputs[f"player-bodies-{side}.png"] = atlas(masked_bodies, side)
        outputs[f"player-worn-{side}.png"] = atlas(
            [class_accents(frame, body) for body in range(3) for frame in worn], side)
    ranged_gloves = posed_frames("gants-tir.png", 3, 4, 12, RANGED_SOURCE)
    two_gloves = posed_frames("gants-deux-mains.png", 3, 2, 6, RANGED_SOURCE)
    wrist_positions = {}
    base_wrist_positions = {}
    raised_hands = raised_hand_frames()
    for pose, gloves in [("two-handed", two_gloves), ("raised", ranged_gloves[6:])]:
        posed, ranged_worn, variants, wrists, base_wrists = ranged_layers(pose, bodies, heads, feet, gloves, raised_hands)
        wrist_positions[pose] = wrists
        base_wrist_positions[pose] = base_wrists
        for side in (32, 128):
            outputs[f"player-{pose}-bodies-{side}.png"] = atlas(variants, side)
            outputs[f"player-{pose}-worn-{side}.png"] = atlas(
                [class_accents(frame, body) for body in range(3) for frame in ranged_worn[:32]]
                + [class_accents(frame, body) for body in range(3) for frame in ranged_worn[32:]], side)
    pistol = Image.open(RANGED_SOURCE / "pistolet-reference.png").convert("RGBA")
    pistol.putalpha(pistol.getchannel("A").point(lambda a: 255 if a >= 128 else 0))
    pistol = pistol.crop(pistol.getchannel("A").getbbox())
    factor = 118 / max(pistol.size)
    size = tuple(round(value * factor) for value in pistol.size)
    tile = Image.new("RGBA", (128, 128))
    tile.paste(pistol.resize(size, Image.Resampling.NEAREST),
               ((128 - size[0]) // 2, (128 - size[1]) // 2))
    outputs["player-reference-pistol-128.png"] = atlas([tile], 128)
    manifest = {
        "version": 4, "body_count": 3, "body_variants": 16,
        "base_hands_start": 48, "worn_start": 101, "worn_count": 32, "worn_classes": 3,
        "right_grip": [24, 20], "feet_baseline": 31,
        "two_handed_grips": [[13, 16], [21, 14]], "raised_grip": [24, 8],
        "sources": {name: hashlib.sha256((SOURCE / name).read_bytes()).hexdigest()
                    for name in ("classes-visages-v4.png", "corps-portes.png",
                                 "tetes-portees.png", "mains-portees.png", "pieds-portes.png")},
    }
    manifest["ranged_sources"] = {
        name: hashlib.sha256((RANGED_SOURCE / name).read_bytes()).hexdigest()
        for name in ("classes-deux-mains.png", "classes-main-levee.png",
                     "corps-two-handed.png", "corps-raised.png", "gants-tir.png",
                     "gants-deux-mains.png", "pistolet-reference.png")}
    manifest["corrected_sources"] = {
        name: hashlib.sha256((CORRECTED_SOURCE / name).read_bytes()).hexdigest()
        for name in ("corps-two-handed-sans-mains.png", "corps-raised-sans-mains.png",
                     "corps-raised-avant-bras-court.png")}
    manifest["ranged_layering"] = "complete clothing and wrists behind weapon; isolated fingers in front"
    manifest["ranged_back_hands"] = {"body_start": 51, "worn_start": 96, "per_class": 6}
    manifest["wrist_positions"] = wrist_positions
    manifest["base_wrist_positions"] = base_wrist_positions
    manifest["anatomy_sources"] = {
        name: hashlib.sha256((ANATOMY_SOURCE / name).read_bytes()).hexdigest()
        for name in ("torses-deux-mains-compacts.png", "torses-bras-leve-compacts.png",
                     "classes-deux-mains-compactes.png", "classes-bras-leve-compactes.png",
                     "mains-pistolet-profil.png")}
    rust = ["// Generated by tools/art/build_player_worn_assets.py; anatomical cuff centers.",
            "#[rustfmt::skip]"]
    groups = [("TWO_WRISTS", wrist_positions["two-handed"]),
              ("RAISED_WRISTS", wrist_positions["raised"]),
              ("TWO_BASE_WRISTS", base_wrist_positions["two-handed"]),
              ("RAISED_BASE_WRISTS", base_wrist_positions["raised"])]
    for name, positions in groups:
        rust.append(f"const {name}: [[[f32; 2]; 2]; {len(positions)}] = [")
        for wrists in positions:
            rust.append("    [" + ", ".join("[" + ", ".join(f"{v:.2f}" for v in point) + "]"
                                            for point in wrists) + "],")
        rust += ["];", "#[rustfmt::skip]"] if name != "RAISED_BASE_WRISTS" else ["];"]
    outputs["player-grips.rs"] = ("\n".join(rust) + "\n").encode()
    outputs["player-worn.json"] = (json.dumps(manifest, indent=2) + "\n").encode()
    return outputs, bodies, worn, masked_bodies


def preview(bodies, worn, variants):
    board = Image.new("RGBA", (1024, 896), (27, 31, 34, 255))
    draw = ImageDraw.Draw(board)
    for index in range(32):
        body = variants[8 if index in (4, 5, 8) else 1] if index < 14 else variants[
            2 if index < 20 else 0 if index < 26 else 4]
        tile = body.copy()
        tile.alpha_composite(worn[index])
        if index < 20 or index >= 26:
            tile.alpha_composite(variants[48])
        x, y = index % 8 * 128, index // 8 * 180
        board.alpha_composite(tile, (x, y + 16))
        draw.text((x + 14, y + 148), str(index + 101), fill="white")
    for index, body in enumerate(bodies):
        board.alpha_composite(body, (128 + index * 256, 740))
        draw.text((128 + index * 256, 876), ["Breche", "Vigie", "Creuset"][index], fill="white")
    board.save(SOURCE / "assemblage-porte.png")


def main():
    from build_crawl_assets import main as build_native
    build_native()
    return
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    outputs, bodies, worn, variants = build()
    if args.check:
        changed = [name for name, data in outputs.items()
                   if not (OUTPUT / name).is_file() or (OUTPUT / name).read_bytes() != data]
        if changed:
            raise SystemExit("Worn sprites out of date: " + ", ".join(changed))
    else:
        for name, data in outputs.items():
            (OUTPUT / name).write_bytes(data)
        preview(bodies, worn, variants)
    print("Worn player sprites: 3 classes, 32 fitted models, 32/128 px; "
          + ("reproducible" if args.check else "packed"))


if __name__ == "__main__":
    main()
