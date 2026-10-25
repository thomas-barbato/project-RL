"""Record the explicit25 October2026 approval without repainting accepted pixels."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/player-walk-12'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    destination = DOC / 'acceptance.json'
    if destination.exists():
        approved = json.loads(destination.read_text())
        assert all(sha(ROOT / name) == digest for name, digest in approved['runtime_asset_sha256'].items())
        print('WALK12 approval already recorded; accepted images unchanged')
        return
    reviewed = json.loads((DOC / 'validation.json').read_text())
    assets = json.loads((DOC / 'asset-verification.json').read_text())
    manifest_path = ROOT / 'assets/animations/walk-12/player/manifest.json'
    assert sha(manifest_path) == reviewed['proposal']['manifest_sha256']
    manifest = json.loads(manifest_path.read_text())
    files = sorted({e['file'] for e in manifest} | {'assets/animations/walk-12/player/manifest.json'})
    assert len(files) == 51 and len(manifest) == 350
    sources = []
    for name, digest in assets['source_hashes'].items():
        assert sha(ROOT / name) == digest, name
        sources.append({'path': name, 'sha256': digest})
    approval = {
        'review_id': 'WALK12-COMMON-r01', 'accepted_date': '2026-10-25',
        'user_quote': 'c\'est bien je pense, à noter que le fait que le personnage se remet "de face" à la fin de l\'animation, ca doit aussi concerner les monstres et les npc de facon générale :) enfn donc je valide :D',
        'runtime_integration_authorized': True, 'integration_verification': 'pending',
        'scope': {'player_classes': 5, 'outfits': 25, 'whole_walking_drawings': 300,
                  'reference_idles': 50, 'shared_weapon_models': 113,
                  'hands_and_helmets': 'actual equipped layers; rigid grips throughout the step',
                  'arrival': 'all actors: player, monsters and NPCs return to their canonical static sprite with equipment',
                  'vertical_player_drawings_unchanged': True,
                  'default_milliseconds_per_cell': 400, 'selectable_milliseconds': [120, 200, 400],
                  'distance_cells': 1, 'native_cell_px': 64},
        'excluded_pending_reviews': ['WALK08 nine form/drone walking cycles'],
        'runtime_asset_sha256': {name: sha(ROOT / name) for name in files},
        'selected_whole_figure_sources': sources,
        'inherited_accepted_review': 'docs/reviews/player-walk-11/acceptance.json',
        'review_validation': 'validation.json', 'integration_validation': 'integration-validation.json'}
    destination.write_text(json.dumps(approval, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(json.dumps({'approval': approval['review_id'], 'runtime_assets': len(files), 'sources': len(sources)}))


if __name__ == '__main__':
    main()
