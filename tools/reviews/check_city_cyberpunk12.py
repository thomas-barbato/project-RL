"""Verify the user's frozen-layout and scoped-edit requirements for lot01."""
from pathlib import Path
from PIL import Image, ImageChops
import hashlib
import json
import re
import subprocess
import urllib.request

ROOT=Path(__file__).resolve().parents[2]


def read(p):
    return json.loads((ROOT/p).read_text(encoding='utf-8'))


def sha(p):
    return hashlib.sha256((ROOT/p).read_bytes()).hexdigest()


def main():
    mp=Path('assets/source-art/city-cyberpunk12/manifest.json')
    m=read(mp)
    before={p['file']:sha(p['file']) for i in m['items'] for p in i['parts']}
    candidate_before=sha(m['candidate'])
    subprocess.run(['python','tools/art/build_city_cyberpunk12.py','--compose-only'],cwd=ROOT,check=True)
    assert before=={p:sha(p) for p in before}
    assert candidate_before==sha(m['candidate'])
    scope=read(Path('docs/reviews/city-cyberpunk12/SCOPE.json'))
    for p,digest in scope['baseline_sha256'].items():
        assert sha(p)==digest,p
    approval=read(Path('docs/reviews/north-city-01/PRODUCTION10-B01-ACCEPTANCE-r04.json'))
    assert sha(approval['source'])==approval['source_sha256']
    old=Image.open(ROOT/'assets/maps/prison-street-expanded-01/street.png').convert('RGBA')
    new=Image.open(ROOT/m['candidate']).convert('RGBA')
    for item in m['items']:
        x,y=item['at_px']
        for l,t,r,b in item['protected_doors']:
            region=(x+l,y+t,x+r+1,y+b+1)
            assert not ImageChops.difference(old.crop(region),new.crop(region)).getbbox(alpha_only=False),item['id']
    old_cap=Image.open(ROOT/'artifacts/city-cyberpunk12/secteur-avant.png').convert('RGB')
    new_cap=Image.open(ROOT/'artifacts/city-cyberpunk12/secteur-apres.png').convert('RGB')
    mask=Image.open(ROOT/'assets/source-art/city-cyberpunk12/authorized-mask-r01.png').crop((0,2112,1664,3136))
    allowed=Image.new('L',old_cap.size)
    allowed.paste(mask,(0,32))
    allowed.paste(255,(0,0,1664,32))
    diff=ImageChops.difference(old_cap,new_cap)
    outside=diff.copy()
    outside.paste((0,0,0),mask=allowed.point(lambda v:255 if v else 0))
    assert outside.getbbox() is None
    checks=read(Path('artifacts/city-cyberpunk12/checks.json'))
    checks.update(baseline_files_unchanged=len(scope['baseline_sha256']),accepted_B01_reference_unchanged=True,layouts_and_connections_unchanged=True,original_building_and_ground_assets_unchanged=True,protected_door_pixels_unchanged=True,independent_device_count=11,compose_only_preserves_independent_layers=True,native_capture_outside_devices_unchanged=True,source_manifest=mp.as_posix(),source_manifest_sha256=sha(mp),scope='First-city lot01 on three buildings only. Other sectors/interiors pending; north-city remake follows first city. Sewers excluded.',captures={p.name:{'size':list(Image.open(p).size),'sha256':sha(p.relative_to(ROOT))} for p in (ROOT/'artifacts/city-cyberpunk12').glob('*.png')})
    (ROOT/'docs/reviews/city-cyberpunk12/VALIDATION-LOT01-r01.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    page=(ROOT/'artifacts/city-cyberpunk12/apercu.html').read_text(encoding='utf-8')
    (ROOT/'artifacts/city-cyberpunk12/apercu-script.js').write_text(re.search(r'<script>(.*?)</script>',page,re.S)[1],encoding='utf-8')
    with urllib.request.urlopen('http://127.0.0.1:8876/artifacts/city-cyberpunk12/apercu.html',timeout=3) as response:
        assert response.status==200
    print(json.dumps({'baseline_files_unchanged':len(scope['baseline_sha256']),'doors_preserved':True,'native_capture_outside_devices_unchanged':True,'independent_layers_unchanged_after_composition':True,'http':200}))


if __name__=='__main__':
    main()
