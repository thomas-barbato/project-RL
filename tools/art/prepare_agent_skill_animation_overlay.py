"""ANIM-OVER01: full painted animation, physical wall covers, blinking footprint."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_particles as previous
from prepare_agent_skill_animation_revision import save_animation

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-10'
DIRECTIONS=['DROITE','GAUCHE','HAUT','BAS']
CANDIDATES=[
    (10,'FE02',f'FE02-A06-{case}-{direction}',f'Souffle · {title} · {direction.lower()}',
     'Le dessin reste complet sur les sols libres. Les surfaces des murs le masquent, les personnages restent devant et les contours clignotants indiquent les cases touchées.')
    for case,title in [('COULOIR','couloir'),('CLOISON','cloison'),('OBSTACLE','obstacle')]
    for direction in DIRECTIONS
]


def protected() -> dict[str,str]:
    hashes=previous.protected()
    for path in sorted(previous.OUT.rglob('*')):
        if path.is_file():
            hashes[path.relative_to(base.ROOT).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists(), 'Preserve the baseline.'
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'review.json',{
        'batch':'ANIM-OVER01','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'skill_id':sid,'variant':v,'name':name,'description':desc,'status':'awaiting_visual_review'}
                      for n,sid,v,name,desc in CANDIDATES],
        'user_direction':'Large complete animation, more visible blinking targeting preview, four cardinal aiming directions. Flames behind characters. User agrees to mask only the part on a solid wall, without removing the entire visual area behind that wall.',
        'source_art':'Exact original ANIM-STYLE01 painted cone and plume atlases; no new generated images, recolouring or deformations.',
        'rendering':'One complete cone drawing rotated by 0/180/-90/90 degrees. Opaque wall tiles repaint above it; free floor receives no visual-footprint clipping or projected shadow. Bodies stay frontal and above all flames. Target contours and review labels stay above the effect.',
        'preview':'Cyan contours 4px thick, pale inner line, dark backing and translucent fill. One brightness cycle per second, alternating full and 42% brightness so the affected zone never disappears. Contours remain visible during the animation.',
        'mechanics':'Affected-cell lists and target flags retain the earlier footprint, rotated into each direction. Decorative flames may be visible on a protected free floor behind a wall; this never grants damage, status, extra range or altered propagation.',
        'burn':'Original small flame animation selected by the user for temporary Brûlure. It is displayed behind affected actors, remains upright regardless of spell aim, and is not shown on protected targets.',
        'scope':'Isolated native debug review. No campaign, GameState, damage calculations, RNG, skill text, cooldown, save, icon or balance changes.',
        'timing':'0.7-second targeting segment is a review montage only. Visual playback and blinking never advance turns or add a preparation delay.',
        'laboratory_ui_candidate':{'variant':'LAB-OVER01','status':'awaiting_separate_review'},
        'prior_work':'ANIM-OCC02 remains intact as fallback. Rejected ANIM-PART01 remains archived.',
    })
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-souffle-particules','--lot-souffle-superpose'),encoding='utf-8-sig')
    source=(base.ROOT/'src/agent_skill_lab_review/fire_painted.rs').read_text(encoding='utf-8')
    start=source.index('    fn burn('); end=source.index('\n}',start)
    burn=source[start:end]
    paths=[base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-06/flame-atlas.png']
    paths+=sorted((base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-06/drawn-frames/flame').glob('*.png'))
    base.dump(OUT/'burn-visual-acceptance-01.json',{
        'review_id':'ANIM-BURN01','date':'2026-10-12','visual_shape_accepted':True,
        'user_acceptance':'parcontre j’ai noté que tu avais fait un effet "flammes" sur les personnages :) c’est génial et ca , ca devra etre l’animation de l’effet temporaire "brulure"',
        'layer_instruction':'fait passer les flammes derrières les personnages s’il te plais :) ca rend super',
        'scope':'Small existing painted flames assigned to the temporary Burn status. Shape approval and behind-body instruction; no damage, chance, duration or other status rule changed.',
        'source_function':burn,'source_function_sha256':hashlib.sha256(burn.encode()).hexdigest(),
        'assets':{p.relative_to(base.ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},
        'campaign_integration':False,'new_overlay_preview_accepted':False,
    })


def package() -> None:
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline, 'Earlier content changed.'
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'],e['id'],e['variant']) for e in entries]==[(n,sid,v) for n,sid,v,_,_ in CANDIDATES]
    old=json.loads((base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-08/native/capture.json').read_text(encoding='utf-8'))
    assert [entry['variant'] for entry in old]==[f'FE02-A04-{case}' for case in ('COULOIR','CLOISON','OBSTACLE')]
    sequences={}; checks=[]; wall_checks=[]
    for i,entry in enumerate(entries):
        if i%4==0:
            assert entry['affected_cells']==old[i//4]['affected_cells']
            assert entry['targets']==old[i//4]['targets']
        assert entry['frames']==90 and entry['fps']==30 and entry['tile']==64
        folder=OUT/'native'/entry['folder']; target=OUT/'frames'/entry['variant']; target.mkdir(parents=True,exist_ok=True)
        x,y,w,h=entry['crop']; frames=[]
        for frame in range(90):
            name=f'{frame:03}.png'
            with Image.open(folder/name) as im:
                assert im.size==(1024,1024)
                scene=im.crop((x,y,x+w,y+h)).convert('RGB'); scene.save(target/name); frames.append(scene)
        with Image.open(folder/'control.png') as im:
            diff=ImageChops.difference(frames[30],im.crop((x,y,x+w,y+h)).convert('RGB'))
        assert diff.getbbox(), 'Overlay must be visible.'
        walls=0
        for cy,row in enumerate(entry['map'].splitlines()):
            for cx,symbol in enumerate(row):
                if symbol=='#':
                    assert diff.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None, f'Wall cover failed: {entry["variant"]} {(cx,cy)}'
                    walls+=1
        wall_checks.append({'variant':entry['variant'],'solid_wall_cells_unchanged':walls,
                            'protected_targets_without_burn':[t['label'] for t in entry['targets'] if not t['affected']]})
        # Preview is isolated before the cast. Sample a contour pixel on an affected tile.
        cx,cy=entry['affected_cells'][0]; pixel=(cx*64+3,cy*64+25)
        bright=frames[3].getpixel(pixel); dim=frames[18].getpixel(pixel)
        assert bright[1]>dim[1]+30 and dim[1]>50, f'Blink not visible: {entry["variant"]} {bright} {dim}'
        sequences[entry['variant']]=frames
        checks.append(save_animation(OUT/f"{entry['variant']}-64.gif",frames,30))
    for case in range(3):
        boards=[]
        for frame in range(90):
            board=Image.new('RGB',(952,1000),'#0b1218'); draw=ImageDraw.Draw(board)
            for direction,label in enumerate(['Droite','Gauche','Haut','Bas']):
                left=12+(direction%2)*476; top=12+(direction//2)*492
                draw.text((left,top),label,font=base.font(18,True),fill='#e0e7e7')
                board.paste(sequences[entries[case*4+direction]['variant']][frame],(left,top+30))
            boards.append(board)
        name=['couloir','cloison','obstacle'][case]
        checks.append(save_animation(OUT/f'{name}-quatre-directions-64.gif',boards,30))
        boards[30].save(OUT/f'{name}-phase.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','protected_files_unchanged':len(baseline),
        'native_capture':'native/capture.json','scene_resampling':False,'wall_cover_checks':wall_checks,
        'preview_blink':'Each of twelve views: bright vs dim cyan contour pixels differ by more than 30 green levels, low phase stays visible.',
        'animations':checks,'directions':'Twelve native views, cardinal rotations without flipping or rotating character art.'})
    for source,name in [('src/agent_skill_lab_review/fire_painted/obstacles/overlay.rs','render-source.rs'),
                         ('src/agent_skill_lab_review.rs','render-parent-source.rs'),
                         ('src/agent_skill_lab_review/fire_painted.rs','painted-source.rs'),
                         ('src/agent_skill_lab_review/fire_painted/obstacles.rs','fixture-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    gallery.OUT=OUT; gallery.CANDIDATES=CANDIDATES
    gallery_entries=[dict(entry,folder=entry['variant']) for entry in entries]
    gallery.write_html(gallery_entries,batch='ANIM-OVER01',title='Souffle superposé · quatre directions',canvas_height=448,
        introduction='Grande animation complète dans les espaces libres. Les surfaces des murs la masquent ; les personnages passent devant. Le contour cyan clignotant indique les cases réellement touchées et reste visible pendant le souffle.',
        footer='Les flammes peuvent rester visibles sur un sol protégé derrière une cloison : les effets restent limités aux cases indiquées. Les petites flammes de Brûlure sont derrière les personnages. Prototype avant validation, sans changement des règles ni avance de tour.')
    print(json.dumps({'output':str(OUT),'protected_files_unchanged':len(baseline),'views':12,'animations':len(checks),'wall_checks':wall_checks}))


if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--prepare',action='store_true'); args=parser.parse_args()
    prepare() if args.prepare else package()
