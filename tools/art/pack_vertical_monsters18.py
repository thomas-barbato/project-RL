"""Condition complete authored creature rows at the canonical native stature."""
from pathlib import Path
import json
import hashlib
import argparse
import statistics
from PIL import Image, ImageDraw, ImageFont
from pack_vertical_walk_equipment18 import complete_figures
from pack_vertical_walk_size18 import structural_box
from pack_vertical_walk_review18 import opaque_box

ROOT=Path(__file__).resolve().parents[2]
ART=ROOT/'assets/source-art/vertical-walk-review-18/extension-C06'
OUT=ART/'conditioned-monsters'
PREVIEW=ROOT/'artifacts/vertical-walk-review-18/current/extension-C06'


def whole_cells(image):
    try:
        return complete_figures(image)
    except AssertionError:
        pass
    # Wide wings can cross a mathematical quarter boundary. Locate genuine
    # transparent gutters for each row instead; never trim an appendage.
    alpha=image.getchannel('A').point(lambda a:255 if a>=32 else 0)
    groups=[]
    for y in range(image.height):
        if sum(alpha.crop((0,y,image.width,y+1)).tobytes())//255>=4:
            if not groups or y-groups[-1][-1]>10:groups.append([])
            groups[-1].append(y)
    assert len(groups)==4,('authored row count',len(groups))
    edges=[0]+[(a[-1]+b[0])//2 for a,b in zip(groups,groups[1:])]+[image.height]
    cells=[]
    for row in range(4):
        strip=alpha.crop((0,edges[row],image.width,edges[row+1]))
        xs=[sum(strip.crop((x,0,x+1,strip.height)).tobytes()) for x in range(image.width)]
        cuts=[0]
        for q in (1,2,3):
            target=round(q*image.width/4)
            x=min(range(max(1,target-45),min(image.width-1,target+46)),key=lambda x:(xs[x],abs(x-target)))
            assert xs[x]==0,('no transparent column gutter',row,q)
            cuts.append(x)
        cuts.append(image.width)
        for col in range(4):
            cell=image.crop((cuts[col],edges[row],cuts[col+1],edges[row+1]))
            b=opaque_box(cell)
            assert b and b[0]>0 and b[2]<=cell.width and b[1]>0 and b[3]<=cell.height,('source crop',row,col)
            whole=cell.crop(b);out=Image.new('RGBA',(whole.width+4,whole.height+4));out.alpha_composite(whole,(2,2));cells.append(out)
    return cells


def main():
    OUT.mkdir(parents=True,exist_ok=True);PREVIEW.mkdir(parents=True,exist_ok=True)
    doc=json.loads((ROOT/'docs/reviews/vertical-walk-18/prompts-C06-monsters.json').read_text('utf8'))
    parser=argparse.ArgumentParser();parser.add_argument('--only',type=int);args=parser.parse_args()
    previous={}
    if args.only:
        previous={p['number']:p for p in json.loads((OUT/'manifest.json').read_text('utf8'))['profiles']}
    profiles=[];errors=[]
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),14)
    for job in doc['jobs']:
        if args.only and args.only!=job['number']:
            profiles.append(previous[job['number']]);continue
        path=ROOT/job.get('selected_output',job['output'])
        if not path.exists():continue
        try:
            cells=whole_cells(Image.open(path).convert('RGBA'))
            if job.get('selected_back_source'):
                back=Image.open(ROOT/job['selected_back_source']).convert('RGBA');new=[]
                for row in range(2):
                    for col in range(4):
                        cell=back.crop((round(col*back.width/4),round(row*back.height/2),round((col+1)*back.width/4),round((row+1)*back.height/2)))
                        box=opaque_box(cell);assert box and box[0]>0 and box[1]>0 and box[2]<cell.width and box[3]<cell.height
                        whole=cell.crop(box);padded=Image.new('RGBA',(whole.width+4,whole.height+4));padded.alpha_composite(whole,(2,2));new.append(padded)
                cells[8:]=new
            original=Image.open(ROOT/job['references'][0]).convert('RGBA')
            ref=structural_box(original);height=ref[3]-ref[1];width=ref[2]-ref[0]
            longitudinal=job.get('size_policy')=='longitudinal_extent_after_true_rotation'
            if longitudinal:height=max(height,width)
            side=max(128,original.width)
            foot=[side//2,job['foot'][1] if original.width==job['native_side'] else ref[3]]
            scales=[];native=[];reduced_cells=[]
            front_indexes=job.get('front_source_cells',[1,2,3,5,6,7])
            back_indexes=job.get('back_source_cells',[9,10,11,13,14,15])
            idle_index=job.get('idle_source_cell',0)
            common_scale=None
            if job.get('scale_policy')=='one_constant_for_selected_coils':
                common_scale=job.get('calibration_height_px',height)/statistics.median(structural_box(cells[n])[3]-structural_box(cells[n])[1] for n in set(front_indexes+back_indexes))
            for row in range(4):
                neutral=structural_box(cells[row*4]);scale=common_scale or height/(neutral[3]-neutral[1]);scales.append(scale)
                for image in cells[row*4:row*4+4]:
                    reduced=image.resize((round(image.width*scale),round(image.height*scale)),Image.Resampling.LANCZOS)
                    reduced_cells.append(reduced)
            # Extend transparent padding when the source has a taller crown or
            # wider moving wing. Keep physical stature and world foot fixed.
            largest=max(max(im.size) for im in reduced_cells)
            top_pad=max(0,max(structural_box(im)[3] for im in reduced_cells)-foot[1]+2)
            if top_pad or largest+4>side:
                old_side=side;side=((max(side+top_pad,largest+4)+63)//64)*64
                foot=[foot[0]+(side-old_side)//2,foot[1]+(side-old_side)//2]
            for reduced in reduced_cells:
                    box=structural_box(reduced);offset=(round(foot[0]-(box[0]+box[2])/2),foot[1]-box[3])
                    assert offset[0]>=0 and offset[1]>=0 and offset[0]+reduced.width<=side and offset[1]+reduced.height<=side,('canvas',job['id'],side,offset,reduced.size)
                    canvas=Image.new('RGBA',(side,side));canvas.alpha_composite(reduced,offset);native.append(canvas)
            sheet=Image.new('RGBA',(side*6,side*2));records=[]
            for direction,indexes in [('front',front_indexes),('back',back_indexes)]:
                for phase,n in enumerate(indexes):
                    sheet.paste(native[n],(phase*side,int(direction=='back')*side))
                    records.append({'direction':direction,'phase':phase,'bounds':list(structural_box(native[n]))})
            out=OUT/f'{job["id"]}.png';idle=OUT/f'{job["id"]}-idle.png';sheet.save(out);native[idle_index].save(idle)
            b=structural_box(native[idle_index]);ratio=(b[2]-b[0])/width
            old_entries=json.loads((ROOT/'assets/animations/walk-07/manifest.json').read_text('utf8'))['entries']
            override=json.loads((ROOT/'assets/animations/walk-11/creatures/manifest.json').read_text('utf8'))
            old_entries+=override if isinstance(override,list) else override['entries']
            old=next(e for e in reversed(old_entries) if e['number']==job['number'])
            profile={'number':job['number'],'name':job['name'],'role':job['role'],'file':out.relative_to(ROOT).as_posix(),
                'standing':idle.relative_to(ROOT).as_posix(),'source':path.relative_to(ROOT).as_posix(),'source_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
                'side':side,'foot':foot,'front_vertical_offset':old.get('front_vertical_offset',0),'shadow':old.get('shadow',True),'frames':records,'row_scale_xy_uniform':scales,'variants':job['variants'],
                'reference_height':height,'reference_width':width,'neutral_width_ratio':round(ratio,3),'anatomy_review_required':True,
                'size_policy':job.get('size_policy','reference_height'),
                'selected_front_source_cells':front_indexes,'selected_back_source_cells':back_indexes,'selected_idle_source_cell':idle_index,
                'scale_policy':job.get('scale_policy','per_authored_neutral_row'),
                'size_check': 'longitudinal_rotation_extent_preserved' if longitudinal else 'width_drift' if abs(ratio-1)>.2 else 'within_reference_envelope'}
            if job.get('selected_back_source'):
                profile['selected_back_source']=job['selected_back_source']
                profile['back_source_sha256']=hashlib.sha256((ROOT/job['selected_back_source']).read_bytes()).hexdigest()
            profiles.append(profile)
            board=Image.new('RGB',(side*4,side+40),'#1e2831');d=ImageDraw.Draw(board)
            for col,(label,im) in enumerate([('Référence',original),('Repos C06',native[idle_index]),('Face C06',native[front_indexes[1]]),('Dos C06',native[back_indexes[4]])]):
                d.text((col*side+3,3),label,font=font,fill='white');board.paste(im,(col*side,30),im)
            board.save(PREVIEW/f'{job["id"]}-native-comparison.png')
        except (AssertionError,ValueError) as error:errors.append({'id':job['id'],'error':str(error)})
    manifest={'review':'VERT18-C06-MON','status':'native_review_ready' if len(profiles)==len(doc['jobs']) and not errors else 'in_progress','normal_game_enabled':False,'native_tile_px':64,
        'requested_models':len(doc['jobs']),'conditioned_models':len(profiles),'profiles':profiles,'conditioning_errors':errors}
    (OUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf8')
    print(json.dumps({'conditioned':len(profiles),'errors':errors,'sizes':[(p['number'],p['neutral_width_ratio'],p['size_check']) for p in profiles]}))


if __name__=='__main__':main()
