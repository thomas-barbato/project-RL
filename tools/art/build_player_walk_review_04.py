"""Painted 3/4 review. Whole arm/weapon silhouettes are NEVER decomposed.

Only an explicitly owned area below the hips is animated. Full painted upper
bodies, complete sleeves, wrists and weapons move together without erasing them.
"""
from __future__ import annotations
import json, math, hashlib, argparse
import copy
from array import array
from collections import deque
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont, ImageFilter

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-04'
DOC=ROOT/'docs/reviews/player-walk-04'
SRC=ROOT/'assets/source-art/player-walk-review-04'

def components(im,threshold=48):
    """Object ownership, not limb extraction or a color-based arm mask."""
    w,h=im.size;a=bytearray(im.getchannel('A').point(lambda p:255 if p>=threshold else 0).tobytes())
    result=[]
    for start in range(w*h):
        if not a[start]:continue
        stack=[start];a[start]=0;points=[];x0=w;y0=h;x1=y1=0;sx=sy=0
        while stack:
            n=stack.pop();points.append(n);x=n%w;y=n//w;sx+=x;sy+=y
            x0=min(x0,x);x1=max(x1,x);y0=min(y0,y);y1=max(y1,y)
            for yy in range(max(0,y-1),min(h,y+2)):
                for xx in range(max(0,x-1),min(w,x+2)):
                    p=yy*w+xx
                    if a[p]:a[p]=0;stack.append(p)
        result.append({'points':points,'area':len(points),'box':(x0,y0,x1+1,y1+1),'center':(sx/len(points),sy/len(points))})
    return result

def figures(path,rows):
    im=Image.open(path).convert('RGBA');w,h=im.size
    comps=components(im);sides=[];warnings=[]
    for col in range(2):
        candidates=[c for c in comps if (c['center'][0]<w/2)==(col==0)]
        primary=sorted(candidates,key=lambda c:c['area'],reverse=True)[:rows]
        assert len(primary)==rows,(path,len(primary),rows)
        primary.sort(key=lambda c:c['center'][1]);groups=[[c] for c in primary]
        for c in candidates:
            if c in primary or c['area']<6:continue
            row=min(range(rows),key=lambda r:abs(primary[r]['center'][1]-c['center'][1]))
            groups[row].append(c)
            if c['area']>primary[row]['area']*.06:warnings.append({'row':row,'column':col,'detached_pixels':c['area']})
        output=[]
        for group in groups:
            points=[n for c in group for n in c['points']]
            mask=bytearray(w*h)
            for n in points:mask[n]=255
            mask=Image.frombytes('L',im.size,bytes(mask)).filter(ImageFilter.MaxFilter(5))
            owned=Image.new('RGBA',im.size);owned.paste(im,(0,0),mask)
            box=owned.getchannel('A').point(lambda p:255 if p>=32 else 0).getbbox()
            owned=owned.crop((max(0,box[0]-3),max(0,box[1]-3),min(w,box[2]+3),min(h,box[3]+3)))
            output.append(owned)
        sides.append(output)
    return [[sides[0][r],sides[1][r]] for r in range(rows)],warnings

def normalize(im,neutral,side=128,foot=(64,96),humanoid=True):
    a=im.getchannel('A').point(lambda p:255 if p>=64 else 0);box=a.getbbox()
    bodyheight=box[3]-box[1]
    bootbox=(0,max(box[1],box[3]-max(5,int(bodyheight*.065))),im.width,box[3])
    boot=a.crop(bootbox);coords=[(i%boot.width+bootbox[0],v) for i,v in enumerate(boot.tobytes()) if v]
    cx=sum(x*v for x,v in coords)/sum(v for _,v in coords)
    if not humanoid:cx=(box[0]+box[2])/2
    band=(max(0,round(cx-bodyheight*.11)),0,min(im.width,round(cx+bodyheight*.11)),im.height)
    top=a.crop(band).getbbox()[1] if humanoid else box[1]
    native=neutral.getchannel('A').point(lambda p:255 if p>=64 else 0)
    center=native.crop((max(0,foot[0]-max(5,side//10)),0,min(side,foot[0]+max(5,side//10)),side)).getbbox()
    nativebox=native.getbbox()
    targettop=center[1] if humanoid and center else nativebox[1]
    # A hovering source keeps its actual gap above the logical ground contact.
    targetbottom=nativebox[3]-1
    fy=box[3]-1;scale=(targetbottom-targettop)/max(1,fy-top)
    resized=im.resize((max(1,round(im.width*scale)),max(1,round(im.height*scale))),Image.Resampling.LANCZOS)
    targetcx=foot[0] if humanoid else (nativebox[0]+nativebox[2])/2
    out=Image.new('RGBA',(side,side));out.alpha_composite(resized,(round(targetcx-cx*scale),round(targetbottom-fy*scale)))
    # Remove only sub-visible resampling fringe, never body interiors.
    alpha=out.getchannel('A').point(lambda p:0 if p<10 else p);out.putalpha(alpha)
    return out

def leg_mask(im,hip,foot):
    """Connected feet below the hip, excluding carried objects beside them."""
    side=im.width;area=Image.new('RGBA',im.size);area.alpha_composite(im.crop((0,hip,side,side)),(0,hip))
    comps=components(area,32);mask=bytearray(side*side)
    # The two central boot contacts own the legs. No mask reaches an elbow,
    # wrist or sleeve; rods beside the legs do not become body pixels.
    for c in comps:
        if c['box'][3]<foot[1]-4:continue
        bottom=[n for n in c['points'] if n//side>=foot[1]-7 and abs(n%side-foot[0])<side*.19]
        if not bottom:continue
        for n in c['points']:mask[n]=255
    return Image.frombytes('L',im.size,bytes(mask))

def animated(im,rig,foot,equipped=False):
    side=im.width;b=im.getchannel('A').point(lambda p:255 if p>=64 else 0).getbbox()
    # With equipment, move only the feet and lower shins. In particular a long
    # diagonal shaft beside the thighs never enters a leg deformation mask.
    hip=round(foot[1]-(foot[1]-b[1])*(.17 if equipped else .33))
    if rig in ['float','serpent','roll']:
        frames=[]
        for f in range(8):
            phase=f/8*math.tau;s=math.sin(phase)
            out=Image.new('RGBA',im.size)
            if rig=='serpent':
                for y in range(side):out.alpha_composite(im.crop((0,y,side,y+1)),(round(math.sin(phase+(y-b[1])*.10)*1.5),y))
            else:out.alpha_composite(im,(0,round(-abs(s)*1.5) if rig=='float' else 0))
            frames.append(out)
        return frames+[im.copy()]
    mask=leg_mask(im,hip,foot);fixed=im.copy();fixed.putalpha(Image.eval(im.getchannel('A'),lambda p:p))
    erase=Image.new('RGBA',im.size);fixed.paste(erase,(0,0),mask)
    legs=Image.new('RGBA',im.size);legs.paste(im,(0,0),mask)
    frames=[]
    for f in range(8):
        phase=f/8*math.tau;s=math.sin(phase);bob=round(-abs(s)*.8)
        out=Image.new('RGBA',im.size)
        if rig=='robe':
            for y in range(hip,side):
                t=max(0,min(1,(y-hip)/max(1,foot[1]-hip)))
                out.alpha_composite(legs.crop((0,y,side,y+1)),(round(s*t*.9),round(y+bob*(1-t))))
        else:
            count=4 if rig in ['quad','crawl','machine'] else 2
            for n in range(count):
                x0=round(n*side/count);x1=round((n+1)*side/count)
                limb=Image.new('RGBA',im.size);limb.alpha_composite(legs.crop((x0,0,x1,side)),(x0,0))
                wave=math.sin(phase+(n%2)*math.pi);lift=max(0,wave)*(1 if rig=='heavy' else 1.5)
                stride=wave*(1.8 if rig=='heavy' else 2.6)
                length=max(4,foot[1]-hip);scale=(length-lift-bob)/length
                shear=stride/max(4,length-lift-bob)
                warped=limb.transform(im.size,Image.Transform.AFFINE,(1,-shear,shear*(hip+bob),0,1/scale,hip-(hip+bob)/scale),Image.Resampling.NEAREST)
                out.alpha_composite(warped)
        # Feet are behind the intact carried object, never painted over it.
        out.alpha_composite(fixed,(0,bob))
        # The entire original upper body, including every shoulder, sleeve,
        # wrist, finger and held object is retained as one painted layer.
        expected=Image.new('RGBA',im.size);expected.alpha_composite(im,(0,bob))
        protected=(0,0,side,max(0,hip+bob-2))
        assert out.crop(protected).tobytes()==expected.crop(protected).tobytes(),'upper-body pixels changed'
        frames.append(out)
    return frames+[im.copy()]

SITE=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')
CLASS_LABELS=['Sans arme','Pistolet · une main','Marteau · une main','Fusil · deux mains','Hache · deux mains','Deux pistolets']

def native(record):return Image.open(SITE/record['image']).convert('RGBA')

def review_record(record):
    """Give small legacy sprites transparent head/weapon margins, at 1:1 scale."""
    r=copy.deepcopy(record)
    if r['size'][0]<128:
        pad=(128-r['size'][0])//2;r['size']=[128,128]
        r['foot']=[v+pad for v in r['foot']]
        r['bounds']=[v+pad for v in r['bounds']]
    return r

def review_front(record,im=None):
    im=native(record) if im is None else im
    if im.width>=128:return im
    pad=(128-im.width)//2;out=Image.new('RGBA',(128,128));out.alpha_composite(im,(pad,pad));return out

def pack(record,views,labels):
    side=record['size'][0];foot=record['foot'];key=record['id']
    sheet=Image.new('RGBA',(9*side,len(views)*3*side))
    board=Image.new('RGB',(3*(side+16),len(views)*(side+20)),'#263136');d=ImageDraw.Draw(board)
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',11)
    for row,directions in enumerate(views):
        for direction,view in enumerate(directions):
            # All arms and carried objects belong to the rigid painted layer.
            for f,frame in enumerate(animated(view,record['rig'],foot,equipped=row>0 or bool(record.get('carried')))):
                sheet.alpha_composite(frame,(f*side,(row*3+direction)*side))
            board.paste(view,(direction*(side+16)+8,row*(side+20)),view)
        d.text((8,row*(side+20)+side),f'{key} · {row:02} · {labels[row]}',font=font,fill='white')
    sheet.save(ART/f'{key}-atlas.png',compress_level=6);board.save(ART/f'{key}-native.png')
    return {'id':key,'atlas':f'{key}-atlas.png','labels':labels,'size':side,'foot':foot,'bounds':record['bounds'],'rows':len(views)*3,'frames':9,'views':len(views)*3,'upper_body_exact':True}

def recolor(im,reference,target):
    """Transfer an existing palette, preserving alpha and object geometry.

    Shared unchanged colors (eyes, weapons, details) are excluded from the
    transfer. No hue rotation over the whole outfit or creature.
    """
    if reference.tobytes()==target.tobytes():return im.copy()
    counts={};unchanged=set()
    for a,b in zip(reference.getdata(),target.getdata()):
        if min(a[3],b[3])<64:continue
        key=a[:3]
        if key==b[:3]:unchanged.add(key)
        else:
            v=counts.setdefault(key,[0,0,0,0])
            for k in range(3):v[k]+=b[k]
            v[3]+=1
    keys=list(dict.fromkeys([p[:3] for p in reference.getdata() if p[3]>=64]))
    if len(keys)>256:
        rgb=reference.convert('RGB');quant=rgb.quantize(colors=255,dither=Image.Dither.NONE)
        pal=quant.getpalette();keys=[tuple(pal[i:i+3]) for i in range(0,len(pal),3)]
    keys=keys[:256]
    palette=Image.new('P',(1,1));flat=[v for p in keys for v in p];palette.putpalette(flat+[0]*(768-len(flat)))
    quant=im.convert('RGB').quantize(palette=palette,dither=Image.Dither.NONE)
    outpalette=[]
    for key in keys:
        if key in unchanged or key not in counts:outpalette+=list(key)
        else:outpalette += [round(v/counts[key][3]) for v in counts[key][:3]]
    quant.putpalette(outpalette+[0]*(768-len(outpalette)));out=quant.convert('RGBA');out.putalpha(im.getchannel('A'))
    # Only mapped material colors are changed. Other painted colors retain
    # their generated shading, rather than being quantized to the reference.
    result=im.copy();rp=result.load();op=out.load();qp=quant.load()
    for y in range(im.height):
        for x in range(im.width):
            k=qp[x,y];color=keys[k] if k<len(keys) else (0,0,0)
            if color in counts and color not in unchanged:rp[x,y]=op[x,y]
    assert result.getchannel('A').tobytes()==im.getchannel('A').tobytes()
    return result

def main():
    ART.mkdir(parents=True,exist_ok=True)
    baseline=json.loads((DOC/'baseline-data.json').read_text('utf-8'))
    records={r['id']:r for r in baseline['records']}
    jobs=json.loads((DOC/'jobs.json').read_text('utf-8'))+json.loads((DOC/'other-jobs.json').read_text('utf-8'))
    cachepath=DOC/'package-cache.json';cache=json.loads(cachepath.read_text('utf-8')) if cachepath.exists() else {}
    code=hashlib.sha256(Path(__file__).read_bytes()).hexdigest();report=[];errors=[]
    for job in jobs:
        path=Path(job['source_destination'])
        if not path.exists():continue
        signature=hashlib.sha256(path.read_bytes()+json.dumps(job,sort_keys=True).encode()+code.encode()).hexdigest()
        old=cache.get(job['id'])
        if old and old['signature']==signature and all((ART/e['atlas']).exists() for e in old['entries']):
            report+=old['entries'];continue
        try:
            count=job.get('rows',6);poses,warnings=figures(path,count);entries=[]
            if job.get('kind')=='actors':
                for row,record in enumerate(job['records']):
                    front=review_front(record);record=review_record(record);side=front.width
                    humanoid=record['rig'] in ['biped','heavy','robe']
                    views=[[normalize(poses[row][0],front,side,record['foot'],humanoid),front,normalize(poses[row][1],front,side,record['foot'],humanoid)]]
                    entry=pack(record,views,['Dessin complet']);entry['source']=str(path.relative_to(ROOT));entry['warnings']=[w for w in warnings if w['row']==row];entries.append(entry)
            else:
                record=job['record'];fronts=[Image.open(ROOT/p).convert('RGBA') for p in job['fronts']];neutral=fronts[0]
                views=[[normalize(pair[0],neutral),fronts[row],normalize(pair[1],neutral)] for row,pair in enumerate(poses)]
                entry=pack(record,views,job.get('labels',CLASS_LABELS));entry['source']=str(path.relative_to(ROOT));entry['warnings']=warnings;entries.append(entry)
            cache[job['id']]={'signature':signature,'entries':entries};report+=entries
            print(json.dumps({'packaged':job['id'],'entries':len(entries),'warnings':len(warnings)}),flush=True)
        except Exception as error:
            errors.append({'job':job['id'],'error':str(error)})
    # Variants use the same accepted geometry. Recolor the complete side view,
    # then use the exact actual front, without inventing another anatomy.
    byid={r['id']:r for r in report}
    for record in baseline['records']:
        key=record['id']
        if key in byid:continue
        if key.startswith('MON-'):basekey=f'MON-{record["number"]:03}-0'
        elif key.startswith('ARMOR-'):basekey=key.rsplit('-',1)[0]+'-0'
        elif key.startswith('ACTOR-') and 3<=int(key.split('-')[1])<=25:basekey=f'BANK-{int(key.split("-")[1])+30:03}'
        else:continue
        if basekey not in byid:continue
        base=records[basekey];baseentry=byid[basekey];atlas=Image.open(ART/baseentry['atlas']).convert('RGBA');side=baseentry['size']
        signature=hashlib.sha256(atlas.tobytes()+json.dumps(record,sort_keys=True).encode()+code.encode()).hexdigest();old=cache.get(key)
        if old and old['signature']==signature and (ART/old['entries'][0]['atlas']).exists():report+=old['entries'];continue
        targetfront=review_front(record);basefront=review_front(base);review=review_record(record);base_review=review_record(base);views=[]
        for row in range(len(baseentry['labels'])):
            if key.startswith('ARMOR-'):
                c=[0,1,2,4,9,11][row]
                target=Image.open(SITE/record['pack'][0]).convert('RGBA').crop((c*128,0,(c+1)*128,128))
                reference=Image.open(SITE/base['pack'][0]).convert('RGBA').crop((c*128,0,(c+1)*128,128))
            else:target=targetfront;reference=basefront
            directions=[]
            for direction in range(3):
                pose=atlas.crop((8*side,(row*3+direction)*side,9*side,(row*3+direction+1)*side))
                if direction==1:pose=target
                else:
                    pose=recolor(pose,reference,target)
                    if pose.size!=target.size or base_review['foot']!=review['foot']:
                        pose=normalize(pose,target,target.width,review['foot'],record['rig'] in ['biped','heavy','robe'])
                directions.append(pose)
            views.append(directions)
        entry=pack(review,views,baseentry['labels']);entry['derived_from']=basekey;entry['warnings']=[];entry['palette_geometry_preserved']=True
        cache[key]={'signature':signature,'entries':[entry]};report.append(entry)
    cachepath.write_text(json.dumps(cache,ensure_ascii=False,indent=2),'utf-8')
    (DOC/'packaging.json').write_text(json.dumps({'review':'WALK04-r01','runtime_integration':False,'entries':report,'errors':errors},ensure_ascii=False,indent=2),'utf-8')
    print(json.dumps({'packaged':len(report),'pending':len(records)-len(report),'warnings':sum(len(r.get('warnings',[])) for r in report),'errors':errors}))

if __name__=='__main__':main()
