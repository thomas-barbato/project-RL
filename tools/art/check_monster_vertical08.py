"""Native checks and original silhouette breadth for models 008, 034 and 036."""
import json
import hashlib
import statistics
from PIL import Image
from audit_monster_vertical08_colors import stats
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-08/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-08/r01/current'
base.REVIEW='MONSTER-VERTICAL08-r01'
base.NUMBERS=[8,34,36]
base.RECORD='MONSTER-VERTICAL08-VALIDATION.json'

if __name__=='__main__':
    manifest=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'))
    palette=json.loads((base.SOURCE/'MON-036-front-material-palette.json').read_text(encoding='utf-8'))
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/palette[file]).read_bytes()).hexdigest()==palette[key]
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    blue=next(p for p in captures['items'] if p['id']=='MON-036-0')
    assert blue['geometry_and_alpha_unchanged'] and blue['palette_changed_pixels']>600
    breadth=[];rear_ochre_pixels=0
    for p in manifest['profiles']:
        front=Image.open(base.ROOT/p['idle_sprite']).getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
        width=front[2]-front[0]
        for direction in ['down','up']:
            bounds=[b['bounds'] for b in p['bounds'] if b['direction']==direction]
            moving_width=statistics.median(b[2]-b[0] for b in bounds)
            # One native pixel accommodates quantisation of whole-figure scaling.
            assert .80*width-1<=moving_width<=1.20*width+1,(p['id'],direction,moving_width,width)
            breadth.append({'id':p['id'],'direction':direction,'moving_to_idle_median_width':moving_width/width})
        if p['number']==36:
            atlas=Image.open(base.ROOT/p['atlas']).convert('RGBA')
            for b in [b for b in p['bounds'] if b['direction']=='up']:
                x0,y0,x1,y1=b['bounds'];height=y1-y0
                upper_back=atlas.crop((b['phase']*160+x0,128+y0+round(.20*height),b['phase']*160+x1,128+y0+round(.65*height)))
                rear_ochre_pixels+=stats(upper_back,'ochre_throat')['pixels']
            assert rear_ochre_pixels==0,('Frontal ochre patch repeated on rear',rear_ochre_pixels)
    base.main()
    path=base.REV/base.RECORD;report=json.loads(path.read_text(encoding='utf-8'))
    report['silhouette_breadth']=breadth
    report['blue_rear_upper_body_ochre_pixels']=rear_ochre_pixels
    path.write_text(json.dumps(report,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
