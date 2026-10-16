"""ANIM-OVER03: restore painted flame shapes beyond the corridor opening."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_stopped as previous

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-12'
CANDIDATES=[(10,'FE02',f'FE02-A08-COULOIR-{direction}',f'Couloir puis pièce · {label}',
    'À la sortie du couloir, les flammes retrouvent leur dessin ample sur le sol libre. Les murs continuent à les arrêter.')
    for direction,label in [('DROITE','droite'),('GAUCHE','gauche'),('HAUT','haut'),('BAS','bas')]]

def protected():
    hashes=previous.protected()
    for path in sorted(previous.OUT.rglob('*')):
        if path.is_file():hashes[path.relative_to(base.ROOT).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes

def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists(),'Preserve the baseline.'
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'review.json',{
        'batch':'ANIM-OVER03','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'skill_id':sid,'variant':v,'name':name,'description':desc,'status':'awaiting_visual_review'} for n,sid,v,name,desc in CANDIDATES],
        'user_request':'At the end of a corridor leading to open space, retain the drawn shape even with a little overflow past cell borders instead of a geometrically cut cone.',
        'rendering':'The existing painted cone remains unchanged. A corridor opening that the original emitter can reach provides an additional visual visibility origin at its free entrance. In the room, the original drawn flame shapes can spread past gameplay-cell edges; rays from the opening still stop on physical walls.',
        'scope':'Visual mask change only in the debug review. No image generation, source deformation, gameplay propagation, range, damage, status chance, icon, actor, save or balance changes.',
        'unchanged':'Targeting disappears on cast; Burn remains attached to affected bodies and keeps its independent montage lifetime. Full walls and other fixtures keep their previous masking.',
        'timing':'Review montage only, with the same targeting/breath/Burn phases as A07; no game turns advance.',
        'laboratory_ui_candidate':{'variant':'LAB-OVER03','status':'awaiting_separate_review'},
        'comparison':'A07 left, A08 right; both same native 64px terrain, actor positions and phase.',
    })
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-souffle-arrete','--lot-souffle-ouverture'),encoding='utf-8-sig')

def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline,'Previous art or reviews changed.'
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    old_entries=json.loads((previous.OUT/'native/capture.json').read_text(encoding='utf-8'))[:4]
    regressions=json.loads((OUT/'native/mask-regression.json').read_text(encoding='utf-8'))
    assert len(regressions)==4 and all(e['mask_pixels_unchanged'] for e in regressions)
    assert [e['variant'] for e in entries]==[v for _,_,v,_,_ in CANDIDATES]
    sequences={};old_sequences={};checks=[];controls=[]
    for e,old in zip(entries,old_entries):
        assert e['affected_cells']==old['affected_cells'] and e['targets']==old['targets']
        x,y,w,h=e['crop'];target=OUT/'frames'/e['variant'];target.mkdir(parents=True,exist_ok=True)
        frames=[];before=[]
        for i in range(72):
            with Image.open(OUT/'native'/e['folder']/f'{i:03}.png') as im:
                assert im.size==(1024,1024)
                frame=im.crop((x,y,x+w,y+h)).convert('RGB');frame.save(target/f'{i:03}.png');frames.append(frame)
            with Image.open(previous.OUT/'frames'/old['variant']/f'{i:03}.png') as im:before.append(im.convert('RGB'))
        for phase in (3,14,52,71):
            assert frames[phase].tobytes()==before[phase].tobytes(),f'Other phases changed: {e["variant"]} {phase}'
        with Image.open(OUT/'native'/e['folder']/'control-024.png') as im:
            effect=ImageChops.difference(frames[24],im.crop((x,y,x+w,y+h)).convert('RGB'))
        for cy,row in enumerate(e['map'].splitlines()):
            for cx,symbol in enumerate(row):
                if symbol=='#':assert effect.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None
        for actor in e['targets']:
            if not actor['affected']:
                cx,cy=actor['cell'];assert effect.crop((cx*64+12,cy*64+12,cx*64+52,cy*64+52)).getbbox() is None,'Protected body visually engulfed.'
        assert ImageChops.difference(frames[24],before[24]).getbbox(),'Opening did not regain any painted flame.'
        sequences[e['variant']]=frames;old_sequences[e['variant']]=before
        checks.append(previous.animation(OUT/f"{e['variant']}-64.gif",frames,24))
        controls.append({'variant':e['variant'],'wall_pixels_unchanged':True,'gameplay_cells_and_targets_unchanged':True,'preview_and_burn_phases_identical':True,'painted_room_shape_restored':True})
    boards=[];comparison=[]
    for i in range(72):
        board=Image.new('RGB',(952,1000),'#0b1218');draw=ImageDraw.Draw(board)
        for index,e in enumerate(entries):
            left=12+(index%2)*476;top=12+(index//2)*492
            draw.text((left,top),e['direction'].capitalize(),font=base.font(18,True),fill='#e0e7e7')
            board.paste(sequences[e['variant']][i],(left,top+30))
        boards.append(board)
        pair=Image.new('RGB',(928,498),'#0b1218');painter=ImageDraw.Draw(pair)
        painter.text((12,12),'A07 · coupe en cône',font=base.font(18,True),fill='#e0e7e7')
        painter.text((476,12),'A08 · dessin après ouverture',font=base.font(18,True),fill='#e0e7e7')
        key=entries[0]['variant'];pair.paste(old_sequences[key][i],(12,42));pair.paste(sequences[key][i],(476,42));comparison.append(pair)
        comparison_folder=OUT/'frames/COMPARE-A07-A08';comparison_folder.mkdir(parents=True,exist_ok=True);pair.save(comparison_folder/f'{i:03}.png')
    checks.append(previous.animation(OUT/'couloir-piece-quatre-directions-64.gif',boards,24))
    checks.append(previous.animation(OUT/'comparaison-ouverture-64.gif',comparison,24))
    boards[24].save(OUT/'couloir-piece-phase.png');comparison[24].save(OUT/'comparaison-phase.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','protected_files_unchanged':len(baseline),'resampling':False,'controls':controls,'animations':checks,'unchanged_other_masks':regressions,'scope':'Only the corridor-opening mask changes. Other fixture rules are covered by native mask regression and targeted tests.'})
    for source,name in [('src/agent_skill_lab_review/fire_painted/obstacles/overlay/stopped/expanded.rs','opening-source.rs'),('src/agent_skill_lab_review/fire_painted/obstacles/overlay/stopped.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted.rs','painted-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    gallery.OUT=OUT;gallery.CANDIDATES=CANDIDATES
    gallery_manifest=[dict(e,folder=e['variant']) for e in entries]+[{'id':'COMPARE','variant':'COMPARE-A07-A08','folder':'COMPARE-A07-A08','fps':24,'frames':72}]
    gallery.write_html(gallery_manifest,batch='ANIM-OVER03',title='Couloir puis pièce · formes dessinées',canvas_height=448,
        introduction='À la sortie du couloir, le dessin reprend son ampleur dans la pièce et peut déborder sur les cases libres. Les murs arrêtent toujours les flammes.',
        footer='Les cases touchées restent celles de l’aperçu, qui disparaît au lancement. Brûlure reste sur les corps affectés. Le dessin et les paramètres sont conservés ; seul le masque visuel à l’ouverture change.')
    html=(OUT/'index.html').read_text(encoding='utf-8')
    html=html.replace('<main class="grid">','<section><h2>Avant / Après · A07 à gauche, A08 à droite</h2><canvas data-variant="COMPARE-A07-A08" width="928" height="498" style="width:928px;height:498px"></canvas></section><main class="grid">')
    (OUT/'index.html').write_text(html,encoding='utf-8')
    print(json.dumps({'views':4,'animations':len(checks),'protected_files_unchanged':len(baseline),'controls':controls}))

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
