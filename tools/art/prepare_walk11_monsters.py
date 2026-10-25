"""Prepare whole-monster repair references and uniformly resized small cultists."""
from pathlib import Path
import json,math,hashlib
from PIL import Image,ImageOps,ImageDraw
from pack_walk09_common import box

ROOT=Path(__file__).resolve().parents[2]
SRC=ROOT/'assets/source-art/player-walk-review-11'
DOC=ROOT/'docs/reviews/player-walk-11'
OUT=ROOT/'assets/animations/walk-11'
ART=ROOT/'artifacts/player-walk-review-10/full-audit/campaign'

def fit(im,height,side,foot):
    b=box(im);source=im.crop((b[0]-2,b[1]-2,b[2]+2,b[3]+2));scale=height/(b[3]-b[1]);best=None
    for _ in range(8):
        small=source.resize((max(1,round(source.width*scale)),max(1,round(source.height*scale))),Image.Resampling.LANCZOS);bb=box(small);h=bb[3]-bb[1]
        if best is None or abs(h-height)<best[0]:best=(abs(h-height),small,bb)
        if h==height:break
        scale*=height/h
    _,im,b=best
    result=Image.new('RGBA',(side,side));result.alpha_composite(im,(round(foot[0]-(b[0]+b[2])/2),foot[1]-b[3]));return result

def main():
    (OUT/'creatures').mkdir(parents=True,exist_ok=True);(OUT/'fronts').mkdir(exist_ok=True)
    refs=json.loads((ART/'references.json').read_text())['references']
    original=json.loads((ROOT/'assets/animations/walk-07/manifest.json').read_text())['entries']
    jobs=[];proposed=[];proof=[]
    for number in [14,15,22,24,115,124]:
        old=next(e for e in original if e['number']==number);ref=next(e for e in refs if e['number']==number and e['variant']==0)
        front=ImageOps.flip(Image.open(ART/ref['file']).convert('RGBA'));front.save(SRC/f'MON-{number:03}-front-native.png')
        detail=SRC/f'MON-{number:03}-front-reference.png';front.resize((768,768),Image.Resampling.NEAREST).save(detail)
        atlas=Image.open(ROOT/old['file']).convert('RGBA')
        if number in [22,24]:
            target=59;factor=target/(box(front)[3]-box(front)[1]);entry=dict(old)
            newfront=fit(front,target,64,old['front_foot']);fb=box(newfront)
            offset=math.floor(old['front_foot'][1]-32-(fb[1]+fb[3])/2+.5)
            for colored in [r for r in refs if r['number']==number]:
                image=ImageOps.flip(Image.open(ART/colored['file']).convert('RGBA'))
                fit(image,target,64,old['front_foot']).save(OUT/f"fronts/MON-{number:03}-{colored['variant']}.png")
            packed=Image.new('RGBA',(896,256));bounds=[]
            for row in range(2):
                for col in range(7):
                    im=atlas.crop((col*128,row*128,(col+1)*128,(row+1)*128));b=box(im)
                    new=fit(im,round((b[3]-b[1])*factor),128,[64,112]);bounds.append(box(new));packed.paste(new,(col*128,row*128))
            entry.update(file=f'assets/animations/walk-11/creatures/MON-{number:03}-0.png',front_bounds=list(fb),front_vertical_offset=offset,vertical_offset=offset,cycle_bounds=[min(b[0] for b in bounds),min(b[1] for b in bounds),max(b[2] for b in bounds),max(b[3] for b in bounds)])
            packed.save(ROOT/entry['file']);proposed.append(entry)
            proof.append(dict(number=number,method='uniform whole figure scale only; original accepted poses and materials',front_target_height=target,uniform_factor=factor))
            continue
        motion='Slight coherent cloth drift only, no feet or walking legs, no extra glow.' if number==124 else 'A compact alternating biped walk, one supporting foot and a narrowly passing bent knee, minimal bob, no lateral leg spreading.'
        for row,side in enumerate(['left','right']):
            guide=Image.new('RGBA',(1536,768))
            for i in range(8):
                col=i if i<7 else 0;im=atlas.crop((col*128,row*128,(col+1)*128,(row+1)*128)).resize((384,384),Image.Resampling.NEAREST);guide.paste(im,((i%4)*384,(i//4)*384))
            guidepath=SRC/f'MON-{number:03}-{side}-old-guide.png';guide.save(guidepath)
            prompt=f'''Use case: identity-preserve. Correct WHOLE FIGURE proportions for existing Project RL monster MON-{number:03}, {ref['name']}, one direction SCREEN {side.upper()}.
Image1 is the exact canonical FRONT design currently used in game. Its head, torso, thinness, limb length, materials, cloak, armor and original objects are authoritative. Image2 is the current directional gait to repair, used ONLY for its8-pose order; do NOT copy its inflated torso, enlarged head, fat arms or thickened cloak. Draw the same physical creature as image1, turned together into near-front three quarters {side.upper()}, never pure side/back. Preserve original outfit and carried objects with rigid straight handles; every head, torso and support points in the same direction. Do not change species, clothing, anatomy, appendages, colors or effects. No new muscles, armor or decorations. In three quarters its projected torso should be slightly narrower than the front, not inflated. {motion}
Output EXACTLY4 columns by2 rows,8 COMPLETE figures, all facing {side.upper()}. Row-major order: stable grounded idle (or floating idle for a spirit), movement0,1,2,3,4,5, duplicate idle. The first3 movement frames accompany one grid cell, the next3 the next cell. Keep physical size, head proportions and body volume constant across cells; generous transparent gaps, no cropped extremities, neighbor fragments, labels, shadows, background or floor. Paint detailed matte readable native64 game art with restrained outlines, matching image1. Transparent alpha.'''
            jobs.append(dict(id=f'MON-{number:03}-{side}',kind='monster',number=number,side=side,front_source=str(detail),input=str(guidepath),destination=str(SRC/f'MON-{number:03}-{side}-proportions-r01.png'),prompt=prompt))
    (DOC/'monster-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (DOC/'small-cultists-conditioning.json').write_text(json.dumps(proof,indent=2)+'\n')
    (OUT/'creatures/manifest.json').write_text(json.dumps(proposed,indent=2)+'\n')
    board=Image.new('RGB',(1200,900),'#303a40');d=ImageDraw.Draw(board)
    for i,number in enumerate([14,15,115,124]):
        d.text((i*300+5,5),f'MON-{number:03} canonical front',fill='white');im=Image.open(SRC/f'MON-{number:03}-front-reference.png');im.thumbnail((290,290));board.paste(im,(i*300,30),im)
        for row,side in enumerate(['left','right']):
            im=Image.open(SRC/f'MON-{number:03}-{side}-old-guide.png');im.thumbnail((290,290));board.paste(im,(i*300,340+row*280),im)
    board.save(DOC/'monster-inputs.png');print(json.dumps(dict(generation_jobs=len(jobs),small_cultists=proof)))

if __name__=='__main__':main()
