"""Offline native premultiplied Pillow sampler, checked against scalar C08."""
import math
from PIL import Image
_planes={}

def posed(source,recipe):
    key=(id(source),recipe['flip'])
    if key not in _planes:
        im=source.transpose(Image.Transpose.FLIP_LEFT_RIGHT) if recipe['flip'] else source
        pixels=list(im.get_flattened_data());planes=[]
        for c in range(4):
            plane=Image.new('F',im.size);plane.putdata([p[3] if c==3 else p[c]*p[3]/255 for p in pixels]);planes.append(plane)
        _planes[key]=(source,planes,im.getchannel('A').getbbox())
    _,planes,box=_planes[key]
    out=Image.new('RGBA',(256,256))
    if not box:return out
    sn,cs=math.sin(recipe['angle']),math.cos(recipe['angle']);scale=recipe['scale']
    px,py=recipe['pivot'];tx,ty=recipe['target'];a,b=cs/scale,sn/scale
    corners=[(tx+(cs*(x-px)-sn*(y-py))*scale,ty+(sn*(x-px)+cs*(y-py))*scale) for x in [box[0]-1,box[2]+1] for y in [box[1]-1,box[3]+1]]
    x0=max(0,math.floor(min(p[0] for p in corners)));y0=max(0,math.floor(min(p[1] for p in corners)))
    x1=min(256,math.ceil(max(p[0] for p in corners))+1);y1=min(256,math.ceil(max(p[1] for p in corners))+1)
    if x1<=x0 or y1<=y0:return out
    coeff=(a,b,px-a*tx-b*ty+a*x0+b*y0,-b,a,py+b*tx-a*ty-b*x0+a*y0)
    values=[p.transform((x1-x0,y1-y0),Image.Transform.AFFINE,coeff,Image.Resampling.BILINEAR).get_flattened_data() for p in planes]
    pixels=[]
    for red,green,blue,alpha in zip(*values):
        pixels.append(tuple(min(255,int(c*255/alpha+.5)) for c in [red,green,blue])+(min(255,int(alpha+.5)),) if alpha>0 else (0,0,0,0))
    patch=Image.new('RGBA',(x1-x0,y1-y0));patch.putdata(pixels);out.paste(patch,(x0,y0));return out
