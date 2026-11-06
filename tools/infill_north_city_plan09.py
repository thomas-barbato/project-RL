"""One-time addition of attached city frontages, preserving existing plan items."""
from pathlib import Path
import json
from prepare_north_city_plan09 import cells,band,connector,reach

ROOT=Path(__file__).resolve().parents[1]
FILE=ROOT/'assets/maps/north-city-01/city-plan09.json'

def main():
    p=json.loads(FILE.read_text(encoding='utf-8'))
    if p.get('frontage_infill_applied'):raise SystemExit('Refusing to repeat the frontage addition')
    original=json.loads(json.dumps(p))
    streets=set().union(*(band(r['points'],r['width']) for r in p['roads']))
    protected=set().union(*(band(r['points'],r['width']+2) for r in p['roads']))
    for group in ['green_spaces','plazas','campuses']:
        for item in p[group]:protected|=cells(item['rect'])
    for t in p['trees']:protected.add(tuple(t['at']))
    for e in p['aftermath']:
        x,y=e['at'];protected|=cells([x-1,y-1,x+1,y+1])
    used=set().union(*(cells(r) for b in p['buildings'] for r in b['rects']))
    accesses={tuple(c) for b in p['buildings'] for c in b['access_path']}
    added=0
    while added<34:
        candidates=[]
        for old in p['buildings']:
            if old['id'].startswith('B'):continue
            for l,t,r,b in old['rects']:
                for w,h in [(4,5),(5,4),(4,4),(3,5)]:
                    for x,y in [(l-w,t),(r+1,t),(l-w,b-h+1),(r+1,b-h+1),(l,t-h),(l,b+1)]:
                        if x<7 or y<4 or x+w>108 or y+h>108:continue
                        rect=[x,y,x+w-1,y+h-1];body=cells(rect)
                        if body&(protected|used|accesses):continue
                        edge={n for a,z in body for n in [(a+1,z),(a-1,z),(a,z+1),(a,z-1)] if n not in body and 0<n[0]<111 and 0<n[1]<111}
                        edge-=used|protected
                        options=sorted(edge,key=lambda a:min(abs(a[0]-sx)+abs(a[1]-sy) for sx,sy in streets))[:2]
                        routes=[connector(a,streets,used|body) for a in options]
                        routes=[r for r in routes if r and len(r)<12]
                        if routes:candidates.append((len(min(routes,key=len)),rect,min(routes,key=len)))
        if not candidates:break
        _,rect,route=min(candidates,key=lambda v:(v[0],v[1][1],v[1][0]))
        n=len(p['buildings'])-9
        x,y=rect[:2]
        kind='house' if y>82 or x<28 else 'apartment'
        if 47<y<83 and 30<x<72:kind='shop'
        if x>87 and 35<y<94:kind='workshop'
        name={'house':'Maison mitoyenne','apartment':'Immeuble mitoyen','shop':'Boutique','workshop':'Petit atelier'}[kind]
        p['buildings'].append(dict(id='H'+str(n).zfill(2),name=name+' '+str(n),kind=kind,rects=[rect],door=list(route[0]),accessible=False,access_path=[list(v) for v in route],art_status='individual-artwork-to-produce-at-native64',frontage='attached'))
        used|=cells(rect);accesses.update(route);added+=1
    assert p['buildings'][:len(original['buildings'])]==original['buildings']
    blocked=used|{tuple(t['at']) for t in p['trees']}; reached=reach(blocked,(41,109))
    assert (99,4) in reached and all(tuple(b['door']) in reached for b in p['buildings'])
    p['frontage_infill_applied']=True
    p['checks'].update(building_count=len(p['buildings']),building_cells=len(used),attached_frontages_added=added)
    FILE.write_text(json.dumps(p,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print('Added',added,'attached frontages;',len(p['buildings']),'buildings total; previous items preserved.')

if __name__=='__main__':main()
