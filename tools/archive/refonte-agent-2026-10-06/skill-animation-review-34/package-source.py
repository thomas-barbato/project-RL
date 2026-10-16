"""Review 34: thicker travelling essence only; unchanged source drawings and ray."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_occult_drains as previous

ROOT=previous.ROOT
BASE=previous.OUT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-34'
old_protected=previous.protected
old_write_html=previous.write_html


def protected():
    hashes=old_protected()
    for p in BASE.rglob('*'):
        if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists()
    previous.base.dump(OUT/'review-protection.json',protected())
    for name in ('spirit-source.png','spirit-atlas.png','ray-source.png','ray-atlas.png','drawing-pack.json','imagegen-spirit-prompt.txt','imagegen-ray-prompt.txt'):
        (OUT/name).write_bytes((BASE/name).read_bytes())
    review=json.loads((BASE/'review.json').read_text(encoding='utf-8'))
    review['status']='siphon_revision_awaiting_visual_review_ray_previously_approved'
    spirit=next(c for c in review['candidates'] if c['id']=='HY05')
    spirit['variant']='HY05-A02';spirit['status']='awaiting_visual_review';spirit.pop('next_variant',None)
    review['feedback']='The siphon is too thin and difficult to read; increase its visual thickness.'
    review['revision']='Travelling spirit shards use twice the perpendicular thickness and 1.5 times the length. Shadow, trajectory, opacity, timing, source drawings, prepared state and Rayon vorace unchanged.'
    review['ray_acceptance']='../skill-animation-review-33/ray-acceptance-01.json'
    next(c for c in review['candidates'] if c['id']=='OC05')['acceptance']=review['ray_acceptance']
    previous.base.dump(OUT/'review.json',review)
    launch=(BASE/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-occulte-drains'","'--lot-occulte-drains' '--ponction-epaisse'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'new_drawings':0,'revision':'HY05-A02'}))


def write_html(entries):
    old_write_html(entries)
    path=OUT/'index.html';html=path.read_text(encoding='utf-8')
    html=html.replace('<h1>Magie occulte · Ponction et Rayon vorace</h1>','<h1>Ponction spirituelle · aspiration plus épaisse</h1>')
    html=html.replace('ANIM-S11 · deux propositions animées · cases natives 64 px','23 / HY05-A02 · épaisseur doublée · 24 / OC05-A01 déjà accepté · cases natives 64 px')
    html=html.replace('<select id="scenario">','<select id="comparison"><option value="new">Ponction épaissie</option><option value="old">Version précédente</option></select><select id="scenario">')
    html=html.replace('im.src=`frames/${v.variant}/${String(i).padStart(3,\'0\')}.png`;',
        "const old=document.getElementById('comparison').value==='old';const variant=old?v.variant.replace('HY05-A02-','HY05-A01-'):v.variant;im.src=`${old?'../skill-animation-review-33/':''}frames/${variant}/${String(i).padStart(3,'0')}.png`;")
    html=html.replace('scenario.onchange=()=>{',"document.getElementById('comparison').onchange=()=>{launched=true;turn=0;elapsed=0;load();};scenario.onchange=()=>{")
    html=html.replace('<p>Ponction spirituelle contracte', '<p><a href="ponction-comparaison-64.gif">Comparer les deux épaisseurs côte à côte</a></p><p>Ponction spirituelle contracte')
    html=html.replace('Ces deux nouveaux rendus restent à valider avant intégration au combat.','Seule la nouvelle épaisseur de Ponction reste à valider ; Rayon vorace conserve le rendu accepté.')
    path.write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(html.split('<script>',1)[1].split('</script>',1)[0],encoding='utf-8')


def clear_name(diff,v,frame):
    if v['valid'] and 17<=frame<34:
        px,py=v['player_cell'];px=(px+.5)*64;py=(py+.5)*64
        baseline=py-43-(frame-17)/24*7
        diff.paste((0,0,0),(int(px-59),int(baseline-19),int(px+200),int(baseline+4)))


def package():
    previous.OUT=OUT;previous.protected=protected;previous.write_html=write_html
    previous.package()
    checks=[]
    for e in json.loads((OUT/'native/capture.json').read_text(encoding='utf-8')):
        assert (OUT/'native'/e['folder']/'timeline.json').read_bytes()==(BASE/'native'/e['folder']/'timeline.json').read_bytes()
        for v in e['views']:
            x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h)
            for p in e['control_frames']:
                with Image.open(OUT/'native'/e['folder']/f'control-{p:03}.png') as new,Image.open(BASE/'native'/e['folder']/f'control-{p:03}.png') as old:
                    diff=ImageChops.difference(new.crop(bounds).convert('RGB'),old.crop(bounds).convert('RGB'));clear_name(diff,v,p)
                    assert diff.getbbox() is None,'Actors or terrain changed.'
            if v['id']=='OC05':
                for p in range(144):
                    with Image.open(OUT/'frames'/v['variant']/f'{p:03}.png') as new,Image.open(BASE/'frames'/v['variant']/f'{p:03}.png') as old:
                        diff=ImageChops.difference(new.convert('RGB'),old.convert('RGB'));clear_name(diff,v,p)
                        assert diff.getbbox() is None,'Accepted ray changed.'
            elif v['valid']:
                def effect(root,variant):
                    with Image.open(root/'frames'/variant/'024.png') as image,Image.open(root/'native'/e['folder']/'control-024.png') as quiet:
                        diff=ImageChops.difference(image.convert('RGB'),quiet.crop(bounds).convert('RGB'))
                        return sum(any(pixel) for pixel in diff.getdata()),diff.getbbox()
                old_count,old_box=effect(BASE,v['variant'].replace('HY05-A02-','HY05-A01-'))
                new_count,new_box=effect(OUT,v['variant'])
                assert new_count>old_count*1.5,(v['variant'],old_count,new_count)
                checks.append({'variant':v['variant'],'old_visible_pixels':old_count,'new_visible_pixels':new_count,'old_bounds':old_box,'new_bounds':new_box})
    for name in ('spirit-source.png','spirit-atlas.png','ray-source.png','ray-atlas.png'):
        assert (OUT/name).read_bytes()==(BASE/name).read_bytes()
    previous.base.dump(OUT/'thickness-verification.json',{'larger_essence':checks,'ray_render_identical':True,'timelines_identical':True,'actors_and_terrain_identical':True,'source_drawings_and_alpha_identical':True,
        'comparison_limit':'Only tiny floating skill-name antialias differences are excluded in the known transient text rectangle at frames 17-33; actor bodies remain compared.'})
    boards=[]
    for i in range(144):
        board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
        for j,(root,variant,label) in enumerate([(BASE,'HY05-A01-HAUT','Version précédente'),(OUT,'HY05-A02-HAUT','Aspiration épaissie')]):
            x=12+j*616;draw.text((x,10),f'23 · Ponction spirituelle · {label}',font=previous.base.font(20,True),fill='#e0e7e7')
            draw.text((x,40),variant,font=previous.base.font(15),fill='#a5b9c5')
            with Image.open(root/'frames'/variant/f'{i:03}.png') as image:board.paste(image,(x,66))
        boards.append(board)
    previous.save_animation(OUT/'ponction-comparaison-64.gif',boards,24)
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'comparison':'native 64px','source_drawings_unchanged':True,'ray_unchanged':True,'timelines_unchanged':True,'status':'HY05-A02 awaiting review'}))


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
