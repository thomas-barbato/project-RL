"""Verify real GPU lighting/animation frames and publish a lossless native loop."""
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageChops
from show_camp_reference_r05 import sha, verify_actors

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/handcrafted-places-review-01'
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def main():
    c = json.loads((ROOT/'assets/prototypes/handcrafted-camp-01/r09/camp.json').read_text(encoding='utf-8'))
    base = json.loads((ROOT/'assets/prototypes/handcrafted-camp-01/r08/camp.json').read_text(encoding='utf-8'))
    assert c['actors']==base['actors'] and c['props']==base['props'] and c['layout']==base['layout']
    assert c['floor']==base['floor'] and c['structure']==base['structure']
    capture = OUT/'engine-r09'
    runtime = json.loads((capture/'full-engine-checks-r09.json').read_text(encoding='utf-8'))
    assert runtime['computed_lighting'] and runtime['ambient_animations']
    assert not (capture/'stderr.log').read_text(encoding='utf-8').strip()
    raw = capture/'r09-reference-native.png'
    exact = ImageChops.difference(Image.open(raw).convert('RGB'),Image.open(OUT/'engine-r08/r08-camp-native.png').convert('RGB')).getbbox() is None
    assert exact
    actors = verify_actors(raw,c,[2,8])
    on = Image.open(capture/'r09-camp-native.png').convert('RGB')
    off = Image.open(capture/'r09-lights-off-native.png').convert('RGB')
    d = np.asarray(on).astype(int)-np.asarray(off).astype(int)
    blocked = int(np.abs(d[606:610,1182:1186]).max())
    lit = float(d[806:810,1182:1186,0].mean())
    assert blocked<=1,('GPU light leaked behind infirmary wall',blocked)
    assert lit>5,('same-room floor was not illuminated',lit)
    player = Image.open(ROOT/c['actors'][0]['file']).convert('RGBA')
    opaque = np.asarray(player)[:,:,3]==255
    x,y = c['actors'][0]['at']
    px,py = x*64-32,(y+1)*64-96
    player_changes = int(((np.abs(d[py:py+128,px:px+128]).max(2)>1)&opaque).sum())
    assert player_changes>0
    animation = capture/'animation'
    frames = [Image.open(animation/f'cryo-{i:02}.png').convert('RGB') for i in range(24)]
    screens = [Image.open(animation/f'screen-{i:02}.png').convert('RGB') for i in range(24)]
    a,b = np.asarray(frames[0]).astype(int),np.asarray(frames[8]).astype(int)
    indicator = int((np.abs(a[170:180,150:170]-b[170:180,150:170]).max(2)>0).sum())
    steam = int((np.abs(a[320:395,80:132]-np.asarray(frames[3]).astype(int)[320:395,80:132]).max(2)>0).sum())
    assert indicator>0 and steam>0
    assert len({f.tobytes() for f in screens})>1
    for name,images in [('cryo',frames),('screen',screens)]:
        dest = OUT/f'camp-01-r09b-{name}-animated.webp'
        images[0].save(dest,save_all=True,append_images=images[1:],duration=320,loop=0,lossless=True)
        with Image.open(dest) as check:
            assert check.is_animated and check.n_frames>1
            elapsed = 0
            for i in range(check.n_frames):
                check.seek(i)
                decoded = check.convert('RGB')
                duration = check.info['duration']
                for source_index in range(elapsed//320,(elapsed+duration)//320):
                    assert ImageChops.difference(decoded,images[source_index]).getbbox() is None
                elapsed += duration
            assert elapsed==24*320
    on.crop((0,192,768,832)).save(OUT/'camp-01-r09-cryo-on-native.png',optimize=True)
    off.crop((0,192,768,832)).save(OUT/'camp-01-r09-cryo-off-native.png',optimize=True)
    for name in ['hall','infirmerie']:
        frame = Image.open(capture/f'r09-{name}-native.png')
        if name=='infirmerie':frame.crop((864,704,1376,1088)).save(OUT/'camp-01-r09-medical-native.png',optimize=True)
    current = OUT/'index.html'
    old = OUT/'camp-r08.html'
    if not old.exists():old.write_text(current.read_text(encoding='utf-8'),encoding='utf-8')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Camp · éclairage et ambiance · r09</title>
<style>*{box-sizing:border-box}body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}header,section{padding:16px 24px}header,.copy{max-width:1100px}h1{font-size:25px;color:#e5d2a8}h2{font-size:20px}h3{font-size:16px}p{line-height:1.5}a{color:#b9d3e9}nav{display:flex;gap:12px;flex-wrap:wrap}button,a.button{color:#e3e9e6;background:#25363c;border:1px solid #667c80;padding:9px 14px;font:inherit;cursor:pointer;text-decoration:none}button[aria-pressed=true]{border-color:#debd7b;background:#3d4033}.viewport{overflow:auto;border:1px solid #33484e;max-width:100%;width:fit-content}img{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}.pair{display:flex;flex-wrap:wrap;gap:24px}.pair article{max-width:100%;overflow:auto}.note{color:#b7c6cc}</style>
<header><h1>PLACE-CAMP-01 · r09 — lumières et ambiance actives</h1><p>Les treize luminaires éclairent maintenant le décor, le mobilier et les personnages dans le rendu. Les murs arrêtent leur lumière. Les anciens halos sont compensés par le shader, en conservant les dessins sources.</p><p>Les voyants des cuves pulsent doucement, les écrans s’animent et une petite bouffée de vapeur apparaît occasionnellement près de la cuve ouverte. Les effets suivent une horloge visuelle indépendante des tours.</p><nav><a class="button" href="#animation">Voir les animations</a><a class="button" href="#lighting">Comparer les lampes</a><a class="button" href="#camp">Camp complet</a><a class="button" href="../../Voir_camp_prototype.cmd">Explorer le prototype</a></nav></header>
<section id="animation"><h2>Captures animées du moteur à taille native</h2><div class="pair"><article><div class="viewport"><img id="cryo-animation" src="camp-01-r09-cryo-animated.webp" width="768" height="640" alt="Animation native : voyants des cuves et vapeur près de la cuve ouverte."></div></article><article><h3>Écran du bureau</h3><div class="viewport"><img id="screen-animation" src="camp-01-r09-screen-animated.webp" width="128" height="96" alt="Balayage discret de l’écran du bureau, sans zoom."></div><p class="note">Boucles sans perte issues de 24 images GPU.<br>Le joueur reste immobile pendant leur capture.</p><button id="pause">Mettre les aperçus en pause</button></article></div></section>
<section id="lighting"><h2>Éclairage appliqué par le moteur</h2><nav><button data-light="on" aria-pressed="true">Lampes allumées</button><button data-light="off" aria-pressed="false">Lampes éteintes</button></nav><p>Les personnages conservent leurs dimensions natives. Leur teinte dépend de la lumière reçue à leur position.</p><div class="viewport"><img id="lighting-image" src="camp-01-r09-cryo-on-native.png" width="768" height="640" alt="Comparaison des sources lumineuses allumées et éteintes."></div><p class="note">Éclairage 2D avec occultations fixes précalculées pour ce plan, puis composition GPU sur le décor et les éléments mobiles. La correction des halos peints est une compensation colorimétrique ; les reliefs et ombres peints propres aux matériaux restent dans les dessins.</p></section>
<section id="camp"><h2>Le camp complet</h2><nav><button data-state="camp" aria-pressed="true">Cryogénie</button><button data-state="hall" aria-pressed="false">Joueur dans le hall</button><button data-state="infirmerie" aria-pressed="false">Joueur dans l’infirmerie</button></nav><p>2 432 × 1 152 px sans réduction ; faire défiler horizontalement si nécessaire.</p><div class="viewport"><img id="camp-image" src="engine-r09/r09-camp-native.png" width="2432" height="1152" alt="Camp éclairé avec les trois lits et les passages libres."></div></section>
<section class="copy"><h2>Explorer l’ambiance en direct</h2><p>Lancer <a href="../../Voir_camp_prototype.cmd">Voir_camp_prototype.cmd</a>. Flèches / ZQSD / WASD : marcher · <b>L</b> : allumer/éteindre les lampes · <b>P</b> : mettre l’ambiance en pause · G : grille · R : recommencer · Échap : quitter.</p><p>Le plan, les quatre PNJ, les trois lits et les passages sont conservés. Ces effets d’ambiance ne changent ni les tours, ni la vision tactique, ni les collisions.</p><p class="note">Le prototype reste séparé de la campagne et de ses sauvegardes. Le dialogue corrigé reste conservé dans la revue précédente.</p><p><a href="camp-r08.html">Revue précédente et dialogue</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/atmosphere-acceptance-r09.json">Accord sur les effets</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/full-camp-validation-r09.json">Vérifications</a></p></section>
<script>document.querySelectorAll('button[data-state]').forEach(button=>button.addEventListener('click',()=>{document.getElementById('camp-image').src='engine-r09/r09-'+button.dataset.state+'-native.png';document.querySelectorAll('button[data-state]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));}));document.querySelectorAll('button[data-light]').forEach(button=>button.addEventListener('click',()=>{document.getElementById('lighting-image').src='camp-01-r09-cryo-'+button.dataset.light+'-native.png';document.querySelectorAll('button[data-light]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));}));let paused=false;document.getElementById('pause').addEventListener('click',()=>{paused=!paused;document.getElementById('cryo-animation').src=paused?'camp-01-r09-cryo-on-native.png':'camp-01-r09-cryo-animated.webp';document.getElementById('screen-animation').src=paused?'engine-r09/animation/screen-00.png':'camp-01-r09-screen-animated.webp';document.getElementById('pause').textContent=paused?'Reprendre les animations':'Mettre les aperçus en pause';});</script></html>'''
    page = page.replace('r09 — lumières et ambiance actives', 'r09b — animations plus visibles')
    page = page.replace('ambiance · r09</title>', 'ambiance · r09b</title>')
    page = page.replace('Les voyants des cuves pulsent doucement, les écrans s’animent et une petite bouffée de vapeur apparaît occasionnellement près de la cuve ouverte.', 'Les voyants des cuves pulsent avec davantage de contraste, le balayage des écrans est plus lumineux et plus rapide, et une bouffée de vapeur plus ample s’élève près de la cuve ouverte toutes les six secondes.')
    page = page.replace('Balayage discret de l’écran', 'Balayage renforcé de l’écran')
    page = page.replace('camp-01-r09-cryo-animated.webp', 'camp-01-r09b-cryo-animated.webp')
    page = page.replace('camp-01-r09-screen-animated.webp', 'camp-01-r09b-screen-animated.webp')
    if c.get('art_approved') and c.get('layout_approved'):
        page = page.replace('r09b — animations plus visibles', 'r09b — carte validée')
        page = page.replace('</h1>', '</h1><p><b>Carte validée par le joueur le 17 octobre 2026.</b> Référence du camp de départ : plan, décors, mobilier, emplacement des PNJ, éclairage et animations. Le raccordement à la campagne reste à réaliser.</p>', 1)
    current.write_text(page,encoding='utf-8')
    if (OUT/'campaign-integration-01/checks.json').exists():
        from show_cryogenic_camp_integration import publish
        publish()
    performance = [json.loads(p.read_text(encoding='utf-8'))|dict(requested=p.stem.removeprefix('performance-'))
                   for p in sorted(capture.glob('performance-*x*.json'))]
    report = dict(review_id='HANDCRAFTED-PLACES-01',revision='r09b',runtime_checks=runtime,
        raw_reference_matches_r08_exactly=True,native_actor_reference_checks=actors,
        actor_source_art_unchanged=True,actor_resize_operations=0,layout_props_actors_unchanged=True,
        gpu_blocked_receiver_max_delta=blocked,gpu_lit_receiver_red_delta=lit,player_lit_pixels_changed=player_changes,
        animated_indicator_pixels=indicator,animated_steam_pixels=steam,
        screen_unique_frames=len({f.tobytes() for f in screens}),lossless_animation_export_verified=True,
        performance=performance,performance_baseline_revision='r09',performance_remeasured_for_visibility=False,
        performance_limit='Baseline measured before r09b visibility tuning. Requested 4K window may be clamped by the desktop; use reported actual window sizes.',
        lighting_scope='Static occlusion data for this fixed camp, GPU lighting on all surfaces; no dynamic furniture shadows.',
        campaign_connected=False)
    (REVIEW/'full-camp-validation-r09.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(gallery=str(current),wall_blocked_gpu_delta=blocked,lit_floor_red_delta=lit,
                          indicator_pixels=indicator,steam_pixels=steam,lossless_loops=True)))


if __name__=='__main__':main()
