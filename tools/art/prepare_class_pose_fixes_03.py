"""Package painted bent-arm patches; accepted originals stay immutable."""
import json
import math
import shutil
from PIL import Image, ImageDraw, ImageChops
from prepare_class_readability_review_01 import ROOT, sha, write_json, native_image

ART = ROOT/'assets/source-art/class-armor-pose-fixes-03'
OUT = ROOT/'artifacts/class-armor-integration-01/pose-fixes-03'
FISTS = {
    0: [(23,10),(40,10)], 1: [(23,11),(41,11)],
    2: [(22,12),(42,12)], 3: [(22,11),(42,11)],
    4: [(19,8),(45,8)], 15: [(24,9),(40,9)],
    16: [(23,10),(41,10)], 17: [(23,10),(41,10)],
    18: [(22,9),(42,9)], 19: [(23,8),(41,8)],
}


def selected(image, points):
    mask=Image.new('L',image.size)
    ImageDraw.Draw(mask).polygon(points,fill=255)
    result=image.copy()
    result.putalpha(ImageChops.multiply(image.getchannel('A'),mask))
    return result,mask


def rigid(image,pivot,angle,dx):
    c,s=math.cos(angle),math.sin(angle)
    px,py=pivot
    return image.transform(image.size,Image.Transform.AFFINE,
        (c,s,px-c*(px+dx)-s*py,-s,c,py+s*(px+dx)-c*py),Image.Resampling.NEAREST)


def metamorphe(manifest):
    source=ART/'ME-POST03-source.png'
    image,framing=native_image(source,56)
    original=Image.open(ROOT/manifest['outfits'][20]['native']['path']).convert('RGBA')
    points=[(24,33),(39,33),(42,39),(49,44),(50,63),(15,63),(16,42),(22,38)]
    _,mask=selected(image,points)
    for palm in manifest['outfits'][20]['palms']:
        x,y=[v-32 for v in palm]
        ImageDraw.Draw(mask).rectangle((x-3,y-4,x+3,y+5),fill=0)
    result=original.copy();result.paste(image,(0,0),mask)
    for palm in manifest['outfits'][20]['palms']:
        x,y=[v-32 for v in palm]
        assert result.crop((x-2,y-2,x+3,y+3)).tobytes()==original.crop((x-2,y-2,x+3,y+3)).tobytes()
    native=ART/'ME-POST03-native64.png';result.save(native)
    mask.save(ART/'ME-POST03-leg-ownership.png')
    entry={'candidate_id':'ME-POST03','index':20,
        'source':{'path':source.relative_to(ROOT).as_posix(),'sha256':sha(source)},
        'native':{'path':native.relative_to(ROOT).as_posix(),'sha256':sha(native)},'framing':framing,
        'leg_ownership_polygon':points,
        'preserved':'original pixels outside leg polygon, including head, torso, arms and hand anchors',
        'status':'requested posture correction; awaiting visual feedback'}
    write_json(ART/'metamorphe-posture.json',entry)
    return entry


def main():
    ART.mkdir(parents=True,exist_ok=True)
    OUT.mkdir(parents=True,exist_ok=True)
    plan=json.loads((ART/'generation.json').read_text('utf-8'))
    m=json.loads((ROOT/'assets/sprites/class-armors-01/manifest.json').read_text('utf-8'))
    meta=metamorphe(m)
    entries=[]
    board=Image.new('RGB',(800,630),'#20272b');d=ImageDraw.Draw(board)
    for job in plan['jobs']:
        index=job['index'];source=ART/f'outfit-{index:02}-raised-source.png'
        if not source.exists(): shutil.copyfile(job['source'],source)
        image,framing=native_image(source,58 if index<5 else 60)
        image.save(ART/f'outfit-{index:02}-raised-native64.png')
        ref=m['outfits'][index]['native'];base=Image.open(ROOT/ref['path']).convert('RGBA')
        head=Image.open(ROOT/'assets/sprites/class-armors-01/head-mask.png').convert('RGBA')
        head=head.crop((index%16*128+32,index//16*128+32,index%16*128+96,index//16*128+96))
        body=base.copy();parts={};palms=[]
        for side in range(2):
            pivot=(24,20) if side==0 else (40,20)
            degrees=40 if index==4 else 60
            angle=math.radians(-degrees if side==0 else degrees)
            hand=FISTS[index][side]
            c,s=math.cos(angle),math.sin(angle)
            target=[pivot[0]+c*(hand[0]-pivot[0])-s*(hand[1]-pivot[1]),
                    pivot[1]+s*(hand[0]-pivot[0])+c*(hand[1]-pivot[1])]
            dx=min(0,math.floor(14-target[0])) if side==0 else max(0,math.ceil(50-target[0]))
            target[0]+=dx
            points=[(0,0),(25,0),(25,15),(27,22),(24,25),(0,25)]
            if side:points=[(64-x,y) for x,y in points]
            arm,_=selected(image,points)
            arm=rigid(arm,pivot,angle,dx)
            for y in range(64):
                for x in range(64):
                    if head.getpixel((x,y))[3] or (25<=x<=39) or y>=31:
                        arm.putpixel((x,y),(0,0,0,0))
            old=[(0,21),(24,21),(24,30),(23,39),(0,40)]
            if side:old=[(64-x,y) for x,y in old]
            _,erase=selected(base,old)
            erase_rgba=Image.new('RGBA',(64,64));erase_rgba.putalpha(erase)
            body.paste((0,0,0,0),(0,0,64,64),erase)
            body.alpha_composite(arm)
            tx,ty=[round(v) for v in target]
            fingers,_=selected(arm,[(tx-1,ty-1),(tx+1,ty-1),(tx+1,ty+1),(tx-1,ty+1)])
            for name,im in [('arm',arm),('erase',erase_rgba),('fingers',fingers)]:
                path=ART/f'outfit-{index:02}-raised-{name}-{side}.png';im.save(path)
                parts[f'{name}-{side}']={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
            palms.append([v+32 for v in target])
        entries.append({'candidate_id':f'POSE03-{index:02}','index':index,'palms':palms,'parts':parts,
            'source':{'path':source.relative_to(ROOT).as_posix(),'sha256':sha(source)},'framing':framing})
        row=index%5;col=0 if index<5 else 1
        for x,im in [(col*400+25,base),(col*400+200,body)]:
            zoom=im.resize((120,120),Image.Resampling.NEAREST);board.paste(zoom,(x,row*124),zoom)
        d.text((col*400+150,row*124+5),f'{index%5+1:02}',fill='white')
    write_json(ART/'production.json',{'review_id':'CLASS-POSE-FIX03','entries':entries,'metamorphe':meta,
        'status':'requested corrective candidate, awaiting user visual feedback',
        'rules':'weapons unchanged; painted arms rigidly posed without stretching; original head and central torso preserved'})
    board.save(OUT/'patched-bodies.png')
    print(json.dumps({'corrected_arm_outfits':len(entries),'metamorphe_posture':20}))


if __name__=='__main__':main()
