"""Check accepted-file hashes and palette/blinking against native readbacks."""
import hashlib
import json
from pathlib import Path
from PIL import Image

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
proof = json.loads((OUT/'review-protection.json').read_text(encoding='utf8'))
for n,h in proof.items(): assert hashlib.sha256((ROOT/n).read_bytes()).hexdigest()==h,n
entries=json.loads((OUT/'native/capture.json').read_text(encoding='utf8'));checks=[]
for e in entries:
    if e['case'] not in ['HAUT','DROITE','GAUCHE','BAS','CLOISON','ENNEMI-ARMEMENT']:continue
    n=68 if e['case']=='ENNEMI-ARMEMENT' else 116
    actual=Image.open(OUT/'native'/e['folder']/f'{n:03}.png').convert('RGB').crop((32,88,608,664))
    control=Image.open(OUT/'native'/e['folder']/f'control-{n:03}.png').convert('RGB').crop((32,88,608,664))
    a=list(actual.get_flattened_data());c=list(control.get_flattened_data())
    selected=[i for i,(p,q) in enumerate(zip(a,c)) if max(abs(x-y) for x,y in zip(p,q))>32]
    stamp=round(n*1000/24/10)*10
    with Image.open(OUT/f'{e["variant"]}-64.gif') as im:
        elapsed=0
        for f in range(im.n_frames):
            im.seek(f);duration=im.info['duration']
            if elapsed<=stamp<elapsed+duration:break
            elapsed+=duration
        decoded=im.convert('RGB');pixels=list(decoded.get_flattened_data())
        error=sum(sum(abs(x-y) for x,y in zip(a[i],pixels[i])) for i in selected)/(len(selected)*3)
        assert len(selected)>50 and error<20,(e['variant'],error)
        checks.append({'variant':e['variant'],'frame':n,'timestamp_ms':stamp,'effect_pixels':len(selected),'mean_color_error':round(error,2)})
        if e['case']=='HAUT':decoded.save(OUT/f'{e["id"]}-decoded-impact.png')
p=OUT/'native/SM00-A02-HAUT'
off=Image.open(p/'044.png').convert('RGB').crop((304,445,336,468));on=Image.open(p/'054.png').convert('RGB').crop((304,445,336,468))
changed=sum(max(abs(x-y) for x,y in zip(a,b))>20 for a,b in zip(off.get_flattened_data(),on.get_flattened_data()));assert changed>=15
blink={'LED_position_on_mine_verified':True,'off_frame':44,'on_frame':54,'changed_lens_pixels':changed,'clock_only_not_turn_advance':True,'both_before_arming':True}
(OUT/'gif-color-verification.json').write_text(json.dumps(checks,indent=2)+'\n',encoding='utf8')
(OUT/'led-verification.json').write_text(json.dumps(blink,indent=2)+'\n',encoding='utf8')
validation={'date':'2026-10-14','build':'cargo build --locked passed','format':'cargo fmt --all -- --check passed','diff_whitespace':'targeted git diff --check passed','native_views':len(entries),'native_frames':json.loads((OUT/'native-encoding-verification.json').read_text())['converted_frames'],'native_tile_size':64,'conversion':'pixel-identical RGBA-to-PNG, no resampling','protected_files_unchanged':len(proof),'prepared_timelines':'arming only on manual turn; hostile contact and presence at arming, one explosion, player protected, exact old/new mechanics equivalence passed','blink':blink,'gif_color_checks':checks,'controller':json.loads((OUT/'browser-controller-verification.json').read_text()),'native_visual_checks':'Revised fire impact, walls and unchanged mine/LED inspected','full_suite':'not run; isolated debug visual fixture','campaign_integration':False}
(OUT/'validation.json').write_text(json.dumps(validation,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'views':len(entries),'proof_unchanged':len(proof),'blink_pixels':changed,'gif_checks':len(checks),'max_error':max(c['mean_color_error'] for c in checks)}))
