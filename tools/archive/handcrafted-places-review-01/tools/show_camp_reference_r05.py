"""Publish the selected full camp from real GPU captures, at native scale."""
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'
ASSETS = ROOT / 'assets/prototypes/handcrafted-camp-01/r05'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_actors(path, config, player_at, origin=(0, 0), subset=None):
    frame = Image.open(path).convert('RGB')
    verified = []
    for actor in subset or config['actors']:
        x, y = player_at if actor['id'] == 'PLAYER' else actor['at']
        sprite = Image.open(ROOT / actor['file']).convert('RGBA')
        px, py = origin[0] + x * 64 - 32, origin[1] + (y + 1) * 64 - 96
        actual = frame.crop((px, py, px + 128, py + 128))
        count = 0
        for a, b in zip(sprite.get_flattened_data(), actual.get_flattened_data()):
            if a[3] == 255:
                assert a[:3] == b, (path.name, actor['id'], 'native actor pixel changed')
                count += 1
        assert count
        verified.append(dict(id=actor['id'], source=actor['file'], sha256=sha(ROOT / actor['file']),
                             opaque_pixels_verified=count, scale=1, placement=[px, py]))
    return verified


def main():
    c = json.loads((ASSETS / 'camp.json').read_text(encoding='utf-8'))
    captures = OUT / 'engine-r05'
    runtime = json.loads((captures / 'full-engine-checks-r05.json').read_text(encoding='utf-8'))
    assert runtime['actual_gpu_capture'] and not runtime['campaign_connected']
    proof = []
    for state in runtime['captures']:
        path = captures / f'r05-{state["name"]}-native.png'
        assert list(Image.open(path).size) == runtime['size']
        actors = verify_actors(path, c, state['player_at'])
        proof.append(dict(state=state['name'], file=path.relative_to(ROOT).as_posix(),
                          sha256=sha(path), actors=actors))

    frame = Image.open(captures / 'r05-camp-native.png').convert('RGB')
    rooms = [('missions', 'Missions · NPC-01', (224, 64, 736, 448)),
             ('armurerie', 'Armurerie · NPC-02', (1056, 64, 1568, 448)),
             ('soins', 'Soins · NPC-03', (224, 672, 736, 1056)),
             ('atelier', 'Atelier · NPC-04', (1056, 672, 1568, 1056))]
    room_cards = []
    for code, label, box in rooms:
        name = f'camp-01-r05-{code}-detail-native.png'
        frame.crop(box).save(OUT / name, optimize=True)
        room_cards.append(f'<article><h3>{label}</h3><img src="{name}" width="512" height="384" alt="{label}, extrait sans zoom de la capture moteur."></article>')

    fonts = [ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', size) for size in [14, 12]]
    board = Image.new('RGBA', (1056, 392), '#111b20')
    draw = ImageDraw.Draw(board)
    records = [dict(id='PLAYER', label='Joueur', file=c['actors'][0]['file']),
               dict(id='NPC-03', label='PNJ', file=c['actors'][3]['file'])] + c['prop_sources']
    records.append(dict(id='DEC-CAMP-02', label='Siège', file='assets/prototypes/handcrafted-camp-01/r04/DEC-CAMP-02-r04.png'))
    for n, prop in enumerate(records):
        x, y = n % 6 * 176, n // 6 * 196
        sprite = Image.open(ROOT / prop['file']).convert('RGBA')
        bounds = sprite.getbbox()
        width, height = bounds[2] - bounds[0], bounds[3] - bounds[1]
        draw.rectangle((x + 2, y + 2, x + 172, y + 192), outline='#32464b')
        draw.text((x + 10, y + 8), prop['id'], fill='#e4c994', font=fonts[0])
        draw.text((x + 10, y + 29), prop['label'], fill='#c7d5d5', font=fonts[1])
        draw.text((x + 10, y + 46), f'{width} × {height} px', fill='#c7d5d5', font=fonts[1])
        board.alpha_composite(sprite, (x + 24, y + 52))
    board.convert('RGB').save(OUT / 'camp-01-r05-proportions-native.png', optimize=True)

    fixture = OUT / 'door-fixture-r05'
    door_check = json.loads((fixture / 'engine-checks-r05-door-fixture.json').read_text(encoding='utf-8'))
    assert door_check['corrected_door_jambs'] and door_check['player_crossed_door']
    fixture_actors = [dict(id='PLAYER', file=c['actors'][0]['file'], at=None),
                      dict(id='NPC-02', file=c['actors'][2]['file'], at=[4, 3]),
                      dict(id='NPC-03', file=c['actors'][3]['file'], at=[11, 4])]
    door_proof = []
    ox, oy = map(int, door_check['map_origin'])
    for state, player_at in [('open', [5, 4]), ('closed', [6, 4]), ('crossed', [8, 4])]:
        path = fixture / f'r05-door-fixture-native-{state}.png'
        actors = verify_actors(path, c, player_at, (ox, oy), fixture_actors)
        Image.open(path).crop((ox + 6 * 64, oy + 3 * 64, ox + 9 * 64, oy + 6 * 64)).save(OUT / f'camp-01-r05-door-{state}-native.png')
        door_proof.append(dict(state=state, sha256=sha(path), actors=actors))

    current = OUT / 'index.html'
    previous = OUT / 'engine-r04.html'
    if not previous.exists():
        previous.write_text(current.read_text(encoding='utf-8'), encoding='utf-8')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<title>Camp complet · référence choisie · r05</title><style>
*{box-sizing:border-box}body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}header,section{padding:16px 24px}header{max-width:1200px}h1{font-size:25px;color:#e5d2a8}h2{font-size:20px}h3{font-size:16px}p{line-height:1.5}a{color:#b9d3e9}nav{display:flex;gap:12px;flex-wrap:wrap}button,a.button{color:#e3e9e6;background:#25363c;border:1px solid #667c80;padding:9px 14px;font:inherit;cursor:pointer;text-decoration:none}button[aria-pressed=true]{border-color:#debd7b;background:#3d4033}.viewport{overflow:auto;border:1px solid #33484e;max-width:100%;width:fit-content}img{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}.rooms{display:flex;flex-wrap:wrap;gap:20px}.rooms article{overflow:auto;max-width:100%;border:1px solid #33484e;padding:12px}.note{color:#b7c6cc}.pending{color:#e3c591}code{color:#d8c8aa}
</style><header><h1>PLACE-CAMP-01 · r05 — le camp de l’image choisie</h1>
<p>Les quatre pièces et le grand passage central reprennent la référence que tu as désignée.
Le mobilier est séparé du fond et ramené à la taille réelle du jeu. Les cinq personnages utilisent leurs sprites acceptés, sans agrandissement.</p>
<p class="pending">La référence visuelle est choisie. Cette adaptation avec le mobilier redimensionné reste à examiner avant son intégration à la campagne.</p>
<nav><a class="button" href="#camp">Camp complet</a><a class="button" href="#pieces">Les quatre pièces</a><a class="button" href="#proportions">Mobilier à taille native</a><a class="button" href="#essayer">Essayer</a><a class="button" href="../../assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r05/camp-reference-user-r05.png">Image de référence choisie</a></nav></header>
<section id="camp"><h2>Capture réelle du moteur — 1 pixel d’image = 1 pixel du jeu</h2>
<nav><button data-state="camp" aria-pressed="true">Vue initiale</button><button data-state="missions" aria-pressed="false">Joueur dans les missions</button><button data-state="soins" aria-pressed="false">Joueur dans les soins</button></nav>
<p>Défilement horizontal si nécessaire : l’image de 1 792 × 1 152 px conserve les tailles natives.</p>
<div class="viewport"><img id="camp-image" src="engine-r05/r05-camp-native.png" width="1792" height="1152" alt="Le camp complet dans le moteur, mobilier et personnages à taille native."></div>
<p class="note">Les passages vers les quatre pièces sont ouverts, comme sur la référence. Les murs, meubles et PNJ bloquent réellement le déplacement. Le joueur peut rejoindre chaque PNJ et les deux sorties latérales.</p></section>
<section id="pieces"><h2>Détails sans zoom</h2><div class="rooms">ROOMS</div></section>
<section id="proportions"><h2>Personnages et mobilier à la même échelle</h2><p>Mêmes fichiers et mêmes tailles que dans la carte. Codes de revue DEC-CAMP-07 à 14 ; le siège DEC-CAMP-02 vient de l’essai précédent.</p>
<div class="viewport"><img src="camp-01-r05-proportions-native.png" width="1056" height="392" alt="Comparaison des personnages et du mobilier, sans zoom."></div></section>
<section><h2>Correction de la porte de l’ancien essai</h2><p>Le morceau détaché provenait du découpage de l’encadrement. Les montants corrigés rejoignent maintenant les murs. Ce test à deux pièces est distinct des passages ouverts du camp complet.</p><div class="rooms">
<article><h3>Ouverte</h3><img src="camp-01-r05-door-open-native.png" width="192" height="192" alt="Porte ouverte corrigée."></article><article><h3>Fermée</h3><img src="camp-01-r05-door-closed-native.png" width="192" height="192" alt="Porte fermée corrigée."></article><article><h3>Après franchissement</h3><img src="camp-01-r05-door-crossed-native.png" width="192" height="192" alt="Le joueur a franchi la porte."></article></div></section>
<section id="essayer"><h2>Se déplacer dans ce camp</h2><p>Dans le dossier du projet, lancer <a href="../../Voir_camp_prototype.cmd">Voir_camp_prototype.cmd</a>.</p><p>Flèches / ZQSD / WASD : marcher · G : grille · R : recommencer · Échap : quitter.</p>
<p class="note">Prototype séparé : les PNJ sont immobiles, sans nouvelle règle de service, et les sorties ne changent pas encore de secteur. L’architecture de ce lieu fixe est peinte ; le mobilier et les acteurs restent des éléments séparés. Aucune sauvegarde de campagne n’est utilisée.</p>
<p><a href="engine-r04.html">Ancien essai r04</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/generation-r05.json">Sources et prompts imagegen</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/full-camp-validation-r05.json">Vérifications</a></p></section>
<script>document.querySelectorAll('button[data-state]').forEach(button=>button.addEventListener('click',()=>{document.getElementById('camp-image').src='engine-r05/r05-'+button.dataset.state+'-native.png';document.querySelectorAll('button[data-state]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));}));</script></html>'''.replace('ROOMS', ''.join(room_cards))
    current.write_text(page, encoding='utf-8')
    report = dict(review_id='HANDCRAFTED-PLACES-01', revision='r05',
                  reference_target_selected=True, native_adaptation_approved=False,
                  campaign_connected=False, runtime_checks=runtime, captures=proof,
                  corrected_door_fixture_checks=door_check, door_captures=door_proof,
                  source_art='assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r05',
                  architecture_layers_separate=True, furnishings_separate=True,
                  actor_resize_operations=0, all_opaque_actor_pixels_equal=True)
    (REVIEW / 'full-camp-validation-r05.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(gallery=str(current), native_size=runtime['size'], actor_checks=len(proof) * 5,
                          door_actor_checks=len(door_proof) * 3, exact=True)))


if __name__ == '__main__':
    main()
