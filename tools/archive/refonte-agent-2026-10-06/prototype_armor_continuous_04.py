"""Continuous wrist registration, identity at each layer boundary."""
from functools import lru_cache
import math
import ast
import diagnose_armor_seams_04 as diag
from PIL import Image, ImageDraw, ImageFont, ImageChops
art=diag.art
POLYS=next(ast.literal_eval(n.value) for n in ast.parse((art.BASE/'prototype_armor_seams_04.py').read_text('utf-8')).body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='POLYS' for t in n.targets))
def polys(ident,pose):
    if ident=='AR-08' and pose=='rifle':return [
        [[335,435],[472,435],[480,472],[495,480],[660,463],[672,588],[585,590],[505,590],[335,580]],POLYS[pose][1]]
    if ident=='AR-08' and pose=='pole':return [
        [[340,435],[474,435],[479,476],[565,502],[673,538],[688,644],[598,654],[431,644],[337,568]],POLYS[pose][1]]
    return POLYS[pose]

def segment_distance(p,a,b):
    ax,ay=a;bx,by=b;x,y=p;dx=bx-ax;dy=by-ay
    t=max(0,min(1,((x-ax)*dx+(y-ay)*dy)/max(1,dx*dx+dy*dy)))
    return math.hypot(x-ax-t*dx,y-ay-t*dy)

def inside(p,poly):
    x,y=p;found=False
    for a,b in zip(poly,poly[1:]+poly[:1]):
        if (a[1]>y)!=(b[1]>y) and x<(b[0]-a[0])*(y-a[1])/(b[1]-a[1])+a[0]:found=not found
    return found

@lru_cache(maxsize=100)
def warped(ident,pose,side):
    src=art.source(ident,pose);poly=polys(ident,pose)[side]
    hand=art.rig.JOINTS[pose][1][side][2];actual=art.joints(ident,pose)[side][2]
    delta=[actual[k]-hand[k] for k in (0,1)]
    def inverse(x,y):
        if not inside((x,y),poly):return x,y
        edge=min(segment_distance((x,y),a,b) for a,b in zip(poly,poly[1:]+poly[:1]))
        t=max(0,min(1,(230-math.hypot(x-hand[0],y-hand[1]))/165))
        t=t*t*(3-2*t)*min(1,edge/35)
        return x+delta[0]*t,y+delta[1]*t
    mesh=[];step=16;w,h=src.size
    for y in range(0,h,step):
        for x in range(0,w,step):
            right=min(w,x+step);bottom=min(h,y+step)
            q=[inverse(*p) for p in [(x,y),(x,bottom),(right,bottom),(right,y)]]
            mesh.append(((x,y,right,bottom),tuple(v for p in q for v in p)))
    return src.transform(src.size,Image.Transform.MESH,mesh,Image.Resampling.BICUBIC)

def continuous_body(ident,pose):
    ref=art.rig.normalize(art.source(ident,'independent'))
    src=art.rig.normalize(art.source(ident,pose))
    def target_mask(polys):return art.rig.mask((art.MS,art.MS),[[art.rig.master(p) for p in poly] for poly in polys])
    old=target_mask(polys(ident,'independent'));new=target_mask(polys(ident,pose))
    core=ref.copy();core.putalpha(ImageChops.multiply(ref.getchannel('A'),ImageChops.invert(old)))
    fill=src.copy();fill.putalpha(ImageChops.multiply(src.getchannel('A'),ImageChops.multiply(old,ImageChops.invert(new))))
    # Narrow arm masks preserve the complete fixed torso without recovery
    # pixels from differently drawn pose torsos.
    for side in (0,1):
        layer=art.rig.normalize(warped(ident,pose,side))
        layer.putalpha(ImageChops.multiply(layer.getchannel('A'),target_mask([polys(ident,pose)[side]])))
        core.alpha_composite(layer)
    return core

if __name__=='__main__':
    ids=['AR-N14','AR-08','AR-10','AR-11','AR-12']
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',20)
    board=Image.new('RGB',(2000,len(ids)*400),'#111c25');d=ImageDraw.Draw(board)
    for row,ident in enumerate(ids):
        for col,pose in enumerate(art.POSES):
            im=continuous_body(ident,pose)
            bg=Image.new('RGBA',im.size,'#afb9bd');bg.alpha_composite(im)
            view=bg.crop((15*art.F,17*art.F,49*art.F,43*art.F)).resize((470,360),Image.Resampling.LANCZOS)
            board.paste(view,(col*500+15,row*400+32))
            d.text((col*500+10,row*400+5),ident+' · '+pose,fill='white',font=font)
    board.save(diag.out/'continuous-prototype.png')
