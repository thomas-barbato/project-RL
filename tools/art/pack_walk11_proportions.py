"""Condition complete painted figures to their own canonical front proportions."""
from pathlib import Path
import hashlib,json
from PIL import Image,ImageDraw
from build_player_walk_matrix_06 import grid_figures
from pack_walk09_common import protected_parts,box
from prepare_walk09_jobs import whole
from audit_walk09_body_volume import measure
from build_walk_rigs_02 import rig

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-11'
OUT=ROOT/'assets/animations/walk-11/player'

def components_of(points):
    todo=set(points);groups=[]
    while todo:
        seed=todo.pop();group=[seed];stack=[seed]
        while stack:
            x,y=stack.pop()
            for dx,dy in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]:
                q=x+dx,y+dy
                if q in todo:todo.remove(q);group.append(q);stack.append(q)
        groups.append(group)
    return groups

def grip_ownership(im,index,facing,frame,legacy_contact_seams=True):
    """Keep complete hands on their arms, with protected thighs/knees below."""
    _,meta=rig(im,index);b=box(im);h=b[3]-b[1];cx=meta['center']
    top=b[1];sy=top+(h-1)*.30;py=top+(h-1)*.56
    spread=8. if index<20 else 7.;palm_spread=15. if index>=15 else 13.
    lower=(round(cx-h*.25),round(py+6),round(cx+h*.25)+1,b[3])
    upper=(round(cx-h*.115),round(py-1),round(cx+h*.115)+1,lower[1])
    core=im.copy();arms=[];fists=[];palms=[];pivots=[]
    for side in range(2):
        sign=1 if side==0 else -1;sx=cx-sign*spread;px=cx-sign*palm_spread;edge=0 if side==0 else 127
        mask=Image.new('L',im.size);md=ImageDraw.Draw(mask)
        md.polygon([(edge,round(sy)),(round(sx+sign*3),round(sy)),(round(px+sign*6),round(py+5)),(edge,round(py+5))],fill=255)
        md.rectangle((lower[0],lower[1],lower[2]-1,lower[3]-1),fill=0)
        md.rectangle((upper[0],upper[1],upper[2]-1,upper[3]-1),fill=0)
        seam=(53,67) if index==2 and facing==-1 and frame==3 and side==0 else ((54,68) if index==4 and facing==-1 and side==0 else None)
        if not legacy_contact_seams:seam=None
        if seam:
            # This bent glove touches the thigh in the source. The seam keeps
            # its neighbouring thigh plate on the body, rather than the hand.
            md.rectangle((*seam,127,127),fill=0)
        # Cloth fragments detached from the actual arm belong to the body.
        alpha=im.getchannel('A');rawmask=mask.copy();todo={(x,y) for y in range(128) for x in range(128) if mask.getpixel((x,y)) and alpha.getpixel((x,y))>=96};components=[]
        while todo:
            seed=todo.pop();points=[seed];stack=[seed]
            while stack:
                x,y=stack.pop()
                for dx,dy in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]:
                    q=x+dx,y+dy
                    if q in todo:todo.remove(q);points.append(q);stack.append(q)
            components.append(points)
        assert components,(index,side)
        main=set(max(components,key=len));bottom=max(y for x,y in main)
        points=[(x,y,alpha.getpixel((x,y))) for x,y in main if y>=bottom-3]
        weight=sum(v for x,y,v in points);hx=sum(x*v for x,y,v in points)/weight;hy=sum(y*v for x,y,v in points)/weight
        # Complete the painted fist beyond the old straight polygon edge.
        # The narrow central thigh guard still owns the leg underneath.
        stack=[q for q in main if q[1]>=bottom-4]
        while stack:
            x,y=stack.pop()
            for dx,dy in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]:
                q=xx,yy=x+dx,y+dy
                if q in main or not (0<=xx<128 and 0<=yy<128):continue
                if abs(xx-hx)>3.5 or yy<hy-3 or yy>hy+4:continue
                if upper[0]<=xx<upper[2] and yy>=upper[1]:continue
                if seam and xx>=seam[0] and yy>=seam[1]:continue
                if alpha.getpixel(q)>=96:main.add(q);stack.append(q)
        mask=Image.new('L',im.size)
        for x,y in main:
            mask.putpixel((x,y),255)
            for dx,dy in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]:
                q=xx,yy=x+dx,y+dy
                if 0<=xx<128 and 0<=yy<128 and not (seam and xx>=seam[0] and yy>=seam[1]) and 0<alpha.getpixel(q)<96 and (rawmask.getpixel(q) or (abs(xx-hx)<=3.5 and abs(yy-hy)<=3.5)):
                    mask.putpixel(q,255)
        arm=Image.new('RGBA',im.size);arm.paste(im,(0,0),mask);core.paste((0,0,0,0),(0,0,128,128),mask)
        ab=box(arm);a=arm.getchannel('A');points=[(x,y,a.getpixel((x,y))) for y in range(max(ab[1],ab[3]-4),ab[3]) for x in range(ab[0],ab[2]) if a.getpixel((x,y))>=128]
        weight=sum(v for x,y,v in points);px=sum(x*v for x,y,v in points)/weight;yy=sum(y*v for x,y,v in points)/weight
        palms.append([round(px,2),round(yy,2)]);pivots.append([sx,sy])
        arms.append(arm)
    # Small inner glove contours can be separated from the sleeve by its
    # painted outline. They still belong to the arm, not the adjacent thigh.
    # These bounded components were inspected in the r03 residual contact page.
    ca=core.getchannel('A');orphans=[0,0]
    groups=components_of((x,y) for y in range(128) for x in range(128) if ca.getpixel((x,y))>=32)
    for group in groups:
        if len(group)>12:continue
        gx=sum(x for x,y in group)/len(group);gy=sum(y for x,y in group)/len(group)
        near=[side for side,(px,py) in enumerate(palms) if abs(gx-px)<=5 and abs(gy-py)<=6]
        if not near:continue
        side=min(near,key=lambda s:(gx-palms[s][0])**2+(gy-palms[s][1])**2)
        owned=set(group)
        for x,y in group:
            for dx,dy in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]:
                q=x+dx,y+dy
                if 0<=q[0]<128 and 0<=q[1]<128 and 0<ca.getpixel(q)<32:owned.add(q)
        for q in owned:
            arms[side].putpixel(q,im.getpixel(q));core.putpixel(q,(0,0,0,0))
        orphans[side]+=len(owned)
    for side,(px,py) in enumerate(palms):
        # Keep all painted glove/cuff pieces and their antialiasing together.
        # Pruning to the largest island left small finger contours on the
        # stretched sleeve while the rest of the hand moved rigidly.
        fist=Image.new('RGBA',im.size);region=(round(px-3),round(py-3),round(px+4),round(py+4))
        fist.paste(arms[side].crop(region),region[:2]);fists.append(fist)
    parts=[core,arms[0],fists[0],arms[1],fists[1]];check=core.copy()
    for arm in arms:check.alpha_composite(arm)
    assert check.tobytes()==im.tobytes(),'ownership changed complete source figure'
    lower=(lower[0],max(lower[1],max(a.getchannel('A').getbbox()[3] for a in arms)),lower[2],lower[3])
    assert core.crop(lower).tobytes()==im.crop(lower).tobytes(),'lower leg armor moved into arm'
    packed=Image.new('RGBA',(640,128))
    for i,p in enumerate(parts):packed.paste(p,(i*128,0))
    meta.update(palms=palms,pivots=pivots,leg_guard=list(lower),upper_leg_guard=list(upper),grip_revision='WALK11-GRIP-r03',reassigned_glove_contour_pixels=orphans)
    return packed,meta

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    jobs=json.loads((DOC/'jobs.json').read_text(encoding='utf-8'))
    previous=json.loads((ROOT/'assets/animations/walk-09/player/manifest.json').read_text())
    entries=[];proof=[];bodies={};errors=[]
    for j in jobs:
        path=Path(j['destination']);index=j['index'];side=j['side'];facing=-1 if side=='left' else 1
        if not path.exists():continue
        try:
            face=Image.open(ROOT/f'assets/source-art/player-walk-review-11/CLASS-{index:02}-front-native128.png').convert('RGBA')
            fm=measure(face);target=fm['height']
            figures,warnings=grid_figures(Image.open(path).convert('RGBA'),4,2)
            assert len(figures)==8 and not warnings,(len(figures),warnings)
            sheet=Image.new('RGBA',(640,896));poses=[];metrics=[]
            for row,fig in enumerate(figures[:7]):
                b=box(fig);desired=target if row in [0,2,5] else target-1
                scale=desired/(b[3]-b[1])
                small=fig.resize((round(fig.width*scale),round(fig.height*scale)),Image.Resampling.LANCZOS);sb=box(small)
                native=Image.new('RGBA',(128,128));native.alpha_composite(small,(round(64-(sb[0]+sb[2])/2),96-sb[3]))
                parts,meta=grip_ownership(native,index,facing,8 if row==0 else row-1);sheet.paste(parts,(0,row*128));poses.append(native)
                entries.append(dict(id=f'CLASS-{index:02}',facing=facing,frame=8 if row==0 else row-1,sheet_row=row,file=f'assets/animations/walk-11/player/CLASS-{index:02}-{side}.png',**meta))
                m=measure(native);metrics.append(dict(frame=8 if row==0 else row-1,uniform_xy_scale=scale,**m,upper_body_width_ratio=round(m['upper_body_mean_occupied_width']/fm['upper_body_mean_occupied_width'],3)))
            sheet.save(OUT/f'CLASS-{index:02}-{side}.png',optimize=True);bodies[index,side]=poses
            proof.append(dict(id=j['id'],source=str(path.relative_to(ROOT)).replace('\\','/'),source_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),front=fm,target_height=target,metrics=metrics))
        except Exception as exc:errors.append(dict(id=j['id'],error=str(exc)))
    (DOC/'conditioning.json').write_text(json.dumps(dict(status='preview_only',normal_game_changed=False,entries=proof,errors=errors),indent=2)+'\n')
    if errors or len(entries)!=350:
        raise SystemExit(json.dumps(dict(errors=errors,entries=len(entries))))
    (OUT/'manifest.json').write_text(json.dumps(entries,indent=2)+'\n')
    for page in range(5):
        board=Image.new('RGB',(1024,720),'#303a40');draw=ImageDraw.Draw(board)
        for rank in range(5):
            index=page*5+rank;y=rank*144;draw.text((5,y+3),f'CLASS-{index:02} FACE | CURRENT LEFT RIGHT | CANDIDATE LEFT RIGHT',fill='white')
            face=Image.open(ROOT/f'assets/source-art/player-walk-review-11/CLASS-{index:02}-front-native128.png').convert('RGBA')
            for col,im in [(0,face)]+[(i+1,whole(Image.open(ROOT/next(e['file'] for e in previous if e['id']==f'CLASS-{index:02}' and e['facing']==f and e['frame']==8)).convert('RGBA'),0)) for i,f in enumerate([-1,1])]+[(i+3,bodies[index,s][0]) for i,s in enumerate(['left','right']) if (index,s) in bodies]:
                p=im.crop((32,32,96,96)).resize((128,128),Image.Resampling.NEAREST);board.paste(p,(col*180,y+15),p)
        board.save(DOC/f'comparison-{page}.png')
    print(json.dumps(dict(sources=len(proof),whole_poses=len(entries),errors=errors,idle_width_ratios={p['id']:p['metrics'][0]['upper_body_width_ratio'] for p in proof})))

if __name__=='__main__':main()
