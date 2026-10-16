"""QA layout of the actual sprite banks; crop/scale only."""
from pathlib import Path
from PIL import Image, ImageDraw
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
board = Image.new("RGBA", (960, 286), (10, 18, 17, 255))
draw = ImageDraw.Draw(board)
for col, (pose, label, grip) in enumerate((
    ("raised", "PISTOLET", (27, 52, 52, 76)),
    ("dual-raised", "DEUX PISTOLETS", (27, 52, 52, 76)),
    ("dual-mixed", "EPEE + PISTOLET", (78, 53, 103, 77)),
)):
    x = col * 320
    draw.text((x + 14, 12), label, fill=(140, 225, 190))
    full = Image.open(ROOT / f"assets/sprites/cyberpunk/player-poses-{pose}-128.png").convert("RGBA").crop((0, 0, 128, 128))
    native = Image.open(ROOT / f"assets/sprites/cyberpunk/player-poses-{pose}-48.png").convert("RGBA").crop((0, 0, 48, 48))
    board.alpha_composite(full, (x + 6, 36))
    board.alpha_composite(full.crop(grip).resize((150, 144), Image.Resampling.NEAREST), (x + 150, 42))
    board.alpha_composite(native, (x + 44, 206))
    draw.text((x + 136, 221), "TAILLE NATIVE / CORPS 30 PX", fill=(120, 169, 149))
    draw.text((x + 153, 191), "PRISE AGRANDIE", fill=(120, 169, 149))
board.save(HERE / "mains-pistolet-final.png")
