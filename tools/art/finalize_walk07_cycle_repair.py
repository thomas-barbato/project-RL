"""Record the actual correction scope without accepting or integrating it."""
import hashlib,json
from pathlib import Path
from PIL import Image
from build_player_walk_matrix_06 import grid_figures
from pack_walk07_cycle_repair import strip
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07'
read=lambda name:json.loads((DOC/name).read_text('utf8'))
write=lambda name,value:(DOC/name).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n','utf8')
assert not read('review-revision.json')['revision'].startswith('r05'), 'Use verify_walk07_frozen_gait.py for the preserved-pixel revision'
jobs=read('unarmed-jobs.json')+read('creature-jobs.json');sources=[]
for job in jobs:
    if job.get('direction_sources'):
        for side in job['direction_sources'].values():
            strip(side['cycle'],8);strip(side['opposite'],3)
        files=[file for side in job['direction_sources'].values() for file in side.values()]
    else:files=[job['source_destination']]
    for file in files:
        p=Path(file);sources.append({'id':job['id'],'file':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
write('source-manifest.json',{'source_tool':'built-in image_gen','selected_source_count':len(sources),'runtime_integration':False,'entries':sources})
packing=read('cycle-repair-packing.json');checks=read('renderer-checks.json')
regressions=[]
for index in [10,24]:
    path=ROOT/f'assets/source-art/player-walk-review-07/UNARMED-{index:02}-r01.png'
    figures,_=grid_figures(Image.open(path).convert('RGBA'))
    regressions.append({'id':f'UNARMED-{index:02}','old_source_heights':[im.height for im in figures],
        'new_native_height_range':next(j['height_range_native'] for j in packing if j['id']==f'UNARMED-{index:02}')})
rejected=read('cycle-repair-rejected.json')
for job in rejected:
    try:strip(job['file'],8)
    except AssertionError:pass
    else:raise AssertionError('Two-row atlas incorrectly accepted as one direction')
write('cycle-repair-checks.json',{'date':'2026-10-24','review_id':'WALK07-r04','runtime_integration':False,
    'pilots':[10,24],'other_reported_family_outfits_pending':8,'old_source_regressions':regressions,
    'native_height_span_limit':2,'new_strips_complete_and_padded':True,'whole_figures_only':True,
    'rejected_two_row_atlases_blocked':2,
    'manual_source_and_native_board_inspection':{'left_faces_left':True,'right_faces_right':True,'opposite_support_half_drawn':True},
    'equipment_export_recipe_test':'passed for both pilots, 117 configurations each',
    'equipment_export_raster_test':'passed for both pilots, 117 configurations each',
    'accepted_runtime_renderer_tests_passed':6,'javascript_checks':checks,'user_visual_acceptance':False})
validation=read('validation.json')
validation.update({'review_id':'WALK07-r04','date':'2026-10-24','status':'two_cycle_repair_pilots_awaiting_visual_review',
    'whole_body_outfits':2,'whole_body_pose_entries':28,'loadout_outfit_pairs':234,'equipment_poses':3276,
    'other_reported_family_outfits_pending':8,'source_bank_outfits_total':25,
    'native_height_ranges':{j['id']:j['height_range_native'] for j in packing},'previous_publication_withdrawn_for_reported_families':True})
validation['checks'].update({'rust_vs_preview_pixel_fixtures':checks['pixel_fixtures'],
    'all_equipment_and_creature_selections_checked':checks['selection_pairs_checked'],
    'complete_figures_and_native_height_checks':'passed for two pilots; artistic gait still awaiting user review'})
write('validation.json',validation)
print(json.dumps({'pilots':2,'real_weapon_models':113,'loadouts':234,'poses':3276,
    'native_height_ranges':validation['native_height_ranges'],'user_visual_acceptance':False}))
