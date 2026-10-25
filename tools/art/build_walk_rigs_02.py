"""Package the accepted three-quarter drawings as independently attached limbs.

No redraw, generated character, yaw of a front sprite or weapon is used here.
The source is the unarmed painted left/right pose of each accepted outfit.
"""
import json
from pathlib import Path
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/animations/walk-04/rigs'

def rig(im, index):
    """Ownership cuts stay outside the torso; shoulders overlap the core."""
    box = im.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()
    top, bottom = box[1], box[3] - 1
    h = bottom - top
    # Locate the torso in its own painting, not at a fixed canvas centre.
    band = im.getchannel('A').crop((0, top + round(h*.23), 128, top + round(h*.34)))
    points = [(i % 128, a) for i, a in enumerate(band.tobytes()) if a >= 128]
    cx = sum(x*a for x,a in points) / sum(a for _,a in points)
    sy = top + h*.30
    py = top + h*.56
    if 15 <= index < 20:  # Open sleeves of the Arcanist, held away from torso.
        spread, palm_spread = 8., 15.
    elif 20 <= index < 25:
        spread, palm_spread = 7., 15.
    else:
        spread, palm_spread = 8., 13.
    palms = [[cx-palm_spread, py], [cx+palm_spread, py]]
    pivots = [[cx-spread, sy], [cx+spread, sy]]
    core = im.copy(); parts = []
    for side in range(2):
        px, yy = palms[side]; sx, syy = pivots[side]
        # Follow the actual outer limb to its bottom. The cut's inside edge
        # runs from the armpit to the inside of the fist, never through chest.
        edge = 0 if side == 0 else 127
        sign = 1 if side == 0 else -1
        poly = [(edge, round(syy)), (round(sx+sign*3),round(syy)),
                (round(px+sign*4),round(yy+5)), (edge,round(yy+5))]
        mask = Image.new('L', im.size); ImageDraw.Draw(mask).polygon(poly, fill=255)
        arm = Image.new('RGBA', im.size); arm.paste(im,(0,0),mask)
        core.paste((0,0,0,0),(0,0,128,128),mask)
        # Measure the painted fist from the lowest occupied rows of this arm.
        a = arm.getchannel('A'); ab = a.point(lambda v:255 if v>=96 else 0).getbbox()
        if ab:
            ys = range(max(ab[1],ab[3]-7),ab[3])
            pts = [(x,y,a.getpixel((x,y))) for y in ys for x in range(ab[0],ab[2]) if a.getpixel((x,y))>=128]
            if pts:
                px = sum(x*v for x,y,v in pts)/sum(v for x,y,v in pts)
                yy = sum(y*v for x,y,v in pts)/sum(v for x,y,v in pts)
                palms[side] = [round(px,2),round(yy,2)]
        fist = Image.new('RGBA',im.size)
        region=(round(px-4),round(yy-4),round(px+5),round(yy+5))
        fist.paste(arm.crop(region), region[:2])
        parts += [arm, fist]
    sheet = Image.new('RGBA',(640,128))
    for n,part in enumerate([core,*parts]):sheet.alpha_composite(part,(n*128,0))
    # Splitting/recombining must preserve every sleeve, hand and body pixel.
    check=core.copy()
    for arm in parts[::2]:check.alpha_composite(arm)
    assert check.tobytes()==im.tobytes(), 'ownership lost painted pixels'
    return sheet, {'palms':palms,'pivots':pivots,'hip':round(top+h*.59),
                   'bottom':bottom,'center':round(cx,2)}

def main():
    OUT.mkdir(parents=True,exist_ok=True)
    manifest=json.loads((OUT.parent/'manifest.json').read_text('utf-8'))
    records=[]
    for entry in manifest['entries']:
        key=entry['id']
        npc=key in [f'ACTOR-{i:02}' for i in range(20,26)]
        if not key.startswith(('CLASS-','ARMOR-')) and not npc:continue
        atlas=Image.open(ROOT/entry['file']).convert('RGBA')
        side=entry['side']
        if side!=128:continue
        index=int(key.split('-')[1]) if key.startswith('CLASS-') else 99
        for facing,direction in ([(-1,0),(0,1),(1,2)] if npc else [(-1,0),(1,2)]):
            im=atlas.crop((8*side,direction*side,9*side,(direction+1)*side))
            if npc:
                shifted=Image.new('RGBA',(128,128))
                shifted.alpha_composite(im,(64-entry['foot'][0],96-entry['foot'][1]));im=shifted
            front=atlas.crop((8*side,side,9*side,2*side))
            # Unarmed silhouettes have no long weapon above the head. Register
            # full visible height, rather than a vertical band that can hit a
            # shoulder in a three-quarter view (notably the Metamorph).
            threshold=lambda a:255 if a>=96 else 0
            box=im.getchannel('A').point(threshold).getbbox()
            target=front.getchannel('A').point(threshold).getbbox()
            scale=(target[3]-target[1])/(box[3]-box[1])
            if abs(scale-1)>.015:
                source=im.crop(box);source=source.resize((round(source.width*scale),target[3]-target[1]),Image.Resampling.NEAREST)
                im=Image.new('RGBA',(128,128));im.alpha_composite(source,(round((box[0]+box[2])/2-source.width/2),target[1]))
            sheet,meta=rig(im,index)
            name=f'{key}-{facing}.png';sheet.save(OUT/name)
            records.append({'id':key,'facing':facing,'file':f'assets/animations/walk-04/rigs/{name}',**meta})
    (OUT/'manifest.json').write_text(json.dumps(records,ensure_ascii=False,indent=2)+'\n','utf-8')
    print(f'{len(records)} independently attached directional rigs')

if __name__=='__main__':main()
