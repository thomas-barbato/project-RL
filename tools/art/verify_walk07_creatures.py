"""Check native creature packing and preserve explicit visual acceptances.

Image/canvas checks do not prove a good gait or the number of anatomical limbs.
Those remain source inspection and player review decisions.
"""
import base64,gzip,hashlib,io,json,math
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-07'
ART=ROOT/'artifacts/player-walk-review-07'
read=lambda p:json.loads(p.read_text('utf8'))
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
for acceptance in sorted(DOC.glob('visual-acceptance-*.json')):
    for entry in read(acceptance).get('files',[]):
        assert hashlib.sha256((ROOT/entry['file']).read_bytes()).hexdigest()==entry['sha256'],entry['file']
reports=[]
lots=[1]+sorted(int(p.stem.rsplit('lot',1)[1]) for p in DOC.glob('creature-jobs-lot*.json'))
for lot in lots:
    suffix='' if lot==1 else f'-lot{lot:02}'
    file=ART/f'preview-data/creatures{suffix}.json'
    if not file.exists():continue
    data=read(file);jobs=read(DOC/f'creature-jobs{suffix}.json')
    packing={p['id']:p for p in read(DOC/f'creature-packing{suffix}.json')}
    atlas=Image.open(io.BytesIO(base64.b64decode(data['atlas'].split(',')[1]))).convert('RGBA')
    reference=Image.open(io.BytesIO(base64.b64decode(data['referenceAtlas'].split(',')[1]))).convert('RGBA')
    frames=json.loads(gzip.decompress(base64.b64decode(data['framesGzip'])))
    assert len(jobs)==len(data['options'])==len(data['referenceRects']) and jobs
    board=Image.new('RGB',(896,len(jobs)*176),'#293132');draw=ImageDraw.Draw(board);cases=[]
    for n,job in enumerate(jobs):
        record=job['record'];pack=packing[job['id']]
        assert data['options'][n]['key']==job['id']
        assert hashlib.sha256(Path(job['source_destination']).read_bytes()).hexdigest()==pack['source_sha256']
        assert not pack['warnings'] and pack['used_poses']==14 and pack['uniform_scaling_only']
        if job.get('strict_source_outer_margin'):assert pack['source_opaque_outer_edge_counts']==[0,0,0,0]
        original=Image.open(DOC/'references'/f'{job["id"]}-native-current.png').convert('RGBA')
        bounds=record['bounds'];foot=record['foot'];origin=data['options'][n]['foot'];current=Image.new('RGBA',(128,128))
        ref_x=origin[0]+bounds[0]-foot[0];ref_y=origin[1]+bounds[1]-foot[1]
        assert 0<=ref_x and ref_x+bounds[2]-bounds[0]<=128 and 0<=ref_y and ref_y+bounds[3]-bounds[1]<=128,(job['id'],'native reference clipped')
        current.paste(original.crop(bounds),(ref_x,ref_y))
        rx,ry,rw,rh=data['referenceRects'][n]
        assert (rw,rh)==(128,128)
        assert reference.crop((rx,ry,rx+rw,ry+rh)).tobytes()==current.tobytes()
        assert len(frames[0][n])==14
        tiles=[]
        for pose,commands in enumerate(frames[0][n]):
            assert len(commands)==1
            tile=Image.new('RGBA',(128,128))
            for sid,x,y in commands:
                ax,ay,w,h=data['rects'][sid]
                assert x>=0 and y>=0 and x+w<=128 and y+h<=128
                tile.paste(atlas.crop((ax,ay,ax+w,ay+h)),(x,y))
            box=tile.getbbox()
            assert box and box[0]>0 and box[1]>0 and box[2]<128 and box[3]<128,(job['id'],pose,'clipped review canvas')
            tiles.append(tile)
        distinct=[]
        for side in range(2):
            distinct.append(len({t.tobytes() for t in tiles[side*7+1:side*7+7]}))
            assert distinct[-1]==6,(job['id'],side,'duplicate walking drawings')
        boxes=[t.getbbox() for t in tiles]
        union=[min(b[0] for b in boxes),min(b[1] for b in boxes),max(b[2] for b in boxes),max(b[3] for b in boxes)]
        height=union[3]-union[1]
        offset=math.floor(origin[1]-32-(union[1]+union[3])/2+.5) if height<64 else 0
        option=data['options'][n]
        assert option['cycleBounds']==pack['cycle_bounds']==union
        assert option['verticalOffset']==pack['vertical_offset']==offset
        if height<64:assert abs(32-origin[1]+offset+(union[1]+union[3])/2)<=.5,(job['id'],'not centered')
        else:assert offset==0,(job['id'],'large creature anchor changed')
        frozen_path=ROOT/f'assets/source-art/player-walk-review-07/accepted/creatures-lot{lot:02}/{job["id"]}-whole-body-native.png'
        if frozen_path.exists():
            frozen=Image.open(frozen_path).convert('RGBA')
            for pose,tile in enumerate(tiles):
                col,side=pose%7,pose//7
                assert frozen.crop((col*128,side*128,(col+1)*128,(side+1)*128)).tobytes()==tile.tobytes(),(job['id'],pose,'accepted drawing changed')
        candidate=job.get('review_id',f'WALK07-M{n+1:02}').split('-')[-1]
        draw.text((4,n*176+5),f'{candidate} actuel',font=font,fill='#eeeade');board.paste(current,(0,n*176+30),current)
        for column,(side,pose) in enumerate([(0,0),(0,2),(0,5),(1,0),(1,2),(1,5)],1):
            tile=tiles[side*7+pose];board.paste(tile,(column*128,n*176+30),tile)
            draw.text((column*128+2,n*176+5),('G' if side==0 else 'D')+(' repos' if not pose else f' phase{pose}'),font=font,fill='#eeeade')
        cases.append({'review_id':job.get('review_id',f'WALK07-M{n+1:02}'),'identity':job['id'],
                      'poses':14,'source_sha256':pack['source_sha256'],'native_height_range':pack['packed_height_range'],
                      'original_native_height':pack['original_native_height'],'distinct_walking_poses_per_direction':distinct,
                      'cycle_visible_height':height,'vertical_offset':offset,'one_fixed_offset_for_all14_poses':True,
                      'source_opaque_outer_edge_counts':pack.get('source_opaque_outer_edge_counts'),
                      'frozen_accepted_pixels_match':frozen_path.exists(),'reference_native_pixels_exact':True,
                      'native_reference_not_clipped':True,'review_origin':origin})
    board_path=ART/f'creature-lot{lot:02}-comparison-native.png';board.save(board_path)
    report={'review_id':f'WALK07-CREATURES-LOT{lot:02}','date':'2026-10-24',
            'execution':'Native image checks; controller checked separately with Node simulated DOM/canvas',
            'runtime_integration':False,'browser_or_game_execution':False,'pose_count':sum(c['poses'] for c in cases),
            'visual_acceptance_record':f'visual-acceptance-creatures-lot{lot:02}.json' if (DOC/f'visual-acceptance-creatures-lot{lot:02}.json').exists() else None,
            'anatomy_limit':'Counts, gait quality and facing consistency require visual review; dimensions and different pixels cannot prove them.',
            'native_comparison_board':board_path.relative_to(ROOT).as_posix(),'cases':cases}
    (DOC/f'creature-validation-lot{lot:02}.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n','utf8');reports.append({'lot':lot,'poses':report['pose_count'],'accepted':bool(report['visual_acceptance_record'])})
print(json.dumps({'lots':reports,'accepted_files_unchanged':True}))
