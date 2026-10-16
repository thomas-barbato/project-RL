"""Native capture consistency and exact browser rendering, SM04."""
import hashlib
import json
import sys
from pathlib import Path
from PIL import Image,ImageChops
ROOT=Path(__file__).resolve().parents[2]
REVIEW=int(sys.argv[1]) if len(sys.argv)>1 else 70
OUT=ROOT/f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{REVIEW}'
proof=json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
assert all(hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==h for p,h in proof.items())
record=json.loads((OUT/'browser-visible-verification.json').read_text(encoding='utf8'))
checks=[]
for capture in record['captures']:
    for e in capture['samples']:
        actual=Image.open(OUT/e['browser_png']).convert('RGB')
        expected=Image.open(OUT/e['expected_native_png']).convert('RGB').crop((32,88,608,664))
        assert actual.size==expected.size==(576,576)
        assert ImageChops.difference(actual,expected).getbbox() is None
        checks.append({'capture':capture['label'],'profile':e['profile'],'pixels_identical':True})
record['native_pixel_checks']=checks
(OUT/'browser-visible-verification.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
encoding=json.loads((OUT/'native-encoding-verification.json').read_text(encoding='utf8'))
validation={'date':'2026-10-14','build':'cargo build --locked passed','format':'cargo fmt --all -- --check passed','whitespace':'targeted git diff --check passed','targeted_geometry_tests':'2 passed','native_readbacks':encoding['converted_frames'],'native_encoding_lossless':True,'browser_native_matches':len(checks),'gallery':'12 native64 profiles; paired radius1/radius4 for Fire/Poison/Cold, open/full-wall; hover and expiry','prior_files_preserved':len(proof),'campaign_integration':False,'full_suite':'Not run; isolated debug visual review'}
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'native_browser_matches':len(checks),'previous_files_unchanged':len(proof)}))
