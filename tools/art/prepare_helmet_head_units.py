"""Condition complete helmeted heads, not floating shells over painted heads."""
from pathlib import Path
import hashlib
import json
import shutil
from collections import deque
from PIL import Image

ROOT=Path(__file__).resolve().parents[2]
SOURCE=ROOT/'assets/source-art/animation-polish-01/helmet-head-units-r02'
OLD=ROOT/'assets/source-art/animation-polish-01/helmet-worn-r01'
OUT=ROOT/'artifacts/animation-polish-01/helmet-worn-r01/current'
REVIEW=ROOT/'docs/reviews/animation-polish-01'


def bands(image):
    alpha=image.getchannel('A')
    rows=[sum(v>=192 for v in alpha.crop((0,y,image.width,y+1)).get_flattened_data())>=8 for y in range(image.height)]
    runs=[]; start=None
    for y,value in enumerate(rows+[False]):
        if value and start is None:start=y
        elif not value and start is not None:runs.append((start,y));start=None
    assert len(runs)==4,runs
    return [0]+[(runs[n][1]+runs[n+1][0])//2 for n in range(3)]+[image.height]


def isolate(cell, threshold=24, keep_extent=False):
    # Condition alpha only: discard disconnected matting specks outside the
    # complete head. Never repaint or change the RGB artwork.
    a=cell.getchannel('A'); pix=a.load(); remaining={(x,y) for y in range(cell.height) for x in range(cell.width) if pix[x,y]>=threshold};best=set()
    while remaining:
        p=remaining.pop();component={p};queue=deque([p])
        while queue:
            x,y=queue.popleft()
            for xx in range(x-1,x+2):
                for yy in range(y-1,y+2):
                    if (xx,yy) in remaining:
                        remaining.remove((xx,yy));component.add((xx,yy));queue.append((xx,yy))
        if len(component)>len(best):best=component
    mask=Image.new('L',cell.size);m=mask.load()
    for x,y in best:m[x,y]=pix[x,y]
    cell.putalpha(mask)
    return cell if keep_extent else cell.crop(mask.getbbox())


def prepare():
    SOURCE.mkdir(parents=True,exist_ok=True)
    before=OUT/'motion02/before'
    if not (before/'worn-tier4-front-0.png').exists():
        before.mkdir(parents=True,exist_ok=True)
        for file in OUT.glob('worn-tier*-*.png'):shutil.copy2(file,before/file.name)
    old=json.loads((REVIEW/'HELM05-SOURCES.json').read_text(encoding='utf-8'))['models']
    records=[]
    for model in range(1,6):
        atlas=Image.new('RGBA',(128,128));views=[]
        if model<3:
            raw=SOURCE/f'H02-W0{model}-heads-r{1 if model==1 else 3:02}.png'
            im=Image.open(raw).convert('RGBA');rows=bands(im)
        for cls in range(4):
            for direction in range(4):
                if model<3:
                    box=(round(direction*im.width/4),rows[cls],round((direction+1)*im.width/4),rows[cls+1])
                    cell=im.crop(box)
                else:
                    view=old[model-1]['views'][direction]
                    raw=ROOT/view['raw_source'];im_closed=Image.open(raw).convert('RGBA');index=view['source_cell'];w,h=im_closed.size
                    cell=im_closed.crop(((index%2)*w//2,(index//2)*h//2,(index%2+1)*w//2,(index//2+1)*h//2))
                crop=isolate(cell)
                if model<3:
                    pts=[x for y in range(max(0,crop.height-3),crop.height) for x in range(crop.width) if crop.getpixel((x,y))[3]>=128]
                    neck_x=sum(pts)/len(pts)+0.5
                else:
                    neck_x=crop.width*[.5,.5,.60,.40][direction]
                factor=17/crop.height;size=(max(1,round(crop.width*factor)),17)
                tile=crop.convert('RGBa').resize(size,Image.Resampling.LANCZOS).convert('RGBA')
                tile=isolate(tile,96,True)
                origin=(16-round(neck_x*factor),24-size[1])
                assert origin[0]>=0 and origin[0]+size[0]<=32
                atlas.paste(tile,(direction*32+origin[0],cls*32+origin[1]))
                views.append({'class_row':cls,'direction':['front','back','left','right'][direction],'raw':raw.relative_to(ROOT).as_posix(),'size':size,'origin':origin,'neck_pivot':[16,24]})
        target=SOURCE/f'H02-W0{model}-atlas.png';atlas.save(target)
        records.append({'model':model,'atlas':target.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'views':views})
        # Full native head grids are separate: do not overlap cells in proofs.
        atlas.resize((512,512),Image.Resampling.NEAREST).save(OUT/f'motion02/H02-W0{model}-heads-zoom4.png')
    (REVIEW/'HELM05-MOTION02-SOURCES.json').write_text(json.dumps({'review':'HELM05-MOTION02','models':records,'world_tile_px':64,'head_module_height_px':17,'native_alpha_conditioning':'single connected head component at alpha96, RGB untouched','complete_head_units':True,'normal_game_enabled':False},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'models':5,'complete_head_views':80,'baseline_sheets':len(list(before.glob('*.png')))}))


if __name__=='__main__':prepare()
