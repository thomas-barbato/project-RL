"""Freeze the five Bastion default heads explicitly approved by the player."""
import json
import shutil

from PIL import Image

from prepare_class_readability_review_01 import ROOT, OUT, REVIEW, SOURCES, gallery, sha, write_json
from prepare_bastion_progression_01 import pixels_hash


def main():
    docs, preview = REVIEW/'bastion-heads-02', OUT/'bastion-heads-02'
    decision = docs/'acceptance.json'
    if decision.exists():
        raise SystemExit('Acceptance already recorded; do not overwrite it.')
    candidates = json.loads((docs/'candidates.json').read_text('utf-8'))
    accepted = SOURCES/'bastion-heads-02/accepted'
    entries = []
    protected = candidates['checks']['all_accepted_bastion_files_preserved']
    assert len(candidates['entries']) == 5
    assert all(sha(ROOT/path) == digest for path, digest in protected.items())
    for item in candidates['entries']:
        native, layer, source = [ROOT/item[key] for key in ('native', 'headgear_layer', 'source')]
        for key, path in [('native', native), ('headgear_layer', layer), ('source', source)]:
            assert sha(path) == item[key+'_sha256']
        sprite = Image.open(native).convert('RGBA')
        overlay = Image.open(layer).convert('RGBA')
        original = Image.open(ROOT/item['approved_body_native']).convert('RGBA')
        assert sprite.size == overlay.size == (64, 64)
        assert overlay.crop((0, 12, 64, 64)).getbbox() is None
        assert sprite.crop((0, 12, 64, 64)).tobytes() == original.crop((0, 12, 64, 64)).tobytes()
        assert pixels_hash(sprite.crop((0, 12, 64, 64))) == item['body_region_rgba_sha256']
        files = {}
        for kind, path, folder in [('native', native, 'native64'), ('headgear_layer', layer, 'layers')]:
            target = accepted/folder/path.name
            target.parent.mkdir(parents=True, exist_ok=True)
            assert not target.exists(), target
            shutil.copyfile(path, target)
            assert sha(target) == sha(path)
            files[kind] = {'path': target.relative_to(ROOT).as_posix(), 'sha256': sha(target)}
        entries.append({'candidate_id': item['candidate_id'], 'number': item['number'],
            'body_candidate': item['body_candidate'], 'source': item['source'],
            'source_sha256': item['source_sha256'], 'body_region': item['body_region'],
            'body_region_rgba_sha256': item['body_region_rgba_sha256'], **files})
        item['status'] = 'accepted_visual_reference'
        item['approved_native'] = files['native']
        item['approved_headgear_layer'] = files['headgear_layer']
    references = []
    for path in sorted(preview.glob('*.png')):
        if path.name.startswith('BA-W01-'):
            continue
        target = accepted/'review'/path.name
        target.parent.mkdir(parents=True, exist_ok=True)
        assert not target.exists(), target
        shutil.copyfile(path, target)
        assert sha(target) == sha(path)
        references.append({'path': target.relative_to(ROOT).as_posix(), 'sha256': sha(target)})
    acceptance_path = decision.relative_to(ROOT).as_posix()
    write_json(decision, {'acceptance_id': 'BASTION-W01-HEAD02-ACCEPT01',
        'review_id': candidates['review_id'], 'date': '2026-10-21',
        'user_answer': 'ok j evalide', 'status': 'accepted_visual_reference',
        'scope': 'Five default head appearances and their shown compositions with the five accepted original-color Bastion armor bodies.',
        'class_id': 'core:bastion', 'native_dimensions': [64, 64],
        'approved_default_heads': 5, 'approved_compositions': 5,
        'approved_candidate_ids': [e['candidate_id'] for e in entries],
        'rule': candidates['rule'],
        'body_acceptance': 'docs/reviews/class-readability-01/bastion-wave-01/acceptance.json',
        'entries': entries, 'review_references': references,
        'provenance': [{'path': (docs/name).relative_to(ROOT).as_posix(), 'sha256': sha(docs/name)}
            for name in ('prompts.json', 'ui-checks.json')],
        'runtime_integration': False, 'new_color_variants': 0,
        'integration_gate': 'Complete and review all armor designs including default heads, separate helmets and weapon hand positions before game integration.',
        'excluded': ['Other class armor or head designs', 'Unseen color combinations or head revisions',
            'Independent loot helmet items', 'Weapon grips or animations', 'In-engine lighting or rendering'],
        'checks': {'all_five_native64': True, 'body_pixels_preserved_below_y': 12,
            'approved_exports_copied_without_pixel_changes': True,
            'all_previous_accepted_bastion_files_unchanged': protected}})
    candidates.update(status='accepted_visual_reference', acceptance=acceptance_path)
    write_json(docs/'candidates.json', candidates)
    policy_path = ROOT/candidates['rule']
    policy = json.loads(policy_path.read_text('utf-8'))
    policy['visual_status'] = 'bastion_default_heads_accepted_other_classes_pending'
    policy['bastion_default_head_appearances'] = 'five_accepted_visual_references'
    policy['bastion_head_acceptance'] = acceptance_path
    write_json(policy_path, policy)
    root = json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    root['approval_scopes']['default_head_appearances'] = 'Bastion five default head appearances accepted; other classes pending'
    root['latest_steering'] = 'Bastion bodies and five default heads accepted at native64. Other class designs, separate helmets and weapon grips still require review. Color variants deferred; no game integration yet.'
    write_json(REVIEW/'candidates.json', root)
    section_path = preview/'section.html'
    section = section_path.read_text('utf-8')
    section = section.replace('Bastion · cinq têtes assorties à examiner', 'Bastion · cinq têtes assorties validées')
    section = section.replace('Ces cinq nouvelles compositions attendent leur validation. Les dessins de tête ne sont pas couverts par l’accord antérieur sur les corps.',
        'Validation enregistrée le 21 octobre 2026 : les cinq têtes et leurs compositions montrées en 64 × 64 sont acceptées. Les corps et leurs validations antérieures sont conservés.')
    section = section.replace('<p>Pas de nouvelle variante de couleur.',
        '<p><a href="../../docs/reviews/class-readability-01/bastion-heads-02/acceptance.json">Accord et références exactes des cinq têtes</a></p>\n<p>Pas de nouvelle variante de couleur.')
    section_path.write_text(section, 'utf-8')
    gallery(root['entries'], root['equipment_inventory'])
    assert all(sha(ROOT/path) == digest for path, digest in protected.items())
    print(json.dumps({'acceptance': acceptance_path, 'default_heads': 5,
        'frozen_native_and_layers': 10, 'review_references': len(references),
        'old_accepted_files_preserved': len(protected), 'runtime_integration': False}))


if __name__ == '__main__':
    main()
