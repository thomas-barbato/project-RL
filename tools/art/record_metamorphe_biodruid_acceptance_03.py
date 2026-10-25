"""Freeze the five explicitly approved Metamorphe outfits and integration permission."""
import json
import shutil
from PIL import Image
from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, sha, write_json


def main():
    name = 'metamorphe-biodruid-wave-03'
    docs, preview = REVIEW/name, OUT/name
    decision = docs/'acceptance.json'
    if decision.exists():
        raise SystemExit('Acceptance already recorded; do not overwrite.')
    data = json.loads((docs/'candidates.json').read_text('utf-8'))
    checks = json.loads((docs/'checks.json').read_text('utf-8'))
    prior = json.loads((docs/'prior-images.json').read_text('utf-8'))
    assert len(data['entries']) == 5 and checks['browser_verification']['status'] == 'verified'
    assert all(sha(ROOT/p) == h for p,h in prior.items())
    frozen = SOURCES/name/'accepted'
    def freeze(path, folder):
        target = frozen/folder/path.name
        assert not target.exists(), target
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(path,target)
        return {'path':target.relative_to(ROOT).as_posix(),'sha256':sha(target)}
    for e in data['entries']:
        assert sha(ROOT/e['source']) == e['source_sha256']
        assert sha(ROOT/e['native']) == e['native_sha256']
        assert sha(preview/(e['candidate_id']+'-native64.png')) == e['native_sha256']
        for ref in e['layers'].values():
            assert sha(ROOT/ref['path']) == ref['sha256']
        body = Image.open(ROOT/e['layers']['body']['path']).convert('RGBA')
        body.alpha_composite(Image.open(ROOT/e['layers']['head']['path']).convert('RGBA'))
        assert body.tobytes() == Image.open(ROOT/e['native']).convert('RGBA').tobytes()
    snapshot = docs/'accepted-review/candidates-as-reviewed.json'
    snapshot.parent.mkdir(exist_ok=True)
    shutil.copyfile(docs/'candidates.json',snapshot)
    entries = []
    for e in data['entries']:
        entries.append({**{k:e[k] for k in ['number','candidate_id','rank','label','head_label','head_ownership_polygon','foot_anchor','native_core_box']},
            'source':freeze(ROOT/e['source'],'sources'),'native':freeze(ROOT/e['native'],'native64'),
            'components':{p:freeze(ROOT/r['path'],'layers') for p,r in e['layers'].items()}})
        e['status'] = 'accepted_visual_reference'
    images = [freeze(p,'review') for p in preview.glob('*.png') if not p.name.startswith('pilot') and not p.name.endswith('-native64.png')]
    provenance = [freeze(docs/n,'review') for n in ['prompts.json','checks.json','REVIEW.md','joint-native64-review.json']]
    provenance.append({'path':snapshot.relative_to(ROOT).as_posix(),'sha256':sha(snapshot)})
    acceptance = decision.relative_to(ROOT).as_posix()
    write_json(decision,{'acceptance_id':'METAMORPHE-BIODRUID-W03-ACCEPT01',
        'review_id':'METAMORPHE-BIODRUID-W03','date':'2026-10-22',
        'user_answer':'je valide :) , on peut donc integrer ces nouveaux dessins, à noter que du coup il faut s’assurer que chaque dessins soit compatible avec toutes les armes :)',
        'status':'accepted_visual_reference','class_id':'core:metamorphe',
        'approved_compositions':5,'approved_default_heads':5,'native_dimensions':[64,64],
        'entries':entries,'review_references':images,'provenance':provenance,
        'reference_acceptances':data['reference_acceptances'],
        'integration_authorized':True,'integration_condition':'verify every approved outfit with all current weapons and active hand configurations',
        'new_color_variants':0,'runtime_integration':'authorized_pending_implementation_and_checks'})
    data.update(status='accepted_visual_reference',acceptance=acceptance)
    write_json(docs/'candidates.json',data)
    checks.update(approval='accepted_by_explicit_user_reply',acceptance=acceptance)
    write_json(docs/'checks.json',checks)
    root = json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    root['metamorphe_current_acceptance'] = acceptance
    root['latest_steering'] = '22 October 2026: all 25 current class outfits accepted. User explicitly authorizes integration and requires compatibility of every outfit with all weapons.'
    root['approval_scopes']['body_armor_variants'] = '25 outfits for five classes accepted; integration authorized subject to all-weapon verification.'
    root['approval_scopes']['default_head_appearances'] = 'All 25 torso-provided default heads accepted.'
    write_json(REVIEW/'candidates.json',root)
    rule = json.loads((REVIEW/'head-appearance-rule.json').read_text('utf-8'))
    rule.update(visual_status='five_classes_accepted_integration_authorized',
                metamorphe_armor_acceptance=acceptance,metamorphe_head_acceptance=acceptance,
                integration_gate='Latest explicit user authorization: integrate approved outfits and verify all current weapons; independent new helmet designs and color variants are not approved by extension.')
    write_json(REVIEW/'head-appearance-rule.json',rule)
    print(json.dumps({'acceptance':acceptance,'all_25_outfits_accepted':True,'integration_authorized':True}))


if __name__ == '__main__':
    main()
