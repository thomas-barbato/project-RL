"""Arrange imagegen drawings and native-scale samples; never draw weapon pixels."""
from pathlib import Path
import hashlib
import importlib.util
import json

from PIL import Image, ImageDraw, ImageFont, ImageFilter

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
INK, PANEL = (11, 21, 21, 255), (17, 28, 29, 255)
TEXT, DIM = (181, 235, 219, 255), (129, 161, 157, 255)
FONT = "C:/Windows/Fonts/consola.ttf"
NAMES = ["Cogneur", "Crache-fer", "Enfonceur", "Brise-ligne", "Long-croc",
         "Broyeuse", "Arc de rupture", "Fournaise", "Bélier", "Fracasseur"]
KINDS = ["Pistolet · industriel", "PM · récupération", "Fusil à pompe · industriel",
         "Fusil d'assaut · militaire", "Fusil long · militaire", "Mitrailleuse · récupération",
         "Fusil à énergie · industriel", "Lance-flammes · industriel",
         "Lance-roquettes · récupération", "Lance-grenades · militaire"]


def label(board, position, text, size=17, color=TEXT):
    ImageDraw.Draw(board).text(position, text, font=ImageFont.truetype(FONT, size), fill=color)


def fit(tile, side, margin=4):
    tile = tile.copy()
    tile.thumbnail((side - margin * 2, side - margin * 2), Image.Resampling.NEAREST)
    result = Image.new("RGBA", (side, side))
    result.alpha_composite(tile, ((side-tile.width)//2, (side-tile.height)//2))
    return result


def components(points):
    points = set(points)
    while points:
        pending = [points.pop()]
        found = set(pending)
        while pending:
            x, y = pending.pop()
            for p in ((x-1,y), (x+1,y), (x,y-1), (x,y+1)):
                if p in points:
                    points.remove(p)
                    found.add(p)
                    pending.append(p)
        yield found


def extract(source, cols, rows, count):
    alpha = source.getchannel("A").point(lambda a: 255 if a >= 128 else 0)
    original = alpha.load()
    grouped = alpha.filter(ImageFilter.MaxFilter(3)).load()
    objects = {}
    for component in components((x,y) for y in range(source.height)
                                for x in range(source.width) if grouped[x,y]):
        points = [p for p in component if original[p]]
        if len(points) < 100:
            continue
        cx, cy = sum(x for x,y in points)/len(points), sum(y for x,y in points)/len(points)
        index = min(rows-1, int(cy*rows/source.height))*cols + min(cols-1, int(cx*cols/source.width))
        if index not in objects or len(points) > len(objects[index]):
            objects[index] = points
    if set(objects) != set(range(count)):
        raise ValueError(f"Expected {count} drawings; found grid cells {sorted(objects)}")
    result = []
    for index in range(count):
        points = objects[index]
        bounds = (min(x for x,y in points), min(y for x,y in points),
                  max(x for x,y in points)+1, max(y for x,y in points)+1)
        tile = source.crop(bounds)
        kept = Image.new("L", tile.size)
        for x,y in points:
            kept.putpixel((x-bounds[0], y-bounds[1]), 255)
        tile.putalpha(kept)
        result.append(tile)
    return result


def prepare_targets():
    target = Image.new("RGBA", (1152, 768))
    for row, outfit in enumerate((0, 3)):
        for col, pose in enumerate(("raised", "two-handed", "dual-raised")):
            atlas = Image.open(ROOT / f"assets/sprites/cyberpunk/player-poses-{pose}-128.png").convert("RGBA")
            ox, oy = outfit%16*128, outfit//16*128
            tile = atlas.crop((ox, oy, ox+128, oy+128))
            target.alpha_composite(tile.resize((384,384), Image.Resampling.NEAREST), (col*384, row*384))
    target.save(OUT / "personnages-cibles.png")
    targets = ROOT / "assets/sprites/cyberpunk"
    hashes = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
              for p in targets.iterdir() if p.is_file()}
    for name in ("src/game_art_worn.rs", "src/game_art_catalog.rs", "src/weapon_hands_app.rs",
                 "tools/art/build_player_outfits.py", "tools/art/build_cyberpunk_assets.py"):
        path = ROOT / name
        hashes[name] = hashlib.sha256(path.read_bytes()).hexdigest()
    (OUT / "runtime-avant.json").write_text(json.dumps(hashes, indent=2), encoding="utf-8")


def weapons():
    tiles = extract(Image.open(OUT / "dix-armes-source-v1.png").convert("RGBA"), 4, 3, 10)
    board = Image.new("RGBA", (1280, 1074), INK)
    label(board, (20, 16), "ARMES À DISTANCE / INDUSTRIE, RÉCUPÉRATION, MILITAIRE", 25)
    label(board, (20, 50), "Propositions · noms de travail · chaque case montre aussi un essai de réduction à 32 px", 17, DIM)
    for index, tile in enumerate(tiles):
        x, y = index%4*320, 88 + index//4*326
        ImageDraw.Draw(board).rectangle((x+8,y,x+311,y+315), fill=PANEL)
        label(board, (x+18,y+11), f"{index+1}. {NAMES[index]}", 19)
        label(board, (x+18,y+41), KINDS[index], 15, DIM)
        board.alpha_composite(fit(tile, 220), (x+50,y+69))
        small = fit(tile, 32, 2)
        board.alpha_composite(small, (x+147,y+278))
        small.save(OUT / f"arme-{index+1:02}-32.png")
        fit(tile, 128, 5).save(OUT / f"arme-{index+1:02}-128.png")
    board.save(OUT / "dix-armes-proposees.png")
    reference = Image.new("RGBA", (768, 384))
    for col, index in enumerate((0,3)):
        reference.alpha_composite(fit(tiles[index], 360), (col*384+12,12))
    reference.save(OUT / "pistolet-fusil-reference.png")


def characters():
    spec = importlib.util.spec_from_file_location("outfit_framing", ROOT / "tools/art/build_player_outfits.py")
    framing = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(framing)
    prepared = framing.frames("personnages-tir-source-v1.png", 3, 2, 6, OUT)
    board = Image.new("RGBA", (1080, 758), INK)
    label(board, (20,16), "ARMES PORTÉES / ACTUEL ET PROPOSITION", 25)
    label(board, (20,50), "Corps conservés à 30 px · pistolet, fusil et deux pistolets · essai avant intégration", 16, DIM)
    names = ("Pistolet levé", "Fusil à deux mains", "Deux pistolets")
    checks = []
    for index, frame in enumerate(prepared):
        row, col = divmod(index,3)
        outfit = (0,3)[row]
        pose = ("raised","two-handed","dual-raised")[col]
        x, y = col*360, 88+row*334
        ImageDraw.Draw(board).rectangle((x+8,y,x+351,y+323), fill=PANEL)
        label(board, (x+18,y+12), ("Brèche" if row==0 else "Cerbère")+" · "+names[col], 18)
        label(board, (x+38,y+43), "ACTUEL", 14, DIM)
        label(board, (x+197,y+43), "PROPOSITION", 14, DIM)
        for s, side in enumerate((128,48)):
            atlas = Image.open(ROOT / f"assets/sprites/cyberpunk/player-poses-{pose}-{side}.png").convert("RGBA")
            ox, oy = outfit%16*side, outfit//16*side
            old = atlas.crop((ox,oy,ox+side,oy+side))
            new = frame.resize((side,side), Image.Resampling.NEAREST)
            py = y+75 if s==0 else y+231
            for which,tile in enumerate((old,new)):
                board.alpha_composite(tile, (x+28+which*176+(128-side)//2,py))
            new.save(OUT / f"personnage-{index+1}-{side}.png")
            if side==48:
                pixels=new.load()
                ys=[yy for yy in range(48) if any(pixels[xx,yy][3]>=128 for xx in range(22,27))]
                checks.append({"figure":index+1,"body_height":max(ys)-min(ys)+1})
        label(board, (x+68,y+293), "Taille native du corps : 30 px", 14, DIM)
    board.save(OUT / "comparatif-armes-portees.png")
    (OUT / "controle-taille.json").write_text(json.dumps(checks,indent=2),encoding="utf-8")
    print("Body-height checks:", checks)


if __name__ == "__main__":
    prepare_targets()
    weapons()
    if (OUT / "personnages-tir-source-v1.png").is_file():
        characters()
    print("10 ranged concepts laid out; targets for 6 complete figures; runtime hashes recorded")
