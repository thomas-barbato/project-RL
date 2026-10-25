"""Build the review page, keeping an opaque sandbox and a restrictive CSP.

This prepares static source only. Sites source helper owns publication.
"""
import hashlib,json,re
from pathlib import Path
from collections import Counter
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'artifacts/player-walk-review-04'
DOC=ROOT/'docs/reviews/player-walk-04'
SITE=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c/sites/player-walk/dist')

def main():
    baseline=json.loads((DOC/'baseline-data.json').read_text('utf-8'))
    report=json.loads((DOC/'packaging.json').read_text('utf-8'));ready={e['id']:e for e in report['entries']}
    assets=set(baseline['assets']);records=baseline['records']
    for record in records:
        record['reviewId']='WALK04-'+record['id']
        if record['id'] in ready:
            entry=ready[record['id']].copy();content=(ART/entry['atlas']).read_bytes();key=hashlib.sha256(content).hexdigest()[:20];path='catalogue/assets/'+key+'.png'
            destination=SITE/path
            if not destination.exists():destination.write_bytes(content)
            assets.add(path);entry['path']=path;record['painted']=entry
            record['size']=[entry['size'],entry['size']];record['foot']=entry['foot'];record['bounds']=entry['bounds']
    data={'review':'WALK04-r01','tile':64,'defaultDuration':400,'durations':[120,200,400],
        'records':records,'scene':baseline['scene'],'assets':sorted(assets),'readyCount':len(ready),'counts':dict(Counter(r['category'] for r in records))}
    (SITE/'catalogue/data04.json').write_text(json.dumps(data,ensure_ascii=False,separators=(',',':')),'utf-8')
    template=(ROOT/'tools/art/player_walk_review_03.html').read_text('utf-8')
    template=template.replace('WALK03-r01','WALK04-r01').replace('Un mouvement pour chaque silhouette.','Deux orientations de trois quarts.')
    template=template.replace('<label id="headField">Casque<select id="helmet"><option value="0">Tête de la tenue</option><option value="1">Casque 1</option><option value="2">Casque 2</option><option value="3">Casque 3</option><option value="4">Casque 4</option><option value="5">Casque 5</option><option value="6">Casque 6</option></select></label>','')
    template=template.replace('<label>Animation<select id="rig"></select></label>','<label>Silhouette<span id="rigLabel"></span></label>')
    template=template.replace('Les côtés sont un essai de déformation 2D des dessins actuels. Les profils redessinés du premier essai du Bastion restent une référence distincte.','Les nouvelles vues latérales sont dessinées de trois quarts. Les bras et les armes restent entiers pendant la marche.')
    template=template.replace('<script>/*__RIG__*/</script>','').replace('/*__APP__*/',(ROOT/'tools/art/player_walk_review_04.js').read_text('utf-8'))
    template=template.replace('.tile img{','.tile canvas{').replace('<strong>Essai de mouvement</strong>','<strong>Nouveau candidat · trois quarts</strong>')
    (SITE/'catalogue/app04.html').write_text(template,'utf-8')
    wrapper=(SITE/'catalogue.html').read_text('utf-8')
    wrapper=wrapper.replace('walk03-','walk04-').replace("fetch('catalogue/data.json')","fetch('catalogue/data04.json')")
    wrapper=wrapper.replace('Catalogue complet des propositions de déplacement','Dessins de trois quarts · revue WALK04')
    wrapper,n=re.subn(r'frame\.srcdoc=.*?;\s*</script>',lambda _: 'frame.srcdoc='+json.dumps(template,ensure_ascii=False).replace('</','<\\/')+';\n</script>',wrapper,flags=re.S)
    assert n==1
    assert 'sandbox="allow-scripts"' in wrapper and 'allow-same-origin' not in wrapper
    assert "connect-src 'none'" in template and "img-src data:" in template
    (SITE/'catalogue.html').write_text(wrapper,'utf-8')
    original=ROOT/'../..' # The first attachment remains byte-for-byte unchanged.
    assert hashlib.sha256((SITE/'index.html').read_bytes()).hexdigest()=='7728275ce9517088bee406689a44934cc2b47d46e210657d96f763c1b9a8a7cc'
    (DOC/'coverage.json').write_text(json.dumps({'review':data['review'],'runtime_integration':False,'ready':len(ready),'total':len(records),'pending':[r['id'] for r in records if r['id'] not in ready],'counts':data['counts'],'equipment_presets_per_outfit':6,'class_presets':['unarmed','pistol101','hammer97','rifle2','axe66','dual101'],'not_covered':['All 113 individual weapon models in directional painted form','All six interchangeable headwear combined with every outfit and weapon'],'errors':report['errors']},ensure_ascii=False,indent=2),'utf-8')
    print(json.dumps({'ready':len(ready),'total':len(records),'assets':len(assets),'page_bytes':len(wrapper.encode()),'original_preserved':True}))

if __name__=='__main__':main()
