"""Record the user's explicit approval of the corrected visual review."""
import hashlib
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
OUT=BASE/'skill-animation-review-69'
TARGET=OUT/'visual-acceptance-01.json'
assert not TARGET.exists()
review=json.loads((OUT/'review.json').read_text(encoding='utf8'))
assert review['review_id']=='SM05-ANIM05-GEL-RESTORE'
assert review['status']=='awaiting_visual_review'
validation=json.loads((OUT/'validation.json').read_text(encoding='utf8'))
assert validation['native_browser_canvas_matches']==8
gel=json.loads((OUT/'gel-reference-verification.json').read_text(encoding='utf8'))
assert gel['accepted_atlas_unchanged'] and gel['draw_body_identical_to_approved_review28']
review.update(status='visually_approved',native_visual_approval=True,visual_approval_date='2026-10-14',visual_acceptance='visual-acceptance-01.json')
(OUT/'review.json').write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'))
dependencies=[
    BASE/'skill-animation-review-28/gel-atlas.png',
    BASE/'skill-animation-review-28/visual-acceptance-01.json',
    BASE/'skill-animation-review-27/gel-atlas.png',
    BASE/'skill-animation-review-68/snow-ice-source.png',
    BASE/'skill-animation-review-63/fire-burst-atlas.png',
    BASE/'skill-animation-review-62/poison-burst-atlas.png',
    BASE/'skill-animation-review-62/frost-burst-atlas.png',
    BASE/'skill-animation-review-61/poison-burst-atlas.png',
    BASE/'skill-animation-review-60/mine-sprite.png',
    BASE/'skill-animation-review-06/flame-atlas.png',
]
record={
    'date':'2026-10-14',
    'user_reply':'ok parfait je valide',
    'reviewed_page':'skill-animation-review-69/cote-a-cote.html',
    'accepted':[{'number':73,'skill':'SM05','variant':e['variant'],'status':'visually_approved','native_tile_size':64} for e in entries],
    'scope':'Visual review69: burning ground, independently animated poison gas and continuous snow/frost/ice surface without uniform colour wash; square3x3 footprint clipped by obstacles; restored accepted GEL-A05 on affected bodies.',
    'gel_reference':'GEL-A05, approved in review28, reused through the same renderer and unchanged drawings.',
    'mechanics':'Existing accepted rules retained; prepared damage examples do not establish new balancing, AI, resistance or status-resolution rules.',
    'other_candidates':'Earlier residue drawings remain archived proposals. SM04 extended footprint and other unreviewed animations retain their separate status.',
    'campaign_integration':False,
    'artifact_hashes':{p.relative_to(OUT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.rglob('*')) if p.is_file() and p!=TARGET},
    'dependency_hashes':{p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in dependencies}
}
TARGET.write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
assert all(hashlib.sha256((OUT/p).read_bytes()).hexdigest()==h for p,h in record['artifact_hashes'].items())
assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in record['dependency_hashes'].items())
print(f'Approved {len(entries)} visual views; {len(record["artifact_hashes"])} files and {len(dependencies)} dependencies verified')
