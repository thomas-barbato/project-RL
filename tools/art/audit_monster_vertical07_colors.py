"""Read-only palette diagnostics for three compact bodies, by material and view."""
import colorsys
import json
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-07/r01'

def stats(image,material):
    pixels=[]
    for c in image.get_flattened_data():
        if c[3]<224:continue
        h,s,v=colorsys.rgb_to_hsv(*[x/255 for x in c[:3]])
        if material=='ochre_cloth':keep=.07<h<.16 and s>.55 and v>.18
        elif material=='olive_and_belly':keep=.07<h<.20 and s>.10 and v>.18
        elif material=='warm_flesh':keep=.025<h<.14 and .15<s<.70 and v>.23
        elif material=='dark_back_or_trousers':keep=s<.35 and .08<v<.46
        else:raise ValueError(material)
        if keep:pixels.append(c[:3])
    if not pixels:return {'pixels':0,'median_rgb':None}
    return {'pixels':len(pixels),'median_rgb':[sorted(c[k] for c in pixels)[len(pixels)//2] for k in range(3)]}

def main():
    manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'));rows=[]
    for p in manifest['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA');front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        materials={26:['ochre_cloth','warm_flesh','dark_back_or_trousers'],28:['olive_and_belly'],32:['warm_flesh','dark_back_or_trousers']}[p['number']]
        for direction,row in [('down',0),('up',1)]:
            moving=atlas.crop((0,row*128,1280,(row+1)*128))
            for material in materials:
                a=stats(moving,material);b=stats(front,material)
                rows.append({'model':p['number'],'direction':direction,'material':material,'moving':a,'retained_front':b,'median_rgb_delta':[x-y for x,y in zip(a['median_rgb'],b['median_rgb'])] if a['median_rgb'] and b['median_rgb'] else None})
    report={'review':'MONSTER-VERTICAL07-r01','source_pngs_modified':False,'method':'Read-only HSV material bins by view; changes of visible material area are not a lighting measurement; supplement to native visual review','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL07-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'direction':r['direction'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
