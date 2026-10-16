"""Check the scoped EQ-S02 correction and render native comparisons."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OLD = BASE / "shield-review-04"
NEW = BASE / "shield-review-07"

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def font(size):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", size)

def main():
    old = json.loads((OLD / "review-shields-SHIELD-VIS01-04.json").read_text(encoding="utf-8"))
    new = json.loads((NEW / "review-shields-SHIELD-VIS01-07.json").read_text(encoding="utf-8"))
    old_c = {r["candidate_id"]: r for r in old["candidates"]}
    for r in new["candidates"]:
        for field in ("source", "held_layer", "inventory_tile"):
            matches = r[field]["binary_sha256"] == old_c[r["candidate_id"]][field]["binary_sha256"]
            assert matches == (r["candidate_id"] != "EQ-S02"), (r["candidate_id"], field)
    key = lambda r: (r["candidate_id"], r["class_number"], r["shield_hand"], r["other_hand_weapon"])
    old_frames = {key(r): r for r in old["compositions"]}
    preserved = 0
    for r in new["compositions"]:
        before = old_frames[key(r)]
        assert r["native_hand_anchor"] == before["native_hand_anchor"]
        assert r["source_frame"] == before["source_frame"]
        assert r["qa"]["outside_shield_pixel_changes"] == 0
        if r["candidate_id"] != "EQ-S02":
            assert r["binary_sha256"] == before["binary_sha256"]
            preserved += 1
    assert preserved == 150
    checked = []
    def walk(value):
        if isinstance(value, dict):
            if "path" in value and "binary_sha256" in value:
                p = ROOT / value["path"]
                assert sha(p) == value["binary_sha256"], p
                checked.append(str(p))
            if "reference" in value and "reference_sha256" in value:
                p = ROOT / value["reference"]
                digest = hashlib.sha256(p.read_text(encoding="utf-8").encode()).hexdigest()
                assert digest == value["reference_sha256"], p
                checked.append(str(p))
            for v in value.values(): walk(v)
        elif isinstance(value, list):
            for v in value: walk(v)
    walk(new)
    tile = Image.open(ROOT / new["floor_source"]["path"]).convert("RGBA")
    floor = Image.new("RGBA", (64, 64))
    for y in (0, 32):
        for x in (0, 32): floor.alpha_composite(tile, (x, y))
    floors = [Image.new("RGBA", (64, 64), (16, 23, 31, 255)),
              Image.new("RGBA", (64, 64), (173, 182, 184, 255)), floor]
    board = Image.new("RGB", (760, 395), (12, 19, 27))
    d = ImageDraw.Draw(board)
    d.text((18, 12), "EQ-S02 · bordure inférieure", font=font(22), fill=(229, 236, 240))
    d.text((18, 45), "Personnage natif 32 px · zoom ×2 · placement conservé", font=font(13), fill=(160, 182, 196))
    for i, label in enumerate(("Fond sombre", "Fond clair", "Sol du jeu")):
        d.text((180 + i * 190, 81), label, font=font(13), fill=(160, 182, 196))
    for row, (bank, label) in enumerate(((OLD, "Avant · 04"), (NEW, "Après · 07"))):
        y = 113 + row * 133
        d.text((18, y + 21), label, font=font(15), fill=(229, 236, 240))
        frame = Image.open(bank / "frames/EQ-S02-C2-L-pistol.png").convert("RGBA")
        for i, bg in enumerate(floors):
            composed = bg.copy()
            composed.alpha_composite(frame)
            x = 177 + i * 190
            board.paste(composed, (x, y))
            zoom = composed.resize((128, 128), Image.Resampling.NEAREST).crop((16, 16, 112, 112))
            board.paste(zoom, (x + 67, y - 16))
    board.save(NEW / "EQ-S02-before-after-native.png")
    diagnostic = Image.new("RGB", (760, 252), (12, 19, 27))
    dd = ImageDraw.Draw(diagnostic)
    dd.text((18, 12), "EQ-S02 · diagnostic 30 / 32 px", font=font(21), fill=(229, 236, 240))
    dd.text((18, 43), "Le jeu conserve ses tiles 32 px. La réduction à 30 px éprouve la lisibilité.", font=font(12), fill=(160, 182, 196))
    frame = Image.open(NEW / "frames/EQ-S02-C2-L-pistol.png").convert("RGBA")
    for row, size in enumerate((30, 32)):
        y = 89 + row * 76
        dd.text((18, y + 19), f"{size} px", font=font(14), fill=(229, 236, 240))
        for i, bg in enumerate(floors):
            composed = bg.copy()
            composed.alpha_composite(frame)
            scene = composed.resize((2 * size, 2 * size), Image.Resampling.NEAREST)
            x = 177 + i * 190
            diagnostic.paste(scene, (x, y))
            zoom = scene.resize((4 * size, 4 * size), Image.Resampling.NEAREST)
            diagnostic.paste(zoom.crop((size // 2, size // 2, size * 7 // 2, size * 7 // 2)), (x + 67, y - 12))
    diagnostic.save(NEW / "EQ-S02-readability-30-32.png")
    result = {"date": "2026-10-10", "review_id": "SHIELD-VIS01", "revision": 7,
              "revised_candidates": ["EQ-S02"], "unchanged_candidates": ["EQ-S01", "EQ-S03", "EQ-S04", "EQ-S05", "EQ-S06"],
              "unchanged_compositions_verified": preserved, "compositions_checked": 180,
              "all_hand_anchors_preserved": True, "pixels_outside_shield_changes": 0,
              "verified_references": len(checked), "native_held_dimensions": [11, 15],
              "native_body_dimensions": [32, 32], "runtime_integration": "none"}
    (NEW / "validation-07.json").write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    feedback = {"review_id": "SHIELD-VIS01", "revision": 7,
                "feedback": "EQ-S02: bordure argentée inférieure illisible et trop fine; différence de construction trop grande entre haut et bas.",
                "correction": "Lower diagonal silver band and bottom cap broadened; vertical connections beneath the horizontal brace widened inward.",
                "accepted_scope": "existing_hand_centered_placement", "visual_approval": "pending_revised_pixels_review",
                "intermediate_reviews": {"5": "first lower-frame pass", "6": "discarded: native rim and central brace less readable"},
                "other_five_candidates": "identical_to_revision_4", "active_assets_modified": False}
    (NEW / "feedback-and-revision-07.json").write_text(json.dumps(feedback, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    print(json.dumps(result))

if __name__ == "__main__":
    main()
