"""Match each direction's organic materials to its actual installed reference."""
import hashlib
import json
from pathlib import Path
from PIL import Image
from prepare_monster_vertical28_color import material,channel_lookup,quant

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-11/r01'
REVIEW='MONSTER-VERTICAL11-r01'
MATERIAL_NAME='earth_organic_materials'
MATERIAL_KINDS=[(1,'red_creases'),(2,'warm_flesh_and_scales')]


def main():
    path=OUT/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'));audit=[]
    for p in m['profiles']:
        atlas=Image.open(ROOT/p['atlas']).convert('RGBA');luts=[];references=[]
        for row,direction in enumerate(['down','up']):
            reference=ROOT/p['idle_sprite'] if direction=='down' else OUT/f'MON-{p["number"]:03}-current-back-guide.png'
            target=Image.open(reference).convert('RGBA');references.append({'direction':direction,'file':reference.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(reference.read_bytes()).hexdigest()})
            source=atlas.crop((0,row*128,1280,(row+1)*128))
            for kind,label in MATERIAL_KINDS:
                a=[c for c in source.get_flattened_data() if c[3]>=224 and material(c)==kind]
                b=[c for c in target.get_flattened_data() if c[3]>=224 and material(c)==kind]
                assert a and b,(p['number'],direction,label)
                lut=channel_lookup(a,b);luts.append(lut)
                prepared=[[lut[c[k]][k] for k in range(3)] for c in a]
                before,expected,after=quant(a),quant(b),quant(prepared)
                assert max(abs(x-y) for aa,bb in zip(after,expected) for x,y in zip(aa,bb))<=2
                audit.append({'model':p['number'],'direction':direction,'material':label,'source_pixels':len(a),'reference_pixels':len(b),'raw':before,'retained_reference':expected,'prepared':after})
        rule={'review':REVIEW,'model':p['number'],'variant':0,'material':MATERIAL_NAME,'material_luts':luts,
              'source_atlas':p['atlas'],'source_sha256':hashlib.sha256((ROOT/p['atlas']).read_bytes()).hexdigest(),
              'references':references,'normal_game_enabled':False,
              'method':'Fixed monotone material RGB curves per direction shared by all eight poses; actual installed front/rear references; preserve alpha, shape, dark outlines and both exact frontal idles; no per-phase lighting gain; no PNG edit'}
        palette=OUT/f'MON-{p["number"]:03}-material-palette.json';palette.write_text(json.dumps(rule,indent=2)+'\n',encoding='utf-8')
        p['material_palette']=palette.relative_to(ROOT).as_posix()
    path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    (ROOT/'docs/reviews/animation-polish-01'/f'{REVIEW.removesuffix("-r01")}-MATERIALS.json').write_text(json.dumps({'review':REVIEW,'png_files_modified':False,'world_gain_unchanged':1.48,'normal_game_enabled':False,'materials':audit},indent=2)+'\n',encoding='utf-8')


if __name__=='__main__':main()
