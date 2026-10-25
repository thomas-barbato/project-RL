"""Freeze the five copper Bastion designs approved on 22 October 2026."""
import json
import shutil

from PIL import Image

from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, gallery, sha, write_json


def main():
    name='bastion-copper-wave-01'
    docs,preview=REVIEW/name,OUT/name
    decision=docs/'acceptance.json'
    if decision.exists():raise SystemExit('Acceptance already recorded; do not overwrite.')
    candidates=json.loads((docs/'candidates.json').read_text('utf-8'))
    prior=json.loads((docs/'prior-images.json').read_text('utf-8'))
    assert len(candidates['entries'])==5
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    accepted=SOURCES/name/'accepted'

    def freeze(path,folder):
        target=accepted/folder/path.name
        target.parent.mkdir(parents=True,exist_ok=True)
        assert not target.exists(),target
        shutil.copyfile(path,target)
        assert sha(target)==sha(path)
        return {'path':target.relative_to(ROOT).as_posix(),'sha256':sha(target)}

    snapshot=docs/'accepted-review/candidates-as-reviewed.json'
    snapshot.parent.mkdir(parents=True,exist_ok=True)
    assert not snapshot.exists()
    shutil.copyfile(docs/'candidates.json',snapshot)
    entries=[]
    for e in candidates['entries']:
        source,native=ROOT/e['source'],ROOT/e['native']
        assert sha(source)==e['source_sha256'] and sha(native)==e['native_sha256']
        assert sha(preview/(e['candidate_id']+'-native64.png'))==sha(native)
        sprite=Image.open(native).convert('RGBA')
        assert sprite.size==(64,64) and sprite.getchannel('A').getextrema()==(0,255)
        components={}
        for part,reference in e['layers'].items():
            path=ROOT/reference['path'];assert sha(path)==reference['sha256']
            components[part]=freeze(path,'layers')
        head=Image.open(ROOT/e['layers']['head']['path']).convert('RGBA')
        body=Image.open(ROOT/e['layers']['body']['path']).convert('RGBA')
        rebuilt=body.copy();rebuilt.alpha_composite(head)
        assert rebuilt.tobytes()==sprite.tobytes()
        entry={'number':e['number'],'candidate_id':e['candidate_id'],'rank':e['rank'],
            'label':e['label'],'source':freeze(source,'sources'),
            'native':freeze(native,'native64'),'components':components,
            'original_source':e['source'],'original_source_sha256':e['source_sha256'],
            'original_native':e['native'],'original_native_sha256':e['native_sha256'],
            'head_ownership_polygon':e['head_ownership_polygon'],'foot_anchor':e['foot_anchor'],
            'native_core_box':e['native_core_box'],'reference_provenance':e['retained_reference']}
        entries.append(entry)
        e['status']='accepted_visual_reference'
        e['approved_native']=entry['native']
    references=[freeze(preview/n,'review') for n in
        ['progression-native64.png','progression-zoom-x3.png','browser-verified.png']]
    evidence=[freeze(docs/n,'review') for n in ['prompts.json','checks.json','ui-checks.json']]
    evidence.append({'path':snapshot.relative_to(ROOT).as_posix(),'sha256':sha(snapshot)})
    acceptance_path=decision.relative_to(ROOT).as_posix()
    write_json(decision,{'acceptance_id':'BASTION-COPPER-W01-ACCEPT01',
        'review_id':'BASTION-COPPER-W01','date':'2026-10-22','user_answer':'je valide :)',
        'status':'accepted_visual_reference','class_id':'core:bastion',
        'scope':'Five shown copper Bastion full outfits and their torso-provided default heads at native64.',
        'approved_compositions':5,'approved_default_heads':5,'native_dimensions':[64,64],
        'approved_candidate_ids':[e['candidate_id'] for e in entries],
        'entries':entries,'review_references':references,'provenance':evidence,
        'protected_previous_files':prior,'runtime_integration':False,'new_color_variants':0,
        'integration_gate':'Complete and review all class armors, independent helmets and weapon hand positions before integration.',
        'excluded':['Other classes','Independent helmet inventory items','Unseen color variants',
            'Production helmet masks and weapon grips','Animations and in-engine lighting','Game mechanics and stats'],
        'checks':{'five_native64_files_match_shown_preview':True,
            'frozen_copies_byte_identical':True,'head_body_recomposition_exact':True,
            'previous_images_and_acceptances_unchanged':len(prior)}})
    candidates.update(status='accepted_visual_reference',acceptance=acceptance_path,
        accepted_date='2026-10-22')
    write_json(docs/'candidates.json',candidates)
    policy_path=REVIEW/'head-appearance-rule.json'
    policy=json.loads(policy_path.read_text('utf-8'))
    policy['historical_bastion_head_acceptance']=policy['bastion_head_acceptance']
    policy.update(visual_status='bastion_copper_outfits_and_default_heads_accepted_other_classes_pending',
        bastion_head_review=(docs/'candidates.json').relative_to(ROOT).as_posix(),
        bastion_head_acceptance=acceptance_path,bastion_copper_armor_acceptance=acceptance_path,
        bastion_default_head_appearances='five_accepted_copper_progression_heads')
    write_json(policy_path,policy)
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    root['latest_steering']='22 October 2026: five copper Bastion outfits and their shown default heads explicitly accepted. Freeze exact native64 references. Other classes, separate helmets, grips and integration remain pending; colors deferred.'
    root['approval_scopes']['body_armor_variants']='Five copper Bastion outfits accepted; other class revisions pending.'
    root['approval_scopes']['default_head_appearances']='Five torso-provided heads of the copper Bastion progression accepted; other classes pending.'
    root['bastion_current_acceptance']=acceptance_path
    write_json(REVIEW/'candidates.json',root)
    section_path=preview/'section.html'
    section=section_path.read_text('utf-8')
    section=section.replace('Bastion · du très léger au blindage brutal','Bastion · cinq armures et têtes par défaut validées')
    section=section.replace('<p>Cinq degrés de protection, à la même hauteur.',
        '<p><strong>Validation enregistrée le 22 octobre 2026.</strong> Cinq degrés de protection, à la même hauteur.')
    section=section.replace('<p class="note">Les coiffes présentées',
        '<p><a href="../../docs/reviews/class-readability-01/bastion-copper-wave-01/acceptance.json">Accord et références exactes</a></p>\n<p class="note">Les coiffes présentées')
    section_path.write_text(section,encoding='utf-8')
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    print(json.dumps({'acceptance':acceptance_path,'approved_outfits':5,'approved_default_heads':5,
        'frozen_native_sources_and_components':25,'previous_images_and_acceptances_unchanged':len(prior),
        'runtime_integration':False}))


if __name__=='__main__':main()
