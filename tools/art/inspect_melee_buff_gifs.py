"""Mechanical native crop montage; decode GIF by elapsed time, not frame index."""
from pathlib import Path
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-50'

def gif_frame(path,n):
    wanted=round(n*1000/24/10)*10
    with Image.open(path) as gif:
        elapsed=0
        for i in range(gif.n_frames):
            gif.seek(i);end=elapsed+gif.info['duration']
            if elapsed<=wanted<end:return gif.convert('RGB').copy()
            elapsed=end
    raise AssertionError((path,wanted,elapsed))

def main():
    variants=['BM02-A01-HAUT','BM02-A01-DROITE','BM02-A01-GAUCHE','BM02-A01-BAS','BM01-A01-HAUT']
    phases=[24,26,28,31,36,47]
    board=Image.new('RGB',(320*len(phases),320*len(variants)),(11,18,24));d=ImageDraw.Draw(board)
    for row,variant in enumerate(variants):
        for col,phase in enumerate(phases):
            im=gif_frame(OUT/f'{variant}-64.gif',phase)
            assert im.size==(576,576)
            # World crop: center both actors and damage, preserving1:1 pixels.
            board.paste(im.crop((128,124,448,420)),(col*320,row*320+24))
            d.text((col*320+8,row*320+5),f'{variant} / {phase}',fill=(224,231,231))
    board.save(OUT/'native-gif-inspection.png')
    print(OUT/'native-gif-inspection.png')
if __name__=='__main__':main()
