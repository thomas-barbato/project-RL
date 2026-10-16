"""Layout only: existing before/after renders, native pixels and palette views."""
from pathlib import Path
import json
from PIL import Image,ImageDraw,ImageFont
HERE=Path(__file__).resolve().parent
BEFORE=HERE.parent/'armor-flat-review-01'
ACCEPTED=HERE.parent/'armor-flat-review-02'
data=json.loads((HERE/'review.json').read_text('utf-8'))
font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',22)
small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)
entries=sorted(data['entries'],key=lambda e:0 if e['id']=='AR-N14' else 1)
overview=Image.new('RGB',(1200,420*len(entries)),(14,21,29));d=ImageDraw.Draw(overview)
palettes=Image.new('RGB',(1400,430*len(entries)),(14,21,29));p=ImageDraw.Draw(palettes)
for row,e in enumerate(entries):
    ident=e['id'];y=row*420;v=2 if ident=='AR-N14' else 0
    old=ACCEPTED if ident=='AR-N14' else BEFORE
    d.text((18,y+8),ident+' · '+e['name'],fill='white',font=font)
    for col,folder in enumerate((old,HERE)):
        x=col*600
        d.text((x+220,y+43),'Avant' if col==0 else 'Après',fill='#b5ccd8',font=small)
        im=Image.open(folder/f'portraits/{ident}-V{v}-00.png').convert('RGBA')
        im.thumbnail((400,335),Image.Resampling.LANCZOS)
        overview.paste(im,(x+100,y+75),im)
        n=Image.open(folder/f'native32/{ident}-V{v}-00.png').convert('RGBA')
        for scale,pos in ((1,(x+490,y+135)),(2,(x+464,y+225))):
            n2=n.resize((64*scale,64*scale),Image.Resampling.NEAREST)
            d.rectangle((*pos,pos[0]+n2.width,pos[1]+n2.height),fill='#aeb8bb')
            overview.paste(n2,pos,n2)
    y=row*430
    p.text((18,y+8),ident+' · trois palettes',fill='white',font=font)
    for v in range(3):
        x=v*465
        im=Image.open(HERE/f'portraits/{ident}-V{v}-00.png').convert('RGBA')
        im.thumbnail((360,320),Image.Resampling.LANCZOS)
        palettes.paste(im,(x+10,y+62),im)
        n=Image.open(HERE/f'native32/{ident}-V{v}-00.png').convert('RGBA')
        palettes.paste(n,(x+380,y+145),n)
        n2=n.resize((128,128),Image.Resampling.NEAREST)
        palettes.paste(n2,(x+330,y+245),n2)
overview.save(HERE/'audit/overview.png')
palettes.save(HERE/'audit/palettes.png')
print('Saved before/after and palette boards')
