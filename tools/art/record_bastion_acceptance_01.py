"""Freeze the exact twenty native views approved on 2026-10-21."""
import json
import shutil

from PIL import Image

from prepare_class_readability_review_01 import ROOT, OUT, REVIEW, gallery, sha, write_json
from prepare_bastion_progression_01 import ART, DEST, DOCS, NECK_Y, pixels_hash


def main():
    decision = DOCS/'acceptance.json'
    if decision.exists():
        raise SystemExit('Acceptance already recorded; do not overwrite it.')
    colors = json.loads((DOCS/'all-armor-colors.json').read_text('utf-8'))
    progression = json.loads((DOCS/'progression.json').read_text('utf-8'))
    pilot = json.loads((DOCS/'readability-pilot.json').read_text('utf-8'))
    bases = {e['candidate_id']:e for e in progression['entries'][:4]+[pilot]}
    accepted = ART/'accepted'
    entries = []
    for row in colors['entries']:
        for item in [bases[row['base']]]+row['variants']:
            candidate = item['candidate_id']
            native = ROOT/item['native']
            source = ROOT/item['source']
            assert sha(native)==item['native_sha256']
            assert sha(source)==item['source_sha256']
            image = Image.open(native).convert('RGBA')
            assert image.size==(64, 64)
            assert pixels_hash(image.crop((0, 0, 64, NECK_Y)))==colors['head_region_sha256']
            body = DEST/(candidate+'-body-native64.png')
            files = {}
            for kind, original, folder in [('native', native, 'native64'), ('body_layer', body, 'layers')]:
                target = accepted/folder/original.name
                target.parent.mkdir(parents=True, exist_ok=True)
                assert not target.exists(), target
                shutil.copyfile(original, target)
                assert sha(target)==sha(original)
                files[kind] = {'path':target.relative_to(ROOT).as_posix(), 'sha256':sha(target)}
            entries.append({'candidate_id':candidate, 'model_number':row['number'],
                'color':item.get('color', 'original'), 'source':item['source'],
                'source_sha256':sha(source), **files})
    assert len(entries)==20 and len({e['candidate_id'] for e in entries})==20
    head = ROOT/progression['head_layer']
    head_target = accepted/'layers'/head.name
    shutil.copyfile(head, head_target)
    board = DEST/'all-armors-colors-native64.png'
    board_target = accepted/'review'/board.name
    board_target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(board, board_target)
    acceptance_path = decision.relative_to(ROOT).as_posix()
    colors.update({'status':'accepted_visual_reference', 'acceptance':acceptance_path})
    for row in colors['entries']:
        for variant in row['variants']:
            variant['status']='accepted_visual_reference'
    write_json(DOCS/'all-armor-colors.json', colors)
    progression['status']='first_four_accepted_fifth_replaced_by_P05_r03'
    progression['acceptance']=acceptance_path
    for item in progression['entries']:
        item['status']='accepted_visual_reference' if item['candidate_id'] in bases else 'superseded_by_BA-W01-P05-r03'
    write_json(DOCS/'progression.json', progression)
    pilot.update({'status':'accepted_visual_reference', 'acceptance':acceptance_path})
    write_json(DOCS/'readability-pilot.json', pilot)
    first_colors = json.loads((DOCS/'colors.json').read_text('utf-8'))
    first_colors.update({'status':'accepted_visual_reference', 'acceptance':acceptance_path})
    for item in first_colors['entries']:
        item['status']='accepted_visual_reference'
    write_json(DOCS/'colors.json', first_colors)
    write_json(decision, {
        'acceptance_id':'BASTION-W01-ACCEPT01', 'review_id':'BASTION-W01', 'date':'2026-10-21',
        'user_answer':'ok je valide :)', 'status':'accepted_visual_reference',
        'scope':'Five Bastion full-body armor models and all fifteen red/blue/green variants shown on the native64 board, including the shared head in these compositions.',
        'class_id':'core:bastion', 'models':5, 'colored_variants':15, 'approved_views':20,
        'native_dimensions':[64,64], 'approved_model_ids':list(bases),
        'shared_head':{'path':head_target.relative_to(ROOT).as_posix(), 'sha256':sha(head_target),
            'region':[0,0,64,NECK_Y], 'rgba_region_sha256':colors['head_region_sha256']},
        'approved_board':{'path':board_target.relative_to(ROOT).as_posix(), 'sha256':sha(board_target)},
        'entries':entries, 'runtime_integration':False,
        'remaining':['Map these appearance models to existing item IDs without changing gameplay.',
            'Prepare weapon grips, material masks and in-engine checks.',
            'Review other class armor waves separately.',
            'Any future simplification changes are new revisions requiring their own visual review.'],
        'excluded':['BA-W01-P05-r02', 'old torso-only candidates', 'other class silhouettes and armor waves',
            'new item names or mechanical rules', 'unseen revisions or weapon poses'],
        'checks':{'all_twenty_native64':True, 'shared_head_pixels_identical':True,
            'approved_exports_copied_without_pixel_changes':True, 'sources_unchanged':True}})
    section = (DEST/'all-colors-section.html').read_text('utf-8')
    section = section.replace('La simplification des quatre premières tenues et les masques de palettes du jeu restent en revue.',
        'Les cinq modèles présentés et leurs couleurs sont validés. Toute nouvelle simplification sera revue séparément ; les masques de palettes du jeu restent à produire.')
    section = section.replace('<h3 id="bastion-colors">',
        '<h3 id="bastion-colors">Validé · ')
    (DEST/'all-colors-section.html').write_text(section, 'utf-8')
    (DEST/'section.html').write_text('''<section id="bastion-wave-01">
<h2>Vague 01 · Bastion · validée le 21 octobre 2026</h2>
<p>Accord enregistré : cinq modèles de très léger à très lourd, avec leurs quinze variantes colorées et la même tête. Les vingt vues exactes sont conservées en 64 × 64.</p>
'''+section+'''
<p class="note">Les apparences sont validées. Leur correspondance avec les objets, les prises d’armes et le rendu en jeu restent à préparer. Aucun dessin de cette vague n’est encore intégré au jeu.</p>
<p><a href="../../docs/reviews/class-readability-01/bastion-wave-01/acceptance.json">Accord et empreintes des vingt vues</a> · <a href="../../docs/reviews/class-readability-01/bastion-wave-01/REVIEW.md">Périmètre de la vague validée</a></p></section>''', 'utf-8')
    candidates = json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    candidates['approval_scopes']['body_armor_variants']='Bastion wave 01 accepted: five models and fifteen color variants; other classes pending'
    candidates['latest_steering']='Bastion wave 01 accepted at native64. Preserve the exact twenty views; future drawing revisions require their own review.'
    write_json(REVIEW/'candidates.json', candidates)
    gallery(candidates['entries'], candidates['equipment_inventory'])
    print(json.dumps({'acceptance':acceptance_path, 'models':5, 'colored_variants':15,
        'approved_native_views':len(entries), 'runtime_integration':False}))


if __name__=='__main__':
    main()
