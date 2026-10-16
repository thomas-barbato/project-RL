"""ANIM-OVER02: native obstacle cuts, targeting dismissal, independent Burn."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageChops, ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_overlay as previous

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-11'
DIRECTIONS=['DROITE','GAUCHE','HAUT','BAS']
CASES=['COULOIR','CLOISON','OBSTACLE','LONG','LONG-CLOISON']

def protected():
    hashes=previous.protected()
    for path in sorted(previous.OUT.rglob('*')):
        if path.is_file():
            hashes[path.relative_to(base.ROOT).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    return hashes

def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists(), 'Preserve the baseline.'
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'visual-acceptance-01.json',{
        'review':'ANIM-OVER01','date':'2026-10-12','source_review':'skill-animation-review-10',
        'user_acceptance':'C’est très bien [...] Je valide donc',
        'accepted':'Existing painted drawings and visual direction, cardinal directions, flames behind actors.',
        'requested_corrections':['Every physical obstacle cuts the animation at its position and prevents its blocked part reappearing behind it.','Target preview disappears on attack confirmation.','Burn remains visible on affected players and monsters.','Add a long corridor test.'],
        'new_render_accepted':False,'campaign_integration':False,
        'reference_hashes':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in previous.OUT.glob('*quatre-directions-64.gif')},
    })
    base.dump(OUT/'review.json',{
        'batch':'ANIM-OVER02','status':'awaiting_corrected_render_review','native_tile_size':64,
        'base_direction_accepted':True,'new_variants':'FE02-A07 / BRULURE-A02-AGENT-MONSTRE',
        'obstacles':'Per-pixel rays from the existing emitter stop at physical wall rectangles. The source drawing is neither deformed nor cut by gameplay-cell borders. A blocking wall cuts its shadow too; other open rays continue.',
        'preview':'Same blinking cyan outline before launch only. No fill or outline survives confirmation.',
        'burn':'Accepted small flame art, upright and attached to the affected body. Main flames behind it and sparse foreground tongues on its silhouette. Separate presentation lifetime continues after the breath stops, then ends. Separate fixture shows the player and a monster; no friendly fire is added.',
        'long_corridor':'13 by 7 cells; eleven free cells in a straight corridor, and a second version barred by a wall. Five-cell skill range retained. Horizontal views captured; all four directions selectable in the native laboratory.',
        'source_art':'Original review-06 cone and plume; no image regeneration, recolouring, body changes or source art deformation.',
        'timing':'0.7s targeting, breath until 1.8s, Burn until 2.7s in the review montage only. These durations never replace skill turns or advance GameState.',
        'scope':'Debug visual prototype. No campaign damage, chance, status duration, range, saves, balance or icons changed.',
        'laboratory_ui_candidate':{'variant':'LAB-OVER02','status':'awaiting_separate_review'},
    })
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-souffle-superpose','--lot-souffle-arrete'),encoding='utf-8-sig')

def animation(path,frames,fps):
    boundaries=[round(i*1000/fps/10)*10 for i in range(len(frames)+1)]
    palette=base.palette_for(frames)
    quantized=[im.quantize(palette=palette,dither=Image.Dither.NONE) for im in frames]
    quantized[0].save(path,save_all=True,append_images=quantized[1:],duration=[b-a for a,b in zip(boundaries,boundaries[1:])],loop=0,disposal=2,optimize=False)
    with Image.open(path) as im:
        duration=0; states=set()
        for i in range(im.n_frames):
            im.seek(i);duration+=im.info['duration'];states.add(hashlib.sha256(im.convert('RGB').tobytes()).hexdigest())
        assert len(states)>5 and duration==3000
        return {'file':path.name,'native_size':list(im.size),'distinct_frames':len(states),'duration_ms':duration,'loop':im.info['loop']}

def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline, 'Earlier art or review changed.'
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries)==17
    old=json.loads((previous.OUT/'native/capture.json').read_text(encoding='utf-8'))
    sequences={}; checks=[]; controls=[]
    for index,entry in enumerate(entries):
        if index<12:
            assert entry['affected_cells']==old[index]['affected_cells'] and entry['targets']==old[index]['targets']
        assert entry['tile']==64 and entry['frames']==72 and entry['fps']==24
        x,y,w,h=entry['crop'];folder=OUT/'native'/entry['folder'];target=OUT/'frames'/entry['variant'];target.mkdir(parents=True,exist_ok=True)
        frames=[]
        for i in range(entry['frames']):
            with Image.open(folder/f'{i:03}.png') as im:
                assert im.size==(1024,1024)
                crop=im.crop((x,y,x+w,y+h)).convert('RGB');crop.save(target/f'{i:03}.png');frames.append(crop)
        with Image.open(folder/'control-052.png') as im:
            quiet=im.crop((x,y,x+w,y+h)).convert('RGB')
        burn_diff=ImageChops.difference(frames[52],quiet)
        assert frames[71].tobytes()==quiet.tobytes(), f'Status did not end: {entry["variant"]}'
        if entry['id']=='FE02':
            with Image.open(folder/'control-024.png') as im:
                control=im.crop((x,y,x+w,y+h)).convert('RGB')
            effect_diff=ImageChops.difference(frames[24],control)
            assert effect_diff.getbbox(), 'Breath missing.'
            for cy,row in enumerate(entry['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol=='#':
                        bounds=(cx*64,cy*64,(cx+1)*64,(cy+1)*64)
                        assert effect_diff.crop(bounds).getbbox() is None, f'Wall overlap: {entry["variant"]}'
                        assert burn_diff.crop(bounds).getbbox() is None, f'Burn on wall: {entry["variant"]}'
            for actor in entry['targets']:
                cx,cy=actor['cell'];bounds=(cx*64,cy*64,(cx+1)*64,(cy+1)*64)
                if actor['affected']: assert burn_diff.crop(bounds).getbbox(), f'Burn missing: {entry["variant"]}'
                else:
                    assert effect_diff.crop(bounds).getbbox() is None, f'Fire beyond obstacle/range: {entry["variant"]}'
                    assert burn_diff.crop(bounds).getbbox() is None, f'Protected target burns: {entry["variant"]}'
            if entry['folder'] in ['CLOISON','LONG-CLOISON']:
                with Image.open(folder/'visibility.png') as im: mask=im.convert('L')
                if entry['direction']=='GAUCHE':mask=mask.transpose(Image.Transpose.ROTATE_180)
                elif entry['direction']=='HAUT':mask=mask.transpose(Image.Transpose.ROTATE_90)
                elif entry['direction']=='BAS':mask=mask.transpose(Image.Transpose.ROTATE_270)
                blocked=mask.point(lambda v:255-v)
                assert ImageChops.multiply(effect_diff,Image.merge('RGB',(blocked,blocked,blocked))).getbbox() is None, f'Fire behind full wall: {entry["variant"]}'
            cx,cy=entry['affected_cells'][0];pixel=(cx*64+3,cy*64+25)
            bright=frames[3].getpixel(pixel);dim=frames[14].getpixel(pixel)
            assert bright[1]>dim[1]+30 and dim[1]>50
            for frame in (17,24,52):
                r,g,b=frames[frame].getpixel(pixel)
                assert not (r<100 and g>140 and b>140), 'Preview persists after confirmation.'
            controls.append({'variant':entry['variant'],'walls_and_protected_targets_unchanged':True,'blink_before_cast_only':True,'burn_after_breath_then_expires':True})
        else:
            for cx,cy in entry['actors']:
                assert burn_diff.crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox(), 'Actor Burn missing.'
            controls.append({'variant':entry['variant'],'player_and_monster_burn_without_breath':True,'ends_with_status':True})
        sequences[entry['variant']]=frames
        checks.append(animation(OUT/f"{entry['variant']}-64.gif",frames,entry['fps']))
    for case in CASES:
        views=[e for e in entries if e['folder']==case];width=952 if len(views)==4 else 856; height=1000
        boards=[]
        for i in range(72):
            board=Image.new('RGB',(width,height),'#0b1218');draw=ImageDraw.Draw(board)
            for column,e in enumerate(views):
                left=12+(column%2)*476 if len(views)==4 else 12
                top=12+(column//2)*492 if len(views)==4 else 12+column*492
                draw.text((left,top),e['direction'].capitalize(),font=base.font(18,True),fill='#e0e7e7')
                board.paste(sequences[e['variant']][i],(left,top+30))
            boards.append(board)
        stem=case.lower();checks.append(animation(OUT/f'{stem}-directions-64.gif',boards,24));boards[24].save(OUT/f'{stem}-phase.png');boards[52].save(OUT/f'{stem}-brulure.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_corrected_render_review','protected_files_unchanged':len(baseline),'native_capture':'native/capture.json','resampling':False,'controls':controls,'animations':checks})
    for source,name in [('src/agent_skill_lab_review/fire_painted/obstacles/overlay/stopped.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/obstacles/overlay.rs','render-base-source.rs'),('src/agent_skill_lab_review/fire_painted.rs','painted-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'views':17,'animations':len(checks),'protected_files_unchanged':len(baseline),'controls':controls}))

def write_html(entries):
    cards=[]
    for e in entries:
        w,h=e['crop'][2:];label=e['title']+' · '+e.get('direction','').lower()
        cards.append(f'<article class="{"wide" if w>448 else ""}"><h2>{e["number"]} · {label}</h2><p>{e["variant"]}</p><canvas data-variant="{e["variant"]}" width="{w}" height="{h}" style="width:{w}px;height:{h}px"></canvas></article>')
    manifest=[dict(e,folder=e['variant']) for e in entries]
    script='''const manifest=MANIFEST;const sequences=new Map();let elapsed=0,last=0,paused=false,ready=false;
const speed=document.getElementById('speed'),pause=document.getElementById('pause');
async function load(){for(const e of manifest){const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.folder}/${String(i).padStart(3,'0')}.png`;})));sequences.set(e.variant,{e,images});}elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · 64 px natifs';}
function tick(now){if(last&&!paused)elapsed+=(now-last)*Number(speed.value);last=now;if(ready)for(const c of document.querySelectorAll('canvas')){const{e,images}=sequences.get(c.dataset.variant);c.getContext('2d').drawImage(images[Math.floor(elapsed*e.fps/1000)%e.frames],0,0);}requestAnimationFrame(tick);}
pause.onclick=()=>{paused=!paused;pause.textContent=paused?'Lecture':'Pause';};document.getElementById('replay').onclick=()=>{elapsed=0;};load().catch(()=>document.getElementById('loading').textContent='Conserver les sous-dossiers avec cette galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Souffle arrêté et Brûlure</title>
<style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1060px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:18px}p{line-height:1.5;color:#a8b7bf}.controls{display:flex;gap:12px;margin:24px 0;flex-wrap:wrap}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:9px 13px;font:inherit;cursor:pointer}.grid{display:grid;grid-template-columns:repeat(2,480px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}article.wide{grid-column:1/-1}canvas{display:block}a{color:#93c1e0}@media(max-width:1040px){.grid{grid-template-columns:480px}.wide{width:864px}}</style>
<h1>Souffle de feu · obstacles et Brûlure</h1><p>ANIM-OVER02 · 10 / FE02-A07 · B01 / BRULURE-A02 · corrections à examiner</p>
<p>Chaque obstacle coupe la portion du souffle qui le rencontre. L'aperçu cyan clignotant disparaît au lancement. Brûlure reste visible sur les personnages affectés après la fin du souffle : flammes principales derrière le corps et petites langues sur sa silhouette.</p>
<p>Le couloir long conserve la portée de cinq cases. Un second couloir est barré par une cloison. La dernière scène montre la Brûlure sur l'agent et sur un monstre, sans attaque ni dégâts alliés.</p>
<div class="controls"><select id="speed"><option value="1">Vitesse ×1</option><option value="0.5">Vitesse ×0,5</option></select><button id="pause">Pause</button><button id="replay">Rejouer</button><span id="loading">Chargement…</span></div>
<main class="grid">CARDS</main><p>Temps de montage uniquement : aucun tour de jeu n'avance. Dessins, icônes et paramètres de combat conservés. <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> depuis ce dossier.</p><script>SCRIPT</script></html>'''.replace('CARDS',''.join(cards)).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
