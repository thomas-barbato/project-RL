"""Package the read-only Rust compositor exports as a self-contained native review."""
from pathlib import Path
import base64
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/walk-arm-projection-14/r01'


def preview_gifs(report):
    """Lay out exact compositor frames; never change a source drawing or rig."""
    font = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 15)
    bold = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Bold.ttf'), 17)
    names = {0: 'Bastion', 10: 'Traqueuse', 15: 'Arcaniste', 20: 'Métamorphe'}
    loadouts = {'axe': 'Hache 067', 'rifle': 'Fusil 002', 'dual': 'Deux armes 068 + 101'}
    for cls, name in names.items():
        for loadout, weapon in loadouts.items():
            cases = [next(c for c in report['cases'] if c['index'] == cls+rank and c['loadout'] == loadout) for rank in range(5)]
            banks = [{v['proposed']: Image.open(OUT/v['file']).convert('RGBA').transpose(Image.Transpose.FLIP_TOP_BOTTOM) for v in c['variants']} for c in cases]
            frames, durations, detail_frames = [], [], []
            sequence = [(move, tick, False) for move in range(2) for tick in range(13)]
            for move, tick, _ in sequence:
                moving = tick < 12
                progress = (tick+1)/12 if moving else 1
                phase = min(tick//4, 2)+move*3
                board = Image.new('RGB', (1024, 684), '#26333e')
                draw = ImageDraw.Draw(board)
                draw.text((16, 10), f'{name} · {weapon} · ARM14-r01', font=bold, fill='#e7edf5')
                draw.text((16, 35), 'Taille native 64 px · G : gauche / D : droite · 1 case en 400 ms · retour de face', font=font, fill='#bdc9d5')
                centers = [138, 326, 514, 702, 890]
                for center, label in zip(centers, ['Face', 'Actuel G', 'Actuel D', 'Candidat G', 'Candidat D']):
                    draw.text((center-43, 66), label, font=bold, fill='#7dd2ff' if center > 600 else '#e7edf5')
                for rank, bank in enumerate(banks):
                    top = 96+rank*116
                    draw.text((14, top+30), f'{rank+1}/5', font=bold, fill='#e7edf5')
                    for column, (proposed, direction) in enumerate([(False, 0), (False, -1), (False, 1), (True, -1), (True, 1)]):
                        center = centers[column]
                        draw.rectangle((center-32, top+8, center+32, top+72), outline='#455a68')
                        source_column = 0 if direction == 0 or not moving else 1+(6 if direction == 1 else 0)+phase
                        sprite = bank[proposed].crop((source_column*128, 0, (source_column+1)*128, 128))
                        offset = 0 if direction == 0 else round(direction*(progress-.5)*64)
                        board.paste(sprite, (center-64+offset, top-24), sprite)
                if move == 0 and tick == 6:
                    board.save(OUT/f'{name.replace("é", "e")}-{loadout}-comparatif.png')
                frames.append(board.quantize(colors=256))
                if cls == 0 and loadout == 'axe':
                    detail = Image.new('RGB', (768, 520), '#26333e')
                    text = ImageDraw.Draw(detail)
                    text.text((16, 12), 'Bastion 1/5 · bras et prises · zoom x2', font=bold, fill='#e7edf5')
                    for column, label in enumerate(['Face', 'Actuel', 'Candidat ARM14']):
                        text.text((column*256+70, 48), label, font=bold, fill='#7dd2ff' if column==2 else '#e7edf5')
                    for row, direction in enumerate([-1, 1]):
                        top = 72+row*220
                        text.text((12, top), 'Gauche' if direction < 0 else 'Droite', font=font, fill='#bdc9d5')
                        for column in range(3):
                            facing = 0 if column == 0 else direction
                            source_column = 0 if facing == 0 or not moving else 1+(6 if facing == 1 else 0)+phase
                            sprite = banks[0][column == 2].crop((source_column*128, 0, (source_column+1)*128, 128)).resize((256,256),Image.Resampling.NEAREST)
                            offset = 0 if facing == 0 else round(facing*(progress-.5)*128)
                            text.rectangle((column*256+64, top+64, column*256+192, top+192),outline='#455a68')
                            detail.paste(sprite,(column*256+offset,top),sprite)
                    detail_frames.append(detail.quantize(colors=256))
                durations.append((30 if tick%3 else 40) if moving else 600)
            filename=f'{name.replace("é", "e")}-{loadout}-natif64.gif'
            frames[0].save(OUT/filename, save_all=True, append_images=frames[1:], duration=durations, loop=0, disposal=2, optimize=False)
            if detail_frames:
                detail_frames[0].save(OUT/'Bastion-axe-detail.gif', save_all=True, append_images=detail_frames[1:], duration=durations, loop=0, disposal=2)
                detail_frames[6].save(OUT/'Bastion-axe-detail.png')


def main():
    report = json.loads((OUT / 'review.json').read_text())
    assert len(report['cases']) == 60 and report['contact_cases'] == 720
    assert not report['contact_failures']
    records = []
    for case in report['cases']:
        record = dict(index=case['index'], loadout=case['loadout'])
        for variant in case['variants']:
            blob = (OUT / variant['file']).read_bytes()
            record['candidate' if variant['proposed'] else 'current'] = (
                'data:image/png;base64,' + base64.b64encode(blob).decode())
        records.append(record)
    template = Path(__file__).with_name('walk_arm_projection_14_review.html').read_text(encoding='utf-8')
    (OUT / 'apercu.html').write_text(template.replace('/*RENDER_DATA*/[]', json.dumps(records)), encoding='utf-8')
    preview_gifs(report)
    print(OUT / 'apercu.html')


if __name__ == '__main__':
    main()
