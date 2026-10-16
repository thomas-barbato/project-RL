"""T01-C03: one transparent reticle reviewed in the native tree and target HUD."""
import hashlib
import json
from pathlib import Path

from PIL import Image
import prepare_agent_skill_animation_aim_recovery as encoder

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-icon-review-55'


def dump(name, data):
    (OUT / name).write_text(json.dumps(data, indent=2, ensure_ascii=False), encoding='utf8')


def package():
    assert not (OUT / 'visual-acceptance-01.json').exists()
    protected = json.loads((OUT / 'protected-current-bank.json').read_text(encoding='utf8'))
    assert all(hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest
               for name, digest in protected.items())
    for size in [24, 32, 64, 128, 192]:
        with Image.open(OUT / f'T01-C03-{size}.png') as im:
            assert im.mode == 'RGBA' and im.size == (size, size)
            assert im.getpixel((size // 2, size // 2))[3] == 0
            assert im.getchannel('A').getextrema() == (0, 255)
    assert Image.open(OUT / 'tree-native/cold-start.png').size == (960, 540)
    entries = json.loads((OUT / 'native/capture.json').read_text(encoding='utf8'))
    checked = []
    manifest = []
    animations = []
    encoder.FRAMES = 160
    selected = ['HAUT', 'DROITE', 'GAUCHE', 'BAS', 'ATTENTE', 'EXPIRATION', 'REMPLACEMENT']
    for e in entries:
        directory = OUT / 'native' / e['folder']
        timeline = json.loads((directory / 'timeline.json').read_text(encoding='utf8'))
        assert e['variant'].startswith('T01-A03-')
        assert all(t['overhead_icon_visible'] == (t['remaining'] > 0) for t in timeline)
        if not e['permitted']:
            assert all(not t['overhead_icon_visible'] for t in timeline)
        elif e['case'] in ['HAUT', 'DROITE', 'GAUCHE', 'BAS', 'MANQUE', 'RAFALE']:
            assert timeline[44]['overhead_icon_visible'] and not timeline[48]['overhead_icon_visible']
        elif e['case'] == 'ATTENTE':
            assert timeline[-1]['overhead_icon_visible'] and timeline[-1]['remaining'] == 6
        elif e['case'] == 'EXPIRATION':
            assert timeline[128]['overhead_icon_visible'] and not timeline[148]['overhead_icon_visible']
        elif e['case'] == 'REMPLACEMENT':
            assert timeline[88]['marked_target'] == [6, 2] and timeline[88]['remaining'] == 6
        checked.append({'variant': e['variant'], 'status_lifetime_checked': True})
        if e['case'] not in selected:
            continue
        frames = []
        canonical = {}
        indices = []
        for n in range(160):
            with Image.open(directory / f'{n:03}.png') as im:
                image = im.convert('RGB')
            digest = hashlib.sha256(image.tobytes()).hexdigest()
            canonical.setdefault(digest, n)
            indices.append(canonical[digest])
            if e['case'] in selected[:4]:
                frames.append(image.crop((32, 88, 608, 664)))
        if frames:
            animations.append(encoder.gif(OUT / f"{e['variant']}-64.gif", frames))
        manifest.append({**e, 'timeline': timeline, 'indices': indices})
    Image.open(OUT / 'native/T01-HAUT/044.png').save(OUT / 'target-native.png')
    dump('review.json', {
        'date': '2026-10-14', 'candidate': 'T01-C03', 'skill': 'T01',
        'name': 'Marquage balistique', 'visual_status': 'awaiting_visual_review',
        'requested_change': 'Only the red reticle on transparent background, in both skill tree and target marking.',
        'shared_preview_source': 'T01-C03-64.png',
        'accepted_bank_changed': False, 'campaign_integration': False,
        'scope': 'Icon revision only; prior mechanics and other drawings preserved.',
        'generation_tool': 'Built-in imagegen', 'prompt': 'T01-C03-prompt.txt',
        'postprocessing': 'Direct Lanczos reductions of generated RGBA; no repaint or recolor.',
    })
    dump('verification.json', {
        'protected_bank_files_unchanged': len(protected),
        'center_transparent_at_all_sizes': True, 'native_tree_capture_size': [960, 540],
        'native_world_tile_size': 64, 'native_capture_resampled': False,
        'mark_lifetimes': checked, 'animations': animations,
    })
    write_html(manifest)
    (OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
    for name in ['skill_art_review.rs', 'agent_skill_window.rs']:
        (OUT / name).write_bytes((ROOT / 'src' / name).read_bytes())
    (OUT / 'target-render-source.rs').write_bytes(
        (ROOT / 'src/agent_skill_lab_review/fire_painted/shot_buffs.rs').read_bytes())
    print(json.dumps({'candidate': 'T01-C03', 'protected': len(protected), 'views': len(entries)}))


def write_html(manifest):
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>T01-C03 · Marquage balistique</title><style>
*{box-sizing:border-box}body{margin:0;background:#0a0e11;color:#e1e7e9;font:16px system-ui}main{max-width:1100px;margin:auto;padding:28px}h1{font-size:26px;margin:0 0 10px}h2{font-size:20px;margin-top:28px}p{color:#adbdc5;line-height:1.5}a{color:#c0c9ff}.icons{display:flex;gap:28px;align-items:center;flex-wrap:wrap}.icons figure{margin:0;text-align:center}.icons img{display:block;background:#161c22;margin:10px auto}.icons figcaption{font-size:13px;color:#adbdc5}.native{display:block;max-width:100%;height:auto}canvas{background:#080c0e;display:block;max-width:100%;height:auto}select,button{font:inherit;color:#e1e7e9;background:#20272f;border:1px solid #53616e;border-radius:4px;padding:8px 12px}button:disabled{opacity:.4}.controls{display:flex;gap:10px;flex-wrap:wrap;margin:16px 0}#status{min-height:26px}.stage{overflow:auto}.note{font-size:13px}figure img{object-fit:contain}
</style><main><h1>T01-C03 · Marquage balistique</h1>
<p>Le viseur rouge seul, sur fond transparent. Le même dessin est utilisé dans l’arbre et au-dessus de la cible.</p>
<div class="icons"><figure><img src="T01-C03-24.png" width="24" height="24"><figcaption>24 px · cible</figcaption></figure><figure><img src="T01-C03-32.png" width="32" height="32"><figcaption>32 px</figcaption></figure><figure><img src="T01-C03-64.png" width="64" height="64"><figcaption>64 px · arbre</figcaption></figure><figure><img src="T01-C03-128.png" width="128" height="128"><figcaption>Détail · 128 px</figcaption></figure></div>
<h2>Dans l’arbre des compétences</h2><img class="native" src="tree-native/cold-start.png" width="960" height="540" alt="Fenêtre native : nouveau viseur de Marquage balistique dans Buffs à distance">
<h2>Au-dessus de la cible · cases de 64 px</h2>
<div class="controls"><select id="scenario" aria-label="Scénario"><option value="HAUT">Haut</option><option value="DROITE">Droite</option><option value="GAUCHE">Gauche</option><option value="BAS">Bas</option><option value="ATTENTE">Marque en attente</option><option value="EXPIRATION">Expiration</option><option value="REMPLACEMENT">Changer de cible</option></select><button id="cast">Lancer le marquage</button><button id="next">Action suivante</button><button id="reset">Recommencer</button></div>
<p id="status">Chargement…</p><div class="stage"><canvas id="world" width="640" height="720" aria-label="Aperçu animé du marquage"></canvas></div>
<p class="note">L’aperçu attend vos actions pour faire avancer les tours. Les valeurs et les cibles du laboratoire sont préparées. La fenêtre ci-dessus utilise un personnage de rang 1 ; le laboratoire présente le rang 11. Cette revue modifie uniquement le dessin.</p>
<p class="note">Proposition visuelle à valider. Créée avec imagegen · <a href="T01-C03-prompt.txt">consigne de génération</a> · <a href="T01-C03-master.png">source transparente</a> · <a href="target-native.png">capture de la cible</a>.</p></main>
<script>const manifest=MANIFEST;const scenario=document.getElementById('scenario'),cast=document.getElementById('cast'),next=document.getElementById('next'),reset=document.getElementById('reset'),status=document.getElementById('status'),canvas=document.getElementById('world');let ready=false,last=0,elapsed=0,launched=false,turn=0,token=0,images=[],entry;
function frame(){if(!launched)return 3+Math.floor((elapsed%600)*12/600);const step=Math.floor(elapsed*24/1000),base=[17,48,68,88,108,128,148][turn],length=turn===0?27:turn===6?8:16;return step<length?base+step:base+length+step%4;}
function ui(){cast.disabled=!ready||launched;next.disabled=!ready||!launched||entry.quiet||turn>=6;if(ready){const t=entry.timeline[frame()];status.textContent=launched?'Marque : '+t.remaining+' tours · recharge : '+t.cooldown+' tours':'Prévisualisation · confirmer pour lancer';}}
async function load(){ready=false;entry=manifest.find(e=>e.case===scenario.value);ui();status.textContent='Chargement…';const n=++token,cache=new Map();const data=await Promise.all(entry.indices.map(i=>{const src='native/'+entry.folder+'/'+String(i).padStart(3,'0')+'.png';if(!cache.has(src))cache.set(src,new Promise((resolve,reject)=>{const im=new Image();im.onload=()=>resolve(im);im.onerror=reject;im.src=src;}));return cache.get(src);}));if(n!==token)return;images=data;elapsed=0;turn=0;launched=false;ready=true;ui();}
scenario.onchange=load;cast.onclick=()=>{launched=true;elapsed=0;turn=0;ui();};next.onclick=()=>{turn++;elapsed=0;ui();};reset.onclick=()=>{launched=false;elapsed=0;turn=0;ui();};function tick(now){if(last&&ready)elapsed+=now-last;last=now;if(ready){ui();canvas.getContext('2d').drawImage(images[frame()],0,0);}requestAnimationFrame(tick);}load().catch(e=>status.textContent='Chargement impossible : '+e);requestAnimationFrame(tick);</script></html>'''
    (OUT / 'index.html').write_text(page.replace('MANIFEST', json.dumps(manifest, ensure_ascii=False)), encoding='utf8')


if __name__ == '__main__':
    package()
