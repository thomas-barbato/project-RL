"""Apply only a bounded upper-body edit to each OWN approved full pose."""
import json,math
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from pack_walk07_cycle_repair import strip
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';ART=ROOT/'artifacts/player-walk-review-07'
TOP,BOTTOM=20,35

def head_center(image):
    alpha=image.getchannel('A');box=alpha.point(lambda a:255 if a>=128 else 0).getbbox()
    end=box[1]+max(1,round((box[3]-box[1])*.21));points=[]
    for y in range(box[1],end):
        for x in range(image.width):
            a=alpha.getpixel((x,y))
            if a>=128:points.append((x,a))
    return sum(x*a for x,a in points)/sum(a for _,a in points)

def bodies(job,target):
    index=job['indices'][0];spec=job['local_retouch_sources'];tiles=[];entries=[]
    source=Image.open(ROOT/spec['approved_atlas']).convert('RGBA')
    row_start=(index%5)*128
    board=Image.new('RGB',(896,288),'#293132');draw=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
    for side,row in [('left',0),('right',1)]:
        figures=strip(spec[side],7)
        for pose,figure in enumerate(figures):
            original=source.crop((pose*64,row_start+row*64,pose*64+64,row_start+row*64+64))
            bb=original.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
            fb=figure.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
            scale=(bb[3]-bb[1])/(fb[3]-fb[1])
            resized=figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.LANCZOS)
            rb=resized.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()
            dx=round(head_center(original)-head_center(resized));dy=bb[1]-rb[1]
            patch=Image.new('RGBA',(64,64));patch.alpha_composite(resized,(dx,dy))
            body=original.copy();body.paste(patch.crop((0,TOP,64,BOTTOM)),(0,TOP))
            assert body.crop((0,0,64,TOP)).tobytes()==original.crop((0,0,64,TOP)).tobytes()
            assert body.crop((0,BOTTOM,64,64)).tobytes()==original.crop((0,BOTTOM,64,64)).tobytes()
            tile=Image.new('RGBA',(128,128));tile.paste(body,(32,32));tiles.append(tile)
            draw.text((pose*128+2,row*144+2),f'{index+1:02} / {side} / '+('repos' if pose==0 else str(pose-1)),font=font,fill='#eeeade')
            native=body.resize((128,128),Image.Resampling.NEAREST);board.paste(native,(pose*128,row*144+16),native)
            entries.append({'side':side,'pose':pose,'protected_head_exact':True,'protected_lower_body_exact':True,
                'edited_native_rows':[TOP,BOTTOM],'patch_scale':scale,'patch_translation':[dx,dy]})
    board.save(ART/f'CLASS-{index:02}-r05-protected-poses-x2.png')
    return tiles,{'id':job['id'],'revision':'r05-local-retouch-frozen-approved-gait','pose_count':14,
        'approved_atlas':spec['approved_atlas'],'head_exact':True,'lower_body_exact':True,'entries':entries,
        'whole_original_pose_retained':True,'no_lower_body_grafting':True,'protected_working_rows_from':32+BOTTOM}

def protect_ownership(sheet,body,meta):
    cutoff=32+BOTTOM
    parts=[sheet.crop((i*128,0,(i+1)*128,128)) for i in range(5)]
    meta['protected_lower_pixels_restored']=sum(sum(a>0 for a in p.getchannel('A').crop((0,cutoff,128,128)).tobytes()) for p in [parts[1],parts[3]])
    parts[0].paste(body.crop((0,cutoff,128,128)),(0,cutoff))
    for part in parts[1:]:part.paste((0,0,0,0),(0,cutoff,128,128))
    # Re-measure actual hands after excluding the protected thighs.
    for side in range(2):
        arm=parts[1+side*2];alpha=arm.getchannel('A');b=alpha.point(lambda a:255 if a>=128 else 0).getbbox()
        assert b is not None
        points=[(x,y,alpha.getpixel((x,y))) for y in range(max(b[1],b[3]-6),b[3]) for x in range(b[0],b[2]) if alpha.getpixel((x,y))>=128]
        weight=sum(a for _,_,a in points);px=sum(x*a for x,y,a in points)/weight;py=sum(y*a for x,y,a in points)/weight
        meta['palms'][side]=[round(px,2),round(py,2)]
        fist=Image.new('RGBA',(128,128));region=(round(px-4),round(py-4),round(px+5),min(cutoff,round(py+5)))
        fist.paste(arm.crop(region),region[:2]);parts[2+side*2]=fist
    check=parts[0].copy()
    for part in [parts[1],parts[3]]:check.alpha_composite(part)
    assert check.tobytes()==body.tobytes(),'arm ownership changed the original pose'
    assert parts[0].crop((0,cutoff,128,128)).tobytes()==body.crop((0,cutoff,128,128)).tobytes()
    assert all(p.getchannel('A').crop((0,cutoff,128,128)).getbbox() is None for p in parts[1:])
    out=Image.new('RGBA',(640,128))
    for i,part in enumerate(parts):out.paste(part,(i*128,0))
    return out,meta

def select_sources():
    jobs=json.loads((DOC/'unarmed-jobs.json').read_text('utf8'))
    for j in jobs:
        index=j['indices'][0]
        if index not in [10,24]:continue
        if j.get('anatomy_sources'):continue
        slug='traqueuse' if index==10 else 'metamorphe'
        j['local_retouch_sources']={'approved_atlas':f'assets/animations/walk-06/{slug}-rifle.png',
            **{side:str(ROOT/f'assets/source-art/player-walk-review-07/LOCAL-{index:02}-r05-{side}.png') for side in ['left','right']}}
        j['review_revision']='r05-local-retouch-frozen-approved-gait'
    (DOC/'unarmed-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
if __name__=='__main__':select_sources()
