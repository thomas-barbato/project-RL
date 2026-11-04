"""Derive a rendering palette from the retained red demon, without editing PNGs."""
import hashlib
import json
import statistics
from bisect import bisect_left,bisect_right
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/monster-vertical-01/r01'
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-color-01/r01'
REV=ROOT/'docs/reviews/animation-polish-01'

def pigment(c):return c[3]>=224 and c[0]>35 and c[1]<c[0]*.48 and c[2]<c[0]*.48

def main():
    # Freeze the exact second lot accepted with a specific color exception on 03.
    acceptance=REV/'MONSTER-VERTICAL02-ACCEPTANCE-r01.json'
    if not acceptance.exists():
        data=json.loads((ROOT/'assets/source-art/animation-polish-01/monster-vertical-02/r01/manifest.json').read_text(encoding='utf-8'))
        assets=[]
        for p in data['profiles']:
            for name in [p['atlas'],p['idle_sprite'],*p['sources']]:
                assets.append({'file':name,'sha256':hashlib.sha256((ROOT/name).read_bytes()).hexdigest()})
        record={'review':'MONSTER-VERTICAL02-r01','date':'2026-11-02','status':'accepted','player_quote':"je valide :) parcontre je note que 03 - démon ailé change de couleur quand il marche (il devient rouge plus clair) .",
                'models':[5,156,165],'variant':0,'directions':['down','up'],'step_ms':400,'cycle_ms':800,'normal_game_enabled':False,
                'exception':{'model':144,'part':'moving red material palette','preserve':'poses, silhouette, timing and retained frontal'},'assets':assets}
        acceptance.write_text(json.dumps(record,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    for a in json.loads(acceptance.read_text(encoding='utf-8'))['assets']:
        assert hashlib.sha256((ROOT/a['file']).read_bytes()).hexdigest()==a['sha256']
    OUT.mkdir(parents=True,exist_ok=True)
    atlas=Image.open(SOURCE/'MON-144-atlas.png').convert('RGBA'); front=Image.open(SOURCE/'MON-144-front.png').convert('RGBA')
    source=[c for c in atlas.crop((0,0,1280,256)).get_flattened_data() if pigment(c)]
    target=[c for c in front.get_flattened_data() if pigment(c)]
    sr=sorted(c[0] for c in source); tr=sorted(c[0] for c in target); rows=[]
    for red in range(256):
        q=(bisect_left(sr,red)+bisect_right(sr,red))/(2*len(sr)); rr=tr[round(q*(len(tr)-1))]
        s=[c for c in source if abs(c[0]-red)<=10]; t=[c for c in target if abs(c[0]-rr)<=10]
        if not s:s=[min(source,key=lambda c:abs(c[0]-red))]
        if not t:t=[min(target,key=lambda c:abs(c[0]-rr))]
        gains=[statistics.median(c[k] for c in t)/max(1,statistics.median(c[k] for c in s)) for k in [1,2]]
        rows.append([rr,*[round(min(1,max(.2,g)),6) for g in gains]])
    rule={'review':'MONSTER-VERTICAL-COLOR01-r01','model':144,'variant':0,'source_atlas':(SOURCE/'MON-144-atlas.png').relative_to(ROOT).as_posix(),
          'source_sha256':hashlib.sha256((SOURCE/'MON-144-atlas.png').read_bytes()).hexdigest(),
          'reference':(SOURCE/'MON-144-front.png').relative_to(ROOT).as_posix(),'reference_sha256':hashlib.sha256((SOURCE/'MON-144-front.png').read_bytes()).hexdigest(),
          'native_cell_px':64,'protected_idle_column':8,'moving_columns':8,'red_min':35,'other_channel_max_ratio':.48,
          'red_lut':rows,'method':'source red-value CDF to retained frontal; conditional green/blue ratios; shared rule for all moving poses',
          'png_files_modified':False,'normal_game_enabled':False}
    path=OUT/'red-material-palette.json'; path.write_text(json.dumps(rule,indent=2)+'\n',encoding='utf-8')
    p=next(p for p in json.loads((SOURCE/'manifest.json').read_text(encoding='utf-8'))['profiles'] if p['number']==144)
    p={**p,'material_palette':path.relative_to(ROOT).as_posix(),'comparison':'previous_review_colors'}
    manifest={'review':'MONSTER-VERTICAL-COLOR01-r01','normal_game_enabled':False,'native_cell_px':64,'step_ms':400,'cycle_ms':800,'profiles':[p]}
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    def quant(values):return [[sorted(c[k] for c in values)[round(q*(len(values)-1))] for k in range(3)] for q in [.1,.5,.9]]
    corrected=[(int(rows[c[0]][0]),round(c[1]*rows[c[0]][1]),round(c[2]*rows[c[0]][2]),c[3]) for c in source]
    print(json.dumps({'source_quantiles':quant(source),'reference_quantiles':quant(target),'corrected_quantiles':quant(corrected),'accepted_lot02_assets':12,'source_pngs_untouched':True}))

if __name__=='__main__':main()
