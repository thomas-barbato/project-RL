"""C12 whole authored two-handed cycles with original C10 modular heads."""
from pathlib import Path
import argparse,ast,json,hashlib
from PIL import Image,ImageDraw,ImageFont,ImageChops
from pack_vertical_armor18 import whole_figures,ORDER,atlas
from pack_vertical_walk_size18 import structural_box,condition
from pack_vertical_walk_review18 import opaque_box

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18'
OUT=ART/'grips-C12/conditioned'
DOC=ROOT/'docs/reviews/vertical-walk-18'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/grips-C12'
# Grip openings inspected on the actual authored fingers, not inherited cuffs.
PALMS={
'bastion-armor1':{'ranged':[[59,64],[73,63]],'melee-wide':[[56,63],[73,63]],'melee-close':[[58.5,67],[59.5,63.5]]},
'bastion-armor2':{'ranged':[[60,62],[72,64]],'melee-wide':[[56,64],[74,64]],'melee-close':[[62,65],[60.5,61.5]]},
'bastion-armor3':{'ranged':[[60,63],[74,63]],'melee-wide':[[56.5,63],[75.5,63]],'melee-close':[[61.5,66],[62,62]]},
'traqueuse-armor1':{'ranged':[[61,62],[72,63]],'melee-wide':[[59,64],[72,64]],'melee-close':[[61.5,66.5],[62.5,62.5]]},
'traqueuse-armor2':{'ranged':[[60.5,64.5],[70.5,66]],'melee-wide':[[57.5,65],[72,65]],'melee-close':[[60,70],[61,66]]},
'traqueuse-armor3':{'ranged':[[61,65],[72,66]],'melee-wide':[[57,65],[74,65]],'melee-close':[[59.5,69.5],[61.5,65.5]]},
'arcaniste-armor1':{'ranged':[[59,64],[72,65]],'melee-wide':[[55,64],[74.5,64]],'melee-close':[[62,67],[63,62.5]]},
'arcaniste-armor2':{'ranged':[[60,62],[71,62]],'melee-wide':[[55.5,63],[74.5,63]],'melee-close':[[61,66],[61,60.5]]},
'arcaniste-armor3':{'ranged':[[58,63],[70,64]],'melee-wide':[[55.5,65.5],[76,65.5]],'melee-close':[[61,68.5],[58.5,64]]},
'metamorphe-armor1':{'ranged':[[61,63],[71,66]],'melee-wide':[[56,66],[73,66]],'melee-close':[[59.5,68.5],[60.5,63.5]]},
'metamorphe-armor2':{'ranged':[[61,64],[71,64]],'melee-wide':[[56,65.5],[74,65.5]],'melee-close':[[62,68],[61,64.5]]},
'metamorphe-armor3':{'ranged':[[60,64],[71,64]],'melee-wide':[[56,66],[74,66]],'melee-close':[[58.5,67],[61,63]]},
}

def read(p):return json.loads(p.read_text('utf8'))
def write(p,d):p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n','utf8')
def rel(p):return p.relative_to(ROOT).as_posix()
def native_cell(sheet,n):return sheet.crop((n%6*256+64,n//6*256+64,n%6*256+192,n//6*256+192))

def complete_source(image):
    try:return whole_figures(image)
    except AssertionError as error:
        if 'boundary' not in str(error):raise
    # A few sources have isolated translucent fringe dots in otherwise empty
    # gutters. Require a genuinely empty structural row in EACH column. Never
    # cut an alpha>=128 body pixel or deform/splice a figure.
    alpha=image.getchannel('A');columns=[]
    for col in range(4):
        left,right=round(col*image.width/4),round((col+1)*image.width/4);edges=[0]
        for q in [1,2,3]:
            center=round(q*image.height/4);radius=round(image.height*.055)
            count,_,y=min((sum(a>=128 for a in alpha.crop((left,y,right,y+1)).tobytes()),abs(y-center),y) for y in range(center-radius,center+radius+1))
            assert count==0,('touching structural figures',col,q,count)
            edges.append(y)
        edges.append(image.height);parts=[]
        for row in range(4):
            part=image.crop((left,edges[row],right,edges[row+1]));a=part.getchannel('A')
            for y in [0,part.height-1]:
                for x in range(part.width):
                    if a.getpixel((x,y))<128:a.putpixel((x,y),0)
            part.putalpha(a);box=opaque_box(part);assert box and box[0]>0 and box[2]<part.width
            whole=part.crop(box);padded=Image.new('RGBA',(whole.width+4,whole.height+4));padded.alpha_composite(whole,(2,2));parts.append(padded)
        columns.append(parts)
    return [columns[c][row] for row in range(4) for c in range(4)]

def fit(reference,target,point):
    x,y=map(round,point);sample=list(reference.crop((x-3,y-3,x+4,y+4)).get_flattened_data());scores=[]
    for dy in range(-4,5):
        for dx in range(-4,5):
            xx,yy=x+dx,y+dy
            if target.getpixel((xx,yy))[3]<128:continue
            patch=target.crop((xx-3,yy-3,xx+4,yy+4));score=0.
            for a,b in zip(sample,patch.get_flattened_data()):
                score+=abs(a[3]-b[3])+sum(abs(a[c]*a[3]/255-b[c]*b[3]/255) for c in range(3))
            scores.append((score/(49*4)+1.4*(dx*dx+dy*dy),xx,yy))
    assert scores,('no hand near',point)
    _,x,y=min(scores);return [x,y]

def track_pair(reference,target,points):
    samples=[list(reference.crop((round(x)-3,round(y)-3,round(x)+4,round(y)+4)).get_flattened_data()) for x,y in points]
    scores=[]
    for dy in range(-2,3):
        for dx in range(-2,3):
            score=0.
            for (x,y),sample in zip(points,samples):
                xx,yy=round(x+dx),round(y+dy);patch=target.crop((xx-3,yy-3,xx+4,yy+4))
                for a,b in zip(sample,patch.get_flattened_data()):
                    score+=abs(a[3]-b[3])+sum(abs(a[c]*a[3]/255-b[c]*b[3]/255) for c in range(3))
            scores.append((score/(2*49*4)+1.4*(dx*dx+dy*dy),dx,dy))
    _,dx,dy=min(scores)
    return [[x+dx,y+dy] for x,y in points]

def main():
    if (DOC/'acceptance-C12.json').exists():
        raise SystemExit('C12 is accepted and frozen; use a new revision for further visual changes.')
    parser=argparse.ArgumentParser();parser.add_argument('--require-complete',action='store_true');args=parser.parse_args()
    OUT.mkdir(parents=True,exist_ok=True);PREVIEW.mkdir(exist_ok=True)
    jobs=read(DOC/'prompts-C12.json')['jobs'];base=read(ART/'armor-C10/conditioned/manifest.json');accepted=read(ART/'extension-C06/conditioned-weapons/manifest.json')
    tree=ast.parse((ROOT/'tools/art/pack_vertical_weapons18.py').read_text('utf8'))
    contacts=next(ast.literal_eval(n.value) for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='CONTACTS' for t in n.targets))
    headmask=Image.open(ART/'candidate-C04/head-ownership.png').convert('RGBA').getchannel('A')
    profiles=[];failures=[];count=0
    for p in base['profiles']:
        name=p['id'].split('-')[0];reference=next(v for v in accepted['profiles'] if v['id']==name);variants={}
        original=Image.open(ROOT/p['family_poses']['one']['complete']).convert('RGBA');heads=[]
        for index in [12,6]:
            im=native_cell(original,index);anchor=p['family_poses']['one']['frames'][index]['head_anchor'];patch=im.crop((48,round(anchor[1])-16,80,round(anchor[1])+16));patch.putalpha(ImageChops.multiply(patch.getchannel('A'),headmask));heads.append(patch)
        for job in [j for j in jobs if j['profile']==p['id']]:
            record=DOC/'generation-C12'/(job['id']+'.json')
            if not record.exists():continue
            try:
                source=ROOT/job.get('selected_output',job['output']);assert hashlib.sha256(source.read_bytes()).hexdigest()==job.get('selected_sha256',read(record)['sha256'])
                figures=complete_source(Image.open(source).convert('RGBA'));normalized=[];scales=[]
                for row in range(4):
                    box=structural_box(figures[row*4]);scale=p['target_height']/(box[3]-box[1]);scales.append(scale);normalized.extend(condition(im,scale,95)[0] for im in figures[row*4:row*4+4])
                family=job['family'];ref=reference['family_poses'][family];refim=Image.alpha_composite(Image.open(ROOT/ref['body']).convert('RGBA'),Image.open(ROOT/ref['hands']).convert('RGBA'));refim=native_cell(refim,12)
                # Locate the actual closed fingers using the same-class authored
                # fist template. Audit sheets expose every chosen point.
                neutral=normalized[0];points=PALMS[p['id']][family]
                assert ((points[1][0]-points[0][0])**2+(points[1][1]-points[0][1])**2)**.5>=3
                images=[];frames=[];cores=[];hands=[]
                for n,index in enumerate(ORDER):
                    im=normalized[index];back=6<=n<12;box=structural_box(im);anchor=[64,box[1]+6]
                    grips=[[128-x,y+box[1]-structural_box(normalized[ORDER[n-6]])[1]] for x,y in frames[n-6]['grips']] if back else track_pair(neutral,im,points)
                    # Shared class head: no new face/hood identity per weapon family.
                    at=(48,round(anchor[1])-16);im.paste((0,0,0,0),(at[0],at[1],at[0]+32,at[1]+32),headmask);im.alpha_composite(heads[int(back)],at)
                    core=im.copy();hand=Image.new('RGBA',(128,128))
                    if not back:
                        for x,y in grips:
                            x,y=round(x),round(y);region=(x-2,y-2,x+3,y+3);hand.paste(im.crop(region),region[:2]);core.paste((0,0,0,0),region)
                    assert Image.alpha_composite(core,hand).tobytes()==im.tobytes()
                    images.append(im);cores.append(core);hands.append(hand);frames.append({'direction':'back' if back else 'front' if n<12 else 'idle','phase':n%6,'grips':grips,'grips_occluded':back,'head_anchor':anchor,'bounds':list(structural_box(im))})
                stem=p['id']+'-'+family;files={}
                for kind,ims in [('body',cores),('hands',hands),('complete',images)]:
                    file=OUT/(stem+'-'+kind+'.png');atlas(ims).save(file);files[kind]=rel(file)
                variants[family]={**files,'frames':frames,'row_uniform_xy_scales':scales,'source':rel(source)};count+=1
                board=Image.new('RGB',(768,390),'#1e2831');draw=ImageDraw.Draw(board)
                for n,im in enumerate(images):
                    x=n%6*128;y=n//6*128;board.paste(im,(x,y),im)
                    if n<6 or n==12:
                        for px,py in frames[n]['grips']:draw.point((x+px,y+py),fill='#00ff00')
                board.save(PREVIEW/(stem+'-landmarks.png'))
            except (AssertionError,OSError,ValueError) as error:failures.append({'id':job['id'],'error':str(error)})
        if variants:profiles.append({**{k:p[k] for k in ['id','number','class_id','outfit','rank','target_height','selector']},'family_poses':variants})
    write(OUT/'manifest.json',{'review':'VERT18-C12','normal_game_enabled':False,'profiles':profiles,'prepared_families':count,'failures':failures})
    write(OUT/'grip-openings-front.json',{'review':'VERT18-C12','method':'manually inspected actual handle openings; both hands tracked as one rigid contact pair, with fixed angle/span through the gait','points':PALMS})
    print(json.dumps({'profiles':len(profiles),'prepared_families':count,'failures':failures}))
    if args.require_complete:assert count==36 and not failures

if __name__=='__main__':main()
