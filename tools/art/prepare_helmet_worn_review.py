"""Pack HELM05 authored viewpoints into neck-anchored native sprite layers."""
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/helmet-worn-r01'
REVIEW=ROOT/'docs/reviews/animation-polish-01'
OUT=ROOT/'artifacts/animation-polish-01/helmet-worn-r01/current'
DIRECTIONS=['front','back','left','right']


def source_cells(image):
    w,h=image.size
    boxes=[(0,0,w//2,h//2),(w//2,0,w,h//2),(0,h//2,w//2,h),(w//2,h//2,w,h)]
    result=[]
    for box in boxes:
        cell=image.crop(box)
        bbox=cell.getchannel('A').point(lambda a:255 if a>=16 else 0).getbbox()
        assert bbox,'Missing source viewpoint'
        result.append((cell.crop(bbox),bbox))
    return result


def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    records=[]
    # Physical scale is shared across the four views of each model. These are
    # review calibrations, not changes to the native64 world or collision size.
    heights=[11,13,13,13,14]
    proof=Image.new('RGB',(704,430),(30,40,49))
    draw=ImageDraw.Draw(proof)
    font=lambda size:ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)
    for n,direction in enumerate(DIRECTIONS):
        draw.text((160+n*128,12),direction,font=font(14),fill='white')
    for model in range(1,6):
        prefix=f'H05-W{model:02}'
        raw=SOURCE/f'{prefix}-views-r02.png'
        if not raw.exists():raw=SOURCE/f'{prefix}-views-r01.png'
        previous=None
        if model==2 and (SOURCE/f'{prefix}-views-r03.png').exists():
            previous=raw
            raw=SOURCE/f'{prefix}-views-r03.png'
        image=Image.open(raw).convert('RGBA')
        crops=source_cells(image)
        old_crops=source_cells(Image.open(previous).convert('RGBA')) if previous else None
        # EYE01 changes the optic, not the physical scale or the true back.
        reference_crops=old_crops if old_crops else crops
        factor=heights[model-1]/reference_crops[0][0].height
        # Generated raw sheets 01-04 placed right/left in the bottom row;
        # 05 followed the requested left/right order. Reorder, never mirror.
        order=[0,1,3,2] if model<=4 else [0,1,2,3]
        atlas=Image.new('RGBA',(128,32))
        views=[]
        for direction,index in enumerate(order):
            keep_back=old_crops is not None and direction==1
            crop,bbox=(old_crops if keep_back else crops)[index]
            size=tuple(max(1,round(v*factor)) for v in crop.size)
            assert max(size)<27,f'Unexpected source-view scale: {raw} / {size}'
            tile=crop.convert('RGBa').resize(size,Image.Resampling.LANCZOS).convert('RGBA')
            origin=(16-size[0]//2,24-size[1])
            atlas.paste(tile,(direction*32+origin[0],origin[1]))
            views.append({'direction':DIRECTIONS[direction],'source_cell':index,
                          'raw_source':(previous if keep_back else raw).relative_to(ROOT).as_posix(),
                          'source_alpha16_bbox':list(bbox),'object_size':list(size),
                          'origin':list(origin),'neck_pivot':[16,24]})
        target=SOURCE/f'{prefix}-atlas.png'
        atlas.save(target)
        draw.text((12,66+(model-1)*76),f'{model:02}',font=font(16),fill='white')
        for direction in range(4):
            tile=atlas.crop((direction*32,0,(direction+1)*32,32)).resize((96,96),Image.Resampling.NEAREST)
            proof.paste(tile,(144+direction*128,28+(model-1)*76),tile)
        records.append({'id':prefix,'raw':raw.relative_to(ROOT).as_posix(),
            'raw_sha256':hashlib.sha256(raw.read_bytes()).hexdigest(),
            'atlas':target.relative_to(ROOT).as_posix(),
            'atlas_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),
            'view_scale':factor,'front_height':heights[model-1],
            'grid':[4,1],'cell':[32,32],'neck_pivot':[16,24],
            'same_artwork_for_all_classes':True,'views':views})
    proof.save(OUT/'source-views-zoom3.png')
    (REVIEW/'HELM05-SOURCES.json').write_text(json.dumps({'review':'HELM05-WORN-r01',
        'mode':'builtin_image_gen','conditioning':'four source cells cropped by alpha16, one common scale per model, centered at lower neck pivot; no repainting or view mirroring',
        'native_world_tile_px':64,'models':records,'normal_game_enabled':False},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    return records


if __name__=='__main__':
    print(json.dumps({'models':len(prepare()),'views':20,'source':str(SOURCE)}))
