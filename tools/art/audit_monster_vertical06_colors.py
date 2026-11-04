"""Read-only material diagnostics for two gun carriers and a pale possessed."""
import colorsys
import json
from pathlib import Path
from PIL import Image
from prepare_monster_vertical03_palette import skin

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-06/r01'

def stats(image,material,palette=None):
    pixels=[]
    for c in image.get_flattened_data():
        if c[3]<224:continue
        rgb=c[:3]
        if palette and skin(rgb):
            r,g,b=palette['red_lut'][rgb[0]];rgb=(r,round(rgb[1]*g),round(rgb[2]*b))
        h,s,v=colorsys.rgb_to_hsv(*[x/255 for x in rgb])
        if material=='red_cloth':keep=(h<.06 or h>.95) and s>.35 and v>.12
        elif material=='warm_skin':keep=.04<h<.17 and .10<s<.60 and v>.20
        elif material=='dark_green_cloth':keep=.20<h<.56 and s>.08 and .08<v<.65
        else:raise ValueError(material)
        if keep:pixels.append(rgb)
    assert pixels,material
    return {'pixels':len(pixels),'median_rgb':[sorted(c[k] for c in pixels)[len(pixels)//2] for k in range(3)]}

def main():
    manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'));rows=[]
    for p in manifest['profiles']:
        moving=Image.open(ROOT/p['atlas']).convert('RGBA').crop((0,0,1280,256));front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        palette=json.loads((ROOT/p['material_palette']).read_text(encoding='utf-8')) if p.get('material_palette') else None
        for material in (['warm_skin','dark_green_cloth'] if p['number']==25 else ['red_cloth']):
            raw=stats(moving,material);a=stats(moving,material,palette);b=stats(front,material)
            rows.append({'model':p['number'],'material':material,'moving_source':raw,'moving_prepared':a,'retained_front':b,'median_rgb_delta':[x-y for x,y in zip(a['median_rgb'],b['median_rgb'])]})
    report={'review':'MONSTER-VERTICAL06-r01','source_pngs_modified':False,'method':'Read-only material HSV bins; supplement to native visual review','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL06-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
