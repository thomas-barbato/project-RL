"""Four catalogue corrections; unchanged anatomy and common weapon attachment."""
from pathlib import Path
from functools import lru_cache
import json, hashlib, argparse
from PIL import Image, ImageChops
import build_armor_flat_review_01 as art

BASE=Path(__file__).resolve().parent
OUT=BASE/'armor-flat-review-03'
PREVIOUS=BASE/'armor-flat-review-01'
ACCEPTED=BASE/'armor-flat-review-02'
art.OUT=OUT
art.CAT=json.loads((OUT/'catalogue.json').read_text('utf-8'))
art.ENTRIES={e['id']:e for e in art.CAT['entries']}
art.RAMPS.update({
 'workpetrol':['#1b2d3b','#324d60','#557189'],
 'silver':['#505b65','#a3adb7','#e1e8eb'],
 'industrialyellow':['#241b0a','#e2ae31','#ffe066'],
})
art.MATERIALS['AR-12']=('workpetrol',None,'olive',None,'red',None)
art.MATERIALS['AR-N14']=('yellow','steelblue','petrol','bone','red','steelblue')

old_source=art.source
@lru_cache(maxsize=40)
def exact_source(ident,pose):
    if ident not in ('AR-08','AR-10','AR-11'):return old_source(ident,pose)
    original=Image.open(PREVIOUS/f'sources/{ident}-poses.png').convert('RGBA')
    i=art.POSES.index(pose);w,h=original.size
    original=original.crop((i%2*w//2,i//2*h//2,(i%2+1)*w//2,(i//2+1)*h//2)).resize((1254,1254),Image.Resampling.LANCZOS)
    if ident=='AR-08':
        # Only the generated helmet crop is used. Clothes, arm surfaces and
        # gloves keep their exact previous source pixels.
        painted=old_source(ident,pose)
        box=(500,55,770,310)
        original.paste(painted.crop(box),box)
    return original
art.source=exact_source

old_materials=art.author_materials
targets={
 'AR-10':[['workpetrol','silver'],['red','silver'],['olive','silver']],
 'AR-11':[['steelblue','ivory'],['olive','bone'],['red','steelblue']],
}
def materials(ident):
    if ident=='AR-N14':
        data=json.loads((OUT/'palettes/AR-N14.json').read_text('utf-8'))
        # Paint shadows extend below the generic yellow ramp. Declare their
        # actual source shades so near-black edges do not jump to bright red.
        data['source_materials']=['industrialyellow','steelblue']
        return data
    if ident not in targets:return old_materials(ident)
    # Existing indexed material maps are native editable sources. Reusing them
    # keeps every fold, contour, glove and alpha pixel exact for palette edits.
    data=json.loads((PREVIOUS/f'palettes/{ident}.json').read_text('utf-8'))
    data['variants']=targets[ident]
    # Complete the material assignments for shaded cloth, without touching
    # the neutral joint/leather entries. No new source geometry is generated.
    extra={
      'AR-10':{'principal':[26,27,30,32,33,41,43,44,45], 'secondary':[25,51]},
      'AR-11':{'principal':[10,48,52,53,55,60], 'secondary':[28,46,50]},
    }[ident]
    for material,indices in extra.items():
        for i in indices:
            for values in data['material_indices'].values():
                if i in values:values.remove(i)
            data['material_indices'][material].append(i)
    data['explicit_assignment_correction']=extra
    data['color_reference']=f'sources/{ident}-poses.png'
    data['method']='Existing explicit indexed materials; new color proposal from built-in imagegen; unchanged source geometry'
    (OUT/f'palettes/{ident}.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
    return data
art.author_materials=materials
old_variant=art.material_variant
def variant(im,data,v):
    if data['id'] in targets:
        local=dict(data);local['variants']=[data['source_materials'],data['variants'][v]]
        result=old_variant(im,local,1)
        # Quantization can associate a neutral pixel with a nearby colored
        # palette sample. Protect genuinely neutral source RGB at every edge.
        r,g,b=im.convert('RGB').split()
        chroma=ImageChops.subtract(ImageChops.lighter(r,ImageChops.lighter(g,b)),ImageChops.darker(r,ImageChops.darker(g,b)))
        mask=chroma.point(lambda x:255 if x>=6 else 0)
        return Image.composite(result,im,mask)
    result=old_variant(im,data,v)
    if data['id']=='AR-N14' and v:
        r,g,b=im.convert('RGB').split()
        # Warm reflections on grey steel are not painted yellow. The yellow
        # source material has a broad chroma; keep metal and near-black ink.
        warm=ImageChops.multiply(ImageChops.subtract(r.point(lambda x:round(x*.5)),b).point(lambda x:255 if x>0 else 0),
             r.point(lambda x:255 if x>40 else 0))
        result=Image.composite(result,im,warm)
    if data['id']=='AR-08':
        # All copper helmet colors remain independent of cloth variants.
        box=(0,0,art.MS,24*art.F)
        result.paste(im.crop(box),box)
    return result
art.material_variant=variant

def validate(data):
    count=0;grips=0
    for e in data['entries']:
        ident=e['id']
        for key,view in e['views'].items():
            n=Image.open(OUT/view['native32']).convert('RGBA')
            b=Image.open(OUT/view['display64']).convert('RGBA')
            assert n.size==(64,64)
            assert b.tobytes()==n.resize((128,128),Image.Resampling.NEAREST).tobytes()
            bounds=n.getchannel('A').getbbox();assert bounds
            assert min(bounds[:2])>0 and bounds[2]<64 and bounds[3]<64,(ident,key,bounds)
            count+=1
        for pose in art.POSES:
            base=art.body(ident,pose)
            for v in range(3):assert variant(base,materials(ident),v).getchannel('A').tobytes()==base.getchannel('A').tobytes()
            for box in ((0,0,art.MS,24*art.F),(0,38*art.F,art.MS,art.MS)):
                assert base.crop(box).tobytes()==art.body(ident,'independent').crop(box).tobytes(),(ident,pose,box)
        for c in data['configurations']:
            for side in (0,1):
                layers=[art.rig.item_layer(i,c['pose'],s)[0] for i,s in c['items'] if s==side or s is None]
                shield=any(s==side and art.rig.old.BY_ID[i]['kind']=='shield' for i,s in c['items'])
                if layers and not shield:
                    glove=art.fingers(ident,c['pose'],side)
                    assert any(ImageChops.multiply(glove.getchannel('A'),l.getchannel('A')).getbbox() for l in layers)
                    grips+=3
    old=json.loads((PREVIOUS/'review.json').read_text('utf-8'))
    assert data['common_rig']==old['common_rig']
    assert data['weapon_transforms']==old['weapon_transforms']
    for p,h in data['protected_active_files'].items():assert hashlib.sha256((art.ROOT/p).read_bytes()).hexdigest()==h,p
    accepted=json.loads((ACCEPTED/'visual-acceptance-01.json').read_text('utf-8'))
    for p,h in accepted['frozen_files'].items():assert hashlib.sha256((ACCEPTED/p).read_bytes()).hexdigest()==h,p
    for c in data['configurations']:
        f=f'native32/AR-N14-V0-{c["number"]}.png'
        assert (OUT/f).read_bytes()==(ACCEPTED/f).read_bytes(),('very heavy reference changed',f)
    report=dict(review_id='ARMOR-FLAT03',revised_ids=list(art.ENTRIES),variants_per_outfit=3,
        configurations_per_outfit=16,exact_nearest_doubling=count,checked_grip_overlaps=grips,
        common_rig_unchanged=True,common_weapon_transforms_unchanged=True,
        palette_alpha_preserved=True,palette_only_source_geometry_preserved=['AR-10','AR-11'],
        very_heavy_reference_compositions_preserved=16,
        fixed_heads_and_lower_bodies_across_poses=True,clipped_views=[],
        accepted_flat02_files_unchanged=len(accepted['frozen_files']),
        active_files_unchanged=len(data['protected_active_files']),
        active_game_content_changed=False,visual_acceptance_pending=True)
    (OUT/'validation.json').write_text(json.dumps(report,indent=2),'utf-8')
    print(json.dumps(report),flush=True)
art.validate=validate

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--build',action='store_true');parser.add_argument('--only')
    args=parser.parse_args()
    art.build(args.only.split(',') if args.only else None) if args.build else art.conditioning()
