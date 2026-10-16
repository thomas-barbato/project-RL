"""Mechanical direct64px export and recolour metrics, no raster repainting."""
import hashlib
import json
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-46'
source=OUT/'possede-source.png'
im=Image.open(source).convert('RGBA')
assert im.getchannel('A').getextrema()==(0,255)
box=im.getchannel('A').point(lambda a:255 if a>32 else 0).getbbox()
crop=im.crop(box)
# Fix the native occupied slot to the approved body's42x62px envelope.
# A sub-percent change of HD framing must not round into a one-pixel actor shift.
reference=json.loads((ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-41/drawing-pack.json').read_text(encoding='utf-8'))
rb=next(p['bounds'] for p in reference['packing'] if p['kind']=='possede')
rw,rh=rb[2]-rb[0],rb[3]-rb[1]
scale=min(62/rw,62/rh)
w,h=round(rw*scale),round(rh*scale)
tile=Image.new('RGBA',(64,64))
tile.paste(crop.resize((w,h),Image.Resampling.LANCZOS),((64-w)//2,63-h))
old=Image.open(ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-41/possede-64.png').convert('RGBA')
a=[v>128 for v in old.getchannel('A').get_flattened_data()]
b=[v>128 for v in tile.getchannel('A').get_flattened_data()]
iou=sum(x and y for x,y in zip(a,b))/sum(x or y for x,y in zip(a,b))
# Purple needs substantial blue relative to red. Red eye/horn accents are
# allowed and must not be classified as violet by red/green contrast alone.
violet=sum(alpha>128 and blue>green+12 and red>green+6 and blue>red*.82
    for red,green,blue,alpha in tile.crop((0,16,64,64)).get_flattened_data())
assert iou>.97 and violet==0,(iou,violet)
tile.save(OUT/'possede-64.png')
(OUT/'drawing-pack.json').write_text(json.dumps({
    'tool':'Built-in imagegen','variant':'TR64-BODY04','source_size':list(im.size),'bounds':box,
    'native_size':[64,64],'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
    'postprocessing':'Alpha-bound crop and direct resize into the existing42x62px occupied slot on64px canvas; no repainting.',
    'fixed_native_occupied_slot':[w,h],
    'no_32px_intermediate':True,'native_silhouette_iou':round(iou,5),'violet_pixels_in_body':violet,
    'palette':'Warm flesh on upper and lower body; ivory bones and red eyes retained.'
},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'variant':'TR64-BODY04','iou':iou,'violet_pixels':violet}))
