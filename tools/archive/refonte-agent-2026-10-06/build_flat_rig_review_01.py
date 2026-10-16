"""Review-only compositor for generated frontal drawings on a shared rig.

New artwork is authored by imagegen. This script selects source arm/finger
layers, registers three measured joints to the common rig, and composes the
existing item illustrations. There are no per-outfit weapon transforms.
"""
from pathlib import Path
from functools import lru_cache
import json, math, hashlib, shutil
from PIL import Image, ImageChops, ImageDraw, ImageFont
import build_comic_catalogue_03 as old

OUT=old.BASE/'flat-rig-review-01'; F=16; MS=1024
S=29/1100
NAMES={1:'Bastion · armure ivoire',4:'Arcaniste · robe rouge'}

def native(p):return [32+(p[0]-625)*S,47+(p[1]-1190)*S]
def master(p):return [v*F for v in native(p)]
def save(im,p):p.parent.mkdir(parents=True,exist_ok=True);im.save(p)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def normalize(im):return old.affine(im,[625,1190],[32*F,47*F],S*F)
def source(c,pose):return Image.open(OUT/'sources'/f'BD-FLAT01-C{c:02}-{pose}.png').convert('RGBA')

# Shoulder below the fixed pauldron, elbow, hand centre. Coordinates were
# measured in the source paintings. Canonical points come from the Bastion.
JOINTS={
 'independent':{
   1:[[[425,435],[380,525],[210,575]],[[830,435],[880,525],[1050,575]]],
   4:[[[425,435],[375,518],[208,572]],[[830,435],[878,518],[1049,572]]]},
 'rifle':{
   1:[[[425,435],[385,531],[610,535]],[[830,435],[875,540],[1005,558]]],
   4:[[[425,435],[385,527],[605,535]],[[830,435],[875,540],[1003,558]]]},
 'heavy':{
   1:[[[425,435],[350,551],[472,610]],[[830,435],[903,551],[813,610]]],
   4:[[[425,435],[350,551],[472,610]],[[830,435],[903,551],[813,610]]]},
 'pole':{
   1:[[[425,435],[415,551],[610,598]],[[830,435],[887,549],[777,459]]],
   4:[[[425,435],[415,551],[610,598]],[[830,435],[887,549],[777,459]]]},
}
POLYS={
 'independent':[
  [[105,435],[482,435],[483,473],[463,519],[438,572],[449,684],[100,684]],
  [[770,435],[1150,435],[1160,684],[805,684],[813,572],[792,519],[770,473]]],
 'rifle':[
  [[335,435],[472,435],[480,472],[495,480],[660,463],[672,590],[585,630],[505,632],[335,630]],
  [[770,435],[889,435],[917,486],[1058,495],[1068,638],[836,639],[770,552]]],
 'heavy':[
  [[340,435],[477,435],[485,500],[531,537],[535,679],[426,687],[284,608],[283,526]],
  [[778,435],[919,435],[960,526],[966,608],[894,687],[763,679],[762,537],[779,500]]],
 'pole':[
  [[340,435],[474,435],[479,476],[565,502],[673,538],[686,645],[598,691],[431,667],[337,568]],
  [[760,403],[822,392],[866,414],[896,467],[956,493],[956,648],[837,649],[799,550],[718,525],[710,410]]],
}

def solve3(rows,vals):
    a=[list(r)+[v] for r,v in zip(rows,vals)]
    for col in range(3):
        pivot=max(range(col,3),key=lambda k:abs(a[k][col]));a[col],a[pivot]=a[pivot],a[col]
        divisor=a[col][col];assert abs(divisor)>1e-8
        a[col]=[v/divisor for v in a[col]]
        for r in range(3):
            if r!=col:
                t=a[r][col];a[r]=[a[r][j]-t*a[col][j] for j in range(4)]
    return [a[r][3] for r in range(3)]

def coefficients(c,pose,side):
    target=[master(p) for p in JOINTS[pose][1][side]]
    src=JOINTS[pose][c][side];rows=[[x,y,1] for x,y in target]
    return tuple(solve3(rows,[p[0] for p in src])+solve3(rows,[p[1] for p in src]))

def registered(im,c,pose,side):
    return im.transform((MS,MS),Image.Transform.AFFINE,coefficients(c,pose,side),resample=Image.Resampling.BICUBIC)

def mask(size,polys):
    m=Image.new('L',size);d=ImageDraw.Draw(m)
    for poly in polys:d.polygon(poly,fill=255)
    return m

def selected(im,polys):
    r=im.copy();r.putalpha(ImageChops.multiply(im.getchannel('A'),mask(im.size,polys)));return r

@lru_cache(maxsize=8)
def body(c,pose):
    reference=source(c,'independent');src=source(c,pose)
    oldmask=mask(reference.size,POLYS['independent'])
    core=reference.copy();core.putalpha(ImageChops.multiply(core.getchannel('A'),ImageChops.invert(oldmask)))
    # Recover only body pixels exposed by the removal of the original sleeves.
    newmask=mask(src.size,POLYS[pose])
    fill=src.copy();fill.putalpha(ImageChops.multiply(src.getchannel('A'),ImageChops.multiply(oldmask,ImageChops.invert(newmask))))
    core.alpha_composite(fill);result=normalize(core)
    for side in (0,1):result.alpha_composite(registered(selected(src,[POLYS[pose][side]]),c,pose,side))
    return result

def fingers(c,pose,side):
    x,y=JOINTS[pose][c][side][2]
    # A narrow piece of the actual rounded knuckles, never an opaque rectangle.
    if pose=='independent':
        lo,hi=(-42,1) if side==0 else (-4,40)
        poly=[[x+lo,y-50],[x+hi-5,y-52],[x+hi+2,y+40],[x+lo+5,y+48]]
    else:
        poly=[[x-19,y-42],[x+29,y-41],[x+35,y+28],[x+19,y+43],[x-25,y+28]]
    return registered(selected(source(c,pose),[poly]),c,pose,side)

CONFIGS=[
 ('00','Sans arme','independent',[]),
 ('01','Un pistolet','independent',[('BD-W102',1)]),
 ('02','Deux pistolets','independent',[('BD-W102',0),('BD-W102',1)]),
 ('03','Deux pistolets-mitrailleurs','independent',[('BD-W108',0),('BD-W108',1)]),
 ('04','Lame et pistolet','independent',[('BD-W069',0),('BD-W102',1)]),
 ('05','Pistolet et lame','independent',[('BD-W102',0),('BD-W069',1)]),
 ('06','Deux lames','independent',[('BD-W069',0),('BD-W069',1)]),
 ('07','Lame et bouclier','independent',[('BD-EQ-S02',0),('BD-W069',1)]),
 ('08','Lame et focaliseur','independent',[('BD-W069',0),('BD-EQ-F01',1)]),
 ('09','Fusil à deux mains','rifle',[('BD-W001',None)]),
 ('10','Mitrailleuse lourde','heavy',[('BD-EQ-GA01',None)]),
 ('11','Hampe à deux mains','pole',[('BD-EQ-H01',None)]),
 ('12','Lance à deux mains','pole',[('BD-W083',None)]),
 ('13','Épée à une main','independent',[('BD-W081',1)]),
 ('14','Deux épées','independent',[('BD-W081',0),('BD-W081',1)]),
 ('15','Épée et bouclier — mains inversées','independent',[('BD-W081',0),('BD-EQ-S02',1)]),
]

def item_layer(ident,pose,side):
    e=old.BY_ID[ident];im=old.raw(ident);obj=im.crop(old.bounds(im));w,h=obj.size
    meta={'review_id':ident,'pose':pose,'side':side}
    if side is not None:
        _,grip,s,_,angle=old.gear_spec(e);angle=0
        if side==0 and e['kind']!='shield':
            obj=obj.transpose(Image.Transpose.FLIP_LEFT_RIGHT);grip=[w-grip[0],grip[1]]
        target=master(JOINTS['independent'][1][side][2])
    elif pose in ('rifle','heavy'):
        if pose=='rifle':u,v=old.GUN_GRIPS[1];support=(.735,.49)
        else:u,v=.20,.68;support=(.69,.73)
        grip=[u*w,v*h];second=[support[0]*w,support[1]*h]
        target=master(JOINTS[pose][1][0][2]);front=master(JOINTS[pose][1][1][2])
        delta=[front[j]-target[j] for j in (0,1)];offset=[second[j]-grip[j] for j in (0,1)]
        s=math.hypot(*delta)/(F*math.hypot(*offset))
        angle=math.atan2(delta[1],delta[0])-math.atan2(offset[1],offset[0])
        meta['support_native']=[v/F for v in front]
    else:
        if e['family']=='shaft':grip=[.5*w,.77*h];s=29/h;sourceangle=-math.pi/2
        else:
            u,v=old.MELEE_GRIPS[int(ident[4:])];grip=[u*w,v*h];s=26/math.hypot(w,h);sourceangle=-math.pi/4
        target=master(JOINTS['pole'][1][0][2]);front=master(JOINTS['pole'][1][1][2])
        angle=math.atan2(front[1]-target[1],front[0]-target[0])-sourceangle
        meta['support_native']=[v/F for v in front]
    layer=old.affine(obj,grip,target,s*F,angle)
    meta.update(grip_native=[v/F for v in target],scale_native=s,angle_degrees=math.degrees(angle),weapon_layer_hash=hashlib.sha256(layer.tobytes()).hexdigest())
    return layer,meta

def outputs(master_im,key):
    native_im=master_im.resize((64,64),Image.Resampling.LANCZOS)
    save(native_im,OUT/'native32'/f'{key}.png')
    save(native_im.resize((128,128),Image.Resampling.NEAREST),OUT/'display64'/f'{key}.png')
    save(master_im.resize((512,512),Image.Resampling.LANCZOS),OUT/'normal'/f'{key}.png')
    # A shared illustration viewport removes empty margins equally for every
    # outfit/configuration. Native sprites keep their full overflow canvas.
    save(master_im.crop((8*F,8*F,58*F,50*F)).resize((600,504),Image.Resampling.LANCZOS),OUT/'portraits'/f'{key}.png')
    return native_im

def build():
    active=old.ROOT/'assets/sprites/cyberpunk/agent-rigs/manifest.json'
    baseline=json.loads(active.read_text('utf-8'));protected={p['path']:sha(old.ROOT/p['path']) for p in baseline['files']}
    rows=[]
    for number,label,pose,items in CONFIGS:
        layers=[item_layer(ident,pose,side) for ident,side in items]
        pair=[]
        for c in (1,4):
            result=body(c,pose).copy()
            # Same item layers are reused byte-for-byte across both outfits.
            for (ident,side),(layer,_) in zip(items,layers):result.alpha_composite(layer)
            for side in (0,1):
                shown=any(s==side or s is None for _,s in items)
                shield=any(s==side and old.BY_ID[i]['kind']=='shield' for i,s in items)
                if shown and not shield:result.alpha_composite(fingers(c,pose,side))
            key=f'BD-FLAT01-{number}-C{c:02}';outputs(result,key)
            pair.append({'outfit':c,'name':NAMES[c],'native32':f'native32/{key}.png','display64':f'display64/{key}.png','normal':f'normal/{key}.png','portrait':f'portraits/{key}.png','weapon_transforms':[m for _,m in layers]})
        rows.append({'id':f'BD-FLAT01-{number}','number':number,'label':label,'pose':pose,'items':[i for i,_ in items],'views':pair})
    # Individual sidearm coverage also uses the independent-hands common pose.
    sidearm=[]
    for n in range(102,114):
        ident=f'BD-W{n:03}';layer,meta=item_layer(ident,'independent',1)
        for c in (1,4):
            im=body(c,'independent').copy();im.alpha_composite(layer);im.alpha_composite(fingers(c,'independent',1))
            outputs(im,f'BD-FLAT01-{ident}-C{c:02}')
        sidearm.append(meta)
    data={'review_id':'BD-FLAT01','status':'candidates_pending_visual_review','body_height_native':29,'logical_tile':32,'display_tile':64,'outfits':NAMES,'configurations':rows,'sidearm_models':sidearm,'common_rig':{p:[[native(q) for q in side] for side in values[1]] for p,values in JOINTS.items()},'source_registration':JOINTS,'arm_masks':POLYS,'method':'source-authored imagegen drawings; measured three-joint arm registration; common item layers; fixed outfit core','protected_files':protected,'limits':['Two outfits only; other outfits await this prototype review.','This prototype does not redesign items, add robes or redefine Metamorphe appearance.','No new art is integrated into the active game.']}
    (OUT/'review.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),'utf-8')
    (OUT/'review-data.js').write_text('window.flatReview='+json.dumps(data,ensure_ascii=False)+';','utf-8')
    shutil.copy2(old.BASE/'comic64-reference-review-04/images/floor32.png',OUT/'floor32.png')
    validate(data)
    boards(data)
    print(f'BD-FLAT01 ready: {len(rows)*2} configuration views + {len(sidearm)*2} individual sidearm views')

def validate(data):
    assert all(sha(old.ROOT/p)==h for p,h in data['protected_files'].items())
    keys=[f"BD-FLAT01-{r['number']}-C{c:02}" for r in data['configurations'] for c in (1,4)]
    keys += [f'BD-FLAT01-BD-W{n:03}-C{c:02}' for n in range(102,114) for c in (1,4)]
    errors=[]
    for key in keys:
        im=Image.open(OUT/'native32'/f'{key}.png').convert('RGBA');big=Image.open(OUT/'display64'/f'{key}.png').convert('RGBA')
        assert big.tobytes()==im.resize((128,128),Image.Resampling.NEAREST).tobytes()
        box=old.bounds(im)
        if box[0]<=0 or box[1]<=0 or box[2]>=64 or box[3]>=64:errors.append(key)
    assert not errors,errors
    for row in data['configurations']:assert row['views'][0]['weapon_transforms']==row['views'][1]['weapon_transforms']
    for c in (1,4):
        ref=Image.open(OUT/'native32'/f'BD-FLAT01-00-C{c:02}.png')
        for pose in ('rifle','heavy','pole'):
            test=body(c,pose).resize((64,64),Image.Resampling.LANCZOS)
            base=body(c,'independent').resize((64,64),Image.Resampling.LANCZOS)
            # Compare the complete high-resolution head region, plus the
            # visible native head. Nearby shoulder pixels may receive a tiny
            # Lanczos contribution from a changed arm.
            assert body(c,pose).crop((0,0,MS,24*F)).tobytes()==body(c,'independent').crop((0,0,MS,24*F)).tobytes()
            assert test.crop((27,0,37,24)).tobytes()==base.crop((27,0,37,24)).tobytes(),(c,pose,'head')
            assert test.crop((0,38,64,64)).tobytes()==base.crop((0,38,64,64)).tobytes(),(c,pose,'legs')
    report={'review_id':'BD-FLAT01','composition_count':len(keys),'exact_32_to_64_doubling':len(keys),'weapon_transforms_identical_between_outfits':len(data['configurations']),'individual_sidearm_models':12,'same_rig_for_both_outfits':True,'heads_and_lower_bodies_fixed_across_poses':True,'protected_active_files_unchanged':len(data['protected_files']),'clipped_compositions':errors,'visual_acceptance_pending':True}
    (OUT/'validation.json').write_text(json.dumps(report,indent=2),'utf-8')

def boards(data):
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)
    for start in range(0,len(data['configurations']),4):
        subset=data['configurations'][start:start+4];board=Image.new('RGB',(1240,490*len(subset)),(14,21,29));d=ImageDraw.Draw(board)
        for j,row in enumerate(subset):
            y=j*490;d.text((20,y+8),f"{row['id']} · {row['label']}",fill='white',font=font)
            for k,v in enumerate(row['views']):
                x=20+k*600;d.text((x,y+36),v['name'],fill='#abc1cc',font=font)
                im=Image.open(OUT/v['normal']).convert('RGBA').resize((400,400),Image.Resampling.LANCZOS);board.paste(im,(x,y+64),im)
                for size,col,path in [(64,410,v['native32']),(128,480,v['display64'])]:
                    d.rectangle((x+col-8,y+310,x+col+size+4,y+314+size),fill='#aab6ba')
                    small=Image.open(OUT/path);board.paste(small,(x+col-4,y+312),small)
        save(board,OUT/'boards'/f'poses-{start//4+1:02}.png')

if __name__=='__main__':build()
