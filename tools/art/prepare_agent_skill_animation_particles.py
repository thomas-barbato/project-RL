"""ANIM-PART01: new complete flame fragments, native left/right review."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_melee as gallery
import prepare_agent_skill_animation_continuous_fire as previous
from prepare_agent_skill_animation_revision import save_animation

OUT = base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-09'
CANDIDATES = [
    (10,'FE02',f'FE02-A05-{case}-{side}',f'Souffle · {title} · {direction}',
     'Petites flammes complètes animées, orientées sur des trajets arrondis. Les murs et les cibles protégées restent préservés.')
    for case,title in [('COULOIR','couloir'),('CLOISON','cloison'),('OBSTACLE','obstacle')]
    for side,direction in [('DROITE','droite'),('GAUCHE','gauche')]
]


def protected() -> dict[str,str]:
    hashes=previous.protected()
    for path in sorted(previous.OUT.rglob('*')):
        if path.is_file():
            hashes[path.relative_to(base.ROOT).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes


def prepare() -> None:
    assert not (OUT/'review-protection.json').exists(), 'Preserve baseline.'
    base.dump(OUT/'review-protection.json',protected())
    master=OUT/'sources/particle-loop-master.png'
    sheet=Image.open(master).convert('RGBA')
    assert sheet.getchannel('A').getextrema()==(0,255)
    atlas=Image.new('RGBA',(64*8,64)); anchors=[]; radius=0.0
    folder=OUT/'drawn-frames'; folder.mkdir(exist_ok=True)
    for frame in range(8):
        col,row=frame%4,frame//4
        bounds=(round(col*sheet.width/4),round(row*sheet.height/2),round((col+1)*sheet.width/4),round((row+1)*sheet.height/2))
        source=sheet.crop(bounds)
        bbox=source.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
        assert bbox
        anchor=((bbox[0]+bbox[2])/2,(bbox[1]+bbox[3])/2)
        scale=.14
        small=source.resize((round(source.width*scale),round(source.height*scale)),Image.Resampling.LANCZOS)
        offset=(round(32-anchor[0]*scale),round(32-anchor[1]*scale))
        native=Image.new('RGBA',(64,64)); native.alpha_composite(small,offset)
        # Preserve real generated alpha. Measure all nonzero pixels for collisions.
        for y in range(64):
            for x in range(64):
                if native.getpixel((x,y))[3]:
                    radius=max(radius,((x+0.5-32)**2+(y+0.5-32)**2)**.5)
        native.save(folder/f'{frame:02}.png'); atlas.alpha_composite(native,(frame*64,0))
        anchors.append({'frame':frame,'source_cell':bounds,'source_anchor':anchor,'destination_anchor':[32,32],'scale':scale,'offset':offset})
    atlas.save(OUT/'particle-atlas.png')
    base.dump(OUT/'conditioning.json',{'method':'Equal-cell slicing, constant scale, centre alignment. Generated alpha intact; no repainting or recolouring. Alpha threshold used only to measure anchors.',
                                      'native_frame_size':[64,64],'maximum_alpha_radius':radius,'frames':anchors})
    assert radius<31, f'Particle needs larger collision bounds: {radius}'
    base.dump(OUT/'review.json',{
        'batch':'ANIM-PART01','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'skill_id':sid,'variant':v,'name':name,'description':desc,'status':'awaiting_visual_review'}
                       for n,sid,v,name,desc in CANDIDATES],
        'user_direction':'Test the proposed modular animated flames. All animations must support aiming left and right regardless of the character pose.',
        'source_art':'New transparent eight-frame small flame fragment generated with built-in image_gen. Approved red-robed character is the comic style reference; older flame is the palette reference.',
        'construction':'Complete animated fragments, moving on cached short paths with rounded corners. Ordinary texture draws; no whole-cone image, shader deformation, opacity field or per-affected-cell clipping.',
        'obstacles':'Conservative circular bounds contain the entire alpha sprite in every orientation. Fragments fit the physical free space, fade near each stream endpoint and never cross solid walls.',
        'routing_scope':'Visual fixture routing only, guided by the unchanged preview footprint. No gameplay propagation around a blocking wall or changed damage.',
        'direction':'World positions, maps, footprints and target flags mirror left/right. Texture orientation follows each path. The frontal character art and equipment are not flipped or redrawn.',
        'timing':'The montage previews targeting for 0.7 seconds, then displays the cast. Flame populations are visually prefilled on launch; no extra skill preparation or progressive damage is introduced. Playback never advances turns.',
        'scope':'Isolated debug proposal. No campaign, damage calculation, RNG, save, balance or icon changes.',
        'laboratory_ui_candidate':{'variant':'LAB-PART01','status':'awaiting_separate_review'},
        'fallback':'ANIM-OCC02 remains intact and retained as fallback, without acceptance implied.',
        'blood_policy':'Fire remains without blood.',
    })
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-souffle-continu','--lot-souffle-particules'),encoding='utf-8-sig')
    print(json.dumps({'maximum_alpha_radius':radius,'protected_files':len(protected())}))


def package() -> None:
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline, 'Earlier content changed.'
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert [(e['number'],e['id'],e['variant']) for e in entries]==[(n,sid,v) for n,sid,v,_,_ in CANDIDATES]
    old=json.loads((previous.OUT/'native/capture.json').read_text(encoding='utf-8'))
    sequences={}; checks=[]; safety=[]
    for i,entry in enumerate(entries):
        if not entry['left']:
            assert entry['affected_cells']==old[i//2]['affected_cells']
            assert entry['targets']==old[i//2]['targets']
        else:
            right=entries[i-1]
            assert entry['affected_cells']==[[6-x,y] for x,y in right['affected_cells']]
            assert [t['affected'] for t in entry['targets']]==[t['affected'] for t in right['targets']]
        assert entry['tile']==64 and entry['fps']==30 and entry['frames']==90
        folder=OUT/'native'/entry['folder']; frames=[]
        destination=OUT/'frames'/entry['folder']; destination.mkdir(parents=True,exist_ok=True)
        x,y,w,h=entry['crop']
        for frame in range(90):
            name=f'{frame:03}.png'
            with Image.open(folder/name) as im:
                assert im.size==(512,512)
                scene=im.crop((x,y,x+w,y+h)).convert('RGB'); scene.save(destination/name); frames.append(scene)
        if not entry['left']:
            with Image.open(previous.OUT/'frames'/old[i//2]['folder']/'010.png') as im:
                assert ImageChops.difference(frames[10],im.convert('RGB')).getbbox() is None, 'Targeting preview changed.'
        with Image.open(folder/'control.png') as im:
            diff=ImageChops.difference(frames[45],im.crop((x,y,x+w,y+h)).convert('RGB'))
        assert diff.getbbox(), 'No visible flames.'
        walls=0
        for cy,row in enumerate(entry['map'].splitlines()):
            for cx,symbol in enumerate(row):
                if symbol=='#':
                    assert diff.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None, f'Wall overlap: {entry["variant"]} {(cx,cy)}'
                    walls+=1
        protected_targets=[]
        for target in entry['targets']:
            if not target['affected']:
                cx,cy=target['cell']; px,py=cx*64+32,cy*64+32
                assert diff.crop((px-12,py-12,px+12,py+12)).getbbox() is None, 'Protected target received fire.'
                protected_targets.append(target['label'])
        safety.append({'variant':entry['variant'],'solid_wall_cells_unchanged':walls,'protected_target_centres_unchanged':protected_targets})
        sequences[entry['variant']]=frames
        checks.append(save_animation(OUT/f"{entry['variant']}-64.gif",frames,30))
        checks.append(save_animation(OUT/f"{entry['variant']}-64-slow.gif",frames,30,True))
    # Each comparison retains native pixels, one direction per panel.
    for case in range(3):
        boards=[]
        for frame in range(90):
            board=Image.new('RGB',(944,500),'#0b1218'); draw=ImageDraw.Draw(board)
            for side,label in enumerate(['Vers la droite','Vers la gauche']):
                left=12+side*468
                draw.text((left,12),label,font=base.font(18,True),fill='#e0e7e7')
                board.paste(sequences[entries[case*2+side]['variant']][frame],(left,40))
            boards.append(board)
        name=['couloir','cloison','obstacle'][case]
        checks.append(save_animation(OUT/f'{name}-deux-directions-64.gif',boards,30))
        boards[45].save(OUT/f'{name}-phase.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','protected_files_unchanged':len(baseline),
        'native_capture':'native/capture.json','scene_resampling':False,'occlusion_checks':safety,'animations':checks,
        'directions':'Six native scenes; exact mirrored affected-cell lists and target flags; frontal character unchanged.'})
    for source,name in [
        ('src/agent_skill_lab_review/fire_painted/obstacles/particles.rs','render-source.rs'),
        ('src/agent_skill_lab_review/fire_painted/obstacles.rs','render-fixture-source.rs'),
        ('src/agent_skill_lab_review.rs','render-parent-source.rs'),
    ]: (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    gallery.OUT=OUT; gallery.CANDIDATES=CANDIDATES
    gallery.write_html(entries,batch='ANIM-PART01',title='Souffle modulaire · gauche et droite',canvas_height=448,
        introduction='Six scènes : les mêmes couloir, cloison et bloc dans les deux directions. Les petits dessins de flammes restent complets, se déplacent et s’orientent sur leurs trajets. Le début de chaque boucle montre les cases touchées.',
        footer='Propositions avant validation. Le ciblage et les cibles protégées restent conservés. Les trajets concernent seulement le rendu, sans dégâts supplémentaires ou propagation de gameplay. Les montages ne font avancer aucun tour.')
    print(json.dumps({'output':str(OUT),'protected_files_unchanged':len(baseline),'animations':len(checks),'native_scenes':6,'checks':safety}))


if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--prepare',action='store_true'); args=parser.parse_args()
    prepare() if args.prepare else package()
