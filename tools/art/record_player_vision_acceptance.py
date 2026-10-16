"""Record the explicit review approval; preserve historical capture evidence."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
OUT = BASE / 'skill-spatial-review-01'
read = lambda p: json.loads(p.read_text(encoding='utf8'))
def write(p, data):
    p.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf8')

approval = {
    'review_id': 'ZONE-VISION02', 'date': '2026-10-14',
    'user_reply': 'ok ca me semble bien :) on peut continuer',
    'review': 'skill-spatial-review-01/index.html',
    'numeric_radius_approved': True, 'displayed_circle_approved': True,
    'radius_cells': 12, 'growth_per_skill_rank': False,
    'integration_scope': {
        'new_games_and_current_laboratory': 12,
        'current_generation_version': 147,
        'previous_generations_through_146': 'Retain radius 8 and historical rule fingerprints.',
        'ordinary_npc_radius': 8, 'specific_npc_profiles': 'unchanged',
        'occlusion_and_circular_distance': 'unchanged',
        'existing_save_files_modified': False,
    },
    'other_animation_approvals': False,
    'compatibility_validation': '2 player_vision_revision_tests passed, including version-146 suspension restoration.',
}
write(OUT/'vision-acceptance-01.json', approval)
report = read(OUT/'progressions.json')
report['vision'].update(campaign_integrated=True, numeric_and_visual_approval_pending=False,
                       integration_scope=approval['integration_scope'], acceptance='vision-acceptance-01.json')
write(OUT/'progressions.json', report)
discussion = read(BASE/'zone-growth-melee-limit-discussion-01.json')
discussion['vision_revision'].update(new_radius=12, numeric_and_visual_approval_pending=False,
    campaign_changed=True, numeric_and_visual_acceptance=approval)
discussion['current_preview'] = 'Progressions de 46 compétences en revue spatiale ; cercle de vision 12 validé. La revue animée 71 reste distincte.'
write(BASE/'zone-growth-melee-limit-discussion-01.json', discussion)
write(OUT/'integration-validation.json', {
    'date': '2026-10-14', 'vision_in_current_campaign': 12,
    'legacy_vision': 8, 'ordinary_npc_vision': 8, 'generation_version': 147,
    'player_vision_revision_tests': 2, 'locked_all_target_check': 'passed',
    'explicit_approval': 'vision-acceptance-01.json', 'save_schema_changed': False,
    'generic_engine_default_changed': False,
    'historical_preapproval_validation': 'validation.json',
})
# Historical PNGs keep their original review captions. Current UI and generator
# state the accepted status so rebuilding a gallery cannot revert the decision.
for path in (ROOT/'tools/art/package_skill_spatial_review.py', OUT/'index.html'):
    text = path.read_text(encoding='utf8')
    text = text.replace('cercle de vision proposé à 12 cases', 'cercle de vision validé à 12 cases')
    text = text.replace('Proposition · 12 cases', 'Validé · 12 cases')
    text = text.replace('Le rayon de vision 12 reste une proposition à valider avant intégration au jeu.',
        'Le rayon de vision 12 est validé et intégré aux nouvelles parties et au laboratoire. Les anciennes sauvegardes gardent leur rayon de huit cases.')
    path.write_text(text, encoding='utf8')
print('Vision 12 validée ; générations historiques conservées.')
