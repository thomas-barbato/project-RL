"""Read-only material diagnostics for low pale bodies and blue-gray covering."""
import colorsys
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-09/r01'

def stats(image,material):
    pixels=[]
    for c in image.get_flattened_data():
        if c[3]<224:continue
        h,s,v=colorsys.rgb_to_hsv(*[x/255 for x in c[:3]])
        if material=='warm_flesh':keep=.03<h<.18 and .12<s<.65 and v>.18
        elif material=='red_creases':keep=(h<.06 or h>.94) and s>.45 and v>.10
        elif material=='blue_gray':keep=.44<h<.72 and s>.10 and v>.10
        elif material=='orange_rim':keep=.015<h<.10 and s>.4 and v>.15
        else:raise ValueError(material)
        if keep:pixels.append(c[:3])
    if not pixels:return {'pixels':0,'median_rgb':None}
    return {'pixels':len(pixels),'median_rgb':[sorted(c[k] for c in pixels)[len(pixels)//2] for k in range(3)]}

def main():
    m=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'));rows=[]
    for p in m['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA');front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        materials=['warm_flesh','red_creases'] if p['number']!=38 else ['warm_flesh','blue_gray','orange_rim']
        for direction,row in [('down',0),('up',1)]:
            moving=atlas.crop((0,row*128,1280,(row+1)*128))
            for material in materials:
                a,b=stats(moving,material),stats(front,material)
                rows.append({'model':p['number'],'direction':direction,'material':material,'moving':a,'retained_front':b,'median_rgb_delta':[x-y for x,y in zip(a['median_rgb'],b['median_rgb'])] if a['median_rgb'] and b['median_rgb'] else None})
    report={'review':'MONSTER-VERTICAL09-r01','source_pngs_modified':False,'method':'Read-only HSV material bins by direction; changing visible material areas is not a lighting measurement; supplement to native visual review','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL09-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'direction':r['direction'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
