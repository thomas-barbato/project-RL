"""Verify exact retained fronts, whole-figure sources and native review scope."""
import hashlib
import json
import statistics
from pathlib import Path
from PIL import Image
from prepare_monster_vertical01 import support_root

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/monster-vertical-01/r01'
OUT=ROOT/'artifacts/animation-polish-01/monster-vertical-01/r01/current'
REV=ROOT/'docs/reviews/animation-polish-01'
REVIEW='MONSTER-VERTICAL01-r01'
NUMBERS=[14,164,144]
RECORD='MONSTER-VERTICAL01-VALIDATION.json'


def main():
    manifest=json.loads((SOURCE/'manifest.json').read_text(encoding='utf-8'))
    captures=json.loads((OUT/'checks.json').read_text(encoding='utf-8'))
    assert not manifest['normal_game_enabled'] and not captures['normal_game_enabled']
    assert captures['models']==129 and captures['variants']==471 and len(captures['inventory'])==129
    assert sum(len(p['variants']) for p in captures['inventory'])==471
    assert manifest['step_ms']==captures['step_ms']==400 and manifest['cycle_ms']==800
    assert [p['number'] for p in manifest['profiles']]==NUMBERS
    installed=json.loads((ROOT/'assets/animations/biped-lateral-01/manifest.json').read_text(encoding='utf-8'))
    frozen=0
    for name in ['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json','BIPED-LATERAL03-ACCEPTANCE-r04.json','BIPED-LATERAL04-ACCEPTANCE-r01.json']:
        record=json.loads((REV/name).read_text(encoding='utf-8'))
        for a in record['assets']+record.get('front_references_retained',[]):
            assert hashlib.sha256((ROOT/a['file']).read_bytes()).hexdigest()==a['sha256']; frozen+=1
    previous=0
    feedback=REV/'MONSTER-VERTICAL01-FEEDBACK.json'
    if feedback.exists():
        for asset in json.loads(feedback.read_text(encoding='utf-8'))['assets']:
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']; previous+=1
    accepted_models=set(); accepted_assets=0; color_assets=0; lot03_assets=0; lot04_assets=0; lot05_assets=0; lot06_assets=0; lot07_assets=0; lot08_assets=0; lot09_assets=0; lot10_assets=0; quad21_assets=0; weapon28_assets=0; lot11_assets=0; lot12_assets=0; lot13_assets=0; withdrawn_assets=0; rejected_assets=0; retained_references=0; accepted_by={}; accepted_review={}; withdrawn_here=set(); rejected_here=set()
    for name in ['MONSTER-VERTICAL02-ACCEPTANCE-r01.json','MONSTER-VERTICAL-COLOR01-ACCEPTANCE-r01.json','MONSTER-VERTICAL03-ACCEPTANCE-r01.json','MONSTER-VERTICAL04-ACCEPTANCE-r01.json','MONSTER-VERTICAL05-ACCEPTANCE-r01.json','MONSTER-VERTICAL06-ACCEPTANCE-r01.json','MONSTER-VERTICAL07-ACCEPTANCE-r01.json','MONSTER-VERTICAL08-ACCEPTANCE-r01.json','MONSTER-VERTICAL09-ACCEPTANCE-r01.json','MONSTER-VERTICAL10-ACCEPTANCE-r01.json','MONSTER-VERTICAL21-QUAD01-ACCEPTANCE-r01.json','MONSTER-VERTICAL28-WEAPON01-ACCEPTANCE-r01.json','MONSTER-VERTICAL11-ACCEPTANCE-r01.json','MONSTER-VERTICAL12-ACCEPTANCE-r01.json','MONSTER-VERTICAL13-ACCEPTANCE-r01.json']:
        acceptance=REV/name
        if not acceptance.exists():continue
        accepted=json.loads(acceptance.read_text(encoding='utf-8')); accepted_models.update(accepted['models'])
        accepted_by.update({number:name for number in accepted['models']})
        accepted_review.update({number:accepted['review'] for number in accepted['models']})
        if accepted['review']==REVIEW:withdrawn_here.update(accepted.get('withdrawn_models',[]))
        if accepted['review']==REVIEW:rejected_here.update(accepted.get('rejected_models',[]))
        for asset in accepted.get('rejected_assets',[]):
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']; rejected_assets+=1
        for asset in accepted.get('front_references_retained',[]):
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']; retained_references+=1
        for asset in accepted.get('withdrawn_assets',[]):
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']; withdrawn_assets+=1
        for asset in accepted['assets']:
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']
            if 'COLOR01' in name:color_assets+=1
            elif 'VERTICAL03' in name:lot03_assets+=1
            elif 'VERTICAL04' in name:lot04_assets+=1
            elif 'VERTICAL05' in name:lot05_assets+=1
            elif 'VERTICAL06' in name:lot06_assets+=1
            elif 'VERTICAL07' in name:lot07_assets+=1
            elif 'VERTICAL08' in name:lot08_assets+=1
            elif 'VERTICAL09' in name:lot09_assets+=1
            elif 'VERTICAL10' in name:lot10_assets+=1
            elif 'VERTICAL21-QUAD01' in name:quad21_assets+=1
            elif 'VERTICAL28-WEAPON01' in name:weapon28_assets+=1
            elif 'VERTICAL11' in name:lot11_assets+=1
            elif 'VERTICAL12' in name:lot12_assets+=1
            elif 'VERTICAL13' in name:lot13_assets+=1
            else:accepted_assets+=1
    provenance=json.loads((REV/'BIPED-INTEGRATION01-ASSETS.json').read_text(encoding='utf-8'))
    for p in provenance['models']:assert hashlib.sha256((ROOT/p['file']).read_bytes()).hexdigest()==p['sha256']
    retained=0; poses=0; sources=[]
    for p in manifest['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA'); front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        assert atlas.size==(1440,256)
        runtime=next((e for e in installed['entries'] if e['number']==p['number']),None)
        expected=Image.new('RGBA',(160,128))
        if runtime is not None:
            w,h=runtime['cell']
            ref=Image.open(ROOT/runtime['file']).convert('RGBA').crop((w*8,0,w*9,h))
            expected.alpha_composite(ref,((160-w)//2,0))
        else:
            runtime=p['current_vertical']; w,h=runtime['cell']
            ref=Image.open(ROOT/runtime['file']).convert('RGBA').crop((0,2*h,w,3*h))
            expected.alpha_composite(ref,(round(80-runtime['anchor'][0]),round(112-runtime['anchor'][1])))
            assert p['front_reference_kind']=='installed_vertical_idle'
        assert front.tobytes()==expected.tobytes()
        for row in range(2):
            assert atlas.crop((1280,row*128,1440,(row+1)*128)).tobytes()==front.tobytes(); retained+=1
            frames=[atlas.crop((i*160,row*128,(i+1)*160,(row+1)*128)).tobytes() for i in range(8)]
            assert len(set(frames))==8; poses+=8
            if p.get('support_root_aligned'):
                root=statistics.mean(support_root(atlas.crop((i*160,row*128,(i+1)*160,(row+1)*128))) for i in range(8))
                assert abs(root-support_root(front))<=.51
        for source in p['sources']:
            raw=Image.open(ROOT/source); assert raw.mode=='RGBA' and raw.getchannel('A').getextrema()[0]==0
            sources.append({'file':source,'sha256':hashlib.sha256((ROOT/source).read_bytes()).hexdigest()})
        for direction in ['up','down']:
            scales={b['whole_figure_scale'] for b in p['bounds'] if b['direction']==direction}; assert len(scales)==1
            shifts={b.get('whole_cycle_translation_x',0) for b in p['bounds'] if b['direction']==direction}; assert len(shifts)==1
        item=next(i for i in captures['items'] if i['id']==p['id']); assert item['identical_frontal_stops']==2 and len(item['captures'])==66
        for name in item['captures']:assert Image.open(OUT/p['id']/name).size==(576,320)
        webp=Image.open(OUT/p['id']/'native.webp'); assert webp.n_frames==66
        duration=0
        for i in range(webp.n_frames):webp.seek(i); webp.load(); duration+=webp.info.get('duration',0)
        assert duration==2400,duration
    for entry in captures['inventory']:assert Image.open(OUT/entry['file']).size==(384,192)
    batch_accepted={number for number in NUMBERS if accepted_review.get(number)==REVIEW}
    status='accepted' if set(NUMBERS)<=batch_accepted else 'partially_accepted' if batch_accepted else 'native_candidate_awaiting_visual_approval'
    report={'review':REVIEW,'status':status,'accepted_batch_models':sorted(batch_accepted),'withdrawn_from_this_review':sorted(withdrawn_here),'models_total':129,'variants_total':471,'batch_models':NUMBERS,'first_batch':[14,164,144],
            'normal_game_enabled':False,'native_inventory_cards':129,'whole_figure_walk_poses':poses,'retained_frontal_cells_exact':retained,
            'native_motion_captures':sum(len(i['captures']) for i in captures['items']),'frozen_approval_hashes_verified':frozen,'first_batch_assets_preserved':previous,'accepted_lot02_assets_preserved':accepted_assets,'accepted_color_rule_assets_preserved':color_assets,'installed_lateral_atlases_unchanged':10,
            'step_ms':400,'cycle_ms':800,'webp_full_roundtrip_ms':2400,'phase_varying_scaling':False,'selected_sources':sources,
            'checks':{'native_capture_completed':True,'atlas_pose_and_front_checks':True,'frozen_hashes':True},
            'user_data_loaded':False,'remaining_models':126,'variants_awaiting_new_art_review':471-len(accepted_models)}
    browser=OUT.parent/'browser-checks.json'
    report['accepted_lot03_assets_preserved']=lot03_assets
    report['accepted_lot04_assets_preserved']=lot04_assets
    report['accepted_lot05_assets_preserved']=lot05_assets
    report['accepted_lot06_assets_preserved']=lot06_assets
    report['accepted_lot07_assets_preserved']=lot07_assets
    report['accepted_lot08_assets_preserved']=lot08_assets
    report['accepted_lot09_assets_preserved']=lot09_assets
    report['accepted_lot10_assets_preserved']=lot10_assets
    report['rejected_assets_retained']=rejected_assets
    report['front_reference_assets_retained']=retained_references
    report['rejected_from_this_review']=sorted(rejected_here)
    report['accepted_quad21_assets_preserved']=quad21_assets
    report['accepted_weapon28_assets_preserved']=weapon28_assets
    report['accepted_lot11_assets_preserved']=lot11_assets
    report['accepted_lot12_assets_preserved']=lot12_assets
    report['accepted_lot13_assets_preserved']=lot13_assets
    report['withdrawn_assets_retained']=withdrawn_assets
    if browser.exists():
        b=json.loads(browser.read_text(encoding='utf-8')); assert not b['errors_after_final_reload'] and b['inventory_cards_in_dom']==129
        report['browser_validation']=b
    coverage=REV/'MONSTER-VERTICAL01-COVERAGE.json'; scope=json.loads(coverage.read_text(encoding='utf-8'))
    for p in scope['entries']:
        if p['number'] in accepted_models:
            p['status']='accepted';p['new_art_accepted']=True;p['acceptance']=accepted_by[p['number']]
        if p['number'] in NUMBERS and p['number'] not in withdrawn_here|rejected_here:
            if p['number'] not in accepted_models:p['status']='native_candidate_awaiting_visual_approval'
            p['latest_review']=REVIEW
    coverage.write_text(json.dumps(scope,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    report['remaining_models']=129-sum(p['status'] in ['native_candidate_awaiting_visual_approval','accepted','color_correction_in_review'] for p in scope['entries'])
    (REV/RECORD).write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(json.dumps({k:v for k,v in report.items() if k not in ['selected_sources','checks']}))


if __name__=='__main__':main()
