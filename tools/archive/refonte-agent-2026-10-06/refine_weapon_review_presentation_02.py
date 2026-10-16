"""Keep comparison subjects inside their panels and preserve integer zooms."""
from copy import deepcopy
from pathlib import Path
import runpy
import json
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OUT = BASE / "weapon-silhouette-review-02"
H = runpy.run_path(str(BASE / "package_weapon_silhouette_review_01.py"), run_name="presentation_helpers")
cell, font, save_png, ref, save_json = H["cell"], H["FONT"], H["save_png"], H["ref"], H["save_json"]
W = H["H"]


def main(bank_revision=2):
    global OUT
    OUT = BASE / f"weapon-silhouette-review-{bank_revision:02d}"
    save_png.__globals__["OUT"] = OUT
    art = ROOT / "assets/sprites/cyberpunk"
    body = cell(Image.open(art / "crawl-bodies-32.png").convert("RGBA"), 9, 32)
    body_map = cell(Image.open(art / "crawl-body-materials-32.png").convert("RGBA"), 9, 32)
    weapon = cell(Image.open(art / "crawl-weapons-48.png").convert("RGBA"), 6, 48)
    weapon_map = cell(Image.open(art / "crawl-weapon-materials-48.png").convert("RGBA"), 6, 48)
    pink = [W["recolor_labels"](body, body_map, {1: W["PINK"], 2: W["BLUE"]}), W["recolor_labels"](weapon, weapon_map, {1: W["PINK"], 2: W["BLUE"]})]
    gray = [W["recolor_labels"](body, body_map, {1: W["GRAY"], 2: W["GRAY"]}), W["recolor_labels"](weapon, weapon_map, {1: W["GRAY"], 2: W["GRAY"]})]
    board = Image.new("RGB", (640, 390), "#0c131b")
    d = ImageDraw.Draw(board)
    d.text((18, 12), "COLOR-GRAY01 · Le gris remplace le rose", font=font(22), fill="#eef0f2")
    d.text((18, 48), "Même équipement · mêmes statistiques et bonus · couleurs proposées", font=font(13), fill="#c2ced5")
    for x, images, label in [(18, pink, "Ancienne palette · rose"), (338, gray, "Palette proposée · gris neutre")]:
        d.rectangle((x, 80, x + 281, 373), fill="#182430", outline="#3b4b59")
        d.text((x + 12, 90), label, font=font(15), fill="#d7e1e8")
        for image, dx in zip(images, [30, 110]):
            zoom = image.resize((image.width * 3, image.height * 3), Image.Resampling.NEAREST)
            bounds = zoom.getbbox()
            assert x <= x + dx + bounds[0] and x + dx + bounds[2] <= x + 281
            board.paste(zoom, (x + dx, 135), zoom)
            board.paste(image, (x + dx + 20, 308), image)
    image_ref = save_png("pink-to-neutral-gray-comparison-02.png", board)
    html = (OUT / "weapon-silhouette-gallery.html").read_text(encoding="utf-8")
    html = html.replace('.zoom{width:160px;height:160px}', '.zoom{width:128px;height:128px}')
    html = html.replace("zoom?'Zoom ×3':'Taille native · 1:1'", "zoom?'Zoom entier':'Taille native · 1:1'")
    path = OUT / "weapon-silhouette-gallery-02.html"
    assert not path.exists()
    path.write_text(html, encoding="utf-8", newline="\n")
    previous = OUT / "weapon-silhouette-review-presentation-01.json"
    result = deepcopy(json.loads(previous.read_text(encoding="utf-8")))
    result["revision"] = 2
    result["previous_presentation"] = ref(previous)
    result["neutral_gray_replacement"] = image_ref
    result["gallery"] = ref(path)
    result["gallery_integer_zooms"] = "native64;desktop192=3x;small_screen128=2x"
    save_json(OUT / "weapon-silhouette-review-presentation-02.json", result)
    print(json.dumps({"comparison_clipping": 0, "gallery_integer_zooms": [1, 2, 3], "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
