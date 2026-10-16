"""Layout generated colorways with names and native-size samples; no pixel recoloring."""
from pathlib import Path
import hashlib
import importlib.util
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
FONT="C:/Windows/Fonts/consola.ttf"
INK,PANEL=(11,21,21,255),(17,28,29,255)
TEXT,DIM=(181,235,219,255),(129,161,157,255)
COLORS=("Rouge","Bleu","Violet","Turquoise","Jaune","Rose")
MID=("#d7475b","#346bdd","#9951db","#13b6b6","#e1c430","#dd439c")


def load_helper(name,path):
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


helpers=load_helper("ranged_preview",ROOT/"artifacts/tir-brutal-propositions-2026-10-05/build_previews.py")
framing=load_helper("body_framing",ROOT/"tools/art/build_player_outfits.py")


def text(board,where,value,size=17,color=TEXT):
    ImageDraw.Draw(board).text(where,value,font=ImageFont.truetype(FONT,size),fill=color)


def base(title,rows,height):
    board=Image.new("RGBA",(1440,130+rows*height),INK)
    text(board,(20,16),title,25)
    text(board,(20,50),"Un même modèle par ligne · seule la palette varie · propositions avant intégration",17,DIM)
    for col,name in enumerate(COLORS):
        x=170+col*210
        ImageDraw.Draw(board).rectangle((x+12,89,x+25,102),fill=MID[col])
        text(board,(x+35,86),name,18)
    return board


def weapons():
    tiles=helpers.extract(Image.open(OUT/"armes-palettes-source-v1.png").convert("RGBA"),6,4,24)
    board=base("VARIANTES DE COULEURS / ARMES",4,218)
    names=("Cogneur","Brise-ligne","Enfonceur","Fournaise")
    kinds=("Pistolet","Fusil d'assaut","Fusil à pompe","Lance-flammes")
    for row,name in enumerate(names):
        y=118+row*218
        text(board,(16,y+67),name,19)
        text(board,(16,y+98),kinds[row],15,DIM)
        for col in range(6):
            x=170+col*210
            tile=tiles[row*6+col]
            ImageDraw.Draw(board).rectangle((x,y,x+201,y+207),fill=PANEL)
            board.alpha_composite(helpers.fit(tile,162),(x+20,y+7))
            native=helpers.fit(tile,32,2)
            board.alpha_composite(native,(x+85,y+172))
            native.save(OUT/f"arme-{row+1}-palette-{col+1}-32.png")
    board.save(OUT/"palettes-armes.png")


def armor():
    source=Image.open(OUT/"armures-palettes-source-v1.png").convert("RGBA")
    tiles=helpers.extract(source,6,3,18)
    frames=framing.frames("armures-palettes-source-v1.png",6,3,18,OUT)
    board=base("VARIANTES DE COULEURS / ARMURES",3,252)
    names=("Armure\nCerbère","Robe de la\nSeconde Ombre","Tenue du\ntechnicien")
    checks=[]
    for row,name in enumerate(names):
        y=118+row*252
        text(board,(16,y+78),name,19)
        for col in range(6):
            index=row*6+col
            x=170+col*210
            ImageDraw.Draw(board).rectangle((x,y,x+201,y+241),fill=PANEL)
            board.alpha_composite(helpers.fit(tiles[index],166),(x+18,y+5))
            native=frames[index].resize((48,48),Image.Resampling.NEAREST)
            board.alpha_composite(native,(x+77,y+178))
            native.save(OUT/f"armure-{row+1}-palette-{col+1}-48.png")
            pixels=native.load()
            ys=[yy for yy in range(48) if any(pixels[xx,yy][3]>=128 for xx in range(22,27))]
            checks.append({"model":row+1,"palette":col+1,"body_height":max(ys)-min(ys)+1})
        text(board,(16,y+170),"Corps : 30 px",14,DIM)
    board.save(OUT/"palettes-armures.png")
    (OUT/"controle-taille.json").write_text(json.dumps(checks,indent=2),encoding="utf-8")
    print("Body heights:",sorted(set(v["body_height"] for v in checks)))


def preservation():
    before=json.loads((OUT/"runtime-avant.json").read_text(encoding="utf-8"))
    changed=[name for name,digest in before.items() if hashlib.sha256((ROOT/name).read_bytes()).hexdigest()!=digest]
    assert not changed,changed
    report={"checked_active_files":len(before),"changed_active_files":changed,"review_only":True}
    (OUT/"preservation.json").write_text(json.dumps(report,indent=2),encoding="utf-8")
    print(f"{len(before)} active files preserved; 24 weapon and 18 armor colorway proposals")


if __name__=="__main__":
    weapons()
    armor()
    preservation()
