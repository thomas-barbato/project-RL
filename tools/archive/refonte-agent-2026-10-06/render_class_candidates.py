"""Review five starting silhouettes from the accepted native sprite bank.

No active asset is changed. Source pixels are copied intact; the larger view
uses nearest-neighbour scaling. These are unarmed outfit proposals, not the
finished character-creation screen or new gameplay definitions.
"""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
SOURCE = ROOT / "assets/sprites/cyberpunk/crawl-bodies-32.png"
CANDIDATES = [
    (1, "Bastion", 9, "plastron_bastion", "Mêlée et survie"),
    (2, "Opérateur", 2, None, "Drones et affaiblissements"),
    (3, "Traqueur", 1, None, "Distance et esquive"),
    (4, "Arcaniste", 11, "veste_de_la_seconde_ombre", "Magie offensive"),
    (5, "Métamorphe", 5, "carapace_chrysalide", "Magie et transformations"),
]


def font(size):
    return ImageFont.truetype("C:/Windows/Fonts/segoeui.ttf", size)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    atlas = Image.open(SOURCE).convert("RGBA")
    cols = atlas.width // 32
    sheet = Image.new("RGB", (1040, 372), "#0c131b")
    draw = ImageDraw.Draw(sheet)
    draw.text((24, 14), "Cinq classes — propositions de tenues", font=font(23), fill="#edf0ef")
    draw.text((24, 48), "Sprites existants acceptés · sans armes · noms et affectations proposés", font=font(14), fill="#aebac4")
    records = []
    for number, name, index, torso, role in CANDIDATES:
        tile = atlas.crop(((index % cols) * 32, (index // cols) * 32,
                           (index % cols + 1) * 32, (index // cols + 1) * 32))
        path = OUT / f"classe-{number}-native-32.png"
        tile.save(path)
        x = 24 + (number - 1) * 204
        draw.rounded_rectangle((x, 82, x + 192, 337), 6, fill="#17222e", outline="#344352")
        draw.text((x + 12, 91), f"{number}. {name}", font=font(18), fill="#edf0ef")
        draw.text((x + 12, 121), "32 × 32 · taille native", font=font(12), fill="#b9c6d0")
        sheet.paste(tile, (x + 80, 145), tile)
        draw.text((x + 12, 184), "Même dessin · zoom ×4", font=font(12), fill="#b9c6d0")
        zoom = tile.resize((128, 128), Image.Resampling.NEAREST)
        sheet.paste(zoom, (x + 32, 203), zoom)
        records.append({"number": number, "proposed_name": name, "role": role,
                        "native_tile": path.name, "source_atlas_index": index,
                        "existing_torso_model": torso, "pixel_changes": 0,
                        "status": "proposed class assignment; existing approved art",
                        "weapons": "not represented in this silhouette review"})
    draw.text((24, 347), "Aucune intégration aux classes jouables. L'équipement final sera revu avec les armes et les poses.",
              font=font(14), fill="#aebac4")
    sheet.save(OUT / "classes-tenues-review.png")
    manifest = {"source": str(SOURCE.relative_to(ROOT)),
                "source_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
                "native_size": 32, "zoom": 4, "candidates": records}
    (OUT / "class-candidates.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(OUT / "classes-tenues-review.png")


if __name__ == "__main__":
    main()
