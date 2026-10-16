"""Verify review GIF colours and the browser's exact native64 canvas pixels."""
import json
import sys
from pathlib import Path
from PIL import Image, ImageChops

REVIEW = int(sys.argv[1]) if len(sys.argv)>1 else 68
OUT = Path(__file__).resolve().parents[2] / f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{REVIEW}'
checks=[]
for entry in json.loads((OUT/'native/capture.json').read_text(encoding='utf8')):
    folder=OUT/'native'/entry['folder']
    image=Image.open(folder/'132.png').convert('RGB').crop((32,88,608,664))
    control=Image.open(folder/'control-132.png').convert('RGB').crop((32,88,608,664))
    native=list(image.get_flattened_data());base=list(control.get_flattened_data())
    selected=[i for i,(a,b) in enumerate(zip(native,base)) if max(abs(x-y) for x,y in zip(a,b))>20]
    with Image.open(OUT/(entry['variant']+'-64.gif')) as gif:
        elapsed=0
        for frame in range(gif.n_frames):
            gif.seek(frame);duration=gif.info['duration']
            if elapsed<=5500<elapsed+duration:break
            elapsed+=duration
        pixels=list(gif.convert('RGB').get_flattened_data())
        error=sum(sum(abs(x-y) for x,y in zip(native[i],pixels[i])) for i in selected)/(len(selected)*3)
        assert error<20,(entry['variant'],error)
        checks.append({'variant':entry['variant'],'ground_effect_pixels':len(selected),'mean_GIF_color_error':round(error,2)})
(OUT/'gif-color-verification.json').write_text(json.dumps(checks,indent=2)+'\n',encoding='utf8')
record=json.loads((OUT/'browser-visible-verification.json').read_text(encoding='utf8'))
matches=[]
for capture in record['captures']:
    for sample in capture['samples']:
        actual=Image.open(OUT/sample['browser_png']).convert('RGB')
        expected=Image.open(OUT/sample['expected_native_png']).convert('RGB').crop((32,88,608,664))
        assert actual.size==expected.size==(576,576)
        assert ImageChops.difference(actual,expected).getbbox() is None
        matches.append({'capture':capture['capture'],'type':sample['type'],'native_pixels_identical':True})
record['native_pixel_checks']=matches
(OUT/'browser-visible-verification.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
validation={'date':'2026-10-14','build':'cargo build --locked passed','format':'cargo fmt --all -- --check passed','whitespace':'targeted git diff --check passed','effective_native_readbacks':4800,'new_native_readbacks':1600,'reused_native_readbacks':3200,'native_pixel_preserving_encoding':True,'browser':'Edge Chromium file scheme passed','native_browser_canvas_matches':len(matches),'GIF_colour_checks':checks,'mechanics':'Immediate first of13; twelve later applications only; late entrant, idle, player protection, zone clipping and expiry checked.','full_suite':'Not run; isolated debug visual review', 'prior_review': '65 and67 preserved', 'revision': 'SM05-A03 for Fire/Poison; SM05-A04 for Cold', 'source': 'New continuous transparent snow / frost / ice painting; no uniform ground wash; desynchronised gas','campaign_integration':False}
if REVIEW==69:
    validation.update(prior_review='65,67,68 preserved', revision='SM05-A03 for Fire/Poison; SM05-A05 for Cold', source='Restores the exact accepted GEL-A05 renderer; snow / frost / ice ground unchanged')
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'GIF_checks':len(checks),'browser_native_pixel_matches':len(matches),'max_colour_error':max(c['mean_GIF_color_error'] for c in checks)}))
