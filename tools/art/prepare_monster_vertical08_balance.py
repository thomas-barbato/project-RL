"""Prepare fixed front-only material gains; never alter source PNGs or poses."""
import colorsys
import hashlib
import json
import statistics
from PIL import Image

def classify(c):
    if c[3]==0:return 0
    h,s,v=colorsys.rgb_to_hsv(*[x/255 for x in c[:3]])
    if .52<h<.75 and s>.10 and v>.12:return 1
    if .085<h<.19 and s>.35 and v>.25:return 2
    return 0

def prepare(root,folder):
    path=folder/'manifest.json';m=json.loads(path.read_text(encoding='utf-8'))
    p=next(p for p in m['profiles'] if p['number']==36)
    moving=Image.open(root/p['atlas']).convert('RGBA').crop((0,0,1280,128));front=Image.open(root/p['idle_sprite']).convert('RGBA')
    luts=[];gains=[]
    for material in [1,2]:
        a=[c[:3] for c in moving.get_flattened_data() if c[3]>=224 and classify(c)==material]
        b=[c[:3] for c in front.get_flattened_data() if c[3]>=224 and classify(c)==material]
        gain=[statistics.median(c[k] for c in b)/statistics.median(c[k] for c in a) for k in range(3)]
        gains.append(gain);luts.append([[min(255,round(r*gain[0])),gain[1],gain[2]] for r in range(256)])
    rule={'review':'MONSTER-VERTICAL08-r01','model':36,'variant':0,'material':'blue_ochre_front','red_min':0,'other_channel_max_ratio':0,'red_lut':luts[0],'ochre_lut':luts[1],
          'method':'Fixed RGB material median gains across all eight front poses; rear, both retained idles, alpha and other materials unchanged; no PNG edits',
          'material_gains':gains,'source_atlas':p['atlas'],'source_sha256':hashlib.sha256((root/p['atlas']).read_bytes()).hexdigest(),'reference':p['idle_sprite'],'reference_sha256':hashlib.sha256((root/p['idle_sprite']).read_bytes()).hexdigest(),'normal_game_enabled':False}
    target=folder/'MON-036-front-material-palette.json';target.write_text(json.dumps(rule,indent=2)+'\n',encoding='utf-8')
    p['material_palette']=target.relative_to(root).as_posix();path.write_text(json.dumps(m,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
