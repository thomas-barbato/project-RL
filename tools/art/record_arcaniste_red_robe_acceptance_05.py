"""Freeze the five explicitly accepted red-robed Arcaniste compositions."""
import json
import shutil
from PIL import Image
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,gallery,sha,write_json


def main():
    name='arcaniste-red-robe-wave-05';docs,preview=REVIEW/name,OUT/name
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

    # Verify all compositions before creating the acceptance and its frozen copies.
    for e in data['entries']:
        assert sha(ROOT/e['source'])==e['source_sha256'] and sha(ROOT/e['native'])==e['native_sha256']
        assert sha(preview/(e['candidate_id']+'-native64.png'))==e['native_sha256']
        actor=Image.open(ROOT/e['native']).convert('RGBA');assert actor.size==(64,64)
        for ref in e['layers'].values():assert sha(ROOT/ref['path'])==ref['sha256']
        head=Image.open(ROOT/e['layers']['head']['path']).convert('RGBA')
        body=Image.open(ROOT/e['layers']['body']['path']).convert('RGBA')
        mask=Image.open(ROOT/e['layers']['head-mask']['path']).convert('L')
        rebuilt=body.copy();rebuilt.paste(head,(0,0),mask);assert rebuilt.tobytes()==actor.tobytes()
        composite=body.copy();composite.alpha_composite(head)
        assert composite.getchannel('A').tobytes()==actor.getchannel('A').tobytes()
        assert all(composite.getpixel((x,y))==actor.getpixel((x,y)) for y in range(64) for x in range(64) if actor.getpixel((x,y))[3]>0)
    snapshot=docs/'accepted-review/candidates-as-reviewed.json'
    snapshot.parent.mkdir(parents=True,exist_ok=True);assert not snapshot.exists()
    shutil.copyfile(docs/'candidates.json',snapshot)
    entries=[]
    for e in data['entries']:
        components={part:freeze(ROOT/ref['path'],'layers') for part,ref in e['layers'].items()}
        entries.append({'number':e['number'],'candidate_id':e['candidate_id'],'rank':e['rank'],'label':e['label'],
            'source':freeze(ROOT/e['source'],'sources'),'native':freeze(ROOT/e['native'],'native64'),'components':components,
            'original_source':e['source'],'original_source_sha256':e['source_sha256'],
            'original_native':e['native'],'original_native_sha256':e['native_sha256'],
            'head_ownership_polygon':e['head_ownership_polygon'],'head_label':e['head_label'],
            'foot_anchor':e['foot_anchor'],'native_core_box':e['native_core_box'],
            'legacy_sprite_reused_exactly':e['legacy_sprite_reused_exactly']})
        e.update(status='accepted_visual_reference',approved_native=entries[-1]['native'])
    references=[freeze(preview/n,'review') for n in ['progression-native64.png','progression-zoom-x3.png',
        'four-classes-native64.png','legacy-and-heavy-native-x4.png','head-progression-native-x4.png',
        'head-layers-audit-x8.png','browser-verified.png']]
    evidence=[freeze(docs/n,'review') for n in ['prompts.json','checks.json','ui-checks.json','REVIEW.md']]
    evidence.append({'path':snapshot.relative_to(ROOT).as_posix(),'sha256':sha(snapshot)})
    acceptance=decision.relative_to(ROOT).as_posix()
    write_json(decision,{'acceptance_id':'ARCANISTE-RED-ROBE-W05-ACCEPT01','review_id':'ARCANISTE-RED-ROBE-W05',
        'date':'2026-10-22','user_answer':'je valide :)','status':'accepted_visual_reference',
        'class_id':'core:arcaniste','class_name':'Arcaniste','presentation_gender':'male',
        'scope':'Five shown red-robed Arcaniste full outfits and their torso-provided default heads at native64, including exact legacy sprite at rank 04.',
        'approved_compositions':5,'approved_default_heads':5,'native_dimensions':[64,64],
        'approved_candidate_ids':[e['candidate_id'] for e in entries],'entries':entries,
        'review_references':references,'provenance':evidence,'protected_previous_files':prior,
        'reference_acceptances':data['reference_acceptances'],'runtime_integration':False,'new_color_variants':0,
        'integration_gate':'Complete and review all class armors, independent helmets and weapon hand positions before integration.',
        'excluded':['Superseded plum Arcaniste ranges','Other class candidates','Independent helmet inventory items',
            'Unseen color variants','Production helmet override masks and weapon grips','Animations and in-engine lighting','Game rules and stats'],
        'checks':{'five_native64_files_match_shown_preview':True,'frozen_copies_byte_identical':True,
            'binary_head_body_recomposition_exact':True,'alpha_composite_exact_for_all_visible_pixels':True,
            'previous_images_and_acceptances_unchanged':len(prior)}})
    data.update(status='accepted_visual_reference',acceptance=acceptance,accepted_date='2026-10-22')
    write_json(docs/'candidates.json',data)
    checks.update(approval='accepted_by_explicit_user_reply',acceptance=acceptance)
    write_json(docs/'checks.json',checks)
    pp=REVIEW/'head-appearance-rule.json';policy=json.loads(pp.read_text('utf-8'))
    policy.update(visual_status='four_classes_accepted_metamorphe_remaining',
        arcaniste_default_head_appearances='five_accepted_red_robed_default_heads; legacy ivory mask at 04 and reinforced masked hood at 05',
        arcaniste_head_acceptance=acceptance,arcaniste_armor_acceptance=acceptance)
    policy['arcaniste_face_consistency']['visual_status']='accepted_as_shown'
    policy['arcaniste_head_attachment']['status']='accepted_as_shown'
    write_json(pp,policy)
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    root['latest_steering']='22 October 2026: five red-robed Arcaniste outfits and default heads explicitly accepted at native64 (je valide :)). Four classes accepted, twenty compositions. Metamorphe remains; independent helmets, weapon grips, variants and integration still deferred.'
    root['approval_scopes']['body_armor_variants']='Five current outfits each for Bastion, Technomancien, Traqueuse and Arcaniste accepted; Metamorphe remains to revise.'
    root['approval_scopes']['default_head_appearances']='Five torso-provided default heads each for four classes accepted; Metamorphe remains to revise.'
    root['arcaniste_current_acceptance']=acceptance
    root['arcaniste_selected_style_reference']['new_range_approval']='accepted_visual_reference'
    write_json(REVIEW/'candidates.json',root)
    section=(preview/'section.html').read_text('utf-8')
    section=section.replace('Arcaniste · retour à la robe rouge','Arcaniste · cinq tenues rouges validées')
    section=section.replace('<p>La progression rejoint','<p><strong>Validation enregistrée le 22 octobre 2026.</strong> La progression rejoint')
    section=section.replace('<p class="note">Nouvelle gamme à examiner.',
        f'<p><a href="../../{acceptance}">Accord et références exactes</a></p>\n<p class="note">Les cinq tenues et leurs têtes par défaut sont validées.')
    for p in (preview/'section.html',OUT/'arcaniste-channeler-wave-01/section.html'):p.write_text(section,encoding='utf-8')
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    print(json.dumps({'acceptance':acceptance,'accepted_outfits':5,'accepted_default_heads':5,
        'frozen_reference_count':len(entries)*5+len(references)+len(evidence),
        'four_accepted_classes':True,'previous_images_and_acceptances_unchanged':len(prior),'runtime_integration':False}))


if __name__=='__main__':main()
