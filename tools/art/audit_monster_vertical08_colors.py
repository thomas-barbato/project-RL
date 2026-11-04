"""Read-only material diagnostics, separating visible front and back surfaces."""
import colorsys
import json
from pathlib import Path
from PIL import Image
from prepare_monster_vertical08_balance import classify

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-08/r01'

def stats(image,material,palette=None):
    pixels=[]
    for c in image.get_flattened_data():
        if c[3]<224:continue
        if palette and (kind:=classify(c)):
            r,g,b=palette['red_lut' if kind==1 else 'ochre_lut'][c[0]]
            c=(r,round(c[1]*g),round(c[2]*b),c[3])
        h,s,v=colorsys.rgb_to_hsv(*[x/255 for x in c[:3]])
        if material=='maroon_flesh':keep=(h<.07 or h>.94) and s>.35 and v>.12
        elif material=='warm_bone':keep=.06<h<.17 and .08<s<.5 and v>.38
        elif material=='orange_flesh':keep=.015<h<.11 and s>.55 and v>.20
        elif material=='blue_gray':keep=.52<h<.75 and s>.10 and v>.12
        elif material=='ochre_throat':keep=.085<h<.19 and s>.35 and v>.25
        else:raise ValueError(material)
        if keep:pixels.append(c[:3])
    if not pixels:return {'pixels':0,'median_rgb':None}
    return {'pixels':len(pixels),'median_rgb':[sorted(c[k] for c in pixels)[len(pixels)//2] for k in range(3)]}

def main():
    manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'));rows=[]
    for p in manifest['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA');front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        palette=json.loads((ROOT/p['material_palette']).read_text(encoding='utf-8')) if p.get('material_palette') else None
        materials={8:['maroon_flesh','warm_bone'],34:['orange_flesh'],36:['blue_gray','ochre_throat']}[p['number']]
        for direction,row in [('down',0),('up',1)]:
            moving=atlas.crop((0,row*128,1280,(row+1)*128))
            for material in materials:
                a=stats(moving,material,palette if row==0 else None);b=stats(front,material)
                rows.append({'model':p['number'],'direction':direction,'material':material,'moving_source':stats(moving,material),'moving_prepared':a,'retained_front':b,'median_rgb_delta':[x-y for x,y in zip(a['median_rgb'],b['median_rgb'])] if a['median_rgb'] and b['median_rgb'] else None})
    report={'review':'MONSTER-VERTICAL08-r01','source_pngs_modified':False,'method':'Read-only HSV material bins per direction; different visible material areas are not a lighting measurement; supplement to native visual review','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL08-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'direction':r['direction'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
