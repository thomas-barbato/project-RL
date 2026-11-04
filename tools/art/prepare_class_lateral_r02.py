"""Whole-figure eight-key-pose proposal; diagram guides are not game sprites."""
from pathlib import Path
import argparse
import hashlib
import json
import statistics
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OLD=ROOT/'assets/source-art/animation-polish-01/class-lateral-02/r01'
OUT=OLD.parent/'r02'
CLASSES=['bastion','arcaniste','metamorphe']
NAMES=['Bastion','Arcaniste','Métamorphe']
STATES=['CONTACT','CHARGE','PASSAGE','RELEVE','CONTACT OPPOSE','CHARGE','PASSAGE','RELEVE','REPOS']


def guides():
    OUT.mkdir(parents=True,exist_ok=True)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',11)
    guide=Image.new('RGB',(1728,440),(30,40,49));draw=ImageDraw.Draw(guide)
    # Coordinates describe whole anatomical poses; no limb from any sprite is transformed.
    near=[[(46,54),(41,67),(36,79)],[(46,55),(40,69),(41,79)],[(46,54),(47,68),(46,79)],[(46,53),(50,67),(51,79)]]
    far=[[(50,54),(54,67),(59,78)],[(50,55),(55,65),(59,74)],[(50,54),(46,62),(49,73)],[(50,53),(39,64),(36,75)]]
    for row,direction in enumerate([-1,1]):
        for phase in range(9):
            tile=Image.new('RGB',(96,96),(30,40,49));d=ImageDraw.Draw(tile)
            d.line((14,82,82,82),fill='#71828b',width=1)
            if phase==8:
                a=[(46,54),(44,68),(44,79)];b=[(50,54),(52,68),(52,79)]
            elif phase<4:a=near[phase];b=far[phase]
            else:a=far[phase-4];b=near[phase-4]
            a=[(46,a[0][1]),*a[1:]];b=[(50,b[0][1]),*b[1:]]
            def points(coords):return [(96-x if direction>0 else x,y) for x,y in coords]
            # The warm near leg and cool far leg retain anatomical ownership across the cycle.
            d.line(points(b),fill='#62aacd',width=3);d.line(points(a),fill='#f1b16a',width=3)
            for leg,color in [(b,'#62aacd'),(a,'#f1b16a')]:
                x,y=leg[-1];d.line(points([(x-5,y+2),(x+2,y+2)]),fill=color,width=3)
                for x,y in points(leg):d.ellipse((x-2,y-2,x+2,y+2),fill=color)
            hip=(a[0][1]+b[0][1])/2
            d.line(points([(48,hip),(48,40)]),fill='#d1dbe0',width=5)
            d.line(points([(42,40),(54,40)]),fill='#d1dbe0',width=3)
            d.ellipse((42,25,54,37),outline='#d1dbe0',width=2)
            swing=[3,2,0,-2,-3,-2,0,2,0][phase]
            d.line(points([(43,40),(41+swing,49),(42+swing,56)]),fill='#f1b16a',width=2)
            d.line(points([(53,40),(55-swing,49),(54-swing,56)]),fill='#62aacd',width=2)
            tile=tile.resize((192,192),Image.Resampling.NEAREST)
            guide.paste(tile,(phase*192,row*220+24))
            draw.text((phase*192+7,row*220+5),f'{phase+1:02d} / {STATES[phase]}',font=font,fill='white')
    guide.save(OUT/'eight-pose-guide.png')
    refs=json.loads((OLD/'references.json').read_text(encoding='utf-8'))
    for identity,label in zip(CLASSES,NAMES):
        ref=next(r for r in refs if r['class']==identity and r['tier']==3)
        front=Image.open(OLD/ref['file']).convert('RGBA')
        board=Image.new('RGB',(512,420),(30,40,49));board.paste(front.resize((384,384),Image.Resampling.NEAREST),(64,32),front.resize((384,384),Image.Resampling.NEAREST))
        ImageDraw.Draw(board).text((12,9),f'{label} / tenue lourde actuelle / garder cette identite',font=font,fill='white')
        board.save(OUT/f'{identity}-reference.png')
    (OUT/'references.json').write_text(json.dumps([r for r in refs if r['class'] in CLASSES and r['tier']==3],indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Prepared eight-key-pose anatomy guide and three active-class references')


def center(image):
    pixels=image.load();total=weighted=0
    for y in range(max(1,round(image.height*.25))):
        for x in range(image.width):
            a=pixels[x,y][3]
            if a>=96:total+=a;weighted+=x*a
    return weighted/total


def pack():
    refs=json.loads((OUT/'references.json').read_text(encoding='utf-8'));profiles=[]
    for r in refs:
        identity=r['class'];raw=Image.open(OUT/f'{identity}-source.png').convert('RGBA')
        assert raw.getchannel('A').getextrema()[0]==0,identity
        cells=[]
        for row in range(2):
            for col in range(9):
                tile=raw.crop((round(col*raw.width/9),round(row*raw.height/2),round((col+1)*raw.width/9),round((row+1)*raw.height/2)))
                box=tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();assert box
                cells.append((row,col,tile.crop(box)))
        scale=r['reference_height']/statistics.median(im.height for row,col,im in cells if col==8)
        targets={row:(96-round(im.width*scale))//2+center(im)*scale for row,col,im in cells if col==8}
        atlas=Image.new('RGBA',(864,192));poses=[]
        for row,col,im in cells:
            native=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.NEAREST)
            x=round(targets[row]-center(im)*scale);y=81-native.height
            assert 0<=x and x+native.width<=96 and y>=0
            pose=Image.new('RGBA',(96,96));pose.alpha_composite(native,(x,y));poses.append((row,col,pose))
            atlas.alpha_composite(pose,(col*96,row*96))
        for row in range(2):
            cycle=[im for rr,col,im in poses if rr==row]
            assert len({im.tobytes() for im in cycle[:8]})==8
            assert cycle[8].tobytes() not in [im.tobytes() for im in cycle[:8]]
        atlas.save(OUT/f'{identity}-atlas.png');atlas.resize((1728,384),Image.Resampling.NEAREST).save(OUT/f'{identity}-poses-zoom.png')
        profiles.append({**r,'atlas':(OUT/f'{identity}-atlas.png').relative_to(ROOT).as_posix(),'source':(OUT/f'{identity}-source.png').relative_to(ROOT).as_posix(),'scale':scale,'whole_figures_only':True,'sha256':hashlib.sha256((OUT/f'{identity}-atlas.png').read_bytes()).hexdigest()})
    (OUT/'manifest.json').write_text(json.dumps({'review':'CLASS-LATERAL02-r02','normal_game_enabled':False,'native_cell_px':64,'walk_frames':8,'lateral_idle_frame':8,'proposal_step_ms':650,'cycle_ms':1300,'frame':[96,96],'anchor':[48,81],'profiles':profiles},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Packed three active-class eight-pose cycles')


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--guides',action='store_true');p.add_argument('--pack',action='store_true');a=p.parse_args()
    if a.guides:guides()
    if a.pack:pack()
