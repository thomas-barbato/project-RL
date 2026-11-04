"""Measure candidate material colors against retained fronts; never edit pixels."""
import colorsys
import json
from functools import lru_cache
from pathlib import Path
from PIL import Image
from prepare_monster_vertical03_palette import skin

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-03/r01'

@lru_cache(maxsize=65536)
def hsv(rgb):return colorsys.rgb_to_hsv(*[v/255 for v in rgb])

def matches(rgb,material):
    h,s,v=hsv(rgb)
    if material=='reddish_brown_skin':return (h<.075 or h>.97) and s>.28 and v>.13
    if material=='gold':return .06<h<.17 and s>.30 and v>.15
    if material=='warm_ivory':return .06<h<.17 and .08<s<.55 and v>.20
    if material=='burgundy_wings':return (h<.06 or h>.97) and s>.40 and v>.13
    raise ValueError(material)

def stats(image,material,palette=None):
    pixels=[]
    for c in image.get_flattened_data():
        if c[3]<224:continue
        rgb=c[:3]
        if palette and skin(rgb):
            r,g,b=palette['red_lut'][rgb[0]];rgb=(r,round(rgb[1]*g),round(rgb[2]*b))
        if matches(rgb,material):pixels.append(rgb)
    assert pixels,material
    rgb=[[sorted(c[k] for c in pixels)[round(q*(len(pixels)-1))] for k in range(3)] for q in [.1,.5,.9]]
    values=sorted(max(c) for c in pixels)
    return {'pixels':len(pixels),'rgb_q10_q50_q90':rgb,'value_q10_q50_q90':[values[round(q*(len(values)-1))] for q in [.1,.5,.9]]}

def main():
    manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8')); rows=[]
    groups={39:['reddish_brown_skin','gold'],146:['gold'],151:['warm_ivory','burgundy_wings']}
    for p in manifest['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA').crop((0,0,1280,256))
        front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        palette=json.loads((ROOT/p['material_palette']).read_text(encoding='utf-8')) if p.get('material_palette') else None
        for material in groups[p['number']]:
            raw=stats(atlas,material); source=stats(atlas,material,palette); reference=stats(front,material)
            rows.append({'model':p['number'],'material':material,'moving_source':raw,'moving_prepared':source,'retained_front':reference,
                         'median_rgb_delta':[a-b for a,b in zip(source['rgb_q10_q50_q90'][1],reference['rgb_q10_q50_q90'][1])]})
    report={'review':'MONSTER-VERTICAL03-r01','source_pngs_modified':False,'method':'Material HSV bins; nearest-rank RGB quantiles; source-art diagnostic only, native comparison remains required','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL03-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
