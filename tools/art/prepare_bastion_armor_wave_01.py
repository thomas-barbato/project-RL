"""Package generated art for one class wave; never changes active game assets."""
from pathlib import Path
import html
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, STREET, font, native_image, sha

WAVE = 'bastion-wave-01'
DEST, DOCS, ART = OUT/WAVE, REVIEW/WAVE, SOURCES/WAVE


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', 'utf-8')


def comparison(images, labels, path, title):
    board = Image.new('RGB', (384, 348), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((12, 10), title, font=font(14), fill='#efddbd')
    width = 384//len(images)
    for i, label in enumerate(labels):
        draw.text((i*width+8, 38), label, font=font(11), fill='#d3dedc')
    for row, (label, color) in enumerate([('Sol sombre', '#343b3f'), ('Sol clair', '#858478'), ('Pénombre', '#12181d')]):
        y = 72+row*90
        draw.rectangle((8, y-3, 376, y+66), fill=color)
        for i, sprite in enumerate(images):
            board.paste(sprite, (i*width+(width-64)//2, y), sprite)
        draw.text((12, y+68), label, font=font(11), fill='#acbfc3')
    board.save(path)


def main():
    for folder in (DEST, DOCS, ART):
        folder.mkdir(parents=True, exist_ok=True)
    if (DOCS/'progression.json').exists():
        raise SystemExit('This torso-only draft is historical. Use prepare_bastion_progression_01.py for the current full-body review.')
    if (DOCS/'acceptance.json').exists():
        raise SystemExit('Accepted wave is frozen; create a new revision.')
    spec = json.loads((DOCS/'prompts.json').read_text('utf-8'))
    active_before = {str(p):sha(p) for p in (
        ROOT/'assets/sprites/art-finish-01/FIN-01.png', ROOT/'content/core/classes/bastion.json5',
        ROOT/'content/core/items/veste_de_veille.json5', ROOT/'content/core/items/combinaison_fantome.json5',
        ROOT/'content/core/items/plastron_bastion.json5',
    )}
    original, original_measurements = native_image(SOURCES/'CL01-r01-source.png', 58)
    entries, images = [], [original]
    for number, key in enumerate(('first', 'second', 'third'), 1):
        entry = spec[key]
        source = ART/(entry['id']+'-source.png')
        shutil.copyfile(entry['generated_file'], source)
        sprite, measurements = native_image(source, 58)
        destination = DEST/(entry['id']+'-native64.png')
        sprite.save(destination)
        images.append(sprite)
        entries.append({'number':number, 'candidate_id':entry['id'], 'item_id':entry['item_id'],
            'name':entry['name'], 'class_id':'core:bastion', 'status':'preliminary_torso_waiting_for_head_review',
            'source':source.relative_to(ROOT).as_posix(), 'source_sha256':sha(source),
            'native':destination.relative_to(ROOT).as_posix(), 'native_sha256':sha(destination),
            'measurements':measurements, 'head_reference':'CL01-r01',
            'production_layers_and_weapon_anchors':'not_produced'})
    heads = []
    for revision in spec['head_revisions']:
        source = SOURCES/(revision['id']+'-source.png')
        shutil.copyfile(revision['generated_file'], source)
        sprite, measurements = native_image(source, 58)
        destination = DEST/(revision['id']+'-native64.png')
        sprite.save(destination)
        heads.append({'candidate_id':revision['id'],
            'status':{'CL01-r02':'not_selected_headset_reading', 'CL01-r03':'not_selected_insufficient_native_readability'}.get(revision['id'], 'awaiting_visual_approval'),
            'source':source.relative_to(ROOT).as_posix(), 'source_sha256':sha(source),
            'native':destination.relative_to(ROOT).as_posix(), 'native_sha256':sha(destination),
            'measurements':measurements})
    proposed_head = Image.open(ROOT/heads[-1]['native']).convert('RGBA')
    comparison([original, proposed_head], ['01 · précédent r01', '01 · proposé r04'],
        DEST/'implant-native64.png', 'Bastion · implant · 64 × 64 natifs')
    comparison(images, ['Base r01', '01 · Technicien', '02 · Fantôme', '03 · Bastion'],
        DEST/'armures-native64.png', 'Bastion · premiers torses · 64 × 64')
    street = Image.open(STREET).convert('RGBA')
    board = Image.new('RGB', (384, 180), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((12, 8), 'Même sol du jeu · montage natif', font=font(14), fill='#efddbd')
    board.paste(street.crop((80, 160, 400, 288)), (32, 35))
    for i, sprite in enumerate(images):
        board.paste(sprite, (40+i*80, 67), sprite)
    board.save(DEST/'armures-scene-native64.png')
    zoom = Image.new('RGB', (440, 252), '#243036')
    draw = ImageDraw.Draw(zoom)
    draw.text((12, 8), 'Zoom entier ×3 · mêmes pixels', font=font(16), fill='#efddbd')
    for i, (sprite, label) in enumerate(((original, 'Précédent r01'), (proposed_head, 'Proposition r04'))):
        enlarged = sprite.resize((192, 192), Image.Resampling.NEAREST)
        zoom.paste(enlarged, (14+i*220, 36), enlarged)
        draw.text((20+i*220, 228), label, font=font(12), fill='#c5d3d4')
    zoom.save(DEST/'implant-zoom-x3.png')
    assert all(sha(Path(path))==digest for path, digest in active_before.items())
    write_json(DOCS/'candidates.json', {
        'review_id':'BASTION-W01', 'date':'2026-10-21', 'status':'awaiting_individual_visual_review',
        'class_id':'core:bastion', 'tile_size':64,
        'workflow':{'one_class_per_wave':True, 'human_review_before_next_wave':True,
                    'next_class_started':False, 'no_automatic_visual_acceptance':True},
        'current_priority':'CL01-r04 cranial implant readability; do not propagate before its review',
        'head_revisions':heads, 'torso_candidates':entries,
        'base_reference':{'id':'CL01-r01', 'measurements':original_measurements},
        'approval_scopes':{'implant_idea':'appreciated_by_user', 'implant_r04_pixels':'pending',
                           'three_torso_interpretations':'pending', 'new_items_or_stats':'none'},
        'checks':{'all_sprites_native64':True, 'aspect_ratio_preserved':True,
            'source_alpha_retained':True, 'foot_baseline_y':61,
            'game_assets_and_content_unchanged':active_before,
            'source_changes_made_by':'built-in image_gen', 'runtime_changed':False,
            'in_engine_capture':False, 'scene_is_montage_not_engine_lighting_validation':True,
            'production_masks_anchors_and_poses_validated':False}})
    cards = ''.join(f'<li><strong>{e["number"]:02d} · {html.escape(e["name"])}</strong> — {e["candidate_id"]}</li>' for e in entries)
    section = '''<section id="bastion-wave-01"><h2>Vague 01 · Bastion</h2>
<p>Un personnage à la fois. Cette vague attend ta revue avant toute suite.</p>
<h3>01 · Implant crânien : rendre l’augmentation compréhensible</h3>
<p class="note">L’idée de l’implant est appréciée ; la petite plaque isolée sur le front est jugée difficile à comprendre. La r04 essaie une zone métallique plus large sur le côté du visage et un œil mécanique, avec l’oreille découverte. Son rendu reste à examiner.</p>
<div class="native-scroll"><img class="board" src="bastion-wave-01/implant-native64.png" width="384" height="348" alt="Bastion : implant précédent r01 et proposition r04, chacun en 64 × 64 sur trois fonds"></div>
<details><summary>Zoom entier ×3 de la comparaison</summary><div class="native-scroll"><img class="board" src="bastion-wave-01/implant-zoom-x3.png" width="440" height="252" alt="Zoom entier ×3 des mêmes pixels, implant précédent et proposé"></div></details>
<h3>Trois premiers torses · propositions préparatoires</h3>
<p class="note">Ces trois essais ont été dessinés sur la tête r01. Ils attendent le choix de l’implant avant toute reprise de la tête. Corps massif et bras puissants du Bastion ; aucun nouveau modèle statistique.</p>
<ul>'''+cards+'''</ul>
<div class="native-scroll"><img class="board" src="bastion-wave-01/armures-native64.png" width="384" height="348" alt="Bastion sans armure, Tenue du technicien, Tenue Fantôme et Armure Bastion à taille native"></div>
<details><summary>Comparer les torses sur le sol du jeu</summary><p class="note">Montage sur une zone vide d’une capture acceptée. Aucun nouvel éclairage moteur n’est simulé.</p><div class="native-scroll"><img class="board" src="bastion-wave-01/armures-scene-native64.png" width="384" height="180" alt="Les trois torses et leur base r01, montage à taille native sur la rue"></div></details>
<p class="note">Dessins de revue uniquement. Les calques de production, points de prise, poses d’armes et contrôles moteur restent à produire après le choix des apparences.</p>
<p><a href="../../docs/reviews/class-readability-01/bastion-wave-01/REVIEW.md">Périmètre de cette vague</a> · <a href="../../docs/reviews/class-readability-01/bastion-wave-01/prompts.json">Prompts et références</a></p></section>'''
    (DEST/'section.html').write_text(section, 'utf-8')
    print(json.dumps({'review_id':'BASTION-W01', 'armor_candidates':len(entries),
                      'head_candidate':heads[-1]['candidate_id'], 'active_game_changed':False}))


if __name__=='__main__':
    main()
