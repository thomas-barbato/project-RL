"""ANIM-S05: native illustrated Mur de feu and Émanation, with obstacles."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_opening as previous
from prepare_agent_skill_animation_revision import save_animation

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-13'
CANDIDATES=[
    (11,'FE03','FE03-A03-LIBRE','Mur de feu · zone ouverte','Un rideau de flammes traversable ; A en sort et conserve sa Brûlure.'),
    (12,'FE04','FE04-A03-LIBRE','Émanation soudaine · zone ouverte','Les fissures annoncent la préparation, puis les gerbes jaillissent après un tour.'),
    (11,'FE03','FE03-A03-CLOISON','Mur de feu · cloison','La ligne et ses flammes sont interrompues par le décor. C reste protégé derrière la cloison.'),
    (12,'FE04','FE04-A03-CLOISON','Émanation soudaine · cloison','Les dessins peuvent dépasser les cases libres ; ils ne traversent pas la cloison. F reste protégé.'),
]

def protected():
    hashes=previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():hashes[p.relative_to(base.ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes

def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists(),'Preserve the baseline.'
    base.dump(OUT/'review-protection.json',protected())
    base.dump(OUT/'review.json',{
        'batch':'ANIM-S05','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'skill_id':sid,'variant':v,'name':name,'description':desc,'status':'awaiting_visual_review'} for n,sid,v,name,desc in CANDIDATES],
        'style':'Accepted ANIM-OVER03 policy. Exact original painted plume atlas; no new images, recolouring or actor modifications.',
        'wall':'Rank-11 example uses the existing seven-cell maximum and five-turn duration. Dense half-cell visual spacing joins drawn plumes into one curtain. A moves across the wall on the first scripted turn; attached Burn follows its body after exit. No displacement blocking.',
        'eruption':'Same radius-one cross fixture as the earlier review. Targeting vanishes on launch, cracks remain as the actual preparation marker, and the burst requires one explicit +1-turn event. Large central and smaller surrounding plumes are visual emphasis only; no damage values changed.',
        'obstacles':'Physical wall cells are covered and disconnected floor behind the full cloison is excluded from the zone drawing. The mask does not follow gameplay-zone borders on free floor.',
        'burn':'Accepted back/body flame layers attached to targets, with staged successful applications. Real chance, damage, resistance and status interactions are not calculated here.',
        'timing':'Five-second montage at 24 capture fps. Launch at 0.7s; explicit scripted turn events at 1.35/1.90/2.40/2.85/3.30/3.80/4.30s. Montage seconds do not become game turns automatically in the native laboratory.',
        'lab':'Manual turns, disabled launch during cooldown, replay/pause/speed, four aiming directions and effective rank controls. LAB-S05 awaits separate review.',
        'scope':'Debug visual fixture only. No campaign mechanics, attributes, loot, saves, skill descriptions, balance or icon changes.',
        'blood':'No blood in these pure Fire effects, following ANIM-BLOOD01.',
    })
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig')
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch.replace('--lot-souffle-ouverture','--lot-feu-zones'),encoding='utf-8-sig')

def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline,'Previous accepted content changed.'
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert [e['variant'] for e in entries]==[v for _,_,v,_,_ in CANDIDATES]
    sequences={};checks=[];controls=[]
    for e in entries:
        assert e['tile']==64 and e['frames']==120 and e['fps']==24
        x,y,w,h=e['crop'];folder=OUT/'native'/e['folder'];target=OUT/'frames'/e['variant'];target.mkdir(parents=True,exist_ok=True)
        frames=[]
        for i in range(120):
            with Image.open(folder/f'{i:03}.png') as im:
                assert im.size==(1280,720)
                crop=im.crop((x,y,x+w,y+h)).convert('RGB');crop.save(target/f'{i:03}.png');frames.append(crop)
        diffs={}
        for phase in e['control_frames']:
            with Image.open(folder/f'control-{phase:03}.png') as im:
                diffs[phase]=ImageChops.difference(frames[phase],im.crop((x,y,x+w,y+h)).convert('RGB'))
        for phase in (24,40,84):
            for cy,row in enumerate(e['map'].splitlines()):
                for cx,symbol in enumerate(row):
                    if symbol=='#':assert diffs[phase].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,f'Fire on physical wall: {e["variant"]}'
        assert diffs[119].getbbox() is None,'Effects did not expire with the scripted turns.'
        assert diffs[24].getbbox() and diffs[40].getbbox(),'Main effect or preparation missing.'
        for actor in e['targets']:
            if not actor['affected']:
                cx,cy=actor['cell'];bounds=(cx*64,cy*64,(cx+1)*64,(cy+1)*64)
                assert all(diffs[i].crop(bounds).getbbox() is None for i in (24,40,84)),'Protected target is engulfed.'
        cx,cy=e['affected_cells'][0];sample=(cx*64+3,cy*64+25)
        bright=frames[3].getpixel(sample);dim=frames[14].getpixel(sample)
        assert bright[1]>dim[1]+30 and dim[1]>50,'Preview blink missing.'
        for phase in (17,24,40):
            r,g,b=frames[phase].getpixel(sample)
            assert not (r<100 and g>140 and b>140),'Targeting survives launch.'
        if e['id']=='FE03':
            dx,dy=e['moving_target_destination'];leaving=(dx*64,dy*64,(dx+1)*64,(dy+1)*64)
            assert diffs[40].crop(leaving).getbbox(),'Burn does not follow A as it exits.'
            assert diffs[84].getbbox(),'Burn does not persist after the wall ends.'
        controls.append({'variant':e['variant'],'walls_and_protected_targets_unchanged':True,'preview_before_cast_only':True,'explicit_turn_lifecycle':True,'burn_attached_after_exit':e['id']=='FE03'})
        sequences[e['variant']]=frames;checks.append(save_animation(OUT/f"{e['variant']}-64.gif",frames,24))
    finish(entries,sequences,checks,controls,len(baseline))

def finish(entries,sequences,checks,controls,protected_count):
    for case in ('LIBRE','CLOISON'):
        candidates=[e for e in entries if e['folder']==case];boards=[]
        for i in range(120):
            board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
            for column,e in enumerate(candidates):
                left=12+column*616
                draw.text((left,10),f'{e["number"]} · {e["title"]}',font=base.font(20,True),fill='#e0e7e7')
                draw.text((left,40),e['variant'],font=base.font(14),fill='#9db3bd')
                board.paste(sequences[e['variant']][i],(left,66))
            boards.append(board)
        stem='zones-ouvertes' if case=='LIBRE' else 'zones-cloison'
        checks.append(save_animation(OUT/f'{stem}-64.gif',boards,24))
        boards[24].save(OUT/f'{stem}-preparation.png');boards[40].save(OUT/f'{stem}-eruption.png');boards[84].save(OUT/f'{stem}-brulure.png')
    base.dump(OUT/'verification.json',{'status':'awaiting_visual_review','protected_files_unchanged':protected_count,'native_capture':'native/capture.json','resampling':False,'controls':controls,'animations':checks})
    for source,name in [('src/agent_skill_lab_review/fire_painted/zones.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted.rs','painted-source.rs'),('src/agent_skill_lab_review/fire_painted/obstacles/overlay/stopped.rs','burn-layer-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'views':len(entries),'animations':len(checks),'protected_files_unchanged':protected_count,'controls':controls}))

def write_html(entries):
    cards=[]
    for n,sid,variant,title,desc in CANDIDATES:
        cards.append(f'<article><h2>{n} · {title}</h2><p class="muted">{variant} · à valider</p><p>{desc}</p><canvas data-variant="{variant}" width="576" height="576"></canvas></article>')
    manifest=[dict(e,folder=e['variant']) for e in entries]
    script='''const manifest=MANIFEST;const sequences=new Map();let elapsed=0,last=0,paused=false,ready=false;
const speed=document.getElementById('speed'),pause=document.getElementById('pause');
async function load(){for(const e of manifest){const images=await Promise.all(Array.from({length:e.frames},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${e.folder}/${String(i).padStart(3,'0')}.png`;})));sequences.set(e.variant,{e,images});}elapsed=0;ready=true;document.getElementById('loading').textContent='Prêt · cases natives 64 px';}
function tick(now){if(last&&!paused)elapsed+=(now-last)*Number(speed.value);last=now;if(ready)for(const c of document.querySelectorAll('canvas')){const{e,images}=sequences.get(c.dataset.variant);c.getContext('2d').drawImage(images[Math.floor(elapsed*e.fps/1000)%e.frames],0,0);}requestAnimationFrame(tick);}
pause.onclick=()=>{paused=!paused;pause.textContent=paused?'Lecture':'Pause';};document.getElementById('replay').onclick=()=>{elapsed=0;};load().catch(()=>document.getElementById('loading').textContent='Conserver les sous-dossiers avec la galerie.');requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Mur de feu et Émanation</title>
<style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:19px}p{line-height:1.5}.muted{color:#9eb1bc}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:9px 13px;font:inherit;cursor:pointer}.grid{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.grid{grid-template-columns:608px}}</style>
<h1>Mur de feu et Émanation soudaine</h1><p class="muted">ANIM-S05 · 11 / FE03-A03 · 12 / FE04-A03 · quatre aperçus à valider</p>
<p>Les flammes reprennent les dessins du rendu validé, avec débordement sur le sol libre et arrêt devant la cloison. L'aperçu cyan clignotant disparaît au lancement. La Brûlure reste attachée aux corps.</p>
<p>Mur de feu reste traversable : A en sort. Émanation conserve ses fissures pendant la préparation et éclate après un tour complet.</p>
<div class="controls"><select id="speed"><option value="1">Vitesse ×1</option><option value="0.5">Vitesse ×0,5</option></select><button id="pause">Pause</button><button id="replay">Rejouer</button><span id="loading">Chargement…</span></div>
<main class="grid">CARDS</main><p class="muted">Les passages de tours sont scénarisés dans ces montages de cinq secondes. Dans le laboratoire natif, +1 tour les avance manuellement ; lecture, pause et ralenti n'avancent aucun tour. Les réussites de Brûlure sont préparées pour voir l'effet, sans calcul de dégâts ou tirage aléatoire.</p>
<p><a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a> depuis ce dossier : direction, rang d'aperçu, lancement, tours, recharge et replay. Les règles de combat et dessins précédemment acceptés sont conservés ; aucune intégration à la campagne par cette revue.</p><script>SCRIPT</script></html>'''.replace('CARDS',''.join(cards)).replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
