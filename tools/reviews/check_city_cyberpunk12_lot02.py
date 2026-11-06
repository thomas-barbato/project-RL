"""Verify frozen maps, scoped security edits and native before/after masks."""
from pathlib import Path
import hashlib
import json
import re
import urllib.request
from PIL import Image, ImageChops

ROOT=Path(__file__).resolve().parents[2]


def read(p):
    return json.loads((ROOT/p).read_text(encoding='utf-8'))


def sha(p):
    return hashlib.sha256((ROOT/p).read_bytes()).hexdigest()


def nonzero_rgb(image):
    r,g,b=image.convert('RGB').split()
    return ImageChops.lighter(ImageChops.lighter(r,g),b).point(lambda v:255 if v else 0)


def main():
    m=read('assets/source-art/city-cyberpunk12/lot02/manifest.json')
    security=read(m['security_revision'])
    approved=read('docs/reviews/city-cyberpunk12/LOT01-ACCEPTANCE-r01.json')
    scope=read('docs/reviews/city-cyberpunk12/SCOPE.json')
    for p,digest in scope['baseline_sha256'].items():
        assert sha(p)==digest,('Map changed',p)
    for p,digest in approved['source_and_layers'].items():
        assert sha(p)==digest,('Approved source changed',p)
    for p,digest in approved['captures'].items():
        assert sha(p)==digest,('Accepted capture changed',p)
    assert sha(m['base'])==m['base_sha256']==approved['candidate_sha256']
    assert len(security['removed_parts'])==4
    assert not any('-acces' in p['id'] for i in security['items'] for p in i['parts'])
    assert sum('-acces' in p['id'] for i in m['items'] for p in i['parts'])==1
    for i in [*m['items'],*m['roofs'],*security['items']]:
        for p in i['parts']:
            assert sha(p['file'])==p['sha256'],p['id']
    base=Image.open(ROOT/m['base']).convert('RGBA')
    after=Image.open(ROOT/m['candidate']).convert('RGBA')
    assert base.size==after.size==(1664,4224)
    assert base.getchannel('A').tobytes()==after.getchannel('A').tobytes()
    guards=[
        (0,[(0,0,527,54),(0,0,36,339),(479,0,527,339),(0,294,527,339)]),
        (1,[(0,0,495,42),(0,0,35,347),(465,0,495,347),(0,278,495,347)]),
        (2,[(234,133,317,193),(375,15,600,126)]),
    ]
    for index,rects in guards:
        x,y=m['items'][index]['at_px']
        for l,t,r,b in rects:
            box=(x+l,y+t,x+r,y+b)
            assert ImageChops.difference(base.crop(box),after.crop(box)).getbbox(alpha_only=False) is None,('Masonry/threshold/breach changed',index,rects)
    for x,y in [(330,264),(280,285)]:
        r,g,b,_=after.getpixel((458+x,1162+y))
        assert b>=r-2,('Apartment floor does not reach the original wall/threshold',x,y)
    authorized=Image.open(ROOT/'assets/source-art/city-cyberpunk12/lot02/authorized-mask-r01.png').convert('L')
    outside=ImageChops.difference(base,after).convert('RGB')
    outside.paste((0,0,0),mask=authorized.point(lambda v:255 if v else 0))
    assert outside.getbbox() is None
    current=read('assets/source-art/city-cyberpunk12/review-current.json')
    evidence=[]
    for view in current['views']:
        mask=authorized.copy()
        for roof in m['roofs']:
            if roof['index'] in view['hide_roofs']:
                continue
            old_roof=Image.open(ROOT/roof['original']).convert('RGBA')
            new_roof=Image.open(ROOT/roof['candidate']).convert('RGBA')
            assert old_roof.getchannel('A').tobytes()==new_roof.getchannel('A').tobytes()
            delta=nonzero_rgb(ImageChops.difference(old_roof,new_roof))
            overlay=Image.new('L',mask.size);overlay.paste(delta,tuple(roof['at_px']))
            mask=ImageChops.lighter(mask,overlay)
        x,y,w,h=view['rect']
        selected=mask.crop((x,y,x+w,y+h))
        allowed=Image.new('L',(w,h+32));allowed.paste(selected,(0,32));allowed.paste(255,(0,0,w,32))
        before_path=Path('artifacts/city-cyberpunk12')/(view['id']+'-avant.png')
        after_path=Path('artifacts/city-cyberpunk12')/(view['id']+'-apres.png')
        before_frame=Image.open(ROOT/before_path).convert('RGB');after_frame=Image.open(ROOT/after_path).convert('RGB')
        assert before_frame.size==after_frame.size==(w,h+32)
        difference=ImageChops.difference(before_frame,after_frame)
        difference.paste((0,0,0),mask=allowed.point(lambda v:255 if v else 0))
        assert difference.getbbox() is None,('Native pixels changed outside requested areas',view['id'])
        evidence.append({'view':view['id'],'size':[w,h+32],'outside_authorized_pixels_unchanged':True,'before_sha256':sha(before_path),'after_sha256':sha(after_path)})
    player='assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png'
    assert sha(player)=='a1fd3482085814c0464fc9e0d3a6986c129c3565522ade4dd5439befbd8ae270'
    checks=read('artifacts/city-cyberpunk12/checks.json')
    assert checks['resource_cache']['pending']==0
    assert checks['resource_cache']['decoded_source_peak_bytes']<=checks['resource_cache']['decoded_source_budget']
    assert checks['resource_cache']['texture_bytes']<=checks['resource_cache']['texture_budget']
    checks.update(active_map_files_preserved=len(scope['baseline_sha256']),approved_source_files_preserved=len(approved['source_and_layers']),accepted_lot01_captures_preserved=True,other_approved_parts_preserved=True,security_layers_removed=4,ordinary_building_scanners=False,added_access_readers_remaining=1,scanner_controller_visible=False,original_walls_thresholds_and_warehouse_breach_preserved=True,apartment_floor_reaches_original_wall_and_threshold=True,source_actor_preserved=True,native_views=evidence,candidate_sha256=sha(m['candidate']),source_manifest='assets/source-art/city-cyberpunk12/lot02/manifest.json',source_manifest_sha256=sha('assets/source-art/city-cyberpunk12/lot02/manifest.json'),security_manifest=m['security_revision'],validation_scope='Artwork composition and unchanged map geometry. Final campaign lighting, movement and door behavior remain to be inspected in game.')
    (ROOT/'docs/reviews/city-cyberpunk12/VALIDATION-LOT02-r01.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    page=(ROOT/'artifacts/city-cyberpunk12/apercu.html').read_text(encoding='utf-8')
    scripts=re.findall(r'<script(?: [^>]*)?>(.*?)</script>',page,re.S)
    (ROOT/'artifacts/city-cyberpunk12/apercu-script.js').write_text(scripts[-1],encoding='utf-8')
    with urllib.request.urlopen('http://127.0.0.1:8876/artifacts/city-cyberpunk12/apercu.html',timeout=3) as response:
        assert response.status==200
    print(json.dumps({'map_files_unchanged':len(scope['baseline_sha256']),'approved_sources_unchanged':len(approved['source_and_layers']),'native_views_checked':len(evidence),'wall_and_threshold_pixels_preserved':True,'security_correction_scoped':True}))


if __name__=='__main__':
    main()
