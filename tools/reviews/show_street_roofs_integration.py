"""Package normal-game roof captures without resizing their pixels."""
from pathlib import Path
import hashlib
import json

from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/street-roofs-review-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-roofs-review-01'


def build():
    cap = OUT / 'engine-integrated-r01'
    checks = json.loads((cap / 'checks.json').read_text(encoding='utf-8'))
    assert checks['integrated'] and checks['generation'] == 156
    assert checks['lighting_production_enabled'] and checks['camp_round_trip']
    assert checks['keyboard_without_hover'] and checks['left_click_interaction']
    scenes = {s['scene']: s for s in checks['scenes']}
    # The distant entrance frame can already see an exterior roof edge, while
    # the room beneath it stays unknown. Do not confuse roof and ground FOV.
    assert not scenes['unknown']['home_floor_visible']
    for name, hidden, door in [('closed', False, 'Closed'), ('open', True, 'Open'),
                               ('inside-closed', True, 'Closed'), ('reclosed', False, 'Closed')]:
        assert scenes[name]['roof_hidden'] == hidden
        assert scenes[name]['door'] == f'Door({door})'
    assert not scenes['closed']['home_floor_visible']
    assert not scenes['reclosed']['home_floor_visible']
    manifest = json.loads((ROOT / 'assets/maps/prison-street-01/street.json').read_text(encoding='utf-8'))
    origin = [manifest['image_origin'][i] * 64 + manifest['image_offset_px'][i] for i in (0, 1)]
    door_in_source = [manifest['home_entry'][i] * 64 - origin[i] for i in (0, 1)]
    agreement = json.loads((DOC / 'acceptance-r01.json').read_text(encoding='utf-8'))
    for relative, expected in agreement['source_sha256'].items():
        assert hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == expected
    crops = {}
    for name, record in scenes.items():
        assert record['render_read_only']
        if name == 'unknown':
            continue
        assert record['door_cell'][2:] == [64, 64]
        frame = Image.open(cap / f'roof-{name}.png').convert('RGB')
        x, y = (round(record['door_cell'][i] - door_in_source[i]) for i in (0, 1))
        box = (x + 32, y + 16, x + 544, y + 432)
        assert box[0] >= 0 and box[1] >= 0 and box[2] <= frame.width and box[3] <= frame.height
        crops[name] = frame.crop(box)
        crops[name].save(OUT / f'home-{name}-native.png')
    # A nearby pavement patch remains in sight in both states. Neither the
    # actual door operation nor the roof switch changes its lighting.
    patch = (280, 360, 344, 392)
    assert ImageChops.difference(crops['closed'].crop(patch), crops['open'].crop(patch)).getbbox() is None
    board = Image.new('RGB', (1044, 460), '#10191e')
    font = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 18)
    for i, (name, label) in enumerate([('closed', '01A · Porte fermée'), ('open', '01A · Porte ouverte')]):
        board.paste(crops[name], (i * 532, 36))
        ImageDraw.Draw(board).text((i * 532 + 12, 6), label, font=font, fill='#dec898')
    board.save(OUT / 'roofs-comparison-native.png')
    report = dict(checks, accepted_sources_intact=True, street_light_patch_identical=True,
                  doorway_fix='DOOR-U01-FIX-01' if manifest.get('doorway') else None,
                  native_crop_size=[512, 416], normal_visibility=True,
                  capture_sha256={p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in cap.glob('*.png')})
    (DOC / 'integration-render-r01.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Toits · intégration validée</title>
<style>body{background:#10191e;color:#d7dfdc;font:17px Segoe UI,sans-serif;margin:24px}h1,h2{color:#dec898}p{max-width:960px;line-height:1.5}.scroll{overflow:auto;border:1px solid #415159;padding:8px;margin:16px 0}img{display:block;max-width:none}button{padding:10px 18px;background:#26383e;color:#dec898;border:1px solid #63747a;cursor:pointer}</style>
<h1>ROOF-U01-01 · Toits intégrés</h1><p>Dessins et fonctionnement acceptés le 19 octobre 2026.
Captures du jeu normal : vraies interactions de porte, pénombre et lumières conservées, cases de 64 px.
La découverte et la vision de la carte s'appliquent normalement ; un toit ne dévoile pas son intérieur.</p>
<div class="scroll"><img src="roofs-comparison-native.png" alt="Porte fermée puis ouverte à taille réelle"></div>
<button id="toggle">Voir la porte ouverte</button><div class="scroll"><img id="home" src="home-closed-native.png" alt="Maison à taille réelle"></div>
<h2>Porte fermée depuis l'intérieur</h2><p>Le toit reste masqué pour garder le personnage visible.</p>
<div class="scroll"><img src="home-inside-closed-native.png" alt="Personnage dedans, porte fermée"></div>
<h2>Porte refermée depuis dehors</h2><div class="scroll"><img src="home-reclosed-native.png" alt="Le toit revient après fermeture extérieure"></div>
<p>Les nouvelles parties de génération 156 possèdent cette porte. Les sauvegardes antérieures conservent leur entrée ouverte.</p>
<script>let open=false;document.getElementById('toggle').onclick=()=>{open=!open;document.getElementById('home').src=open?'home-open-native.png':'home-closed-native.png';document.getElementById('toggle').textContent=open?'Voir la porte fermée':'Voir la porte ouverte';};</script></html>'''
    (OUT / 'index.html').write_text(page, encoding='utf-8')
    if 'threshold' in crops:
        page=page.replace('<h2>Porte fermée depuis l\'intérieur</h2>',
            '<h2>Passage du seuil</h2><p>Le luminaire est fixé au mur à droite. Le passage ouvert reste libre, avec la vraie source lumineuse au même emplacement que le luminaire.</p><div class="scroll"><img src="home-threshold-native.png" alt="Passage du seuil sans luminaire devant le personnage"></div><h2>Porte fermée depuis l\'intérieur</h2>')
        (OUT / 'index.html').write_text(page,encoding='utf-8')
    print(json.dumps({k: v for k, v in report.items() if k not in ('capture_sha256', 'scenes')}, ensure_ascii=False))


if __name__ == '__main__':
    build()
