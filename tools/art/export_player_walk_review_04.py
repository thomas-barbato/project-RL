"""Export a self-contained review. This never navigates a browser to file://."""
import base64,json,re
from pathlib import Path
from build_player_walk_review_04 import SITE,ART,DOC
DEST=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c')
data=json.loads((SITE/'catalogue/data04.json').read_text('utf-8'))
paths={r['image'] for r in data['records']}|{r['painted']['path'] for r in data['records'] if r.get('painted')}|{data['scene']['floor'],data['scene']['item']}
data['assets']=sorted(paths)
embedded={p:'data:image/png;base64,'+base64.b64encode((SITE/p).read_bytes()).decode('ascii') for p in paths}
wrapper=(SITE/'catalogue.html').read_text('utf-8')
old="const data=fetch('catalogue/data04.json').then(r=>{if(!r.ok)throw Error('Catalogue indisponible');return r.json();});"
assert old in wrapper
wrapper=wrapper.replace(old,'const data=Promise.resolve('+json.dumps(data,ensure_ascii=False,separators=(',',':')).replace('</','<\\/')+');const embedded='+json.dumps(embedded,separators=(',',':'))+';')
wrapper,n=re.subn(r'const response=await fetch\(m\.path\);.*?reader\.readAsDataURL\(blob\);\}\);','return embedded[m.path];',wrapper,flags=re.S);assert n==1
wrapper=wrapper.replace("connect-src 'self'","connect-src 'none'")
wrapper=wrapper.replace('href="armes.html"','href="player-walk-armed-02.html"').replace('<a href="index.html">Premier comparatif</a>','')
path=DEST/'player-walk-catalogue-04.html';path.write_text(wrapper,'utf-8')
(DEST/'player-walk-armed-02.html').write_bytes((SITE/'armes.html').read_bytes().replace(b'href="catalogue.html"',b'href="player-walk-catalogue-04.html"'))
print(json.dumps({'standalone':str(path),'bytes':path.stat().st_size,'entries':len(data['records']),'sandbox_preserved':True,'network_required':False}))
