from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
import json,base64,math,hashlib

ROOT=Path(__file__).resolve().parents[2];OUT=Path(__file__).resolve().parent
manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))
font=ImageFont.truetype(r'C:\Windows\Fonts\consola.ttf',15)
small=ImageFont.truetype(r'C:\Windows\Fonts\consola.ttf',12)
poses=['Sans arme','Épée','Fusil','Pistolet','Épée à deux mains','Deux épées','Deux pistolets','Mixte']
def frames(name,count):
    raw=(OUT/name).read_bytes();size=48*48*4
    assert len(raw)==count*8*size
    return [Image.frombytes('RGBA',(48,48),raw[n*size:(n+1)*size]) for n in range(count*8)]
figures=frames('figures.rgba',17);npcs=frames('npc-figures.rgba',6)
atlas=Image.new('RGBA',(8*48,17*48))
for i,im in enumerate(figures):atlas.alpha_composite(im,((i%8)*48,(i//8)*48))
atlas.save(OUT/'figures-48.png')
npc_atlas=Image.new('RGBA',(8*48,6*48))
for i,im in enumerate(npcs):npc_atlas.alpha_composite(im,((i%8)*48,(i//8)*48))
npc_atlas.save(OUT/'npc-figures-48.png')
for start,end in [(0,6),(6,12),(12,17)]:
    board=Image.new('RGB',(1110,80+(end-start)*128),'#141c21');d=ImageDraw.Draw(board)
    d.text((12,12),f'{start+1:02}–{end:02} / UN CORPS, ARMES ET PRISES EN COUCHES SÉPARÉES',font=font,fill='#eff4ef')
    d.text((12,36),'Essai de composition. Pixels agrandis ×2 ; corps natif de 30 px dans une tile de 32.',font=small,fill='#abc0c8')
    for pose,label in enumerate(poses):d.text((210+pose*110,61),label.replace('Épée à deux mains','Épée 2 mains'),font=small,fill='#80dacf')
    for i in range(start,end):
        y=80+(i-start)*128
        if i%2==0:d.rectangle((5,y,1104,y+125),fill='#202a30')
        d.text((12,y+26),f'{i+1:02}',font=font,fill='#80dacf')
        d.text((12,y+48),manifest['armor'][i]['name'][:24],font=small,fill='#edf1ef')
        for p in range(8):
            im=figures[i*8+p].resize((96,96),Image.Resampling.NEAREST)
            board.paste(im,(204+p*110,y+21),im)
    board.save(OUT/f'poses-{start+1:02}-{end:02}.png')

def data(path):
    return 'data:image/png;base64,'+base64.b64encode((OUT/path).read_bytes()).decode()
for group in ('armor','weapons','monsters','grips'):
    for entry in manifest[group]:entry['image']=data(entry['path'])
manifest['figure_atlas']=data('figures-48.png')
manifest['npc_atlas']=data('npc-figures-48.png')
manifest['boards']={name:data(name+'.png') for name in ['armures-numerote','armes-numerote','bestiaire-numerote',
                       'reference-crawl','comparaison-32','poses-01-06','poses-07-12','poses-13-17']}
manifest['scene_captures']=[{'name':p.stem,'image':data(p.name)} for p in sorted(OUT.glob('sols-*.png'))]
if (OUT/'scene-native.png').exists():manifest['scene_captures'].append({'name':'Scene native','image':data('scene-native.png')})
html=(OUT/'review-template.html').read_text(encoding='utf-8').replace('__DATA__',json.dumps(manifest,ensure_ascii=False))
(OUT/'galerie.html').write_text(html,encoding='utf-8')
# New sibling page in the running review server; older approved/rejected galleries stay intact.
(ROOT/'artifacts/armures-bicolores-completes-2026-10-05/galerie-crawl.html').write_text(html,encoding='utf-8')
baseline=json.loads((OUT/'baseline.json').read_text(encoding='utf-8'))
changed=[name for name,digest in baseline.items() if hashlib.sha256((ROOT/name).read_bytes()).hexdigest()!=digest]
assert not changed,changed
report={'active_files_checked':len(baseline),'active_files_changed':changed,'tiles':57,'composition_frames':184,
        'native_tile_size':32,'palette_limits':{'armor':24,'weapons':16,'monsters':24},'preview_only':True}
(OUT/'preservation.json').write_text(json.dumps(report,indent=2),encoding='utf-8')
print(json.dumps(report))
