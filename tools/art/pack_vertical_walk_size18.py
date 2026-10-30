"""C05: static whole-figure calibration and coherent authored front idle.

Keep the accepted C01/C02/C03 and the rejected C04 evidence immutable. Each
authored row has its own neutral figure, used as a scale reference for that
row's three steps. No per-frame runtime zoom, limb warp or strip animation.
"""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
from PIL import Image, ImageDraw, ImageFont
from pack_vertical_walk_equipment18 import complete_figures
from pack_vertical_walk_review18 import opaque_box
from pack_vertical_walk_headgear18 import replace_head

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/vertical-walk-review-18'
OUT = ART / 'candidate-C05'
PREVIEW = ROOT / 'artifacts/vertical-walk-review-18/current/size-C05'


def structural_box(image):
    """Measure the body, excluding isolated alpha fringe pixels."""
    alpha = image.getchannel('A')
    points = {(i % image.width, i // image.width)
              for i, a in enumerate(alpha.tobytes()) if a >= 128}
    groups = []
    while points:
        seed = points.pop()
        group = [seed]
        todo = [seed]
        while todo:
            x, y = todo.pop()
            for dx, dy in ((-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)):
                neighbor = (x+dx, y+dy)
                if neighbor in points:
                    points.remove(neighbor)
                    group.append(neighbor)
                    todo.append(neighbor)
        groups.append(group)
    assert groups, 'empty figure'
    minimum = max(16, max(map(len, groups)) * .015)
    kept = [p for g in groups if len(g) >= minimum for p in g]
    return (min(x for x,y in kept), min(y for x,y in kept),
            max(x for x,y in kept)+1, max(y for x,y in kept)+1)


def condition(figure, scale, foot, structural=True):
    dimensions = (round(figure.width*scale), round(figure.height*scale))
    reduced = figure.resize(dimensions, Image.Resampling.LANCZOS)
    box = structural_box(reduced) if structural else opaque_box(reduced)
    offset = (round(64-(box[0]+box[2])/2), foot-box[3])
    canvas = Image.new('RGBA', (128,128))
    canvas.alpha_composite(reduced, offset)
    return canvas, dimensions, offset


def remap_palms(palms, old, new):
    _, od, oo = old
    _, nd, no = new
    return [[round((v-oo[k]+.5)*nd[k]/od[k]-.5+no[k],2)
             for k,v in enumerate(palm)] for palm in palms]


def corrected_back_figures():
    image = Image.open(ART/'traqueuse-lourde-back-size05-r01-source.png').convert('RGBA')
    cells = []
    for row in range(2):
        for col in range(4):
            cell = image.crop((round(col*image.width/4),round(row*image.height/2),
                               round((col+1)*image.width/4),round((row+1)*image.height/2)))
            box = opaque_box(cell)
            assert box and box[0]>0 and box[1]>0 and box[2]<cell.width and box[3]<cell.height
            whole=cell.crop(box);padded=Image.new('RGBA',(whole.width+4,whole.height+4))
            padded.alpha_composite(whole,(2,2));cells.append(padded)
    return cells


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    PREVIEW.mkdir(parents=True, exist_ok=True)
    profiles = []
    audit = []
    originals = []
    for folder in ('candidate-r01','candidate-C02','candidate-C03'):
        originals.extend(json.loads((ART/folder/'manifest.json').read_text('utf8'))['profiles'])
    for original in originals:
        identity = original['id']
        raw = Image.open(ROOT/original['source']).convert('RGBA')
        cells = complete_figures(raw)
        original_cells = cells[:]
        if identity == 'traqueuse-lourde':
            cells[8:] = corrected_back_figures()
        old_idle = Image.open(ROOT/original['standing']).convert('RGBA')
        old_box = structural_box(old_idle)
        target_height, foot = old_box[3]-old_box[1], old_box[3]
        row_scales = []
        for n in (0,4,8,12):
            b = structural_box(cells[n])
            row_scales.append(target_height/(b[3]-b[1]))
        native = [condition(cell,row_scales[n//4],foot) for n,cell in enumerate(cells)]
        sheet = Image.new('RGBA',(768,256))
        frames = []
        for old_frame in original['frames']:
            back = old_frame['direction']=='back'
            phase = old_frame['phase']
            n = (8 if back else 0) + (4 if phase>=3 else 0) + phase%3+1
            old_transform = condition(original_cells[n],original['source_scale_xy_uniform'],original['original_foot_y'],False)
            frame = deepcopy(old_frame)
            frame['palms'] = remap_palms(frame['palms'], old_transform, native[n])
            frame['box'] = list(opaque_box(native[n][0]))
            frame['structural_box'] = list(structural_box(native[n][0]))
            frames.append(frame)
            sheet.paste(native[n][0],(phase*128,int(back)*128))
        # Cell zero is an actual neutral authored pose, not the last moving frame.
        idle = native[0][0]
        idle_palms = remap_palms(frames[0]['palms'],native[1],native[0])
        path = OUT/f'{identity}.png'
        standing = OUT/f'{identity}-idle.png'
        sheet.save(path)
        idle.save(standing)
        p = deepcopy(original)
        p.update({'id':identity+'-size05','file':path.relative_to(ROOT).as_posix(),
            'standing':standing.relative_to(ROOT).as_posix(),'size_revision':True,
            'standing_palms':idle_palms,'standing_head_anchor':[64,structural_box(idle)[1]+6],
            'frames':frames,'row_scale_xy_uniform':row_scales,'target_structural_height':target_height,
            'original_idle_reference':original['standing'],'accepted_reference':original['file'],
            'source_sha256':hashlib.sha256((ROOT/original['source']).read_bytes()).hexdigest()})
        if identity == 'traqueuse-lourde':
            p['corrected_back_source'] = (ART/'traqueuse-lourde-back-size05-r01-source.png').relative_to(ROOT).as_posix()
            p['corrected_back_source_sha256'] = hashlib.sha256((ROOT/p['corrected_back_source']).read_bytes()).hexdigest()
        profiles.append(p)
        heights = [f['structural_box'][3]-f['structural_box'][1] for f in frames]
        assert max(heights)-min(heights)<=4,(identity,heights)
        assert all(abs(h-target_height)<=3 for h in heights),(identity,heights,target_height)
        audit.append({'id':identity,'target_height':target_height,'idle_height':structural_box(idle)[3]-structural_box(idle)[1],
            'front_heights':heights[:6],'back_heights':heights[6:],
            'row_scale_xy_uniform':row_scales,'whole_figure_static_conditioning':True})
    # Reuse the exact C04 modular heads and helmet designs on the corrected bodies.
    c04 = json.loads((ART/'candidate-C04/manifest.json').read_text('utf8'))
    for old in c04['profiles']:
        identity = old['id'].removesuffix('-headgear')
        base = next(p for p in profiles if p['id']==identity+'-size05')
        p = deepcopy(base)
        parts = deepcopy(old['head_parts'])
        mask = Image.open(ROOT/parts['replace_mask']).convert('RGBA')
        heads = [Image.open(ROOT/parts[k]).convert('RGBA') for k in ('front','back')]
        body = Image.open(ROOT/base['file']).convert('RGBA')
        sheet = Image.new('RGBA',body.size)
        for frame in p['frames']:
            back = frame['direction']=='back'
            phase = frame['phase']
            cell = body.crop((phase*128,back*128,(phase+1)*128,(back+1)*128))
            anchor = [64,frame['structural_box'][1]+6]
            replaced = replace_head(cell,heads[int(back)],mask,anchor)
            assert cell.crop((0,60,128,128)).tobytes()==replaced.crop((0,60,128,128)).tobytes()
            sheet.paste(replaced,(phase*128,back*128))
            frame['head_anchor'] = anchor
            frame['box'] = list(opaque_box(replaced))
        idle = Image.open(ROOT/base['standing']).convert('RGBA')
        parts['standing_anchor'] = base['standing_head_anchor']
        idle = replace_head(idle,heads[0],mask,parts['standing_anchor'])
        path = OUT/f'{identity}-headgear.png'; standing = OUT/f'{identity}-headgear-idle.png'
        sheet.save(path);idle.save(standing)
        p.update({'id':identity+'-headgear-size05','file':path.relative_to(ROOT).as_posix(),
            'standing':standing.relative_to(ROOT).as_posix(),'head_variants':True,'head_parts':parts,
            'modular_heads_reference':old['file']})
        profiles.append(p)
    manifest = {'review':'VERT18-C05','status':'review_only','normal_game_enabled':False,
        'native_tile_px':64,'whole_authored_figures_only':True,'coherent_authored_front_idle':True,'profiles':profiles}
    (OUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf8')
    (PREVIEW/'conditioning-checks.json').write_text(json.dumps({'profiles':audit,'headgear_profiles':4,
        'runtime_size_scaling':False,'accepted_files_modified':False},indent=2)+'\n','utf8')
    board=Image.new('RGB',(1050,9*150),'#1e2831');d=ImageDraw.Draw(board)
    font=ImageFont.truetype(str(ROOT/'assets/fonts/AtkinsonHyperlegible-Regular.ttf'),13)
    for row,(old,new) in enumerate(zip(originals,profiles[:9])):
        d.text((5,row*150+5),f'{row+1:02} '+old['id'],font=font,fill='white')
        original_sheet=Image.open(ROOT/old['file']).convert('RGBA');new_sheet=Image.open(ROOT/new['file']).convert('RGBA')
        for col,(label,im) in enumerate([
            ('Ancien repos',Image.open(ROOT/old['standing']).convert('RGBA')),
            ('Ancienne face',original_sheet.crop((0,0,128,128))),
            ('Ancien dos',original_sheet.crop((384,128,512,256))),
            ('C05 repos',Image.open(ROOT/new['standing']).convert('RGBA')),
            ('C05 face',new_sheet.crop((0,0,128,128))),
            ('C05 dos',new_sheet.crop((384,128,512,256))) ]):
            x=190+col*140
            d.text((x,row*150+5),label,font=font,fill='white')
            board.paste(im,(x,row*150+30),im)
    board.save(PREVIEW/'all-bodies-before-after-native64.png')
    print(json.dumps({'profiles':13,'moving_figures':156,'audit':audit}))


if __name__=='__main__':main()
