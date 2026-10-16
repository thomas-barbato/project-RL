"""Package the accepted terrain sheets for the existing game's 32px renderer."""
import argparse
import io
import json
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "artifacts/sprites-cyberpunk-crawl-review-2026-10-04"
OUTPUT = ROOT / "assets/sprites/cyberpunk"


def cell(source, column, row, columns, rows):
    # Match decor.js from the original review, including its separator inset.
    w, h = source.width / columns, source.height / rows
    bounds = (column * w + 3, row * h + 3,
              (column + 1) * w - 3, (row + 1) * h - 3)
    return source.resize((32, 32), Image.Resampling.NEAREST, box=bounds)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    floors = Image.open(SOURCE / "sols-32-review-v1.png").convert("RGBA")
    walls = Image.open(SOURCE / "murs-32-review-v1.png").convert("RGBA")
    tiles = [cell(floors, c, r, 4, 4) for r in range(4) for c in range(4)]
    tiles += [cell(walls, c, r, 2, 2) for r in range(2) for c in range(2)]
    atlas = Image.new("RGBA", (32 * len(tiles), 32))
    for i, tile in enumerate(tiles):
        atlas.paste(tile, (i * 32, 0))
    png = io.BytesIO()
    atlas.save(png, format="PNG")
    sources = json.dumps({
        "sources": ["sols-32-review-v1.png", "murs-32-review-v1.png"],
        "tile_size": 32, "floor_variants": 16, "wall_materials": 4,
        "purpose": "accepted materials for existing decor; no map or loot changes",
        "source_type": "existing concept sheets sampled as in decor.js",
    }, ensure_ascii=False, indent=2) + "\n"
    outputs = {
        "terrain-materials-32.png": png.getvalue(),
        "terrain-materials-sources.json": sources.encode("utf-8"),
    }
    if args.check:
        for name, data in outputs.items():
            if not (OUTPUT / name).is_file() or (OUTPUT / name).read_bytes() != data:
                raise SystemExit(f"Terrain asset differs: {name}")
        print("20 terrain tiles: reproducible assets verified")
    else:
        OUTPUT.mkdir(parents=True, exist_ok=True)
        for name, data in outputs.items():
            (OUTPUT / name).write_bytes(data)
        print("20 accepted terrain tiles packaged")


if __name__ == "__main__":
    main()
