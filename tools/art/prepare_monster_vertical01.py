"""Whole-figure vertical review; keep the currently accepted fronts verbatim."""
import argparse
import hashlib
import json
import re
import statistics
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from prepare_class_lateral_r02 import center

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/monster-vertical-01/r01'
REV=ROOT/'docs/reviews/animation-polish-01'
NAMES={14:'Cultiste sans arme',164:'Cultiste au fusil',144:'Démon ailé au sol'}
REVIEW='MONSTER-VERTICAL01-r01'
ALIGN_SUPPORT_ROOT=False


def support_root(image):
    box=image.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
    pixels=image.load(); total=weighted=0
    for y in range(box[3]-max(1,round((box[3]-box[1])*.14)),box[3]):
        for x in range(box[0],box[2]):
            alpha=pixels[x,y][3]
            if alpha>=96:total+=alpha;weighted+=x*alpha
    return weighted/total


def references(write_inventory=True):
    OUT.mkdir(parents=True,exist_ok=True)
    vertical=json.loads((ROOT/'assets/animations/vertical-18/manifest.json').read_text(encoding='utf-8'))
    installed=json.loads((ROOT/'assets/animations/biped-lateral-01/manifest.json').read_text(encoding='utf-8'))
    metadata={}
    for path in sorted((ROOT/'docs/reviews/player-walk-07').glob('creature-jobs*.json')):
        rows=json.loads(path.read_text(encoding='utf-8-sig'))
        if not isinstance(rows,list):continue
        for r in rows:
            if isinstance(r,dict) and 'record' in r:metadata[r.get('id')]=r
    inventory=[]
    for n in sorted({p['number'] for p in vertical['monsters']}):
        variants=[p for p in vertical['monsters'] if p['number']==n]
        data=metadata.get(f'MON-{n:03}-0',{}); record=data.get('record',{})
        inventory.append({'number':n,'id':f'MON-{n:03}-0','variants':[p['variant'] for p in variants],
                          'label':data.get('label',f'MON-{n:03}'),'declared_rig':record.get('rig'),'declared_locomotor_legs':data.get('locomotor_legs'),
                          'anatomy_requires_native_review':True,'current_vertical':next(p for p in variants if p['variant']==0),
                          'status':'candidate_in_preparation' if n in NAMES else 'pending_review','new_art_accepted':False})
        if n==32:
            inventory[-1].update({'legacy_declared_rig':record.get('rig'),'declared_rig':'quadruped','declared_locomotor_legs':4,'anatomy_correction':'MONSTER-VERTICAL07-AMENDMENT-r02.json'})
    scope={'review':'MONSTER-VERTICAL01','date':'2026-11-02','scope':'all current monster models and their existing variants',
           'player_quote':'je dirais tous','models':len(inventory),'variants':sum(len(p['variants']) for p in inventory),
           'step_ms':400,'new_vertical_art_integrated':False,'first_batch':[14,164,144],
           'morphology_policy':'Ground bipeds use alternating supports; other anatomies retain their own locomotion. Metadata is not visual proof.',
           'entries':inventory}
    if write_inventory:
        (REV/'MONSTER-VERTICAL01-COVERAGE.json').write_text(json.dumps(scope,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    refs=[]
    for n,name in NAMES.items():
        v=next(v for v in vertical['monsters'] if v['number']==n and v['variant']==0)
        e=next((e for e in installed['entries'] if e['number']==n),None)
        front=Image.new('RGBA',(160,128))
        if e is not None:
            w,h=e['cell']; sheet=Image.open(ROOT/e['file']).convert('RGBA')
            old=sheet.crop((8*w,0,9*w,h)); front.alpha_composite(old,((160-w)//2,0))
            reference_kind='accepted_lateral_front'
        else:
            w,h=v['cell']; sheet=Image.open(ROOT/v['file']).convert('RGBA')
            old=sheet.crop((0,2*h,w,3*h))
            anchor_y=max(112,v['anchor'][1])
            front.alpha_composite(old,(round(80-v['anchor'][0]),round(anchor_y-v['anchor'][1])))
            reference_kind='installed_vertical_idle'
            catalog=(ROOT/'assets/sprites/campaign-monsters/catalog.rs').read_text(encoding='utf-8')
            shadow=re.search(r'MonsterArt \{ number: '+str(n)+r', variant: 0,.*?shadow: (\d+),',catalog)
            assert shadow
            e={'file':v['file'],'vertical_offset':v['offset'],'front_shadow':int(shadow[1])}
        path=OUT/f'MON-{n:03}-front.png'; front.save(path)
        box=front.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox(); figure=front.crop(box)
        figure.resize((figure.width*6,figure.height*6),Image.Resampling.NEAREST).save(OUT/f'MON-{n:03}-appearance-reference.png')
        cw,ch=v['cell']; back=Image.open(ROOT/v['file']).convert('RGBA').crop((0,ch,cw,2*ch))
        bb=back.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox(); b=back.crop(bb)
        b.resize((b.width*6,b.height*6),Image.Resampling.NEAREST).save(OUT/f'MON-{n:03}-current-back-guide.png')
        refs.append({'id':f'MON-{n:03}-0','number':n,'name':name,'frame_width':160,'frame_height':128,
                     'reference_height':box[3]-box[1],'vertical_offset':e['vertical_offset'],'idle_sprite':path.relative_to(ROOT).as_posix(),
                     'idle_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'accepted_front_source':e['file'],
                     'front_shadow':e['front_shadow'],'current_vertical':v,'front_reference_kind':reference_kind,'accepted':False})
        if reference_kind=='installed_vertical_idle' and anchor_y!=112:
            refs[-1]['atlas_anchor_y']=anchor_y
    # The installed player guides preserve their real figures and projection.
    # Repeated contact guides indicate the extra settling pose to draw; they are
    # instructions, never substituted into a candidate's walking frames.
    p=next(p for p in vertical['profiles'] if p['id']=='bastion'); f=p['families']['one']; cw,ch=f['cell']
    body=Image.open(ROOT/f['body_file']).convert('RGBA'); hands=Image.open(ROOT/f['hands_file']).convert('RGBA')
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',13)
    for side,direction in [(0,'down'),(1,'up')]:
        board=Image.new('RGB',(960,1056),'#1e2831'); draw=ImageDraw.Draw(board)
        for i,phase in enumerate([0,0,1,2,3,3,4,5,12]):
            index=phase+side*6 if phase<6 else 12
            box=((index%6)*cw,(index//6)*ch,(index%6+1)*cw,(index//6+1)*ch)
            pose=Image.alpha_composite(body.crop(box),hands.crop(box))
            bounds=pose.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox(); pose=pose.crop(bounds)
            zoom=pose.resize((pose.width*4,pose.height*4),Image.Resampling.NEAREST)
            x=i%3*320+(320-zoom.width)//2; y=i//3*352+326-zoom.height
            board.paste(zoom,(x,y),zoom)
            label=f'{i+1:02} · '+('repos frontal' if i==8 else 'contact → appui → passage → reprise')
            draw.text((i%3*320+8,i//3*352+8),label,font=font,fill='white')
        board.save(OUT/f'player-{direction}-guide.png')
    (OUT/'references.json').write_text(json.dumps(refs,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(f'Inventory: {len(inventory)} models, {scope["variants"]} variants; prepared {len(refs)} canonical appearance references')


def verify_frozen_assets():
    frozen=0
    records=['BIPED-LATERAL03-ACCEPTANCE-r03-partial.json','BIPED-LATERAL03-ACCEPTANCE-r04.json','BIPED-LATERAL04-ACCEPTANCE-r01.json']
    if (REV/'MONSTER-VERTICAL01-FEEDBACK.json').exists():records.append('MONSTER-VERTICAL01-FEEDBACK.json')
    if (REV/'MONSTER-VERTICAL02-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL02-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL-COLOR01-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL-COLOR01-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL03-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL03-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL04-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL04-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL05-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL05-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL06-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL06-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL07-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL07-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL08-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL08-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL09-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL09-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL10-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL10-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL21-QUAD01-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL21-QUAD01-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL28-WEAPON01-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL28-WEAPON01-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL11-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL11-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL12-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL12-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL13-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL13-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL14-ACCEPTANCE-r02.json').exists():records.append('MONSTER-VERTICAL14-ACCEPTANCE-r02.json')
    if (REV/'MONSTER-VERTICAL15-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL15-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL16-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL16-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL17-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL17-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL18-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL18-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL19-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL19-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL20-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL20-ACCEPTANCE-r01.json')
    if (REV/'MONSTER-VERTICAL-ALL-ACCEPTANCE-r01.json').exists():records.append('MONSTER-VERTICAL-ALL-ACCEPTANCE-r01.json')
    for name in records:
        record=json.loads((REV/name).read_text(encoding='utf-8'))
        for asset in record['assets']+record.get('front_references_retained',[])+record.get('withdrawn_assets',[])+record.get('rejected_assets',[]):
            assert hashlib.sha256((ROOT/asset['file']).read_bytes()).hexdigest()==asset['sha256']; frozen+=1
    return frozen


def main():
    frozen=verify_frozen_assets()
    profiles=[]
    for r in json.loads((OUT/'references.json').read_text(encoding='utf-8')):
        idle=Image.open(ROOT/r['idle_sprite']).convert('RGBA'); atlas=Image.new('RGBA',(1440,256)); sources=[]; bounds=[]
        assert hashlib.sha256((ROOT/r['idle_sprite']).read_bytes()).hexdigest()==r['idle_sha256']
        for side,direction in enumerate(['down','up']):
            path=OUT/f'MON-{r["number"]:03}-{direction}-source.png'; raw=Image.open(path).convert('RGBA'); sources.append(path.relative_to(ROOT).as_posix())
            broad=OUT/f'MON-{r["number"]:03}-{direction}-broad-source.png'
            if broad.exists():
                path=broad; raw=Image.open(path).convert('RGBA'); sources[-1]=path.relative_to(ROOT).as_posix()
            occlusion=OUT/f'MON-{r["number"]:03}-{direction}-occlusion-source.png'
            if occlusion.exists():
                path=occlusion; raw=Image.open(path).convert('RGBA'); sources[-1]=path.relative_to(ROOT).as_posix()
            palette_source=OUT/f'MON-{r["number"]:03}-{direction}-palette-source.png'
            if palette_source.exists():
                path=palette_source; raw=Image.open(path).convert('RGBA'); sources[-1]=path.relative_to(ROOT).as_posix()
            assert raw.getchannel('A').getextrema()[0]==0
            figures=[]
            for phase in range(8):
                x,y=phase%3,phase//3
                part=raw.crop((round(x*raw.width/3),round(y*raw.height/3),round((x+1)*raw.width/3),round((y+1)*raw.height/3)))
                box=part.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox(); assert box
                figures.append(part.crop(box))
            # One uniform scale per source sheet, never a zoom varying by phase.
            scale=r['reference_height']/statistics.median(p.height for p in figures)
            poses=[]
            for phase,figure in enumerate(figures):
                native=figure.resize((round(figure.width*scale),round(figure.height*scale)),Image.Resampling.NEAREST)
                xx=round(80-center(figure)*scale); yy=112-r['reference_height']
                assert 0<=xx and xx+native.width<=160,(r['number'],direction,phase,native.size,xx)
                pose=Image.new('RGBA',(160,128)); pose.alpha_composite(native,(xx,yy)); poses.append(pose)
            # A carried axe can bias the upper silhouette. Align the complete
            # cycle once to the retained support root, never a limb or a phase.
            shift=round(support_root(idle)-statistics.mean(support_root(pose) for pose in poses)) if ALIGN_SUPPORT_ROOT else 0
            for phase,pose in enumerate(poses):
                bb=pose.getchannel('A').getbbox(); assert bb[0]+shift>=0 and bb[2]+shift<=160
                placed=Image.new('RGBA',(160,128)); placed.alpha_composite(pose,(shift,0)); atlas.alpha_composite(placed,(phase*160,side*128))
                bounds.append({'direction':direction,'phase':phase,'whole_figure_scale':scale,'whole_cycle_translation_x':shift,'bounds':placed.getchannel('A').getbbox()})
            atlas.paste(idle,(8*160,side*128))
            assert len({atlas.crop((i*160,side*128,(i+1)*160,(side+1)*128)).tobytes() for i in range(8)})==8
        file=OUT/f'MON-{r["number"]:03}-atlas.png'; atlas.save(file)
        profiles.append({**r,'atlas':file.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(file.read_bytes()).hexdigest(),'sources':sources,'bounds':bounds,'support_root_aligned':ALIGN_SUPPORT_ROOT})
    manifest={'review':REVIEW,'normal_game_enabled':False,'native_cell_px':64,'panel_width':192,'panel_height':320,
              'step_ms':400,'cycle_ms':800,'phase_weights':[180,180,170,120]*2,'frozen_source_hashes_verified':frozen,'profiles':profiles}
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Three new vertical proposals packed; accepted fronts and runtime sprites untouched')


if __name__=='__main__':
    parser=argparse.ArgumentParser(); parser.add_argument('--references',action='store_true'); args=parser.parse_args()
    if args.references:references()
    else:main()
