"""Condition whole generated figures for LATERAL02; never alter accepted art."""
from pathlib import Path
import hashlib
import json
import statistics
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/source-art/animation-polish-01/lateral-identity-02'

def upper_center(image):
    pixels=image.load()
    total=weighted=0
    for y in range(round(image.height*.25)):
        for x in range(image.width):
            alpha=pixels[x,y][3]
            if alpha>=96:
                total+=alpha;weighted+=x*alpha
    return weighted/total

def prepare_r01(revision=1):
    target=OUT if revision==1 else OUT/f'r0{revision}'
    source=target/f'LATERAL02-r0{revision}-source.png'
    raw=Image.open(source).convert('RGBA')
    reference=Image.open(OUT/'reference-12.png').convert('RGBA')
    bounds=reference.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
    height=bounds[3]-bounds[1]
    cells=[]
    for row in range(2):
        for col in range(7):
            tile=raw.crop((round(col*raw.width/7),round(row*raw.height/2),round((col+1)*raw.width/7),round((row+1)*raw.height/2)))
            box=tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            assert box
            tile=tile.crop(box)
            cells.append((row,col,tile,box))
    scale=height/statistics.median(tile.height for row,col,tile,box in cells if col==6)
    targets={}
    for row,col,tile,box in cells:
        if col==6:
            native_width=round(tile.width*scale)
            targets[row]=(96-native_width)//2+upper_center(tile)*scale
    if revision==3:
        for row,direction in enumerate(['left','right']):
            idle=Image.open(OUT/f'r02/candidate-02-{direction}-6.png').convert('RGBA')
            box=idle.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
            targets[row]=box[0]+upper_center(idle.crop(box))
    records=[]
    atlas=Image.new('RGBA',(672,192))
    for row,col,tile,box in cells:
        native=tile.resize((round(tile.width*scale),round(tile.height*scale)),Image.Resampling.NEAREST)
        x=round(targets[row]-upper_center(tile)*scale)
        y=81-native.height
        assert x>=0 and x+native.width<=96 and y>=0
        frame=Image.new('RGBA',(96,96));frame.alpha_composite(native,(x,y))
        direction='left' if row==0 else 'right'
        if revision==3 and col==6:
            frame=Image.open(OUT/f'r02/candidate-02-{direction}-6.png').convert('RGBA')
        name=f'candidate-0{revision}-{direction}-{col}.png'
        frame.save(target/name);atlas.alpha_composite(frame,(col*96,row*96))
        records.append({'file':name,'row':row,'phase':col,'source_bbox':list(box),'native_bbox':frame.getchannel('A').getbbox(),'upper_anchor_x':x+upper_center(tile)*scale})
    atlas.save(target/f'candidate-0{revision}-atlas.png')
    if revision==3:
        atlas.resize((2016,576),Image.Resampling.NEAREST).save(target/'candidate-03-poses-zoom.png')
    for row in range(2):
        frames=[Image.open(target/r['file']).convert('RGBA') for r in records if r['row']==row]
        assert len({im.tobytes() for im in frames[:6]})==6
        assert len({im.crop((0,60,96,96)).tobytes() for im in frames[:6]})==6
        assert frames[6].tobytes() not in [im.tobytes() for im in frames[:6]]
        anchors=[r['upper_anchor_x'] for r in records if r['row']==row]
        assert max(anchors)-min(anchors)<1.01
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    board=Image.new('RGB',(384,134),(30,40,49));draw=ImageDraw.Draw(board)
    for col,(path,label) in enumerate([(OUT/'reference-12.png','Face validée'),(target/f'candidate-0{revision}-left-6.png',f'0{revision} / gauche'),(target/f'candidate-0{revision}-right-6.png',f'0{revision} / droite'),(OUT/'reference-09.png','Dos validé')]):
        frame=Image.open(path).convert('RGBA');board.paste(frame,(col*96,32),frame)
        draw.text((col*96+3,9),label,font=font,fill=(235,241,243))
    board.save(target/f'identity-native-0{revision}.png')
    board.resize((1536,536),Image.Resampling.NEAREST).save(target/f'identity-zoom-0{revision}.png')
    metadata={'review':f'LATERAL02-r0{revision}','class':'Traqueuse','armor_stage':3,'profile':'traqueuse-armor3','source_size':list(raw.size),'reference_opaque_height':height,'uniform_scale':scale,'sampling':'nearest','whole_figures_only':True,'baseline_y':81,'upper_anchor':'one whole-figure translation, locked to each directional idle; no limb/head edits','native_cell_px':64,'frame_canvas':[96,96],'anchor':[48,81],'distinct_walk_frames_per_direction':6,'distinct_lower_body_frames_per_direction':6,'separate_directional_idle':True,'normal_game_enabled':False,'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),'frames':records}
    if revision==3:
        metrics=[]
        for direction in ['left','right']:
            for n in range(7):
                before=Image.open(OUT/f'r02/candidate-02-{direction}-{n}.png').convert('RGBA')
                after=Image.open(target/f'candidate-03-{direction}-{n}.png').convert('RGBA')
                if n==6:
                    assert before.tobytes()==after.tobytes()
                elif n in [0,2,3,5]:
                    def span(im):
                        b=im.crop((0,60,96,83)).getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
                        return b[2]-b[0]
                    metrics.append({'direction':direction,'phase':n,'before':span(before),'after':span(after)})
        metadata['idle_reused_byte_identical']=True
        metadata['lower_body_envelope_px']=metrics
        metadata['median_envelope_reduction']=1-statistics.median(m['after']/m['before'] for m in metrics)
        assert metadata['median_envelope_reduction']>0.15
    (target/f'conditioning-r0{revision}.json').write_text(json.dumps(metadata,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(json.dumps({k:metadata[k] for k in ['review','reference_opaque_height','uniform_scale','normal_game_enabled']}))

def prepare_r02():
    target = OUT / 'r02'
    reference = Image.open(OUT / 'reference-12.png').convert('RGBA')
    bounds = reference.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
    height = bounds[3] - bounds[1]
    sources = [
        ('half-a-source.png', 4, {0: 0, 6: 3}, 3),
        ('passing-a-source.png', 2, {1: 0, 2: 1}, 0),
        ('half-b-source.png', 3, {3: 0, 4: 1, 5: 2}, 0),
    ]
    extracted = []
    calibrations = []
    for name, columns, phases, calibration_col in sources:
        source = Image.open(target / name).convert('RGBA')
        cells = {}
        for row in range(2):
            for col in range(columns):
                tile = source.crop((round(col*source.width/columns), round(row*source.height/2), round((col+1)*source.width/columns), round((row+1)*source.height/2)))
                box = tile.getchannel('A').point(lambda a:255 if a>=96 else 0).getbbox()
                assert box
                cells[row, col] = tile.crop(box)
        # Sources have different canvas resolutions: one calibration per sheet,
        # shared by all its complete figures, never a per-frame body-part edit.
        scale = height / statistics.median(cells[row, calibration_col].height for row in range(2))
        calibrations.append({'source':name,'scale':scale,'calibration_column':calibration_col,'sha256':hashlib.sha256((target/name).read_bytes()).hexdigest()})
        for row in range(2):
            for phase, col in phases.items():
                extracted.append((row,phase,cells[row,col],scale,name,col))
    anchors = {}
    for row,phase,tile,scale,name,col in extracted:
        if phase == 6:
            anchors[row] = (96-round(tile.width*scale))//2 + upper_center(tile)*scale
    records = []
    atlas = Image.new('RGBA',(672,192))
    for row,phase,tile,scale,name,col in extracted:
        native = tile.resize((round(tile.width*scale),round(tile.height*scale)),Image.Resampling.NEAREST)
        x = round(anchors[row]-upper_center(tile)*scale)
        y = 81-native.height
        assert 0 <= x and x+native.width <= 96 and y >= 0
        frame=Image.new('RGBA',(96,96));frame.alpha_composite(native,(x,y))
        filename=f'candidate-02-{"left" if row==0 else "right"}-{phase}.png'
        frame.save(target/filename);atlas.alpha_composite(frame,(phase*96,row*96))
        records.append({'file':filename,'row':row,'phase':phase,'source':name,'source_column':col,'native_bbox':frame.getchannel('A').getbbox(),'upper_anchor_x':x+upper_center(tile)*scale})
    for row in range(2):
        frames=[Image.open(target/f'candidate-02-{"left" if row==0 else "right"}-{n}.png').convert('RGBA') for n in range(7)]
        assert len({frame.tobytes() for frame in frames[:6]})==6
        assert len({frame.crop((0,60,96,96)).tobytes() for frame in frames[:6]})==6
        assert frames[6].tobytes() not in [frame.tobytes() for frame in frames[:6]]
        xs=[r['upper_anchor_x'] for r in records if r['row']==row]
        assert max(xs)-min(xs)<1.01
    atlas.save(target/'candidate-02-atlas.png')
    atlas.resize((2016,576),Image.Resampling.NEAREST).save(target/'candidate-02-poses-zoom.png')
    metadata={'review':'LATERAL02-r02','scope':'Traqueuse heavy only; Bastion unchanged','normal_game_enabled':False,'cell_px':64,'frame_canvas':[96,96],'anchor':[48,81],'sources':calibrations,'whole_figures_only':True,'sampling':'nearest','frames':sorted(records,key=lambda r:(r['row'],r['phase'])),'anatomical_alternation':'contact near -> passing far -> reach far -> contact far -> passing near -> reach near; visual review required'}
    (target/'conditioning-r02.json').write_text(json.dumps(metadata,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
    print(json.dumps({'review':metadata['review'],'frames':len(records),'reference_height':height}))

if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser();parser.add_argument('--revision',choices=['r01','r02','r03'],default='r01')
    args=parser.parse_args()
    prepare_r02() if args.revision=='r02' else prepare_r01(3 if args.revision=='r03' else 1)
