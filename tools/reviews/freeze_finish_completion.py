"""Freeze the delegated visual decision and its durable proof set."""
import json,shutil
from pathlib import Path
from art_finish_completion import ROOT,ART,DOC,OUT
from art_finish_batch_02 import sha,write

def freeze():
    assert not (DOC/'acceptance.json').exists(),'Completion already frozen'
    checks=json.loads((DOC/'integration-checks.json').read_text('utf-8'))
    assert checks['native_appearances']==471 and checks['previous_resources_unchanged']==630
    log=(OUT/'completion-graphics-tests.log').read_text('utf-8');assert '46 passed; 0 failed' in log
    exports=json.loads((DOC/'exports.json').read_text('utf-8'))['exports']
    jobs=json.loads((ART/'prompts.json').read_text('utf-8'))['jobs'];requests=[]
    for job in jobs:
        if job['number']==113:continue
        receipts=sorted((ART/'generation').glob(job['id']+'*.json'))
        receipts=[p for p in receipts if json.loads(p.read_text('utf-8')).get('id')==job['id']]
        assert receipts,job['id']
        receipts.sort(key=lambda p:p.stat().st_mtime_ns);receipt=json.loads(receipts[-1].read_text('utf-8'))
        requests.append(dict(id=job['id'],number=job['number'],source=Path(job['output']).relative_to(ROOT).as_posix(),sha256=sha(Path(job['output'])),generation_receipt=receipts[-1].relative_to(ROOT).as_posix(),prompt=receipt['prompt'],input_arguments=receipt['referenced_image_paths']))
    for n in [3,4,5]:
        ident=f'FIN-{n:02}-RIFLE';p=ART/'generation'/f'{ident}.json';r=json.loads(p.read_text('utf-8'))
        requests.append(dict(id=ident,source=Path(r['output']).relative_to(ROOT).as_posix(),sha256=sha(Path(r['output'])),generation_receipt=p.relative_to(ROOT).as_posix(),prompt=r['prompt'],input_arguments=[r['source']]))
    assert len(requests)==122
    write(ART/'prompts-used.json',dict(mode='built-in imagegen',active_new_monster_sources=119,new_arm_pose_sources=3,retained_model=113,requests=requests,revision_inputs='Superseded framing sources are preserved as rejected-crop-source.png alongside active source masters; original tool arguments remain in the receipts.'))
    proof=DOC/'proofs';proof.mkdir(exist_ok=True)
    files=list((OUT/'completion-engine').glob('native-*.png'))
    files+=list(OUT.glob('completion-worn-*-native.png'))+list(OUT.glob('completion-worn-palettes-*.png'))
    files+=list(OUT.glob('completion-weapons-*-native.png'))+[OUT/'completion-outfit-icons-native.png']
    files+=list(OUT.glob('completion-*-viewport.png'))+[OUT/'completion-npc-control.png',OUT/'completion-bestiaire-native.png']
    files+=list((OUT/'furniture-audit').glob('*.png'))
    for p in files:shutil.copy2(p,proof/p.name)
    shutil.copy2(OUT/'completion-graphics-tests.log',DOC/'graphics-tests.log')
    shutil.copy2(OUT/'completion-performance/timings.json',DOC/'performance-debug.json')
    write(DOC/'proof-hashes.json',{p.name:sha(p) for p in sorted(proof.glob('*.png'))})
    assets={p.relative_to(ROOT).as_posix():sha(p) for p in (ROOT/'assets/sprites/art-finish-completion-06').rglob('*.png')}
    write(DOC/'accepted-assets.json',assets)
    acceptance=dict(review='ART-FINISH-COMPLETE-06',date='2026-10-20',status='accepted_and_integrated',authority='Codex visual review under explicit user delegation',user_statements=['je te laisse tous les finires','et les valider'],native_cell=64,new_monster_drawings=119,retained_existing_model=113,total_existing_models=129,appearances=471,worn_outfits=19,new_arm_pose_drawings=3,weapon_models=113,outfit_icons=19,camp_npcs_reviewed_and_retained=4,reserved_casters_not_activated=13,gameplay_and_content_ids_unchanged=True,visual_review=['native master comparisons','all current palette appearances','19 outfit material comparisons','left/right and two-handed equipment','eight real-engine street-lighting scenes','camp NPC comparison'],tests=dict(graphics_passed=46,graphics_failed=0,full_suite_not_run=True,debug_build='passed',release_check='passed',format_check='passed',git_diff_check='passed',native_gpu_opaque_pixels=109126,native_gpu_rgb_error=0,gameplay_fingerprints_unchanged=True),furniture='Audit complete; two oversized street beds reported separately, no scenery/collision change in this batch',generation_exception='113 retained after two built-in generator refusals; no further retries',source_mode='built-in imagegen',assets_manifest='accepted-assets.json',proofs='proof-hashes.json')
    checks['status']='accepted_and_integrated';checks['accepted_assets']=len(assets);write(DOC/'integration-checks.json',checks)
    inv=json.loads((DOC/'inventory.json').read_text('utf-8'));inv.update(status='completed',new_monster_drawings=119,retained_existing_models=[113],completed_appearances=471,worn_outfits=19,weapon_models=113,outfit_icons=19,furniture_audit='completed_report_only');write(DOC/'inventory.json',inv)
    write(DOC/'acceptance.json',acceptance)
    print('Frozen:',len(assets),'compiled PNG resources;',len(files),'proof images; 122 active generated source masters')

if __name__=='__main__':freeze()
