"""Repack the installed creature poses with six articulated support frames.

The idle view is the immutable source; repeated runs never deform prior frames.
This changes presentation only. Character equipment uses the modular Rust rig.
"""
import json,math,hashlib
from pathlib import Path
from PIL import Image,ImageDraw
from build_player_walk_review_04 import leg_mask

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/animations/walk-04'
PHASE=[-1.,-.15,.85,1.,.15,-.85,0.,0.]

def cycle(im,rig,foot):
    box=im.getchannel('A').point(lambda a:255 if a>=64 else 0).getbbox()
    bottom=box[3]-1;top=box[1];h=bottom-top;side=im.width
    hip=round(top+h*(.60 if rig not in ['quad','crawl','machine'] else .67))
    mask=leg_mask(im,hip,(foot[0],bottom))
    legs=Image.new('RGBA',im.size);legs.paste(im,(0,0),mask)
    fixed=im.copy();fixed.paste((0,0,0,0),(0,0,side,side),mask)
    frames=[]
    for f,wave in enumerate(PHASE):
        out=Image.new('RGBA',im.size)
        if rig in ['float','winged','roll','serpent']:
            if rig=='roll':
                # A wheel/rolling organism actually rotates; translating a
                # motionless wheel was the same defect as the frozen feet.
                out=im.rotate((f%6)*12,Image.Resampling.NEAREST,center=((box[0]+box[2])/2,(box[1]+box[3])/2))
            elif rig=='serpent':
                for y in range(side):
                    out.alpha_composite(im.crop((0,y,side,y+1)),(round(math.sin(f/6*math.tau+(y-top)*.10)*2.4),y))
            else:out.alpha_composite(im,(0,-round(abs(wave)*2)))
            frames.append(out);continue
        count=4 if rig in ['quad','crawl','machine'] else 2
        length=max(8,bottom-hip);knee=hip+length*.49
        for n in range(count):
            w=wave*(-1 if n%2 else 1);advance=w*length*.31;lift=max(0,w)*length*.20
            x0=round(n*side/count);x1=round((n+1)*side/count)
            for upper in [True,False]:
                if upper:y0,y1,dy0,dy1,dx0,dx1=hip-2,knee+2,0,-lift*.45,0,advance*.47
                else:y0,y1,dy0,dy1,dx0,dx1=knee-2,bottom+2,-lift*.45,-lift,advance*.47,advance
                segment=Image.new('RGBA',im.size)
                segment.alpha_composite(legs.crop((x0,max(0,round(y0)),x1,min(side,math.ceil(y1)))),(x0,max(0,round(y0))))
                sy=((y1+dy1)-(y0+dy0))/(y1-y0);sh=(dx1-dx0)/(y1-y0)
                # PIL inverse of x'=x+sh*(y-y0)+dx0; y'=sy*(y-y0)+y0+dy0.
                transformed=segment.transform(im.size,Image.Transform.AFFINE,
                    (1,-sh/sy,sh*(y0+dy0)/sy-dx0,0,1/sy,y0-(y0+dy0)/sy),Image.Resampling.NEAREST)
                out.alpha_composite(transformed)
        out.alpha_composite(fixed)
        frames.append(out)
    return frames+[im.copy()]

def main():
    manifest=json.loads((OUT/'manifest.json').read_text('utf-8'))
    baseline=json.loads((ROOT/'docs/reviews/player-walk-04/baseline-data.json').read_text('utf-8'))
    records={r['id']:r for r in baseline['records']};count=0
    for entry in manifest['entries']:
        if entry['id'].startswith(('CLASS-','ARMOR-')):continue
        record=records[entry['id']];path=ROOT/entry['file'];sheet=Image.open(path).convert('RGBA');side=entry['side']
        for row in range(sheet.height//side):
            idle=sheet.crop((8*side,row*side,9*side,(row+1)*side))
            for f,im in enumerate(cycle(idle,record['rig'],entry['foot'])):sheet.paste(im,(f*side,row*side))
        sheet.save(path,compress_level=6);entry['sha256']=hashlib.sha256(path.read_bytes()).hexdigest();count+=1
    manifest['review']='WALK02-FIX02';manifest['movement_frames']=6;manifest['frames_per_cell']=3
    (OUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf-8')
    print(f'{count} creature atlases rebuilt, six articulated support poses')

if __name__=='__main__':main()
