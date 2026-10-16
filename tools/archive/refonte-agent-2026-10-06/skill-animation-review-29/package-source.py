"""ANIM-S10: arcane bolt and occult seal blast, native isolated visual review."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image,ImageChops,ImageDraw
import prepare_agent_skill_animation_review as base
import prepare_agent_skill_animation_gel_foreground as previous
from prepare_agent_skill_animation_revision import save_animation

OUT=base.ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-29'
GENERATED=Path(r'C:\Users\User\.codex\generated_images\01a11010-433f-7fc2-ac29-ffb696dec5ea')
SOURCES={'bolt':'exec-793f6069-6b59-401b-bc7b-9705ca8d8bb8.png','burst':'exec-9828ca28-d673-438e-b27e-567616de758c.png'}


def protected():
    hashes=previous.protected()
    for p in sorted(previous.OUT.rglob('*')):
        if p.is_file():hashes[p.relative_to(base.ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    assert not (OUT/'review-protection.json').exists()
    base.dump(OUT/'review-protection.json',protected())
    packing=[]
    for kind,name in SOURCES.items():
        source=GENERATED/name
        (OUT/f'{kind}-source.png').write_bytes(source.read_bytes())
        with Image.open(source) as image:
            assert image.mode=='RGBA' and image.size==(1536,1024)
            assert image.getchannel('A').getextrema()[0]==0
            atlas=Image.new('RGBA',(1536,256))
            for i in range(6):
                x,y=i%3*512,i//3*512
                atlas.paste(image.crop((x,y,x+512,y+512)).resize((256,256),Image.Resampling.LANCZOS),(i*256,0))
            atlas.save(OUT/f'{kind}-atlas.png')
        packing.append({'kind':kind,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'source_size':[1536,1024],'layout':[3,2],'cell_size':[256,256],'alpha_preserved':True})
    base.dump(OUT/'drawing-pack.json',{'mode':'Built-in imagegen','packing':packing,'postprocessing':'Equal-cell cropping and uniform resize only; no repainting.'})
    base.dump(OUT/'review.json',{'batch':'ANIM-S10','status':'awaiting_visual_review','native_tile_size':64,
        'candidates':[{'number':n,'id':sid,'variant':sid+'-A01','name':title,'status':'awaiting_visual_review'}
            for n,sid,title in [(21,'A01','Trait arcanique'),(22,'A02','Déflagration occulte')]],
        'bolt':'Short violet dart with dark filaments, .18-second cosmetic flight and .22-second black-knot impact. No added status or piercing/interception rule.',
        'blast':'Blood-colored ground seal ruptures into angular black/violet fragments. Six drawings played once over .55 seconds, no remaining field or blood injury status.',
        'timing':'Cosmetic motion does not advance turns, recharges or repeat prepared contacts. Both impacts disappear without waiting for a turn.',
        'geometry':'Prepared target and 3x3 zone use current catalogue ranges. Physical walls cut drawings; a full cloison refuses the hidden bolt target and keeps the blast on the accessible side. The close case places the agent in its own zone without contact. Diagonal/final geometry remains separate.',
        'scope':'Isolated debug fixture, no damage, RNG, real FOV/AI, saves or campaign integration. Previous rules, catalogue, approved frost/fire and actors are preserved.'})
    launch=(previous.OUT/'Ouvrir_laboratoire.ps1').read_text(encoding='utf-8-sig').replace("'--lot-gel-statut' '--gel-status-v2' '--gel-status-front'","'--lot-occulte-impacts'")
    (OUT/'Ouvrir_laboratoire.ps1').write_text(launch,encoding='utf-8-sig')
    print(json.dumps({'prepared':str(OUT),'atlases':2}))


def convert():
    native=(OUT/'native').resolve()
    count=0
    for raw in native.rglob('*.rgba'):
        assert raw.resolve().is_relative_to(native)
        data=raw.read_bytes()
        assert len(data)==1280*720*4
        png=raw.with_suffix('.png')
        Image.frombytes('RGBA',(1280,720),data).save(png,compress_level=1)
        with Image.open(png) as image:assert image.tobytes()==data
        raw.unlink()  # Verified temporary readback, only inside this review.
        count+=1
    if count:base.dump(OUT/'native-encoding-verification.json',{'converted_frames':count,'pixel_identical_to_native_readback':True,'resampling':False})
    else:assert (OUT/'native-encoding-verification.json').exists()


def package():
    baseline=json.loads((OUT/'review-protection.json').read_text(encoding='utf-8'))
    assert protected()==baseline
    convert()
    entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf-8'))
    assert len(entries)==7
    checks,animations,sequences=[],[],{}
    for e in entries:
        folder=OUT/'native'/e['folder']
        timeline=json.loads((folder/'timeline.json').read_text(encoding='utf-8'))
        for index,v in enumerate(e['views']):
            target=OUT/'frames'/v['variant'];target.mkdir(parents=True,exist_ok=True)
            x,y,w,h=v['crop'];bounds=(x,y,x+w,y+h)
            frames=[]
            for i in range(e['frames']):
                with Image.open(folder/f'{i:03}.png') as image:frame=image.crop(bounds).convert('RGB')
                frame.save(target/f'{i:03}.png',compress_level=1);frames.append(frame)
            diffs={}
            for p in e['control_frames']:
                with Image.open(folder/f'control-{p:03}.png') as image:diffs[p]=ImageChops.difference(frames[p],image.crop(bounds).convert('RGB'))
                for cy,row in enumerate(e['map'].splitlines()):
                    for cx,symbol in enumerate(row):
                        if symbol=='#':assert diffs[p].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None,(v['variant'],p,cx,cy)
            rows=[r['states'][index] for r in timeline]
            expected=v['expected_contacts']
            initial_cd=(2 if index==0 else 4) if v['valid'] else 0
            assert all(r['contacts']==expected and r['cooldown']==initial_cd and r['turn']==0 for r in rows[17:48])
            assert all(r['player_contacts']==0 for r in rows)
            assert all(r['contacts']==expected for r in rows[17:])
            assert [rows[p]['turn'] for p in e['turn_frames']]==[1,2,3,4]
            assert [rows[p]['cooldown'] for p in e['turn_frames']]==[max(0,initial_cd-n) for n in (1,2,3,4)]
            assert all(diffs[p].getbbox() is None for p in (35,47,50,66,83,100,111)), 'Instant effect persisted.'
            if v['valid']:
                assert any(diffs[p].getbbox() for p in (19,21,23,26,29)), 'Effect invisible.'
                cx,cy=v['affected_cells'][0];sample=(cx*64+3,cy*64+25)
                assert frames[3].getpixel(sample)[1]>frames[14].getpixel(sample)[1]+25, 'Preview is static.'
                assert not(rows[35]['visual_active'])
            else:
                assert not any(r['launched'] for r in rows) and not any(r['visual_active'] for r in rows)
                assert all(d.getbbox() is None for d in diffs.values())
            if e['folder']=='CLOISON':
                for at in v['target_cells'][:5]:
                    cx,cy=at
                    assert all(diffs[p].crop((cx*64,cy*64,(cx+1)*64,(cy+1)*64)).getbbox() is None for p in e['control_frames'])
            sequences[v['variant']]=frames
            if not v['valid']:
                # A refused bolt deliberately has no animation. Preserve the
                # earlier static export separately instead of treating it as VFX.
                stale=OUT/f"{v['variant']}-64.gif"
                if stale.exists():
                    archive=OUT/f"{v['variant']}-static-preview.gif"
                    assert stale.resolve().is_relative_to(OUT.resolve()) and archive.resolve().is_relative_to(OUT.resolve())
                    assert not archive.exists()
                    stale.rename(archive)
            if v['valid'] and e['folder'] in ('HAUT','OBSTACLE','CLOISON','PROCHE'):
                animations.append(save_animation(OUT/f"{v['variant']}-64.gif",frames,e['fps']))
            checks.append({'variant':v['variant'],'solid_walls_clear':True,'preview_blinks_and_ends':True,'contacts_once_per_launch':True,
                'timers_turn_driven':True,'blocked_bolt_refused':not v['valid'],'agent_not_hurt_by_own_zone':True,'instant_effect_ends':True})
        if e['folder']=='HAUT':
            for p in (14,21,23,29,35):
                with Image.open(folder/f'{p:03}.png') as image:image.save(OUT/f'phase-{p:03}-native.png')
    for folder in ('HAUT','CLOISON','PROCHE'):
        boards=[]
        for i in range(112):
            board=Image.new('RGB',(1232,650),'#0b1218');draw=ImageDraw.Draw(board)
            for j,(sid,title) in enumerate([('A01','21 · Trait arcanique'),('A02','22 · Déflagration occulte')]):
                left=12+j*616
                draw.text((left,10),title,font=base.font(20,True),fill='#e0e7e7')
                draw.text((left,40),f'{sid}-A01 · {folder}',font=base.font(15),fill='#a5b9c5')
                board.paste(sequences[f'{sid}-A01-{folder}'][i],(left,66))
            boards.append(board)
        animations.append(save_animation(OUT/f'occulte-{folder.lower()}-64.gif',boards,24))
    base.dump(OUT/'verification.json',{'protected_files_unchanged':len(baseline),'controls':checks,'animations':animations,'resampling_of_native_capture':False})
    for source,name in [('src/agent_skill_lab_review/fire_painted/occult.rs','render-source.rs'),('src/agent_skill_lab_review/fire_painted/zones.rs','clip-source.rs'),('src/agent_skill_lab_review.rs','render-parent-source.rs')]:
        (OUT/name).write_bytes((base.ROOT/source).read_bytes())
    (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
    write_html(entries)
    print(json.dumps({'native_views':14,'animations':len(animations),'protected_files':len(baseline),'controls':'passed'}))


def write_html(entries):
    manifest=[{'folder':e['folder'],'views':[{k:v[k] for k in ('id','variant','valid')} for v in e['views']]} for e in entries]
    script='''const manifest=MANIFEST;let ready=false,last=0,elapsed=0,paused=false,launched=true,turn=0,token=0,sequences;
const scenario=document.getElementById('scenario'),next=document.getElementById('next'),cast=document.getElementById('cast'),starts=[17,48,65,82,99],ends=[47,64,81,98,111];
function entry(){return manifest.find(e=>e.folder===scenario.value);}
function frame(){const f=Math.floor(elapsed*24/1000);return launched?Math.min(starts[turn]+f,ends[turn]):f%17;}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||turn>=4||(turn===0&&elapsed<600);document.getElementById('status').textContent=launched?`Tour ${turn} · recharge Trait ${entry().views[0].valid?Math.max(0,2-turn):0} / Déflagration ${Math.max(0,4-turn)}`:'Aperçu clignotant · avant lancement';}
async function load(){ready=false;ui();const n=++token,e=entry();const loaded=await Promise.all(e.views.map(async v=>[v.id,await Promise.all(Array.from({length:112},(_,i)=>new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=`frames/${v.variant}/${String(i).padStart(3,'0')}.png`;})))]));if(n!==token)return;sequences=new Map(loaded);elapsed=0;ready=true;ui();}
function tick(now){if(last&&ready&&!paused)elapsed+=now-last;last=now;if(ready){ui();for(const canvas of document.querySelectorAll('canvas'))canvas.getContext('2d').drawImage(sequences.get(canvas.dataset.skill)[frame()],0,0);}requestAnimationFrame(tick);}
next.onclick=()=>{if(!next.disabled){turn++;elapsed=0;ui();}};cast.onclick=()=>{if(!cast.disabled){launched=true;turn=0;elapsed=0;ui();}};document.getElementById('reset').onclick=()=>{launched=false;turn=0;elapsed=0;ui();};scenario.onchange=()=>{launched=true;turn=0;elapsed=0;load().catch(()=>document.getElementById('status').textContent='Chargement impossible.');};document.getElementById('pause').onclick=()=>{paused=!paused;document.getElementById('pause').textContent=paused?'Lecture':'Pause';};load();requestAnimationFrame(tick);'''.replace('MANIFEST',json.dumps(manifest))
    html='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Project RL · Impacts occultes</title><style>*{box-sizing:border-box}body{margin:0 auto;padding:24px;max-width:1320px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}h2{font-size:20px}p{line-height:1.5}.muted{color:#a5b9c5}.controls{display:flex;gap:12px;flex-wrap:wrap;margin:24px 0}button,select{background:#17232d;color:#e0e8ed;border:1px solid #405664;border-radius:5px;padding:10px 14px;font:inherit;cursor:pointer}button:disabled{opacity:.4;cursor:default}.pair{display:grid;grid-template-columns:repeat(2,608px);gap:16px}article{padding:15px;background:#101b23;border:1px solid #273e4c;border-radius:8px}canvas{display:block;width:576px;height:576px}a{color:#93c1e0}@media(max-width:1296px){.pair{grid-template-columns:608px}}</style>
<h1>Magie occulte · Trait et déflagration</h1><p class="muted">ANIM-S10 · deux rendus à examiner en cases natives 64 px</p><div class="controls"><select id="scenario"><option value="HAUT">Vers le haut</option><option value="DROITE">Vers la droite</option><option value="GAUCHE">Vers la gauche</option><option value="BAS">Vers le bas</option><option value="OBSTACLE">Obstacle partiel</option><option value="CLOISON">Cloison pleine</option><option value="PROCHE">Agent proche · dans sa propre zone</option></select><button id="next">+1 tour</button><button id="cast">Lancer</button><button id="reset">Recommencer l’aperçu</button><button id="pause">Pause</button></div><p id="status"></p><main class="pair"><article><h2>21 · Trait arcanique</h2><p class="muted">A01-A01 · trait violet, nœud noir à l’impact</p><canvas data-skill="A01" width="576" height="576"></canvas></article><article><h2>22 · Déflagration occulte</h2><p class="muted">A02-A01 · sceau et fragments occultes</p><canvas data-skill="A02" width="576" height="576"></canvas></article></main>
<p>Le Trait atteint sa cible avec un trajet visuel de 0,18 seconde, puis un impact de 0,22 seconde. La Déflagration brise un sceau au sol et projette ses fragments sur 0,55 seconde. Les deux effets se terminent seuls ; aucune zone ni nouveau statut ne reste au sol.</p><p>Les prévisualisations clignotent avant le lancement et disparaissent à la confirmation. La cloison refuse la cible inaccessible du Trait ; la Déflagration peut viser la case libre devant elle, puis le mur coupe son dessin. Dans l’exemple proche, l’agent est dans sa propre zone sans subir son sort.</p><p>Les recharges diminuent uniquement avec « +1 tour ». Attendre ou lire les dessins ne répète aucun contact. Les quatre orientations gardent les personnages de face.</p><p class="muted">Prototype isolé : positions, visibilité et contacts préparés. Aucun dégât, hasard, IA réelle, sauvegarde ou changement de campagne. La géométrie finale des diagonales et grandes cibles reste distincte. Le rouge appartient au sceau ; aucun Saignement n’est ajouté. Ces nouveaux rendus attendent leur validation.</p><p><a href="occulte-haut-64.gif">Séquence des deux effets</a> · <a href="occulte-cloison-64.gif">Cloison</a> · <a href="occulte-proche-64.gif">Agent dans la zone</a> · <a href="Ouvrir_laboratoire.ps1">Laboratoire natif</a></p><p class="muted">Dessins créés avec imagegen intégré : <a href="bolt-source.png">source Trait</a> / <a href="imagegen-bolt-prompt.txt">prompt</a> · <a href="burst-source.png">source Déflagration</a> / <a href="imagegen-burst-prompt.txt">prompt</a>.</p><script>SCRIPT</script></html>'''.replace('SCRIPT',script)
    (OUT/'index.html').write_text(html,encoding='utf-8');(OUT/'gallery-script.js').write_text(script,encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');args=parser.parse_args()
    prepare() if args.prepare else package()
