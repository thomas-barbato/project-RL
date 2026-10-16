"""Layout isolated raster proposals; crop/resize only, no weapon or character drawing."""
from pathlib import Path
import hashlib
import importlib.util
import json

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
HELPERS = ROOT / "artifacts/melee-futuriste-propositions-2026-10-05/build_previews.py"
spec = importlib.util.spec_from_file_location("previous_preview_helpers", HELPERS)
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)
crop_cell, fit, label = helpers.crop_cell, helpers.fit, helpers.label
INK, PANEL, DIM = helpers.INK, helpers.PANEL, helpers.DIM

NAMES = ["Lame de rupture", "Couperet de récupération", "Lame de brèche",
         "Couteau d’assaut", "Hache de chantier", "Marteau pilonneur",
         "Lance de ferraille", "Matraque blindée", "Pic de démolition"]
STYLES = ["Industriel", "Récupération", "Militaire", "Militaire", "Récupération",
          "Industriel", "Récupération", "Militaire", "Industriel"]


def weapons():
    source = Image.open(OUT / "neuf-armes-brutales-source.png").convert("RGBA")
    board = Image.new("RGBA", (1080, 1032), INK)
    label(board, (20, 16), "ARMES FUTURISTES / MÉTAL, DENTS ET MÉCANISMES", 24)
    label(board, (20, 49), "Trois lames au choix (1–3), puis six autres designs · noms de travail", 17, DIM)
    for index, name in enumerate(NAMES):
        x, y = index % 3 * 360, 88 + index // 3 * 308
        ImageDraw.Draw(board).rectangle((x + 8, y, x + 351, y + 295), fill=PANEL)
        label(board, (x + 18, y + 12), f"{index + 1}. {name}", 18)
        label(board, (x + 18, y + 39), STYLES[index], 15, DIM)
        corrected_baton = OUT / "matraque-sans-garde-source.png"
        tile = (crop_cell(Image.open(corrected_baton).convert("RGBA"), 0, 1, 1, 200)
                if index == 7 and corrected_baton.exists()
                else crop_cell(source, index, 3, 3, 200))
        board.alpha_composite(fit(tile, 192), (x + 84, y + 68))
        board.alpha_composite(fit(tile, 32, margin=2), (x + 164, y + 263))
    board.save(OUT / "neuf-armes-brutales.png")
    refs = Image.new("RGBA", (960, 320))
    for index in range(3):
        tile = fit(crop_cell(source, index, 3, 3, 200), 304)
        refs.alpha_composite(tile, (index * 320 + 8, 8))
    refs.save(OUT / "trois-lames-reference.png")


def held():
    path = OUT / "personnages-lames-brutales-source.png"
    if not path.exists():
        return
    source = Image.open(path).convert("RGBA")
    board = Image.new("RGBA", (1044, 756), INK)
    label(board, (20, 16), "TROIS LAMES PORTÉES / COMPARAISON", 24)
    label(board, (20, 49), "Mêmes tenues et même pose · contrôle des mains et de la garde · aperçu", 17, DIM)
    for col in range(3):
        x = col * 348
        ImageDraw.Draw(board).rectangle((x + 8, 86, x + 339, 738), fill=PANEL)
        label(board, (x + 20, 96), f"{col + 1}. {NAMES[col]}", 18)
        label(board, (x + 20, 123), STYLES[col], 15, DIM)
        for row in range(2):
            index, y = row * 3 + col, 156 + row * 291
            label(board, (x + 20, y), "Cerbère" if row == 0 else "Fantôme", 16, DIM)
            tile = fit(crop_cell(source, index, 3, 2, 128), 128, feet=True)
            board.alpha_composite(tile.resize((176, 176), Image.Resampling.NEAREST), (x + 86, y + 29))
            board.alpha_composite(tile.resize((32, 32), Image.Resampling.NEAREST), (x + 158, y + 228))
    board.save(OUT / "trois-lames-portees.png")


def ground():
    source = Image.open(OUT / "neuf-armes-brutales-source.png").convert("RGBA")
    floors = Image.open(ROOT / "assets/sprites/cyberpunk/terrain-materials-32.png").convert("RGBA")
    board = Image.new("RGBA", (1044, 388), INK)
    label(board, (20, 16), "ESSAIS D’ICÔNES AU SOL / TROIS LAMES", 24)
    label(board, (20, 49), "Icônes provisoires dérivées des dessins · ×3 et taille réelle 32 pixels", 17, DIM)
    for col in range(3):
        x = col * 348
        label(board, (x + 18, 96), f"{col + 1}. {STYLES[col]}", 19)
        icon = fit(crop_cell(source, col, 3, 3, 200), 32, margin=2)
        icon.save(OUT / f"icone-lame-{col + 1}-32.png")
        for index, floor_index in enumerate((0, 4, 8)):
            tile = floors.crop((floor_index * 32, 0, (floor_index + 1) * 32, 32))
            tile.alpha_composite(icon)
            board.alpha_composite(tile.resize((96, 96), Image.Resampling.NEAREST), (x + 15 + index * 105, 137))
            board.alpha_composite(tile, (x + 47 + index * 105, 271))
        label(board, (x + 101, 330), "32 pixels", 16, DIM)
    board.save(OUT / "essais-icones-sol.png")


def preservation():
    before = json.loads((OUT / "runtime-avant.json").read_text())
    changed = [path for path, digest in before.items()
               if hashlib.sha256((ROOT / path).read_bytes()).hexdigest() != digest]
    result = {"status": "preview_only_pending_user_validation", "runtime_files_checked": len(before),
              "runtime_files_changed": changed, "game_integration": False}
    (OUT / "preservation.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    if changed:
        raise ValueError(f"Active files changed: {changed}")
    print(f"Proposal only; {len(before)} active files unchanged.")


if __name__ == "__main__":
    weapons()
    held()
    ground()
    preservation()
