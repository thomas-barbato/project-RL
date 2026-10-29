"""Pack the existing imagegen grating into accepted circulation contours only."""
from pathlib import Path
import hashlib
import json
import numpy as np
from PIL import Image,ImageEnhance,ImageFilter

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'assets/source-art/floor-future-01'
ART=BASE/'corridor-grate-01'
DOC=ROOT/'docs/reviews/floor-future-01'


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    if (DOC/'acceptance-06.json').exists():
        raise SystemExit('06 is frozen after approval; use build_corridor_wear_review_07.py for the wear proposal.')
    accepted=json.loads((DOC/'acceptance-05.json').read_text(encoding='utf-8'))
    for row in accepted['assets']:assert sha(ROOT/row['path'])==row['after_sha256']
    floor=Image.open(BASE/'floor.png').convert('RGBA');structure=Image.open(BASE/'structure.png').convert('RGBA')
    source=floor.copy();source.alpha_composite(structure);raw=np.array(source)
    ground=np.array(Image.open(BASE/'ground-mask.png').convert('L'))>0
    contours=json.loads((DOC/'ground-contours.json').read_text())
    rooms=np.zeros(ground.shape,dtype=bool);room_boxes=[]
    for trace in contours['traces']:
        if trace['name'] in ('cryo','storage','security','medical','workshop'):
            x0,y0,x1,y1=trace['box']
            # Retain the existing one-pixel steel rim at the material seam.
            # It also prevents nearest texture sampling from bleeding mesh
            # into the last projected room pixel at a half-pixel boundary.
            rooms[max(0,y0-1):y1+1,max(0,x0-1):x1+1]=True
            room_boxes.append(trace['box'])
    corridor=ground&~rooms
    # Native material conditioning only: no new drawing, panel frame or 64px repeat.
    swatch=Image.open(ART/'grating-source.png').convert('RGB').resize((192,192),Image.Resampling.LANCZOS)
    swatch=ImageEnhance.Contrast(swatch).enhance(.5)
    pixels=np.array(swatch).astype('float32')*np.array([1.,1.08,1.18])
    swatch=Image.fromarray(np.clip(pixels,0,255).astype('uint8'))
    atlas=Image.new('RGB',(384,384));atlas.paste(swatch,(0,0));atlas.paste(swatch.transpose(Image.Transpose.FLIP_LEFT_RIGHT),(192,0));atlas.paste(swatch.transpose(Image.Transpose.FLIP_TOP_BOTTOM),(0,192));atlas.paste(swatch.transpose(Image.Transpose.ROTATE_180),(192,192))
    texture=Image.new('RGB',source.size)
    for y in range(0,source.height,384):
        for x in range(0,source.width,384):texture.paste(atlas,(x,y))
    # Use the original baked exposure, just as for 05, with the same shader.
    original=Image.open(BASE/'baseline-r00/floor.png').convert('RGBA')
    light=np.array(original.convert('RGB').filter(ImageFilter.GaussianBlur(32))).astype('float32')
    coverage=np.array(original.getchannel('A').filter(ImageFilter.GaussianBlur(32))).astype('float32')/255
    light/=np.maximum(coverage[...,None],.02)
    reference=np.array(Image.open(ROOT/'assets/source-art/floor-continuous-01/material-source.png').convert('RGB')).mean(axis=(0,1))
    replacement=np.clip(np.array(texture).astype('float32')*light/np.maximum(reference,1),0,255).astype('uint8')
    final=raw.copy();final[corridor,:3]=replacement[corridor]
    assert np.array_equal(final[~corridor],raw[~corridor])
    for x0,y0,x1,y1 in room_boxes:assert np.array_equal(final[y0:y1,x0:x1],raw[y0:y1,x0:x1])
    outputs=[]
    for name,old in [('floor.png',floor),('structure.png',structure)]:
        data=np.array(old);opaque=data[:,:,3]>0;data[opaque,:3]=final[opaque,:3]
        path=ART/name;Image.fromarray(data).save(path)
        assert np.array_equal(data[:,:,3],np.array(old)[:,:,3])
        outputs.append({'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)})
    Image.fromarray((corridor*255).astype('uint8')).save(ART/'corridor-mask.png')
    report={'review':'SOL-FUT01-r02','candidate_number':6,'status':'awaiting_visual_approval','accepted_baseline':5,'rooms_unchanged_pixels':True,'unchanged_room_boxes':room_boxes,'preserved_05_threshold_rim_px':1,'outside_corridor_changed_pixels':0,'corridor_pixels':int(corridor.sum()),'source_alphas_unchanged':True,'grating_source_sha256':sha(ART/'grating-source.png'),'grating_reused_from':'existing built-in imagegen source, formerly tested in rooms as 03; now limited to circulation','native_conditioning':'192px source, mirrored 384px atlas, contrast 0.5, steel tint 1.0/1.08/1.18','outputs':outputs,'new_obstacles_or_gameplay_changes':False}
    (DOC/'corridor-grate-conditioning.json').write_text(json.dumps(report,indent=2)+'\n')
    print('06 prepared: five room rectangles match accepted 05 exactly; only circulation ground changed.')


if __name__=='__main__':main()
