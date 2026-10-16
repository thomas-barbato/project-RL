"""Generate review scenes and a numbered local gallery using packaged source art."""
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
import json,hashlib,shutil
ROOT=Path(__file__).resolve().parents[2];OUT=Path(__file__).resolve().parent
SERVER=ROOT/'artifacts/armures-bicolores-completes-2026-10-05'
CAT=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))
F=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',13)
S=ImageFont.truetype('C:/Windows/Fonts/consola.ttf',10)
terrain=Image.open(ROOT/'assets/sprites/cyberpunk/terrain-materials-32.png').convert('RGBA')
floor=terrain.crop((0,0,32,32))
floor.save(OUT/'review/sol-jeu.png')
actors=Image.open(ROOT/'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
player=actors.crop((0,0,32,32))
items=Image.open(ROOT/'assets/sprites/cyberpunk/crawl-items-32.png').convert('RGBA')
loot=items.crop((0,0,32,32))
scenes=[]
def stamp(canvas,im,xy):canvas.paste(im,xy,im)
def scene(entries,label):
    # Each horizontal group uses five logical tiles per entry, at native 1:1.
    width=960;rows=(len(entries)+5)//6;height=90+rows*100
    canvas=Image.new('RGB',(width,height),'#121b22');d=ImageDraw.Draw(canvas)
    d.text((14,12),label,font=F,fill='#b9e7db')
    d.text((14,34),'COMPOSITION DE REVUE / taille native / loot au-dessus du corps',font=S,fill='#a8bdc7')
    for i,e in enumerate(entries):
        x=i%6*160;y=70+i//6*100
        side=e['tile_size']
        for xx in range(0,160,32):
            for yy in range(0,96,32):
                stamp(canvas,floor,(x+xx,y+yy))
        d.text((x+5,y+3),f'{e["number"]:02}',font=S,fill='#e4f1ec')
        a=Image.open(OUT/e['living']);b=Image.open(OUT/e['corpse'])
        stamp(canvas,player,(x+3,y+61))
        if side==32:stamp(canvas,a,(x+36,y+61));stamp(canvas,b,(x+70,y+61));stamp(canvas,b,(x+106,y+61));stamp(canvas,loot,(x+106,y+61))
        else:stamp(canvas,a,(x+34,y+28));stamp(canvas,b,(x+96,y+28));stamp(canvas,loot,(x+111,y+57))
    file=f'review/{label}.png';canvas.save(OUT/file);scenes.append(file)
for start,end in [(1,16),(17,28),(29,32),(33,44),(45,55)]:
    entries=[e for e in CAT if start<=e['number']<=end]
    if entries:scene(entries,f'scene-{start:02}-{end:02}')
html=(OUT/'gallery.html').read_text(encoding='utf-8').replace('__DATA__',json.dumps(CAT,ensure_ascii=False)).replace('__SCENES__',json.dumps(scenes)).replace('__FLOOR__','review/sol-jeu.png')
(OUT/'galerie-bestiaire.html').write_text(html,encoding='utf-8')
dest=SERVER/'bestiaire-propose';dest.mkdir(exist_ok=True)
for folder in ('tiles','review'):
    shutil.copytree(OUT/folder,dest/folder,dirs_exist_ok=True)
shutil.copyfile(OUT/'manifest.json',dest/'manifest.json')
# The served gallery sits at root: qualify references, CSS floor and scene paths.
served=html.replace('"tiles/','"bestiaire-propose/tiles/').replace('"review/','"bestiaire-propose/review/').replace("url('review/","url('bestiaire-propose/review/")
(SERVER/'galerie-bestiaire.html').write_text(served,encoding='utf-8')
baseline=json.loads((OUT/'baseline.json').read_text(encoding='utf-8'))
changed=[p for p,d in baseline.items() if hashlib.sha256((ROOT/p).read_bytes()).hexdigest()!=d]
report=dict(references=len(CAT),new_enemies=sum(e['number']<=28 for e in CAT),bosses=sum(29<=e['number']<=32 for e in CAT),existing_corpses=sum(e['number']>=33 for e in CAT),tiles=len(CAT)*2,active_files_checked=len(baseline),active_files_changed=changed,native_scene_compositions=scenes,preview_only=True,engine_tests='not run; production code unchanged',review_status='awaiting user opinion')
assert not changed,changed
(OUT/'validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps(report,ensure_ascii=False))
