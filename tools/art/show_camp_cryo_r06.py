"""Publish native GPU evidence for the cryogenics addition to the accepted camp."""
import json
from pathlib import Path

from PIL import Image, ImageChops, ImageDraw, ImageFont
from show_camp_reference_r05 import verify_actors, sha

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'
ASSETS = ROOT / 'assets/prototypes/handcrafted-camp-01/r06'


def main():
    c = json.loads((ASSETS / 'camp.json').read_text(encoding='utf-8'))
    captures = OUT / 'engine-r06'
    runtime = json.loads((captures / 'full-engine-checks-r06.json').read_text(encoding='utf-8'))
    assert runtime['actual_gpu_capture'] and runtime['spawn'] == [2, 8]
    proof = []
    for state in runtime['captures']:
        path = captures / f'r06-{state["name"]}-native.png'
        assert list(Image.open(path).size) == runtime['size']
        actors = verify_actors(path, c, state['player_at'])
        proof.append(dict(state=state['name'], file=path.relative_to(ROOT).as_posix(),
                          sha256=sha(path), actors=actors))
    hall = Image.open(captures / 'r06-hall-native.png').convert('RGB')
    approved = OUT / 'engine-r05/r05-camp-native.png'
    assert ImageChops.difference(hall.crop((640, 0, 2432, 1152)), Image.open(approved).convert('RGB')).getbbox() is None
    first = Image.open(captures / 'r06-camp-native.png').convert('RGB')
    first.crop((0, 192, 640, 832)).save(OUT / 'camp-01-r06-cryo-native.png', optimize=True)
    first.crop((416, 448, 768, 640)).save(OUT / 'camp-01-r06-connection-native.png', optimize=True)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 14)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    board = Image.new('RGBA', (576, 200), '#111b20')
    draw = ImageDraw.Draw(board)
    records = [dict(id='PLAYER', label='Joueur', file=c['actors'][0]['file'])] + c['prop_sources'][-2:]
    for i, item in enumerate(records):
        x = i * 192
        sprite = Image.open(ROOT / item['file']).convert('RGBA')
        box = sprite.getbbox()
        draw.rectangle((x + 2, 2, x + 188, 196), outline='#32464b')
        draw.text((x + 12, 8), item['id'], font=font, fill='#e4c994')
        draw.text((x + 12, 29), item['label'], font=small, fill='#c7d5d5')
        draw.text((x + 12, 46), f'{box[2]-box[0]} × {box[3]-box[1]} px', font=small, fill='#c7d5d5')
        board.alpha_composite(sprite, (x + 32, 64))
    board.convert('RGB').save(OUT / 'camp-01-r06-pods-native.png', optimize=True)

    current = OUT / 'index.html'
    old = OUT / 'camp-r05.html'
    if not old.exists():
        old.write_text(current.read_text(encoding='utf-8'), encoding='utf-8')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Camp · salle de cryogénie · r06</title>
<style>*{box-sizing:border-box}body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}header,section{padding:16px 24px}header{max-width:1100px}h1{font-size:25px;color:#e5d2a8}h2{font-size:20px}p{line-height:1.5}a{color:#b9d3e9}nav{display:flex;gap:12px;flex-wrap:wrap}button,a.button{color:#e3e9e6;background:#25363c;border:1px solid #667c80;padding:9px 14px;font:inherit;cursor:pointer;text-decoration:none}button[aria-pressed=true]{border-color:#debd7b;background:#3d4033}.viewport{overflow:auto;border:1px solid #33484e;max-width:100%;width:fit-content}img{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}.pair{display:flex;flex-wrap:wrap;gap:24px}.pair article{max-width:100%;overflow:auto}.note{color:#b7c6cc}</style>
<header><h1>PLACE-CAMP-01 · r06 — salle de cryogénie à gauche</h1>
<p>Le joueur apparaît à côté de sa cuve ouverte. Six cuves fermées abritent les autres candidats, derrière leur verre givré. Un passage rejoint directement le hall.</p>
<p>Le camp r05 est validé et conserve exactement son rendu. Les ajouts de cette revue sont la salle, les deux modèles de cuve et le point de départ.</p>
<nav><a class="button" href="#cryo">Voir la cryogénie</a><a class="button" href="#camp">Camp complet</a><a class="button" href="#pods">Cuves à taille native</a><a class="button" href="../../Voir_camp_prototype.cmd">Explorer le prototype</a></nav></header>
<section id="cryo"><h2>Point de départ — capture moteur à taille native</h2><div class="pair"><article><div class="viewport"><img src="camp-01-r06-cryo-native.png" width="640" height="640" alt="Salle de cryogénie, six cuves fermées et une cuve ouverte à côté du joueur."></div></article><article><h2>Raccord au hall, sans zoom</h2><div class="viewport"><img src="camp-01-r06-connection-native.png" width="352" height="192" alt="Le passage ouvert entre la cryogénie et le hall."></div><p class="note">Les cuves bloquent leurs cases.<br>Le joueur peut sortir de la salle et rejoindre les quatre PNJ.</p></article></div></section>
<section id="camp"><h2>Le camp avec sa nouvelle salle</h2><nav><button data-state="camp" aria-pressed="true">Réveil en cryogénie</button><button data-state="hall" aria-pressed="false">Joueur dans le hall</button><button data-state="missions" aria-pressed="false">Joueur dans les missions</button></nav><p>Image de 2 432 × 1 152 px, sans réduction : faire défiler horizontalement si nécessaire.</p><div class="viewport"><img id="camp-image" src="engine-r06/r06-camp-native.png" width="2432" height="1152" alt="Le camp complet avec la salle de cryogénie à gauche."></div></section>
<section id="pods"><h2>DEC-CAMP-15 et 16 — nouveaux candidats à taille native</h2><p>Même échelle que dans la carte : joueur de 65 px de haut, cuves de 96 px. Les nouveaux dessins restent présentés dans ce prototype avant leur intégration à la campagne.</p><div class="viewport"><img src="camp-01-r06-pods-native.png" width="576" height="200" alt="Joueur et deux cuves alignés à la même taille que dans le jeu."></div></section>
<section><h2>Explorer le camp</h2><p>Lancer <a href="../../Voir_camp_prototype.cmd">Voir_camp_prototype.cmd</a> dans le dossier du projet. Flèches / ZQSD / WASD : marcher · G : grille · R : recommencer · Échap : quitter.</p><p class="note">Le prototype démarre désormais en cryogénie. Les cuves sont des décors ; aucune interaction ou animation de réveil n’est ajoutée par cette revue. Les services et la sortie vers les secteurs restent à raccorder.</p><p><a href="camp-r05.html">Camp r05 validé</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/generation-r06.json">Sources et prompts imagegen</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/full-camp-validation-r06.json">Vérifications</a></p></section>
<script>document.querySelectorAll('button[data-state]').forEach(button=>button.addEventListener('click',()=>{document.getElementById('camp-image').src='engine-r06/r06-'+button.dataset.state+'-native.png';document.querySelectorAll('button[data-state]').forEach(b=>b.setAttribute('aria-pressed',String(b===button)));}));</script></html>'''
    current.write_text(page, encoding='utf-8')
    report = dict(review_id='HANDCRAFTED-PLACES-01', revision='r06',
                  base_revision_approved='r05', new_extension_approved=False,
                  campaign_connected=False, runtime_checks=runtime, captures=proof,
                  accepted_camp_gpu_pixels_exact=True, accepted_camp_capture_sha256=sha(approved),
                  comparison='r06-hall-native.png cropped at [640,0,2432,1152] equals r05-camp-native.png',
                  actor_resize_operations=0, all_opaque_actor_pixels_equal=True,
                  native_assets=[dict(file=p.relative_to(ROOT).as_posix(), sha256=sha(p))
                                 for p in sorted(ASSETS.iterdir()) if p.is_file()])
    (REVIEW / 'full-camp-validation-r06.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(gallery=str(current), actor_checks=15, accepted_camp_gpu_pixels_exact=True,
                          pods={'closed':6,'open':1}, spawn=runtime['spawn'])))


if __name__ == '__main__':
    main()
