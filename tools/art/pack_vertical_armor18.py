"""Prepare whole C10 armor figures at native64; never modify approved banks."""
from pathlib import Path
import json,math
from PIL import Image,ImageDraw,ImageFont
from pack_vertical_walk_equipment18 import complete_figures
from pack_vertical_walk_size18 import structural_box,condition
from pack_vertical_walk_review18 import opaque_box

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/vertical-walk-18'
ART=ROOT/'assets/source-art/vertical-walk-review-18/armor-C10'
OUT=ART/'conditioned'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/armor-C10'
CLASS={'core:bastion':('bastion',3,58),'core:traqueur':('traqueuse',5,56),'core:arcaniste':('arcaniste',6,60),'core:metamorphe':('metamorphe',7,56)}
ORDER=[1,2,3,5,6,7,9,10,11,13,14,15,0]

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(p):return p.relative_to(ROOT).as_posix()

def palm(image,hint):
    hx,hy=hint;points=[]
    for y in range(round(hy)-5,round(hy)+6):
        for x in range(round(hx)-5,round(hx)+6):
            r,g,b,a=image.getpixel((x,y))
            if a>=128 and max(r,g,b)<160:
                w=a*((255-max(r,g,b))/255)**2/(1+(x-hx)**2+(y-hy)**2)
                points.append((x,y,w))
    assert points,('missing hand',hint)
    total=sum(v[2] for v in points)
    return [round(sum(v[i]*v[2] for v in points)/total,2) for i in range(2)]

def atlas(images):
    sheet=Image.new('RGBA',(1536,768))
    for n,im in enumerate(images):sheet.alpha_composite(im,(n%6*256+64,n//6*256+64))
    return sheet

def whole_figures(image):
    try:return complete_figures(image)
    except AssertionError as error:
        if 'row boundary' not in str(error):raise
    # Different columns can have slightly different transparent row breaks.
    # Cut each column at a real alpha<32 valley, preserving all structural pixels.
    alpha=image.getchannel('A');columns=[]
    for col in range(4):
        left,right=round(col*image.width/4),round((col+1)*image.width/4);edges=[0]
        for q in [1,2,3]:
            center=round(q*image.height/4);radius=round(image.height*.055)
            count,y=min((sum(a>=32 for a in alpha.crop((left,y,right,y+1)).tobytes()),y) for y in range(center-radius,center+radius+1))
            assert count==0,('no transparent column boundary',col,q,count)
            edges.append(y)
        edges.append(image.height);parts=[]
        for row in range(4):
            part=image.crop((left,edges[row],right,edges[row+1]));a=part.getchannel('A')
            for y in [0,part.height-1]:
                for x in range(part.width):
                    if a.getpixel((x,y))<32:a.putpixel((x,y),0)
            part.putalpha(a);box=opaque_box(part);assert box and box[0]>0 and box[2]<part.width
            whole=part.crop(box);padded=Image.new('RGBA',(whole.width+4,whole.height+4));padded.alpha_composite(whole,(2,2));parts.append(padded)
        columns.append(parts)
    return [columns[c][r] for r in range(4) for c in range(4)]

def main():
    OUT.mkdir(parents=True,exist_ok=True);PREVIEW.mkdir(parents=True,exist_ok=True)
    jobs=[j for j in read(DOC/'prompts-C10.json')['jobs'] if j['family']=='one'];reference=read(ROOT/'assets/source-art/vertical-walk-review-18/extension-C06/conditioned-weapons/manifest.json')
    profiles=[];errors=[];font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),12)
    for job in jobs:
        source=ROOT/job.get('selected_output',job['output'])
        if not source.exists():continue
        name,body,height=CLASS[job['class_id']]
        try:
            figures=whole_figures(Image.open(source).convert('RGBA'));assert len(figures)==16
            normalized=[];scales=[]
            for row in range(4):
                box=structural_box(figures[row*4]);scale=height/(box[3]-box[1]);scales.append(scale)
                normalized.extend(condition(im,scale,95)[0] for im in figures[row*4:row*4+4])
            baseline=next(p for p in reference['profiles'] if p['id']==name)['family_poses']['one']['frames']
            cores=[];fingers=[];frames=[];raw=[]
            for n,index in enumerate(ORDER):
                im=normalized[index];box=structural_box(im);back=6<=n<12
                grips=[palm(im,h) for h in baseline[n]['grips']]
                core=im.copy();hand=Image.new('RGBA',(128,128))
                for x,y in grips:
                    x,y=round(x),round(y);area=(x-2,y-2,x+3,y+3);hand.paste(im.crop(area),area[:2]);core.paste((0,0,0,0),area)
                cores.append(core);fingers.append(hand);raw.append(im)
                frames.append({'direction':'back' if back else 'front' if n<12 else 'idle','phase':n%6,'grips':grips,'head_anchor':[64,box[1]+6],'grips_occluded':False,'bounds':list(box)})
            bodyfile=OUT/(job['profile']+'-one-body.png');handfile=OUT/(job['profile']+'-one-hands.png');fullfile=OUT/(job['profile']+'-one-complete.png');atlas(cores).save(bodyfile);atlas(fingers).save(handfile);atlas(raw).save(fullfile)
            idle=OUT/(job['profile']+'-idle.png');raw[12].save(idle)
            # Inspection only: native whole figures and explicit owned-hand markers.
            board=Image.new('RGB',(6*128,3*145+20),'#1e2831');draw=ImageDraw.Draw(board)
            for n,im in enumerate(raw):
                x,y=(n%6)*128,(n//6)*145;board.paste(im,(x,y+18),im);draw.text((x+4,y+2),f'{n+1:02}',font=font,fill='white')
                for px,py in frames[n]['grips']:draw.point((x+round(px),y+18+round(py)),fill='#32ff55')
            board.save(PREVIEW/(job['profile']+'-hands-audit.png'))
            profiles.append({'id':job['profile'],'number':job['number'],'class_id':job['class_id'],'outfit':job['index'],'rank':job['rank'],'selector':{'kind':'player','body':body,'equipment':[{1:110,2:102,3:101}[job['rank']],None,None,None]},'source':rel(source),'reference_front':job['references'][0],'original_armor':job['references'][1],'target_height':height,'foot_y':95,'row_static_scales':scales,'idle':rel(idle),'family_poses':{'one':{'body':rel(bodyfile),'hands':rel(handfile),'complete':rel(fullfile),'frames':frames}}})
        except (AssertionError,ValueError,OSError) as e:errors.append({'id':job['id'],'error':str(e)})
    write(OUT/'manifest.json',{'review':'VERT18-C10','normal_game_enabled':False,'native_tile_px':64,'profiles':profiles,'conditioning_errors':errors})
    print(json.dumps({'prepared_profiles':len(profiles),'conditioning_errors':errors}))

if __name__=='__main__':main()
