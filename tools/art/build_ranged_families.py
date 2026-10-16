"""Pack approved gun drawings; no raster drawing or recoloring."""
import argparse
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "artifacts/tir-brutal-propositions-2026-10-05"
OUTPUT = ROOT / "assets/sprites/cyberpunk"
ORDER = (4, 7, 8, 10, 6, 5, 9, 3, 1, 2)

def main():
    from build_crawl_assets import main as build_native
    build_native()
    return
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    for side in (32, 128):
        atlas = Image.new("RGBA", (16 * side, side))
        for cell, source in enumerate(ORDER):
            tile = Image.open(SOURCE / f"arme-{source:02}-{side}.png").convert("RGBA")
            if tile.size != (side, side):
                raise ValueError(f"Invalid source size {source}/{side}")
            atlas.paste(tile, (cell * side, 0))
        path = OUTPUT / f"ranged-families-{side}.png"
        same = path.exists() and Image.open(path).convert("RGBA").tobytes() == atlas.tobytes()
        if args.check and not same:
            raise SystemExit(f"Ranged atlas out of date: {path.name}")
        if not args.check and not same:
            atlas.save(path, optimize=True)
    print("Ranged art: ten firearm families, including pistols and SMGs; "
          + ("reproducible" if args.check else "packed"))

if __name__ == "__main__":
    main()
