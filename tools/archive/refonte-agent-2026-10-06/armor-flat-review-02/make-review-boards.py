"""Lay out generated review compositions; no new illustration is drawn here."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont

OUT=Path(__file__).resolve().parent
data=json.loads((OUT/'review.json').read_text('utf-8'))
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',25)
smallfont=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)

def paste(canvas,source,at):canvas.paste(source,at,source)

overview=Image.new('RGB',(1500,1660),(13,21,29));d=ImageDraw.Draw(overview)
d.text((24,15),'ARMOR-FLAT02 · Huit propositions et le Manteau de mutation conservé',font=font,fill='white')
for i,e in enumerate(data['entries']):
    x=i%3*500;y=60+i//3*530
    d.text((x+20,y+8),e['id'],font=font,fill='#7be4d0')
    d.text((x+20,y+42),e['name'],font=smallfont,fill='#d4e0e8')
    im=Image.open(OUT/e['views']['0-00']['portrait']).convert('RGBA').resize((475,399),Image.Resampling.LANCZOS)
    paste(overview,im,(x+12,y+74))
    d.text((x+20,y+484),'Référence conservée' if e['id']=='AR-N06' else 'Nouveau dessin · à examiner',font=smallfont,fill='#a9bdcb')
overview.save(OUT/'audit/silhouettes.png')

configs=['00','02','07','09','10','11'];labels=['Sans arme','Deux pistolets','Lame + bouclier','Fusil','Arme lourde','Deux mains mêlée']
board=Image.new('RGB',(1600,1700),(13,21,29));d=ImageDraw.Draw(board)
d.text((18,12),'Prises communes · affichage en 64 px, agrandissement exact ×2',font=font,fill='white')
for c,label in enumerate(labels):d.text((160+c*235,55),label,font=smallfont,fill='#c7d7df')
for row,e in enumerate(data['entries']):
    y=82+row*175;d.text((16,y+57),e['id'],font=font,fill='#7be4d0')
    for c,cfg in enumerate(configs):
        im=Image.open(OUT/e['views']['0-'+cfg]['display64']).convert('RGBA')
        x=160+c*235;d.rectangle((x,y,x+127,y+127),fill='#aeb8bb');paste(board,im,(x,y))
board.save(OUT/'audit/equipment.png')

palettes=Image.new('RGB',(1300,2040),(13,21,29));d=ImageDraw.Draw(palettes)
d.text((18,12),'Palettes · vues exactes en 64 px et 32 px',font=font,fill='white')
for row,e in enumerate(data['entries']):
    y=55+row*218;d.text((14,y+42),e['id'],font=font,fill='#7be4d0')
    for v in range(3):
        x=165+v*375;view=e['views'][f'{v}-00']
        for size,key,dx in ((128,'display64',0),(64,'native32',150)):
            im=Image.open(OUT/view[key]).convert('RGBA')
            d.rectangle((x+dx,y+20,x+dx+size-1,y+20+size-1),fill='#aeb8bb');paste(palettes,im,(x+dx,y+20))
        d.text((x,y+156),'Référence' if not v else f'Variante {v+1}',font=smallfont,fill='#c7d7df')
palettes.save(OUT/'audit/palettes-native.png')
print('three review boards ready')
