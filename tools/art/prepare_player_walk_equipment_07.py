"""Keep the requested next equipment/bestiary scope explicit after WALK06."""
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
DOC.mkdir(parents=True,exist_ok=True)
baseline=json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf-8'))
configs=[]
for config in baseline['classConfigs']:
    clean={k:v for k,v in config.items() if k!='label'}
    clean['status']='integrated_original_no_extra_helmet' if clean['weapon']==2 and clean['hands'] is None and clean['head']==0 else 'pending_full_pose_adaptation_and_review'
    configs.append(clean)
models=sorted({c['weapon'] for c in configs if c['weapon'] is not None})
assert models==list(range(113))
pack_file=DOC/'equipment-preview-pack.json'
packed=json.loads(pack_file.read_text('utf-8')) if pack_file.exists() else []
ready=len(packed)==5 and all(p['options']==117 and p['armors']==5 for p in packed)
if ready:
    for config in configs:
        config['new_cycle_review_status']='proposal_ready' if config['head']==0 and config['hands'] is None else 'pending_additional_case'
data={'review_id':'WALK07-EQUIPMENT-SCOPE','status':'proposals_ready_not_integrated' if ready else 'scope_prepared_no_new_images_accepted',
    'method':'docs/reviews/player-walk-05/REGLE-MARCHE-LATERALE.md',
    'previous_integrated_scope':'WALK06: model2 only, 25 outfits, Original, no extra helmet',
    'weapon_models':models,'weapon_model_count':len(models),'class_armor_silhouettes':25,
    'single_model_outfit_cases':25*len(models),'configurations':configs,
    'required_additional_cases':['both weapon hands where supported','actual mixed/dual loadouts',
        'all six equipped helmets on each outfit','existing equipment palettes','legacy compatible outfits'],
    'next_after_equipment':'game bestiary, preserving each creature morphology and material variants',
    'vertical_movement_changed':False,'new_lots_must_be_shown_before_integration':True}
if ready:data['new_proposals']={'weapon_models':113,'outfits':25,'configurations_per_outfit':117,
    'loadout_outfit_pairs':2925,'poses':40950,'extra_configurations':['unarmed','dual_pistols','blade_pistol','dual_blades'],
    'creature_cycles':7,'remaining_non_class_catalogue_records':627,'visual_acceptance':False,'runtime_integration':False}
revision_path=DOC/'review-revision.json'
if revision_path.exists():
    revision=json.loads(revision_path.read_text('utf8'))
    data['status']=revision['status']
    data['cycle_revision']=revision
    count=len(revision['repair_pilot_indices'])
    data['new_proposals']={'revision':revision['revision'],'repair_pilot_class_indices':revision['repair_pilot_indices'],
        'repair_pilot_outfits':count,'repair_pilot_loadout_outfit_pairs':count*117,'repair_pilot_poses':count*117*14,
        'accepted_class_indices':revision.get('accepted_class_indices',[]),
        'awaiting_visual_class_indices':revision.get('awaiting_visual_class_indices',revision['repair_pilot_indices']),
        'other_reported_family_outfits_pending':10-count,'weapon_models':113,'runtime_integration':False,
        'checked_family_visual_acceptance':not revision.get('awaiting_visual_class_indices',revision['repair_pilot_indices']),
        'accepted_creature_ids':revision.get('accepted_creature_ids',[]),'full_bestiary_visual_acceptance':False}
(DOC/'equipment-scope.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf-8')
print(json.dumps({'weapon_models':len(models),'single_model_outfit_cases':25*len(models),
    'scope_only':not ready and not revision_path.exists(),'proposal_ready':ready,'cycle_repair_pilots':count if revision_path.exists() else 0},indent=2))
