"""Read-only skin, flesh and bone diagnostics alongside native comparison."""
import json
from pathlib import Path
from PIL import Image
from audit_monster_vertical09_colors import stats

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-10/r01'

def main():
    m=json.loads((OUT/'manifest.json').read_text(encoding='utf-8'));rows=[]
    for p in m['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA');front=Image.open(ROOT/p['idle_sprite']).convert('RGBA')
        for direction,row in [('down',0),('up',1)]:
            moving=atlas.crop((0,row*128,1280,(row+1)*128))
            for material in ['warm_flesh','red_creases']:
                a,b=stats(moving,material),stats(front,material)
                rows.append({'model':p['number'],'direction':direction,'material':material,'moving':a,'retained_front':b,'median_rgb_delta':[x-y for x,y in zip(a['median_rgb'],b['median_rgb'])] if a['median_rgb'] and b['median_rgb'] else None})
    report={'review':'MONSTER-VERTICAL10-r01','source_pngs_modified':False,'method':'Read-only HSV bins by direction; visible material areas and orientations differ; supplement to native review, not a lighting measurement','materials':rows}
    (ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL10-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'direction':r['direction'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
