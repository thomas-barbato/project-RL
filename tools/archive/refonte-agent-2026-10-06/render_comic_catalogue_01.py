"""Condition independent comic sources and place them on fixed reviewed rigs.

Review artifacts only. Operations are sampling, palette conditioning and layer
composition; all new drawings come from imagegen. No runtime files are written.
"""
from pathlib import Path
import argparse,base64,hashlib,json,math
from io import BytesIO
from functools import lru_cache
from PIL import Image,ImageChops,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2];BASE=Path(__file__).resolve().parent
OUT=BASE/'comic-catalogue-review-01';ART=ROOT/'assets/sprites/cyberpunk'
BG=(15,23,32);FG=(235,241,245);SUB=(159,181,195);SIZE=128;SHIFT=32
RIG=json.loads((ART/'agent-rigs/manifest.json').read_text(encoding='utf-8'))
CAT=json.loads((ART/'manifest.json').read_text(encoding='utf-8'))
BODY_IDS={0:'BD-B001',1:'BD-B002',2:'BD-B003',**{i:f'BD-A{i-2:03}' for i in range(3,17)}}

def font(n):return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',n)
@lru_cache(maxsize=16)
def atlas(path):return Image.open(path).convert('RGBA')
@lru_cache(maxsize=512)
def cell(path,index,side):
    im=atlas(path);x=index%16*side;y=index//16*side
    return im.crop((x,y,x+side,y+side))
def save(path,im):path.parent.mkdir(parents=True,exist_ok=True);im.save(path)
def bbox(im,threshold=64):return im.getchannel('A').point(lambda a:255 if a>=threshold else 0).getbbox()
def crop(im):return im.crop(bbox(im))
def indexed(im,n=24):
    a=im.getchannel('A').point(lambda v:255 if v>=128 else 0)
    rgb=im.convert('RGB').quantize(n,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE).convert('RGBA');rgb.putalpha(a)
    return rgb
def url(im):
    b=BytesIO();im.save(b,'PNG');return 'data:image/png;base64,'+base64.b64encode(b.getvalue()).decode()
@lru_cache(maxsize=17)
def reference_body(index):
    if index in RIG['body_bindings']:return cell(ART/'agent-rigs/bodies-32.png',RIG['body_bindings'].index(index),32)
    return cell(ART/'crawl-bodies-32.png',index,32)
def body_layers(index):
    original=reference_body(index);ident=BODY_IDS[index];p=OUT/'sources'/f'{ident}.png'
    if not p.exists():return original,original.resize((256,256),Image.Resampling.NEAREST),False
    source=crop(Image.open(p).convert('RGBA'));b=bbox(original,128);w,h=b[2]-b[0],b[3]-b[1]
    body=Image.new('RGBA',(32,32));sample=source.resize((w,h),Image.Resampling.LANCZOS)
    # A source-authored geometry stencil preserves the reviewed silhouette and
    # hand/elbow pixels. It is never inferred from weapon bounds.
    sample.putalpha(original.crop(b).getchannel('A'));body.paste(sample,(b[0],b[1]))
    pixels=[]
    for old,new in zip(original.get_flattened_data(),body.get_flattened_data()):
        pixels.append(old if old[3] and max(old[:3])<42 else new)
    body.putdata(pixels);body=indexed(body,24)
    master=Image.new('RGBA',(256,256));master.alpha_composite(source.resize((w*8,h*8),Image.Resampling.LANCZOS),(b[0]*8,b[1]*8))
    save(OUT/'bodies'/f'{ident}-native32.png',body);save(OUT/'bodies'/f'{ident}-master256.png',master)
    assert body.getchannel('A').tobytes()==original.getchannel('A').tobytes()
    return body,master,True
def profile_for(index):
    if index in RIG['body_bindings']:return RIG['profiles'][RIG['body_bindings'].index(index)]
    return {'number':1,'gloves':{'L':[6,19],'R':[25,19]},'elbows':{'L':[8,16],'R':[22,16]},'forearm_masks':[[3,17,10,23],[22,17,30,23]]}
@lru_cache(maxsize=128)
def own_sleeve(index,layer_index):
    # The reviewed arm geometry is reusable; its sleeve color belongs to the
    # outfit, independently of the weapon. Recolor only authored material 2.
    body=reference_body(index);arms=cell(ART/'agent-rigs/arms-64.png',layer_index,64)
    if index in RIG['body_bindings']:
        j=RIG['body_bindings'].index(index);mask=cell(ART/'agent-rigs/body-materials-32.png',j,32)
    else:mask=cell(ART/'crawl-body-materials-32.png',index,32)
    elbow=profile_for(index)['elbows']['L'];label=mask.getpixel(tuple(elbow))[0]
    if label not in (1,2):label=2 if index==2 else 1
    colors=sorted({body.getpixel((x,y))[:3] for y in range(32) for x in range(32) if body.getpixel((x,y))[3] and mask.getpixel((x,y))[0]==label},key=sum)
    if not colors:return arms
    ramp=[colors[0],colors[len(colors)//2],colors[-1]];material=cell(ART/'agent-rigs/arm-materials-64.png',layer_index,64)
    result=arms.copy()
    for y in range(64):
        for x in range(64):
            pixel=arms.getpixel((x,y))
            if pixel[3] and material.getpixel((x,y))[0]==2:
                value=max(pixel[:3])/255
                low,high,t=(ramp[0],ramp[1],min(1,max(0,(value-.12)/.41))) if value<.53 else (ramp[1],ramp[2],min(1,max(0,(value-.53)/.47)))
                result.putpixel((x,y),tuple(round(low[k]*(1-t)+high[k]*t) for k in range(3))+(pixel[3],))
    assert result.getchannel('A').tobytes()==arms.getchannel('A').tobytes()
    return result
def snap(im,point):
    # Use the centre of the visible graspable material in a narrow source row.
    x,y=point;opaque=[(xx,yy) for yy in range(max(0,round(y)-2),min(im.height,round(y)+3)) for xx in range(im.width) if im.getpixel((xx,yy))[3]>=128]
    if not opaque:opaque=[(xx,yy) for yy in range(im.height) for xx in range(im.width) if im.getpixel((xx,yy))[3]>=128]
    return min(opaque,key=lambda p:(p[0]-x)**2+(p[1]-y)**2)
def object_image(e):
    raw=Image.open(ROOT/e['source_path']).convert('RGBA');subject=crop(raw)
    s=min(330/subject.width,330/subject.height);display=subject.resize((max(1,round(subject.width*s)),max(1,round(subject.height*s))),Image.Resampling.LANCZOS)
    n=min(28/subject.width,28/subject.height);small=indexed(subject.resize((max(1,round(subject.width*n)),max(1,round(subject.height*n))),Image.Resampling.LANCZOS),16)
    icon=Image.new('RGBA',(32,32));icon.alpha_composite(small,((32-small.width)//2,(32-small.height)//2))
    save(OUT/'icons'/f"{e['review_id']}-native32.png",icon);save(OUT/'masters'/f"{e['review_id']}.png",display)
    return subject,display,icon
def affine_object(im,source_grips,target_grips,scale_single=None,angle_single=0,hires=False):
    p=source_grips[0];q=target_grips[0]
    if len(source_grips)==2:
        dx,dy=source_grips[1][0]-p[0],source_grips[1][1]-p[1]
        ex,ey=target_grips[1][0]-q[0],target_grips[1][1]-q[1]
        scale=math.hypot(ex,ey)/math.hypot(dx,dy);angle=math.atan2(ey,ex)-math.atan2(dy,dx)
    else:scale=scale_single;angle=angle_single
    factor=4 if hires else 1;c=math.cos(angle);s=math.sin(angle)
    a,b=c/scale,s/scale;d,f=-s/scale,c/scale
    tx=p[0]-a*q[0]-b*q[1];ty=p[1]-d*q[0]-f*q[1]
    coeff=(a/factor,b/factor,tx,d/factor,f/factor,ty)
    result=im.transform((SIZE*factor,SIZE*factor),Image.Transform.AFFINE,coeff,resample=Image.Resampling.BICUBIC if hires else Image.Resampling.NEAREST)
    return result,{'source_grips':[list(v) for v in source_grips],'target_grips':[list(v) for v in target_grips],
        'scale':scale,'angle_deg':round(math.degrees(angle),2)}
def gear(e,subject,index):
    p=profile_for(index);family=e['family'];w,h=subject.size;main=snap(subject,(w*.5,h*.88));targets=[]
    native=Image.new('RGBA',(SIZE,SIZE));normal=Image.new('RGBA',(SIZE*4,SIZE*4));arms=Image.new('RGBA',(SIZE,SIZE));fingers=arms.copy();meta={}
    if e['kind']=='shield':
        point=[48+p['gloves']['L'][0],48+p['gloves']['L'][1]];scale=22/h
        native,meta=affine_object(subject,[(w/2,h/2)],[point],scale_single=scale)
        normal,_=affine_object(subject,[(w/2,h/2)],[point],scale_single=scale,hires=True)
        meta['shield_center_on_hand']=True;meta['hands_in_front_of_shield']=False
    elif e['hold']!='TwoHanded':
        target=[48+p['gloves']['R'][0],48+p['gloves']['R'][1]]
        angle=0;scale=(22 if family in (0,1,4,16) else 36 if family in (5,'focalizer') else 16)/h
        if family in (6,7):
            main=snap(subject,(w*.24,h*.78));scale=(11 if family==6 else 14)/w
            elbow=p['elbows']['R'];hand=p['gloves']['R'];angle=math.atan2(hand[1]-elbow[1],hand[0]-elbow[0])
        elif family=='focalizer':
            scale=18/h;elbow=p['elbows']['R'];hand=p['gloves']['R'];angle=math.atan2(hand[1]-elbow[1],hand[0]-elbow[0])+math.pi/2
        native,meta=affine_object(subject,[main],[target],scale_single=scale,angle_single=angle)
        normal,_=affine_object(subject,[main],[target],scale_single=scale,angle_single=angle,hires=True)
        meta['new_forearms']=0
    else:
        chosen=2 if family=='shaft' else 10 if family=='heavy_machinegun' else 11 if family=='plasma' else family
        grips=CAT['weapon_families'][chosen]['hands'];targets=[[40+v[0],40+v[1]] for v in grips]
        if chosen in (2,3):
            src=[snap(subject,(w*.5,h*.91)),snap(subject,(w*.5,h*.77))]
        elif chosen in (8,9,11,15,17) and family not in ('heavy_machinegun','plasma'):
            # Ordinary rifles use their rear grip and a farther forward
            # support surface. The hands stay at the reviewed positions; a
            # wider source grip separation avoids oversized ordinary guns.
            src=[snap(subject,(w*.24,h*.79)),snap(subject,(w*.84,h*.48))]
        else:
            src=[snap(subject,(w*.30,h*.79)),snap(subject,(w*.62,h*.36))]
        native,meta=affine_object(subject,src,targets)
        normal,_=affine_object(subject,src,targets,hires=True)
        profile=p['number']-1;layer_index=profile*25+chosen
        arms.alpha_composite(own_sleeve(index,layer_index),(SHIFT,SHIFT))
        fingers.alpha_composite(cell(ART/'agent-rigs/fingers-64.png',layer_index,64),(SHIFT,SHIFT))
        meta['approved_pose_family']=chosen;meta['separate_arms_and_fingers']=True
        meta['ordinary_firearm_compact']=chosen in (8,9,11,15,17) and family not in ('heavy_machinegun','plasma')
        meta['finger_weapon_contacts']=[]
        for gx,gy in targets:
            contact=sum(native.getpixel((x,y))[3]>=128 and fingers.getpixel((x,y))[3]>=128
                for y in range(gy-2,gy+3) for x in range(gx-2,gx+3))
            assert contact>=1,(e['review_id'],index,'fingers',contact)
            meta['finger_weapon_contacts'].append(contact)
    native=indexed(native,16)
    for point in meta['target_grips']:
        xx,yy=map(round,point)
        contacts=sum(native.getpixel((x,y))[3]>0 for y in range(yy-2,yy+3) for x in range(xx-2,xx+3))
        assert contacts>=2,(e['review_id'],index,point,contacts)
    assert all(native.getpixel((x,y))[3]==0 for x in range(SIZE) for y in (0,SIZE-1)),e['review_id']
    assert all(native.getpixel((x,y))[3]==0 for y in range(SIZE) for x in (0,SIZE-1)),e['review_id']
    return native,normal,arms,fingers,meta
def compose(e,subject,index,bodies):
    body,master,ready=bodies[index];p=profile_for(index)
    native=Image.new('RGBA',(SIZE,SIZE));normal=Image.new('RGBA',(SIZE*4,SIZE*4))
    gear_native,gear_master,arms,fingers,meta=gear(e,subject,index)
    fixed=body.copy();fixed_master=master.copy()
    if e['hold']=='TwoHanded':
        for b in p['forearm_masks']:
            fixed.paste((0,0,0,0),tuple(b));fixed_master.paste((0,0,0,0),tuple(v*8 for v in b))
        native.alpha_composite(fixed,(48,48));native.alpha_composite(arms);native.alpha_composite(gear_native);native.alpha_composite(fingers)
        normal.alpha_composite(fixed_master.resize((128,128),Image.Resampling.LANCZOS),(192,192))
        normal_arms,normal_fingers=normal_pose_from_body(master,p,meta['target_grips'],arms,fingers)
        normal.alpha_composite(normal_arms);normal.alpha_composite(gear_master);normal.alpha_composite(normal_fingers)
        meta['normal_pose_source']='generated_outfit_forearms_clipped_to_authored_arm_geometry;authored_front_gloves'
    else:
        # One-hand weapons behind authored gloves; shields in FRONT of the hand.
        if e['kind']!='shield':native.alpha_composite(gear_native);normal.alpha_composite(gear_master)
        native.alpha_composite(fixed,(48,48));normal.alpha_composite(fixed_master.resize((128,128),Image.Resampling.LANCZOS),(192,192))
        if e['kind']=='shield':native.alpha_composite(gear_native);normal.alpha_composite(gear_master)
    # Uniformly enlarge the complete rig around the foot anchor. Body scale is
    # computed only from the fixed BODY, so oversized equipment cannot shrink it.
    bb=bbox(body,128);scale=32/(bb[3]-bb[1]);anchor=(64,48+bb[3]-1)
    coefficients=(1/scale,0,anchor[0]*(1-1/scale),0,1/scale,anchor[1]*(1-1/scale))
    native=native.transform((SIZE,SIZE),Image.Transform.AFFINE,coefficients,resample=Image.Resampling.NEAREST)
    normal=normal.transform((512,512),Image.Transform.AFFINE,coefficients[:2]+(coefficients[2]*4,)+coefficients[3:5]+(coefficients[5]*4,),resample=Image.Resampling.BICUBIC)
    meta.update(body_id=BODY_IDS[index],body_native_height=32,body_scale_from_body_only=scale,body_source_ready=ready,
        layering='fixed_body+authored_pose_arms+independent_weapon+fingers;shield_front',logical_tile=32,composition_canvas=SIZE)
    return native,normal,meta

def normal_pose_from_body(master,profile,grips,native_arms,native_fingers):
    arms=Image.new('RGBA',(512,512));fingers=arms.copy()
    for side,box,target in zip(('L','R'),profile['forearm_masks'],grips):
        source=Image.new('RGBA',master.size);b=(box[0]*8,max(0,box[1]-1)*8,box[2]*8,box[3]*8)
        source.paste(master.crop(b),(b[0],b[1]))
        elbow=profile['elbows'][side];glove=profile['gloves'][side]
        start=[v*8 for v in elbow];end=[v*8 for v in glove]
        ts=[(48+v)*4 for v in elbow];te=[v*4 for v in target]
        sx,sy=end[0]-start[0],end[1]-start[1];tx,ty=te[0]-ts[0],te[1]-ts[1]
        ls,lt=math.hypot(sx,sy),math.hypot(tx,ty);ux,uy=sx/ls,sy/ls;vx,vy=tx/lt,ty/lt
        k=ls/lt;perp=2
        a,b=k*ux*vx+perp*uy*vy,k*ux*vy-perp*uy*vx
        c,d=k*uy*vx-perp*ux*vy,k*uy*vy+perp*ux*vx
        coefficients=(a,b,start[0]-a*ts[0]-b*ts[1],c,d,start[1]-c*ts[0]-d*ts[1])
        arms.alpha_composite(source.transform((512,512),Image.Transform.AFFINE,coefficients,resample=Image.Resampling.BICUBIC))
    # A texture crop must never introduce pieces of torso or cuff outside the
    # approved forearm silhouette. Foreground gloves keep their authored shape
    # and black material, so a source crop cannot add a pale extra forearm.
    mask=native_arms.getchannel('A').resize((512,512),Image.Resampling.BICUBIC)
    arms.putalpha(ImageChops.multiply(arms.getchannel('A'),mask))
    base=native_arms.resize((512,512),Image.Resampling.BICUBIC);base.alpha_composite(arms)
    fingers=native_fingers.resize((512,512),Image.Resampling.BICUBIC)
    return base,fingers

def base_frame(index,bodies,subject=None,kind=None):
    body,master,ready=bodies[index];body=body.copy();master=master.resize((128,128),Image.Resampling.LANCZOS)
    boxes=[];parts=[]
    if subject is not None:
        p=profile_for(index)
        if kind=='head':
            head_boxes={9:(12,2,20,11),2:(11,2,20,10),1:(10,1,22,10),11:(9,2,22,11),5:(12,2,20,11)}
            boxes=[head_boxes[index]];parts=[subject]
        elif kind=='hands':
            boxes=[(x-2,y-2,x+3,y+3) for x,y in (p['gloves']['L'],p['gloves']['R'])]
            parts=[crop(subject.crop((0,0,subject.width//2,subject.height))),crop(subject.crop((subject.width//2,0,subject.width,subject.height)))]
        else:
            for x0,x1 in ((0,16),(16,32)):
                b=bbox(body.crop((x0,25,x1,32)),128);boxes.append((x0+b[0],25+b[1],x0+b[2],25+b[3]))
            parts=[crop(subject.crop((0,0,subject.width//2,subject.height))),crop(subject.crop((subject.width//2,0,subject.width,subject.height)))]
        for b,part in zip(boxes,parts):
            w,h=b[2]-b[0],b[3]-b[1]
            body.paste((0,0,0,0),b);master.paste((0,0,0,0),tuple(v*4 for v in b))
            body.alpha_composite(indexed(part.resize((w,h),Image.Resampling.LANCZOS),12),(b[0],b[1]))
            master.alpha_composite(part.resize((w*4,h*4),Image.Resampling.LANCZOS),(b[0]*4,b[1]*4))
    native=Image.new('RGBA',(SIZE,SIZE));normal=Image.new('RGBA',(512,512))
    native.alpha_composite(body,(48,48));normal.alpha_composite(master,(192,192))
    bb=bbox(bodies[index][0],128);scale=32/(bb[3]-bb[1]);ax,ay=64,48+bb[3]-1
    coeff=(1/scale,0,ax*(1-1/scale),0,1/scale,ay*(1-1/scale))
    native=native.transform((SIZE,SIZE),Image.Transform.AFFINE,coeff,resample=Image.Resampling.NEAREST)
    normal=normal.transform((512,512),Image.Transform.AFFINE,(coeff[0],0,coeff[2]*4,0,coeff[4],coeff[5]*4),resample=Image.Resampling.BICUBIC)
    return native,normal,{'body_id':BODY_IDS[index],'body_source_ready':ready,'body_native_height':32,'body_scale_from_body_only':scale,'kind':kind,'attachment_boxes':boxes}

def frame_record(cid,label,native,normal,meta,suffix):
    np=f'frames/{cid}-{suffix}-native.png';mp=f'frames/{cid}-{suffix}-normal.png'
    save(OUT/np,native);save(OUT/mp,normal)
    return {'class':label,'native':np,'normal':mp,'metadata':meta}
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--pilot',action='store_true');args=parser.parse_args()
    catalog=json.loads((OUT/'catalogue.json').read_text(encoding='utf-8'));entries=[e for e in catalog['entries'] if (ROOT/e['source_path']).exists()]
    bodies={i:body_layers(i) for i in range(17)};floor_tile=Image.open(BASE/'weapon-silhouette-review-03/review-floor-native32.png').convert('RGBA')
    floor=Image.new('RGBA',(SIZE,SIZE))
    for y in range(0,SIZE,32):
        for x in range(0,SIZE,32):floor.alpha_composite(floor_tile,(x,y))
    results=[];payload=[]
    for e in entries:
        subject,display,icon=object_image(e);frames=[]
        if e['kind'] in ('weapon','shield'):
            for n,index in enumerate(RIG['body_bindings'],1):
                native,normal,meta=compose(e,subject,index,bodies)
                frames.append(frame_record(e['review_id'],RIG['profiles'][n-1]['class'],native,normal,meta,f'C{n}'))
        elif e['kind']=='outfit':
            native,normal,meta=base_frame(e['body_index'],bodies)
            frames.append(frame_record(e['review_id'],'Sans arme',native,normal,meta,'base'))
            for k,weapon_id in enumerate(('BD-W069','BD-W102','BD-W001')):
                weapon=next((a for a in entries if a['review_id']==weapon_id),None)
                if weapon:
                    native,normal,meta=compose(weapon,crop(Image.open(ROOT/weapon['source_path']).convert('RGBA')),e['body_index'],bodies)
                    frames.append(frame_record(e['review_id'],weapon['name'],native,normal,meta,f'weapon{k}'))
        else:
            for n,index in enumerate(RIG['body_bindings'],1):
                native,normal,meta=base_frame(index,bodies,subject,e['kind'])
                frames.append(frame_record(e['review_id'],RIG['profiles'][n-1]['class'],native,normal,meta,f'C{n}'))
        payload.append({k:e.get(k) for k in ('review_id','content_id','name','kind','family_name')}|{'master':f"masters/{e['review_id']}.png",'icon':f"icons/{e['review_id']}-native32.png",'frames':frames})
        results.append({'review_id':e['review_id'],'source_sha256':hashlib.sha256((ROOT/e['source_path']).read_bytes()).hexdigest(),'placements':[f['metadata'] for f in frames]})
    board=Image.new('RGB',(1400,820),BG);d=ImageDraw.Draw(board)
    d.text((22,18),'BD-CATALOGUE01 · premiers montages séparés',font=font(27),fill=FG)
    d.text((22,59),'Sources isolées · personnage de 32 px · équipement indépendant, débordement autorisé · essais à examiner',font=font(14),fill=SUB)
    pilot=[e for e in payload if e['review_id'] in ('BD-W001','BD-W069','BD-W102','BD-A007','BD-EQ-S02')]
    for j,e in enumerate(pilot):
        x=18+j*274;d.text((x,106),e['review_id'],font=font(19),fill=FG);d.text((x,134),e['name'][:29],font=font(14),fill=SUB)
        display=Image.open(OUT/'masters'/f"{e['review_id']}.png").convert('RGBA');display.thumbnail((245,280),Image.Resampling.LANCZOS)
        board.paste(display,(x+(256-display.width)//2,178+(280-display.height)//2),display)
        if e['frames']:
            native=Image.open(OUT/e['frames'][0]['native']).convert('RGBA')
            scene=floor.copy();scene.alpha_composite(native)
            board.paste(scene,(x+65,479));board.paste(scene.resize((256,256),Image.Resampling.NEAREST),(x,630))
    save(OUT/'pilot-native-board.png',board)
    report={'review_id':'BD-CATALOGUE01','generated_models':len(entries),'target_models':178,'approval':'pending;not_integrated','results':results}
    (OUT/'render-validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    save(OUT/'floor-native.png',floor)
    data={'entries':payload,'floor':'floor-native.png','generated':len(entries),'target':178}
    (OUT/'gallery-data.json').write_text(json.dumps(data,ensure_ascii=False),encoding='utf-8')
    (OUT/'gallery-data.js').write_text('window.CATALOGUE='+json.dumps(data,ensure_ascii=False)+';',encoding='utf-8')
    for page in range((len(payload)+19)//20):
        board=Image.new('RGB',(1400,1240),BG);d=ImageDraw.Draw(board)
        d.text((20,12),f'BD-CATALOGUE01 · modèles distincts · page {page+1} · essais à examiner',font=font(23),fill=FG)
        for j,e in enumerate(payload[page*20:(page+1)*20]):
            x=15+j%5*278;y=62+j//5*291
            d.text((x,y),e['review_id'],font=font(17),fill=FG);d.text((x,y+23),e['name'][:32],font=font(13),fill=SUB)
            im=Image.open(OUT/e['master']).convert('RGBA');im.thumbnail((248,150),Image.Resampling.LANCZOS);board.paste(im,(x+(256-im.width)//2,y+51+(150-im.height)//2),im)
            native=Image.open(OUT/e['frames'][0]['native']).convert('RGBA');scene=floor.copy();scene.alpha_composite(native)
            # 88-pixel surroundings, body at its true native 32-pixel height.
            scene=scene.crop((20,20,108,108));board.paste(scene,(x+83,y+204))
        save(OUT/f'catalogue-board-{page+1:02}.png',board)
    print(json.dumps({'generated':len(entries),'body_sources':sum(v[2] for v in bodies.values()),'placements':sum(len(v['placements']) for v in results),'runtime_changes':0}))

if __name__=='__main__':main()
