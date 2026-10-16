"""Review-only registration/composition of existing imagegen source layers.

No painting or new illustration: polygons only select existing source pixels.
The old pixel-art atlas is used ONLY in the labelled before comparison.
"""
from pathlib import Path
from functools import lru_cache
import hashlib, json, math
from PIL import Image, ImageChops, ImageDraw, ImageFont

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
OLD = BASE / 'comic-catalogue-review-01'
OUT = BASE / 'comic-catalogue-review-02'
SIDE, FACTOR = 64, 16

# Coordinates measured on each original unarmed BD drawing, not the old atlas.
REG = {
 'BD-A007': {'class':'Bastion', 'old_class':1, 'body_index':9,
  'L':{'elbow':[399,464], 'hand':[343,686],
       'arm':[[0,400],[336,450],[480,475],[456,565],[424,623],[432,800],[0,800]],
       'glove':[[288,632],[391,628],[411,668],[418,711],[389,758],[278,759],[274,674]]},
  'R':{'elbow':[851,464], 'hand':[925,685],
       'arm':[[792,478],[926,452],[1254,400],[1254,810],[826,810],[829,626],[809,554]],
       'glove':[[855,629],[963,631],[985,675],[988,721],[966,759],[866,759],[839,709]]}},
 'BD-B003': {'class':'Technomancien', 'old_class':2, 'body_index':2,
  'L':{'elbow':[370,514], 'hand':[333,688],
       'arm':[[324,492],[429,530],[393,623],[390,716],[354,776],[284,757],[277,697],[293,592]],
       'glove':[[309,627],[370,644],[395,698],[363,779],[286,756],[278,701]]},
  'R':{'elbow':[878,514], 'hand':[919,690],
       'arm':[[824,534],[911,497],[956,585],[974,645],[975,714],[952,775],[876,775],[863,701],[857,619]],
       'glove':[[870,644],[944,628],[976,681],[975,723],[951,779],[875,779],[863,702]]}},
 'BD-A009': {'class':'Arcaniste', 'old_class':4, 'body_index':11,
  'L':{'elbow':[379,468], 'hand':[327,682],
       'arm':[[220,450],[379,460],[470,489],[440,629],[392,652],[390,790],[220,800]],
       'glove':[[311,625],[364,630],[379,699],[353,759],[288,739],[277,684]]},
  'R':{'elbow':[865,468], 'hand':[926,682],
       'arm':[[790,489],[875,450],[1040,450],[1050,800],[850,800],[823,658],[805,613]],
       'glove':[[887,631],[949,625],[980,681],[974,731],[944,760],[884,745],[873,692]]}}
}

# Per-model semantic grips are explicitly measured on original illustrations.
GEAR = {
 'BD-W001': {'grips':[[535,570],[1180,485]], 'extent':22, 'axis':'width', 'pose':'rifle'},
 'BD-W102': {'grips':[[267,794]], 'extent':8.5, 'axis':'width', 'pose':'pistol'},
 'BD-W069': {'grips':[[508,1123]], 'extent':9, 'axis':'height', 'pose':'knife'},
 'BD-EQ-S02': {'grips':[[530,715]], 'extent':18, 'axis':'height', 'pose':'shield'},
}
CASES = [
 ('BD-REPAIR02-01','Bastion — armure seule','BD-A007',None),
 ('BD-REPAIR02-02','Bastion — couteau céramique','BD-A007','BD-W069'),
 ('BD-REPAIR02-03','Bastion — Cogneur P-01','BD-A007','BD-W102'),
 ('BD-REPAIR02-04','Bastion — Brenek A-80','BD-A007','BD-W001'),
 ('BD-REPAIR02-05','Bastion — bouclier EQ-S02','BD-A007','BD-EQ-S02'),
 ('BD-REPAIR02-06','Technomancien — Cogneur P-01','BD-B003','BD-W102'),
 ('BD-REPAIR02-07','Arcaniste — Brenek A-80','BD-A009','BD-W001'),
]

def save(im, path):
 path.parent.mkdir(parents=True,exist_ok=True); im.save(path)

@lru_cache()
def source(ident):
 return Image.open(OLD/'sources'/f'{ident}.png').convert('RGBA')

def alpha_bbox(im):
 return im.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()

def select(im, polygon):
 mask=Image.new('L',im.size); ImageDraw.Draw(mask).polygon(polygon,fill=255)
 layer=im.copy(); layer.putalpha(ImageChops.multiply(im.getchannel('A'),mask))
 return layer

def arm_component(im,polygon,anchor):
 # A selection polygon can also touch a trouser edge. Keep only source pixels
 # connected to the measured hand: never transplant the unrelated clothing.
 layer=select(im,polygon); alpha=layer.getchannel('A'); px=alpha.load()
 seed=tuple(anchor)
 if not px[seed]:
  seed=min(((x,y) for y in range(anchor[1]-10,anchor[1]+11) for x in range(anchor[0]-10,anchor[0]+11) if px[x,y]),key=lambda p:(p[0]-anchor[0])**2+(p[1]-anchor[1])**2)
 seen={seed}; pending=[seed]
 while pending:
  x,y=pending.pop()
  for dy in (-1,0,1):
   for dx in (-1,0,1):
    q=(x+dx,y+dy)
    if 0<=q[0]<im.width and 0<=q[1]<im.height and q not in seen and px[q]:
     seen.add(q); pending.append(q)
 mask=Image.new('L',im.size); mp=mask.load()
 for q in seen:mp[q]=255
 layer.putalpha(ImageChops.multiply(alpha,mask)); return layer

def without(im,layer):
 result=im.copy(); result.putalpha(ImageChops.subtract(im.getchannel('A'),layer.getchannel('A')))
 return result

def transformed(im, pivot, target, scale, angle=0, stretch_x=1):
 c,s=math.cos(angle),math.sin(angle)
 a,b,d,e=c/(scale*stretch_x),s/(scale*stretch_x),-s/scale,c/scale
 coeff=(a/FACTOR,b/FACTOR,pivot[0]-a*target[0]-b*target[1],
        d/FACTOR,e/FACTOR,pivot[1]-d*target[0]-e*target[1])
 return im.transform((SIDE*FACTOR,SIDE*FACTOR),Image.Transform.AFFINE,coeff,resample=Image.Resampling.BICUBIC)

def registration(body):
 im=source(body); box=alpha_bbox(im); scale=32/(box[3]-box[1])
 # Match the reviewed body's width/height, without copying its pixel contour,
 # shadows, hands or texture. This registration belongs only to the pilot.
 atlas=Image.open(ROOT/'assets/sprites/cyberpunk/agent-rigs/bodies-32.png').convert('RGBA')
 i=REG[body]['old_class']-1; oldbox=alpha_bbox(atlas.crop((i*32,0,(i+1)*32,32)))
 width=32*(oldbox[2]-oldbox[0])/(oldbox[3]-oldbox[1])
 stretch_x=width/((box[2]-box[0])*scale)
 pivot=((box[0]+box[2])/2,box[3]); target=(32,48)
 def point(p):return [target[0]+(p[0]-pivot[0])*scale*stretch_x,target[1]+(p[1]-pivot[1])*scale]
 return scale,pivot,target,point,stretch_x

def native(im):
 # Rasterise the same high-resolution composition once. No old shadow stencil.
 small=im.resize((SIDE,SIDE),Image.Resampling.LANCZOS)
 alpha=small.getchannel('A').point(lambda a:255 if a>=128 else 0)
 # Palette conditioning keeps flat areas, without dithering or black overrides.
 small=small.convert('RGB').quantize(40,method=Image.Quantize.MEDIANCUT,dither=Image.Dither.NONE).convert('RGBA')
 small.putalpha(alpha); return small

def compose(body,weapon):
 im=source(body); reg=REG[body]; scale,pivot,origin,point,stretch_x=registration(body)
 fixed=im.copy(); rear=[]; gloves=[]; contacts=[]
 if weapon:
  spec=GEAR[weapon]; obj=source(weapon); box=alpha_bbox(obj)
  oscale=spec['extent']/(box[2]-box[0] if spec['axis']=='width' else box[3]-box[1])
  angle=0
  if spec['pose']=='rifle':
   main=[point(reg['L']['elbow'])[0]+.4,33.0]
   # Width is explicit; supporting hand follows the actual foregrip.
   grips=[main,[main[0]+(spec['grips'][1][0]-spec['grips'][0][0])*oscale,
                main[1]+(spec['grips'][1][1]-spec['grips'][0][1])*oscale]]
   for side,grip in zip(('L','R'),grips):
    part=arm_component(im,reg[side]['arm'],reg[side]['hand']); glove=select(part,reg[side]['glove'])
    fixed=without(fixed,part)
    p=reg[side]['elbow']; q=reg[side]['hand']; t=point(p)
    v=((q[0]-p[0])*stretch_x,q[1]-p[1]); u=(grip[0]-t[0],grip[1]-t[1])
    k=math.hypot(*u)/math.hypot(*v); rot=math.atan2(u[1],u[0])-math.atan2(v[1],v[0])
    rear.append(transformed(without(part,glove),p,t,k,rot,stretch_x))
    gloves.append(transformed(glove,p,t,k,rot,stretch_x))
    contacts.append({'side':side,'target':grip,'forearm_length_ratio':round(k/scale,3)})
   gear=transformed(obj,spec['grips'][0],grips[0],oscale)
  else:
   side='L' if spec['pose']=='shield' else 'R'; target=point(reg[side]['hand'])
   if spec['pose']=='pistol':
    hand=point(reg[side]['hand']); elbow=point(reg[side]['elbow'])
    angle=math.atan2(hand[1]-elbow[1],hand[0]-elbow[0])
   gear=transformed(obj,spec['grips'][0],target,oscale,angle)
   if spec['pose']!='shield':gloves.append(transformed(select(im,reg[side]['glove']),pivot,origin,scale,stretch_x=stretch_x))
   contacts.append({'side':side,'target':target})
 else:gear=None
 frame=transformed(fixed,pivot,origin,scale,stretch_x=stretch_x)
 for layer in rear:frame.alpha_composite(layer)
 if gear:frame.alpha_composite(gear)
 for layer in gloves:frame.alpha_composite(layer)
 meta={'body_source':body,'weapon_source':weapon,'body_scale':scale,'body_width_registration':stretch_x,'logical_body_height':32,
       'same_source_for_large_and_native':True,'old_pixel_layers_used':False,
       'draw_order':['registered_body','source_forearms','weapon','source_gloves'],
       'contacts':contacts,'weapon_extent':GEAR[weapon]['extent'] if weapon else None,
       'weapon_extent_axis':GEAR[weapon]['axis'] if weapon else None}
 return frame,native(frame),meta

def before(body,weapon):
 if weapon:stem=f'{weapon}-C{REG[body]["old_class"]}'
 else:stem=f'{body}-base'
 small=Image.open(OLD/'frames'/f'{stem}-native.png').convert('RGBA').crop((32,32,96,96))
 large=Image.open(OLD/'frames'/f'{stem}-normal.png').convert('RGBA').crop((128,128,384,384))
 return large,small

def scene(im):
 floor=Image.open(OLD/'floor-native.png').convert('RGBA').crop((0,0,64,64)); floor.alpha_composite(im)
 return floor

def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
 OUT.mkdir(parents=True,exist_ok=True)
 (OUT/'source-registration.json').write_text(json.dumps({'bodies':REG,'gear':GEAR},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
 records=[]
 font=lambda n:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',n)
 board=Image.new('RGB',(1050,1250),(15,23,32)); draw=ImageDraw.Draw(board)
 draw.text((24,16),'BD-REPAIR02 · remontage des dessins sources',font=font(25),fill='white')
 draw.text((24,51),'Avant / nouvel essai · mêmes modèles · aucun remplacement dans le jeu',font=font(16),fill=(170,195,212))
 for j,(ident,title,body,weapon) in enumerate(CASES):
  large,small,meta=compose(body,weapon); oldlarge,oldsmall=before(body,weapon)
  prefix=OUT/'frames'/ident
  for suffix,img in [('large',large),('native',small),('scene',scene(small)),('before-large',oldlarge),('before-native',oldsmall)]:
   save(img,Path(str(prefix)+f'-{suffix}.png'))
  entry={'review_id':ident,'title':title,'body':body,'weapon':weapon,'metadata':meta}
  records.append(entry)
  if j in (0,2,3,5,6):
   row=(0,2,3,5,6).index(j); y=88+row*226
   draw.text((24,y),f'{ident} · {title}',font=font(17),fill='white')
   for x,label,img in [(40,'Montage précédent',oldlarge),(340,'Nouvel essai',large)]:
    draw.text((x,y+30),label,font=font(14),fill=(170,195,212))
    preview=img.resize((320,320),Image.Resampling.LANCZOS).crop((48,64,280,260))
    board.paste(preview,(x,y+53),preview)
   draw.text((710,y+33),'32 px réels',font=font(14),fill=(170,195,212))
   board.paste(scene(small),(719,y+70))
   draw.text((840,y+33),'Zoom natif ×2',font=font(14),fill=(170,195,212))
   board.paste(scene(small).resize((128,128),Image.Resampling.NEAREST),(840,y+70))
 save(board,OUT/'before-after-board.png')
 save(Image.open(OLD/'floor-native.png').convert('RGBA').crop((0,0,32,32)),OUT/'floor-native32.png')
 active={str(p.relative_to(ROOT)):digest(p) for p in (ROOT/'assets/sprites/cyberpunk/agent-rigs').glob('*') if p.is_file()}
 report={'review_id':'BD-REPAIR02','approval':'pending;review_only','source_drawings_modified':0,
         'current_game_assets':active,'cases':records,
         'limitations':['Only three bodies and four equipment models are registered in this diagnostic pilot.',
                        'Source glove pose remains the unarmed pose; wrapping fingers needs its own visual review.']}
 (OUT/'review.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
 print(json.dumps({'cases':len(records),'output':str(OUT)},ensure_ascii=False))

if __name__=='__main__':main()
