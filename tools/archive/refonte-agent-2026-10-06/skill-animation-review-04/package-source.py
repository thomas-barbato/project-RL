"""ANIM-S03: Charge and Tourbillon, native frames with previously accepted art."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw

import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-04'
CANDIDATES = [
    (8, 'ME05', 'ME05-A01', 'Charge', 'Une ruée jusqu’au contact, avec traînée basse, impact et blessure sanglante.'),
    (9, 'ME06', 'ME06-A01', 'Tourbillon', 'Un croissant de lame fait le tour de l’agent, frappe les huit voisins et laisse les blessures saigner.'),
]


def protected() -> dict[str, str]:
    hashes = gallery.protected()
    previous = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-03'
    for variant in ['B01-A01', 'ME03-A01', 'ME04-A01-MUR', 'ME04-A01-CIBLE']:
        path = previous / f'{variant}-64.gif'
        hashes[path.relative_to(base.ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    path = previous / 'visual-acceptance-01.json'
    hashes[path.relative_to(base.ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    assert not (OUT / 'review-protection.json').exists(), 'Preserve the existing baseline.'
    base.dump(OUT / 'review-protection.json', protected())
    base.dump(OUT / 'review.json', {
        'batch':'ANIM-S03', 'status':'awaiting_visual_review', 'native_tile_size':64,
        'candidates':[{'number':number,'skill_id':sid,'variant':variant,'name':name,'description':description,
                       'status':'awaiting_visual_review'} for number,sid,variant,name,description in CANDIDATES],
        'scope':'Independent visual fixtures; no GameState, damage calculations or campaign integration.',
        'charge_example':'Visible target four cells away; the agent ends in the adjacent free cell and makes one hit.',
        'whirl_example':'Eight adjacent flesh targets; their hit presentation is staggered along the arc, within one action.',
        'bleed_preview':'The accepted 3-turn Bleeding status is shown; frame time does not reduce its turn timer.',
        'laboratory_ui_candidate':{'variant':'LAB-S03','status':'awaiting_separate_review'},
        'unchanged':'Previously accepted icons, actor art, catalogue parameters and animation GIFs.',
    })
    previous = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-03/Ouvrir_laboratoire.ps1'
    launch = previous.read_text(encoding='utf-8-sig').replace('--lot-melee','--lot-assaut')
    (OUT / 'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')


def package() -> None:
    baseline = json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected() == baseline, 'Accepted artwork, catalogue or previous animations changed.'
    entries = json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'],e['id'],e['variant']) for e in entries] == [(n,sid,var) for n,sid,var,_,_ in CANDIDATES]
    sequences = {}
    checks = []
    for entry in entries:
        assert entry['tile']==64 and entry['frames']==90 and entry['fps']==30
        files=sorted((OUT/'native'/entry['folder']).glob('*.png'))
        assert len(files)==entry['frames']
        x,y,w,h=entry['crop']
        frames=[]
        folder=OUT/'frames'/entry['folder']
        folder.mkdir(parents=True,exist_ok=True)
        for file in files:
            with Image.open(file) as im:
                assert list(im.size)==entry['framebuffer_size']==[512,384]
                frame=im.crop((x,y,x+w,y+h)).convert('RGB')
                frame.save(folder/file.name)
                frames.append(frame)
        sequences[entry['variant']]=frames
        checks.append(save_animation(OUT/f"{entry['variant']}-64.gif",frames,30))
        checks.append(save_animation(OUT/f"{entry['variant']}-64-slow.gif",frames,30,True))
    boards=[]
    for index in range(90):
        board=Image.new('RGB',(988,494),'#0b1218')
        draw=ImageDraw.Draw(board)
        draw.text((20,12),'ANIM-S03 · Charge et Tourbillon',font=base.font(24,True),fill='#e0e7e7')
        draw.text((20,47),'08–09 · À valider · cases natives de 64 px',font=base.font(15),fill='#a0b0b8')
        for col,(number,_,variant,title,_) in enumerate(CANDIDATES):
            left=20+col*490
            draw.text((left,86),f'{number:02} · {title}',font=base.font(19,True),fill='#e0e7e7')
            draw.text((left,113),variant,font=base.font(13),fill='#9ab0bc')
            board.paste(sequences[variant][index],(left,146))
        boards.append(board)
    checks.append(save_animation(OUT/'animations-lot-03.gif',boards,30))
    sheet=Image.new('RGB',(1860,722),'#0b1218')
    draw=ImageDraw.Draw(sheet)
    for row,(number,_,variant,title,_) in enumerate(CANDIDATES):
        top=12+row*354
        draw.text((12,top),f'{number:02} · {title}',font=base.font(18,True),fill='#e0e7e7')
        for col,index in enumerate([5,18,25,55]):
            sheet.paste(sequences[variant][index],(12+col*460,top+28))
    sheet.save(OUT/'inspection-phases.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','accepted_files_unchanged':len(baseline),
        'native_capture':'native/capture.json','source':'Macroquad framebuffer','resampling':False,'animations':checks})
    for source,target in [('src/agent_skill_lab_review/assault.rs','render-source.rs'),
                          ('src/agent_skill_lab_review.rs','render-parent-source.rs'),
                          ('src/agent_skill_lab_review/melee.rs','render-helper-source.rs')]:
        (OUT/target).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    (OUT/'gallery-source.py').write_bytes(Path(gallery.__file__).read_bytes())
    gallery.OUT=OUT
    gallery.CANDIDATES=CANDIDATES
    gallery.write_html(entries,batch='ANIM-S03',title='Animations · Lot 03',
        introduction='Charge termine sa ruée au contact. Tourbillon frappe les huit cases voisines, avec le saignement déjà prévu.',
        footer='Aperçus visuels isolés : dégâts réels à raccorder au combat. Le saignement dure trois tours de jeu ; ralentir ou rejouer la lecture ne modifie pas cette durée.')
    print(json.dumps({'output':str(OUT),'accepted_files_unchanged':len(baseline),'animated_files':len(checks)}))


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--prepare',action='store_true')
    args=parser.parse_args()
    prepare() if args.prepare else package()
