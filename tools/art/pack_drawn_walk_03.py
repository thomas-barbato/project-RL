"""Use drawn walking legs; never shear a standing pose into a stride.

The real torso, sleeves, weapon, fingers and helmet remain independent.
"""
import json
from pathlib import Path
from PIL import Image,ImageFilter
from build_player_walk_review_04 import components
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/animations/walk-04/drawn'

def lower_owned(im,waist):
    lower=im.copy();lower.paste((0,0,0,0),(0,0,im.width,waist))
    comps=components(lower,48)
    # Hands must not become a second pair of forearms below the belt. Keep
    # only the pelvis/leg silhouette which reaches the soles, not loose cuffs.
    if not comps:raise ValueError('empty drawn lower body')
    main=max(comps,key=lambda c:c['box'][3]*im.width+c['area']/im.width)
    keep=[c for c in comps if c is main or (c['area']>main['area']*.20 and c['box'][3]>main['box'][3]-im.height*.12)]
    mask=bytearray(im.width*im.height)
    for c in keep:
        for n in c['points']:mask[n]=255
    mask=Image.frombytes('L',im.size,bytes(mask)).filter(ImageFilter.MaxFilter(3))
    out=Image.new('RGBA',im.size);out.paste(lower,(0,0),mask)
    return out

def make(index,path,original=False):
    atlas=Image.open(ROOT/f'assets/animations/walk-04/CLASS-{index:02}.png').convert('RGBA')
    sheet=Image.open(path).convert('RGBA');cw,ch=sheet.width//6,sheet.height//(8 if original else 2)
    entries=[]
    for facing,row,di in [(-1,7 if original else 0,0),(1,1 if original else 1,2)]:
        target=atlas.crop((1024,di*128,1152,(di+1)*128));box=target.getchannel('A').point(lambda a:255 if a>=64 else 0).getbbox()
        target_h=box[3]-box[1];waist=round(box[1]+target_h*.45)
        frames=[];boxes=[]
        for f in range(6):
            im=sheet.crop((f*cw,row*ch,(f+1)*cw,(row+1)*ch));b=im.getchannel('A').point(lambda a:255 if a>=48 else 0).getbbox()
            frames.append(im);boxes.append(b)
        scale=target_h/max(b[3]-b[1] for b in boxes)
        pack=Image.new('RGBA',(768,128))
        for f,(im,b) in enumerate(zip(frames,boxes)):
            cropped=im.crop(b);native=cropped.resize((round(cropped.width*scale),round(cropped.height*scale)),Image.Resampling.LANCZOS)
            # Head/torso centre, not the foot-span, fixes the body to its cell.
            na=native.getchannel('A');head=na.crop((0,0,native.width,max(3,round(native.height*.20))))
            pts=[(i%head.width,a) for i,a in enumerate(head.tobytes()) if a>=64]
            headx=sum(x*a for x,a in pts)/sum(a for _,a in pts)
            ta=target.getchannel('A').crop((0,box[1],128,box[1]+round(target_h*.20)))
            pts=[(i%128,a) for i,a in enumerate(ta.tobytes()) if a>=64];targetx=sum(x*a for x,a in pts)/sum(a for _,a in pts)
            positioned=Image.new('RGBA',(128,128));positioned.alpha_composite(native,(round(targetx-headx),box[3]-native.height))
            low=lower_owned(positioned,waist-2);pack.alpha_composite(low,(f*128,0))
        name=f'CLASS-{index:02}-{facing}.png';pack.save(OUT/name)
        entries.append({'id':f'CLASS-{index:02}','facing':facing,'file':f'assets/animations/walk-04/drawn/{name}','waist':waist,'frames':6,'source':str(path.relative_to(ROOT))})
    return entries

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    entries=[]
    entries+=make(3,ROOT/'artifacts/player-walk-review-01/bastion-walk-native64.png',True)
    entries+=make(10,ROOT/'assets/source-art/player-walk-review-04/CLASS-10-drawn-walk03-r02-source.png')
    (OUT/'manifest.json').write_text(json.dumps(entries,ensure_ascii=False,indent=2)+'\n','utf8')
    print(f'{len(entries)} actual drawn six-pose cycles; preview correction, no leg deformation')
if __name__=='__main__':main()
