"""Condition whole class figures; never graft or deform individual body parts."""
from pathlib import Path
import argparse
import hashlib
import json
import statistics
from PIL import Image, ImageDraw, ImageFont, ImageOps

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/class-lateral-02/r01'
RAW=ROOT/'artifacts/animation-polish-01/class-lateral-02/raw-references'
CLASSES=['bastion','technomancien','arcaniste','metamorphe']
LABELS=['Bastion','Technomancien','Arcaniste','Métamorphe']
TIERS=['01 / Base','02 / Légère','03 / Intermédiaire','04 / Lourde','05 / Très lourde']


def references():
    OUT.mkdir(parents=True,exist_ok=True)
    records=json.loads((RAW/'references.json').read_text(encoding='utf-8'))
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)
    for identity,label in zip(CLASSES,LABELS):
        board=Image.new('RGB',(1920,450),(30,40,49));draw=ImageDraw.Draw(board)
        draw.text((12,8),f'{label} / FRONTAUX ACTUELS : identité à conserver dans les nouveaux côtés',font=font,fill='white')
        for record in [r for r in records if r['class']==identity]:
            tier=record['tier']
            # Macroquad export_png flips CPU images for framebuffer exports.
            front=ImageOps.flip(Image.open(RAW/record['file']).convert('RGBA'))
            front.save(OUT/record['file'])
            bbox=front.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            record.update(reference_height=bbox[3]-bbox[1],bounds=bbox,sha256=hashlib.sha256((OUT/record['file']).read_bytes()).hexdigest())
            zoom=front.resize((384,384),Image.Resampling.NEAREST)
            board.paste(zoom,(tier*384,48),zoom)
            draw.text((tier*384+18,38),TIERS[tier],font=font,fill='#e9c084')
        board.save(OUT/f'{identity}-reference.png')
    (OUT/'references.json').write_text(json.dumps(records,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Prepared twenty exact normal-render front references and four family boards')


def upper_center(image):
    weighted=total=0
    for y in range(max(1,round(image.height*.25))):
        for x in range(image.width):
            alpha=image.getpixel((x,y))[3]
            if alpha>=96:weighted+=x*alpha;total+=alpha
    return weighted/total


def pack(heavy_only=False):
    records=json.loads((OUT/'references.json').read_text(encoding='utf-8'))
    results=[]
    for record in records:
        if heavy_only and record['tier']!=3:continue
        identity,tier=record['class'],record['tier'];stem=f'{identity}-t{tier}'
        source=OUT/f'{stem}-source.png'
        if not source.exists():continue
        raw=Image.open(source).convert('RGBA');cells=[]
        for row in range(2):
            for col in range(7):
                frame=raw.crop((round(col*raw.width/7),round(row*raw.height/2),round((col+1)*raw.width/7),round((row+1)*raw.height/2)))
                box=frame.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();assert box,(stem,row,col)
                cells.append((row,col,frame.crop(box)))
        scale=record['reference_height']/statistics.median(frame.height for row,col,frame in cells if col==6)
        targets={row:(96-round(frame.width*scale))//2+upper_center(frame)*scale for row,col,frame in cells if col==6}
        atlas=Image.new('RGBA',(672,192));poses=[]
        for row,col,frame in cells:
            native=frame.resize((round(frame.width*scale),round(frame.height*scale)),Image.Resampling.NEAREST)
            x=round(targets[row]-upper_center(frame)*scale);y=81-native.height
            assert 0<=x and x+native.width<=96 and y>=0,(stem,row,col,x,y)
            pose=Image.new('RGBA',(96,96));pose.alpha_composite(native,(x,y))
            poses.append((row,col,pose));atlas.alpha_composite(pose,(col*96,row*96))
        for row in range(2):
            cycle=[pose for r,col,pose in poses if r==row]
            assert len({p.tobytes() for p in cycle[:6]})==6
            assert cycle[6].tobytes() not in [p.tobytes() for p in cycle[:6]]
        atlas.save(OUT/f'{stem}-atlas.png')
        atlas.resize((1344,384),Image.Resampling.NEAREST).save(OUT/f'{stem}-poses-zoom.png')
        results.append({**record,'source':source.relative_to(ROOT).as_posix(),'atlas':(OUT/f'{stem}-atlas.png').relative_to(ROOT).as_posix(),'scale':scale,'whole_figures_only':True,'sha256':hashlib.sha256((OUT/f'{stem}-atlas.png').read_bytes()).hexdigest()})
    (OUT/'manifest.json').write_text(json.dumps({'review':'CLASS-LATERAL02-r01','native_cell_px':64,'anchor':[48,81],'frame':[96,96],'layout':'row0 left / row1 right; six walk poses + lateral idle','unarmed_only':True,'normal_game_enabled':False,'profiles':results},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(f'Packed {len(results)} complete lateral outfit cycles')


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--references',action='store_true');p.add_argument('--pack',action='store_true');p.add_argument('--heavy-only',action='store_true');args=p.parse_args()
    if args.references:references()
    if args.pack:pack(args.heavy_only)
