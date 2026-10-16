"""ANIM-OCC02: continuous native fire and unchanged discrete targeting preview."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_obstacles as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-08'
CANDIDATES = [
    (10, 'FE02', 'FE02-A04-COULOIR', 'Souffle de feu · couloir',
     'Le ciblage indique les cases touchées, puis les flammes se déploient avec un contour continu dans l’ouverture.'),
    (10, 'FE02', 'FE02-A04-CLOISON', 'Souffle de feu · cloison devant',
     'Le courant se déforme près de la cloison et s’arrête contre sa surface. Les cibles derrière restent protégées.'),
    (10, 'FE02', 'FE02-A04-OBSTACLE', 'Souffle de feu · obstacle partiel',
     'Le courant se déforme autour du bloc, avec une ombre continue. La cible située derrière le bloc reste protégée.'),
]


def protected() -> dict[str,str]:
    hashes = previous.protected()
    for path in sorted(previous.OUT.rglob('*')):
        if path.is_file():
            hashes[path.relative_to(base.ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists(), 'Preserve the initial baseline.'
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'review.json',{
        'batch':'ANIM-OCC02','status':'awaiting_visual_review','native_tile_size':64,
        'user_direction':'Preview the actual affected cells first; let the animation move continuously like a current, respecting obstacles rather than affected-cell edges. Principle confirmed by the player; rendered variants still require review.',
        'candidates':[{'number':n,'skill_id':sid,'variant':v,'name':title,'description':desc,
                       'status':'awaiting_visual_review'} for n,sid,v,title,desc in CANDIDATES],
        'previous_review':'ANIM-OCC01: per-affected-cell visual clipping disliked; original files preserved for comparison.',
        'scope':'Isolated debug laboratory. The targeting footprint, demonstrated affected targets, rank, duration and cooldown rules remain unchanged. No campaign or combat integration.',
        'source_art':'Unchanged painted fire atlas from ANIM-STYLE01. Shader sampling deforms the displayed masses near physical surfaces; source raster assets remain intact.',
        'visual_field':'Overlapping compact round kernels join into a continuous visual field guided by the unchanged gameplay footprint. Source-to-pixel rays prevent kernels crossing solid wall rectangles. Surface normals bend drawn texture motion. There is no hard clip at affected-cell borders; protected target centres and all solid wall pixels stay clear.',
        'visual_vs_gameplay':'The displayed current may use free parts of a cell independently of whether that whole cell is in the gameplay footprint. Cosmetic fragments grant no damage or new propagation. Protected targets remain unaffected.',
        'fluid_scope':'A stylized visual current, not a complete hydrodynamic simulation. No gameplay spreading around a blocking wall, extra range or extra hits are introduced.',
        'targeting_preview':'Discrete affected-cell preview stays identical to ANIM-OCC01. The 0.7-second targeting segment is a review montage, not skill preparation or world-turn advancement.',
        'blood_policy':'Fire without blood. Actors, equipment and compact skill icons unchanged.',
        'style_rule':'General animation style and exact rendered variants still await player validation.',
        'laboratory_ui_candidate':{'variant':'LAB-OCC02','status':'awaiting_separate_review'},
    })
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-souffle-obstacles','--lot-souffle-continu'),encoding='utf-8-sig')


def package() -> None:
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline, 'Earlier drawings or prototypes changed.'
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    old=json.loads((previous.OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'],e['id'],e['variant']) for e in entries]==[(n,sid,v) for n,sid,v,_,_ in CANDIDATES]
    sequences={}; checks=[]; gpu=[]
    for entry,before in zip(entries,old,strict=True):
        assert entry['affected_cells']==before['affected_cells'] and entry['targets']==before['targets']
        assert entry['map']==before['map'] and entry['origin']==before['origin']
        folder=OUT/'native'/entry['folder']
        files=sorted(folder.glob('[0-9][0-9][0-9].png'))
        assert len(files)==entry['frames']==120 and entry['tile']==64 and entry['fps']==30
        x,y,w,h=entry['crop']; frames=[]
        target=OUT/'frames'/entry['folder']; target.mkdir(parents=True,exist_ok=True)
        for file in files:
            with Image.open(file) as im:
                assert im.size==(512,512)
                frame=im.crop((x,y,x+w,y+h)).convert('RGB')
                frame.save(target/file.name); frames.append(frame)
        with Image.open(previous.OUT/'frames'/before['folder']/'010.png') as im:
            assert ImageChops.difference(frames[10],im.convert('RGB')).getbbox() is None, 'Targeting preview changed.'
        with Image.open(folder/'control.png') as im:
            control=im.crop((x,y,x+w,y+h)).convert('RGB')
        diff=ImageChops.difference(frames[60],control)
        assert diff.getbbox(), 'Visual current must be visible.'
        walls=0
        for cy,row in enumerate(entry['map'].splitlines()):
            for cx,symbol in enumerate(row):
                if symbol=='#':
                    assert diff.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None, 'Fire leaked onto solid wall.'
                    walls+=1
        protected_targets=[]
        for target_info in entry['targets']:
            if not target_info['affected']:
                cx,cy=target_info['cell']; px,py=cx*64+32,cy*64+32
                assert diff.crop((px-12,py-12,px+12,py+12)).getbbox() is None, 'Protected target centre received fire.'
                protected_targets.append(target_info['label'])
        gpu.append({'variant':entry['variant'],'solid_wall_cells_unchanged':walls,
                    'protected_target_centres_unchanged':protected_targets,'targeting_preview_identical':True})
        sequences[entry['variant']]=frames
        checks.append(save_animation(OUT/f"{entry['variant']}-64.gif",frames,30))
        checks.append(save_animation(OUT/f"{entry['variant']}-64-slow.gif",frames,30,True))
    comparison=[]
    old_folder=old[2]['folder']; new_variant=entries[2]['variant']
    for i in range(120):
        board=Image.new('RGB',(944,500),'#0b1218'); draw=ImageDraw.Draw(board)
        draw.text((12,12),'Avant · découpe par cases',font=base.font(18,True),fill='#e0e7e7')
        draw.text((480,12),'Essai · courant continu',font=base.font(18,True),fill='#e0e7e7')
        with Image.open(previous.OUT/'frames'/old_folder/f'{i:03}.png') as im:
            board.paste(im,(12,40))
        board.paste(sequences[new_variant][i],(480,40)); comparison.append(board)
    checks.append(save_animation(OUT/'comparaison-obstacle-64.gif',comparison,30))
    comparison[60].save(OUT/'comparaison-obstacle-phase.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','protected_files_unchanged':len(baseline),
        'scene_resampling':False,'native_capture':'native/capture.json','occlusion_checks':gpu,'animations':checks,
        'continuous_boundary':'CPU compact round density kernels and wall constraints, shader texture motion; no affected-cell alpha clip.'})
    for source,name in [
        ('src/agent_skill_lab_review/fire_painted/obstacles/fluid.rs','render-source.rs'),
        ('src/agent_skill_lab_review/fire_painted/obstacles.rs','render-fixture-source.rs'),
        ('src/agent_skill_lab_review.rs','render-parent-source.rs'),
    ]: (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    gallery.OUT=OUT; gallery.CANDIDATES=CANDIDATES
    gallery.write_html(entries,batch='ANIM-OCC02',title='Souffle de feu · courant continu',canvas_height=448,
        introduction='L’aperçu des cases touchées est conservé. Après le lancement, le dessin des flammes utilise des contours continus et se déforme près des surfaces. Les trois scènes restent identiques pour comparer le rendu.',
        footer='Courant visuel proposé, sans changement de dégâts ou de portée. Les murs et les cibles protégées restent préservés. La présentation du ciblage au début de la boucle ne crée aucun délai de préparation du sort.')
    html=(OUT/'index.html').read_text(encoding='utf-8').replace('<main class="grid">',
        '<p><a href="comparaison-obstacle-64.gif">Comparer l’ancien masquage au nouveau courant</a></p><main class="grid">')
    (OUT/'index.html').write_text(html,encoding='utf-8')
    print(json.dumps({'output':str(OUT),'protected_files_unchanged':len(baseline),'animations':len(checks),'gpu_checks':gpu}))


if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--prepare',action='store_true'); args=parser.parse_args()
    prepare() if args.prepare else package()
