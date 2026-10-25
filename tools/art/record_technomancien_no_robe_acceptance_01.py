"""Freeze the five robe-free Technomancien outfits explicitly approved at native64."""
import json
import shutil
from PIL import Image
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,gallery,sha,write_json


def main():
    name='technomancien-no-robe-heads-02'
    docs,preview=REVIEW/name,OUT/name
    decision=docs/'acceptance.json'
    if decision.exists():raise SystemExit('Acceptance already recorded; do not overwrite.')
    data=json.loads((docs/'candidates.json').read_text('utf-8'))
    checks=json.loads((docs/'checks.json').read_text('utf-8'))
    prior=json.loads((docs/'prior-images.json').read_text('utf-8'))
    assert len(data['entries'])==5 and checks['non_head_native_pixel_changes']==[0]*5
    assert checks['ui_execution']=='verified_in_app_browser'
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    frozen=SOURCES/name/'accepted'

    def freeze(path,folder):
        target=frozen/folder/path.name
        assert not target.exists(),target
        target.parent.mkdir(parents=True,exist_ok=True)
        shutil.copyfile(path,target);assert sha(target)==sha(path)
        return {'path':target.relative_to(ROOT).as_posix(),'sha256':sha(target)}

    snapshot=docs/'accepted-review/candidates-as-reviewed.json'
    snapshot.parent.mkdir(parents=True,exist_ok=True)
    assert not snapshot.exists();shutil.copyfile(docs/'candidates.json',snapshot)
    entries=[]
    for e in data['entries']:
        source,native=ROOT/e['source'],ROOT/e['native']
        assert sha(source)==e['source_sha256'] and sha(native)==e['native_sha256']
        assert sha(preview/(e['candidate_id']+'-native64.png'))==sha(native)
        image=Image.open(native).convert('RGBA')
        assert image.size==(64,64) and image.getchannel('A').getextrema()==(0,255)
        layers={}
        for part,ref in e['layers'].items():
            path=ROOT/ref['path'];assert sha(path)==ref['sha256'];layers[part]=freeze(path,'layers')
        head=Image.open(ROOT/e['layers']['head']['path']).convert('RGBA')
        body=Image.open(ROOT/e['layers']['body']['path']).convert('RGBA')
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==image.tobytes()
        entries.append({'number':e['number'],'candidate_id':e['candidate_id'],'rank':e['rank'],'label':e['label'],
            'source':freeze(source,'sources'),'native':freeze(native,'native64'),'components':layers,
            'head_edit_reference':freeze(ROOT/e['generated_head_edit'],'provenance'),
            'source_head_mask':freeze(ROOT/e['source_head_mask'],'provenance'),
            'head_ownership_polygon':e['head_region_polygon'],'foot_anchor':e['foot_anchor'],
            'native_core_box':e['native_core_box'],'body_origin_candidate':e['original_candidate_id'],
            'original_source':e['source'],'original_source_sha256':e['source_sha256'],
            'original_native':e['native'],'original_native_sha256':e['native_sha256']})
        e['status']='accepted_visual_reference';e['approved_native']=entries[-1]['native']
    references=[freeze(preview/n,'review') for n in [
        'progression-native64.png','progression-zoom-x3.png','bastion-comparison-native64.png',
        'heads-before-after-x5.png','head-layers-audit-x8.png','browser-verified.png']]
    evidence=[freeze(docs/n,'review') for n in ['prompts.json','checks.json','ui-checks.json','REVIEW.md']]
    evidence.append({'path':snapshot.relative_to(ROOT).as_posix(),'sha256':sha(snapshot)})
    acceptance_path=decision.relative_to(ROOT).as_posix()
    write_json(decision,{'acceptance_id':'TECHNOMANCIEN-NOROBE-ACCEPT01',
        'review_id':'TECHNOMANCIEN-NOROBE-HEADS02','date':'2026-10-22','user_answer':'ok je valide',
        'status':'accepted_visual_reference','class_id':'core:technomancien',
        'scope':'Five shown robe-free Technomancien full outfits including their brighter progressively protected torso-provided default heads at native64.',
        'approved_compositions':5,'approved_default_heads':5,'native_dimensions':[64,64],
        'approved_candidate_ids':[e['candidate_id'] for e in entries],
        'entries':entries,'review_references':references,'provenance':evidence,
        'protected_previous_files':prior,'runtime_integration':False,'new_color_variants':0,
        'integration_gate':'Complete and review all class armor designs, independent helmets and weapon hand positions before integration.',
        'excluded':['Other class candidates','Superseded robe and dark-head designs','Independent helmet inventory items',
            'Unseen color variants','Production helmet replacement masks and weapon grips','Animations and in-engine lighting','Game mechanics and stats'],
        'checks':{'five_native64_files_match_shown_preview':True,'frozen_copies_byte_identical':True,
            'head_body_recomposition_exact':True,'body_pixels_outside_head_unchanged':True,
            'previous_images_and_acceptances_unchanged':len(prior)}})
    data.update(status='accepted_visual_reference',acceptance=acceptance_path,accepted_date='2026-10-22')
    write_json(docs/'candidates.json',data)
    checks.update(approval='accepted_by_explicit_user_reply',acceptance=acceptance_path)
    write_json(docs/'checks.json',checks)
    policy_path=REVIEW/'head-appearance-rule.json'
    policy=json.loads(policy_path.read_text('utf-8'))
    policy.update(visual_status='bastion_copper_and_technomancien_no_robe_outfits_and_default_heads_accepted_other_classes_pending',
        technomancien_default_head_appearances='five_accepted_brighter_progressively_protected_robe_free_heads',
        technomancien_head_review=(docs/'candidates.json').relative_to(ROOT).as_posix(),
        technomancien_head_acceptance=acceptance_path,technomancien_armor_acceptance=acceptance_path)
    write_json(policy_path,policy)
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    root['latest_steering']='22 October 2026: five robe-free Technomancien outfits and brighter progressive default heads explicitly accepted at native64. Copper Bastion remains accepted. Other classes, independent helmets, grips and integration remain pending.'
    root['approval_scopes']['body_armor_variants']='Five copper Bastion and five robe-free Technomancien outfits accepted; other classes pending review.'
    root['approval_scopes']['default_head_appearances']='Five copper Bastion and five revised robe-free Technomancien default heads accepted; other classes pending.'
    root['technomancien_current_acceptance']=acceptance_path
    write_json(REVIEW/'candidates.json',root)
    section_path=OUT/'technomancien-no-robe-01/section.html'
    section=section_path.read_text('utf-8')
    section=section.replace('Technomancien · têtes plus claires et progressives','Technomancien · cinq armures et têtes par défaut validées')
    section=section.replace('<p>Cinq protections sans robe ni capuche.',
        '<p><strong>Validation enregistrée le 22 octobre 2026.</strong> Cinq protections sans robe ni capuche.')
    section=section.replace('<p class="note">Nouvelles têtes à examiner.',
        '<p><a href="../../docs/reviews/class-readability-01/technomancien-no-robe-heads-02/acceptance.json">Accord et références exactes</a></p>\n<p class="note">Les cinq tenues et leurs têtes par défaut sont validées.')
    section_path.write_text(section,encoding='utf-8')
    (preview/'section.html').write_text(section,encoding='utf-8')
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    print(json.dumps({'acceptance':acceptance_path,'approved_outfits':5,'approved_default_heads':5,
        'frozen_native_sources_and_components':25,'additional_frozen_mask_and_edit_references':10,
        'previous_images_and_acceptances_unchanged':len(prior),'runtime_integration':False}))


if __name__=='__main__':main()
