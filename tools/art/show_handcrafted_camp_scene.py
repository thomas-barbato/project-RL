"""Compose a scenery review with frozen native actors; never mutate game assets.

Scenery was generated with built-in imagegen. Pillow only conditions its canvas
and composites the accepted actors and existing contact-shadow effect.
"""
import hashlib
import html
import json
from pathlib import Path

from PIL import Image
from review_contact_shadows import contact_shadow

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'
OUT = ROOT / 'artifacts/handcrafted-places-review-01'
SOURCE = ROOT / 'assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/PLACE-CAMP-01-r02-background.png'
CANVAS = (1792, 1152)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    plan = json.loads((REVIEW / 'review.json').read_text(encoding='utf-8'))['records'][0]
    selected = json.loads((REVIEW / 'npc-reference-selection-r02.json').read_text(encoding='utf-8'))
    background = Image.open(SOURCE).convert('RGBA')
    original_size = background.size
    # The generated background is a visual composition, not an engine tilemap.
    # Actor pixels below are always left at their accepted native dimensions.
    scene = background.resize(CANVAS, Image.Resampling.LANCZOS)
    scene.save(OUT / 'camp-01-r02-decor.png', optimize=True)
    actors = []
    for npc, slot in zip(selected, plan['service_slots']):
        assert npc['visual_approved'] is True
        actors.append(dict(id=npc['id'], revision=npc['revision'], label=npc['label'],
                           source=npc['native']['file'], expected_sha256=npc['native']['sha256'],
                           foot=npc['native']['foot_anchor'], at=slot['at']))
    ref = plan['player_reference']
    actors.append(dict(id='PLAYER', revision='AR-N07-0-00', label='Joueur',
                       source=ref['source'], expected_sha256=ref['sha256'],
                       foot=ref['foot_anchor'], at=plan['player_at']))
    for actor in actors:
        source = ROOT / actor['source']
        assert sha(source) == actor['expected_sha256']
        sprite = Image.open(source).convert('RGBA')
        x, y = actor['at']
        foot = actor['foot']
        position = (round((x + .5) * 64 - foot[0]), (y + 1) * 64 - foot[1])
        shadow, shadow_metadata = contact_shadow(sprite, foot)
        scene.alpha_composite(shadow, position)
        scene.alpha_composite(sprite, position)
        actor.update(position=list(position), canvas=list(sprite.size),
                     body_bounds=list(sprite.getbbox()), scale=1,
                     resize_operations=0, contact_shadow=shadow_metadata)
    scene = scene.convert('RGB')
    result = OUT / 'camp-01-r02-native64.png'
    scene.save(result, optimize=True)
    # Verify the saved image, not just the in-memory composition.
    saved = Image.open(result).convert('RGB')
    for actor in actors:
        sprite = Image.open(ROOT / actor['source']).convert('RGBA')
        px, py = actor['position']
        crop = saved.crop((px, py, px + sprite.width, py + sprite.height))
        checked = 0
        for original, actual in zip(sprite.get_flattened_data(), crop.get_flattened_data()):
            if original[3] == 255:
                assert original[:3] == actual
                checked += 1
        actor['opaque_pixels_verified'] = checked
    # Native-size crops let smaller windows inspect each service without zoom.
    rooms = [
        ('S1', 'Missions', 'NPC-01', (192, 64, 736, 432)),
        ('S2', 'Armurerie', 'NPC-02', (1024, 64, 1568, 432)),
        ('S3', 'Soins', 'NPC-03', (192, 688, 736, 1040)),
        ('S4', 'Atelier et coffre', 'NPC-04', (1024, 688, 1568, 1040)),
    ]
    room_links = []
    for code, label, npc, bounds in rooms:
        name = f'camp-01-r02-{code.lower()}.png'
        saved.crop(bounds).save(OUT / name, optimize=True)
        room_links.append(f'<figure><figcaption>{code} · {html.escape(label)} · {npc}</figcaption><img src="{name}" alt="{html.escape(label)} avec le PNJ validé à taille native."></figure>')
    page = '''<!doctype html>
<html lang="fr"><meta charset="utf-8"><title>Premier camp — décors et PNJ</title>
<style>
body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}
header{padding:16px 24px;max-width:1080px}h1{font-size:22px;color:#e5d2a8}
a{color:#b9d3e9;margin-right:20px}p{line-height:1.5}section{padding:12px 0;scroll-margin-top:12px}
h2{font-size:18px;margin-left:24px}img{display:block;max-width:none;width:auto;height:auto;image-rendering:pixelated}
.rooms{display:flex;flex-wrap:wrap;gap:18px;padding:0 24px}.rooms figure{margin:0;border:1px solid #36464c}
figcaption{padding:10px 12px;background:#1b282f}.note{color:#bccbd1}
</style>
<header><h1>PLACE-CAMP-01 · r02 — décors et PNJ</h1>
<p>Premier aperçu habillé du camp : poste de missions, armurerie, soins, atelier et coffre.
Le joueur et les quatre PNJ reprennent exactement leurs sprites validés, sans agrandissement.</p>
<p class="note">Composition visuelle à examiner. Le décor et son implantation restent proposés ;
ceci n’est pas une capture du moteur. La géométrie illustrée diffère légèrement du schéma et ne définit pas les collisions.</p>
<p class="note">À corriger après ton retour : les accès paraissent murés et le mobilier est trop grand par rapport aux personnages. La fidélité des sprites ne valide pas l’échelle du décor.</p>
<p><a href="#scene">Camp complet · taille native</a><a href="#rooms">Détails · taille native</a><a href="#plan">Plan précédent</a><a href="camp-01-r02-native64.png">Ouvrir l’image complète</a></p>
</header><section id="scene"><h2>Camp complet — personnages à taille réelle</h2>
<img src="camp-01-r02-native64.png" width="1792" height="1152" alt="Camp industriel avec son passage central, quatre pièces aménagées, les quatre PNJ de services et le joueur à taille native."></section>
<section id="rooms"><h2>Chaque espace — extraits sans zoom</h2><div class="rooms">ROOMS</div></section>
<section id="plan"><h2>r01 — schéma de départ, conservé pour comparaison</h2>
<img src="camp-01-overview.png" alt="Schéma initial avec quatre espaces de services.">
<p style="margin-left:24px"><a href="camp-01-native64.png">Schéma initial à 64 px par case</a></p></section></html>
'''.replace('ROOMS', '\n'.join(room_links))
    (OUT / 'index.html').write_text(page, encoding='utf-8')
    validation = dict(place='PLACE-CAMP-01', revision='r02',
        source_background=dict(file=SOURCE.relative_to(ROOT).as_posix(), sha256=sha(SOURCE),
                               original_canvas=list(original_size), conditioned_canvas=list(CANVAS),
                               resampling='LANCZOS, environment only'),
        actors=actors, actor_count=len(actors), actor_resize_operations=0,
        all_actor_opaque_pixels_match_sources=True, contact_shadows_separate=True,
        layout_approved=False, art_approved=False, player_texts_approved=False,
        runtime_connected=False, new_gameplay_effects=False,
        geometry_scope='Visual illustration only; original logical room arrangement retained, exact generated wall coordinates differ. No tile collision proof.',
        output=dict(file=result.relative_to(ROOT).as_posix(), size=list(saved.size), sha256=sha(result)))
    (REVIEW / 'scene-validation-r02.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(dict(actors=len(actors), native_actor_pixels_verified=True,
                          actor_resize_operations=0, scene_size=list(saved.size),
                          runtime_connected=False), ensure_ascii=False))


if __name__ == '__main__':
    main()
