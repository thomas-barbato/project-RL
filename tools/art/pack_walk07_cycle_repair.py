"""Whole-pose repairs: two facing strips plus explicitly opposite supports."""
import json,statistics
from pathlib import Path
from PIL import Image
from build_player_walk_review_04 import components
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'

def strip(path,count):
    source=Image.open(path).convert('RGBA')
    all_parts=components(source)
    primary=sorted(all_parts,key=lambda p:p['area'],reverse=True)[:count]
    assert len(primary)==count and primary[-1]['area']>primary[0]['area']*.7,(path,'incomplete figures')
    assert sum(p['area']>primary[0]['area']*.4 for p in all_parts)==count,(path,'unexpected whole-figure count')
    assert max(p['center'][1] for p in primary)-min(p['center'][1] for p in primary)<source.height*.15,(path,'not a single pose row')
    primary.sort(key=lambda p:p['center'][0]);groups=[[p] for p in primary]
    ids={id(p) for p in primary}
    for part in all_parts:
        if id(part) in ids or part['area']<8:continue
        owner=min(range(count),key=lambda i:sum((part['center'][axis]-primary[i]['center'][axis])**2 for axis in [0,1]))
        groups[owner].append(part)
    figures=[]
    for group in groups:
        b=[min(p['box'][0] for p in group),min(p['box'][1] for p in group),max(p['box'][2] for p in group),max(p['box'][3] for p in group)]
        assert b[0]>1 and b[1]>1 and b[2]<source.width-1 and b[3]<source.height-1,(path,'figure touches canvas boundary',b)
        figures.append(source.crop(b))
    assert min(im.height for im in figures)/max(im.height for im in figures)>.94,(path,'scale or missing lower body')
    return figures

def bodies(job,target):
    tiles=[];audit=[]
    for side in ['left','right']:
        specification=job['direction_sources'][side]
        a=strip(specification['cycle'],8);b=strip(specification['opposite'],3)
        for family,figures,selected in [('first',a,list(range(4))),('opposite',b,list(range(3)))]:
            scale=min(target/statistics.median(im.height for im in figures),58/max(im.width for im in figures))
            for n in selected:
                figure=figures[n]
                image=figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.LANCZOS)
                alpha=image.getchannel('A').point(lambda v:255 if v>=128 else 0)
                bb=alpha.getbbox();pix=alpha.load();centers=[]
                for y in range(bb[1]+round((bb[3]-bb[1])*.25),bb[1]+round((bb[3]-bb[1])*.48)):
                    xs=[x for x in range(image.width) if pix[x,y]]
                    if xs:centers.append((min(xs)+max(xs))/2)
                center=statistics.median(centers)
                tile=Image.new('RGBA',(128,128));tile.alpha_composite(image,(64-round(center),96-bb[3]));tiles.append(tile)
                audit.append({'side':side,'half':family,'source_pose':n,'native_height':bb[3]-bb[1],
                    'torso_center':center,'source_height':figure.height,'uniform_scale':scale})
    heights=[p['native_height'] for p in audit]
    assert max(heights)-min(heights)<=2,(job['id'],'native height jump',heights)
    return tiles,{'id':job['id'],'pose_count':len(tiles),'entries':audit,
        'height_range_native':[min(heights),max(heights)],'whole_figures_only':True,'no_leg_grafting_or_axis_deformation':True}

def select_sources():
    jobs=json.loads((DOC/'unarmed-jobs.json').read_text('utf8'))
    for job in jobs:
        index=job['indices'][0]
        if index not in [10,24]:continue
        if job.get('local_retouch_sources'):continue
        job['direction_sources']={side:{'cycle':str(ROOT/f'assets/source-art/player-walk-review-07/UNARMED-{index:02}-r03-{side}.png'),
            'opposite':str(ROOT/f'assets/source-art/player-walk-review-07/UNARMED-{index:02}-r04-opposite-{side}.png')} for side in ['left','right']}
        job['review_revision']='r04-opposite-support-pilot'
    (DOC/'unarmed-jobs.json').write_text(json.dumps(jobs,ensure_ascii=False,indent=2)+'\n','utf8')

if __name__=='__main__':select_sources()
