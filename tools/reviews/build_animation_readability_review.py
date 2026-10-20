"""Publish ANIM-READ-01 in the single current preview, from native engine captures."""
import json
from pathlib import Path

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/refonte-agent-2026-10-06/animation-readability-01"
CURRENT = OUT.parent / "apercu-animations.html"
DOC = ROOT / "docs/reviews/animation-readability-01"

HTML = r'''<!doctype html>
<html lang="fr"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Project RL · Rythme et lisibilité · ANIM-READ-01</title><link rel="icon" href="data:,">
<style>
:root{color-scheme:dark;font-family:system-ui,sans-serif;background:#0c1115;color:#e2e9ec}*{box-sizing:border-box}
body{max-width:1020px;margin:0 auto;padding:26px 22px 40px}h1{font-size:28px;margin:8px 0 12px;font-weight:650}p{line-height:1.55;margin:10px 0}.eyebrow{font-size:12px;letter-spacing:.13em;color:#91c9bb}.muted{color:#a2b2bc;font-size:14px}
.controls{display:flex;gap:12px;flex-wrap:wrap;align-items:end;margin:22px 0 16px}label{font-size:13px;color:#a8bbc6;display:grid;gap:7px}button,select{font:inherit;color:#e3eeef;background:#1a252d;border:1px solid #3b505a;border-radius:6px;padding:9px 12px;min-height:40px}button{cursor:pointer}button:hover{background:#273a43}input{accent-color:#85d3ba}.checks{display:flex;gap:22px;flex-wrap:wrap}.checks label{display:flex;align-items:center;gap:8px}
.native{overflow-x:auto;padding:4px 0 12px}.comparison{display:grid;grid-template-columns:448px 448px;gap:18px;width:914px}.pane h2{font-size:17px;margin:10px 0 7px}.pane.proposed h2{color:#a3e0c6}.canvas-frame{width:448px;height:320px;background:#101619;outline:1px solid #30434d}canvas{display:block;width:448px;height:320px;image-rendering:pixelated}.caption{font-size:13px;color:#a8bbc6;line-height:1.5;min-height:40px;margin:10px 0}
.timeline{display:flex;gap:12px;align-items:center}.timeline input{flex:1;min-width:120px}.timeline output{min-width:64px;text-align:right;color:#b8cad4;font-variant-numeric:tabular-nums}.notice{padding:14px 17px;border-left:3px solid #6aa58f;background:#131d24;margin-top:20px}.notice p{margin:2px 0}.error{color:#ffb3a8}a{color:#a4d5c5}footer{border-top:1px solid #2b3b43;margin-top:24px;padding-top:12px}
</style></head><body>
<div class="eyebrow">ANIM-READ-01 · PROPOSITION À VALIDER</div>
<h1>Un rythme plus posé, des effets plus lisibles</h1>
<p>Mêmes dessins, mêmes cases de <strong>64 px</strong>. À gauche le rendu actuel ; à droite une proposition qui laisse davantage de temps pour lire l’action et renforce certains effets discrets.</p>
<div class="controls">
<label>Animation<select id="case"></select></label>
<label>Environnement<select id="light"><option value="sombre">Sombre</option><option value="clair">Plus éclairé</option></select></label>
<button id="pause">Pause</button><button id="replay">Rejouer</button>
</div>
<div class="checks"><label><input type="checkbox" id="phase">Comparer le contraste à phase identique</label><label><input type="checkbox" id="contrast" checked>Appliquer le contraste proposé</label></div>
<p id="status" class="muted" role="status">Chargement des captures du moteur…</p>
<div class="native"><div class="comparison">
<section class="pane"><h2>Actuel</h2><div class="canvas-frame"><canvas id="before" width="448" height="320" aria-label="Animation actuelle, taille native"></canvas></div><p id="before-note" class="caption"></p></section>
<section class="pane proposed"><h2>Proposition</h2><div class="canvas-frame"><canvas id="after" width="448" height="320" aria-label="Animation proposée, taille native"></canvas></div><p id="after-note" class="caption"></p></section>
</div></div>
<div class="timeline"><label for="scrub">Revoir un instant</label><input id="scrub" type="range" min="0" max="2400" step="5" value="0"><output id="clock">0 ms</output></div>
<p class="muted">Lecture en boucle avec une pause entre les actions. Les images sont toujours affichées à leur taille de jeu ; sur une petite fenêtre, faire défiler la comparaison horizontalement.</p>
<div class="notice"><p id="rationale"></p></div>
<footer><p class="muted">Huit cas représentatifs capturés avec les fonctions de rendu du jeu, dans deux éclairages de revue contrôlés. Le renforcement porte seulement sur la différence créée par l’effet : le décor et les acteurs gardent leur rendu. Aucun nouveau dessin ni halo ajouté.</p>
<p class="muted">Cette galerie est isolée. Le lancement normal conserve les animations acceptées. La respiration, les menus et les effets persistants feront l’objet d’un contrôle distinct ; les durées en tours, dégâts, portées et sauvegardes ne sont pas modifiés.</p></footer>
<script>
'use strict';
const data=__DATA__,folder='./animation-readability-01/',el=id=>document.getElementById(id);
const contexts=[el('before').getContext('2d'),el('after').getContext('2d')];
contexts.forEach(c=>c.imageSmoothingEnabled=false);
for(const row of data.cases){const option=document.createElement('option');option.value=row.number;option.textContent=`${String(row.number).padStart(2,'0')} · ${row.title}`;el('case').append(option);}
let row=data.cases[0],sheets=null,playing=true,age=0,last=performance.now(),generation=0,period=2.4;
const explanations={A01:'Le trajet et l’impact occulte restent visibles plus longtemps. Le violet conserve sa couleur, avec un contraste plus soutenu.',OC05:'Le drain est très bref et discret. La proposition allonge sa lecture et renforce sa présence sur le sol sombre.',FR06:'Les éclats de glace se déploient plus lentement. Le renforcement du contraste reste léger pour préserver leurs détails clairs.',SM02:'Le nuage de poison se développe plus lentement, avec une présence verte plus nette sur les matières sombres.',B01:'Le mouvement de coupe et les éclats prennent davantage de temps ; la trace garde sa forme et sa portée.',FE05:'Le feu est déjà lumineux : son intensité reste identique. Seuls le trajet et le déploiement de l’explosion sont ralentis.',glide:'Un pas passe de 80 à 120 ms. Cette comparaison porte sur le glissement ; les commandes devront continuer à être acceptées sans attendre son arrivée.',reaction:'La cible revient plus lentement à sa position. L’amplitude du mouvement reste identique.'};
function notes(){const source=Math.round(row.seconds*1000),target=Math.round(source*row.duration_scale);el('before-note').textContent=`${source} ms · rythme et contraste actuels`;el('after-note').textContent=`${el('phase').checked?source:target} ms · ${el('contrast').checked&&row.contrast_gain>1?'contraste renforcé de '+Math.round((row.contrast_gain-1)*100)+' %':'intensité actuelle'}`;el('rationale').textContent=explanations[row.id];}
async function readImage(name){const img=new Image();img.src=folder+name;await img.decode();if(img.naturalWidth!==data.frame_width*data.columns||img.naturalHeight!==data.frame_height*Math.ceil(data.frames/data.columns))throw Error('Dimensions de planche incorrectes : '+name);return img;}
async function load(){const ticket=++generation;row=data.cases.find(r=>r.number===Number(el('case').value));const light=el('light').value,n=String(row.number).padStart(2,'0');sheets=null;el('status').className='muted';el('status').textContent='Chargement des captures du moteur…';notes();try{const images=await Promise.all([readImage(`${n}-${light}-0.png`),readImage(`${n}-${light}-1.png`)]);if(ticket!==generation)return;sheets=images;age=0;last=performance.now();el('status').textContent=`${n} · ${row.title} · ${light==='sombre'?'sol sombre':'sol plus éclairé'} · taille native 1:1`;draw();}catch(error){if(ticket!==generation)return;el('status').className='error';el('status').textContent='Aperçu indisponible : '+error.message;}}
function paint(context,img,time){const frame=Math.max(0,Math.min(data.frames-1,Math.floor(time*data.fps)));context.drawImage(img,(frame%data.columns)*data.frame_width,Math.floor(frame/data.columns)*data.frame_height,data.frame_width,data.frame_height,0,0,data.frame_width,data.frame_height);}
function draw(){if(!sheets)return;const scale=el('phase').checked?1:row.duration_scale;paint(contexts[0],sheets[0],age);paint(contexts[1],sheets[el('contrast').checked?1:0],age/scale);el('scrub').value=Math.round(age*1000);el('clock').value=Math.round(age*1000)+' ms';}
function tick(now){const dt=Math.min(.1,(now-last)/1000);last=now;if(playing&&sheets){age=(age+dt)%period;draw();}requestAnimationFrame(tick);}
el('case').onchange=load;el('light').onchange=load;
el('pause').onclick=()=>{playing=!playing;el('pause').textContent=playing?'Pause':'Lire';};
el('replay').onclick=()=>{age=0;playing=true;last=performance.now();el('pause').textContent='Pause';draw();};
el('scrub').oninput=()=>{playing=false;el('pause').textContent='Lire';age=Number(el('scrub').value)/1000;draw();};
for(const id of ['phase','contrast'])el(id).onchange=()=>{notes();draw();};
load();requestAnimationFrame(tick);
</script></body></html>'''


def main():
    data = json.loads((OUT / "capture.json").read_text(encoding="utf-8"))
    assert data["review"] == "ANIM-READ-01" and not data["normal_game_activation"]
    checks = []
    width, height = data["frame_width"], data["frame_height"]
    def phase(sheet, frame):
        x = frame % data["columns"] * width
        y = frame // data["columns"] * height
        return sheet.crop((x, y, x + width, y + height)).convert("RGB")

    for row in data["cases"]:
        for light in ("sombre", "clair"):
            paths = [OUT / f'{row["number"]:02d}-{light}-{panel}.png' for panel in range(2)]
            for path in paths:
                raw = path.with_suffix(".rgba")
                if raw.exists():
                    size = (width * data["columns"], height * ((data["frames"] + data["columns"] - 1) // data["columns"]))
                    payload = raw.read_bytes()
                    assert len(payload) == size[0] * size[1] * 4, raw
                    with Image.frombytes("RGBA", size, payload).transpose(Image.Transpose.FLIP_TOP_BOTTOM) as sheet:
                        sheet.save(path, compress_level=1)
            with Image.open(paths[0]) as before, Image.open(paths[1]) as after:
                expected = (width * data["columns"], height * ((data["frames"] + data["columns"] - 1) // data["columns"]))
                assert before.size == after.size == expected, paths
                rest = phase(before, data["frames"] - 1)
                assert ImageChops.difference(rest, phase(after, data["frames"] - 1)).getbbox() is None, (row["id"], light, "rest changed")
                animated, changed, untouched_errors = 0, 0, 0
                for frame in range(data["frames"]):
                    a, b = phase(before, frame), phase(after, frame)
                    diff = ImageChops.difference(a, b)
                    animated += ImageChops.difference(a, rest).getbbox() is not None
                    changed += diff.getbbox() is not None
                    # Outside the actual effect, a static production scene must
                    # match exactly in both columns, at every captured phase.
                    if row["contrast_gain"] > 1:
                        effect_channels = ImageChops.difference(a, rest).split()
                        effect_mask = ImageChops.lighter(ImageChops.lighter(effect_channels[0], effect_channels[1]), effect_channels[2])
                        unchanged_mask = effect_mask.point(lambda value: 255 if value == 0 else 0)
                        channels = diff.split()
                        changed_mask = ImageChops.lighter(ImageChops.lighter(channels[0], channels[1]), channels[2]).point(lambda value: 255 if value else 0)
                        untouched_errors += ImageChops.multiply(unchanged_mask, changed_mask).histogram()[255]
                assert animated > 0, (row["id"], light, "animation empty")
                assert (changed > 0) == (row["contrast_gain"] > 1), (row["id"], light, "contrast missing or unexpected")
                assert untouched_errors == 0, (row["id"], light, untouched_errors)
                checks.append({"id":row["id"],"light":light,"animated_phases":animated,"contrast_changed_phases":changed,"unchanged_scene_pixel_errors":untouched_errors,"rest_identical":True})
    CURRENT.write_text(HTML.replace("__DATA__", json.dumps(data, ensure_ascii=False)), encoding="utf-8")
    DOC.mkdir(parents=True, exist_ok=True)
    (DOC / "proposal.json").write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (OUT / "pixel-checks.json").write_text(json.dumps(checks, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for row in data["cases"]:
        for light in ("sombre", "clair"):
            for panel in range(2):
                raw = OUT / f'{row["number"]:02d}-{light}-{panel}.rgba'
                if raw.exists():
                    raw.unlink()
    print(CURRENT)


if __name__ == "__main__":
    main()
