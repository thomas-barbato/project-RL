"""Contact sheet from GIF timestamps, with native64px world crops."""
from pathlib import Path
from PIL import Image,ImageDraw
ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-51'
def at(path,frame):
    ms=round(frame*1000/24/10)*10
    with Image.open(path) as gif:
        elapsed=0
        for i in range(gif.n_frames):
            gif.seek(i);end=elapsed+gif.info['duration']
            if elapsed<=ms<end:return gif.convert('RGB').copy()
            elapsed=end
    raise AssertionError((path,frame))
def main():
    variants=['BM05-A01-HAUT','BM05-A01-DROITE','BM05-A01-GAUCHE','BM05-A01-BAS','BM03-A01-HAUT']
    frames=[26,29,31,34,38,44];board=Image.new('RGB',(1920,1760),(11,18,24));d=ImageDraw.Draw(board)
    for row,variant in enumerate(variants):
        crop=(240,144,560,472) if 'DROITE' in variant else (0,144,320,472) if 'GAUCHE' in variant else (128,216,448,544) if 'BAS' in variant else (128,16,448,344)
        for col,f in enumerate(frames):
            im=at(OUT/f'{variant}-64.gif',f);assert im.size==(576,576)
            # Source rectangle remains1:1; no enlarged32px screenshot.
            board.paste(im.crop(crop),(col*320,row*352+24))
            d.text((col*320+8,row*352+5),f'{variant}/{f}',fill=(224,231,231))
    board.save(OUT/'gif-inspection.png');at(OUT/'BM05-A01-HAUT-64.gif',31).save(OUT/'gif-native-inspection.png')
    print(OUT/'gif-inspection.png')
if __name__=='__main__':main()
