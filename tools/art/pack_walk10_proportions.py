"""Condition the Bastion proportions proposal; never overwrite installed WALK09."""
from pathlib import Path
import json,hashlib
from PIL import Image,ImageDraw
from build_player_walk_matrix_06 import grid_figures
from pack_walk09_common import protected_parts
from audit_walk09_body_volume import bounds,measure

ROOT=Path(__file__).resolve().parents[2]
SRC=ROOT/'assets/source-art/player-walk-review-10'
OUT=SRC/'candidate'
DOC=ROOT/'docs/reviews/player-walk-10'

def main():
    OUT.mkdir(exist_ok=True)
    face=Image.open(ROOT/'docs/reviews/player-walk-09/volume-audit/CLASS-00-front.png').convert('RGBA')
    target=measure(face)['height'];entries=[];proof=[]
    for side,facing in [('left',-1),('right',1)]:
        path=SRC/f'CLASS-00-{side}-proportions-r01.png'
        figures,warnings=grid_figures(Image.open(path).convert('RGBA'),4,2)
        assert len(figures)==8 and not warnings,(len(figures),warnings)
        sheet=Image.new('RGBA',(640,896));metrics=[]
        for row,fig in enumerate(figures[:7]):
            b=bounds(fig);height=target if row in [0,2,5] else target-1
            scale=height/(b[3]-b[1]);small=fig.resize((round(fig.width*scale),round(fig.height*scale)),Image.Resampling.LANCZOS)
            sb=bounds(small);native=Image.new('RGBA',(128,128))
            native.alpha_composite(small,(round(64-(sb[0]+sb[2])/2),96-sb[3]))
            parts,meta=protected_parts(native,0);sheet.paste(parts,(0,row*128))
            frame=8 if row==0 else row-1
            native.save(OUT/f'CLASS-00-{side}-{frame}.png')
            entries.append(dict(id='CLASS-00',facing=facing,frame=frame,sheet_row=row,file=f'assets/source-art/player-walk-review-10/candidate/CLASS-00-{side}-parts.png',**meta))
            m=measure(native);metrics.append(dict(frame=frame,scale_uniform_xy=scale,**m,upper_body_width_ratio=round(m['upper_body_mean_occupied_width']/measure(face)['upper_body_mean_occupied_width'],3)))
        sheet.save(OUT/f'CLASS-00-{side}-parts.png',optimize=True)
        proof.append(dict(side=side,source=str(path.relative_to(ROOT)).replace('\\','/'),sha256=hashlib.sha256(path.read_bytes()).hexdigest(),metrics=metrics))
    (OUT/'manifest.json').write_text(json.dumps(entries,indent=2)+'\n')
    (DOC/'conditioning.json').write_text(json.dumps(dict(status='preview_only',normal_game_changed=False,front_reference=measure(face),target_height=target,sources=proof),indent=2)+'\n')
    print(json.dumps(dict(target=target,source_count=2,whole_poses=len(entries),upper_body_width_ratios={p['side']:[m['upper_body_width_ratio'] for m in p['metrics']] for p in proof}),indent=2))

if __name__=='__main__':main()
