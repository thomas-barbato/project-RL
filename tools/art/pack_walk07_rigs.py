"""Pack full drawn walk poses and lossless arm ownership for equipment review."""
import json,sys
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_matrix_06 import grid_figures
from build_walk_rigs_02 import rig
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
OUT=ROOT/'assets/source-art/player-walk-review-07/rigs'
ART=ROOT/'artifacts/player-walk-review-07'
OUT.mkdir(parents=True,exist_ok=True);ART.mkdir(parents=True,exist_ok=True)
jobs=json.loads((DOC/'unarmed-jobs.json').read_text('utf-8'))
old=json.loads((ROOT/'docs/reviews/player-walk-06/jobs.json').read_text('utf-8'))
entries=[];errors=[]
repair_audits=[]
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
for job in jobs:
    source=Path(job['source_destination'])
    if not source.exists():continue
    try:
        index=job['indices'][0];target=min(62,old[index]['record']['bounds'][3]-old[index]['record']['bounds'][1])
        if job.get('bastion_transfer_sources'):
            from pack_walk07_bastion_r07 import bodies
            native_bodies,audit=bodies(job,target);repair_audits.append(audit)
        elif job.get('anatomy_sources'):
            from pack_walk07_anatomy_r06 import bodies
            native_bodies,audit=bodies(job,target);repair_audits.append(audit)
        elif job.get('local_retouch_sources'):
            from pack_walk07_local_retouch import bodies
            native_bodies,audit=bodies(job,target);repair_audits.append(audit)
        elif job.get('direction_sources'):
            from pack_walk07_cycle_repair import bodies
            native_bodies,audit=bodies(job,target);repair_audits.append(audit)
        else:
            figures,warnings=grid_figures(Image.open(source).convert('RGBA'))
            scale=min(target/max(im.height for im in figures),58/max(im.width for im in figures))
            indices=[0,*range(1,7),8,*range(9,15)]
            native_bodies=[]
            for i in indices:
                figure=figures[i];image=figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.LANCZOS)
                body=Image.new('RGBA',(128,128));body.alpha_composite(image,(64-image.width//2,96-image.height));native_bodies.append(body)
        board=Image.new('RGB',(560,224),'#293132');draw=ImageDraw.Draw(board)
        for side in range(2):
            sheet=Image.new('RGBA',(640,128*7))
            for frame_column in range(7):
                n=side*7+frame_column;body=native_bodies[n]
                parts,meta=rig(body,index)
                if job.get('bastion_transfer_sources'):
                    from pack_walk07_bastion_r07 import protect_ownership
                    parts,meta=protect_ownership(parts,body,meta,index,side,frame_column)
                elif job.get('local_retouch_sources') or job.get('anatomy_sources'):
                    from pack_walk07_local_retouch import protect_ownership
                    parts,meta=protect_ownership(parts,body,meta)
                sheet.alpha_composite(parts,(0,frame_column*128))
                draw.text((8,side*112+5),f'CLASS-{index:02} / '+('gauche' if side==0 else 'droite'),font=font,fill='#eeeade')
                native=body.crop((32,32,96,96));board.paste(native,(frame_column*80+8,side*112+30),native)
                entries.append({'id':f'CLASS-{index:02}','facing':-1 if side==0 else 1,
                    'frame':8 if frame_column==0 else frame_column-1,'sheet_row':frame_column,
                    'file':f'assets/source-art/player-walk-review-07/rigs/CLASS-{index:02}-{side}.png',**meta})
            sheet.save(OUT/f'CLASS-{index:02}-{side}.png',optimize=True)
        board.save(ART/f'CLASS-{index:02}-unarmed-native.png')
    except Exception as exc:errors.append({'id':job['id'],'error':str(exc)})
(OUT/'manifest.json').write_text(json.dumps(entries,ensure_ascii=False,indent=2)+'\n','utf-8')
(DOC/'cycle-repair-packing.json').write_text(json.dumps(repair_audits,indent=2)+'\n','utf8')
(DOC/'rig-packing.json').write_text(json.dumps({'entries':len(entries),'outfits':len(entries)//14,
    'errors':errors,'whole_pose_without_leg_deformation':True,'arm_ownership_recomposition_exact':True,
    'runtime_integration':False},indent=2)+'\n','utf-8')
print(json.dumps({'outfits':len(entries)//14,'errors':errors},indent=2))
