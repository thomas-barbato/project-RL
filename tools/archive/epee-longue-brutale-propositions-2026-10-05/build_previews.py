"""Review raster proposals: crop, clean alpha, resize, and arrange. No art drawing."""
from pathlib import Path
import hashlib
import importlib.util
import json

from PIL import Image, ImageDraw, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "preview_helpers", ROOT / "artifacts/melee-futuriste-propositions-2026-10-05/build_previews.py")
helpers = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helpers)
fit, label = helpers.fit, helpers.label
INK, PANEL, DIM = helpers.INK, helpers.PANEL, helpers.DIM
NAMES = ("BRÈCHE", "Armure Cerbère", "VIGIE", "Tenue Fantôme")
BODY_HEIGHT = 120
FRAME = 256


def body_anchors(tile):
    """Find boots and head in a central strip; an outboard blade never sets scale."""
    alpha = tile.getchannel("A")
    bottom = alpha.getbbox()[3]-1
    sole = alpha.crop((0, max(0, bottom-5), tile.width, bottom+1)).getbbox()
    center = (sole[0]+sole[2]-1)/2
    half_band = max(2, round(tile.width*0.025))
    left, right = round(center)-half_band, round(center)+half_band+1
    top = alpha.crop((left, 0, right, bottom+1)).getbbox()[1]
    return center, top, bottom


def normalize(tile):
    center, top, bottom = body_anchors(tile)
    scale = BODY_HEIGHT/(bottom-top+1)
    size = (round(tile.width*scale), round(tile.height*scale))
    resized = tile.resize(size, Image.Resampling.NEAREST)
    x = round(FRAME/2-center*scale)
    y = FRAME-4-size[1]
    if x < 0 or y < 0 or x+size[0] > FRAME:
        raise ValueError("Weapon exceeds the common body-anchored preview frame")
    frame = Image.new("RGBA", (FRAME, FRAME))
    frame.alpha_composite(resized, (x, y))
    return frame


def crop(image, index=0, cols=1, rows=1):
    tile = helpers.crop_cell(image, index, cols, rows)
    alpha = tile.getchannel("A")
    grouped = alpha.filter(ImageFilter.MaxFilter(9))
    points = {(x, y) for y in range(tile.height) for x in range(tile.width)
              if grouped.getpixel((x, y))}
    largest = set()
    while points:
        pending = [points.pop()]
        component = set(pending)
        while pending:
            x, y = pending.pop()
            for point in ((x-1, y), (x+1, y), (x, y-1), (x, y+1)):
                if point in points:
                    points.remove(point)
                    component.add(point)
                    pending.append(point)
        if len(component) > len(largest):
            largest = component
    kept = Image.new("L", tile.size)
    for point in largest:
        if alpha.getpixel(point):
            kept.putpixel(point, 255)
    tile.putalpha(kept)
    return tile.crop(kept.getbbox())


def references():
    previous = ROOT / "artifacts/joueur-armes-integrees-2026-10-05"
    classes = Image.open(previous / "classes-armees.png").convert("RGBA")
    armor = Image.open(previous / "tenues-side.png").convert("RGBA")
    figures = (crop(classes, 0, 3, 3), crop(armor, 0, 4, 4),
               crop(classes, 1, 3, 3), crop(armor, 4, 4, 4))
    board = Image.new("RGBA", (960, 960))
    for index, tile in enumerate(figures):
        board.alpha_composite(fit(tile, 440, margin=16, feet=True),
                              (20 + index % 2 * 480, 20 + index // 2 * 480))
    board.save(OUT / "personnages-reference.png")
    brutal = Image.open(ROOT / "artifacts/melee-brutale-propositions-2026-10-05/neuf-armes-brutales-source.png").convert("RGBA")
    crop(brutal, 0, 3, 3).save(OUT / "style-metal-reference.png")


def sword():
    path = OUT / "epee-longue-source.png"
    if not path.exists():
        return
    tile = crop(Image.open(path).convert("RGBA"))
    tile.save(OUT / "epee-longue-reference.png")
    board = Image.new("RGBA", (620, 660), INK)
    label(board, (22, 18), "ÉPÉE LONGUE INDUSTRIELLE", 25)
    label(board, (22, 54), "Proposition · acier abîmé, lame droite et longue", 16, DIM)
    board.alpha_composite(fit(tile, 470, margin=10), (75, 90))
    board.alpha_composite(fit(tile, 32, margin=2), (295, 578))
    label(board, (210, 622), "Aperçu 32 × 32 pixels", 16, DIM)
    board.save(OUT / "epee-longue.png")


def players():
    path = OUT / "personnages-epee-longue-source.png"
    if not path.exists():
        return
    before = Image.open(OUT / "personnages-reference.png").convert("RGBA")
    source = Image.open(path).convert("RGBA")
    board = Image.new("RGBA", (1120, 448), INK)
    label(board, (20, 16), "ÉPÉE LONGUE PORTÉE / APERÇU", 24)
    label(board, (20, 49), "Corps à la même échelle · lame droite, garde réduite · deux prises", 17, DIM)
    for index, name in enumerate(NAMES):
        x = index * 280
        ImageDraw.Draw(board).rectangle((x+6, 86, x+273, 431), fill=PANEL)
        label(board, (x+15, 98), name, 18)
        label(board, (x+15, 127), "ACTUEL", 13, DIM)
        label(board, (x+143, 127), "PROPOSITION", 13, DIM)
        for col, image in enumerate((before, source)):
            full = normalize(crop(image, index, 2, 2))
            tile = full.resize((128, 128), Image.Resampling.NEAREST)
            board.alpha_composite(tile, (x+10+col*132, 152))
            board.alpha_composite(full.resize((64, 64), Image.Resampling.NEAREST),
                                  (x+42+col*132, 288))
            # Full figure enlarged to check that the weapon is continuous and centered.
            if col:
                tile.save(OUT / f"personnage-{index+1}-128.png")
                full.save(OUT / f"personnage-{index+1}-256.png")
                full.resize((64, 64), Image.Resampling.NEAREST).save(OUT / f"personnage-{index+1}-64.png")
        label(board, (x+40, 359), "Réduction · corps 30 pixels", 13, DIM)
        label(board, (x+17, 392), "Prise à droite" if index%2 == 0 else "Prise à gauche", 15, DIM)
    board.save(OUT / "comparatif-epee-longue-portee.png")


def baton():
    previous = ROOT / "artifacts/melee-brutale-propositions-2026-10-05"
    path = previous / "matraque-sans-garde-source.png"
    if not path.exists():
        return
    before = Image.open(ROOT / "artifacts/joueur-hache-brutale-2026-10-05/matraque-reference.png").convert("RGBA")
    after = Image.open(path).convert("RGBA")
    board = Image.new("RGBA", (650, 424), INK)
    label(board, (20, 18), "MATRAQUE / GARDE RETIRÉE", 25)
    for col, image in enumerate((before, after)):
        x = 20 + col*325
        label(board, (x, 72), "AVANT" if col == 0 else "CORRIGÉE", 17, DIM)
        tile = crop(image)
        board.alpha_composite(fit(tile, 250), (x+10, 102))
        board.alpha_composite(fit(tile, 32, margin=2), (x+120, 365))
    board.save(OUT / "matraque-sans-garde.png")


def grips():
    one = OUT / "personnages-epee-longue-source.png"
    two = OUT / "personnages-epee-geante-source.png"
    if not one.exists() or not two.exists():
        return
    images = (Image.open(one).convert("RGBA"), Image.open(two).convert("RGBA"))
    board = Image.new("RGBA", (960, 1044), INK)
    label(board, (20, 16), "ÉPÉES INDUSTRIELLES / DEUX PRISES", 25)
    label(board, (20, 52), "Même hauteur de corps et même ligne des pieds · deux épées validées", 17, DIM)
    measures = []
    for row, name in enumerate(("BRÈCHE", "Armure Cerbère")):
        y = 98 + row*464
        for col, image in enumerate(images):
            x = col*480
            draw = ImageDraw.Draw(board)
            draw.rectangle((x+8, y, x+471, y+450), fill=PANEL)
            label(board, (x+20, y+12), name, 19)
            label(board, (x+248, y+12), "UNE MAIN" if col == 0 else "DEUX MAINS", 19)
            raw = crop(image, row, 2, 2 if col == 0 else 1)
            center, top, bottom = body_anchors(raw)
            tile = normalize(raw)
            for anchor in (252-BODY_HEIGHT, 252):
                line_y = y+42+round(anchor*1.25)
                draw.line((x+35, line_y, x+444, line_y), fill=(52, 71, 70, 255), width=1)
            board.alpha_composite(tile.resize((320, 320), Image.Resampling.NEAREST),
                                  (x+80, y+42))
            native = tile.resize((64, 64), Image.Resampling.NEAREST)
            board.alpha_composite(native, (x+101, y+374))
            label(board, (x+185, y+394), "Réduction : corps 30 pixels", 15, DIM)
            tile.save(OUT / f"taille-commune-{row}-{col}-256.png")
            native.save(OUT / f"taille-commune-{row}-{col}-64.png")
            measured_center, measured_top, measured_bottom = body_anchors(tile)
            measures.append({"outfit": name, "hands": col+1,
                             "source_body_top": top, "source_feet": bottom,
                             "source_body_height": bottom-top+1,
                             "target_body_height": BODY_HEIGHT,
                             "measured_body_height": measured_bottom-measured_top+1,
                             "measured_feet": measured_bottom})
    board.save(OUT / "comparatif-une-deux-mains.png")
    (OUT / "body-scale.json").write_text(json.dumps(measures, indent=2)+"\n", encoding="utf-8")
    same = Image.new("RGBA", (1080, 566), INK)
    label(same, (20, 16), "TAILLE DU PERSONNAGE / CORPS IDENTIQUES EN HAUTEUR", 24)
    label(same, (20, 52), "Épées validées · échelle du corps commune, indépendante de la lame", 17, DIM)
    for col, (row, hands) in enumerate(((0, 0), (1, 0), (0, 1), (1, 1))):
        x = col*270
        draw = ImageDraw.Draw(same)
        draw.rectangle((x+6, 84, x+263, 548), fill=PANEL)
        label(same, (x+16, 97), ("BRÈCHE", "Armure Cerbère")[row], 18)
        label(same, (x+16, 125), "UNE MAIN" if hands == 0 else "DEUX MAINS", 15, DIM)
        tile = Image.open(OUT / f"taille-commune-{row}-{hands}-256.png").convert("RGBA")
        for line_y in (266, 386):
            draw.line((x+15, line_y, x+254, line_y), fill=(52, 71, 70, 255))
        same.alpha_composite(tile, (x+7, 134))
        same.alpha_composite(tile.resize((64, 64), Image.Resampling.NEAREST), (x+103, 423))
        label(same, (x+52, 514), "Corps : 30 pixels", 16, DIM)
    same.save(OUT / "tailles-communes.png")


def preservation():
    before = json.loads((OUT / "runtime-avant.json").read_text())
    changed = [path for path, digest in before.items()
               if hashlib.sha256((ROOT / path).read_bytes()).hexdigest() != digest]
    result = {"status": "preview_only_pending_user_validation", "runtime_files_checked": len(before),
              "runtime_files_changed": changed, "game_integration": False}
    (OUT / "preservation.json").write_text(json.dumps(result, indent=2)+"\n", encoding="utf-8")
    if changed:
        raise ValueError(f"Active files changed: {changed}")
    print(f"Proposal only; {len(before)} active files unchanged.")


if __name__ == "__main__":
    references()
    sword()
    players()
    baton()
    grips()
    preservation()
