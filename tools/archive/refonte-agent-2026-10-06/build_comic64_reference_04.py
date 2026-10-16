"""Review original BD-STYLE01 drawings at exactly the preferred nearest x2.

This packages existing artwork; it never reconstructs limbs or equipment.
No output goes to active game assets.
"""
from pathlib import Path
import hashlib
import json
import shutil

from PIL import Image, ImageDraw, ImageFont

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
OUT = BASE / "comic64-reference-review-04"
ORIGINAL = BASE / "comic-style-review-01"
NAMES = ["Bastion", "Technomancien", "Traqueur", "Arcaniste", "Métamorphe"]
GEAR = ["Lame et bouclier", "Pistolet", "Fusil de précision",
        "Hampe de canalisation", "Couteau et focaliseur"]
BG, FG, SUB = (14, 21, 29), (235, 240, 244), (160, 182, 196)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def ref(path):
    return {"path": path.relative_to(ROOT).as_posix(), "binary_sha256": sha(path)}


def font(size):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", size)


def main():
    assert not OUT.exists(), "Preserve existing reviews; create a new revision."
    reference = json.loads((BASE / "comic-style-reference-03.json").read_text("utf-8"))
    for entry in reference["files"]:
        assert sha(BASE / entry["path"]) == entry["binary_sha256"]
    active = json.loads((ROOT / "assets/sprites/cyberpunk/agent-rigs/manifest.json").read_text("utf-8"))
    before = {entry["path"]: sha(ROOT / entry["path"]) for entry in active["files"]}
    OUT.mkdir()
    (OUT / "images").mkdir()
    floor = Image.open(BASE / "weapon-silhouette-review-03/review-floor-native32.png").convert("RGBA")
    assert floor.size == (32, 32)
    floor.save(OUT / "images/floor32.png")
    entries, originals, displays = [], [], []
    for index, name in enumerate(NAMES, 1):
        cid = f"BD-C{index:02}"
        original = ORIGINAL / f"{cid}-tile32.png"
        master = ORIGINAL / f"{cid}-comic-master.png"
        tile = Image.open(original).convert("RGBA")
        assert tile.size == (64, 64), "Two-cell composition canvas, not body size."
        doubled = tile.resize((128, 128), Image.Resampling.NEAREST)
        # Every source pixel must become an identical 2x2 RGBA block.
        for y in range(64):
            for x in range(64):
                for dy in (0, 1):
                    for dx in (0, 1):
                        assert doubled.getpixel((2*x+dx, 2*y+dy)) == tile.getpixel((x, y))
        shutil.copy2(original, OUT / "images" / original.name)
        shutil.copy2(master, OUT / "images" / master.name)
        destination = OUT / "images" / f"{cid}-display64.png"
        doubled.save(destination)
        entries.append({
            "id": cid, "number": index, "name": name, "gear": GEAR[index-1],
            "source32": f"images/{original.name}", "display64": f"images/{destination.name}",
            "master": f"images/{master.name}", "original": ref(original),
            "output": ref(destination), "changed_source_pixels": 0,
            "enlargement": "Each original RGBA pixel repeated exactly 2x2; no other treatment."
        })
        originals.append(tile)
        displays.append(doubled)

    def ground(size, side):
        image = Image.new("RGBA", (side, side))
        tile = floor.resize((size, size), Image.Resampling.NEAREST)
        for y in range(0, side, size):
            for x in range(0, side, size):
                image.alpha_composite(tile, (x, y))
        return image

    board = Image.new("RGB", (1500, 760), BG)
    draw = ImageDraw.Draw(board)
    draw.text((24, 18), "BD-STYLE01 · référence conservée · affichage 64 px", font=font(28), fill=FG)
    draw.text((24, 59), "Dessins de la première planche · agrandissement exact ×2 · poses et équipements inchangés", font=font(16), fill=SUB)
    metadata = json.loads((ORIGINAL / "review-BD-STYLE01-01.json").read_text("utf-8"))
    for i, entry in enumerate(entries):
        x = 24 + i*294
        draw.text((x, 105), f"{i+1:02} · {entry['name']}", font=font(19), fill=FG)
        draw.text((x, 137), entry["gear"], font=font(13), fill=SUB)
        source = Image.open(OUT / entry["master"]).convert("RGBA")
        normalization = metadata["candidates"][i]["normalization"][1]
        body_height = normalization["source_body_bottom"] - normalization["source_body_top"]
        ratio = 279 / body_height
        large = source.resize((round(source.width*ratio), round(source.height*ratio)), Image.Resampling.LANCZOS)
        board.paste(large, (x+132-large.width//2, 475-large.height), large)
        draw.text((x+8, 505), "Case 32 px", font=font(14), fill=SUB)
        draw.text((x+122, 505), "Case 64 px · BD ×2", font=font(14), fill=FG)
        scene32 = ground(32, 64)
        scene32.alpha_composite(originals[i])
        scene64 = ground(64, 128)
        scene64.alpha_composite(displays[i])
        board.paste(scene32, (x+10, 563))
        board.paste(scene64, (x+120, 531))
        if i:
            draw.line((x-15, 100, x-15, 680), fill=(37, 50, 61))
    draw.text((24, 704), "Les armes peuvent déborder de la case. Aucun redessin, filtre de palette ou mélange avec les anciens bras.", font=font(15), fill=SUB)
    board.save(OUT / "reference-32-64-board.png")

    # Static scene at exact screen scale, without adding entities or gameplay.
    scene = Image.new("RGBA", (960, 384))
    tile64 = floor.resize((64, 64), Image.Resampling.NEAREST)
    for y in range(0, scene.height, 64):
        for x in range(0, scene.width, 64):
            scene.alpha_composite(tile64, (x, y))
    for i, display in enumerate(displays):
        scene.alpha_composite(display, (32 + i*192, 160))
    scene.save(OUT / "five-characters-at-64.png")
    data = {"review_id": "BD64-REFERENCE04", "characters": entries,
            "floor": "images/floor32.png", "logical_tile_px": 32,
            "preferred_display_tile_px": 64, "canvas": [768, 448]}
    (OUT / "review-data.js").write_text("window.reviewData=" + json.dumps(data, ensure_ascii=False) + ";\n", "utf-8")
    shutil.copy2(BASE / "comic64_reference_template_04.html", OUT / "index.html")
    assert all(sha(BASE / entry["path"]) == entry["binary_sha256"] for entry in reference["files"])
    assert all(sha(ROOT / path) == digest for path, digest in before.items())
    manifest = {
        "review_id": "BD64-REFERENCE04", "date": "2026-10-10",
        "reference": ref(BASE / "comic-style-reference-03.json"),
        "request": "Proceed with original BD-STYLE01 and 64px display if needed.",
        "status": "reference review; no new drawings or art integration",
        "characters": entries, "board": ref(OUT / "reference-32-64-board.png"),
        "scene": ref(OUT / "five-characters-at-64.png"), "gallery": ref(OUT / "index.html"),
        "qa": {"original_reference_files_verified": len(reference["files"]),
               "doubled_rgba_pixels_verified": 5*128*128,
               "active_art_assets_unchanged": len(before), "changed_source_pixels": 0,
               "anatomy_pose_attachment_and_layer_changes": 0},
        "limits": ["Complete equipped reference drawings, not interchangeable equipment layers.",
                   "Scene is an art preview on existing terrain, not a gameplay capture.",
                   "Other catalogue equipment has not been repaired by this review."],
        "generator": ref(Path(__file__))
    }
    (OUT / "review.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+"\n", "utf-8")
    print(json.dumps(manifest["qa"]))


if __name__ == "__main__":
    main()
