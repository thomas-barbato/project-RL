"""Package the approved review assets; never modifies the original drawings.

Run with Python and Pillow. --check verifies reproducibility and coverage.
The client embeds the outputs, so no Python or review server is needed to play.
"""
import argparse
import io
import json
import math
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / "artifacts/sprites-cyberpunk-crawl-review-2026-10-04"
ACTOR_REVIEW = ROOT / "artifacts/personnages-echelle-32-2026-10-04"
OUTPUT = ROOT / "assets/sprites/cyberpunk"
COLS = 16
TWO_HANDED_CATEGORIES = {"assault_rifles", "energy_rifles", "flamethrowers",
                         "grenade_launchers", "machine_guns", "rifles",
                         "rocket_launchers", "shotguns"}
ONE_HANDED_CATEGORIES = {"pistols", "handguns", "revolvers", "submachine_guns"}


def weapon_hold(model):
    """Visual grip metadata only; attack range never decides handedness."""
    if model["group"] == "melee":
        return "MeleeTwoHanded" if model["category"] in {"axes", "swords"} else "Side"
    if model["category"] in ONE_HANDED_CATEGORIES:
        return "Raised"
    if (model["category"] in TWO_HANDED_CATEGORIES or
            model["id"] in ("core:flamethrower", "core:needle_launcher")):
        return "TwoHanded"
    raise ValueError("Missing visual handedness: " + model["id"])


def read(name):
    return json.loads((REVIEW / name).read_text(encoding="utf-8"))


def grid_image(sprite):
    image = Image.new("RGBA", (sprite["width"], sprite["height"]))
    colors = [(0, 0, 0, 0) if c is None else tuple(bytes.fromhex(c[1:])) + (255,)
              for c in sprite["colors"]]
    image.putdata([colors[p] for row in sprite["pixels"] for p in row])
    return image


def js_round(value):
    return math.floor(value + 0.5)


def cell(image, model):
    x, y = model["index"] % model["columns"], model["index"] // model["columns"]
    return image.crop((js_round(x * image.width / model["columns"]),
                       js_round(y * image.height / model["rows"]),
                       js_round((x + 1) * image.width / model["columns"]),
                       js_round((y + 1) * image.height / model["rows"])))


def portrait(source):
    bounds = source.getchannel("A").point(lambda a: 255 if a > 32 else 0).getbbox()
    if not bounds:
        raise ValueError("Empty source illustration")
    cropped = source.crop(bounds)
    scale = min(118 / cropped.width, 118 / cropped.height)
    size = (max(1, js_round(cropped.width * scale)), max(1, js_round(cropped.height * scale)))
    image = Image.new("RGBA", (128, 128))
    image.paste(cropped.resize(size, Image.Resampling.LANCZOS),
                ((128 - size[0]) // 2, (128 - size[1]) // 2))
    return image


def atlas(images, side):
    image = Image.new("RGBA", (COLS * side, math.ceil(len(images) / COLS) * side))
    for index, tile in enumerate(images):
        assert tile.size == (side, side)
        assert tile.getchannel("A").getbbox(), index
        image.paste(tile, ((index % COLS) * side, (index // COLS) * side))
    buffer = io.BytesIO()
    image.save(buffer, format="PNG", optimize=True)
    return buffer.getvalue()


def contact(image):
    pixels = image.load()
    bottom = max(y for y in range(32) for x in range(32) if pixels[x, y][3] > 128)
    xs = [x for y in range(max(0, bottom - 3), bottom + 1)
          for x in range(32) if pixels[x, y][3] > 128]
    width = max(8, min(20, max(xs) - min(xs) + 3))
    center = max(math.ceil(width / 2), min(32 - math.ceil(width / 2), js_round((min(xs) + max(xs)) / 2)))
    return [center, min(30, bottom), width]


def silhouette(source):
    bounds = source.getchannel("A").point(lambda a: 255 if a > 32 else 0).getbbox()
    if not bounds:
        raise ValueError("Empty actor or weapon")
    return source.crop(bounds)


def actor_tile(source):
    # Crop transparent source margins before fitting, and anchor the feet.
    cropped = silhouette(source)
    factor = min(30 / cropped.width, 30 / cropped.height)
    size = (max(1, js_round(cropped.width * factor)), max(1, js_round(cropped.height * factor)))
    tile = Image.new("RGBA", (32, 32))
    tile.paste(cropped.resize(size, Image.Resampling.NEAREST), ((32 - size[0]) // 2, 31 - size[1]))
    return tile


def held_tile(source, model, side):
    # A separate presentation layer: switching weapons never selects a body.
    cropped = silhouette(source)
    category = model["category"]
    if model["group"] == "firearms":
        width, height, right, top = 24, 11, 30, 14
    elif category == "spears":
        width, height, right, top = 7, 26, 29, 4
    elif category == "knives":
        width, height, right, top = 7, 14, 29, 11
    else:
        width, height, right, top = 12, 22, 30, 7
    factor = min(width / cropped.width, height / cropped.height)
    size = (max(1, js_round(cropped.width * factor)), max(1, js_round(cropped.height * factor)))
    scale = side // 32
    image = Image.new("RGBA", (side, side))
    image.paste(cropped.resize((size[0] * scale, size[1] * scale),
                              Image.Resampling.NEAREST if side == 32 else Image.Resampling.LANCZOS),
                ((right - size[0]) * scale, top * scale))
    return image


def build():
    equipment, supplies = read("equipment-catalog.json"), read("pickup-catalog.json")
    models = equipment["models"] + supplies["models"]
    icons = read("ground-icons-jupiter/native-grid-source.json")
    native = read("native/native-grid-source.json")
    keys = [icon["key"] for icon in icons["icons"]]
    assert len(keys) == 16 and len(models) == 150
    assert set(icons["mapping"]) == {model["id"] for model in models}
    assert {icons["mapping"][m["id"]] for m in models if m["group"] == "armor"} == {"armor"}
    images = {}
    pictures = []
    held32, held128, held_models = [], [], []
    for model in models:
        assert (ROOT / model["source"]).is_file(), model["id"]
        source = images.setdefault(model["atlas"], Image.open(REVIEW / model["atlas"]).convert("RGBA"))
        frame = cell(source, model)
        pictures.append(portrait(frame))
        if model["group"] in ("firearms", "melee"):
            held_models.append({"id": model["id"], "index": len(held32), "hold": weapon_hold(model)})
            held32.append(held_tile(frame, model, 32))
            held128.append(held_tile(frame, model, 128))
    actor32, actor128 = [], []
    source = Image.open(ACTOR_REVIEW / "classes-couleurs-v3.png").convert("RGBA")
    for index in range(3):
        frame = cell(source, {"index": index, "columns": 3, "rows": 1})
        actor32.append(actor_tile(frame))
        actor128.append(portrait(frame))
    source = Image.open(REVIEW / "bestiaire-face-v2.png").convert("RGBA")
    for f in read("face-bestiary-frames.json")["frames"]:
        frame = source.crop((f["x"], f["y"], f["x"] + f["width"], f["y"] + f["height"]))
        actor32.append(actor_tile(frame))
        actor128.append(portrait(frame))
    floors = [grid_image(tile) for family in native["floors"] for tile in family]
    floors.append(grid_image(native["wall"]))
    ground = [grid_image(icon["sprite"]) for icon in icons["icons"]]
    melee_source = ROOT/"artifacts/joueur-quatre-prises-2026-10-05/icone-melee-source.png"
    sword = Image.open(melee_source).convert("RGBA")
    sword.putalpha(sword.getchannel("A").point(lambda a: 255 if a >= 128 else 0))
    sword = sword.crop(sword.getchannel("A").getbbox())
    sword.thumbnail((28, 28), Image.Resampling.NEAREST)
    tile = Image.new("RGBA", (32, 32))
    tile.paste(sword, ((32-sword.width)//2, (32-sword.height)//2))
    ground[keys.index("melee")] = tile
    outputs = {
        "items-128.png": atlas(pictures, 128),
        "ground-32.png": atlas(ground, 32),
        "actors-32.png": atlas(actor32, 32),
        "actors-128.png": atlas(actor128, 128),
        "held-weapons-32.png": atlas(held32, 32),
        "held-weapons-128.png": atlas(held128, 128),
        "terrain-32.png": atlas(floors, 32),
    }
    entries = [{"id": m["id"], "illustration": i, "icon": keys.index(icons["mapping"][m["id"]]),
                "source": m["atlas"]} for i, m in enumerate(models)]
    # Append the approved one-hand families without shifting any old index.
    # Their portraits/icons use the separate ranged atlas at runtime; no new
    # raster artwork is generated or pasted into the archived individual bank.
    plan = json.loads((ROOT / "artifacts/pistolets-pm-integration-2026-10-05/modeles.json").read_text(encoding="utf-8"))
    for family in plan["families"]:
        for model in family["models"]:
            entries.append({"id": "core:" + model["id"], "illustration": len(entries),
                            "icon": keys.index("ranged"), "source": "ranged-families-128.png",
                            "family_cell": family["art_family"]})
            held_models.append({"id": "core:" + model["id"], "index": len(held_models), "hold": "Raised"})
    manifest = {"version": 3, "columns": COLS, "item_count": len(entries), "individual_item_count": len(models), "icon_keys": keys,
                "actor_count": len(actor32), "terrain_count": len(floors), "items": entries,
                "actor_contacts": [contact(image) for image in actor32],
                "actor_bounds": [list(image.getchannel("A").point(lambda a: 255 if a > 128 else 0).getbbox())
                                 for image in actor32],
                "class_bodies": [{"id": "core:breche", "body": 0, "color": "red"},
                                 {"id": "core:vigie", "body": 1, "color": "blue"},
                                 {"id": "core:creuset", "body": 2, "color": "green"}],
                "held_weapons": held_models,
                "actor_sources": [str((ACTOR_REVIEW / "classes-couleurs-v3.png").relative_to(ROOT)).replace("\\", "/"),
                                  "bestiaire-face-v2.png", "face-bestiary-frames.json"],
                "source_review": str(REVIEW.relative_to(ROOT)).replace("\\", "/")}
    outputs["manifest.json"] = (json.dumps(manifest, ensure_ascii=False, indent=2) + "\n").encode()
    lines = ["// Generated by tools/art/build_cyberpunk_assets.py; do not edit by hand.",
             "#[rustfmt::skip]", "pub(super) fn item(id: &str) -> Option<ItemArt> {", "    Some(match id {"]
    lines += [f'        "{e["id"]}" => ItemArt {{ icon: {e["icon"]}, illustration: {e["illustration"]} }},'
              for e in entries]
    lines += ["        _ => return None,", "    })", "}",
              "#[rustfmt::skip]", "pub(super) fn held_weapon(id: &str) -> Option<u16> {", "    Some(match id {"]
    lines += [f'        "{e["id"]}" => {e["index"]},' for e in held_models]
    lines += ["        _ => return None,", "    })", "}",
              "#[rustfmt::skip]", "pub(super) fn weapon_hold(index: u16) -> WeaponHold {",
              "    match index {"]
    lines += [f'        {e["index"]} => WeaponHold::{e["hold"]},' for e in held_models]
    lines += ["        _ => WeaponHold::Side,", "    }", "}",
              f"pub(super) const HELD_WEAPON_COUNT: u16 = {len(held_models)};",
              f"pub(crate) const ACTOR_COUNT: usize = {len(actor32)};",
              "pub(super) const CONTACTS: [[u8; 3]; ACTOR_COUNT] = ["]
    lines += [f"    [{x}, {y}, {w}]," for x, y, w in manifest["actor_contacts"]]
    lines += ["];"]
    return outputs, ("\n".join(lines) + "\n").encode()


def main():
    # The accepted native pipeline owns the active catalogs and atlases now.
    from build_crawl_assets import main as build_native
    build_native()
    return
    check = argparse.ArgumentParser()
    check.add_argument("--check", action="store_true")
    args = check.parse_args()
    outputs, catalog = build()
    targets = {OUTPUT / name: data for name, data in outputs.items()}
    targets[ROOT / "src/game_art_catalog.rs"] = catalog
    if args.check:
        mismatches = [str(path.relative_to(ROOT)) for path, data in targets.items()
                      if not path.exists() or path.read_bytes() != data]
        if mismatches:
            raise SystemExit("Assets out of date: " + ", ".join(mismatches))
    else:
        OUTPUT.mkdir(parents=True, exist_ok=True)
        for path, data in targets.items():
            path.write_bytes(data)
    print("Assets: 162 items (150 archived individual images + 12 ranged family entries), 113 weapon categories; "
          + ("reproducible" if args.check else "packaged"))


if __name__ == "__main__":
    main()
