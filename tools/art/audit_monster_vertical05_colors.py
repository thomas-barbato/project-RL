"""Read-only color diagnostics for the retained cloth and masks of three cultists."""
import colorsys
import json
from PIL import Image
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-05/r01'

def stats(image,material):
    pixels=[]
    for c in image.get_flattened_data():
        if c[3]<224:continue
        h,s,v=colorsys.rgb_to_hsv(*[x/255 for x in c[:3]])
        keep=((h<.06 or h>.95) and s>.35 and v>.12) if material=='red_cloth' else (s<.25 and v>.48)
        if keep:pixels.append(c[:3])
    assert pixels,material
    return {'pixels':len(pixels),'median_rgb':[sorted(c[k] for c in pixels)[len(pixels)//2] for k in range(3)]}

def main():
    manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'));rows=[]
    for p in manifest['profiles']:
        moving=Image.open(ROOT/p['atlas']).convert('RGBA').crop((0,0,1280,256));front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        for material in ['red_cloth','light_mask_or_metal']:
            a=stats(moving,material);b=stats(front,material)
            rows.append({'model':p['number'],'material':material,'moving':a,'retained_front':b,'median_rgb_delta':[x-y for x,y in zip(a['median_rgb'],b['median_rgb'])]})
    report={'review':'MONSTER-VERTICAL05-r01','source_pngs_modified':False,'method':'Read-only material HSV bins; supplements native visual comparison','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL05-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
