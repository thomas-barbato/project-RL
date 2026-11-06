"""Bootstrap a distinct large city proposal. Never overwrite an edited plan."""
from pathlib import Path
from collections import deque
import json
import math

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'assets/maps/north-city-01/city-plan09.json'
N = 112

def cells(rect):
    l,t,r,b=rect
    return {(x,y) for y in range(t,b+1) for x in range(l,r+1)}

def path(points):
    result=[]
    for (x,y),(tx,ty) in zip(points,points[1:]):
        dx,dy=tx-x,ty-y
        count=max(abs(dx),abs(dy))*2 or 1
        for i in range(count+1):
            p=(round(x+dx*i/count),round(y+dy*i/count))
            if result and p[0]!=result[-1][0] and p[1]!=result[-1][1]:
                result.append((p[0],result[-1][1]))
            if not result or result[-1]!=p: result.append(p)
    return result

def band(points,width):
    result=set()
    # A continuous street is rasterized at tile centres. Its centreline is
    # cardinal-connected even where the illustration will have curved curbs.
    for x,y in path(points):
        for dy in range(-width,width+1):
            for dx in range(-width,width+1):
                if math.hypot(dx,dy)<=width/2+.15 and 0<x+dx<N-1 and 0<y+dy<N-1:
                    result.add((x+dx,y+dy))
    return result

def reach(blocked,start):
    found={start}; q=deque([start])
    while q:
        x,y=q.popleft()
        for p in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)]:
            if 0<p[0]<N-1 and 0<p[1]<N-1 and p not in blocked and p not in found:
                found.add(p);q.append(p)
    return found

def connector(start,targets,blocked):
    q=deque([start]); prev={start:None}; end=None
    while q:
        p=q.popleft()
        if p in targets: end=p;break
        x,y=p
        for n in [(x+1,y),(x,y+1),(x-1,y),(x,y-1)]:
            if 0<n[0]<N-1 and 0<n[1]<N-1 and n not in blocked and n not in prev:
                prev[n]=p;q.append(n)
    if end is None:return []
    out=[]
    while end is not None:out.append(end);end=prev[end]
    return list(reversed(out))

def main():
    if OUT.exists(): raise SystemExit('Refusing to overwrite the editable city-plan09.json')
    roads=[]
    def road(id,name,width,pts,kind='street'):
        roads.append(dict(id=id,name=name,width=width,points=pts,kind=kind))
    road('R01','Avenue vers la caserne',4,[[41,110],[41,102],[45,96],[46,85],[51,77],[54,70],[55,61],[60,52],[60,42],[66,34],[66,26],[76,21],[82,13],[93,9],[99,4]],'avenue')
    road('R02','Rue des faubourgs',3,[[3,99],[23,99],[41,102],[62,100],[83,102],[109,95]])
    road('R03','Rue de la vieille enceinte',2,[[3,110],[3,81],[4,62],[3,44],[4,27],[5,4]])
    road('R04','Rue du commissariat',3,[[4,29],[15,29],[28,29],[40,30],[52,29],[66,26]])
    road('R05','Rue de l’école et des services',3,[[4,57],[20,57],[31,54],[43,51],[60,52],[73,49],[90,48],[109,46]])
    road('R06','Rue des halles',3,[[8,82],[23,83],[36,80],[51,77],[69,79],[86,77],[109,78]])
    road('R07','Boulevard des ateliers',3,[[109,7],[105,22],[102,37],[103,52],[108,64],[104,83],[109,108]])
    road('R08','Accès des pompiers',2,[[66,36],[73,44],[86,45],[102,40]])
    road('R09','Rue des cours',2,[[44,4],[43,20],[45,33],[48,41],[43,51]])
    road('R10','Passages du quartier ancien',2,[[7,64],[21,63],[34,66],[33,73],[36,80],[36,98]])
    road('R11','Promenade du jardin',1,[[65,63],[74,64],[83,61],[94,57],[103,52]],'path')
    road('R12','Rue du poste militaire',2,[[79,22],[90,25],[104,22]])
    road('R13','Rue des jardins de fond de cour',2,[[32,88],[47,89],[65,87],[84,85],[104,83]])
    road('R14','Allée du parc',1,[[71,68],[76,68],[83,74]],'path')
    streets=set().union(*(band(r['points'],r['width']) for r in roads))
    pavement=set().union(*(band(r['points'],r['width']+2) for r in roads))
    green=[dict(id=i,name=n,rect=r,kind=k) for i,n,r,k in [
        ('G01','Jardin public', [68,61,85,75],'park'),
        ('G02','Arbres de la cour d’école',[8,52,24,55],'school'),
        ('G03','Jardins des maisons anciennes',[13,84,30,89],'gardens'),
        ('G04','Lisière du domaine militaire',[83,3,95,7],'greenbelt'),
        ('G05','Friche des ateliers',[90,29,99,36],'wasteland'),
        ('G06','Jardins de fond de cour',[52,82,67,85],'gardens')]]
    plazas=[dict(id=i,name=n,rect=r) for i,n,r in [
        ('P01','Place du marché',[48,64,61,72]),
        ('P02','Parvis du commissariat',[25,27,37,29]),
        ('P03','Cour des pompiers',[75,37,89,43]),
        ('P04','Cour de l’école',[12,43,23,50])]]
    campuses=[dict(id=i,name=n,rect=r) for i,n,r in [
        ('CP-POL','Commissariat',[17,10,37,26]),
        ('CP-ECO','École',[8,34,25,51]),
        ('CP-FEU','Caserne de pompiers',[72,27,89,43]),
        ('CP-MIL','Poste militaire',[83,10,98,22])]]
    events=[dict(id=i,name=n,at=p,kind=k) for i,n,p,k in [
        ('EV01','Évacuation civile interrompue',[52,68],'civilian'),
        ('EV02','Barricade et militaires tombés',[92,23],'military'),
        ('EV03','Petit monstre mort dans le jardin',[79,70],'creature'),
        ('EV04','Traces de secours près de l’école',[17,49],'civilian'),
        ('EV05','Pompiers et matériel abandonné',[82,40],'civilian'),
        ('EV06','Sang sur le parvis',[29,28],'military'),
        ('EV07','Ruelle éventrée',[24,88],'creature')]]
    green_cells=set().union(*(cells(g['rect']) for g in green))
    plaza_cells=set().union(*(cells(g['rect']) for g in plazas))
    campus_cells=set().union(*(cells(g['rect']) for g in campuses))
    event_cells=set().union(*(cells([e['at'][0]-1,e['at'][1]-1,e['at'][0]+1,e['at'][1]+1]) for e in events))
    buildings=[]; occupied=set(); accesses=set()
    def add(id,name,kind,rects,door=None,accessible=False):
        footprint=set().union(*(cells(r) for r in rects))
        if footprint&occupied:raise ValueError('Building overlap: '+id)
        if footprint&streets:raise ValueError('Road overlap: '+id+' '+str(sorted(footprint&streets)[:4]))
        if door is None:
            options={p for x,y in footprint for p in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)] if p not in footprint and 0<p[0]<N-1 and 0<p[1]<N-1}
            options-=occupied
            candidates=sorted(options,key=lambda p:min(abs(p[0]-s[0])+abs(p[1]-s[1]) for s in streets))[:8]
            paths=[connector(p,streets,occupied|footprint) for p in candidates]
            paths=[p for p in paths if p]
            if not paths:raise ValueError('No entrance: '+id)
            access=min(paths,key=len);door=access[0]
        else:
            door=tuple(door);access=connector(door,streets,occupied|footprint)
        if not access:raise ValueError('No access route '+id)
        occupied.update(footprint);accesses.update(access)
        buildings.append(dict(id=id,name=name,kind=kind,rects=rects,door=list(door),accessible=accessible,access_path=[list(p) for p in access],art_status='individual-artwork-to-produce-at-native64'))
    add('B01','Immeuble éventré','apartment',[[12,91,20,96]],accessible=True)
    add('B02','Logements abandonnés','house',[[77,91,85,97]],accessible=True)
    add('B03','Immeuble du quartier ancien','apartment',[[12,68,20,75]],accessible=True)
    add('B04','Îlot effondré','ruin',[[93,64,102,69]])
    add('B05','Commerce et logement','shop',[[46,55,52,59]],accessible=True)
    add('B06','Caserne de pompiers','fire',[[74,29,87,35]],door=[80,36],accessible=True)
    add('B07','École','school',[[8,34,18,40],[21,34,25,46]],door=[17,41],accessible=True)
    add('B08','Commissariat','police',[[17,10,37,15],[17,16,22,20],[32,16,37,20],[24,21,35,26]],door=[29,27],accessible=True)
    add('B09','Poste militaire','military',[[86,15,96,21]],door=[91,22],accessible=True)
    add('B10','Maisons incendiées','ruin',[[54,7,61,10],[64,7,71,10]])
    protected=pavement|green_cells|plaza_cells|campus_cells|event_cells
    # Fixed bootstrap packing is only an initial authoring aid. IDs and every
    # individual placement are persisted; a later edit never reruns this.
    candidates=[]
    for y in range(4,105,2):
        for x in range(7,104,2):
            w=5+(x//7+y//9)%3;h=4+(x//11+y//8)%3
            rect=[x,y,x+w-1,y+h-1]
            body=cells(rect)
            if body&(protected|occupied):continue
            distance=min(abs(x+w//2-sx)+abs(y+h//2-sy) for sx,sy in streets)
            if distance<=11:candidates.append((distance,(x*17+y*31)%43,x,y,w,h))
    for _,__,x,y,w,h in sorted(candidates):
        if len(buildings)>=86:break
        rect=[x,y,x+w-1,y+h-1];body=cells(rect)
        if body&(protected|occupied|accesses):continue
        n=len(buildings)-9
        kind='house' if y>82 or x<28 else 'apartment'
        if 47<y<83 and 30<x<72:kind='shop'
        if x>87 and 35<y<94:kind='workshop'
        name={'house':'Maison','apartment':'Immeuble','shop':'Commerce','workshop':'Atelier'}[kind]
        add('H'+str(n).zfill(2),name+' '+str(n),kind,[rect],accessible=n%4==0)
    trees=[]
    for g in green:
        l,t,r,b=g['rect']
        for y in range(t+1,b,3):
            for x in range(l+1,r,3):
                p=(x,y)
                if p not in occupied|streets|accesses|event_cells:
                    trees.append(dict(id='T'+str(len(trees)+1).zfill(3),at=[x,y],area=g['id']))
    for r in roads:
        if r['kind']=='path':continue
        for i,(x,y) in enumerate(path(r['points'])):
            if i%7:continue
            for dx,dy in [(0,3),(3,0)]:
                p=(x+dx,y+dy)
                if 1<p[0]<110 and 1<p[1]<110 and p not in occupied|streets|accesses|event_cells and p in pavement:
                    if all(abs(p[0]-t['at'][0])+abs(p[1]-t['at'][1])>2 for t in trees):
                        trees.append(dict(id='T'+str(len(trees)+1).zfill(3),at=list(p),area='street'))
    blocked=occupied|{tuple(t['at']) for t in trees}
    reached=reach(blocked,(41,109))
    assert (99,4) in reached
    assert all(tuple(b['door']) in reached for b in buildings)
    result=dict(review='CITY-NORTH01-PLAN09-r01',status='layout-proposal-awaiting-prevalidation',width=N,height=N,native_cell=64,native_world=[N*64,N*64],entrance=[41,109],street_links=[dict(at=[18,1],arrival=[41,108]),dict(at=[18,2],arrival=[41,109])],future_exit=dict(at=[99,4],target='core:military_barracks'),roads=roads,green_spaces=green,plazas=plazas,campuses=campuses,buildings=buildings,trees=trees,aftermath=events,police=dict(building='B08',upper_floor='assets/maps/north-city-01/police_upper.json',lower_floors=[dict(variants=['assets/maps/north-city-01/police_basement_layout%02d.json'%n for n in [1,2,3]]),dict(variants=['assets/maps/north-city-01/police_deep_layout%02d.json'%n for n in [1,2,3]])],choice='at-game-creation-and-persisted',stair_anchors=[[31,23],[34,23]]),inspiration=[dict(city='Strasbourg',url='https://www.visitstrasbourg.fr/wp-content/uploads/2020/09/OTSR_Plan-guide_2021.pdf'),dict(city='Paris',url='https://www.paris.fr/pages/plans-donnees-et-documents-d-urbanisme-3708')],layout_policy='Fixed authored city; independent of the seeded police basement variants. Bootstrap refuses to overwrite this file.',checks=dict(all_building_entrances_reachable=True,exit_reachable=True,building_count=len(buildings),enterable_candidates=sum(b['accessible'] for b in buildings),tree_count=len(trees),building_cells=len(occupied),street_cells=len(streets)),gameplay_integrated=False)
    OUT.write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(result['checks'],ensure_ascii=False))

if __name__=='__main__':main()
