"""Animate the six corrected creatures at actual native scale for review."""
from pathlib import Path
import json
from PIL import Image,ImageOps,ImageDraw

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-11'

def main():
    refs=json.loads((ROOT/'artifacts/player-walk-review-10/full-audit/campaign/references.json').read_text())['references']
    original=json.loads((ROOT/'assets/animations/walk-07/manifest.json').read_text())['entries']
    proposed=json.loads((ROOT/'assets/animations/walk-11/creatures/manifest.json').read_text());data=[]
    for e in proposed:
        n=e['number'];old=next(x for x in original if x['number']==n);r=next(x for x in refs if x['number']==n and x['variant']==0)
        face=ImageOps.flip(Image.open(ROOT/'artifacts/player-walk-review-10/full-audit/campaign'/r['file']).convert('RGBA'))
        newface=Image.open(ROOT/f'assets/animations/walk-11/fronts/MON-{n:03}-0.png').convert('RGBA') if n in [22,24] else face
        data.append((e,old,r,face,newface,Image.open(ROOT/old['file']).convert('RGBA'),Image.open(ROOT/e['file']).convert('RGBA')))
    frames=[];durations=[]
    for phase in range(28):
        board=Image.new('RGB',(960,1040),'#253136');d=ImageDraw.Draw(board)
        d.text((8,8),'MONSTRES - taille native64 / 400ms = une case',fill='white')
        d.text((80,26),'FACE ACTUELLE',fill='white');d.text((240,26),'FACE CORRIGEE',fill='white');d.text((420,26),'MARCHE ACTUELLE',fill='white');d.text((700,26),'NOUVEL ESSAI',fill='white')
        step=phase//14;local=phase%14;moving=local<12;p=1+step*3+min(local//4,2) if moving else 0;progress=min(local/12,1)
        for row,(e,old,r,face,newface,oldatlas,newatlas) in enumerate(data):
            y=55+row*162;d.text((8,y+60),f"M{e['number']:03}",fill='white')
            for x,im,offset in [(80,face,old['front_vertical_offset']),(240,newface,e['front_vertical_offset'])]:
                yy=y+42+64-r['foot'][1]+offset;board.paste(im,(x+32-r['foot'][0],yy),im)
            for direction in range(2):
                yy=y+18+direction*72;offset=round((1-progress if direction==0 else progress)*64)
                for x,atlas,entry in [(420,oldatlas,old),(700,newatlas,e)]:
                    d.rectangle((x,yy,x+127,yy+63),outline='#46545a');im=atlas.crop((p*128,direction*128,(p+1)*128,(direction+1)*128))
                    board.paste(im,(x+offset-64,yy-48+entry['vertical_offset']),im)
        frames.append(board);durations.append([30,40,30][local%3] if moving else 260)
    frames[0].save(DOC/'Monstres-native64.gif',save_all=True,append_images=frames[1:],duration=durations,loop=0,optimize=False)
    print('Six monster comparisons saved, 400ms movement and 520ms preview rest per cell')

if __name__=='__main__':main()
