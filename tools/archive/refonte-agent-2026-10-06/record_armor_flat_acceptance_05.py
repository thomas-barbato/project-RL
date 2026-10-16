"""Record the user's acceptance of the six corrections, without changing art."""
from pathlib import Path
import hashlib, json

BASE=Path(__file__).resolve().parent
OUT=BASE/'armor-flat-review-05'
ids=['AR-08','AR-10','AR-11','AR-12','AR-N13','AR-N14']
review=json.loads((OUT/'review.json').read_text('utf-8'))
frozen={}
for entry in review['entries']:
    if entry['id'] not in ids: continue
    for view in entry['views'].values():
        for relative in view.values():
            path=(OUT/relative).resolve()
            frozen[path.relative_to(BASE).as_posix()]=hashlib.sha256(path.read_bytes()).hexdigest()
    entry['review_state']='accepted'
    entry['visual_acceptance']='ARMOR-FLAT05-ACCEPT01'
receipt={'review_id':'ARMOR-FLAT05-ACCEPT01','accepted_on':'2026-10-10','user_quote':'oui','context':'Following the consolidated ARMOR-FLAT05 correction review, before the request to integrate classes.','accepted_visual_ids':ids,'palettes_per_outfit':3,'equipment_configurations':16,'scope':'Exact six corrected drawings, their displayed palettes and common equipment compositions. Other pending candidates keep their previous state.','names_and_mechanics':'No additional object names, defense values or attribute requirements are inferred.','frozen_files':frozen}
(OUT/'visual-acceptance-01.json').write_text(json.dumps(receipt,ensure_ascii=False,indent=2)+'\n','utf-8')
review['accepted_visual_ids']=sorted(set(review['accepted_visual_ids'])|set(ids))
review['corrections_pending_visual_acceptance']=[]
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n','utf-8')
(OUT/'review-data.js').write_text('window.armorReview='+json.dumps(review,ensure_ascii=False)+';','utf-8')
print(f'Accepted six corrections; {len(frozen)} original files frozen.')
