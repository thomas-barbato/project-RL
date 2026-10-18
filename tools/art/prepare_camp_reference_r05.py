"""Prepare the selected four-room illustration and separate native props.

Imagegen creates the edited architecture/isolated furnishings. This script
only conditions their canvases, packs existing pixels and records geometry.
"""
import hashlib
import json
from collections import deque
from pathlib import Path
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r05'
OUT=ROOT/'assets/prototypes/handcrafted-camp-01/r05'
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def binary(im):
    im=im.convert('RGBA');im.putalpha(im.getchannel('A').point(lambda a:255 if a>=160 else 0));return im


def components(im):
    todo={(x,y) for y in range(im.height) for x in range(im.width) if im.getpixel((x,y))[3]}
    groups=[]
    while todo:
        first=todo.pop();pending=[first];group={first}
        while pending:
            x,y=pending.pop()
            for n in [(x-1,y),(x+1,y),(x,y-1),(x,y+1)]:
                if n in todo:todo.remove(n);pending.append(n);group.add(n)
        groups.append(group)
    return groups


def main():
    OUT.mkdir(parents=True,exist_ok=True)
    walls=Image.open(ROOT/'assets/prototypes/handcrafted-camp-01/r04/walls-native64.png').convert('RGBA')
    def wall(m):return walls.crop(((m%4)*64,(m//4)*64,(m%4+1)*64,(m//4+1)*64))
    ns=[wall(5).getpixel((x,0)) for x in range(64)]
    ew=[wall(10).getpixel((0,y)) for y in range(64)]
    # Real end caps join directly to the two wall edges. The old whole-frame
    # crop left disconnected crossbars; no crop from that frame is reused.
    horizontal=Image.new('RGBA',(64,64))
    horizontal.alpha_composite(wall(1).crop((0,36,64,48)),(0,4))
    horizontal.alpha_composite(wall(4).crop((0,16,64,28)),(0,48))
    vertical=Image.new('RGBA',(64,64))
    vertical.alpha_composite(wall(8).crop((36,0,48,64)),(4,0))
    vertical.alpha_composite(wall(2).crop((16,0,28,64)),(48,0))
    for t in range(4):
        for i in range(64):
            horizontal.putpixel((i,t),ns[i]);horizontal.putpixel((i,63-t),ns[i])
            vertical.putpixel((t,i),ew[i]);vertical.putpixel((63-t,i),ew[i])
    old=Image.open(ROOT/'assets/prototypes/handcrafted-camp-01/r04/doors-native64.png').convert('RGBA')
    doors=[]
    for i,piece in enumerate([horizontal,horizontal.copy(),vertical,vertical.copy()]):
        if i in [1,3]:
            panel=old.crop(((i%2)*64+16,(i//2)*64+16,(i%2)*64+48,(i//2)*64+48))
            piece.alpha_composite(panel,(16,16))
        doors.append(piece)
    for i,piece in enumerate(doors):
        groups=components(piece)
        assert len(groups)==(2 if i%2==0 else 1),(i,[len(g) for g in groups])
        for group in groups:
            assert any((y in (0,63) if i<2 else x in (0,63)) for x,y in group),'floating door component'
    atlas=Image.new('RGBA',(128,128))
    for i,piece in enumerate(doors):atlas.alpha_composite(piece,((i%2)*64,(i//2)*64))
    atlas.save(OUT/'doors-native64.png')
    architecture=Image.open(SOURCE/'architecture-source-r05.png').convert('RGBA').resize((1792,1152),Image.Resampling.LANCZOS)
    grid=[list('#'*28) for _ in range(18)]
    for y in range(7,10):
        for x in range(1,27):grid[y][x]='.'
    for low,high in [(4,11),(17,24)]:
        for y in [2,3,4,12,13,14]:
            for x in range(low,high):grid[y][x]='.'
    for x in [7,20]:
        for y in [5,6,10,11]:grid[y][x]='.'
    grid[8][0]=grid[8][27]='.'
    (OUT/'camp-layout.txt').write_text('\n'.join(''.join(r) for r in grid)+'\n',encoding='utf-8')
    floor=Image.new('RGBA',architecture.size);solid=Image.new('RGBA',architecture.size)
    for y,row in enumerate(grid):
        for x,t in enumerate(row):
            tile=architecture.crop((x*64,y*64,(x+1)*64,(y+1)*64))
            (solid if t=='#' else floor).alpha_composite(tile,(x*64,y*64))
    floor.save(OUT/'camp-floor-native.png',optimize=True)
    solid.save(OUT/'camp-structure-native.png',optimize=True)
    # The source atlas has unequal horizontal spacing. Explicit object crops
    # avoid cutting a large desk at a nominal quarter-cell boundary.
    sheet=binary(Image.open(SOURCE/'furniture-source-r05.png'))
    bounds=[(42,0,487,455),(543,0,1086,455),(1168,0,1341,455),(1497,0,1690,455),
            (111,455,332,887),(487,455,680,887),(786,455,1304,887),(1416,455,1703,887)]
    sizes=[(90,50),(90,64),(28,64),(34,72),(30,56),(26,40),(82,48),(42,34)]
    names=['Bureau de missions','Établi d’armurier','Râtelier','Lit médical','Armoire médicale','Chariot médical','Établi technique','Coffre']
    props=[]
    for i,(box,size,label) in enumerate(zip(bounds,sizes,names),7):
        item=sheet.crop(box);item=item.crop(item.getbbox()).resize(size,Image.Resampling.LANCZOS)
        alpha=binary(item).getchannel('A');item=item.convert('RGB').quantize(colors=32,dither=Image.Dither.NONE).convert('RGBA');item.putalpha(alpha)
        canvas=Image.new('RGBA',(128,128));canvas.alpha_composite(item,(64-size[0]//2,96-size[1]))
        p=OUT/f'DEC-CAMP-{i:02}-r05.png';canvas.save(p,optimize=True)
        props.append(dict(id=f'DEC-CAMP-{i:02}',label=label,file=p.relative_to(ROOT).as_posix(),native_size=list(size),sha256=sha(p)))
    actors=[
        dict(id='PLAYER',file='assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png',at=[12,8]),
        dict(id='NPC-01',file='assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-01-r03/NPC-01-r03-native64.png',at=[8,4]),
        dict(id='NPC-02',file='assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-02-r02/NPC-02-r02-native64.png',at=[21,4]),
        dict(id='NPC-03',file='assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-03-r02/NPC-03-r02-native64.png',at=[8,13]),
        dict(id='NPC-04',file='assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-04-r02/NPC-04-r02-native64.png',at=[21,13]),
    ]
    for a in actors:a.update(sha256=sha(ROOT/a['file']),foot_anchor=[64,96],scale=1)
    placements=[(7,7,2,2),(9,4,2,1),(11,5,2,1),(14,4,4,1),
                (8,19,2,2),(9,23,2,1),(11,17,2,1),(14,23,4,1),
                (10,4,13,1),(11,9,12,1),(12,9,13,1),
                (13,18,12,2),(14,23,13,1),(11,17,12,1)]
    instances=[]
    for model,x,y,w in placements:
        instances.append(dict(id=f'DEC-CAMP-{model:02}',file=props[model-7]['file'],at=[x,y],width=w,height=1))
    for x,y in [(7,3),(20,3),(20,13)]:
        instances.append(dict(id='DEC-CAMP-02',file='assets/prototypes/handcrafted-camp-01/r04/DEC-CAMP-02-r04.png',at=[x,y],width=1,height=1))
    blocked={(p['at'][0]+dx,p['at'][1]) for p in instances for dx in range(p['width'])}
    blocked|={tuple(a['at']) for a in actors[1:]}
    start=tuple(actors[0]['at']);seen={start};q=deque([start])
    while q:
        x,y=q.popleft()
        for n in [(x-1,y),(x+1,y),(x,y-1),(x,y+1)]:
            nx,ny=n
            if 0<=nx<28 and 0<=ny<18 and grid[ny][nx]!='#' and n not in blocked and n not in seen:seen.add(n);q.append(n)
    for a in actors[1:]:
        x,y=a['at'];assert any(n in seen for n in [(x-1,y),(x+1,y),(x,y-1),(x,y+1)]),a['id']
    for x,y in [(7,4),(20,4),(7,12),(20,12),(0,8),(27,8)]:assert (x,y) in seen,(x,y)
    data=dict(place='PLACE-CAMP-01',revision='r05',width=28,height=18,cell=64,
        floor='assets/prototypes/handcrafted-camp-01/r05/camp-floor-native.png',structure='assets/prototypes/handcrafted-camp-01/r05/camp-structure-native.png',
        actors=actors,props=instances,prop_sources=props,
        passages=[[7,5],[20,5],[7,11],[20,11]],
        exits=[[0,8],[27,8]],passage_type='Open architectural passages from the selected reference; no additional door bars painted over them.',
        movable_door_fixture='Existing two-room test uses the corrected r05 jamb atlas; separate from these open architectural passages.',
        art_approved=False,layout_approved=False,campaign_connected=False,
        reachable_floor_cells=len(seen),all_npcs_approachable=True,door_components_anchored=True,
        selected_reference='assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r05/camp-reference-user-r05.png')
    (OUT/'camp.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (REVIEW/'full-camp-r05.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(native_camp=[1792,1152],actors=5,props=len(instances),door_components_anchored=True,all_npcs_approachable=True)))


if __name__=='__main__':main()
