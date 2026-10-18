"""Publish the accepted officer's placement, with native GPU preservation checks."""
import html
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw
from show_camp_reference_r05 import verify_actors, sha

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def main():
    c = json.loads((ROOT / 'assets/prototypes/handcrafted-camp-01/r07/camp.json').read_text(encoding='utf-8'))
    base = json.loads((ROOT / 'assets/prototypes/handcrafted-camp-01/r06/camp.json').read_text(encoding='utf-8'))
    decision = json.loads((REVIEW / 'cryo-officer-placement-01.json').read_text(encoding='utf-8'))
    assert c['floor'] == base['floor'] and c['structure'] == base['structure']
    assert c['props'] == base['props']
    for a, b in zip(c['actors'], base['actors']):
        assert a['id'] == b['id'] and a['file'] == b['file']
        if a['id'] != 'NPC-01':
            assert a == b
    captures = OUT / 'engine-r07'
    runtime = json.loads((captures / 'full-engine-checks-r07.json').read_text(encoding='utf-8'))
    assert runtime['actual_gpu_capture'] and runtime['actor_count'] == 5
    proof = []
    for state in runtime['captures']:
        path = captures / f'r07-{state["name"]}-native.png'
        actors = verify_actors(path, c, state['player_at'])
        proof.append(dict(state=state['name'], file=path.relative_to(ROOT).as_posix(),
                          sha256=sha(path), actors=actors))
    first = Image.open(captures / 'r07-camp-native.png').convert('RGB')
    previous = Image.open(OUT / 'engine-r06/r06-camp-native.png').convert('RGB')
    officer = next(a for a in c['actors'] if a['id'] == 'NPC-01')
    sprite = Image.open(ROOT / officer['file']).convert('RGBA')
    allowed = Image.new('L', first.size)
    # Restrict permitted changes to the officer's pixels and separate shadow,
    # both at the old desk and at the newly requested entrance position.
    for x, y in [decision['previous_at'], decision['new_at']]:
        px, py = x * 64 - 32, (y + 1) * 64 - 96
        allowed.paste(sprite.getchannel('A').point(lambda a: 255 if a else 0), (px, py))
        ImageDraw.Draw(allowed).rectangle((px + 48, py + 93, px + 79, py + 97), fill=255)
    difference = ImageChops.difference(first, previous)
    assert difference.getbbox() is not None
    assert ImageChops.multiply(difference, ImageChops.invert(allowed).convert('RGB')).getbbox() is None
    first.crop((0, 192, 768, 832)).save(OUT / 'camp-01-r07-cryo-native.png', optimize=True)
    first.crop((320, 384, 768, 640)).save(OUT / 'camp-01-r07-entrance-native.png', optimize=True)
    Image.open(captures / 'r07-officier-native.png').crop((320, 384, 768, 640)).save(OUT / 'camp-01-r07-conversation-native.png', optimize=True)
    current = OUT / 'index.html'
    historical = OUT / 'camp-r06.html'
    if not historical.exists():
        historical.write_text(current.read_text(encoding='utf-8'), encoding='utf-8')
    dialogue = ''.join('<p>« ' + html.escape(t) + ' »</p>' for t in decision['dialogue_proposal']['blocks'])
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Camp · officier à la cryogénie · r07</title>
<style>*{box-sizing:border-box}body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}header,section{padding:16px 24px}header,.copy{max-width:1100px}h1{font-size:25px;color:#e5d2a8}h2{font-size:20px}p{line-height:1.5}a{color:#b9d3e9}nav{display:flex;gap:12px;flex-wrap:wrap}button,a.button{color:#e3e9e6;background:#25363c;border:1px solid #667c80;padding:9px 14px;font:inherit;cursor:pointer;text-decoration:none}button[aria-pressed=true]{border-color:#debd7b;background:#3d4033}.viewport{overflow:auto;border:1px solid #33484e;max-width:100%;width:fit-content}img{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}.pair{display:flex;flex-wrap:wrap;gap:24px}.pair article{max-width:100%;overflow:auto}.note{color:#b7c6cc}.pending{color:#e5c68b}</style>
<header><h1>PLACE-CAMP-01 · r07 — accueil à la cryogénie</h1>
<p>NPC-01 r03, l’officier casqué au baudrier ivoire déjà validé, attend à l’intérieur de la salle, juste à côté de l’entrée. Le passage vers le hall reste libre.</p><p>Il est déplacé depuis son bureau : c’est le même personnage, chargé d’expliquer le réveil du détenu et la mission. Son dessin et sa taille sont inchangés.</p><nav><a class="button" href="#cryo">Voir son emplacement</a><a class="button" href="#camp">Camp complet</a><a class="button" href="#dialogue">Proposition de dialogue</a><a class="button" href="../../Voir_camp_prototype.cmd">Explorer le prototype</a></nav></header>
<section id="cryo"><h2>Officier à l’entrée — taille native</h2><div class="pair"><article><div class="viewport"><img src="camp-01-r07-cryo-native.png" width="768" height="640" alt="Le joueur près de sa cuve et l’officier à l’entrée de la cryogénie."></div></article><article><h2>Approche sans bloquer le passage</h2><div class="viewport"><img src="camp-01-r07-conversation-native.png" width="448" height="256" alt="Le joueur a rejoint l’officier, le passage reste ouvert."></div><p class="note">Capture réelle après déplacement dans le moteur.<br>Le PNJ est approchable depuis la salle et le hall.</p></article></div></section>
<section id="camp"><h2>Le camp complet</h2><nav><button data-state="camp" aria-pressed="true">Point de départ</button><button data-state="officier" aria-pressed="false">Devant l’officier</button><button data-state="hall" aria-pressed="false">Dans le hall</button></nav><p>2 432 × 1 152 px sans zoom ; faire défiler horizontalement si nécessaire.</p><div class="viewport"><img id="camp-image" src="engine-r07/r07-camp-native.png" width="2432" height="1152" alt="Le camp complet avec l’officier près de la cryogénie."></div></section>
<section id="dialogue" class="copy"><h2>DIAL-CRYO-01 r01 — proposition d’accueil</h2><p class="pending">Texte proposé pour validation. Il est présenté ici et n’est pas encore joué par le PNJ.</p>DIALOGUE<p class="note">D’après le récit de campagne, section « Le prisonnier et sa mission ». Aucun objectif chiffré, choix de réponse, verrou ou récompense n’est ajouté par cette proposition.</p></section>
<section class="copy"><h2>Explorer le prototype</h2><p>Lancer <a href="../../Voir_camp_prototype.cmd">Voir_camp_prototype.cmd</a>. Flèches / ZQSD / WASD : marcher · G : grille · R : recommencer · Échap : quitter.</p><p class="note">Les parcours jusqu’aux quatre PNJ et à la sortie sont vérifiés. Aucune sauvegarde de campagne n’est utilisée.</p><p><a href="camp-r06.html">Cryogénie r06</a> · <a href="camp-r05.html">Camp r05 validé</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/cryo-officer-placement-01.json">Placement et texte proposés</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/full-camp-validation-r07.json">Vérifications</a></p></section>
<script>document.querySelectorAll('button[data-state]').forEach(button=>button.addEventListener('click',()=>{document.getElementById('camp-image').src='engine-r07/r07-'+button.dataset.state+'-native.png';document.querySelectorAll('button[data-state]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));}));</script></html>'''.replace('DIALOGUE', dialogue)
    current.write_text(page, encoding='utf-8')
    report = dict(review_id='HANDCRAFTED-PLACES-01', revision='r07', actor_count=5,
                  reused_actor='NPC-01-r03', actor_sha256=sha(ROOT / officer['file']),
                  native_actor_pixels_exact=True, captures=proof, runtime_checks=runtime,
                  environment_assets_unchanged=True, furniture_unchanged=True,
                  gpu_changes_only_at_old_and_new_officer_pixels=True,
                  duplicate_officer=False, dialogue_connected=False,
                  decision='cryo-officer-placement-01.json', campaign_connected=False)
    (REVIEW / 'full-camp-validation-r07.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(gallery=str(current), actor_checks=15, officer_native_exact=True,
                          only_officer_moved=True, dialogue_connected=False)))


if __name__ == '__main__':
    main()
