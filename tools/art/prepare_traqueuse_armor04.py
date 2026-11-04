"""Whole-figure Traqueuse armor family with canonical VERT18 front/back frames."""
from pathlib import Path
import argparse
import hashlib
import json
import statistics
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'assets/source-art/animation-polish-01/lateral-identity-02'
OUT=BASE/'r04'
PROFILES=['traqueuse','traqueuse-armor1','traqueuse-armor2','traqueuse-armor3','traqueuse-lourde']
LABELS=['01 / Base','02 / Légère','03 / Intermédiaire','04 / Lourde','05 / Très lourde']


def upper_center(image):
    total=weighted=0
    for y in range(round(image.height*.25)):
        for x in range(image.width):
            alpha=image.getpixel((x,y))[3]
            if alpha>=96:total+=alpha;weighted+=x*alpha
    return weighted/total


def references():
    OUT.mkdir(parents=True,exist_ok=True)
    manifest=json.loads((ROOT/'assets/animations/vertical-18/manifest.json').read_text())
    records=[]
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)
    for tier,identity in enumerate(PROFILES):
        p=next(p for p in manifest['profiles'] if p['id']==identity)
        family=p['families']['ranged']
        body=Image.open(ROOT/family['body_file']).convert('RGBA')
        hands=Image.open(ROOT/family['hands_file']).convert('RGBA')
        w,h=family['cell'];ox,oy=family['origin'];ax,ay=p.get('anchor',[128,160])
        for n in range(13):
            box=((n%6)*w,(n//6)*h,(n%6+1)*w,(n//6+1)*h)
            frame=Image.new('RGBA',(96,96))
            for sheet in [body,hands]:frame.alpha_composite(sheet.crop(box),(round(ox-ax+48),round(oy-ay+81)))
            frame.save(OUT/f'tier{tier}-canonical-{n}.png')
        board=Image.new('RGB',(960,360),(30,40,49));draw=ImageDraw.Draw(board)
        draw.text((12,8),f'TRAQUEUSE / {LABELS[tier]} / identité de référence',fill='white',font=font)
        for col,(n,label) in enumerate([(12,'Face / repos'),(2,'Face / marche'),(9,'Dos / marche')]):
            frame=Image.open(OUT/f'tier{tier}-canonical-{n}.png').convert('RGBA').resize((288,288),Image.Resampling.NEAREST)
            board.paste(frame,(col*320+16,40),frame);draw.text((col*320+20,334),label,fill='white',font=font)
        board.save(OUT/f'tier{tier}-reference.png')
        bounds=Image.open(OUT/f'tier{tier}-canonical-12.png').getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
        records.append({'tier':tier,'profile':identity,'outfit':p['outfit'],'family':'ranged','reference_height':bounds[3]-bounds[1],'body':family['body_file'],'hands':family['hands_file'],'anchor':p.get('anchor',[128,160])})
    (OUT/'references.json').write_text(json.dumps(records,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Prepared five complete canonical reference sets')


def pack():
    records=json.loads((OUT/'references.json').read_text(encoding='utf-8'))
    runtime=ROOT/'assets/animations/lateral-04';runtime.mkdir(parents=True,exist_ok=True)
    packed=[]
    for ref in records:
        tier=ref['tier'];atlas=Image.new('RGBA',(672,384));poses=[]
        if tier==3:
            for row,direction in enumerate(['left','right']):
                for n in range(7):
                    poses.append((row,n,Image.open(BASE/f'r03/candidate-03-{direction}-{n}.png').convert('RGBA')))
            scale=None
        else:
            raw=Image.open(OUT/f'tier{tier}-source.png').convert('RGBA');cells=[]
            for row in range(2):
                for col in range(7):
                    frame=raw.crop((round(col*raw.width/7),round(row*raw.height/2),round((col+1)*raw.width/7),round((row+1)*raw.height/2)))
                    box=frame.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();assert box
                    cells.append((row,col,frame.crop(box)))
            scale=ref['reference_height']/statistics.median(frame.height for row,col,frame in cells if col==6)
            targets={row:(96-round(frame.width*scale))//2+upper_center(frame)*scale for row,col,frame in cells if col==6}
            for row,col,frame in cells:
                native=frame.resize((round(frame.width*scale),round(frame.height*scale)),Image.Resampling.NEAREST)
                x=round(targets[row]-upper_center(frame)*scale);y=81-native.height
                assert 0<=x and x+native.width<=96 and y>=0
                pose=Image.new('RGBA',(96,96));pose.alpha_composite(native,(x,y));poses.append((row,col,pose))
        for row,col,pose in poses:
            pose.save(OUT/f'tier{tier}-{"left" if row==0 else "right"}-{col}.png')
            atlas.alpha_composite(pose,(col*96,row*96))
        for row in range(2):
            cycle=[pose for r,col,pose in poses if r==row]
            assert len({pose.tobytes() for pose in cycle[:6]})==6
            assert cycle[6].tobytes() not in [pose.tobytes() for pose in cycle[:6]]
        for n in range(6):
            atlas.alpha_composite(Image.open(OUT/f'tier{tier}-canonical-{n}.png').convert('RGBA'),(n*96,192))
            atlas.alpha_composite(Image.open(OUT/f'tier{tier}-canonical-{n+6}.png').convert('RGBA'),(n*96,288))
        atlas.alpha_composite(Image.open(OUT/f'tier{tier}-canonical-12.png').convert('RGBA'),(576,192))
        atlas.alpha_composite(Image.open(OUT/f'tier{tier}-canonical-11.png').convert('RGBA'),(576,288))
        for n in range(13):
            col,row=(n%6,2+n//6) if n<12 else (6,2)
            assert atlas.crop((col*96,row*96,(col+1)*96,(row+1)*96)).tobytes()==Image.open(OUT/f'tier{tier}-canonical-{n}.png').convert('RGBA').tobytes()
        if tier==3:
            assert atlas.crop((0,0,672,192)).tobytes()==Image.open(BASE/'r03/candidate-03-atlas.png').convert('RGBA').tobytes()
        path=runtime/f'traqueuse-tier{tier}.png';atlas.save(path)
        atlas.save(OUT/f'tier{tier}-atlas.png')
        atlas.resize((1344,768),Image.Resampling.NEAREST).save(OUT/f'tier{tier}-poses-zoom.png')
        packed.append({**ref,'candidate':tier+1,'scale':scale,'whole_figures_only':True,'runtime':str(path.relative_to(ROOT)).replace('\\','/'),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'canonical_front_back_reused':True,'heavy_lateral_r03_unchanged':tier==3})
    (runtime/'manifest.json').write_text(json.dumps({'review':'LATERAL02-r04','native_cell_px':64,'atlas':[672,384],'frame':[96,96],'anchor':[48,81],'rows':['left','right','front + idle','back'],'normal_game_default_enabled':False,'trial_only':True,'unarmed_only':True,'profiles':packed},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Packed five armor atlases with canonical front/back and consistent idle')


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--references',action='store_true');p.add_argument('--pack',action='store_true');args=p.parse_args()
    if args.references:references()
    if args.pack:pack()
