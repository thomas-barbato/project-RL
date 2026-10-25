"""Pack complete monster paintings without altering individual body parts."""
from pathlib import Path
import json,hashlib
from PIL import Image,ImageDraw
from build_player_walk_matrix_06 import grid_figures
from prepare_walk11_monsters import fit
from pack_walk09_common import box

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-11'
OUT=ROOT/'assets/animations/walk-11/creatures'
SRC=ROOT/'assets/source-art/player-walk-review-11'

def main():
    jobs=json.loads((DOC/'monster-jobs.json').read_text(encoding='utf-8'))
    old=json.loads((ROOT/'assets/animations/walk-07/manifest.json').read_text())['entries']
    entries=json.loads((OUT/'manifest.json').read_text());entries=[e for e in entries if e['number'] in [22,24]]
    proof=[]
    for number in [14,15,115,124]:
        entry=dict(next(e for e in old if e['number']==number));front=Image.open(SRC/f'MON-{number:03}-front-native.png').convert('RGBA');fb=box(front);target=fb[3]-fb[1]
        packed=Image.new('RGBA',(896,256));bounds=[]
        for row,side in enumerate(['left','right']):
            j=next(j for j in jobs if j['number']==number and j['side']==side);p=Path(j['destination'])
            figures,warnings=grid_figures(Image.open(p).convert('RGBA'),4,2);assert len(figures)==8 and not warnings,(j['id'],len(figures),warnings)
            for col,im in enumerate(figures[:7]):
                desired=target if col in [0,2,5] or number==124 else target-1
                result=fit(im,desired,128,[64,112]);bounds.append(box(result));packed.paste(result,(col*128,row*128))
            proof.append(dict(id=j['id'],source=str(p.relative_to(ROOT)).replace('\\','/'),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),target_height=target,whole_figure_uniform_xy_only=True))
        entry.update(file=f'assets/animations/walk-11/creatures/MON-{number:03}-0.png',cycle_bounds=[min(b[0] for b in bounds),min(b[1] for b in bounds),max(b[2] for b in bounds),max(b[3] for b in bounds)])
        packed.save(ROOT/entry['file'],optimize=True);entries.append(entry)
    entries.sort(key=lambda e:e['number']);assert len(entries)==6
    (OUT/'manifest.json').write_text(json.dumps(entries,indent=2)+'\n')
    (DOC/'monster-conditioning.json').write_text(json.dumps(dict(status='preview_only',normal_game_changed=False,sources=proof),indent=2)+'\n')
    board=Image.new('RGB',(1100,900),'#303a40');d=ImageDraw.Draw(board)
    for row,e in enumerate(entries):
        number=e['number'];y=row*150;d.text((5,y+3),f'MON-{number:03} FRONT / CURRENT L R / CANDIDATE L R',fill='white')
        front=Image.open(ROOT/f'assets/animations/walk-11/fronts/MON-{number:03}-0.png').convert('RGBA') if number in [22,24] else Image.open(SRC/f'MON-{number:03}-front-native.png').convert('RGBA')
        b=box(front);cropped=front.crop(b);zoom=cropped.resize((cropped.width*2,cropped.height*2),Image.Resampling.NEAREST);board.paste(zoom,(40,y+147-zoom.height),zoom)
        for group,atlaspath in enumerate([next(a['file'] for a in old if a['number']==number),e['file']]):
            atlas=Image.open(ROOT/atlaspath).convert('RGBA')
            for direction in range(2):
                im=atlas.crop((0,direction*128,128,(direction+1)*128));b=box(im);im=im.crop(b).resize(((b[2]-b[0])*2,(b[3]-b[1])*2),Image.Resampling.NEAREST);board.paste(im,(240+group*400+direction*180,y+147-im.height),im)
    board.save(DOC/'monster-comparison-x2.png');print(json.dumps(dict(monsters=6,repainted_sources=8,small_cultists=2)))

if __name__=='__main__':main()
