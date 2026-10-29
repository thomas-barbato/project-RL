"""Replace the approved concrete beneath intact source architecture across both layers."""
from collections import deque
from pathlib import Path
import hashlib
import json
import sys
import numpy as np
from PIL import Image, ImageFilter

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/floor-continuous-01/interior-r03'
DOC=ROOT/'docs/reviews/floor-continuous-01'
sys.path.insert(0,str(Path(__file__).parent))
from build_floor_continuous_review_01 import material


def whole_architecture(source):
    raw=np.array(source)
    outline=(raw[:,:,:3].max(axis=2)<45)|(raw[:,:,3]==0)
    # Loose, thin floor-joint strokes are not connected to whole architecture.
    # Remove them before closing outlines, so they cannot attach to a wall foot.
    w,h=source.size;visited=bytearray(outline.astype('uint8').tobytes())
    for i in range(len(visited)):
        if visited[i]!=1:continue
        visited[i]=2;queue=deque([i]);points=[];count=0
        while queue:
            p=queue.popleft();count+=1
            if count<=256:points.append(p)
            x=p%w
            for q in ((p-1 if x else -1),(p+1 if x+1<w else -1),p-w,p+w):
                if 0<=q<len(visited) and visited[q]==1:
                    visited[q]=2;queue.append(q)
        if count<=256:
            xs=[p%w for p in points];ys=[p//w for p in points]
            bw=max(xs)-min(xs)+1;bh=max(ys)-min(ys)+1
            if (bw<=8 and bh>=12) or (bh<=8 and bw>=12):
                outline.reshape(-1)[points]=False
    outline=np.array(Image.fromarray((outline*255).astype('uint8')).filter(ImageFilter.MaxFilter(3)))>0
    w,h=source.size;state=bytearray(outline.astype('uint8').tobytes());holes=[]
    for i in range(len(state)):
        if state[i]:continue
        state[i]=2;queue=deque([i]);component=[];count=0
        while queue:
            p=queue.popleft();count+=1
            if count<=25000:component.append(p)
            x=p%w
            for q in ((p-1 if x else -1),(p+1 if x+1<w else -1),p-w,p+w):
                if 0<=q<len(state) and state[q]==0:
                    state[q]=2;queue.append(q)
        if count<=25000:
            rgb=raw.reshape(-1,4)[component,:3].astype('int16')
            # Broad warm floor patches enclosed by old joints are not hardware.
            if count<=180 or np.mean(rgb[:,2]+2>=rgb[:,0])>.15:
                holes.append(count)
                for p in component:state[p]=1
    return np.frombuffer(state,dtype='uint8').reshape(h,w)==1,holes


def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    ART.mkdir(parents=True,exist_ok=True)
    floor_path=ROOT/'assets/maps/cryogenic-camp-01/floor.png'
    structure_path=ROOT/'assets/maps/cryogenic-camp-01/structure.png'
    floor=Image.open(floor_path).convert('RGBA');structure=Image.open(structure_path).convert('RGBA')
    source=floor.copy();source.alpha_composite(structure)
    protected,holes=whole_architecture(source)
    # Warm light housings do not share the blue steel pigment. Preserve their
    # complete source rectangles, including the bright lens and its front face.
    scene=json.loads((ROOT/'assets/maps/cryogenic-camp-01/camp.json').read_text())
    fixtures=[]
    raw=np.array(source)
    for lamp in scene['atmosphere']['lights']:
        x0,y0,x1,y1=lamp['box'];x0=max(0,x0-8);y0=max(0,y0-8);x1+=8;y1+=8
        patch=raw[y0:y1,x0:x1,:3]
        ys,xs=np.where((patch[:,:,0]>235)&(patch[:,:,1]>215)&(patch[:,:,2]>150))
        assert len(xs),'Every source lamp must have a readable bright core'
        a=x0+int(xs.min());b=y0+int(ys.min());c=x0+int(xs.max())+1;d=y0+int(ys.max())+1
        if c-a>d-b:
            if lamp['origin'][1]*64+32 < lamp['at'][1]:
                boxes=[(a-14,b-8,c+14,d+24)]
            else:
                boxes=[(a-24,b-23,c+24,b-8),(a-14,b-8,c+14,d+10)]
        else:
            boxes=[(a-23,b-24,a-8,d+24),(a-8,b-12,c+10,d+12)]
        for x0,y0,x1,y1 in boxes:
            box=(max(0,x0),max(0,y0),min(source.width,x1),min(source.height,y1))
            x0,y0,x1,y1=box;protected[y0:y1,x0:x1]=True;fixtures.append(box)
    # Logical layer boundaries cut through real ground and real wall feet.
    # Extend the working floor domain over the visible margins in structure.png.
    eligible=np.array(floor.getchannel('A').filter(ImageFilter.BoxBlur(32)))>0
    # A historical map join contains one painted floor joint, not hardware.
    protected[512:576,624:663]=False
    mask=eligible&~protected
    original=np.array(source)
    light=np.array(floor.convert('RGB').filter(ImageFilter.GaussianBlur(32))).astype('float32')
    cov=np.array(floor.getchannel('A').filter(ImageFilter.GaussianBlur(32))).astype('float32')/255
    light/=np.maximum(cov[...,None],.02)
    reference=np.array(Image.open(ROOT/'assets/source-art/floor-continuous-01/material-source.png').convert('RGB')).mean(axis=(0,1))
    tex=np.array(material(source.size)).astype('float32')
    replacement=np.clip(tex*light/np.maximum(reference,1),0,255).astype('uint8')
    final=original.copy();final[mask,:3]=replacement[mask]
    assert np.array_equal(final[~mask],original[~mask])
    assert np.array_equal(final[protected],original[protected])
    for x0,y0,x1,y1 in fixtures:
        assert np.array_equal(final[y0:y1,x0:x1],original[y0:y1,x0:x1])
    Image.fromarray((mask*255).astype('uint8')).save(ART/'floor-selection.png')
    outputs=[]
    for name,old in [('floor.png',floor),('structure.png',structure)]:
        data=np.array(old);opaque=data[:,:,3]>0;data[opaque,:3]=final[opaque,:3]
        target=ART/name;Image.fromarray(data).save(target)
        assert np.array_equal(data[:,:,3],np.array(old)[:,:,3])
        outputs.append({'candidate':target.relative_to(ROOT).as_posix(),'sha256':sha(target),'alpha_unchanged':True})
    repaired=Image.open(ART/'floor.png').convert('RGBA');repaired.alpha_composite(Image.open(ART/'structure.png'))
    assert np.array_equal(np.array(repaired),final)
    report={'review':'SOL-CONT01-r03','candidate_number':4,'status':'awaiting_visual_approval','material':'accepted continuous concrete only; grating 03 rejected','source_floor':{'path':floor_path.relative_to(ROOT).as_posix(),'sha256':sha(floor_path)},'source_structure':{'path':structure_path.relative_to(ROOT).as_posix(),'sha256':sha(structure_path)},'protected_source_pixels_changed':0,'outside_ground_mask_pixels_changed':0,'source_layer_alpha_unchanged':True,'light_housing_rectangles':fixtures,'light_housing_pixels_changed':0,'hardware_regions_preserved':len(holes),'largest_hardware_region':max(holes,default=0),'ground_pixels_in_floor_layer':int(np.count_nonzero(mask&(np.array(floor)[:,:,3]>0))),'ground_pixels_in_structure_layer':int(np.count_nonzero(mask&(np.array(structure)[:,:,3]>0))),'outputs':outputs,'no_gameplay_change':True}
    (DOC/'interior-r03-conditioning.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report))


if __name__=='__main__':main()
