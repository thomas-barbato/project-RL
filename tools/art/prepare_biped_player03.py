"""Use actual approved player drawings as the pose reference, not a new gait."""
from pathlib import Path
import argparse
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont
from prepare_class_lateral_r02 import center

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/biped-lateral-01/r03'
PLAYER=ROOT/'assets/source-art/animation-polish-01/class-lateral-02/r02/bastion-atlas.png'


def references():
    OUT.mkdir(parents=True,exist_ok=True)
    sheet=Image.open(PLAYER).convert('RGBA')
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',16)
    for row,side in enumerate(['left','right']):
        board=Image.new('RGB',(1152,1248),'#1e2831');d=ImageDraw.Draw(board)
        for phase in range(9):
            figure=sheet.crop((phase*96,row*96,(phase+1)*96,(row+1)*96))
            # Whole-figure padding only; the original player pixels stay exact.
            tile=Image.new('RGBA',(128,128));tile.alpha_composite(figure,(16,31))
            x=(phase%3)*384;y=(phase//3)*416
            zoom=tile.resize((384,384),Image.Resampling.NEAREST)
            board.paste(zoom,(x,y+32),zoom)
            d.text((x+8,y+7),f'{phase+1:02d} / Bastion / {side} / '+('repos' if phase==8 else 'pose validee'),font=font,fill='white')
        board.save(OUT/f'player-{side}-poses.png')
    refs=json.loads((ROOT/'assets/source-art/animation-polish-01/biped-lateral-01/r01/references.json').read_text(encoding='utf-8'))
    r=next(r for r in refs if r['number']==144)
    (OUT/'references.json').write_text(json.dumps([r],ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    (OUT/'player-reference.json').write_text(json.dumps({'review':'BIPED-LATERAL01-r03','file':PLAYER.relative_to(ROOT).as_posix(),
        'sha256':hashlib.sha256(PLAYER.read_bytes()).hexdigest(),'source_frame':[96,96],'source_anchor':[48,81],
        'padded_frame':[128,128],'padded_anchor':[64,112],'phase_durations_ms':[180,180,170,120,180,180,170,120],
        'review_travel_px':40,'unaltered_player_pixels':True},indent=2)+'\n',encoding='utf-8')
    print('Prepared exact current Bastion pose references, both directions')


def pack():
    r=json.loads((OUT/'references.json').read_text(encoding='utf-8'))[0]
    cells=[]
    for row,side in enumerate(['left','right']):
        raw=Image.open(OUT/f'MON-144-{side}-source.png').convert('RGBA')
        assert raw.getchannel('A').getextrema()[0]==0
        for phase in range(9):
            x,y=phase%3,phase//3
            cell=raw.crop((round(x*raw.width/3),round(y*raw.height/3),round((x+1)*raw.width/3),round((y+1)*raw.height/3)))
            box=cell.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox();assert box
            cells.append((row,phase,cell.crop(box)))
    atlas=Image.new('RGBA',(1152,256));top=112-r['reference_height'];checks=[]
    for row,phase,im in cells:
        # Condition complete figures uniformly in X/Y before the atlas is built.
        # This removes source-sheet registration/size drift; no limb is moved
        # alone, and the renderer never changes scale between animation frames.
        scale=r['reference_height']/im.height
        native=im.resize((round(im.width*scale),r['reference_height']),Image.Resampling.NEAREST)
        x=round(64-center(im)*scale)
        assert 0<=x and x+native.width<=128 and top+native.height<120
        pose=Image.new('RGBA',(128,128));pose.alpha_composite(native,(x,top));atlas.alpha_composite(pose,(phase*128,row*128))
        checks.append({'side':row,'phase':phase,'whole_figure_conditioning_scale':scale,'bounds':pose.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()})
    for row in range(2):
        assert len({atlas.crop((p*128,row*128,(p+1)*128,(row+1)*128)).tobytes() for p in range(8)})==8
    path=OUT/'MON-144-atlas.png';atlas.save(path)
    atlas.resize((2304,512),Image.Resampling.NEAREST).save(OUT/'MON-144-poses-zoom.png')
    (OUT/'manifest.json').write_text(json.dumps({'review':'BIPED-LATERAL01-r03','normal_game_enabled':False,
        'gait_source':json.loads((OUT/'player-reference.json').read_text(encoding='utf-8')),'frame':[128,128],'anchor':[64,112],
        'conditioning':'whole-figure uniform resampling to canonical native height, fixed crown and sole registration; no runtime scaling',
        'bounds':checks,'profiles':[{**r,'atlas':path.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
        'sources':[(OUT/f'MON-144-{side}-source.png').relative_to(ROOT).as_posix() for side in ['left','right']]}]},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Packed MON-144 from player-referenced full figures')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--references',action='store_true');parser.add_argument('--pack',action='store_true');args=parser.parse_args()
    if args.references:references()
    if args.pack:pack()
