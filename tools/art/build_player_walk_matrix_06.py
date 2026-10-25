"""Pack proposed class/armor cycles and the accepted reference, without game edits."""
from __future__ import annotations
import base64,io,json
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_review_04 import components

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-06'
ART=ROOT/'artifacts/player-walk-review-06'
VIS=Path('C:/Users/User/.codex/visualizations/2026/10/23/01a16812-c028-73a3-9d94-d6e3343cb79c')


def grid_figures(image,columns=4,rows=4):
    count=columns*rows
    parts=components(image)
    primary=sorted(parts,key=lambda p:p['area'],reverse=True)[:count]
    assert len(primary)==count and primary[-1]['area']>primary[0]['area']*.4,'Missing/incomplete figures'
    assert sum(p['area']>primary[0]['area']*.4 for p in parts)==count,'Unexpected figure count'
    primary.sort(key=lambda p:p['center'][1])
    primary=[p for row in range(rows) for p in sorted(primary[row*columns:(row+1)*columns],key=lambda p:p['center'][0])]
    ids={id(p) for p in primary};groups=[[p] for p in primary];warnings=[]
    for part in parts:
        if id(part) in ids or part['area']<8:continue
        owner=min(range(count),key=lambda i:sum((part['center'][a]-primary[i]['center'][a])**2 for a in (0,1)))
        if part['area']>primary[owner]['area']*.04:warnings.append({'cell':owner,'detached_area':part['area']})
        groups[owner].append(part)
    result=[]
    for group in groups:
        b=(max(0,min(p['box'][0] for p in group)-2),max(0,min(p['box'][1] for p in group)-2),
           min(image.width,max(p['box'][2] for p in group)+2),min(image.height,max(p['box'][3] for p in group)+2))
        assert b[2]-b[0]<image.width*(1.24/columns) and b[3]-b[1]<image.height*(1.16/rows),('Overlapping cells',b)
        result.append(image.crop(b))
    return result,warnings


def proposed(job):
    figures,warnings=grid_figures(Image.open(job['source_destination']).convert('RGBA'))
    target=min(62,job['record']['bounds'][3]-job['record']['bounds'][1])
    scale=min(target/max(p.height for p in figures),60/max(p.width for p in figures))
    tiles=[]
    for image in figures:
        im=image.resize((round(image.width*scale),round(image.height*scale)),Image.Resampling.LANCZOS)
        tile=Image.new('RGBA',(64,64));tile.alpha_composite(im,(32-im.width//2,63-im.height));tiles.append(tile)
    return [tiles[0],*tiles[1:7],tiles[8],*tiles[9:15]],warnings,scale


def original(walk_path,idle_path):
    walk=Image.open(walk_path).convert('RGBA');idle=Image.open(idle_path).convert('RGBA')
    tiles=[]
    for row in (7,1):
        tiles.append(idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64)))
        for f in range(6):tiles.append(walk.crop((f*64,row*64,(f+1)*64,(row+1)*64)))
    return tiles


def main():
    ART.mkdir(parents=True,exist_ok=True)
    jobs=json.loads((DOC/'jobs.json').read_text('utf-8'))
    atlas=Image.new('RGBA',(448,26*128));entries=[];errors=[];pending=[]
    font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
    for index,j in enumerate(jobs):
        try:
            if index==10:
                accepted=ROOT/'assets/source-art/player-walk-review-05/accepted/native64'
                tiles=original(accepted/'traqueuse-rifle-walk.png',accepted/'traqueuse-rifle-idle.png');warnings=[];scale=None
            elif Path(j['source_destination']).exists():tiles,warnings,scale=proposed(j)
            else:pending.append(j['id']);continue
            for n,tile in enumerate(tiles):atlas.alpha_composite(tile,(n%7*64,index*128+n//7*64))
            board=Image.new('RGB',(560,224),'#293132');draw=ImageDraw.Draw(board)
            for side in range(2):
                draw.text((8,side*112+5),j['label']+(' / gauche' if side==0 else ' / droite'),font=font,fill='#eeeade')
                for f in range(7):board.paste(tiles[side*7+f],(f*80+8,side*112+30),tiles[side*7+f])
            board.save(ART/f'{j["id"]}-native-poses.png')
            entries.append({'id':j['id'],'index':index,'number':index+1,'label':j['label'],
                'class_index':j['class_index'],'stage':j['stage'],'status':'accepted_reference' if index==10 else 'proposal',
                'target_height':min(62,j['record']['bounds'][3]-j['record']['bounds'][1]),
                'scale':scale,'warnings':warnings,'frames':14})
        except Exception as exc:errors.append({'id':j['id'],'error':str(exc)})
    bastion=ROOT/'assets/source-art/player-walk-review-01/accepted/native64'
    for n,tile in enumerate(original(bastion/'bastion-walk-native64.png',bastion/'bastion-idle-native64.png')):
        atlas.alpha_composite(tile,(n%7*64,25*128+n//7*64))
    atlas_path=ART/'class-armor-cycles-lossless.webp';atlas.save(atlas_path,lossless=True,quality=100,method=6,exact=True)
    assert Image.open(atlas_path).convert('RGBA').tobytes()==atlas.tobytes(),'Lossless encoder changed pixels'
    native=Image.new('RGB',(624,600),'#293132');draw=ImageDraw.Draw(native)
    names=['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe']
    for stage in range(5):draw.text((146+stage*96,7),f'Tenue{stage+1}',font=font,fill='#eeeade')
    for ci,name in enumerate(names):draw.text((8,56+ci*112),name,font=font,fill='#eeeade')
    for e in entries:
        tile=atlas.crop((0,e['index']*128,64,e['index']*128+64));native.paste(tile,(144+e['stage']*96,32+e['class_index']*112),tile)
        draw.text((144+e['stage']*96,97+e['class_index']*112),f'{e["number"]:02}',font=font,fill='#eeeade')
    native.save(ART/'all25-native-overview.png')
    payload={'atlas':'data:image/webp;base64,'+base64.b64encode(atlas_path.read_bytes()).decode('ascii'),
        'entries':entries,'classes':names,'pending':pending,'errors':errors}
    (ART/'inline-data.json').write_text(json.dumps(payload,ensure_ascii=False,separators=(',',':')),encoding='utf-8')
    template=ROOT/'tools/art/player_walk_matrix_06.html'
    fragments=[]
    if template.exists():
        slugs=['bastion','technomancien','traqueuse','arcaniste','metamorphe']
        for ci,name in enumerate(names):
            group=[e for e in entries if e['class_index']==ci]
            if len(group)!=5:continue
            image=atlas.crop((0,ci*640,448,(ci+1)*640));buffer=io.BytesIO()
            image.save(buffer,format='WEBP',lossless=True,quality=100,method=6,exact=True)
            encoded=buffer.getvalue()
            assert Image.open(io.BytesIO(encoded)).convert('RGBA').tobytes()==image.tobytes()
            (ART/f'{slugs[ci]}-cycles-lossless.webp').write_bytes(encoded)
            data={'atlas':'data:image/webp;base64,'+base64.b64encode(encoded).decode('ascii'),
                'className':name,'entries':group}
            fragment=(template.read_text('utf-8').replace('__CLASS_INDEX__',str(ci)).replace('__CLASS_NAME__',name)
                .replace('__WALK_MATRIX_DATA__',json.dumps(data,ensure_ascii=False,separators=(',',':'))))
            size=len(fragment.encode('utf-8'));assert size<1_000_000,size
            target=VIS/f'player-walk-{slugs[ci]}.html';target.write_text(fragment,encoding='utf-8')
            (ART/f'{slugs[ci]}-preview-fragment.html').write_text(fragment,encoding='utf-8')
            fragments.append({'class':name,'path':str(target),'bytes':size,'atlas_bytes':len(encoded)})
        for ci,name in enumerate(names):
            board=Image.new('RGB',(560,1120),'#293132')
            for stage in range(5):
                source=ART/f'CLASS-{ci*5+stage:02}-native-poses.png'
                if source.exists():board.paste(Image.open(source),(0,stage*224))
            board.save(ART/f'{slugs[ci]}-all-native-poses.png')
    report={'review_id':'WALK06-CLASS-ARMOR-MATRIX','configuration':'rifle only; remaining grips/helmets/bestiary pending',
        'status':'awaiting_visual_review','ready_count':len(entries),'pending':pending,'errors':errors,
        'entries':entries,'atlas_bytes':atlas_path.stat().st_size,'fragments':fragments,
        'compression':'lossless WebP; decoded RGBA equality verified','runtime_integration':False}
    (DOC/'packaging.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({k:v for k,v in report.items() if k!='entries'},ensure_ascii=True,indent=2))


if __name__=='__main__':main()
