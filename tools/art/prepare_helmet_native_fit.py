"""MOTION03: one reduction from authored heads to final wearer-sized pixels."""
from pathlib import Path
from functools import lru_cache
import hashlib
import json
import math
import shutil
from PIL import Image
from prepare_helmet_head_units import bands, isolate

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/helmet-head-fit-r03'
HEADS=ROOT/'assets/source-art/animation-polish-01/helmet-head-units-r02'
REVIEW=ROOT/'docs/reviews/animation-polish-01'
OUT=ROOT/'artifacts/animation-polish-01/helmet-worn-r01/current'


def height(outfit,model):
    factor=[1.08,1.,.91,.97,1.03][outfit//5]*[1.,1.035,1.07,1.105,1.14][outfit%5]
    return max(12,math.floor(12.5*factor+.5))+(model==5)


@lru_cache(None)
def source_view(model,cls,direction):
    if model>=3 and cls>0:return source_view(model,0,direction)
    if model<3:
        raw=HEADS/f'H02-W0{model}-heads-r{1 if model==1 else 3:02}.png'
        im=Image.open(raw).convert('RGBA');rows=bands(im)
        cell=im.crop((round(direction*im.width/4),rows[cls],round((direction+1)*im.width/4),rows[cls+1]))
    else:
        old=json.loads((REVIEW/'HELM05-SOURCES.json').read_text(encoding='utf-8'))['models'][model-1]['views'][direction]
        raw=ROOT/old['raw_source'];im=Image.open(raw).convert('RGBA');index=old['source_cell'];w,h=im.size
        cell=im.crop(((index%2)*w//2,(index//2)*h//2,(index%2+1)*w//2,(index//2+1)*h//2))
    crop=isolate(cell)
    if model<3:
        pts=[x for y in range(max(0,crop.height-3),crop.height) for x in range(crop.width) if crop.getpixel((x,y))[3]>=128]
        neck=sum(pts)/len(pts)+.5
    else:neck=crop.width*[.5,.5,.60,.40][direction]
    return crop,neck,raw.relative_to(ROOT).as_posix()


def prepare():
    SOURCE.mkdir(parents=True,exist_ok=True)
    proof=OUT/'motion03';proof.mkdir(exist_ok=True)
    before=proof/'before'
    if not (before/'worn-tier4-front-0.png').exists():
        before.mkdir(exist_ok=True)
        for file in OUT.glob('worn-tier*-*.png'):shutil.copy2(file,before/file.name)
    records=[]
    for model in range(1,6):
        atlas=Image.new('RGBA',(128,640));views=[]
        for cls,group in enumerate([0,2,3,4]):
            for tier in range(5):
                outfit=group*5+tier;row=cls*5+tier
                for direction in range(4):
                    crop,neck,raw=source_view(model,cls,direction)
                    h=height(outfit,model);factor=h/crop.height;size=(max(1,round(crop.width*factor)),h)
                    # Exactly one RGB resampling, from the full-resolution art.
                    tile=crop.convert('RGBa').resize(size,Image.Resampling.LANCZOS).convert('RGBA')
                    tile=isolate(tile,128,True)
                    tile.putalpha(tile.getchannel('A').point(lambda a:255 if a>=128 else 0))
                    origin=(16-round(neck*factor),24-h)
                    atlas.paste(tile,(direction*32+origin[0],row*32+origin[1]))
                    views.append({'outfit':outfit,'class_row':cls,'armor_stage':tier,'direction':['front','back','left','right'][direction],'raw':raw,'size':size,'origin':origin,'neck_pivot':[16,24]})
        file=SOURCE/f'H03-W0{model}-atlas.png';atlas.save(file)
        records.append({'model':model,'atlas':file.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(file.read_bytes()).hexdigest(),'views':views})
        for tier in range(5):
            view=Image.new('RGBA',(128,128))
            for cls in range(4):view.paste(atlas.crop((0,(cls*5+tier)*32,128,(cls*5+tier+1)*32)),(0,cls*32))
            view.resize((512,512),Image.Resampling.NEAREST).save(proof/f'H03-W0{model}-tier{tier}-heads-zoom4.png')
    data={'review':'HELM05-MOTION03','base_height_px':12.5,'final_height_range_px':[12,16],'world_tile_px':64,'resampling':'single premultiplied Lanczos reduction from original authored art to final integer size; binary alpha128; integer pixel copy at runtime','old_head_sources_preserved':True,'models':records,'normal_game_enabled':False}
    (REVIEW/'HELM05-MOTION03-SOURCES.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'models':5,'native_fitted_views':400,'runtime_resampling':False,'baseline_sheets':len(list(before.glob('*.png')))}))


if __name__=='__main__':prepare()
