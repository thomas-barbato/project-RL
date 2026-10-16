"""ANIM-S10 revision: brighter violet in the native renderer, same raster drawings."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_occult_impacts as previous

BASE=previous.OUT
OUT=previous.base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-30'
VERSION='A02'
PREVIOUS_VERSION='A01'
RENDER_OPTION='--occulte-violet-clair'
PALETTE_NAME='Violet éclairci'
old_protected=previous.protected
old_write_html=previous.write_html


def protected():
    hashes=old_protected()
    for p in sorted(BASE.rglob('*')):
        if p.is_file():hashes[p.relative_to(previous.base.ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists()
    previous.base.dump(OUT/'review-protection.json',protected())
    names=['bolt-source.png','bolt-atlas.png','burst-source.png','burst-atlas.png','imagegen-bolt-prompt.txt','imagegen-burst-prompt.txt','drawing-pack.json']
    for name in names:(OUT/name).write_bytes((BASE/name).read_bytes())
    previous.base.dump(OUT/'review.json',{'batch':'ANIM-S10','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'id':sid,'variant':sid+'-'+VERSION,'name':name,'status':'awaiting_visual_review'}
            for n,sid,name in [(21,'A01','Trait arcanique'),(22,'A02','Déflagration occulte')]],
        'feedback':'The user likes the drawings but finds the dark violet hard to see on the game board.',
        'revision':'Only the native effect shader lifts colored violet highlights toward pale lavender. Dark cores and crimson seal stay dark; textures and alpha are unchanged.',
        'retained':'Exact previous drawings, sizes, placement, timings, walls, prepared contacts and gameplay rules.',
        'scope':'Isolated review; no campaign integration or general palette approval. Previous review remains intact.'})
    launch=(BASE/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    for option in ('--occulte-violet-clair','--occulte-violet-lavande','--occulte-violet-franc'):
        launch=launch.replace(f" '{option}'",'')
    launch=launch.replace("'--lot-occulte-impacts'",f"'--lot-occulte-impacts' '{RENDER_OPTION}'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'new_drawings':0,'textures_reused':4}))


def write_html(entries):
    old_write_html(entries)
    path=OUT/'index.html';html=path.read_text(encoding='utf-8')
    html=html.replace('<h1>Magie occulte · Trait et déflagration</h1>','<h1>Magie occulte · Violet plus lisible</h1>')
    html=html.replace('ANIM-S10 · deux rendus à examiner en cases natives 64 px',f'A01-{VERSION} / A02-{VERSION} · mêmes dessins · {PALETTE_NAME} · cases natives 64 px')
    html=html.replace('<select id="scenario">',f'<select id="palette"><option value="clair">{PALETTE_NAME}</option><option value="sombre">Violet précédent</option></select><select id="scenario">')
    html=html.replace('im.src=`frames/${v.variant}/${String(i).padStart(3,\'0\')}.png`;',
        ('const dark=document.getElementById(\'palette\').value===\'sombre\';const variant=dark?v.variant.replace(\'-CURRENT-\',\'-PREVIOUS-\'):v.variant;im.src=`${dark?\'../BASE/\':\'\'}frames/${variant}/${String(i).padStart(3,\'0\')}.png`;').replace('CURRENT',VERSION).replace('PREVIOUS',PREVIOUS_VERSION).replace('BASE',BASE.name))
    html=html.replace('scenario.onchange=()=>{','document.getElementById(\'palette\').onchange=()=>{launched=true;turn=0;elapsed=0;load();};scenario.onchange=()=>{')
    html=html.replace('<p>Le Trait atteint sa cible','<p><a href="violet-comparaison-64.gif">Comparer les deux palettes côte à côte</a></p><p>Le Trait atteint sa cible')
    html=html.replace('Dessins créés avec imagegen intégré :','Dessins existants conservés, issus d’imagegen intégré. Seule la palette violette est ajustée au rendu :')
    path.write_text(html,encoding='utf-8')
    script=html.split('<script>',1)[1].split('</script>',1)[0]
    (OUT/'gallery-script.js').write_text(script,encoding='utf-8')


def package(comparisons_only=False):
    previous.OUT=OUT
    previous.protected=protected
    previous.write_html=write_html
    if comparisons_only:
        assert protected()==json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
        assert len(json.loads((OUT/'verification.json').read_text(encoding='utf-8'))['controls'])==14
    else:
        previous.package()
    checks=[]
    for e in json.loads((OUT/'native/capture.json').read_text(encoding='utf-8')):
        old_timeline=json.loads((BASE/'native'/e['folder']/'timeline.json').read_text(encoding='utf-8'))
        timeline=json.loads((OUT/'native'/e['folder']/'timeline.json').read_text(encoding='utf-8'))
        assert old_timeline==timeline,'Animation or turn state changed.'
        for v in e['views']:
            for phase in e['control_frames']:
                with Image.open(OUT/'native'/e['folder']/f'control-{phase:03}.png') as current,Image.open(BASE/'native'/e['folder']/f'control-{phase:03}.png') as old:
                    x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h)
                    diff=ImageChops.difference(current.crop(bounds).convert('RGB'),old.crop(bounds).convert('RGB'))
                    # Changing version digits warms the font atlas differently.
                    # Ignore only the brief skill-name text rectangle; actor bodies,
                    # terrain, HUD and all frames after its disappearance stay exact.
                    if v['valid'] and 17<=phase<34:
                        px,py=v['player_cell'];px=(px+.5)*64;py=(py+.5)*64
                        baseline=py-43-(phase-17)/24*7
                        diff.paste((0,0,0),(int(px-59),int(baseline-19),int(px+160),int(baseline+4)))
                    assert diff.getbbox() is None,'Terrain or actors changed outside the floating skill name.'
            if v['valid']:
                changed=False
                for phase in (19,21,23,26,29):
                    old_path=BASE/'frames'/v['variant'].replace('-'+VERSION+'-','-'+PREVIOUS_VERSION+'-')/f'{phase:03}.png'
                    new_path=OUT/'frames'/v['variant']/f'{phase:03}.png'
                    with Image.open(old_path) as old,Image.open(new_path) as current:
                        changed|=ImageChops.difference(old.convert('RGB'),current.convert('RGB')).getbbox() is not None
                assert changed,'No visible color correction.'
            checks.append({'variant':v['variant'],'timeline_identical':True,'actors_and_terrain_identical':True,'source_textures_and_alpha_identical':True})
    for name in ('bolt-source.png','bolt-atlas.png','burst-source.png','burst-atlas.png'):
        assert (OUT/name).read_bytes()==(BASE/name).read_bytes()
    previous.base.dump(OUT/'contrast-verification.json',{'controls':checks,'change':'Effect shader color only; alpha, timings, positions and source drawings unchanged.',
        'comparison_limit':'Tiny antialias differences in the transient skill-name text are excluded only within its known rectangle during frames 17-33. Actor bodies, terrain and all later frames match exactly.'})
    boards=[]
    for i in range(112):
        board=Image.new('RGB',(1232,1238),'#0b1218');draw=ImageDraw.Draw(board)
        for row,(sid,title) in enumerate([('A01','21 · Trait arcanique'),('A02','22 · Déflagration occulte')]):
            for col,(root,variant,label) in enumerate([(BASE,sid+'-'+PREVIOUS_VERSION+'-HAUT','Violet précédent'),(OUT,sid+'-'+VERSION+'-HAUT',PALETTE_NAME)]):
                x=12+col*616;y=12+row*616
                draw.text((x,y),f'{title} · {label}',font=previous.base.font(19,True),fill='#e0e7e7')
                with Image.open(root/'frames'/variant/f'{i:03}.png') as image:board.paste(image,(x,y+30))
        boards.append(board)
    previous.save_animation(OUT/'violet-comparaison-64.gif',boards,24)
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'comparison':'native 64 px','drawings_unchanged':True,'timings_unchanged':True,'status':'awaiting_visual_review'}))


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--comparisons-only',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package(args.comparisons_only)
