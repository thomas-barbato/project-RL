"""Review 35: dense essence stream and broad occult filament; same prepared rules."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_spirit_thickness as previous

ROOT=previous.ROOT
BASE=previous.OUT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-35'
drains=previous.previous
old_write_html=previous.old_write_html


def protected():
    hashes=previous.protected()
    for p in BASE.rglob('*'):
        if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists()
    drains.base.dump(OUT/'review-protection.json',protected())
    for name in ('spirit-source.png','spirit-atlas.png','ray-source.png','ray-atlas.png','drawing-pack.json','imagegen-spirit-prompt.txt','imagegen-ray-prompt.txt'):
        (OUT/name).write_bytes((BASE/name).read_bytes())
    review=json.loads((BASE/'review.json').read_text(encoding='utf-8'))
    review['status']='awaiting_visual_review'
    for c in review['candidates']:
        c['variant']={'HY05':'HY05-A03','OC05':'OC05-A02'}[c['id']]
        c['status']='awaiting_visual_review';c.pop('acceptance',None)
    review.pop('ray_acceptance',None)
    review['feedback']="non globalement c'est toujours trop leger :) que ce soit le rayon ou la ponction :)"
    review['revision']='Dense multi-drawing moving essence stream for Ponction, broader initial shadow; broad layered black/red/purple filament and larger returning pulse for Rayon. All drawings belong to the same single prepared drain.'
    review['retained']='Exact source PNGs and alpha, purple palette, prepared targeting, timelines, .48-second Ponction, turn-driven Rayon and interruptions. No new gameplay, extra damage, contacts or drains.'
    review['previous_approval']='OC05-A01 was previously accepted, preserved in review 33. The latest feedback explicitly requests revising both skills; neither new variant inherits approval.'
    drains.base.dump(OUT/'review.json',review)
    launch=(BASE/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace(" '--ponction-epaisse'",'').replace("'--lot-occulte-drains'","'--lot-occulte-drains' '--drains-intenses'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'new_drawings':0,'candidates':['HY05-A03','OC05-A02']}))


def write_html(entries):
    old_write_html(entries)
    path=OUT/'index.html';html=path.read_text(encoding='utf-8')
    html=html.replace('<h1>Magie occulte · Ponction et Rayon vorace</h1>','<h1>Drains occultes · flux renforcés</h1>')
    html=html.replace('ANIM-S11 · deux propositions animées · cases natives 64 px','23 / HY05-A03 · 24 / OC05-A02 · densité renforcée · cases natives 64 px')
    html=html.replace('<select id="scenario">','<select id="comparison"><option value="new">Flux renforcés</option><option value="old">Version précédente</option></select><select id="scenario">')
    html=html.replace('im.src=`frames/${v.variant}/${String(i).padStart(3,\'0\')}.png`;',
        "const old=document.getElementById('comparison').value==='old';const variant=old?v.variant.replace('HY05-A03-','HY05-A02-').replace('OC05-A02-','OC05-A01-'):v.variant;im.src=`${old?'../skill-animation-review-34/':''}frames/${variant}/${String(i).padStart(3,'0')}.png`;")
    html=html.replace('scenario.onchange=()=>{',"document.getElementById('comparison').onchange=()=>{launched=true;turn=0;elapsed=0;load();};scenario.onchange=()=>{")
    html=html.replace('HY05-A03 · ombre contractée, essence aspirée une fois','HY05-A03 · ombre contractée et flux dense d’essence')
    html=html.replace('OC05-A02 · lien persistant, pulsation par drain','OC05-A02 · faisceau épais, pulsation par drain')
    html=html.replace('<p>Ponction spirituelle contracte','<p><a href="intensite-comparaison-64.gif">Comparer les deux intensités côte à côte</a></p><p>Ponction spirituelle contracte')
    path.write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(html.split('<script>',1)[1].split('</script>',1)[0],encoding='utf-8')


def package():
    drains.OUT=OUT;drains.protected=protected;drains.write_html=write_html
    drains.package()
    checks=[]
    for e in json.loads((OUT/'native/capture.json').read_text(encoding='utf-8')):
        assert (OUT/'native'/e['folder']/'timeline.json').read_bytes()==(BASE/'native'/e['folder']/'timeline.json').read_bytes()
        for v in e['views']:
            x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h)
            for p in e['control_frames']:
                with Image.open(OUT/'native'/e['folder']/f'control-{p:03}.png') as new,Image.open(BASE/'native'/e['folder']/f'control-{p:03}.png') as old:
                    diff=ImageChops.difference(new.crop(bounds).convert('RGB'),old.crop(bounds).convert('RGB'));previous.clear_name(diff,v,p)
                    assert diff.getbbox() is None,'Actors or terrain changed.'
            if v['valid']:
                phase=24 if v['id']=='HY05' else 47
                old_variant=v['variant'].replace('HY05-A03-','HY05-A02-').replace('OC05-A02-','OC05-A01-')
                def pixels(root,variant):
                    with Image.open(root/'frames'/variant/f'{phase:03}.png') as image,Image.open(root/'native'/e['folder']/f'control-{phase:03}.png') as quiet:
                        diff=ImageChops.difference(image.convert('RGB'),quiet.crop(bounds).convert('RGB'))
                        return sum(any(pixel) for pixel in diff.get_flattened_data()),diff.getbbox()
                old_count,old_box=pixels(BASE,old_variant);new_count,new_box=pixels(OUT,v['variant'])
                assert new_count>old_count*2.0,(v['variant'],old_count,new_count)
                checks.append({'variant':v['variant'],'phase':phase,'old_visible_pixels':old_count,'new_visible_pixels':new_count,'old_bounds':old_box,'new_bounds':new_box})
    for name in ('spirit-source.png','spirit-atlas.png','ray-source.png','ray-atlas.png'):
        assert (OUT/name).read_bytes()==(BASE/name).read_bytes()
    drains.base.dump(OUT/'intensity-verification.json',{'larger_visible_effects':checks,'timelines_identical':True,'actors_and_terrain_identical':True,'source_drawings_and_alpha_identical':True,
        'comparison_limit':'Only tiny floating skill-name antialias differences are excluded in the known transient text rectangle at frames 17-33; actor bodies remain compared.'})
    boards=[]
    for i in range(144):
        board=Image.new('RGB',(1232,1238),'#0b1218');draw=ImageDraw.Draw(board)
        for row,(sid,title,old_version,new_version) in enumerate([('HY05','23 · Ponction spirituelle','A02','A03'),('OC05','24 · Rayon vorace','A01','A02')]):
            for col,(root,version,label) in enumerate([(BASE,old_version,'Version précédente'),(OUT,new_version,'Flux renforcé')]):
                x=12+col*616;y=12+row*616;variant=f'{sid}-{version}-HAUT'
                draw.text((x,y),f'{title} · {label}',font=drains.base.font(19,True),fill='#e0e7e7')
                with Image.open(root/'frames'/variant/f'{i:03}.png') as image:board.paste(image,(x,y+30))
        boards.append(board)
    drains.save_animation(OUT/'intensite-comparaison-64.gif',boards,24)
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    print(json.dumps({'comparison':'native 64px','source_drawings_unchanged':True,'timelines_unchanged':True,'status':'both intensified variants awaiting review'}))


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
