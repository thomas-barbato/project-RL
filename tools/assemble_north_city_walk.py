"""Pack unmodified engine captures as a lossless walkthrough animation."""
from pathlib import Path
import argparse
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "artifacts/north-city-01"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--keep-frames", action="store_true")
    args = parser.parse_args()
    checks = json.loads((OUT / "checks.json").read_text(encoding="utf-8"))
    walk = checks["house_walk"]
    paths = [OUT / "walk-frames" / f"frame-{f['frame']:03}.png" for f in walk["frames"]]
    frames = []
    for path in paths:
        with Image.open(path) as source:
            assert source.size == (walk["width"], walk["height"])
            frames.append(source.convert("RGBA"))
    destination = OUT / walk["animation"]
    frames[0].save(destination, format="WEBP", save_all=True, append_images=frames[1:],
                   duration=[900 if f["pause"] else 120 for f in walk["frames"]],
                   lossless=True, method=4, loop=0)
    with Image.open(destination) as result:
        assert result.n_frames == len(frames)
        for i, original in enumerate(frames):
            result.seek(i)
            # No crop, resize, repaint or interpolation: preserve every pixel.
            assert result.convert("RGBA").tobytes() == original.tobytes()
    walk["animation_qa"] = dict(lossless=True, frames=len(frames),
                               all_frames_pixel_equal_to_engine=True,
                               bytes=destination.stat().st_size)
    (OUT / "checks.json").write_text(json.dumps(checks, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    if not args.keep_frames:
        temporary = (OUT / "walk-frames").resolve()
        assert temporary.parent == OUT.resolve() and temporary.name == "walk-frames"
        for path in paths:
            assert path.resolve().parent == temporary
            path.unlink()
    print(f"{destination}: {len(frames)} unchanged native engine frames")


if __name__ == "__main__":
    main()
