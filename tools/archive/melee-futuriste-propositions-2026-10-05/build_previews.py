"""Present imagegen proposals: crop, alpha-clean, resize and arrange; no weapon drawing."""
from pathlib import Path
import hashlib
import json

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INK = (11, 21, 21, 255)
PANEL = (17, 28, 29, 255)
TEXT = (181, 235, 219, 255)
DIM = (129, 161, 157, 255)
FONT = "C:/Windows/Fonts/consola.ttf"


def label(board, position, text, size=16, color=TEXT):
    ImageDraw.Draw(board).text(position, text, font=ImageFont.truetype(FONT, size), fill=color)


def crop_cell(source, index, cols, rows, cutoff=128):
    x, y = index % cols, index // cols
    tile = source.crop((round(x * source.width / cols), round(y * source.height / rows),
                        round((x + 1) * source.width / cols), round((y + 1) * source.height / rows)))
    alpha = tile.getchannel("A").point(lambda a: 255 if a >= cutoff else 0)
    tile.putalpha(alpha)
    bounds = alpha.getbbox()
    if not bounds:
        raise ValueError(f"Empty image cell {index}")
    return tile.crop(bounds)


def fit(tile, side, margin=4, feet=False):
    tile = tile.copy()
    tile.thumbnail((side - 2 * margin, side - 2 * margin), Image.Resampling.NEAREST)
    frame = Image.new("RGBA", (side, side))
    y = side - margin - tile.height if feet else (side - tile.height) // 2
    frame.alpha_composite(tile, ((side - tile.width) // 2, y))
    return frame


def weapon_board():
    old = Image.open(OUT / "armes-actuelles-reference.png").convert("RGBA")
    new = Image.open(OUT / "six-armes-futuristes-source.png").convert("RGBA")
    names = ["Lame d’induction", "Couteau à impulsion", "Hache de brèche",
             "Marteau à percussion", "Lance conductrice", "Matraque à décharge"]
    board = Image.new("RGBA", (1080, 714), INK)
    label(board, (20, 16), "ARMES DE MÊLÉE / PROPOSITIONS VISUELLES", 24)
    label(board, (20, 49), "Actuel → proposition · noms de travail · aucune nouvelle mécanique", 17, DIM)
    for index, name in enumerate(names):
        x, y = index % 3 * 360, 88 + index // 3 * 304
        ImageDraw.Draw(board).rectangle((x + 8, y, x + 351, y + 291), fill=PANEL)
        label(board, (x + 20, y + 12), f"{index + 1}. {name}", 19)
        label(board, (x + 26, y + 44), "ACTUEL", 14, DIM)
        label(board, (x + 188, y + 44), "PROPOSITION", 14, DIM)
        for col, image in enumerate((old, new)):
            tile = crop_cell(image, index, 3, 2, 128 if col == 0 else 220)
            board.alpha_composite(fit(tile, 148), (x + 22 + col * 164, y + 70))
            board.alpha_composite(fit(tile, 32, margin=2), (x + 78 + col * 164, y + 235))
        label(board, (x + 124, y + 268), "32 pixels", 13, DIM)
    board.save(OUT / "comparatif-six-armes.png")


def ground_board():
    old = Image.open(ROOT / "artifacts/sprites-cyberpunk-crawl-review-2026-10-04/ground-icons-jupiter/melee-32.png").convert("RGBA")
    new_source = Image.open(OUT / "icone-melee-futuriste-v2-source.png").convert("RGBA")
    new = fit(crop_cell(new_source, 0, 1, 1, 128), 32, margin=2)
    new.save(OUT / "icone-melee-futuriste-32.png")
    floor_source = Image.open(ROOT / "assets/sprites/cyberpunk/terrain-materials-32.png").convert("RGBA")
    floor_indices = (0, 4, 8)
    board = Image.new("RGBA", (680, 360), INK)
    label(board, (20, 15), "ICÔNE DES ARMES DE MÊLÉE AU SOL", 23)
    label(board, (20, 46), "Ancienne épée / lame futuriste · agrandissement ×3", 16, DIM)
    for col, icon in enumerate((old, new)):
        x = 20 + col * 338
        label(board, (x, 82), "ACTUELLE" if col == 0 else "PROPOSÉE", 19)
        for index, floor_index in enumerate(floor_indices):
            tile = floor_source.crop((floor_index * 32, 0, (floor_index + 1) * 32, 32))
            tile.alpha_composite(icon)
            board.alpha_composite(tile.resize((96, 96), Image.Resampling.NEAREST), (x + index * 103, 120))
            board.alpha_composite(tile, (x + index * 103 + 32, 254))
        label(board, (x + 42, 310), "Taille réelle : 32 × 32", 15, DIM)
    board.save(OUT / "comparatif-icone-sol.png")


def player_board():
    old = Image.open(OUT / "personnages-actuels-reference.png").convert("RGBA")
    corrected = OUT / "personnages-epee-futuriste-v2-source.png"
    new = Image.open(corrected if corrected.exists() else OUT / "personnages-epee-futuriste-source.png").convert("RGBA")
    board = Image.new("RGBA", (1120, 368), INK)
    label(board, (20, 16), "ÉPÉE PORTÉE / AVANT ET PROPOSITION", 24)
    label(board, (20, 48), "Aperçu sur quatre tenues · pose conservée · détails à 32 pixels", 17, DIM)
    names = ["Armure Cerbère", "Armure Azurite", "Tenue Fantôme", "Tenue du technicien"]
    for index, name in enumerate(names):
        x = index * 280
        ImageDraw.Draw(board).rectangle((x + 6, 85, x + 273, 345), fill=PANEL)
        label(board, (x + 13, 95), name, 18)
        label(board, (x + 17, 124), "ACTUEL", 13, DIM)
        label(board, (x + 145, 124), "PROPOSITION", 13, DIM)
        for col, image in enumerate((old, new)):
            tile = fit(crop_cell(image, index, 2, 2, 128), 128, feet=True)
            board.alpha_composite(tile, (x + 10 + col * 132, 150))
            board.alpha_composite(tile.resize((32, 32), Image.Resampling.NEAREST), (x + 58 + col * 132, 293))
    board.save(OUT / "comparatif-epee-portee.png")


def preservation():
    before = json.loads((OUT / "runtime-avant.json").read_text())
    changed = [relative for relative, expected in before.items()
               if hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() != expected]
    result = {"status": "preview_only_pending_user_validation", "runtime_files_checked": len(before),
              "runtime_files_changed": changed, "game_integration": False}
    (OUT / "preservation.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    if changed:
        raise ValueError(f"Runtime files changed during preview preparation: {changed}")
    print(f"Preview-only: {len(before)} runtime/content files unchanged.")


def guard_board():
    source = OUT / "personnages-epee-futuriste-v2-source.png"
    if not source.exists():
        return
    image = Image.open(source).convert("RGBA")
    board = Image.new("RGBA", (1040, 570), INK)
    label(board, (20, 16), "CORRECTION DE L’ÉPÉE PORTÉE", 24)
    label(board, (20, 49), "Lame centrée sur la garde · détail agrandi · aperçu avant intégration", 17, DIM)
    names = ["Cerbère", "Azurite", "Fantôme", "Technicien"]
    for index, name in enumerate(names):
        x = index * 260
        ImageDraw.Draw(board).rectangle((x + 8, 87, x + 251, 550), fill=PANEL)
        label(board, (x + 20, 96), name, 19)
        tile = fit(crop_cell(image, index, 2, 2, 128), 128, feet=True)
        board.alpha_composite(tile.resize((160, 160), Image.Resampling.NEAREST), (x + 50, 125))
        board.alpha_composite(tile.resize((32, 32), Image.Resampling.NEAREST), (x + 114, 295))
        label(board, (x + 80, 332), "32 pixels", 15, DIM)
        closeup = tile.crop((13, 32, 58, 91)).resize((135, 177), Image.Resampling.NEAREST)
        board.alpha_composite(closeup, (x + 63, 365))
    board.save(OUT / "garde-corrigee.png")


if __name__ == "__main__":
    weapon_board()
    ground_board()
    if (OUT / "personnages-epee-futuriste-source.png").exists():
        player_board()
    guard_board()
    preservation()
