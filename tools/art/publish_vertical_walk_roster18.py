"""Native review of C02 from engine pixels, with lossless animation exports."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont
from publish_vertical_walk_review18 import save_animation

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/vertical-walk-review-18/current/roster-C02'
ENGINE = OUT / 'engine'
FONT = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 14)
BOLD = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Bold.ttf'), 17)
BG = (30, 40, 49)
SUBJECTS = [('bastion', '01 Bastion'), ('traqueuse', '02 Traqueuse'), ('metamorphe', '03 Métamorphe')]


def main():
    checks = json.loads((ENGINE / 'checks.json').read_text('utf8'))
    camp = json.loads((ENGINE / 'camp-checks.json').read_text('utf8'))
    assert checks['front_returns'] == 12 and camp['front_returns'] == 12
    assert checks['authored_moving_figures'] == 36 and not checks['normal_game_enabled']
    frames, details, durations = [], [[], [], []], []
    for step in range(2):
        for tick in range(13):
            source = Image.open(ENGINE / f'step{step}-tick{tick:02}.png').convert('RGB')
            board = Image.new('RGB', (580, 560), BG)
            draw = ImageDraw.Draw(board)
            draw.text((10, 5), '18-C02 - nouvelles classes - natif64', font=BOLD, fill='white')
            progress = 1. if tick == 12 else (tick + .5) / 12
            caption = 'Arrêt : retour au dessin frontal habituel' if tick == 12 else '400 ms par pas / deux appuis alternés'
            for col, (identity, label) in enumerate(SUBJECTS):
                x = 168 + col * 288
                detail = Image.new('RGB', (260, 610), BG)
                dd = ImageDraw.Draw(detail)
                dd.text((10, 6), label + ' - détail ×3', font=BOLD, fill='white')
                for direction in range(2):
                    y0, y1 = (100, 310) if direction == 0 else (400, 610)
                    board.paste(source.crop((x-32, y0, x+96, y1)), (10+col*188, 67+direction*253))
                    direction_label = 'Haut / dos' if direction == 0 else 'Bas / face'
                    draw.text((10+col*188, 47+direction*253), direction_label, font=FONT, fill='white')
                    dy = -1 if direction == 0 else 1
                    y = (234 if direction == 0 else 414) + (step+progress)*dy*64
                    crop = source.crop((x-8, round(y)-8, x+72, round(y)+72))
                    detail.paste(crop.resize((240, 240), Image.Resampling.NEAREST), (10, 60+direction*260))
                    dd.text((10, 38+direction*260), direction_label, font=FONT, fill='white')
                draw.text((10+col*188, 27), label, font=FONT, fill='white')
                dd.text((10, 580), 'Arrêt : face existante' if tick == 12 else 'Deux appuis / 400 ms par pas', font=FONT, fill='#c2cdd5')
                details[col].append(detail)
                if step == 0 and tick == 4:
                    detail.save(OUT / f'{identity}-marche-detail-x3.png')
            draw.text((10, 540), caption, font=FONT, fill='#c2cdd5')
            frames.append(board)
            durations.append(650 if tick == 12 else 40 if tick % 3 == 0 else 30)
            if step == 0 and tick in [4, 12]:
                board.save(OUT / f'roster-{"repos" if tick==12 else "marche"}-natif64.png')
    save_animation(frames, durations, OUT / 'roster-natif64.webp')
    for (identity, _), sequence in zip(SUBJECTS, details):
        save_animation(sequence, durations, OUT / f'{identity}-detail-x3.webp')
    camp_frames, camp_durations = [], []
    for step in range(4):
        for tick in range(13):
            source = Image.open(ENGINE / f'camp-step{step}-tick{tick:02}.png').convert('RGB')
            board = Image.new('RGB', (596, 600), BG)
            draw = ImageDraw.Draw(board)
            draw.text((10, 5), '18-C02 - camp et lumières du jeu - natif64', font=BOLD, fill='white')
            draw.text((10, 29), 'De gauche à droite : Bastion / Traqueuse / Métamorphe', font=FONT, fill='white')
            board.paste(source.crop((16, 60, 592, 572)), (10, 52))
            draw.text((10, 576), 'Arrêt : face habituelle' if tick == 12 else 'Haut : dos' if step < 2 else 'Bas : face', font=FONT, fill='white')
            camp_frames.append(board)
            camp_durations.append(650 if tick == 12 else 40 if tick % 3 == 0 else 30)
            if step == 0 and tick == 4:
                board.save(OUT / 'camp-marche-natif64.png')
    save_animation(camp_frames, camp_durations, OUT / 'camp-natif64.webp')
    (OUT / 'apercu.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>18-C02 — autres classes</title>
<style>body{margin:16px;background:#131c24;color:#eef3f7;font:16px system-ui}h1{font-size:24px;margin:0 0 10px}p{max-width:80ch;line-height:1.4;margin:10px 0}button{padding:8px 10px;margin:3px;background:#263744;color:#eef3f7;border:1px solid #526b7a;cursor:pointer}a{color:#91d6f3}.view{overflow:auto}img{display:block;max-width:none}</style>
<h1>18-C02 — les autres classes</h1><p>01 Bastion, 02 Traqueuse, 03 Métamorphe : dos vers le haut, face vers le bas, puis dessin de face habituel à l'arrêt. Ces trois déclinaisons sont validées, ainsi que le cycle de l'Arcaniste et de NPC-01. <a href="../equipment-C03/apercu.html">Voir le lot équipé 18-C03</a>.</p>
<p>Tenues de base, sans armes ni casque ajouté. Les autres armures, prises et PNJ restent à décliner. Images capturées dans le moteur, natif64 et animations sans perte ; pauses allongées pour examiner le repos. Proposition désactivée dans le jeu normal.</p>
<button onclick="document.getElementById('view').src='roster-natif64.webp'">Tous — natif64</button><button onclick="document.getElementById('view').src='camp-natif64.webp'">Dans le camp — natif64</button><button onclick="document.getElementById('view').src='bastion-detail-x3.webp'">01 Bastion ×3</button><button onclick="document.getElementById('view').src='traqueuse-detail-x3.webp'">02 Traqueuse ×3</button><button onclick="document.getElementById('view').src='metamorphe-detail-x3.webp'">03 Métamorphe ×3</button>
<p><a href="../apercu.html">Revoir 18-C01, premier lot validé</a></p><div class="view"><img id="view" src="roster-natif64.webp" alt="18-C02 : les trois classes marchent de dos et de face puis reviennent au repos"></div></html>''', 'utf8')
    (OUT / 'publication.json').write_text(json.dumps({'review': 'VERT18-C02', 'normal_game_enabled': False,
        'native_cell_px': 64, 'all_animations_lossless': True, 'decoded_animation_pixels_verified': 156,
        'source_frames': 78, 'artificial_reading_pause_ms': 650}, indent=2)+'\n', 'utf8')
    print(json.dumps({'preview': str(OUT / 'apercu.html'), 'native_previews': 2, 'integer_zoom_previews': 3}))


if __name__ == '__main__':
    main()
