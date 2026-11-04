"""Read-only material comparison for the possessed batch."""
import json
from PIL import Image
import audit_monster_vertical03_colors as base

OUT=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-04/r01'

def main():
    manifest=json.loads((OUT/'manifest.json').read_text(encoding='utf-8')); rows=[]
    for p in manifest['profiles']:
        atlas=Image.open(base.ROOT/p['atlas']).convert('RGBA').crop((0,0,1280,256))
        front=Image.open(base.ROOT/p['idle_sprite']).convert('RGBA')
        palette=json.loads((base.ROOT/p['material_palette']).read_text(encoding='utf-8')) if p.get('material_palette') else None
        for material in ['warm_ivory','gold']:
            raw=base.stats(atlas,material); source=base.stats(atlas,material,palette); reference=base.stats(front,material)
            rows.append({'model':p['number'],'material':material,'moving_source':raw,'moving_prepared':source,'retained_front':reference,
                         'median_rgb_delta':[a-b for a,b in zip(source['rgb_q10_q50_q90'][1],reference['rgb_q10_q50_q90'][1])]})
    report={'review':'MONSTER-VERTICAL04-r01','source_pngs_modified':False,'method':'Read-only HSV material bins; numerical diagnostics supplement native review','materials':rows}
    (base.ROOT/'docs/reviews/animation-polish-01/MONSTER-VERTICAL04-COLORS.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(json.dumps([{'model':r['model'],'material':r['material'],'median_rgb_delta':r['median_rgb_delta']} for r in rows]))

if __name__=='__main__':main()
