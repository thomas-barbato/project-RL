"""Whole-figure Bastion transfer, with explicit leg/hand pixel ownership."""
import json,statistics
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont,ImageFilter
from pack_walk07_cycle_repair import strip
from build_player_walk_review_04 import components
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';ART=ROOT/'artifacts/player-walk-review-07';SRC=ROOT/'assets/source-art/player-walk-review-07'

def bodies(job,target):
    index=job['indices'][0];tiles=[];entries=[]
    board=Image.new('RGB',(896,288),'#293132');draw=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
    for side,row in [('left',0),('right',1)]:
        path=job['bastion_transfer_sources'][side];figures=strip(ROOT/path,7)
        scale=target/statistics.median(p.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()[3]-p.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox()[1] for p in figures)
        for col,figure in enumerate(figures):
            im=figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.LANCZOS)
            alpha=im.getchannel('A');bb=alpha.point(lambda a:255 if a>=128 else 0).getbbox();h=bb[3]-bb[1];centers=[]
            for y in range(bb[1]+round(h*.25),bb[1]+round(h*.43)):
                xs=[x for x in range(im.width) if alpha.getpixel((x,y))>=128]
                if xs:centers.append((min(xs)+max(xs))/2)
            center=statistics.median(centers);dx=64-round(center);dy=96-bb[3]
            body=Image.new('RGBA',(128,128));body.alpha_composite(im,(dx,dy));tiles.append(body)
            native=body.crop((32,32,96,96)).resize((128,128),Image.Resampling.NEAREST);board.paste(native,(col*128,row*144+16),native)
            draw.text((col*128+2,row*144+2),f'{index+1:02} / '+side+' / '+('repos' if col==0 else str(col-1)),font=font,fill='#eeeade')
            entries.append({'direction':side,'column':col,'source':path,'uniform_scale':scale,'native_height':h,'translation':[dx,dy]})
    heights=[e['native_height'] for e in entries]
    assert max(heights)-min(heights)<=2,(job['id'],'height jump',heights)
    board.save(ART/f'CLASS-{index:02}-r07-whole-poses-x2.png')
    board.resize((1792,576),Image.Resampling.NEAREST).save(ART/f'CLASS-{index:02}-r07-whole-poses-x4.png')
    return tiles,{'id':job['id'],'revision':job.get('review_revision','r07-exact-bastion-whole-pose-transfer'),'pose_count':14,'entries':entries,
        'native_height_range':[min(heights),max(heights)],'whole_figures_only':True,'one_uniform_scale_per_direction':True,
        'no_head_only_rotation':True,'no_leg_transforms_or_grafting':True}

def protect_ownership(sheet,body,meta,index,side,col):
    # Hanging fists may extend below the waist. Preserve the connected leg
    # components rather than cutting ALL arms at a horizontal coordinate.
    cutoff=67;lower=body.copy();lower.paste((0,0,0,0),(0,0,128,cutoff))
    parts=components(lower,threshold=96);legs=[p for p in parts if p['box'][3]>=87]
    assert legs,(index,side,col,'no legs')
    mask=Image.new('L',(128,128));pixels=mask.load()
    for part in legs:
        for point in part['points']:pixels[point%128,point//128]=255
    mask=mask.filter(ImageFilter.MaxFilter(3));mask.paste(0,(0,0,128,cutoff))
    # At the waist a hanging glove can touch the thigh in the drawing. That
    # contact must not make the entire glove part of the fixed leg component.
    # These bounds cover the inspected Traqueuse outfits and Metamorphe pilot;
    # below the fists, retain the complete leg component without a width cut.
    mask.paste(0,(0,cutoff,55,77));mask.paste(0,(74,cutoff,128,77))
    parts=[sheet.crop((i*128,0,(i+1)*128,128)) for i in range(5)]
    left_alpha=parts[1].getchannel('A').tobytes();right_alpha=parts[3].getchannel('A').tobytes()
    restored=sum(1 for i,a in enumerate(mask.tobytes()) if a and (left_alpha[i] or right_alpha[i]))
    parts[0].paste(body,(0,0),mask)
    for part in parts[1:]:part.paste((0,0,0,0),(0,0),mask)
    for arm_side in range(2):
        arm=parts[1+2*arm_side];alpha=arm.getchannel('A');bb=alpha.point(lambda a:255 if a>=128 else 0).getbbox();assert bb
        points=[(x,y,alpha.getpixel((x,y))) for y in range(max(bb[1],bb[3]-6),bb[3]) for x in range(bb[0],bb[2]) if alpha.getpixel((x,y))>=128]
        weight=sum(a for x,y,a in points);px=sum(x*a for x,y,a in points)/weight;py=sum(y*a for x,y,a in points)/weight
        meta['palms'][arm_side]=[round(px,2),round(py,2)]
        fist=Image.new('RGBA',(128,128));bounds=(round(px-4),round(py-4),round(px+5),round(py+5));fist.paste(arm.crop(bounds),bounds[:2]);fist.paste((0,0,0,0),(0,0),mask);parts[2+2*arm_side]=fist
    recomposed=parts[0].copy()
    for arm in [parts[1],parts[3]]:recomposed.alpha_composite(arm)
    assert recomposed.tobytes()==body.tobytes(),'ownership changed the whole drawing'
    for part in parts[1:]:assert not any(a and b for a,b in zip(part.getchannel('A').tobytes(),mask.tobytes()))
    own=SRC/'rigs/ownership';own.mkdir(exist_ok=True)
    path=own/f'CLASS-{index:02}-{side}-{col}-legs.png';mask.save(path)
    meta.update({'leg_ownership_mask':path.relative_to(ROOT).as_posix(),'leg_pixels_reassigned_from_arms':restored})
    out=Image.new('RGBA',(640,128))
    for i,part in enumerate(parts):out.paste(part,(i*128,0))
    return out,meta

def select_sources():
    for stem in ['review-revision','unarmed-jobs','validation','source-manifest','cycle-repair-packing','renderer-checks','raster-audit','equipment-preview-pack']:
        p=DOC/f'{stem}.json';old=DOC/f'{stem}-r06.json'
        if p.exists() and not old.exists():old.write_bytes(p.read_bytes())
    jobs=json.loads((DOC/'unarmed-jobs.json').read_text('utf8'))
    for j in jobs:
        i=j['indices'][0]
        if i not in [10,24]:continue
        j['bastion_transfer_sources']={side:f'assets/source-art/player-walk-review-07/BASTION-TRANSFER-{i:02}-r07-{side}.png' for side in ['left','right']}
        for side in ['left','right']:
            if j.get(f'prosthesis_{side}_source'):j['bastion_transfer_sources'][side]=j[f'prosthesis_{side}_source']
        j['review_revision']='r07-exact-bastion-whole-pose-transfer'
    (DOC/'unarmed-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
    revision=json.loads((DOC/'review-revision.json').read_text('utf8'))
    revision.update({'revision':'r07-exact-bastion-whole-pose-transfer','status':'awaiting_visual_review','previous_revision_status':'r06_rejected_by_user',
        'reference_mode':'exact_original_bastion_canvas02','player_feedback':"la tête de la traqueuse n'est plus dans le bon sens, ses jambes ne bougent presque plus ; la démarche de la metamorphe ne fonctionne pas. S'inspirer du Bastion."})
    (DOC/'review-revision.json').write_text(json.dumps(revision,ensure_ascii=False,indent=2)+'\n','utf8')
if __name__=='__main__':select_sources()
