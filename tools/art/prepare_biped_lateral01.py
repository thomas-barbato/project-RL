"""Current creature references and whole-figure packing for BIPED-LATERAL01."""
from pathlib import Path
import argparse
import hashlib
import json
import statistics
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/source-art/animation-polish-01/biped-lateral-01/r01'
DOC = ROOT / 'docs/reviews/animation-polish-01'
NUMBERS = [5, 14, 156, 144]


def prepare():
    OUT.mkdir(parents=True, exist_ok=True)
    current = json.loads((ROOT/'assets/animations/vertical-18/manifest.json').read_text(encoding='utf-8'))
    bank = json.loads((ROOT/'assets/sprites/campaign-monsters/manifest.json').read_text(encoding='utf-8'))
    candidates = json.loads((DOC/'BIPED-MOTION01-INVENTORY.json').read_text(encoding='utf-8'))['candidates']
    installed = {e['number']: e for e in bank['entries'] if e['variant'] == 0}
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    overview = Image.new('RGB', (8*160, 11*160), '#1e2831')
    audit = []
    rejected = (DOC/'BIPED-LATERAL01-DECISION-r01.json').exists()
    revision = 'r03' if (OUT.parent/'r03/manifest.json').exists() else 'r02'
    for i, row in enumerate(candidates):
        n = row['number']
        entry = next((e for e in current['monsters'] if e['number'] == n and e['variant'] == 0), None)
        live = installed.get(n)
        conflict = row.get('rig') in ['quad', 'crawl'] or 'FOUR main supporting' in row.get('motion', '')
        status = 'anatomy_conflict_requires_review' if conflict else 'pending_visual_review'
        if not live or not entry:
            status = 'not_in_current_render_bank'
        if rejected and n in NUMBERS:
            status = f'{revision}_candidate_under_review' if n == 144 else 'r01_rejected_animation'
        audit.append({**row, 'current_catalog_present': bool(live and entry), 'status': status,
                      'candidate_ready': n in NUMBERS and not rejected, 'normal_game_enabled': False})
        x, y = i % 8 * 160, i // 8 * 160
        d = ImageDraw.Draw(overview)
        d.text((x+5,y+4), f'{n:03d} / {row.get("rig")}', font=font, fill='#f3b580' if conflict else 'white')
        if entry:
            w,h=entry['cell']; sheet=Image.open(ROOT/entry['file']).convert('RGBA')
            front=sheet.crop((0,2*h,w,3*h))
            overview.paste(front,(x+(160-w)//2,y+152-h),front)
    overview.save(OUT/'current-biped-inventory.png')
    (DOC/'BIPED-LATERAL01-COVERAGE.json').write_text(json.dumps({
        'review':'BIPED-LATERAL01','date':'2026-11-02','declared_two_support_candidates':len(candidates),
        'first_review_models':NUMBERS,'anatomy_conflicts':[a['id'] for a in audit if a['status']=='anatomy_conflict_requires_review'],
        'status':f'r01_rejected_{revision}_single_demon_under_review' if rejected else 'first_morphology_batch_in_review',
        'normal_game_enabled':False,'entries':audit,
    },indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    refs=[]
    for n in NUMBERS:
        e=next(e for e in current['monsters'] if e['number']==n and e['variant']==0)
        w,h=e['cell']; sheet=Image.open(ROOT/e['file']).convert('RGBA')
        front=sheet.crop((0,2*h,w,3*h)); box=front.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
        front.save(OUT/f'MON-{n:03}-front.png')
        board=Image.new('RGB',(512,440),'#1e2831')
        zoom=front.resize((w*4,h*4),Image.Resampling.NEAREST)
        board.paste(zoom,((512-w*4)//2,435-h*4),zoom)
        label=installed[n]['display_name']
        if '\u00c3' in label:
            try: label=label.encode('latin-1').decode('utf-8')
            except (UnicodeEncodeError, UnicodeDecodeError): pass
        ImageDraw.Draw(board).text((10,8),f'MON-{n:03} / {label} / identite frontale actuelle',font=font,fill='white')
        board.save(OUT/f'MON-{n:03}-reference.png')
        refs.append({'id':f'MON-{n:03}-0','number':n,'name':label,'reference_height':box[3]-box[1],
                     'front_file':(OUT/f'MON-{n:03}-front.png').relative_to(ROOT).as_posix(),
                     'front_source':e['file'],'vertical_offset':e['offset'],
                     'source':(OUT/f'MON-{n:03}-source.png').relative_to(ROOT).as_posix()})
    (OUT/'references.json').write_text(json.dumps(refs,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(f'Current native reference inventory: {len(audit)} candidates, {len(refs)} first models')


def pack():
    profiles=[]
    for r in json.loads((OUT/'references.json').read_text(encoding='utf-8')):
        raw=Image.open(ROOT/r['source']).convert('RGBA')
        assert raw.getchannel('A').getextrema()[0]==0
        cells=[]
        for row in range(2):
            for col in range(9):
                tile=raw.crop((round(col*raw.width/9),round(row*raw.height/2),round((col+1)*raw.width/9),round((row+1)*raw.height/2)))
                box=tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
                assert box, (r['id'],row,col)
                cells.append((row,col,tile.crop(box)))
        scale=r['reference_height']/statistics.median(im.height for row,col,im in cells if col==8)
        atlas=Image.new('RGBA',(9*128,2*128))
        hashes=[]
        for row,col,im in cells:
            native=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.NEAREST)
            assert native.width<=128 and native.height<=112,(r['id'],native.size)
            pose=Image.new('RGBA',(128,128));pose.alpha_composite(native,((128-native.width)//2,112-native.height))
            atlas.alpha_composite(pose,(col*128,row*128)); hashes.append(hashlib.sha256(pose.tobytes()).hexdigest())
        assert len(set(hashes[:8]))==8 and len(set(hashes[9:17]))==8
        path=OUT/f'MON-{r["number"]:03}-atlas.png';atlas.save(path)
        atlas.resize((2304,512),Image.Resampling.NEAREST).save(OUT/f'MON-{r["number"]:03}-poses-zoom.png')
        profiles.append({**r,'atlas':path.relative_to(ROOT).as_posix(),'scale':scale,
                         'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
    (OUT/'manifest.json').write_text(json.dumps({'review':'BIPED-LATERAL01-r01','normal_game_enabled':False,
        'frame':[128,128],'anchor':[64,112],'native_cell_px':64,'walk_frames':8,'idle_frame':8,
        'step_ms':650,'phase_durations_ms':[180,180,170,120,180,180,170,120],'profiles':profiles},indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print('Packed four whole-figure monster cycles')


def pack_weapon():
    target=OUT.parents[1]/'weapon-fit-01/r01'
    raw=Image.open(target/'bastion-ready-source.png').convert('RGBA')
    cells=[]
    for row in range(2):
        for col in range(9):
            tile=raw.crop((round(col*raw.width/9),round(row*raw.height/2),round((col+1)*raw.width/9),round((row+1)*raw.height/2)))
            box=tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            assert box
            cells.append((row,col,tile.crop(box)))
    scale=58/statistics.median(im.height for row,col,im in cells if col==8)
    atlas=Image.new('RGBA',(9*128,2*128))
    from prepare_class_lateral_r02 import center
    targets={row:64 for row in range(2)}
    for row,col,im in cells:
        native=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.NEAREST)
        pose=Image.new('RGBA',(128,128))
        pose.alpha_composite(native,(round(targets[row]-center(im)*scale),112-native.height))
        atlas.alpha_composite(pose,(col*128,row*128))
    atlas.save(target/'bastion-ready-atlas.png')
    atlas.resize((2304,512),Image.Resampling.NEAREST).save(target/'bastion-ready-zoom.png')
    from PIL import ImageDraw
    grid=Image.new('RGB',(768,768),'#1e2831');d=ImageDraw.Draw(grid)
    pose=atlas.crop((0,0,128,128)).resize((768,768),Image.Resampling.NEAREST);grid.paste(pose,(0,0),pose)
    for x in range(0,128,4):d.line((x*6,0,x*6,767),fill='#50606b');d.text((x*6,0),str(x),fill='white')
    for y in range(0,128,4):d.line((0,y*6,767,y*6),fill='#50606b');d.text((0,y*6),str(y),fill='white')
    grid.save(target/'bastion-ready-anchor-grid.png')
    print('Packed whole Bastion rifle-ready poses, no independent limb deformation')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--pack',action='store_true');parser.add_argument('--pack-weapon',action='store_true');args=parser.parse_args()
    if args.prepare:prepare()
    if args.pack:pack()
    if args.pack_weapon:pack_weapon()
