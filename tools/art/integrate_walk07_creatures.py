"""Freeze the reviewed native bodies, apply M100/M104 repairs, then install.

No limb edits, body mirroring, palette changes or simulation changes. --install
requires the explicit authorization recorded with the exact selected pixels.
"""
import argparse,base64,gzip,hashlib,io,json,math,shutil
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from build_player_walk_matrix_06 import grid_figures

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
INT=DOC/'integration'
SRC=ROOT/'assets/source-art/player-walk-review-07'
ART=ROOT/'artifacts/player-walk-review-07'
DEST=ROOT/'assets/animations/walk-07'
QUOTE="M104 Démons et créations 106 : les déplacements sont inversés par rapport à la posture.\nM100 Possédés et hybrides 099 , la version gauche va dans la bonne direction mais n'est pas tourné vers la gauche.\nLe reste est validé, il faudra donc réparer les images que je t'ai signalé, puis supprimer les fichiers devenu inutiles et integrer au jeu :)"
read=lambda p:json.loads(p.read_text('utf8'))
write=lambda p,x:p.write_text(json.dumps(x,ensure_ascii=False,indent=2)+'\n','utf8')
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rgba=lambda im:hashlib.sha256(im.tobytes()).hexdigest()

def save_exact(path,image):
    path.parent.mkdir(parents=True,exist_ok=True)
    if path.exists():
        assert Image.open(path).convert('RGBA').tobytes()==image.tobytes(),f'Preserve existing selected pixels: {path}'
    else:image.save(path,optimize=True)

def native_entries():
    for lot in range(1,30):
        suffix='' if lot==1 else f'-lot{lot:02}'
        jobs=read(DOC/f'creature-jobs{suffix}.json')
        data=read(ART/f'preview-data/creatures{suffix}.json')
        atlas=Image.open(io.BytesIO(base64.b64decode(data['atlas'].split(',')[1]))).convert('RGBA')
        frames=json.loads(gzip.decompress(base64.b64decode(data['framesGzip'])))
        for n,job in enumerate(jobs):
            option=data['options'][n];assert option['key']==job['id']
            tiles=[]
            for commands in frames[0][n]:
                assert len(commands)==1
                sid,x,y=commands[0];ax,ay,w,h=data['rects'][sid]
                tile=Image.new('RGBA',(128,128));tile.paste(atlas.crop((ax,ay,ax+w,ay+h)),(x,y));tiles.append(tile)
            assert len(tiles)==14
            yield lot,job,option,tiles

def cycle_image(tiles):
    image=Image.new('RGBA',(896,256))
    for n,tile in enumerate(tiles):image.paste(tile,(n%7*128,n//7*128))
    return image

def prepare():
    INT.mkdir(exist_ok=True)
    entries=[];proofs=[];accepted=[];corrections=[];boards=[]
    for lot,job,option,tiles in native_entries():
        ident=job['id'];original=[im.copy() for im in tiles]
        if ident in ['MON-099-0','MON-106-0']:
            save_exact(SRC/f'integration-corrections-r07/{ident}-before-native.png',cycle_image(original))
        if ident=='MON-106-0':
            # Correct the association, retaining all exact original drawings
            # and their phase order. Reversing time would change the gait.
            tiles=tiles[7:]+tiles[:7]
            assert all(tiles[n].tobytes()==original[(n+7)%14].tobytes() for n in range(14))
        if ident=='MON-099-0':
            source=SRC/'MON-099-0-left-integration-r07.png'
            figures,warnings=grid_figures(Image.open(source).convert('RGBA'),4,2)
            assert not warnings
            figures=[im.crop(im.getbbox()) for im in figures[:7]]
            height=original[7].getbbox()[3]-original[7].getbbox()[1]
            factor=height/figures[0].height
            floor=option['foot'][1]+job['record']['bounds'][3]-job['record']['foot'][1]
            for n,figure in enumerate(figures):
                im=figure.resize((round(figure.width*factor),round(figure.height*factor)),Image.Resampling.LANCZOS)
                x=round(option['foot'][0]-im.width/2);y=floor-im.height
                assert 0<x and x+im.width<128 and 0<y and y+im.height<128
                tile=Image.new('RGBA',(128,128));tile.paste(im,(x,y));tiles[n]=tile
            assert all(tiles[n].tobytes()==original[n].tobytes() for n in range(7,14))
            corrections.append({'identity':ident,'method':'new whole LEFT figures; RIGHT native RGBA preserved exactly',
                'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
                'prompt':'docs/reviews/player-walk-07/prompts/MON-099-0-left-integration-r07.txt',
                'uniform_left_source_factor':factor,'right_native_pose_sha256':[rgba(im) for im in original[7:]],
                'right_native_unchanged':True,'per_phase_scaling':False,'procedural_limb_changes':False})
        image=cycle_image(tiles)
        if lot<=11:
            frozen=SRC/f'accepted/creatures-lot{lot:02}/{ident}-whole-body-native.png'
            assert Image.open(frozen).convert('RGBA').tobytes()==image.tobytes()
        elif ident in ['MON-099-0','MON-106-0']:
            frozen=SRC/f'integration-corrections-r07/{ident}-whole-body-native.png';save_exact(frozen,image)
        else:
            frozen=SRC/f'accepted/creatures-bulk-01/{ident}-whole-body-native.png';save_exact(frozen,image)
            accepted.append({'review_id':job['review_id'],'identity':ident,'atlas':frozen.relative_to(ROOT).as_posix()})
        boxes=[im.getbbox() for im in tiles]
        bounds=[min(b[0] for b in boxes),min(b[1] for b in boxes),max(b[2] for b in boxes),max(b[3] for b in boxes)]
        offset=math.floor(option['foot'][1]-32-(bounds[1]+bounds[3])/2+.5) if bounds[3]-bounds[1]<64 else 0
        b=job['record']['bounds'];foot=job['record']['foot']
        front_offset=math.floor(foot[1]-32-(b[1]+b[3])/2+.5) if b[3]-b[1]<64 else 0
        entry={'id':ident,'number':job['record'].get('number',0),'file':f'assets/animations/walk-07/creatures/{ident}.png',
            'side':128,'foot':option['foot'],'vertical_offset':offset,'shadow':bool(option['shadow']),
            'cycle_bounds':bounds,'front_vertical_offset':front_offset,'front_bounds':b,'front_foot':foot}
        entries.append(entry)
        proof={'id':ident,'review_id':job.get('review_id','WALK07-M06'),'lot':lot,
            'native_source':frozen.relative_to(ROOT).as_posix(),'native_source_sha256':sha(frozen),
            'rgba_sha256':rgba(image),'native_pose_sha256':[rgba(im) for im in tiles],
            'approved_pixels_unchanged':ident not in ['MON-099-0','MON-106-0']}
        proofs.append(proof)
        if ident in ['MON-099-0','MON-106-0']:boards.append((job,tiles))
    assert len(entries)==130 and len({e['id'] for e in entries})==130
    assert len(accepted)==69 and len({e['number'] for e in entries})==130
    files=[{'file':e['atlas'],'sha256':sha(ROOT/e['atlas'])} for e in accepted]
    write(DOC/'visual-acceptance-creatures-bulk-01.json',{'review_id':'WALK07-BULK-ACCEPT01','date':'2026-10-24',
        'timezone':'Europe/Paris','player_quote':QUOTE,'status':'visual_accepted','cases':accepted,'files':files,
        'excluded_reported_cases':['WALK07-M100','WALK07-M104'],'runtime_integration_authorized':True,
        'scope':'Exact69 other base cycles in groups01..09; earlier59 approvals remain unchanged; no extra palettes or legacy identities.'})
    corrections.append({'identity':'MON-106-0','method':'exchange complete directional halves, preserving every source pixel and phase order'})
    write(INT/'authorization.json',{'date':'2026-10-24','timezone':'Europe/Paris','player_quote':QUOTE,
        'runtime_integration_authorized':True,'correction_then_integration_authorized':True,
        'bulk_visual_accepted_count':69,'previously_accepted_count':59,'requested_corrections':corrections,
        'corrections_separately_visually_accepted':False,'corrected_pixels_and_scope':proofs,
        'duration_default_ms':400,'vertical_gait_changed':False,'palette_variants_newly_accepted':False})
    manifest={'format_version':1,'review_id':'WALK07-INTEGRATION01','native_tile':64,'columns':7,'rows':2,
        'layout':'row0 LEFT, row1 RIGHT; column0 idle, columns1..6 walk; three frames per cell',
        'entries':entries}
    write(INT/'prepared-runtime-manifest.json',manifest)
    board=Image.new('RGB',(896,len(boards)*304),'#293132');d=ImageDraw.Draw(board);font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',15)
    for row,(job,tiles) in enumerate(boards):
        for side in range(2):
            d.text((4,row*304+side*152+3),job['review_id']+' '+job['id']+(' / gauche' if side==0 else ' / droite'),font=font,fill='white')
            for col in range(7):board.paste(tiles[side*7+col],(col*128,row*304+side*152+22),tiles[side*7+col])
    board.save(INT/'corrections-native.png')
    return manifest,proofs

def install():
    auth=read(INT/'authorization.json');assert auth['runtime_integration_authorized'] and auth['correction_then_integration_authorized']
    manifest=read(INT/'prepared-runtime-manifest.json');proofs={e['id']:e for e in auth['corrected_pixels_and_scope']}
    for entry in manifest['entries']:
        proof=proofs[entry['id']];source=ROOT/proof['native_source']
        assert sha(source)==proof['native_source_sha256']
        target=ROOT/entry['file'];target.parent.mkdir(parents=True,exist_ok=True)
        if target.exists():assert target.read_bytes()==source.read_bytes()
        else:shutil.copyfile(source,target)
    write(DEST/'manifest.json',manifest)
    print(json.dumps({'installed_cycles':len(manifest['entries']),'native_poses':len(manifest['entries'])*14,
        'campaign_base_models':129,'palette_variants_changed':False,'pixels_match_selected_sources':True}))

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--install',action='store_true');args=parser.parse_args()
    if args.install:install()
    else:
        m,p=prepare();print(json.dumps({'prepared_cycles':len(m['entries']),'new_acceptances':69,'requested_repairs':2,'runtime_installed':False}))
