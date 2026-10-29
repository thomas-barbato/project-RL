"""Pack generated deck material into explicit, source-outline-traced ground contours.

No pixel classification by decorative tile or logical-layer alpha is used.
Both original architecture layers are treated as one drawing before re-packing.
"""
from pathlib import Path
import hashlib
import json
import numpy as np
from PIL import Image,ImageDraw,ImageFilter

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/floor-future-01'
DOC=ROOT/'docs/reviews/floor-future-01'
OUT=ROOT/'artifacts/floor-continuous-review-01/future-r04'


def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()


def region(raw,box,top_probe,top_limit,bottom_probe,bottom_limit,left_window,right_window,open_edges=()):
    """Trace the first inked structural contour in authored search windows.

    Windows are in visual source pixels, never cells. Top/bottom rays account
    for vents and other complete hanging architecture instead of clipping them.
    """
    x0,y0,x1,y1=box;ink=raw[:,:,:3].max(axis=2)<14
    top=[];bottom=[];left=[];right=[]
    for x in range(x0,x1):
        hits=[y for y in range(top_probe,top_limit-1,-1) if ink[y,x]]
        top.append(max(y0,hits[0]+1) if hits else y0)
        hits=[y for y in range(bottom_probe,bottom_limit+1) if ink[y,x]]
        bottom.append(min(y1,hits[0]) if hits else y1)
    for y in range(y0,y1):
        hits=[x for x in range(left_window[0],left_window[1]-1,-1) if ink[y,x]]
        left.append(max(x0,hits[0]+1) if hits else x0)
        hits=[x for x in range(right_window[0],right_window[1]+1) if ink[y,x]]
        right.append(min(x1,hits[0]) if hits else x1)
    if 'left' in open_edges:left=[x0]*(y1-y0)
    if 'right' in open_edges:right=[x1]*(y1-y0)
    yy,xx=np.mgrid[y0:y1,x0:x1]
    selected=(yy>=np.array(top)[None,:])&(yy<np.array(bottom)[None,:])&(xx>=np.array(left)[:,None])&(xx<np.array(right)[:,None])
    mask=np.zeros(raw.shape[:2],dtype=bool);mask[y0:y1,x0:x1]=selected
    return mask,{'box':box,'top':top,'bottom':bottom,'left':left,'right':right}


def main():
    if (DOC/'acceptance-05.json').exists():
        raise SystemExit('05 is frozen after approval. Use build_corridor_grate_review_01.py for the current corridor-only proposal.')
    ART.mkdir(parents=True,exist_ok=True);DOC.mkdir(parents=True,exist_ok=True);OUT.mkdir(parents=True,exist_ok=True)
    floor_path=ROOT/'assets/maps/cryogenic-camp-01/floor.png';structure_path=ROOT/'assets/maps/cryogenic-camp-01/structure.png'
    floor=Image.open(floor_path).convert('RGBA');structure=Image.open(structure_path).convert('RGBA')
    source=floor.copy();source.alpha_composite(structure);raw=np.array(source)
    mask=np.zeros(raw.shape[:2],dtype=bool);traces=[];room_masks=[]
    # Real room extents include the lower margins formerly stranded in structure.png.
    specs=[('cryo',(42,295,449,753),350,276,680,780,(57,20),(440,480),[]),
        ('storage',(910,126,1334,327),174,105,250,350,(926,880),(1320,1360),[]),
        ('security',(1742,126,2166,327),174,105,250,350,(1758,1712),(2152,2192),[]),
        ('medical',(910,740,1334,961),800,720,900,985,(926,880),(1320,1360),[]),
        ('workshop',(1742,740,2166,961),800,720,900,985,(1758,1712),(2152,2192),[])]
    for name,box,tp,tl,bp,bl,lw,rw,opens in specs:
        part,trace=region(raw,box,tp,tl,bp,bl,lw,rw,opens)
        mask|=part;room_masks.append((name,box,part));traces.append({'name':name,**trace})
    hall,trace=region(raw,(704,410,2368,644),544,400,570,680,(734,680),(2340,2400),['left','right'])
    mask|=hall;traces.append({'name':'hall',**trace})
    # Authored open thresholds use their visible inked jambs, not the cell split.
    connectors=[(444,486,718,584),(2358,486,2432,584)]
    connector,trace=region(raw,connectors[0],548,480,548,592,(450,430),(674,712),['left','right'])
    mask|=connector;traces.append({'name':'cryo-open-threshold',**trace})
    mask[490:580,624:663]=True # Clear original painted joint; no hardware here.
    connector,trace=region(raw,connectors[1],548,480,548,592,(2360,2340),(2420,2431),['left','right'])
    mask|=connector;traces.append({'name':'outer-exit-threshold',**trace})
    # Trace the actual corridor jambs row by row. A protected cell-width strip
    # would retain exactly the old paving fragments this review removes.
    for x0,y0,x1,y1 in [(1080,316,1162,452),(1912,316,1994,452),(1080,638,1162,778),(1912,638,1994,778)]:
        centre=(x0+x1)//2;rows=[]
        for y in range(y0,y1):
            hits=[x for x in range(centre,x0-33,-1) if raw[y,x,:3].max()<14]
            left=max(x0,hits[0]+1) if hits else x0
            hits=[x for x in range(centre,x1+33) if raw[y,x,:3].max()<14]
            right=min(x1,hits[0]) if hits else x1
            if left<right:mask[y,left:right]=True
            rows.append([left,right])
        traces.append({'name':'vertical-threshold','box':[x0,y0,x1,y1],'row_bounds':rows})
    # Protect complete real light housings. Light metadata boxes can contain
    # ground, so use bright cores and the observed source housing margins.
    protect=[(714,488,736,589),(2339,488,2359,589)]
    scene=json.loads((ROOT/'assets/maps/cryogenic-camp-01/camp.json').read_text())
    for lamp in scene['atmosphere']['lights']:
        x0,y0,x1,y1=lamp['box'];x0=max(0,x0-8);y0=max(0,y0-8);x1+=8;y1+=8
        p=raw[y0:y1,x0:x1,:3];ys,xs=np.where((p[:,:,0]>235)&(p[:,:,1]>215)&(p[:,:,2]>150))
        assert len(xs)
        a=x0+int(xs.min());b=y0+int(ys.min());c=x0+int(xs.max())+1;d=y0+int(ys.max())+1
        if c-a>d-b:
            if lamp['origin'][1]*64+32<lamp['at'][1]:boxes=[(a-12,b-8,c+13,d+12)]
            else:boxes=[(a-24,b-23,c+24,b-8),(a-12,b-8,c+12,d+9)]
        else:boxes=[(a-23,b-24,a-8,d+24),(a-8,b-12,c+10,d+12)]
        for bx0,by0,bx1,by1 in boxes:protect.append((max(0,bx0),max(0,by0),min(source.width,bx1),min(source.height,by1)))
    for x0,y0,x1,y1 in protect:mask[y0:y1,x0:x1]=False
    # Independent ground probes cover the formerly missed strips at room edges.
    probes=[('cryo-lower-edge',(116,738,180,748)),('storage-lower-edge',(960,317,1024,324)),('security-lower-edge',(1792,317,1856,324)),('medical-lower-edge',(960,950,1024,958)),('workshop-lower-edge',(1792,950,1856,958))]
    for name,(x0,y0,x1,y1) in probes:
        assert mask[y0:y1,x0:x1].all(),f'Old ground left in critical edge strip: {name}'
    # New material comes only from imagegen; packing never invents floor artwork.
    deck=Image.open(ART/'deck-source.png').convert('RGB')
    patch=deck.resize((512,512),Image.Resampling.LANCZOS)
    atlas=Image.new('RGB',(1024,1024));atlas.paste(patch,(0,0));atlas.paste(patch.transpose(Image.Transpose.FLIP_LEFT_RIGHT),(512,0));atlas.paste(patch.transpose(Image.Transpose.FLIP_TOP_BOTTOM),(0,512));atlas.paste(patch.transpose(Image.Transpose.ROTATE_180),(512,512))
    texture=Image.new('RGB',source.size)
    for y in range(0,source.height,1024):
        for x in range(0,source.width,1024):texture.paste(atlas,(x,y))
    for name,(x0,y0,x1,y1),part in room_masks:
        # Each room gets one broad design, with no per-cell restart or border strip.
        design=deck.transpose(Image.Transpose.ROTATE_180) if name in ('security','workshop') else deck
        size=max(x1-x0,y1-y0);native=design.resize((size,size),Image.Resampling.LANCZOS)
        cx=(size-(x1-x0))//2;cy=(size-(y1-y0))//2
        native=native.crop((cx,cy,cx+x1-x0,cy+y1-y0))
        image=Image.new('RGB',source.size);image.paste(native,(x0,y0))
        texture=Image.composite(image,texture,Image.fromarray((part*255).astype('uint8')))
    # Preserve only broad baked exposure, not the old tile joints or tile pigments.
    light=np.array(floor.convert('RGB').filter(ImageFilter.GaussianBlur(32))).astype('float32')
    cov=np.array(floor.getchannel('A').filter(ImageFilter.GaussianBlur(32))).astype('float32')/255
    light/=np.maximum(cov[...,None],.02)
    # Retain the same broad source exposure used by the accepted lighting shader.
    reference=np.array(Image.open(ROOT/'assets/source-art/floor-continuous-01/material-source.png').convert('RGB')).mean(axis=(0,1))
    replacement=np.clip(np.array(texture).astype('float32')*light/np.maximum(reference,1),0,255).astype('uint8')
    final=raw.copy();final[mask,:3]=replacement[mask]
    assert np.array_equal(final[~mask],raw[~mask])
    for x0,y0,x1,y1 in protect:assert np.array_equal(final[y0:y1,x0:x1],raw[y0:y1,x0:x1])
    outputs=[]
    for name,old in [('floor.png',floor),('structure.png',structure)]:
        data=np.array(old);a=data[:,:,3]>0;data[a,:3]=final[a,:3]
        path=ART/name;Image.fromarray(data).save(path);outputs.append({'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)})
        assert np.array_equal(data[:,:,3],np.array(old)[:,:,3])
    Image.fromarray((mask*255).astype('uint8')).save(ART/'ground-mask.png')
    Image.fromarray(final).save(OUT/'architecture-candidate.png')
    overlay=raw.copy();overlay[mask,:3]=(raw[mask,:3].astype('uint16')+np.array([0,180,200]))//2
    Image.fromarray(overlay).save(OUT/'ground-contour-check.png')
    (DOC/'ground-contours.json').write_text(json.dumps({'review':'SOL-FUT01-r01','candidate_number':5,'source_outline_threshold':14,'traces':traces,'open_thresholds':connectors,'whole_source_hardware_bounds':protect,'critical_edge_probes':probes,'critical_edge_probes_all_replaced':True},indent=2)+'\n')
    (DOC/'conditioning.json').write_text(json.dumps({'review':'SOL-FUT01-r01','candidate_number':5,'status':'awaiting_visual_approval','source_floor_sha256':sha(floor_path),'source_structure_sha256':sha(structure_path),'deck_source_sha256':sha(ART/'deck-source.png'),'outputs':outputs,'outside_ground_mask_pixels_changed':0,'protected_hardware_pixels_changed':0,'source_alphas_unchanged':True,'ground_pixels':int(mask.sum()),'ground_in_structure_layer_pixels':int(np.count_nonzero(mask&(np.array(structure)[:,:,3]>0))),'native_cell':64,'gameplay_changed':False},indent=2)+'\n')
    print('05 packed across the five whole room floors and gallery; outside-mask pixels unchanged.')


if __name__=='__main__':main()
