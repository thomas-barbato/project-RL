"""Pack whole painted figures with shared phase registration and protected legs."""
from pathlib import Path
import hashlib,json
from PIL import Image,ImageDraw
from build_player_walk_matrix_06 import grid_figures
from build_walk_rigs_02 import rig
from prepare_walk09_jobs import whole

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-09'
SRC=ROOT/'assets/source-art/player-walk-review-09'
DEST=ROOT/'assets/animations/walk-09/player'

def box(im):return im.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def protected_parts(im,index):
    packed,meta=rig(im,index)
    parts=[packed.crop((p*128,0,(p+1)*128,128)) for p in range(5)]
    b=box(im);h=b[3]-b[1];cx=meta['center']
    guard=(round(cx-h*.22),round(b[1]+h*.56),round(cx+h*.22),b[3])
    # Ownership only: retain the source's central thighs/knees/shins/boots
    # in the core so raising an arm cannot take away an armor plate.
    parts[0].paste(im.crop(guard),guard[:2])
    for part in parts[1:]:part.paste((0,0,0,0),guard)
    for side in range(2):
        arm=parts[1+2*side];a=arm.getchannel('A');ab=box(arm)
        if ab:
            points=[(x,y,a.getpixel((x,y))) for y in range(max(ab[1],ab[3]-6),ab[3]) for x in range(ab[0],ab[2]) if a.getpixel((x,y))>=128]
            if points:
                weight=sum(v for x,y,v in points);px=sum(x*v for x,y,v in points)/weight;py=sum(y*v for x,y,v in points)/weight
                meta['palms'][side]=[round(px,2),round(py,2)]
                fist=Image.new('RGBA',(128,128));region=(round(px-4),round(py-4),round(px+5),round(py+5));fist.paste(arm.crop(region),region[:2]);parts[2+2*side]=fist
    check=parts[0].copy()
    for p in [1,3]:check.alpha_composite(parts[p])
    assert check.tobytes()==im.tobytes(),'Ownership changes the complete painting'
    assert parts[0].crop(guard).tobytes()==im.crop(guard).tobytes()
    meta['leg_guard']=list(guard)
    result=Image.new('RGBA',(640,128))
    for p,part in enumerate(parts):result.paste(part,(p*128,0))
    return result,meta

def main():
    DEST.mkdir(parents=True,exist_ok=True)
    previous=json.loads((ROOT/'assets/animations/walk-08/player/manifest.json').read_text())
    jobs=json.loads((DOC/'direction-jobs.json').read_text('utf-8'))
    entries=[];proof=[];errors=[];bodies={}
    for index in range(25):
        target=0
        for facing in [-1,1]:
            e=next(e for e in previous if e['id']==f'CLASS-{index:02}' and e['facing']==facing and e['frame']==8)
            bb=box(whole(Image.open(ROOT/e['file']).convert('RGBA'),e['sheet_row']));target=max(target,bb[3]-bb[1])
        target=min(target,62)
        for side,facing in [('left',-1),('right',1)]:
            j=next(j for j in jobs if j['index']==index and j['side']==side)
            try:
                source=Image.open(j['source']).convert('RGBA');figures,warnings=grid_figures(source,4,2)
                sheet=Image.new('RGBA',(640,896));poses=[];scales=[]
                for row,f in enumerate(figures[:7]):
                    bb=box(f);desired=target if row in [0,2,5] else target-1
                    scale=desired/(bb[3]-bb[1]);scales.append(scale)
                    small=f.resize((round(f.width*scale),round(f.height*scale)),Image.Resampling.LANCZOS);sb=box(small)
                    im=Image.new('RGBA',(128,128));im.alpha_composite(small,(round(64-(sb[0]+sb[2])/2),96-sb[3]))
                    parts,meta=protected_parts(im,index);sheet.paste(parts,(0,row*128));poses.append(im)
                    entries.append({'id':f'CLASS-{index:02}','facing':facing,'frame':8 if row==0 else row-1,'sheet_row':row,'file':f'assets/animations/walk-09/player/CLASS-{index:02}-{side}.png',**meta})
                sheet.save(DEST/f'CLASS-{index:02}-{side}.png',optimize=True);bodies[index,side]=poses
                heights=[box(p)[3]-box(p)[1] for p in poses]
                proof.append({'id':j['id'],'source':str(Path(j['source']).relative_to(ROOT)).replace('\\','/'),'source_sha256':digest(Path(j['source'])),'guide_sha256':digest(Path(j['gait_reference'])),'warnings':warnings,'scales':scales,'whole_figure_uniform_xy_only':True,'leg_armor_owned_by_core':True,'height_range':[min(heights),max(heights)],'target':target})
            except Exception as exc:errors.append({'id':j['id'],'error':str(exc)})
    (DOC/'conditioning.json').write_text(json.dumps({'entries':proof,'errors':errors},indent=2)+'\n','utf-8')
    if errors:print(json.dumps({'errors':errors}));return
    assert len(entries)==350
    (DEST/'manifest.json').write_text(json.dumps(entries,indent=2)+'\n','utf-8')
    names=['Bastion','Technomancien','Traqueuse','Arcaniste','Metamorphe']
    for cls in range(5):
        board=Image.new('RGB',(896,1440),'#303a40');draw=ImageDraw.Draw(board)
        for rank in range(5):
            for direction,side in enumerate(['left','right']):
                y=(rank*2+direction)*144;draw.text((8,y+2),f'{names[cls]} {rank+1}/5 {side}',fill='white')
                for p,im in enumerate(bodies[cls*5+rank,side]):
                    q=im.crop((32,32,96,96)).resize((128,128),Image.Resampling.NEAREST);board.paste(q,(p*128,y+16),q)
        board.save(DOC/f'{names[cls]}-all-poses-x2.png')
    for rank in range(5):
        frames=[];durations=[]
        for pose in [0,1,2,3,0,4,5,6,0]:
            frame=Image.new('RGB',(640,304),'#303a40');draw=ImageDraw.Draw(frame)
            for cls in range(5):
                draw.text((cls*128+5,3),names[cls],fill='white')
                for direction,side in enumerate(['left','right']):
                    im=bodies[cls*5+rank,side][pose].crop((32,32,96,96)).resize((128,128),Image.Resampling.NEAREST);frame.paste(im,(cls*128,20+direction*140),im)
            frames.append(frame);durations.append(260 if pose==0 else 130)
        frames[0].save(DOC/f'common-outfit-{rank+1}.gif',save_all=True,append_images=frames[1:],duration=durations,loop=0,optimize=False)
    print(json.dumps({'outfits':25,'whole_poses':350,'sources':50,'errors':errors,'warnings':[e['id'] for e in proof if e['warnings']],'height_ranges_over_2px':[e['id'] for e in proof if e['height_range'][1]-e['height_range'][0]>2]}))

if __name__=='__main__':main()
