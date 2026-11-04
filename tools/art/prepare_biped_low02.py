"""Condition entire low-step demon figures with a stable upper-body anchor."""
from pathlib import Path
import hashlib
import json
import statistics
from PIL import Image
from prepare_class_lateral_r02 import center

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/biped-lateral-01/r02'


def main():
    r=json.loads((OUT/'references.json').read_text(encoding='utf-8'))[0]
    cells=[]
    for row,side in enumerate(['left','right']):
        raw=Image.open(OUT/f'MON-144-{side}-source.png').convert('RGBA')
        assert raw.getchannel('A').getextrema()[0]==0
        for col in range(9):
            x,y=col%3,col//3
            tile=raw.crop((round(x*raw.width/3),round(y*raw.height/3),round((x+1)*raw.width/3),round((y+1)*raw.height/3)))
            box=tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            assert box;cells.append((row,col,tile.crop(box)))
    scales={row:r['reference_height']/im.height for row,col,im in cells if col==8}
    top=112-r['reference_height']
    atlas=Image.new('RGBA',(1152,256));bounds=[]
    for row,col,im in cells:
        scale=scales[row]
        native=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.NEAREST)
        x=round(64-center(im)*scale);y=top
        assert 0<=x and x+native.width<=128 and top+native.height<=116
        pose=Image.new('RGBA',(128,128));pose.alpha_composite(native,(x,y))
        atlas.alpha_composite(pose,(col*128,row*128));bounds.append({'side':row,'phase':col,'top':top,'bottom':top+native.height,'x':x,'width':native.width})
    path=OUT/'MON-144-atlas.png';atlas.save(path)
    atlas.resize((2304,512),Image.Resampling.NEAREST).save(OUT/'MON-144-poses-zoom.png')
    (OUT/'manifest.json').write_text(json.dumps({'review':'BIPED-LATERAL01-r02','normal_game_enabled':False,'frame':[128,128],
        'anchor':[64,112],'alignment':'whole-figure upper-body crown and horizontal upper-body centroid, fixed scale',
        'step_ms':650,'travel_px_per_cycle':12,'bounds':bounds,'profiles':[{**r,'atlas':path.relative_to(ROOT).as_posix(),
        'sources':[(OUT/f'MON-144-{s}-source.png').relative_to(ROOT).as_posix() for s in ['left','right']],
        'scales_by_direction':scales,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}]},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Packed only MON-144: low steps and fixed whole-figure upper anchor')


if __name__=='__main__':main()
