"""Compose review boards from the generated catalogue; no asset painting."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont
ROOT=Path(__file__).resolve().parent
DATA=json.loads((ROOT/'review.json').read_text('utf-8'))
FONT=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',21)
SMALL=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',16)
def paste(canvas,path,xy,size=None):
    im=Image.open(ROOT/path).convert('RGBA')
    if size:im=im.resize(size,Image.Resampling.LANCZOS)
    canvas.paste(im,xy,im)
def board(entries,name):
    rows=(len(entries)+2)//3;can=Image.new('RGB',(1800,730*rows),(14,21,29));d=ImageDraw.Draw(can)
    for j,e in enumerate(entries):
        x=j%3*600;y=j//3*730
        d.text((x+20,y+10),e['id']+' · '+e['name'],font=FONT,fill='white')
        pic=e['views']['0-07'];paste(can,pic['portrait'],(x+45,y+50),(510,428))
        for v in range(3):
            dx=x+37+v*180;view=e['views'][f'{v}-07']
            d.text((dx,y+480),'Référence' if v==0 else f'Variante {v+1}',font=SMALL,fill='#b4c7d0')
            d.rectangle((dx,y+510,dx+127,y+637),fill='#aeb8bb');paste(can,view['display64'],(dx,y+510))
            d.text((dx,y+645),'64 px · ×2',font=SMALL,fill='#b4c7d0')
            d.rectangle((dx+90,y+660,dx+153,y+723),fill='#aeb8bb');paste(can,view['native32'],(dx+90,y+660))
            d.text((dx,y+681),'32 px',font=SMALL,fill='#b4c7d0')
    can.save(ROOT/'audit'/name)
outfits=[e for e in DATA['entries'] if e['kind']=='outfit']
board([e for e in outfits if e['category']!='current'],'new-proposals.png')
for start in range(0,len(outfits),6):
    entries=outfits[start:start+6];can=Image.new('RGB',(1500,len(entries)*370),(14,21,29));d=ImageDraw.Draw(can)
    for row,e in enumerate(entries):
        y=row*370;d.text((10,y+5),e['id']+' · '+e['name'],font=FONT,fill='white')
        for col,cfg in enumerate(('00','02','09','10','11')):
            v=e['views'][f'0-{cfg}'];x=col*300
            paste(can,v['portrait'],(x+5,y+40),(280,235))
            d.rectangle((x+90,y+238,x+217,y+365),fill='#aeb8bb');paste(can,v['display64'],(x+90,y+238))
    can.save(ROOT/'audit'/f'equipment-{start//6+1}.png')
print('New proposals board and four equipment audit boards saved.')
