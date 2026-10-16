"""Record the user's scoped art approval without changing issued review files."""
from pathlib import Path
import hashlib, json

ROOT=Path(__file__).resolve().parents[2]
BASE=Path(__file__).resolve().parent
def jref(p):
    return {'reference':p.relative_to(ROOT).as_posix(),
            'reference_sha256':hashlib.sha256(p.read_text(encoding='utf-8').encode()).hexdigest()}
def write(p,data):
    assert not p.exists(),p
    p.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')

bank=BASE/'focalizer-review-01/review-focalizers-FOCAL-VIS01-01.json'
data=json.loads(bank.read_text(encoding='utf-8'))
approval=BASE/'focalizer-visual-acceptance-01.json'
write(approval,{
    'date':'2026-10-10','review_id':'FOCAL-VIS01-ACCEPT01',
    'user_acceptance':"j'aime bien ces models, je les valides",
    'accepted_presentation_url':'http://127.0.0.1:59134/focalizer-gallery-01.html',
    'accepted_bank':jref(bank),
    'approved_candidates':[{k:r[k] for k in ('candidate_id','held_layer','inventory_tile','source')} for r in data['candidates']],
    'approval_scope':'Six displayed focalizer drawings and their concrete hand placements on the five fixed class bodies; reviewed one-handed combinations.',
    'approved_placements':len(data['placements']),
    'reviewed_compositions':len(data['compositions']),
    'item_names_RP_stats':'not_defined_or_approved_by_this_visual_acceptance',
    'runtime_integration':'not_integrated;art_review_bank_accepted',
    'historical_review_files':'unchanged;this_separate_record_supersedes_pending_visual_status'
})
previous=BASE/'review-equipment-series-EQUIP-SERIES01-04.json'
plan=json.loads(previous.read_text(encoding='utf-8'))
plan.update(revision=5,previous_proposal=jref(previous),
            current_focalizer_acceptance=jref(approval),
            pixel_approval='shield_revision7_and_focalizer_revision1_accepted_separately;channeling_shafts_and_heavy_art_pending')
for series in plan['series']:
    if series['family_id']=='caster:focalizer':
        series['model_art']='six_drawings_and_concrete_placements_accepted;FOCAL-VIS01-ACCEPT01'
write(BASE/'review-equipment-series-EQUIP-SERIES01-05.json',plan)
doc=ROOT/'docs/REVUE_TEXTES_EFFETS_AGENT.md'
with doc.open('a',encoding='utf-8',newline='\n') as f:
    f.write('''
### FOCAL-VIS01-ACCEPT01 — Six focaliseurs validés

Le joueur valide les modèles présentés dans `focalizer-gallery-01.html` :
« j'aime bien ces models, je les valides ». Cet accord porte sur les six
dessins EQ-F01 à EQ-F06 et les prises concrètes montrées sur les cinq
silhouettes, avec les combinaisons à une main de la révision 1.

La banque et ses empreintes restent inchangées ; un enregistrement distinct
conserve cet accord. Les noms d'objets, textes RP et statistiques restent à
définir séparément. La production se poursuit avec six hampes de canalisation
à deux mains EQ-H01 à EQ-H06, selon la direction déjà acceptée.

Accord : [FOCAL-VIS01-ACCEPT01](../artifacts/refonte-agent-2026-10-06/focalizer-visual-acceptance-01.json).
Inventaire : [EQUIP-SERIES01, révision 5](../artifacts/refonte-agent-2026-10-06/review-equipment-series-EQUIP-SERIES01-05.json).
''')
print(json.dumps({'approved_candidates':6,'placements':len(data['placements']),'historical_bank_unchanged':True}))
