"""Render-output layout only. No source bitmap edits or active replacements."""
from pathlib import Path
import hashlib,json
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
OLD=ROOT/'artifacts/armures-bicolores-completes-2026-10-05'
# Reuse the existing layout; only the isolated CPU renderer inputs change.
layout=(OLD/'build_gallery.py').read_text(encoding='utf-8')
layout=layout.replace("f'{version}-{side}-{palette}.rgba'","f'{version}-{side}-{palette}-0.rgba'")
layout=layout.replace('17 tenues — propositions numérotées','17 tenues — corps commun par armure')
layout=layout.replace('17 propositions numérotées : armures et tenues de base','17 tenues numérotées : un corps commun par armure')
layout=layout.replace('Recoloration des dessins existants, sans intégration dans le jeu. Corps de 30 pixels, contours et alpha inchangés. Deux zones de couleur, trois valeurs par matière colorée. Les armes et les silhouettes ne sont pas redessinées dans cet essai.',
    'Un dessin de référence par tenue pour le casque, le torse et les jambes. Les bras, mains et armes changent de pose et peuvent cacher une partie du torse. Corps de 30 pixels et variantes bicolores conservés. Aperçu avant intégration.')
layout=layout.replace("let model=3;","let model=9;")
for former,now in (('Vert / bleu nuit','Vert / bleu'),('Cuivre / bleu nuit','Cuivre / bleu'),('Olive / bleu nuit','Olive / bleu'),('Rouge / anthracite','Rouge / turquoise')):
    layout=layout.replace(former,now)
layout=layout.replace('Actuel /','Essai précédent /').replace('Inspection actuelle /','Inspection précédente /')
layout=layout.replace('Proposition /','Corps commun /').replace('Inspection proposée /','Inspection avec corps commun /')
exec(compile(layout,str(OUT/'layout_reused.py'),'exec'))

MODELS=json.loads((OUT/'modeles.json').read_text(encoding='utf-8'))
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',16)
small=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',13)
title=ImageFont.truetype('C:/Windows/Fonts/arialbd.ttf',22)
labels=('Sans arme','Épée','Fusil','Pistolet','Épée 2 mains','Deux épées','Deux pistolets','Mixte')
old=Image.open(OUT/'avant-48-0-0.png').convert('RGBA')
new=Image.open(OUT/'apres-48-0-0.png').convert('RGBA')

def crop(sheet,index,pose):return sheet.crop((pose*48,index*48,(pose+1)*48,(index+1)*48))
for start in (0,6,12):
    count=min(6,17-start)
    board=Image.new('RGBA',(1230,94+count*212),(16,23,29,255));d=ImageDraw.Draw(board)
    d.text((16,10),f'{start+1:02d}–{start+count:02d} / MÊME ARMURE, HUIT POSES',font=title,fill=(231,237,242))
    d.text((16,41),'Essai précédent puis corps commun · dessin sans arme choisi comme référence · zoom ×2',font=font,fill=(172,190,203))
    for pose,label in enumerate(labels):d.text((191+pose*126,73),label,font=small,fill=(107,212,213))
    for row,index in enumerate(range(start,start+count)):
        y=94+row*212
        if row%2==0:d.rectangle((8,y,1222,y+207),fill=(24,32,39))
        d.text((16,y+14),f'{index+1:02d}',font=title,fill=(107,212,213))
        name=MODELS[index]['name']
        lines=[];line=''
        for word in name.split():
            attempt=(line+' '+word).strip()
            if d.textbbox((0,0),attempt,font=font)[2]>158 and line:lines.append(line);line=word
            else:line=attempt
        lines.append(line)
        for n,line in enumerate(lines):d.text((16,y+46+n*19),line,font=font,fill=(231,237,242))
        d.text((16,y+122),'Avant / après',font=small,fill=(172,190,203))
        for pose in range(8):
            x=185+pose*126
            board.alpha_composite(crop(old,index,pose).resize((96,96),Image.Resampling.NEAREST),(x,y+2))
            board.alpha_composite(crop(new,index,pose).resize((96,96),Image.Resampling.NEAREST),(x,y+105))
    board.save(OUT/f'coherence-{start+1:02d}-{start+count:02d}.png')

shutil=__import__('shutil')
shutil.copy2(OUT/'galerie.html',OLD/'galerie-coherente.html')
print('Same numbered gallery updated in a new file: galerie-coherente.html. Previous gallery intact.')
