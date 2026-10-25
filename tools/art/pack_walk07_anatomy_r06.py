"""Pack complete corrected figures; no leg transforms or material mirroring."""
import json,statistics
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_review_04 import components
from pack_walk07_cycle_repair import strip
ROOT=Path(__file__).resolve().parents[2];DOC=ROOT/'docs/reviews/player-walk-07';SRC=ROOT/'assets/source-art/player-walk-review-07';ART=ROOT/'artifacts/player-walk-review-07'

def two_rows(path):
    image=Image.open(path).convert('RGBA');parts=components(image)
    primary=sorted(parts,key=lambda p:p['area'],reverse=True)[:14]
    assert len(primary)==14 and primary[-1]['area']>primary[0]['area']*.65
    primary.sort(key=lambda p:p['center'][1]);primary=[p for row in range(2) for p in sorted(primary[row*7:(row+1)*7],key=lambda p:p['center'][0])]
    ids={id(p) for p in primary};groups=[[p] for p in primary]
    for p in parts:
        if id(p) in ids or p['area']<8:continue
        n=min(range(14),key=lambda n:sum((p['center'][a]-primary[n]['center'][a])**2 for a in [0,1]));groups[n].append(p)
    out=[]
    for group in groups:
        box=(max(0,min(p['box'][0] for p in group)-2),max(0,min(p['box'][1] for p in group)-2),min(image.width,max(p['box'][2] for p in group)+2),min(image.height,max(p['box'][3] for p in group)+2))
        out.append(image.crop(box))
    return out

def bodies(job,target):
    index=job['indices'][0];spec=job['anatomy_sources'];figures=two_rows(ROOT/spec['main']);source_labels=[spec['main']]*14
    if index==10:figures[:7]=strip(ROOT/spec['left'],7);source_labels[:7]=[spec['left']]*7
    else:
        support=strip(ROOT/spec['first_support'],2);figures[2]=support[0];figures[9]=support[1];source_labels[2]=source_labels[9]=spec['first_support']
        figures[5]=strip(ROOT/spec['second_support_left'],2)[0];source_labels[5]=spec['second_support_left']
        figures[12]=strip(ROOT/spec['second_support_right'],1)[0];source_labels[12]=spec['second_support_right']
    tiles=[];entries=[];board=Image.new('RGB',(896,288),'#293132');d=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
    for n,figure in enumerate(figures):
        b=figure.getchannel('A').point(lambda a:255 if a>=128 else 0).getbbox();scale=target/(b[3]-b[1])
        im=figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.LANCZOS);alpha=im.getchannel('A');b=alpha.point(lambda a:255 if a>=128 else 0).getbbox();centers=[]
        for y in range(b[1]+round(target*.3),b[1]+round(target*.5)):
            xs=[x for x in range(im.width) if alpha.getpixel((x,y))>=128]
            if xs:centers.append((min(xs)+max(xs))/2)
        center=statistics.median(centers);tile=Image.new('RGBA',(128,128));dx=64-round(center);dy=96-b[3];tile.alpha_composite(im,(dx,dy));tiles.append(tile)
        side,col=divmod(n,7);native=tile.crop((32,32,96,96));large=native.resize((128,128),Image.Resampling.NEAREST)
        d.text((col*128+2,side*144+2),f'{index+1:02} / '+('G' if side==0 else 'D')+' / '+['repos','A0','A1','A2','B0','B1','B2'][col],font=font,fill='#eeeade');board.paste(large,(col*128,side*144+16),large)
        entries.append({'side':side,'column':col,'source':source_labels[n],'native_height':b[3]-b[1],'uniform_scale':scale,'translation':[dx,dy],'whole_figure':True})
    board.save(ART/f'CLASS-{index:02}-r06-anatomy-poses-x2.png');board.resize((1792,576),Image.Resampling.NEAREST).save(ART/f'CLASS-{index:02}-r06-anatomy-poses-x4.png')
    assert max(e['native_height'] for e in entries)-min(e['native_height'] for e in entries)<=1
    return tiles,{'id':job['id'],'revision':'r06-anatomical-side-and-alternating-support','pose_count':14,'entries':entries,'whole_figures_only':True,'no_lower_body_grafting':True,'no_body_mirroring':True,'no_axis_deformation':True}

def select_sources():
    for stem in ['review-revision','validation','source-manifest','cycle-repair-packing','unarmed-jobs','renderer-checks','raster-audit','equipment-preview-pack']:
        p=DOC/f'{stem}.json';archive=DOC/f'{stem}-r05.json'
        if p.exists() and not archive.exists():archive.write_bytes(p.read_bytes())
    jobs=json.loads((DOC/'unarmed-jobs.json').read_text('utf8'))
    for j in jobs:
        i=j['indices'][0]
        if i not in [10,24]:continue
        if j.get('bastion_transfer_sources'):continue
        j['anatomy_sources']={'main':f'assets/source-art/player-walk-review-07/ANATOMY-{i:02}-r06b.png'}
        j['anatomy_sources'].update({'left':'assets/source-art/player-walk-review-07/ANATOMY-10-r06c-left.png'} if i==10 else {'first_support':'assets/source-art/player-walk-review-07/ANATOMY-24-r06c-first-support.png'})
        if i==24:j['anatomy_sources'].update({'second_support_left':'assets/source-art/player-walk-review-07/ANATOMY-24-r06d-second-support.png','second_support_right':'assets/source-art/player-walk-review-07/ANATOMY-24-r06e-second-support-right.png'})
        j['review_revision']='r06-anatomical-side-and-alternating-support'
    (DOC/'unarmed-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')
    revision=json.loads((DOC/'review-revision.json').read_text('utf8'))
    revision.update({'revision':'r06-anatomical-side-and-alternating-support','status':'awaiting_visual_review','previous_revision_status':'r05_rejected_for_leg_identity_and_gait','reference_mode':'same_character_previous_r05_unarmed','player_feedback':"euh , pourquoi sur la traqueuse la jambe de metale change (elle est parfois à droite parfois à gauche) ? et concernant la metamorphe , la démarche est ratée (mais presque réussie) ."})
    (DOC/'review-revision.json').write_text(json.dumps(revision,ensure_ascii=False,indent=2)+'\n','utf8')
if __name__=='__main__':select_sources()
