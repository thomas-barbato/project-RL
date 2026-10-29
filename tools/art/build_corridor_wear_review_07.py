"""Composite imagegen wear onto the existing metal bars near circulation edges."""
from pathlib import Path
import hashlib
import json
import numpy as np
from PIL import Image,ImageEnhance,ImageFilter

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'assets/source-art/floor-future-01'
SIX=BASE/'corridor-grate-01'
ART=BASE/'corridor-wear-07'
DOC=ROOT/'docs/reviews/floor-future-01'


def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text(encoding='utf-8'))


def main():
    if (DOC/'acceptance-07.json').exists():
        raise SystemExit('07 is frozen after approval; preserve the accepted sources and native references.')
    accepted=read(DOC/'acceptance-06.json')
    for r in accepted['assets']:assert sha(ROOT/r['path'])==r['after_sha256']
    floor=Image.open(SIX/'floor.png').convert('RGBA');structure=Image.open(SIX/'structure.png').convert('RGBA')
    source=floor.copy();source.alpha_composite(structure);raw=np.array(source)
    mask_image=Image.open(SIX/'corridor-mask.png').convert('L');mask=np.array(mask_image)>0
    wear=Image.open(ART/'wear-source.png').convert('RGBA')
    a=np.array(wear.getchannel('A'));assert a.min()==0 and a.max()>100 and np.mean(a==0)>.4
    layer=Image.new('RGBA',source.size)
    # Each circulation section uses one continuous decal, not one per game cell.
    sections=[((704,447,2368,642),False),((444,490,718,580),False),((2358,490,2432,580),True),
        ((1080,328,1162,447),True),((1912,328,1994,447),True),((1080,643,1162,739),True),((1912,643,1994,739),True)]
    for (x0,y0,x1,y1),vertical in sections:
        patch=wear.transpose(Image.Transpose.ROTATE_90) if vertical else wear
        patch=patch.resize((x1-x0,y1-y0),Image.Resampling.LANCZOS)
        layer.alpha_composite(patch,(x0,y0))
    # Reconstruct only the accepted material's bar coverage, preserving apertures.
    native=Image.open(SIX/'grating-source.png').convert('RGB').resize((192,192),Image.Resampling.LANCZOS)
    native=ImageEnhance.Contrast(native).enhance(.5)
    v=np.array(native).astype('float32')*np.array([1.,1.08,1.18]);native=Image.fromarray(np.clip(v,0,255).astype('uint8'))
    atlas=Image.new('RGB',(384,384));atlas.paste(native,(0,0));atlas.paste(native.transpose(Image.Transpose.FLIP_LEFT_RIGHT),(192,0));atlas.paste(native.transpose(Image.Transpose.FLIP_TOP_BOTTOM),(0,192));atlas.paste(native.transpose(Image.Transpose.ROTATE_180),(192,192))
    texture=Image.new('RGB',source.size)
    for y in range(0,source.height,384):
        for x in range(0,source.width,384):texture.paste(atlas,(x,y))
    lum=np.array(texture).astype('float32').mean(axis=2)
    low,high=np.percentile(lum,[45,75]);bars=np.clip((lum-low)/max(high-low,1),0,1)
    edge=1-np.array(mask_image.filter(ImageFilter.BoxBlur(22))).astype('float32')/255
    edge=np.clip(edge*2.5,0,1)
    decal=np.array(layer).astype('float32')
    alpha=(decal[:,:,3]/255)*bars*(.3+.7*edge)*.95*mask
    original=Image.open(BASE/'baseline-r00/floor.png').convert('RGBA')
    light=np.array(original.convert('RGB').filter(ImageFilter.GaussianBlur(32))).astype('float32')
    cov=np.array(original.getchannel('A').filter(ImageFilter.GaussianBlur(32))).astype('float32')/255
    light/=np.maximum(cov[...,None],.02)
    ref=np.array(Image.open(ROOT/'assets/source-art/floor-continuous-01/material-source.png').convert('RGB')).mean(axis=(0,1))
    color=np.clip(decal[:,:,:3]*light/np.maximum(ref,1),0,255)
    final=raw.copy();final[:,:,:3]=np.clip(raw[:,:,:3]*(1-alpha[:,:,None])+color*alpha[:,:,None],0,255).astype('uint8')
    untouched=(alpha==0);assert np.array_equal(final[untouched],raw[untouched])
    assert np.array_equal(final[~mask],raw[~mask])
    holes=mask&(bars==0);assert np.array_equal(final[holes],raw[holes])
    six=read(DOC/'corridor-grate-conditioning.json')
    for x0,y0,x1,y1 in six['unchanged_room_boxes']:assert np.array_equal(final[y0:y1,x0:x1],raw[y0:y1,x0:x1])
    outputs=[]
    for name,old in [('floor.png',floor),('structure.png',structure)]:
        data=np.array(old);opaque=data[:,:,3]>0;data[opaque,:3]=final[opaque,:3]
        path=ART/name;Image.fromarray(data).save(path);outputs.append({'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)})
        assert np.array_equal(data[:,:,3],np.array(old)[:,:,3])
    changed=np.any(final[:,:,:3]!=raw[:,:,:3],axis=2)
    Image.fromarray(np.clip(alpha*255,0,255).astype('uint8')).save(ART/'wear-mask.png')
    report={'review':'SOL-FUT01-r03','candidate_number':7,'status':'awaiting_visual_approval','accepted_baseline':6,'wear_source_sha256':sha(ART/'wear-source.png'),'source_has_genuine_transparency':True,'rooms_05_unchanged':True,'outside_circulation_changed_pixels':0,'mesh_apertures_changed_pixels':0,'aperture_pixels_preserved':int(holes.sum()),'changed_pixels':int(changed.sum()),'changed_edge_pixels':int(np.count_nonzero(changed&(edge>.4))),'changed_centre_pixels':int(np.count_nonzero(changed&(edge<=.4))),'source_alphas_unchanged':True,'sections':sections,'outputs':outputs,'gameplay_changed':False}
    (DOC/'wear-07-conditioning.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(report))


if __name__=='__main__':main()
