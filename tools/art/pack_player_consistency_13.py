"""Condition the proposed Technomancien figures and show native64 comparisons."""
from pathlib import Path
import hashlib
import json
import statistics
from PIL import Image, ImageDraw
from build_player_walk_matrix_06 import grid_figures
from audit_walk09_body_volume import measure

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / 'assets/source-art/player-consistency-13'
DOC = ROOT / 'docs/reviews/player-consistency-13'


def box(image):
    return image.getchannel('A').point(lambda a: 255 if a >= 96 else 0).getbbox()


def assert_complete_sheet(image):
    alpha = image.getchannel('A')
    for border in [(0,0,1,image.height),(image.width-1,0,image.width,image.height),
                   (0,0,image.width,1),(0,image.height-1,image.width,image.height)]:
        assert max(alpha.crop(border).tobytes()) < 48, 'An opaque figure touches the source canvas edge'


def native(figure, scale):
    small = figure.resize((round(figure.width * scale), round(figure.height * scale)), Image.Resampling.LANCZOS)
    bounds = box(small)
    result = Image.new('RGBA', (128, 128))
    result.alpha_composite(small, (round(64 - (bounds[0]+bounds[2])/2), 96 - bounds[3]))
    # Conditioning only: discard sub-visible interpolation fringe, retain anatomy.
    result.putalpha(result.getchannel('A').point(lambda a: 0 if a < 10 else a))
    return result


def old_poses(index, manifest, bank):
    x, y = index % 16 * 128, index // 16 * 128
    result = [bank.crop((x, y, x+128, y+128))]
    for facing in [-1, 1]:
        for frame in range(6):
            e = next(e for e in manifest if e['id'] == f'CLASS-{index:02}' and e['facing'] == facing and e['frame'] == frame)
            sheet = Image.open(ROOT / e['file']).convert('RGBA'); yy = e['sheet_row'] * 128
            whole = sheet.crop((0, yy, 128, yy+128))
            for part in [1, 3]:
                whole.alpha_composite(sheet.crop((part*128, yy, (part+1)*128, yy+128)))
            result.append(whole)
    return result


def bastion_review(manifest, bank):
    proposed, previous, records = [], [], []
    for rank in [3, 4, 5]:
        source = SRC / f'BA-C13-{rank:02}-walk-r01-source.png'
        if not source.exists():
            return
        image = Image.open(source).convert('RGBA')
        assert_complete_sheet(image)
        figures, warnings = grid_figures(image, 6, 2)
        assert not warnings, warnings
        old = old_poses(rank-1, manifest, bank)
        height = measure(old[0])['height']
        scale = height / max(box(f)[3]-box(f)[1] for f in figures)
        images = [old[0]] + [native(f, scale) for f in figures]
        proposed.append(images); previous.append(old)
        sheet = Image.new('RGBA', (13*128, 128))
        for col, im in enumerate(images):sheet.paste(im,(col*128,0))
        sheet.save(SRC/f'BA-C13-{rank:02}-whole-native128.png')
        metrics = [measure(im) for im in images]
        assert max(m['height'] for m in metrics)-min(m['height'] for m in metrics) <= 3
        assert len({im.tobytes() for im in images[1:7]}) == 6
        assert len({im.tobytes() for im in images[7:]}) == 6
        records.append({'candidate':f'BA-C13-{rank:02}-r01','rank':rank,'new_whole_walking_figures':12,
                        'front_unchanged':True,'walking_scale_uniform':scale,'metrics':metrics,
                        'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest()})
    board = Image.new('RGB',(864,510),'#253136');draw=ImageDraw.Draw(board)
    draw.text((8,6),'Bastion - blindages 3/5 a 5/5 - natif64 et zoom x2',fill='white')
    for row, rank in enumerate([3,4,5]):
        y=54+row*148
        for col,(label,im) in enumerate(zip(['Face inchangee','Marche actuelle','Nouvelle gauche','Nouvelle droite'],
                    [previous[row][0],previous[row][2],proposed[row][2],proposed[row][8]])):
            x=8+col*210;tile=im.crop((32,32,96,96));draw.text((x,y-20),f'{rank}/5 {label}',fill='white')
            board.paste(tile,(x,y),tile);zoom=tile.resize((128,128),Image.Resampling.NEAREST);board.paste(zoom,(x+66,y-10),zoom)
    board.save(DOC/'Bastion-blindages-avant-apres-native64.png')
    frames=[];durations=[]
    for step,(start,end) in enumerate([(2,1),(1,0),(0,1),(1,2)]):
        direction=0 if end<start else 1
        for sample in range(11):
            moving=sample<10;phase=min(sample*3//10,2)+step%2*3;col=1+direction*6+phase if moving else 0
            x=round((start+(end-start)*sample/10)*64);board=Image.new('RGB',(540,390),'#253136');draw=ImageDraw.Draw(board)
            draw.text((8,7),'Bastion - proposition de blindage coherent - natif64',fill='white')
            draw.text((32,27),'ACTUEL',fill='white');draw.text((292,27),'CANDIDAT C13',fill='white')
            for row,rank in enumerate([3,4,5]):
                y=68+row*102;draw.text((8,y-14),f'Tenue {rank}/5',fill='white')
                for pane,collection in enumerate([previous,proposed]):
                    left=32+pane*260
                    for tile in range(3):draw.rectangle((left+tile*64,y,left+tile*64+63,y+63),outline='#46545a')
                    im=collection[row][col];board.paste(im,(left+x-32,y-32),im)
            frames.append(board);durations.append(40 if moving else 300)
    frames[0].save(DOC/'Bastion-blindages-face-marche-native64.gif',save_all=True,append_images=frames[1:],duration=durations,loop=0,optimize=False)
    return records


def main():
    manifest = json.loads((ROOT / 'assets/animations/walk-12/player/manifest.json').read_text())
    bank = Image.open(ROOT / 'assets/sprites/class-armors-01/base.png').convert('RGBA')
    proposed, previous, report = [], [], []
    for rank in range(1, 6):
        revision = 2 if rank == 1 else 1
        front_file = SRC / f'TE-C13-{rank:02}-front-r{revision:02}-source.png'
        walk_file = SRC / f'TE-C13-{rank:02}-walk-r{revision:02}-source.png'
        front = Image.open(front_file).convert('RGBA')
        walking = Image.open(walk_file).convert('RGBA')
        figures, warnings = grid_figures(walking, 6, 2)
        assert not warnings, warnings
        repair = SRC / f'TE-C13-{rank:02}-frame11-r03-source.png'
        repair_scale = None
        if repair.exists():
            replacement = Image.open(repair).convert('RGBA')
            assert_complete_sheet(replacement)
            old_box, new_box = box(figures[11]), box(replacement)
            repair_scale = (old_box[3]-old_box[1]) / (new_box[3]-new_box[1])
            figures[11] = replacement.resize((round(replacement.width*repair_scale),
                                              round(replacement.height*repair_scale)), Image.Resampling.LANCZOS)
            for figure in figures:
                assert_complete_sheet(figure)
        else:
            assert_complete_sheet(walking)
        walking_scale = 58 / max(box(f)[3]-box(f)[1] for f in figures)
        fb = box(front)
        images = [native(front, 58 / (fb[3]-fb[1]))] + [native(f, walking_scale) for f in figures]
        proposed.append(images); previous.append(old_poses(rank+4, manifest, bank))
        sheet = Image.new('RGBA', (13*128, 128))
        for col, im in enumerate(images):
            sheet.paste(im, (col*128, 0))
        sheet.save(SRC / f'TE-C13-{rank:02}-whole-native128.png')
        images[0].save(SRC / f'TE-C13-{rank:02}-front-native128.png')
        metrics = [measure(im) for im in images]
        heights = [m['height'] for m in metrics]
        assert max(heights)-min(heights) <= 2, (rank, heights)
        assert len({im.tobytes() for im in images[1:7]}) == 6
        assert len({im.tobytes() for im in images[7:]}) == 6
        assert all(im.getpixel((0, 0))[3] == 0 for im in images)
        report.append({'candidate':f'TE-C13-{rank:02}-r{revision:02}','rank':rank,'whole_figures':13,
                       'walking_scale_uniform_for_all_twelve':walking_scale,
                       'complete_figure_repair':{'file':repair.name,'pose':11,'source_resolution_scale':repair_scale} if repair_scale else None,
                       'metrics':metrics,'source_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [front_file,walk_file]+([repair] if repair_scale else [])},
                       'walk_front_upper_width_ratio':round(statistics.median(m['upper_body_mean_occupied_width'] for m in metrics[1:])/metrics[0]['upper_body_mean_occupied_width'],3)})

    board = Image.new('RGB', (864, 800), '#253136'); draw = ImageDraw.Draw(board)
    draw.text((8, 7), 'Technomancien - proposition C13 - natif64 et zoom x2', fill='white')
    for rank in range(5):
        y = 54 + rank*148
        labels = ['Face actuelle', 'Nouvelle face', 'Nouvelle gauche', 'Nouvelle droite']
        figures = [previous[rank][0], proposed[rank][0], proposed[rank][2], proposed[rank][8]]
        for col, (label, whole) in enumerate(zip(labels, figures)):
            x = 8 + col*210; tile = whole.crop((32, 32, 96, 96))
            draw.text((x, y-20),f'{rank+1}/5 {label}',fill='white')
            board.paste(tile,(x,y),tile)
            zoom=tile.resize((128,128),Image.Resampling.NEAREST)
            board.paste(zoom,(x+66,y-10),zoom)
    board.save(DOC/'Technomancien-avant-apres-native64.png')

    frames, durations = [], []
    for step, (start,end) in enumerate([(2,1),(1,0),(0,1),(1,2)]):
        direction = 0 if end < start else 1
        for sample in range(11):
            moving=sample<10;phase=min(sample*3//10,2)+step%2*3
            col=1+direction*6+phase if moving else 0
            x=round((start+(end-start)*sample/10)*64)
            board=Image.new('RGB',(540,590),'#253136');draw=ImageDraw.Draw(board)
            draw.text((8,7),'Technomancien - 400ms/case - natif64 - proposition',fill='white')
            draw.text((32,27),'ACTUEL',fill='white');draw.text((292,27),'CANDIDAT C13',fill='white')
            for rank in range(5):
                y=68+rank*102;draw.text((8,y-14),f'Tenue {rank+1}/5',fill='white')
                for pane, collection in enumerate([previous,proposed]):
                    left=32+pane*260
                    for tile in range(3):draw.rectangle((left+tile*64,y,left+tile*64+63,y+63),outline='#46545a')
                    im=collection[rank][col];board.paste(im,(left+x-32,y-32),im)
            frames.append(board);durations.append(40 if moving else 300)
    frames[0].save(DOC/'Technomancien-face-marche-avant-apres-native64.gif',save_all=True,append_images=frames[1:],duration=durations,loop=0,optimize=False)
    frames[10].save(DOC/'Technomancien-retour-face-native64.png')
    light=Image.new('RGB',board.size,'#acb3b2')
    for rank in range(5):
        for col,im in enumerate([proposed[rank][0],proposed[rank][2],proposed[rank][8]]):light.paste(im,(40+col*170,rank*110),im)
    light.save(DOC/'Technomancien-controle-fond-clair.png')
    bastion = bastion_review(manifest, bank)
    (DOC/'candidate-conditioning.json').write_text(json.dumps({'revision':'PLAYER-CONSISTENCY13-r02','status':'pending_visual_approval','normal_game_changed':False,'native_tile':64,'outfits':report,'bastion_outfits':bastion,'new_whole_figures':65+(36 if bastion else 0),'new_walking_poses':60+(36 if bastion else 0),'new_fronts':5,'movement_ms_per_cell':400,'preview_rest_ms':300,'equipment_compositor_not_changed':True,'review_images_unarmed':True},indent=2)+'\n')
    print(json.dumps({'outfits':5,'whole_figures':65,'height_ranges':[[min(m['height'] for m in r['metrics']),max(m['height'] for m in r['metrics'])] for r in report],'walk_front_upper_width_ratios':[r['walk_front_upper_width_ratio'] for r in report]}))


if __name__=='__main__':main()
