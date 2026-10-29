"""Condition the generated material into isolated floor-layer candidates.

Artwork comes from imagegen. This script preserves source geometry/alpha,
conditions its native scale and retains broad source illumination for the
existing light-compensation shader. No live map or collision file is changed.
"""
from pathlib import Path
import hashlib
import json
import sys
from PIL import Image, ImageDraw, ImageFilter, ImageChops
import numpy as np

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/floor-continuous-01'
DOC=ROOT/'docs/reviews/floor-continuous-01'


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path):return json.loads(path.read_text(encoding='utf-8'))


def material(size,source_path=None):
    # Native conditioning of a large source, never a repeated 64 px slab.
    source=Image.open(source_path or ART/'material-source.png').convert('RGB').resize((512,512),Image.Resampling.LANCZOS)
    # Mirrored atlas borders have identical edge pixels, preventing tile seams.
    atlas=Image.new('RGB',(1024,1024))
    atlas.paste(source,(0,0));atlas.paste(source.transpose(Image.Transpose.FLIP_LEFT_RIGHT),(512,0))
    atlas.paste(source.transpose(Image.Transpose.FLIP_TOP_BOTTOM),(0,512))
    atlas.paste(source.transpose(Image.Transpose.ROTATE_180),(512,512))
    result=Image.new('RGB',size)
    for y in range(0,size[1],1024):
        for x in range(0,size[0],1024):result.paste(atlas,(x,y))
    return result


def condition(source,mask,target,texture=None):
    old=Image.open(source).convert('RGBA');rgb=old.convert('RGB')
    coverage=old.getchannel('A')
    # Normalized convolution avoids borrowing black void pixels at map edges.
    light=np.array(rgb.filter(ImageFilter.GaussianBlur(32)),dtype=np.float32)
    cov=np.array(coverage.filter(ImageFilter.GaussianBlur(32)),dtype=np.float32)/255
    light/=np.maximum(cov[...,None],0.02)
    tex=np.array(texture or material(old.size),dtype=np.float32)
    reference=np.array(Image.open(ART/'material-source.png').convert('RGB'),dtype=np.float32).mean(axis=(0,1))
    replacement=Image.fromarray(np.clip(tex*light/np.maximum(reference,1),0,255).astype('uint8')).convert('RGBA')
    candidate=Image.composite(replacement,old,mask);candidate.putalpha(coverage)
    candidate.save(target)
    changed=ImageChops.difference(old,candidate).convert('RGB')
    outside=ImageChops.multiply(changed,ImageChops.invert(mask).point(lambda a:255 if a==255 else 0).convert('RGB'))
    assert outside.getbbox() is None,'Pixels outside the authorized floor mask changed'
    assert old.getchannel('A').tobytes()==candidate.getchannel('A').tobytes()
    return {'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),'candidate':target.relative_to(ROOT).as_posix(),'candidate_sha256':sha(target),'size':list(old.size),'outside_mask_changed':False,'alpha_unchanged':True}


def main():
    if (DOC/'acceptance-r01.json').exists():
        raise SystemExit('SOL-CONT01-r01 is frozen after exterior integration; use build_camp_floor_review_02.py for the current interior proposal.')
    ART.mkdir(parents=True,exist_ok=True);DOC.mkdir(parents=True,exist_ok=True)
    camp=ROOT/'assets/maps/cryogenic-camp-01/floor.png'
    source=Image.open(camp).convert('RGBA')
    mask=source.getchannel('A').filter(ImageFilter.MinFilter(17)).filter(ImageFilter.GaussianBlur(2))
    # Preserve dark hardware attached to an edge, without mistaking cool
    # highlights on an old floor joint for a piece of metal.
    pixels=np.array(source)
    near_edge=np.array(source.getchannel('A').filter(ImageFilter.MinFilter(65)))<255
    keep=(pixels[:,:,:3].max(axis=2)<55)&near_edge
    keep=np.array(Image.fromarray((keep*255).astype('uint8')).filter(ImageFilter.MaxFilter(7)))>0
    arr=np.array(mask);arr[keep]=0;mask=Image.fromarray(arr)
    mask.save(ART/'camp-mask.png')
    records=[condition(camp,mask,ART/'camp-floor.png')]

    # Recompose editable ground beneath the intact source modules; visual
    # paving boundaries must never follow collision-grid/obstacle cells.
    sys.path.insert(0,str(ROOT/'tools/reviews'))
    import build_street_paving_review as paving
    from integrate_street_repairs_09 import REMOVED
    repair=ROOT/'assets/source-art/first-sector-review-01/u01-repairs-r09'
    tiles=[Image.open(paving.ART/f'{name}-native.png').convert('RGBA') for name in ['PAV-U01-06','PAV-U01-06B','PAV-U01-06C']]
    mix=[tiles[0]]+[tiles[1]]*6+[tiles[2]]*5
    grass=read(paving.ART/'composition-checks.json')['grass_at_px']
    worn={(x,y):tiles[1+(x+y)%2] for y in (7,8) for x in range(9,13)}
    kwargs=dict(grass=grass,warehouse=repair/'warehouse-native.png',repair_join=True,omit_modules=REMOVED)
    original_asphalt=Image.open(repair/'ROAD-U01-09-conditioned.png').convert('RGBA')
    baseline=paving.compose(mix,asphalt=original_asphalt,paving_overrides=worn,**kwargs)
    active_path=ROOT/'assets/maps/prison-street-expanded-01/street.png'
    active=Image.open(active_path).convert('RGBA')
    assert baseline.tobytes()==active.tobytes(),'Editable baseline must match the real city exactly'

    # The preserved southern arrival is also a source layer. Its continuous
    # ground is conditioned inside authored visual polygons, never grid cells.
    arrival_path=ROOT/'assets/maps/prison-street-01/street.png'
    arrival=Image.open(arrival_path).convert('RGBA')
    road_region=Image.new('L',arrival.size);rd=ImageDraw.Draw(road_region)
    rd.polygon([(0,401),(955,401),(981,395),(1004,380),(1021,358),(1033,333),(1040,311),(1040,0),(1193,0),(1193,180),(1201,206),(1215,227),(1237,243),(1262,252),(1536,252),(1536,316),(1197,316),(1170,323),(1146,339),(1128,362),(1119,389),(1116,420),(1116,552),(1106,577),(1090,594),(1063,605),(1013,609),(605,609),(579,615),(555,631),(537,655),(527,681),(524,714),(524,1024),(302,1024),(302,691),(294,663),(277,641),(253,625),(227,618),(0,618)],fill=255)
    rd.rectangle((0,968,1536,1024),fill=255)
    paving_region=Image.new('L',arrival.size);d=ImageDraw.Draw(paving_region)
    polygons=[[(82,329),(950,329),(972,312),(984,288),(984,0),(1020,0),(1020,309),(1013,334),(1000,355),(980,371),(956,380),(82,380)],
        [(1213,0),(1232,0),(1232,172),(1536,172),(1536,229),(1263,229),(1241,223),(1226,210),(1216,192)],
        [(0,638),(225,638),(247,643),(265,657),(278,677),(282,699),(282,842),(0,842)],
        [(547,711),(552,685),(565,661),(584,644),(608,638),(1168,638),(1168,648),(704,648),(704,842),(547,842)],
        [(0,929),(302,929),(302,946),(0,946)],
        [(526,929),(1536,929),(1536,946),(526,946)],
        [(326,866),(490,866),(490,907),(326,907)]]
    for points in polygons:d.polygon(points,fill=255)
    # The polygons stop inside the original curved curbs. Whole objects and
    # lane paint are excluded separately; no selection follows tile joints.
    raw=np.array(arrival).astype('float32');r,g,b=raw[:,:,0],raw[:,:,1],raw[:,:,2]
    road=np.array(road_region)>0
    paved=np.array(paving_region)>0
    road &= (b>=r-2)&(r-g<18)&(raw[:,:,:3].max(axis=2)<170)
    # Flat floor pigment excludes vegetation and blue metal objects.
    paved &= (r>=b+4)&(r>=g-8)&(r-g<22)
    # The gate sill is an unobstructed floor source, with its mechanism outside.
    paved[866:908,326:491]=True
    lane_paint=(r>125)&(g>85)&(b<r*.65)
    road &= ~lane_paint
    paint=Image.fromarray(((road|paved)*255).astype('uint8'));d=ImageDraw.Draw(paint)
    # Keep road paint, the gate mechanism, drainage grates, papers and bollards.
    for box in [(296,842,530,866),(296,908,530,929),
                (720,394,775,429),(1037,194,1064,241),(105,414,142,449),(617,482,660,516),(1080,534,1143,578),
                (625,295,710,350),(807,301,958,350),(971,79,1018,180),(975,310,1022,354),
                (228,764,326,943),(491,764,576,943),(160,710,250,839)]:d.rectangle(box,fill=0)
    for x,y in [(273,365),(501,365),(1243,190),(1415,211),(240,630),(270,746),(1121,602)]:d.rectangle((x-14,y-27,x+14,y+19),fill=0)
    # No blur across protected source-object pixels or curb edges.
    records.append(condition(arrival_path,paint,ART/'arrival-source.png'))
    arrival=Image.open(ART/'arrival-source.png').convert('RGBA')

    size=active.size;whole=material(size).convert('RGBA')
    # Retain the old material's mean pigment; the existing lighting pass is kept.
    old_mean=np.array(tiles[2])[:,:,:3].mean(axis=(0,1));new_mean=np.array(whole)[:,:,:3].mean(axis=(0,1))
    pixels=np.array(whole);pixels[:,:,:3]=np.clip(pixels[:,:,:3]*old_mean/new_mean,0,255).astype('uint8');whole=Image.fromarray(pixels)
    turns=[None,Image.Transpose.ROTATE_90,Image.Transpose.ROTATE_180,Image.Transpose.ROTATE_270]
    inverse=[None,Image.Transpose.ROTATE_270,Image.Transpose.ROTATE_180,Image.Transpose.ROTATE_90]
    overrides={}
    for y in range(66):
        for x in range(26):
            v=((x*0x1f123bb5)^(y*0x5f356495)^0x6a09e667)&0xffffffff;v^=v>>16;v=(v*0x45d9f3b)&0xffffffff;v^=v>>16
            patch=whole.crop((x*64,y*64,x*64+64,y*64+64));turn=inverse[(v>>12)%4]
            overrides[(x,y)]=patch if turn is None else patch.transpose(turn)
    # Reuse the already approved irregular asphalt, with mirrored native edges.
    road=Image.open(repair/'ROAD-U01-09-native.png').convert('RGBA').resize((128,128),Image.Resampling.LANCZOS)
    asphalt=Image.new('RGBA',(256,256));asphalt.paste(road,(0,0));asphalt.paste(road.transpose(Image.Transpose.FLIP_LEFT_RIGHT),(128,0));asphalt.paste(road.transpose(Image.Transpose.FLIP_TOP_BOTTOM),(0,128));asphalt.paste(road.transpose(Image.Transpose.ROTATE_180),(128,128))
    candidate=paving.compose(mix,asphalt=asphalt,paving_overrides=overrides,arrival_override=arrival,**kwargs)
    candidate.save(ART/'street-0.png')
    entry_path=ROOT/read(ROOT/'assets/maps/prison-sewer-01/entry.json')['background']
    entry=Image.open(entry_path).convert('RGBA')
    delta=np.any(np.array(entry)!=np.array(active),axis=2)
    final=np.array(candidate);final[delta]=np.array(entry)[delta]
    Image.fromarray(final).save(ART/'street-1.png')
    for i,path in enumerate([active_path,entry_path]):
        records.append({'source':path.relative_to(ROOT).as_posix(),'source_sha256':sha(path),'candidate':(ART/f'street-{i}.png').relative_to(ROOT).as_posix(),'candidate_sha256':sha(ART/f'street-{i}.png'),'size':list(size),'whole_source_modules_preserved':True,'baseline_recomposition_byte_identical':True,'sewer_entry_patch_preserved':True})
    report={'review':'SOL-CONT01-r01','status':'candidate_not_integrated','native_cell':64,'generated_material':'material-source.png','conditioning':'512px native material, mirrored atlas without seams; broad original source illumination retained; original geometry and alpha unchanged','gameplay_layout_or_collision_changed':False,'records':records}
    (DOC/'conditioning.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(records))


if __name__=='__main__':main()
