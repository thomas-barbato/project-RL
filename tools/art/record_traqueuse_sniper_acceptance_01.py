"""Freeze the five Traqueuse outfits explicitly accepted at native64."""
import json
import shutil
from PIL import Image
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,gallery,sha,write_json


def main():
    name='traqueuse-sniper-wave-01';docs,preview=REVIEW/name,OUT/name
    decision=docs/'acceptance.json'
    if decision.exists():raise SystemExit('Acceptance already recorded; do not overwrite.')
    data=json.loads((docs/'candidates.json').read_text('utf-8'))
    checks=json.loads((docs/'checks.json').read_text('utf-8'))
    prior=json.loads((docs/'prior-images.json').read_text('utf-8'))
    assert len(data['entries'])==5 and checks['ui_execution']=='verified_in_app_browser'
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    frozen=SOURCES/name/'accepted'

    def freeze(path,folder):
        target=frozen/folder/path.name;assert not target.exists(),target
        target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(path,target)
        assert sha(target)==sha(path)
        return {'path':target.relative_to(ROOT).as_posix(),'sha256':sha(target)}

    snapshot=docs/'accepted-review/candidates-as-reviewed.json'
    snapshot.parent.mkdir(parents=True,exist_ok=True);assert not snapshot.exists()
    shutil.copyfile(docs/'candidates.json',snapshot)
    entries=[]
    for e in data['entries']:
        source,native=ROOT/e['source'],ROOT/e['native']
        assert sha(source)==e['source_sha256'] and sha(native)==e['native_sha256']
        assert sha(preview/(e['candidate_id']+'-native64.png'))==sha(native)
        image=Image.open(native).convert('RGBA')
        assert image.size==(64,64) and image.getchannel('A').getextrema()==(0,255)
        components={}
        for part,ref in e['layers'].items():
            path=ROOT/ref['path'];assert sha(path)==ref['sha256'];components[part]=freeze(path,'layers')
        head=Image.open(ROOT/e['layers']['head']['path']).convert('RGBA')
        body=Image.open(ROOT/e['layers']['body']['path']).convert('RGBA')
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==image.tobytes()
        entries.append({'number':e['number'],'candidate_id':e['candidate_id'],'rank':e['rank'],'label':e['label'],
            'source':freeze(source,'sources'),'native':freeze(native,'native64'),'components':components,
            'original_source':e['source'],'original_source_sha256':e['source_sha256'],
            'original_native':e['native'],'original_native_sha256':e['native_sha256'],
            'head_ownership_polygon':e['head_ownership_polygon'],'head_label':e['head_label'],
            'foot_anchor':e['foot_anchor'],'native_core_box':e['native_core_box']})
        e.update(status='accepted_visual_reference',approved_native=entries[-1]['native'])
    references=[freeze(preview/n,'review') for n in ['progression-native64.png','progression-zoom-x3.png',
        'three-classes-native64.png','head-progression-native-x4.png','head-layers-audit-x8.png','browser-verified.png']]
    evidence=[freeze(docs/n,'review') for n in ['prompts.json','checks.json','ui-checks.json','REVIEW.md']]
    evidence.append({'path':snapshot.relative_to(ROOT).as_posix(),'sha256':sha(snapshot)})
    acceptance_path=decision.relative_to(ROOT).as_posix()
    write_json(decision,{'acceptance_id':'TRAQUEUSE-SNIPER-W01-ACCEPT01','review_id':'TRAQUEUSE-SNIPER-W01',
        'date':'2026-10-22','user_answer':"d'accord on valide :)",'status':'accepted_visual_reference',
        'class_id':'core:traqueur','class_name':'Traqueuse','presentation_gender':'female',
        'scope':'Five shown female sniper/ninja Traqueuse full outfits and their torso-provided progressive default heads at native64.',
        'approved_compositions':5,'approved_default_heads':5,'native_dimensions':[64,64],'native_height':56,
        'approved_candidate_ids':[e['candidate_id'] for e in entries],'entries':entries,
        'review_references':references,'provenance':evidence,'protected_previous_files':prior,
        'reference_acceptances':data['reference_acceptances'],'runtime_integration':False,'new_color_variants':0,
        'integration_gate':'Complete and review all class armor designs, independent helmets and weapon hand positions before integration.',
        'excluded':['Other class candidates','Superseded Traqueuse designs including P05-r01','Independent helmet inventory items',
            'Unseen color variants','Production helmet replacement masks and weapon grips','Animations and in-engine lighting','Game mechanics and stats'],
        'checks':{'five_native64_files_match_shown_preview':True,'frozen_copies_byte_identical':True,
            'head_body_recomposition_exact':True,'previous_images_and_acceptances_unchanged':len(prior)}})
    data.update(status='accepted_visual_reference',acceptance=acceptance_path,accepted_date='2026-10-22')
    write_json(docs/'candidates.json',data)
    checks.update(approval='accepted_by_explicit_user_reply',acceptance=acceptance_path)
    write_json(docs/'checks.json',checks)
    policy_path=REVIEW/'head-appearance-rule.json';policy=json.loads(policy_path.read_text('utf-8'))
    policy.update(visual_status='bastion_technomancien_traqueuse_outfits_and_default_heads_accepted_two_classes_pending',
        traqueuse_default_head_appearances='five_accepted_sniper_ninja_default_heads',
        traqueuse_head_review=(docs/'candidates.json').relative_to(ROOT).as_posix(),
        traqueuse_head_acceptance=acceptance_path,traqueuse_armor_acceptance=acceptance_path)
    write_json(policy_path,policy)
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    root['latest_steering']='22 October 2026: five sniper/ninja Traqueuse outfits and progressive default heads explicitly accepted at native64. Bastion and Technomancien remain accepted. Arcaniste and Metamorphe revisions, independent helmets, grips and integration remain pending.'
    root['approval_scopes']['body_armor_variants']='Five current outfits each for Bastion, Technomancien and Traqueuse accepted; Arcaniste and Metamorphe pending.'
    root['approval_scopes']['default_head_appearances']='Five torso-provided default heads each for Bastion, Technomancien and Traqueuse accepted; Arcaniste and Metamorphe pending.'
    root['traqueuse_current_acceptance']=acceptance_path;write_json(REVIEW/'candidates.json',root)
    section_path=preview/'section.html';section=section_path.read_text('utf-8')
    section=section.replace('Traqueuse · cinq protections de tireuse / ninja','Traqueuse · cinq armures et têtes par défaut validées')
    section=section.replace('<p>Une femme athlétique,','<p><strong>Validation enregistrée le 22 octobre 2026.</strong> Une femme athlétique,')
    section=section.replace('<p class="note">Propositions à examiner.',
        '<p><a href="../../docs/reviews/class-readability-01/traqueuse-sniper-wave-01/acceptance.json">Accord et références exactes</a></p>\n<p class="note">Les cinq tenues et leurs têtes par défaut sont validées.')
    section_path.write_text(section,encoding='utf-8');gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    print(json.dumps({'acceptance':acceptance_path,'approved_outfits':5,'approved_default_heads':5,
        'frozen_sources_natives_and_components':25,'previous_images_and_acceptances_unchanged':len(prior),'runtime_integration':False}))


if __name__=='__main__':main()
