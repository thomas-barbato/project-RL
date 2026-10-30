"""Present the captured engine frames at native64 and an explicit integer zoom."""
from pathlib import Path
import json
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/vertical-walk-review-18/current'
ENGINE = OUT / 'engine'
FONT = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Regular.ttf'), 14)
BOLD = ImageFont.truetype(str(ROOT / 'assets/fonts/AtkinsonHyperlegible-Bold.ttf'), 17)
BG = (30, 40, 49)


def save_animation(frames, durations, path):
    frames[0].save(path, save_all=True, append_images=frames[1:], duration=durations,
                   loop=0, lossless=True, quality=100, method=4)
    # Compare decoded animation pixels to the actual engine presentation.
    with Image.open(path) as decoded:
        assert decoded.n_frames == len(frames)
        for n, expected in enumerate(frames):
            decoded.seek(n)
            assert decoded.convert('RGB').tobytes() == expected.convert('RGB').tobytes()


def main():
    checks = json.loads((ENGINE / 'checks.json').read_text('utf8'))
    assert checks['front_returns'] == 12
    assert not checks['normal_game_enabled']
    for player_column, name in [(0, 'sans-arme'), (1, 'equipe')]:
        native_frames, detail_frames, durations = [], [], []
        for step in range(2):
            for tick in range(13):
                source = Image.open(ENGINE / f'step{step}-tick{tick:02}.png').convert('RGB')
                board = Image.new('RGB', (580, 330), BG)
                draw = ImageDraw.Draw(board)
                draw.text((10, 6), '18-C01 - marche haut / bas - natif64', font=BOLD, fill='white')
                detail = Image.new('RGB', (520, 600), BG)
                dd = ImageDraw.Draw(detail)
                dd.text((10, 6), '18-C01 - detail moteur x3', font=BOLD, fill='white')
                progress = 1. if tick == 12 else (tick + .5) / 12
                for subject, column in enumerate([player_column, 2]):
                    for direction in range(2):
                        x = 168 + column * 288
                        y0, y1 = (80, 330) if direction == 0 else (380, 630)
                        panel = subject * 2 + direction
                        board.paste(source.crop((x-32, y0, x+96, y1)), (10+panel*140, 47))
                        label = ('Joueur' if subject == 0 else 'PNJ') + (' haut / dos' if direction == 0 else ' bas / face')
                        draw.text((10+panel*140, 30), label, font=FONT, fill='white')
                        dy = -1 if direction == 0 else 1
                        y = (234 if direction == 0 else 414) + (step+progress)*dy*64
                        crop = source.crop((x-8, round(y)-8, x+72, round(y)+72))
                        detail.paste(crop.resize((240, 240), Image.Resampling.NEAREST),
                                     (10+subject*260, 60+direction*260))
                        dd.text((10+subject*260, 38+direction*260), label, font=FONT, fill='white')
                caption = 'Arret : retour au dessin de face habituel' if tick == 12 else 'Deux vrais cycles dessines / un pas = 400 ms'
                draw.text((10, 310), caption, font=FONT, fill='#c2cdd5')
                dd.text((10, 578), caption, font=FONT, fill='#c2cdd5')
                native_frames.append(board)
                detail_frames.append(detail)
                durations.append(650 if tick == 12 else 40 if tick % 3 == 0 else 30)
                if step == 0 and tick in [4, 12]:
                    detail.save(OUT / f'{name}-{"repos" if tick==12 else "marche"}-detail-x3.png')
        save_animation(native_frames, durations, OUT / f'{name}-natif64.webp')
        save_animation(detail_frames, durations, OUT / f'{name}-detail-x3.webp')
    camp_checks = json.loads((ENGINE / 'camp-checks.json').read_text('utf8'))
    assert camp_checks['front_returns'] == 8
    assert camp_checks['moving_and_idle_actor_gain'] == 1.48
    camp_frames, camp_durations = [], []
    for step in range(4):
        for tick in range(13):
            source = Image.open(ENGINE / f'camp-step{step}-tick{tick:02}.png').convert('RGB')
            board = Image.new('RGB', (596, 582), BG)
            draw = ImageDraw.Draw(board)
            draw.text((10, 5), '18-C01 - camp et lumieres du jeu - natif64', font=BOLD, fill='white')
            board.paste(source.crop((16, 60, 592, 572)), (10, 34))
            caption = 'Arret : dessin de face habituel' if tick == 12 else 'Vers le haut : de dos' if step < 2 else 'Vers le bas : de face'
            draw.text((10, 557), caption, font=FONT, fill='white')
            # GIF's 256 colors can discard the few red coat pixels among a
            # detailed floor. Lossless animated WebP preserves the engine ink.
            camp_frames.append(board)
            camp_durations.append(650 if tick == 12 else 40 if tick % 3 == 0 else 30)
            if step == 0 and tick in [4, 12]:
                board.save(OUT / f'camp-{"repos" if tick==12 else "marche"}-natif64.png')
    save_animation(camp_frames, camp_durations, OUT / 'camp-natif64.webp')
    html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>18-C01 - marche verticale</title>
<style>body{margin:16px;background:#131c24;color:#eef3f7;font:16px system-ui}h1{font-size:24px;margin:0 0 10px}p{max-width:80ch;line-height:1.4;margin:10px 0}button{padding:9px 12px;margin:3px;background:#263744;color:#eef3f7;border:1px solid #526b7a;cursor:pointer}.view{overflow:auto}img{display:block;max-width:none}</style>
<h1>18-C01 — marche verticale</h1><p>Vers le haut : de dos. Vers le bas : de face. À chaque arrêt, retour au dessin de face habituel. Les images viennent du moteur, avec un pas de 400 ms ; les pauses prolongées servent à lire l'aperçu.</p>
<p>Premier lot validé : Arcaniste, sa version équipée et garde du camp. Les autres tenues et PNJ restent à décliner. Prototype désactivé dans le jeu normal. <a style="color:#91d6f3" href="roster-C02/apercu.html">Voir le nouveau lot 18-C02</a>.</p>
<button onclick="document.getElementById('view').src='sans-arme-natif64.webp'">Natif64 — joueur et PNJ</button><button onclick="document.getElementById('view').src='equipe-natif64.webp'">Natif64 — avec les armes</button><button onclick="document.getElementById('view').src='equipe-detail-x3.webp'">Détail moteur ×3</button><button onclick="document.getElementById('view').src='camp-natif64.webp'">Dans le camp — natif64</button>
<div class="view"><img id="view" src="equipe-natif64.webp" alt="18-C01 : joueur et PNJ marchent de dos et de face puis reviennent au repos de face"></div></html>'''
    (OUT / 'apercu.html').write_text(html, encoding='utf8')
    (OUT / 'publication.json').write_text(json.dumps({'review': 'VERT18-C01', 'normal_game_enabled': False,
        'source': 'engine/checks.json', 'native_cell_px': 64, 'detail_zoom': 3,
        'scene_source': 'engine/camp-checks.json', 'no_resampling_in_native_preview': True,
        'all_animations_lossless': True, 'decoded_animation_pixels_verified': True,
        'artificial_reading_pause_ms': 650}, indent=2)+'\n', encoding='utf8')
    print(json.dumps({'preview': str(OUT / 'apercu.html'), 'native_previews': 3, 'integer_zoom_previews': 2}))


if __name__ == '__main__':
    main()
