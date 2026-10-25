"""Condition the R08 whole-figure proposal; never modify the runtime bank."""
from __future__ import annotations

import hashlib
import base64
import io
import json
import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from build_player_walk_matrix_06 import grid_figures

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / "docs/reviews/player-walk-07/m100-torso-r08"
SOURCE = ROOT / "assets/source-art/player-walk-review-07"
RUNTIME = ROOT / "assets/animations/walk-07/creatures/MON-099-0.png"


def digest(tile):
    return hashlib.sha256(tile.tobytes()).hexdigest()


def main():
    baseline = json.loads((DOC / "rejection.json").read_text("utf-8"))
    before = RUNTIME.read_bytes()
    assert hashlib.sha256(before).hexdigest() == baseline["runtime_sha256_before"]
    runtime = Image.open(RUNTIME).convert("RGBA")
    right = [runtime.crop((i * 128, 128, (i + 1) * 128, 256)) for i in range(7)]
    assert [digest(p) for p in right] == baseline["right_pose_sha256_before"]
    figures, warnings = grid_figures(Image.open(SOURCE / "M100-r08-left-cycle.png").convert("RGBA"), 4, 2)
    assert not warnings, warnings
    figures = [p.crop(p.getbbox()) for p in figures[:7]]
    b = right[0].getbbox()
    # One scale for the complete left series. No per-frame anatomy/size edits.
    scale = (b[3] - b[1]) / figures[0].height
    assert max(p.height * scale for p in figures) <= 108
    assert max(p.width * scale for p in figures) <= 120
    left = []
    for figure in figures:
        reduced = figure.resize((round(figure.width * scale), round(figure.height * scale)), Image.Resampling.LANCZOS)
        tile = Image.new("RGBA", (128, 128))
        tile.alpha_composite(reduced, (round(64 - reduced.width / 2), 112 - reduced.height))
        left.append(tile)
    poses = left + right
    atlas = Image.new("RGBA", (896, 256))
    for i, tile in enumerate(poses):
        atlas.paste(tile, ((i % 7) * 128, (i // 7) * 128))
    target = SOURCE / "M100-r08-whole-body-native.png"
    atlas.save(target)
    assert atlas.crop((0, 128, 896, 256)).tobytes() == runtime.crop((0, 128, 896, 256)).tobytes()
    boxes = [p.getbbox() for p in poses]
    union = [min(b[0] for b in boxes), min(b[1] for b in boxes), max(b[2] for b in boxes), max(b[3] for b in boxes)]
    offset = math.floor(112 - 32 - (union[1] + union[3]) / 2 + .5) if union[3] - union[1] < 64 else 0
    font = ImageFont.truetype("C:/Windows/Fonts/arial.ttf", 16)
    board = Image.new("RGB", (7 * 256, 2 * 272), "#303a40")
    draw = ImageDraw.Draw(board)
    for i, tile in enumerate(poses):
        row, col = divmod(i, 7)
        draw.text((col * 256 + 12, row * 272 + 5), ("Gauche R08" if row == 0 else "Droite conservee") + (" / repos" if col == 0 else f" / {col}"), font=font, fill="#f2f4f5")
        board.paste(tile.resize((256, 256), Image.Resampling.NEAREST), (col * 256, row * 272 + 16), tile.resize((256, 256), Image.Resampling.NEAREST))
    board.save(DOC / "whole-body-pair-x2.png")
    # A focused still for evaluating the chest perspective, not a one-direction sheet.
    detail = Image.new("RGB", (640, 320), "#303a40")
    for side, tile in enumerate((left[0], right[0])):
        enlarged = tile.resize((384, 384), Image.Resampling.NEAREST)
        detail.paste(enlarged, (side * 320 - 32, -64), enlarged)
    ImageDraw.Draw(detail).text((12, 6), "Gauche R08", font=font, fill="#f2f4f5")
    ImageDraw.Draw(detail).text((332, 6), "Droite conservee", font=font, fill="#f2f4f5")
    detail.save(DOC / "torso-opposite-directions-x3.png")
    assert RUNTIME.read_bytes() == before
    report = {
        "review_id": "WALK07-M100-R08", "status": "awaiting_visual_review",
        "runtime_integrated": False, "source": str(SOURCE / "M100-r08-left-cycle.png"),
        "native_proposal": str(target), "uniform_scale_left": scale,
        "left_pose_sizes_before_reduction": [list(p.size) for p in figures],
        "cycle_bounds": union, "vertical_offset": offset, "foot": [64, 112],
        "right_7_poses_exact_rgba": True, "runtime_file_unchanged": True,
        "native_rgba_sha256": digest(atlas), "native_file_sha256": hashlib.sha256(target.read_bytes()).hexdigest(),
        "pose_rgba_sha256": [digest(p) for p in poses], "warnings": warnings,
        "conditioning": "whole-figure alpha crop, one uniform scale across 7 left poses, fixed foot anchor; no limb edits",
        "visual_orientation_requires_human_review": True,
    }
    (DOC / "proposal.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", "utf-8")
    encoded = io.BytesIO()
    atlas.save(encoded, "WEBP", lossless=True, quality=100, exact=True, method=6)
    assert Image.open(io.BytesIO(encoded.getvalue())).convert("RGBA").tobytes() == atlas.tobytes()
    data = {"atlas": "data:image/webp;base64," + base64.b64encode(encoded.getvalue()).decode("ascii")}
    template = (ROOT / "tools/art/walk07_m100_torso_r08.html").read_text("utf-8")
    fragment = template.replace("__M100_R08_DATA__", json.dumps(data, separators=(",", ":")))
    assert len(fragment.encode("utf-8")) < 1_000_000
    visual = Path("C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/m100-torso-r08.html")
    visual.write_text(fragment, "utf-8")
    report["preview"] = str(visual)
    report["preview_bytes"] = len(fragment.encode("utf-8"))
    (DOC / "proposal.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", "utf-8")
    print(json.dumps({k: report[k] for k in ("status", "cycle_bounds", "uniform_scale_left", "right_7_poses_exact_rgba", "runtime_file_unchanged")}))


if __name__ == "__main__":
    main()
