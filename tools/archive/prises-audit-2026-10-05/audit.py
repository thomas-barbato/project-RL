"""Diagnostic contact sheets only; compose the active native raster layers."""
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
ART=ROOT/'assets/sprites/cyberpunk'
manifest=json.loads((ART/'manifest.json').read_text(encoding='utf-8'))
names=[m['name'] for m in manifest['weapon_families']]
one=[0,1,4,5,6,7,16]
two=[2,3,8,9,10,11,12,13,14,15,17]
images={n:Image.open(ART/f'crawl-{n}.png').convert('RGBA') for n in ('bodies-32','arms-48','fingers-48','weapons-48')}

def cell(name,index,side):
    im=images[name];x=index%16*side;y=index//16*side
    return im.crop((x,y,x+side,y+side))

def compose(body,family,left=False,other=None):
    fixed=cell('bodies-32',body,32)
    layers=[(family,left)]+([] if other is None else [(other,not left)])
    for y in range(17,23):
        for x in range(32):
            if any((f in two and (3<=x<10 or 22<=x<30)) or
                   (f in one and y >= (19 if f in (6,7) else 18) and
                    (3<=x<10 if l else 22<=x<30)) for f,l in layers):
                fixed.putpixel((x,y),(0,0,0,0))
    result=Image.new('RGBA',(48,48));result.alpha_composite(fixed,(8,8))
    for f,l in layers:
        for bank in ('arms-48','weapons-48','fingers-48'):
            piece=cell(bank,f,48)
            if l: piece=piece.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
            result.alpha_composite(piece)
    return result

def main():
    preview=Image.new('RGB',(6*232,3*256),(26,36,43));d=ImageDraw.Draw(preview)
    for f in range(18):
        x=f%6*232;y=f//6*256
        d.text((x+8,y+8),f'{f+1:02} {names[f]}',fill='white')
        for col,body in enumerate([9,5]):
            im=compose(body,f).resize((144,144),Image.Resampling.NEAREST)
            if col==0: preview.paste(im,(x+6,y+36),im)
            else:
                im=compose(body,f)
                preview.paste(im,(x+168,y+80),im)
        if f in one:
            im=compose(9,f,True);preview.paste(im,(x+20,y+194),im)
            im=compose(9,f,False,f);preview.paste(im,(x+93,y+194),im)
        else:
            im=cell('weapons-48',f,48);preview.paste(im,(x+20,y+194),im)
        d.text((x+153,y+208),'1:1',fill='white')
    preview.save(HERE/'corrected-families.png')
    # Every compatible player/NPC body, every family and one-handed side.
    count=0
    for group,families in [('one',one),('two',two)]:
        for page,start in enumerate(range(0,23,12),1):
            n=min(12,23-start);cols=len(families)*(2 if group=='one' else 1)
            board=Image.new('RGB',(60+cols*60,35+n*58),(26,36,43));d=ImageDraw.Draw(board)
            for j,f in enumerate(families):
                sides=[True,False] if group=='one' else [False]
                for k,left in enumerate(sides):
                    col=j*len(sides)+k
                    d.text((60+col*60,8),f'{f+1:02}'+(' G' if left else ' D' if group=='one' else ''),fill='white')
                    for row,body in enumerate(range(start,start+n)):
                        y=35+row*58;im=compose(body,f,left)
                        board.paste(im,(60+col*60,y),im);count+=1
                        if col==0:d.text((4,y+16),f'{body+1:02}',fill='white')
            board.save(HERE/f'full-{group}-{page}.png')
    # All 49 mixed one-handed families on representative native bodies.
    for body in [0,5,9,17,22]:
        board=Image.new('RGB',(35+7*60,35+7*58),(26,36,43));d=ImageDraw.Draw(board)
        for row,a in enumerate(one):
            d.text((4,35+row*58+16),f'{a+1:02}',fill='white')
            for col,b in enumerate(one):
                if row==0:d.text((35+col*60+14,8),f'{b+1:02}',fill='white')
                im=compose(body,a,True,b);board.paste(im,(35+col*60,35+row*58),im)
        board.save(HERE/f'dual-{body+1:02}.png')
    print(f'{count} body/family/side combinations + 245 mixed pairs rendered for inspection')

if __name__=='__main__':main()
