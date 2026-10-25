"""Publish the requested furniture/doorway edits and pending ambience proposals."""
import html
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw
from show_camp_reference_r05 import verify_actors, sha

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def main():
    c = json.loads((ROOT / 'assets/prototypes/handcrafted-camp-01/r08/camp.json').read_text(encoding='utf-8'))
    base = json.loads((ROOT / 'assets/prototypes/handcrafted-camp-01/r07/camp.json').read_text(encoding='utf-8'))
    decision = json.loads((REVIEW / 'camp-adjustments-r08.json').read_text(encoding='utf-8'))
    assert c['actors'] == base['actors'] and c['layout'] == base['layout']
    assert not any(p['id'] == 'DEC-CAMP-02' for p in c['props'])
    assert sum(p['id'] == 'DEC-CAMP-10' for p in c['props']) == 3
    captures = OUT / 'engine-r08'
    runtime = json.loads((captures / 'full-engine-checks-r08.json').read_text(encoding='utf-8'))
    assert runtime['actual_gpu_capture']
    proof = []
    for state in runtime['captures']:
        path = captures / f'r08-{state["name"]}-native.png'
        actors = verify_actors(path, c, state['player_at'])
        proof.append(dict(state=state['name'], file=path.relative_to(ROOT).as_posix(),
                          sha256=sha(path), actors=actors))
    first = Image.open(captures / 'r08-camp-native.png').convert('RGB')
    previous = Image.open(OUT / 'engine-r07/r07-camp-native.png').convert('RGB')
    allowed = Image.new('L', first.size)
    changed_props = [p for p in base['props'] if p['id'] == 'DEC-CAMP-02']
    changed_props += [p for p in c['props'] if p['id'] == 'DEC-CAMP-10' and p not in base['props']]
    for p in changed_props:
        x, y = p['at']
        px, py = x * 64 - 32, (y + 1) * 64 - 96
        sprite = Image.open(ROOT / p['file']).convert('RGBA')
        allowed.paste(sprite.getchannel('A').point(lambda a: 255 if a else 0), (px, py))
        ImageDraw.Draw(allowed).rectangle((px + 48, py + 93, px + 79, py + 97), fill=255)
    for x0, y0, x1, y1 in c['modified_architecture_rects']:
        ImageDraw.Draw(allowed).rectangle((x0, y0, x1 - 1, y1 - 1), fill=255)
    difference = ImageChops.difference(first, previous)
    assert ImageChops.multiply(difference, ImageChops.invert(allowed).convert('RGB')).getbbox() is None
    hall = Image.open(captures / 'r08-hall-native.png')
    for name, box in [('infirmerie', (864, 704, 1376, 1088)),
                      ('door-medical', (992, 640, 1248, 896)),
                      ('door-workshop', (1824, 640, 2080, 896))]:
        hall.crop(box).save(OUT / f'camp-01-r08-{name}-native.png', optimize=True)
    current = OUT / 'index.html'
    historical = OUT / 'camp-r07.html'
    if not historical.exists():
        historical.write_text(current.read_text(encoding='utf-8'), encoding='utf-8')
    dialogue = ''.join('<p>« ' + html.escape(t) + ' »</p>' for t in decision['dialogue']['blocks'])
    lighting = decision['lighting_proposal']
    proposals = ''.join(f'<article class="card"><h3>{a["id"]} · {html.escape(a["title"])}</h3><p>{html.escape(a["description"])}</p><p class="note">Proposition à examiner'+(' · conseillée pour commencer' if a['recommended_first'] else '')+'</p></article>' for a in decision['animation_proposals'])
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Camp · accès et infirmerie · r08</title>
<style>*{box-sizing:border-box}body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}header,section{padding:16px 24px}header,.copy{max-width:1100px}h1{font-size:25px;color:#e5d2a8}h2{font-size:20px}h3{font-size:16px}p{line-height:1.5}a{color:#b9d3e9}nav{display:flex;gap:12px;flex-wrap:wrap}button,a.button{color:#e3e9e6;background:#25363c;border:1px solid #667c80;padding:9px 14px;font:inherit;cursor:pointer;text-decoration:none}button[aria-pressed=true]{border-color:#debd7b;background:#3d4033}.viewport{overflow:auto;border:1px solid #33484e;max-width:100%;width:fit-content}img{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}.pair{display:flex;flex-wrap:wrap;gap:24px}.pair article{max-width:100%;overflow:auto}.note{color:#b7c6cc}.card{max-width:360px;border:1px solid #33484e;padding:14px}.pending{color:#e5c68b}</style>
<header><h1>PLACE-CAMP-01 · r08 — accès dégagés et infirmerie</h1><p>Les trois chaises ont été retirées. L’infirmerie compte maintenant trois lits, en gardant la secouriste et le chariot accessibles.</p><p>Les lampes qui traversaient les deux ouvertures du bas sont déplacées sur les murs voisins. Le sol continue à travers les accès.</p><nav><a class="button" href="#details">Infirmerie et ouvertures</a><a class="button" href="#camp">Camp complet</a><a class="button" href="#dialogue">Dialogue corrigé</a><a class="button" href="#ambiance">Lumières et animations proposées</a></nav></header>
<section id="details"><h2>Captures du moteur à taille native</h2><div class="pair"><article><h3>Infirmerie · trois lits</h3><div class="viewport"><img src="camp-01-r08-infirmerie-native.png" width="512" height="384" alt="Infirmerie avec trois lits et la secouriste."></div></article><article><h3>Accès aux soins</h3><img src="camp-01-r08-door-medical-native.png" width="256" height="256" alt="Passage dégagé et lampe sur le mur latéral."></article><article><h3>Accès à l’atelier</h3><img src="camp-01-r08-door-workshop-native.png" width="256" height="256" alt="Accès dégagé à l’atelier, sans bande lumineuse traversante."></article></div></section>
<section id="camp"><h2>Le camp complet</h2><nav><button data-state="camp" aria-pressed="true">Point de départ</button><button data-state="hall" aria-pressed="false">Dans le hall</button><button data-state="infirmerie" aria-pressed="false">Près de la secouriste</button></nav><p>2 432 × 1 152 px, sans réduction. Faire défiler horizontalement si nécessaire.</p><div class="viewport"><img id="camp-image" src="engine-r08/r08-camp-native.png" width="2432" height="1152" alt="Le camp sans chaises, avec trois lits et les deux ouvertures du bas corrigées."></div></section>
<section id="dialogue" class="copy"><h2>DIAL-CRYO-01 r02 — correction demandée</h2>DIALOGUE<p class="note">Les passages inchangés du texte aimé sont conservés. La mention des autres détenus est remplacée par l’absence de choix. Le dialogue reste présenté dans la revue et n’est pas encore déclenché dans le prototype.</p></section>
<section id="ambiance" class="copy"><h2>LUM-CAMP-01 — éclairage calculé</h2><p>LIGHTING</p><p class="note">PREPARATION</p><p class="pending">Proposition pour la prochaine étape. Les captures ci-dessus utilisent encore l’éclairage peint ; les deux lampes ont seulement été déplacées pour dégager les accès.</p><h2>Petites animations d’ambiance proposées</h2><div class="pair">PROPOSALS</div><p>Pour commencer : éclairage stable, voyants des cuves et écrans. La vapeur peut venir ensuite si elle sert l’ambiance.</p><p class="note">SCOPE</p></section>
<section class="copy"><h2>Explorer le prototype</h2><p>Lancer <a href="../../Voir_camp_prototype.cmd">Voir_camp_prototype.cmd</a>. Flèches / ZQSD / WASD : marcher · G : grille · R : recommencer · Échap : quitter.</p><p class="note">Les parcours vers les quatre PNJ et la sortie sont vérifiés. Les propositions d’ambiance sont présentées avant leur activation, conformément à AGENTS.md.</p><p><a href="camp-r07.html">Revue précédente</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/camp-adjustments-r08.json">Décisions et propositions</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/generation-r08.json">Sources et prompt imagegen des deux ouvertures</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/full-camp-validation-r08.json">Vérifications</a></p></section>
<script>document.querySelectorAll('button[data-state]').forEach(button=>button.addEventListener('click',()=>{document.getElementById('camp-image').src='engine-r08/r08-'+button.dataset.state+'-native.png';document.querySelectorAll('button[data-state]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));}));</script></html>'''
    page = page.replace('DIALOGUE', dialogue).replace('LIGHTING', html.escape(lighting['description'])).replace('PREPARATION', html.escape(lighting['preparation_required'])).replace('PROPOSALS', proposals).replace('SCOPE', html.escape(decision['animation_scope']))
    current.write_text(page, encoding='utf-8')
    report = dict(review_id='HANDCRAFTED-PLACES-01', revision='r08', captures=proof,
                  runtime_checks=runtime, actors_unchanged=True, actor_opaque_pixels_exact=True,
                  changes_confined_to_requested_furniture_and_doorways=True,
                  chairs_removed=3, beds=3, actor_count=5,
                  real_lighting_implemented=False, ambient_animations_implemented=False,
                  generation='generation-r08.json', proposals='camp-adjustments-r08.json',
                  campaign_connected=False)
    (REVIEW / 'full-camp-validation-r08.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(gallery=str(current), actor_checks=15, chairs_removed=3,
                          beds=3, doorways_corrected=2, other_pixels_preserved=True)))


if __name__ == '__main__':
    main()
