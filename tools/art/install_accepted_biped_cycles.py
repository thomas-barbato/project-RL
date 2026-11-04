"""Install only reviewed monster poses, retaining the frontal shown at arrival."""
import hashlib
import json
import re
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'assets/animations/biped-lateral-01'
REVIEWS = ROOT/'docs/reviews/animation-polish-01'


def main():
    frozen = []
    for name in ['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json', 'BIPED-LATERAL03-ACCEPTANCE-r04.json', 'BIPED-LATERAL04-ACCEPTANCE-r01.json']:
        record = json.loads((REVIEWS/name).read_text(encoding='utf-8'))
        frozen += record['assets']+record.get('front_references_retained', [])
    for asset in frozen:
        assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest() == asset['sha256']
    profiles = json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-03/r04/manifest.json').read_text(encoding='utf-8'))['profiles']
    profiles += json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-04/r01/manifest.json').read_text(encoding='utf-8'))['profiles'][:3]
    assert sorted(p['number'] for p in profiles) == [5,14,39,144,145,146,151,156,164,165]
    vertical = json.loads((ROOT/'assets/animations/vertical-18/manifest.json').read_text(encoding='utf-8'))
    catalog = (ROOT/'assets/sprites/campaign-monsters/catalog.rs').read_text(encoding='utf-8')
    entries = []; provenance = []; OUT.mkdir(parents=True, exist_ok=True)
    for p in sorted(profiles, key=lambda p:p['number']):
        n=p['number']; width=p.get('frame_width',128)
        source=Image.open(ROOT/p['atlas']).convert('RGBA'); atlas=source.copy()
        e=next(e for e in vertical['monsters'] if e['number']==n and e['variant']==0)
        shadow=int(re.search(r'number: '+str(n)+r', variant: 0,.*?shadow: (\d+)',catalog)[1])
        front=Image.new('RGBA',(width,128))
        if p.get('idle_sprite'):
            front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
            front_source=p['idle_sprite']; front_shadow=shadow if n in [39,146,151] else 17
        else:
            w,h=e['cell']; canonical=Image.open(ROOT/e['file']).convert('RGBA').crop((0,2*h,w,3*h))
            front.alpha_composite(canonical,(width//2-e['anchor'][0],112-e['anchor'][1]))
            front_source=e['file']; front_shadow=shadow
        assert front.size==(width,128)
        for side in range(2):
            atlas.paste(front,(8*width,side*128))
            assert atlas.crop((0,side*128,8*width,(side+1)*128)).tobytes()==source.crop((0,side*128,8*width,(side+1)*128)).tobytes()
        file=OUT/f'MON-{n:03}-0.png'; atlas.save(file)
        entries.append({'number':n,'variant':0,'file':file.relative_to(ROOT).as_posix(),'cell':[width,128],'anchor':[width//2,112],
                        'vertical_offset':p['vertical_offset'],'walk_shadow':shadow if n in [39,146,151] else 17,'front_shadow':front_shadow})
        provenance.append({'number':n,'file':entries[-1]['file'],'sha256':hashlib.sha256(file.read_bytes()).hexdigest(),
                           'walk_source':p['atlas'],'walk_sha256':p['sha256'],'front_source':front_source,'walk_pixels_unchanged':True})
    manifest={'format_version':1,'native_tile_px':64,'step_ms':400,'cycle_ms':800,'phase_weights':[180,180,170,120]*2,'entries':entries}
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
    (REVIEWS/'BIPED-INTEGRATION01-ASSETS.json').write_text(json.dumps({'review':'BIPED-INTEGRATION01','authorization':'tu peux integrer :)','models':provenance,'frozen_asset_hashes_checked':len(frozen)},indent=2)+'\n',encoding='utf-8')
    print('Installed 10 accepted cycles; 160 moving poses preserved; original arrival fronts retained')


if __name__=='__main__': main()
