"""Keep Regeneration's reviewed pixels; replace only the unapproved Purge preview."""
import hashlib
import json
import sys
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
REVIEWS=ROOT/'artifacts/refonte-agent-2026-10-06'
BASE=REVIEWS/'skill-animation-review-77'
number=int(sys.argv[1]) if len(sys.argv)>1 else 78
assert number in (78,79)
CLEANUP=number==79
REVIEW=REVIEWS/f'skill-animation-review-{number}'
DATA=REVIEWS/'apercu-animations-data'
NATIVE=REVIEW/'_capture'
original=json.loads((BASE/'animations.json').read_text(encoding='utf8'))
original_checks=json.loads((BASE/'validation.json').read_text(encoding='utf8'))['checks']
previous=json.loads((REVIEWS/'skill-animation-review-78/animations.json').read_text(encoding='utf8')) if CLEANUP else []
previous_checks=json.loads((REVIEWS/'skill-animation-review-78/validation.json').read_text(encoding='utf8'))['checks'] if CLEANUP else []
entries=[e for e in original if e['id']=='SU04']
checks=[c for c in original_checks if c['variant'].startswith('SU04-')]
captured=json.loads((NATIVE/'capture.json').read_text(encoding='utf8'))
assert len(captured)==7
reference_checked=False
for e in captured:
    old=next(s for s in original if s['id']==e['id'] and s['label']==e['label'])
    expected=[dict(s) for s in old['timeline']]
    if CLEANUP and e['id']=='SU05' and e['case']=='INTERRUPTION':
        # This review no longer injects a new poison application after Purge.
        for s in expected[48:]:s.update(poison=False,interrupted=False)
    assert e['timeline']==expected, 'Only the explicitly removed demo reapplication may change.'
    indices=e.pop('indices');cols=e.pop('native_columns');count=e['unique_images'];rows=(count+cols-1)//cols
    native=Image.open(NATIVE/e['folder']/'atlas.png').convert('RGB')
    assert native.size==(576*cols,576*rows)
    atlas=Image.new('RGB',native.size);hashes=[]
    for i in range(count):
        y=(rows-1-i//cols)*576
        frame=native.crop(((i%cols)*576,y,(i%cols+1)*576,y+576))
        atlas.paste(frame,((i%cols)*576,(i//cols)*576))
        hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    if e['id']=='SU04':
        approved=next(c for c in original_checks if c['variant']==old['variant'])
        assert [hashes[n] for n in indices]==[approved['frame_rgb_sha256'][n] for n in old['indices']]
        reference_checked=True
        print('Regeneration native replay: all 112 frames unchanged.',flush=True)
        continue
    assert e['variant'].startswith('SU05-A03-' if CLEANUP else 'SU05-A02-')
    if CLEANUP:
        before=next(s for s in previous if s['label']==e['label'] and s['id']=='SU05')
        before_check=next(c for c in previous_checks if c['variant']==before['variant'])
        assert [hashes[indices[n]] for n in range(12,48)]==[before_check['frame_rgb_sha256'][before['indices'][n]] for n in range(12,48)], 'The approved golden pulse must stay unchanged.'
        assert hashes[indices[0]]!=before_check['frame_rgb_sha256'][before['indices'][0]], 'The pre-cast flames must be removed.'
    name=e['variant']+'.webp';atlas.save(DATA/name,'WEBP',lossless=True,method=2)
    with Image.open(DATA/name) as packed:
        for i,h in enumerate(hashes):
            frame=packed.crop(((i%cols)*576,(i//cols)*576,(i%cols+1)*576,(i//cols+1)*576)).convert('RGB')
            assert hashlib.sha256(frame.tobytes()).hexdigest()==h
    e.update(atlas='apercu-animations-data/'+name,columns=cols,indices=indices)
    entries.append(e)
    checks.append(dict(variant=e['variant'],frames=len(indices),unique_images=count,atlas_bytes=(DATA/name).stat().st_size,pixel_equality=True,timeline=True,frame_rgb_sha256=hashes))
    print(e['variant'],count,(DATA/name).stat().st_size,flush=True)
assert reference_checked and len(entries)==12
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf8')
template=(ROOT/'tools/art/survival_care_review_template.html').read_text(encoding='utf8')
old_title='ANIM-S36 · Candidats 81–82 · rang effectif 11 · rendus à valider'
assert old_title in template
template=template.replace(old_title,'ANIM-S36-REV02 · 81 : soin conservé · 82 : Purge révisée à valider · rang effectif 11')
template=template.replace('Purge expulse le poison et la brûlure.','Purge retire le poison et la brûlure.')
template=template.replace('Purge retire ici deux altérations préparées,','Purge entoure brièvement le corps de fils blanc doré, avec « Purge » au-dessus de la tête. Elle retire ici deux altérations préparées,')
if CLEANUP:
    template=template.replace('ANIM-S36-REV02','ANIM-S36-REV03').replace('Purge révisée à valider','Purge seule, aperçu nettoyé')
    template=template.replace('Blessure / nouvelle altération','Interruption du soin')
    template=template.replace('Blessure / réapplication','Blessure / +1 tour')
    template=template.replace('La blessure arrête Régénération ; le poison peut revenir après Purge.','La blessure arrête Régénération ; Purge est terminée.')
    template=template.replace('Le poison et la brûlure de cet aperçu servent à montrer le retrait des statuts, sans simuler leurs dégâts périodiques.','Les altérations sont indiquées uniquement dans la barre d’état avant Purge ; aucun effet de statut supplémentaire n’est dessiné sur le personnage dans cet aperçu.')
(REVIEWS/'apercu-animations.html').write_text(template.replace('__ENTRIES__',text),encoding='utf8')
(REVIEW/'validation.json').write_text(json.dumps(dict(native_tile=64,scenes=12,lossless=True,healing_112_frames_unchanged=True,all_prepared_mechanics_unchanged=not CLEANUP,purge_isolated=CLEANUP,gameplay_rules_unchanged=True,approved_purge_pulse_unchanged=CLEANUP,checks=checks),indent=2),encoding='utf8')
print('New Purge atlas MiB:',round(sum(c['atlas_bytes'] for c in checks if c['variant'].startswith('SU05-'))/2**20,2))
