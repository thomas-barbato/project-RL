"""Historical baked figures; active entry point delegates to native layers.

Weapons and hands belong to each authored figure. No separate equipment or
weapon layer is assembled by this asset path or by the player renderer.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "artifacts/joueur-quatre-prises-2026-10-05"
UNARMED_SOURCE = ROOT / "artifacts/joueur-tenues-completes-2026-10-04"
RANGED_SOURCE = ROOT / "artifacts/palettes-integration-2026-10-05"
OUTPUT = ROOT / "assets/sprites/cyberpunk"
SIDE = 192
BODY_HEIGHT = 120
POSES = ("side", "melee-two-handed", "two-handed", "raised", "dual-melee", "dual-raised", "dual-mixed")
MODELS = (
    "armure_cerbere", "brigandine_cognefer", "carapace_chrysalide",
    "composite_carapace", "combinaison_fantome", "manteau_de_mue",
    "plastron_bastion", "patched_plating", "veste_de_la_seconde_ombre",
    "veste_de_membranes", "veste_de_peau_seconde", "veste_de_veille",
    "veste_matelassee", "veste_a_fibres_croisees",
)


def components(points):
    points = set(points)
    while points:
        pending = [points.pop()]
        component = set(pending)
        while pending:
            x, y = pending.pop()
            for point in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
                if point in points:
                    points.remove(point)
                    component.add(point)
                    pending.append(point)
        yield component


def frames(name, columns, rows, count, source=SOURCE):
    sheet = Image.open(source / name).convert("RGBA")
    alpha = sheet.getchannel("A").point(lambda a: 255 if a >= 128 else 0)
    grouped = alpha.filter(ImageFilter.MaxFilter(3))
    grouped_pixels = grouped.load()
    original_pixels = alpha.load()
    figures = {}
    for component in components((x,y) for y in range(sheet.height) for x in range(sheet.width) if grouped_pixels[x,y]):
        points = [point for point in component if original_pixels[point]]
        if len(points) < 100: continue
        # Assign the whole connected figure to its cell, then crop its actual bounds.
        # Blades and boots may cross an imaginary grid line; that line must not clip them.
        cx, cy = sum(x for x,y in points)/len(points), sum(y for x,y in points)/len(points)
        index = min(rows-1, int(cy*rows/sheet.height))*columns + min(columns-1, int(cx*columns/sheet.width))
        if index not in figures or len(points) > len(figures[index]): figures[index] = points
    result = []
    for index in range(count):
        points = figures[index]
        left, top = min(x for x,y in points), min(y for x,y in points)
        right, bottom = max(x for x,y in points)+1, max(y for x,y in points)+1
        frame = sheet.crop((left, top, right, bottom))
        kept = Image.new("L", frame.size)
        for x,y in points: kept.putpixel((x-left,y-top), 255)
        frame.putalpha(kept)
        alpha = frame.getchannel("A")
        bottom = alpha.getbbox()[3]-1
        sole = alpha.crop((0, max(0, bottom-5), frame.width, bottom+1)).getbbox()
        center = (sole[0]+sole[2]-1)/2
        half_band = max(2, round(frame.height*0.10))
        top = alpha.crop((round(center)-half_band, 0, round(center)+half_band+1, bottom+1)).getbbox()[1]
        scale = BODY_HEIGHT/(bottom-top+1)
        size = (round(frame.width*scale), round(frame.height*scale))
        x, y = round(SIDE/2-center*scale), SIDE-4-size[1]
        if x < 1 or y < 1 or x+size[0] >= SIDE:
            raise ValueError(f"Figure exceeds common body frame: {name}/{index}: {size}, {x}/{y}")
        tile = Image.new("RGBA", (SIDE, SIDE))
        tile.paste(frame.resize(size, Image.Resampling.NEAREST), (x, y))
        result.append(tile)
    return result


def atlas(tiles, side):
    image = Image.new("RGBA", (16*side, ((len(tiles)+15)//16)*side))
    for index, tile in enumerate(tiles):
        image.paste(tile.resize((side, side), Image.Resampling.NEAREST),
                    (index%16*side, index//16*side))
    stream = io.BytesIO()
    image.save(stream, format="PNG", optimize=True)
    return stream.getvalue()


def build():
    unarmed = frames("classes-casquees-poses.png", 3, 3, 3, UNARMED_SOURCE)
    unarmed += frames("tenues-side-final.png", 4, 4, 14, UNARMED_SOURCE)
    prepared = {"unarmed": unarmed}
    for pose in POSES:
        if pose in ("two-handed", "raised", "dual-raised", "dual-mixed"):
            prepared[pose] = frames(f"tenues-{pose}.png", 4, 5, 17, RANGED_SOURCE)
            continue
        classes = frames(f"classes-{pose}.png", 3, 1, 3)
        armor = frames(f"tenues-{pose}.png", 4, 4, 14)
        # Canonical single-hand figures hold the weapon in the character RIGHT hand.
        if pose == "side":
            classes = [tile.transpose(Image.Transpose.FLIP_LEFT_RIGHT) for tile in classes]
        prepared[pose] = classes + armor
    output = {}
    for pose, tiles in prepared.items():
        for side in (48, 128):
            output[f"player-poses-{pose}-{side}.png"] = atlas(tiles, side)
    sources = [SOURCE/f"{family}-{pose}.png" for pose in POSES if pose not in ("two-handed", "raised", "dual-raised", "dual-mixed") for family in ("classes", "tenues")]
    sources += [RANGED_SOURCE/f"tenues-{pose}.png" for pose in ("two-handed", "raised", "dual-raised", "dual-mixed")]
    manifest = {
        "version": 3, "outfit_count": 17, "armor_armed_variants": 98,
        "base_classes": ["breche", "vigie", "creuset"],
        "torso_start": 101, "outfit_start": 3, "torso_models": MODELS,
        "appearance": "torso chooses complete outfit; weapon category chooses authored armed figure",
        "poses": ["unarmed", *POSES], "layering": "one complete figure, weapon and hands included",
        "source_frame": SIDE, "body_height": BODY_HEIGHT, "world_cell": 48, "world_body_height": 30, "feet_baseline": 187,
        "sources": {str(path.relative_to(ROOT)).replace("\\", "/"): hashlib.sha256(path.read_bytes()).hexdigest() for path in sources},
        "unarmed_sources": {name: hashlib.sha256((UNARMED_SOURCE/name).read_bytes()).hexdigest()
                            for name in ("classes-casquees-poses.png", "tenues-side-final.png")},
    }
    output["player-poses.json"] = (json.dumps(manifest, indent=2)+"\n").encode()
    return output, prepared


def preview(prepared):
    board = Image.new("RGBA", (1280, 17*176+36), (19, 25, 27, 255))
    draw = ImageDraw.Draw(board)
    for col, pose in enumerate(POSES):
        draw.text((col*160+160, 6), pose, fill="white")
    for index in range(17):
        y = index*176+36
        draw.text((8, y+20), ("Breche", "Vigie", "Creuset")[index] if index<3
                  else str(98+index)+" "+MODELS[index-3], fill="white")
        for col, pose in enumerate(POSES):
            board.alpha_composite(prepared[pose][index].resize((128, 128), Image.Resampling.NEAREST), (160+col*160, y))
            board.alpha_composite(prepared[pose][index].resize((48, 48), Image.Resampling.NEAREST), (200+col*160, y+128))
    board.save(RANGED_SOURCE / "tenues-assemblees.png")


def main():
    from build_crawl_assets import main as build_native
    build_native()
    return
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    output, prepared = build()
    if args.check:
        changed = [name for name, data in output.items()
                   if not (OUTPUT/name).is_file() or (OUTPUT/name).read_bytes()!=data]
        if changed:
            raise SystemExit("Outfits out of date: "+", ".join(changed))
    else:
        for name, data in output.items():
            (OUTPUT/name).write_bytes(data)
        preview(prepared)
    print("Complete player outfits: 3 helmeted classes + 14 torso models, 7 armed poses, 48/128 px; common 30px body; "
          + ("reproducible" if args.check else "packed"))


if __name__ == "__main__":
    main()
