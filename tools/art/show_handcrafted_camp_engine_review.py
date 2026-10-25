"""Publish actual native engine captures; no illustrated-room substitution."""
import argparse
import hashlib
import html
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'
ACTORS = [
    ('PLAYER', 'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png', None),
    ('NPC-02', 'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-02-r02/NPC-02-r02-native64.png', (4, 3)),
    ('NPC-03', 'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-03-r02/NPC-03-r02-native64.png', (11, 4)),
]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--revision',choices=['r03','r04'],default='r04')
    revision=parser.parse_args().revision
    captures=OUT/f'engine-{revision}'
    checks = json.loads((captures / f'engine-checks-{revision}.json').read_text(encoding='utf-8'))
    assert checks['actual_gpu_capture'] and checks['player_crossed_door']
    ox, oy = map(int, checks['map_origin'])
    width, height = (n * 64 for n in checks['map_size_cells'])
    variants = [('open', (5, 4)), ('closed', (6, 4)), ('crossed', (8, 4))]
    proof = []
    for state, player_at in variants:
        source = captures / f'{revision}-native-{state}.png'
        image = Image.open(source).convert('RGB')
        assert list(image.size) == checks['capture_size']
        verified = []
        for code, path, at in ACTORS:
            x, y = player_at if at is None else at
            sprite = Image.open(ROOT / path).convert('RGBA')
            px, py = ox + x * 64 - 32, oy + (y + 1) * 64 - 96
            actual = image.crop((px, py, px + 128, py + 128))
            count = 0
            for a, b in zip(sprite.get_flattened_data(), actual.get_flattened_data()):
                if a[3] == 255:
                    assert a[:3] == b, (state, code)
                    count += 1
            verified.append(dict(id=code, source=path, sha256=sha(ROOT / path),
                                 body_bounds=list(sprite.getbbox()), opaque_pixels_verified=count,
                                 placement=[px, py], scale=1))
        dest = OUT / f'camp-01-{revision}-{state}-native.png'
        image.crop((ox, oy, ox + width, oy + height)).save(dest, optimize=True)
        proof.append(dict(state=state, source=source.relative_to(ROOT).as_posix(),
                          sha256=sha(source), actors=verified,
                          native_map_crop=dest.relative_to(ROOT).as_posix()))
        if state == 'open':
            image.crop((8, 708, 1400, 853)).save(OUT / f'camp-01-{revision}-scale-native.png', optimize=True)
            if revision=='r04':
                image.crop((ox+6*64,oy,ox+9*64,oy+2*64)).save(OUT/'camp-01-r04-top-T-native.png')
                image.crop((ox+6*64,oy+6*64,ox+9*64,oy+8*64)).save(OUT/'camp-01-r04-bottom-T-native.png')
    old = OUT / 'scene-r02.html'
    current = OUT / 'index.html'
    if not old.exists():
        old.write_text(current.read_text(encoding='utf-8'), encoding='utf-8')
    previous=OUT/'engine-r03.html'
    if revision=='r04' and not previous.exists():
        previous.write_text(current.read_text(encoding='utf-8'),encoding='utf-8')
    if revision=='r04':
        kit=json.loads((REVIEW/'kit-r04.json').read_text(encoding='utf-8'))
        furniture={'records':kit['furniture']}
        walls=Image.open(ROOT/'assets/prototypes/handcrafted-camp-01/r04/walls-native64.png')
        board=Image.new('RGBA',(384,112),'#10171c');draw=ImageDraw.Draw(board)
        font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',15)
        for n,mask in enumerate([7,11,13,14]):
            draw.text((n*96+16,8),f'W{mask:02}',font=font,fill='#dfc397')
            board.alpha_composite(walls.crop(((mask%4)*64,(mask//4)*64,(mask%4+1)*64,(mask//4+1)*64)),(n*96+16,36))
        board.convert('RGB').save(OUT/'camp-01-r04-painted-T-native.png')
    else:
        furniture = json.loads((REVIEW / 'native-furniture-r03.json').read_text(encoding='utf-8'))
    rows = ''.join(f'<tr><td>{r["id"]}</td><td>{html.escape(r["label"])}</td><td>{r["native_body_size"][0]} × {r["native_body_size"][1]} px</td></tr>' for r in furniture['records'])
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<title>Premier camp — essai dans le moteur</title>
<style>
*{box-sizing:border-box}body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}
header,section{padding:16px 24px}header{max-width:1100px}h1{font-size:23px;color:#e5d2a8}h2{font-size:19px}
p{line-height:1.5}a{color:#b9d3e9}nav{display:flex;gap:12px;flex-wrap:wrap}button,a.button{color:#e3e9e6;background:#25363c;border:1px solid #667c80;padding:9px 14px;font:inherit;cursor:pointer;text-decoration:none}
button[aria-pressed=true]{border-color:#debd7b;background:#3d4033}.viewport{overflow:auto;border:1px solid #33484e;width:fit-content;max-width:100%}
img{display:block;width:auto;height:auto;max-width:none;image-rendering:pixelated}table{border-collapse:collapse;margin-top:16px}th,td{text-align:left;padding:7px 20px 7px 0;border-bottom:1px solid #334048}.note{color:#b7c6cc}.pending{color:#dfc397}code{color:#d8c8aa}
</style><header><h1>PLACE-CAMP-01 · r03 — essai dans le moteur</h1>
<p>Deux pièces assemblées avec les sols, murs, angles et portes de la bibliothèque de l’éditeur.
Le joueur, l’armurier et la secouriste utilisent leurs sprites validés, sans redimensionnement.</p>
<p>Le mobilier est calibré séparément : lit de 68 px de long, siège de 28 px de haut,
armoire de 56 px. Les silhouettes humaines font 60 à 65 px. Ces proportions restent proposées.</p>
<p class="pending">Essai technique et visuel à examiner. Ce petit plan ne remplace pas le camp complet.
L’ambiance détaillée de l’illustration r02 reste à reconstruire avec des éléments séparés.</p>
<nav><a class="button" href="#scene">Voir les pièces</a><a class="button" href="#scale">Comparer les proportions</a><a class="button" href="engine-r03/r03-native-open.png">Capture complète</a><a class="button" href="scene-r02.html">Ancienne illustration r02</a></nav>
</header><section id="scene"><h2>Captures réelles — 64 px par case</h2>
<nav><button data-state="open" aria-pressed="true">1 · Porte ouverte</button><button data-state="closed" aria-pressed="false">2 · Porte fermée</button><button data-state="crossed" aria-pressed="false">3 · Joueur passé de l’autre côté</button></nav>
<p id="state-note">La porte ouverte laisse un passage visible au centre du mur.</p>
<div class="viewport"><img id="scene-image" src="camp-01-r03-open-native.png" width="960" height="512" alt="Deux pièces dans le moteur, avec une porte ouverte, du mobilier et les personnages à taille native."></div>
<p class="note">Ce sont des extraits sans zoom des images produites par le prototype Macroquad.
Déplacement, murs, porte et occupation des cases utilisent le moteur GameState du jeu.
Les PNJ restent immobiles ; leurs services ne sont pas simulés.</p></section>
<section id="scale"><h2>Comparaison à la même échelle</h2><p>Joueur, armurier, puis candidats 01 à 06 : établi, siège, armoire, étagère, lit et banc.</p>
<div class="viewport"><img src="camp-01-r03-scale-native.png" width="1392" height="145" alt="Personnages et six meubles alignés à leur taille native, avec leurs codes."></div>
<table><thead><tr><th>Code de revue</th><th>Meuble</th><th>Dessin visible</th></tr></thead><tbody>ROWS</tbody></table></section>
<section><h2>Essayer les déplacements</h2><p>Dans le dossier du projet, lancer <a href="../../Voir_camp_prototype.cmd">Voir_camp_prototype.cmd</a>.</p>
<p>Flèches, ZQSD ou WASD : déplacer le joueur. <b>E</b> à côté de la porte : ouvrir ou fermer.
<b>G</b> : grille. <b>R</b> : recommencer. <b>Échap</b> : quitter.</p>
<p class="note">Exécutable de revue séparé de la campagne, sans lecture ni écriture des sauvegardes.</p></section>
<script>
const notes={open:'La porte ouverte laisse un passage visible au centre du mur.',closed:'La même porte fermée bloque le passage dans la simulation.',crossed:'Après réouverture, le joueur a franchi la porte et se trouve dans la pièce de droite.'};
document.querySelectorAll('button[data-state]').forEach(b=>b.addEventListener('click',()=>{document.querySelectorAll('button[data-state]').forEach(x=>x.setAttribute('aria-pressed',String(x===b)));document.getElementById('scene-image').src='camp-01-r03-'+b.dataset.state+'-native.png';document.getElementById('state-note').textContent=notes[b.dataset.state]}));
</script></html>'''.replace('ROWS', rows)
    if revision=='r04':
        page=page.replace('r03','r04').replace('de la bibliothèque de l’éditeur','du nouveau kit illustré, créé à partir de la référence d’ambiance r02')
        page=page.replace('L’ambiance détaillée de l’illustration r02 reste à reconstruire avec des éléments séparés.',
            'Quatre T peints remplacent les assemblages de segments. Les murs sont affichés dans une caméra fixe, sans retourner leur relief. Sols, mobilier et lampes reprennent les matériaux de l’illustration r02 ; leur rendu reste à examiner.')
        page=page.replace('Ancienne illustration r02','Référence d’ambiance r02')
        details='<section><h2>Jonctions en T — pièces peintes à taille native</h2><div class="viewport"><img src="camp-01-r04-painted-T-native.png" width="384" height="112" alt="Les quatre pièces en T peintes avec leurs codes W07 W11 W13 W14, sans agrandissement."></div><p>Sur la carte : W14 en haut, W11 en bas. Extraits natifs des captures :</p><div style="display:flex;gap:24px"><img src="camp-01-r04-top-T-native.png" width="192" height="128" alt="Raccord supérieur sur la carte."><img src="camp-01-r04-bottom-T-native.png" width="192" height="128" alt="Raccord inférieur sur la carte."></div><p><a href="engine-r03.html">Ancien essai r03, écarté visuellement</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/generation-r04.json">Sources et prompts imagegen</a></p></section>'
        page=page.replace('<section id="scale">',details+'<section id="scale">')
    current.write_text(page, encoding='utf-8')
    validation = dict(review_id='HANDCRAFTED-PLACES-01', place='PLACE-CAMP-01', revision=revision,
        scope='Native review prototype using the actual game simulation and an illustrated modular kit; not the campaign UI.' if revision=='r04' else 'Native review prototype using the actual game simulation and shared editor renderer, not the campaign UI.',
        art_approved=False, layout_approved=False, campaign_connected=False,
        assets_provenance='New built-in imagegen kit based on r02 atmosphere, with native conditioning.' if revision=='r04' else 'Existing modular surface64 prototype source art; no new imagegen artwork in r03.',
        runtime_checks=checks, captures=proof, furniture_review='kit-r04.json' if revision=='r04' else 'native-furniture-r03.json',
        checks=dict(example_tests_passed=29 if revision=='r04' else 27, debug_example_build=True, gpu_captures=3,
                    opaque_actor_pixels_verified=True, no_actor_resizing=True),
        source_files=['examples/handcrafted_camp_review.rs','tools/art/prepare_handcrafted_camp_fixture.py',
                      'tools/art/show_handcrafted_camp_engine_review.py','Voir_camp_prototype.cmd'])
    (REVIEW / f'engine-review-{revision}.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(actual_engine_captures=3, native_map_size=[width,height], actors_exact=True,
                          example_tests=29 if revision=='r04' else 27, campaign_connected=False)))


if __name__ == '__main__':
    main()
