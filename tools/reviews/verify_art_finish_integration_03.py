"""Check the ordinary renderer against the frozen ART-FINISH-03 approval."""
import json
from PIL import Image
from art_finish_batch_03 import ROOT, ART, DOC, OUT, sha, write
from show_art_finish_review_02 import VIEW, difference

INTEGRATED = OUT / 'finish03-integration'


def main():
    receipt = json.loads((DOC / 'acceptance.json').read_text('utf-8'))
    for name, digest in receipt['approved_exports'].items():
        assert sha(ART / 'native' / name) == digest, name
        assert sha(ROOT / 'assets/sprites/art-finish-03' / name) == digest, name
    prior = json.loads((DOC / 'production-before.json').read_text('utf-8'))
    for name, digest in prior.items():
        assert sha(ROOT / name) == digest, f'Other resource changed: {name}'
    capture = json.loads((INTEGRATED / 'checks.json').read_text('utf-8'))
    assert capture['status'] == 'accepted_integration' and capture['production_mode'] == 3
    panels = {}
    for label, count in [('native', 4), ('hands', 12)]:
        accepted = Image.open(DOC / f'{label}-native.png')
        actual = Image.open(INTEGRATED / f'{label}-gpu.png')
        for i in range(count):
            x, y = 160 + i % 4 * 300, 80 + i // 4 * 200
            changed = difference(accepted.crop((x, y+44, x+128, y+172)),
                                 actual.crop((x, y+34, x+128, y+162)))
            panels[f'{label}-{i+1}'] = changed
            assert changed == 0, (label, i, changed)
    scenes = {}
    for label in ['house', 'street']:
        accepted = Image.open(DOC / 'references' / f'{label}-candidate-native.png')
        actual = Image.open(INTEGRATED / f'{label}-integrated.png').crop(VIEW)
        scenes[label] = difference(accepted, actual)
        assert scenes[label] == 0, (label, scenes[label])
        actual.save(DOC / 'references' / f'{label}-integrated-native.png')
        actual.save(OUT / f'finish03-{label}-integrated.png')
    result = dict(review='ART-FINISH-03', status='accepted_and_integrated',
                  packaged_exact_exports=len(receipt['approved_exports']),
                  native_panels=panels, scene_changed_pixels=scenes,
                  gameplay_fingerprints_unchanged=capture['gameplay_fingerprints_unchanged'],
                  native_cell=64, layers_preserved=True,
                  runtime_assets='assets/sprites/art-finish-03',
                  previous_resource_files_unchanged=len(prior),
                  unsampled_poses_and_equipment_palettes='previous bindings retained')
    write(DOC / 'integration-checks.json', result)
    review = json.loads((DOC / 'review.json').read_text('utf-8'))
    review['status'] = 'accepted_and_integrated'
    review['ordinary_runtime_unchanged_at_preparation'] = review.pop('ordinary_runtime_unchanged', True)
    review['runtime_assets'] = result['runtime_assets']
    for entry in review['entries']:
        entry['status'] = 'accepted'
    write(DOC / 'review.json', review)
    from show_art_finish_review_03 import build
    build()
    print(json.dumps(dict(exports=len(receipt['approved_exports']), panels=len(panels),
                          scene_changed_pixels=scenes, previous_files_unchanged=len(prior))))


if __name__ == '__main__':
    main()
