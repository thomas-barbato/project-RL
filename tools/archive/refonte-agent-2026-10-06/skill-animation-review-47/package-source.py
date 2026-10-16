"""Conditionnement mécanique de TR64-BODY05, sans retouche du dessin."""
import hashlib
import json
import re
from pathlib import Path
from PIL import Image, ImageDraw
import prepare_agent_skill_animation_review as base

ROOT = base.ROOT
OUT = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-47'
PREV = ROOT / 'artifacts/refonte-agent-2026-10-06/skill-animation-review-46'
source = OUT / 'possede-source.png'
image = Image.open(source).convert('RGBA')
assert image.getchannel('A').getextrema() == (0, 255)
box = image.getchannel('A').point(lambda a: 255 if a > 32 else 0).getbbox()
native = Image.new('RGBA', (64, 64))
# Conserver l'enveloppe occupée 42x62 et l'ancrage des pieds des précédentes revues.
native.paste(image.crop(box).resize((42, 62), Image.Resampling.LANCZOS), (11, 1))
native.save(OUT / 'possede-64.png')
prior = Image.open(PREV / 'possede-64.png').convert('RGBA')
a = [p > 128 for p in prior.getchannel('A').get_flattened_data()]
b = [p > 128 for p in native.getchannel('A').get_flattened_data()]
iou = sum(x and y for x, y in zip(a, b)) / sum(x or y for x, y in zip(a, b))
assert iou > .96, iou
board = Image.new('RGB', (700, 490), '#0b1218')
draw = ImageDraw.Draw(board)
for column, (sprite, title, variant) in enumerate([
    (prior, 'Avant · corps chair', 'TR64-BODY04 · essai précédent'),
    (native, '36 · muscles rouges', 'TR64-BODY05 · proposition')
]):
    x = 24 + column * 350
    draw.text((x, 14), title, font=base.font(19, True), fill='#e0e7e7')
    zoom = sprite.resize((192, 192), Image.Resampling.NEAREST)
    board.paste(zoom, (x + 46, 49), zoom)
    draw.text((x, 249), 'Zoom ×3 du dessin 64 px', font=base.font(14), fill='#a5b9c5')
    for i, (colour, label) in enumerate([('#111820', 'Sombre'), ('#bfc3c1', 'Clair'), ('#3e3936', 'Fond brun')]):
        tx, ty = x + 7 + i * 100, 295
        background = Image.new('RGB', (64, 64), colour)
        board.paste(background, (tx, ty))
        board.paste(sprite, (tx, ty), sprite)
        draw.text((tx, 371), label, font=base.font(13), fill='#a5b9c5')
    draw.text((x, 427), variant, font=base.font(14), fill='#e0e7e7')
draw.text((24, 463), 'Muscles rouges · tendons chair · os ivoire · yeux rouges', font=base.font(14), fill='#a5b9c5')
board.save(OUT / 'comparison-native.png')
html = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>36 · Possédé · muscles et tendons</title><style>*{box-sizing:border-box}body{max-width:1100px;margin:auto;padding:24px;background:#0b1218;color:#e0e7e7;font:16px Segoe UI,sans-serif}h1{font-size:26px}p{line-height:1.5}.muted{color:#a5b9c5}a{color:#93c1e0}.scroll{overflow:auto}.board{width:700px;height:490px;max-width:none}.source{display:block;width:640px;max-width:100%;height:auto;background:#101820}</style><h1>36 · Possédé · muscles rouges et tendons chair</h1><p class="muted">TR64-BODY05 · proposition de recoloration</p><p>Muscles rouges sur le torse, les bras et les jambes ; tendons couleur chair. Les os restent ivoire et les yeux rouges. La variante attend votre validation visuelle.</p><div class="scroll"><img class="board" src="comparison-native.png" alt="Avant et proposition, avec vues natives 64 px et zoom ×3"></div><h2>Dessin source</h2><a href="possede-source.png"><img class="source" src="possede-source.png" alt="Possédé : muscles rouges et tendons chair"></a><p class="muted">Export direct depuis le dessin haute définition, sans intermédiaire 32 px. Aucun changement aux compétences ou aux animations du jeu.</p><p><a href="possede-64.png">Dessin natif 64 px</a> · <a href="imagegen-muscles-tendons-prompt.txt">Prompt imagegen exact</a> · <a href="review.json">Fiche de revue</a></p></html>'''
(OUT / 'index.html').write_text(html, encoding='utf-8')
links = set(re.findall(r'(?:href|src)="([^"]+)"', html))
metadata = {
    'batch': 'ANIM-TR64-COLOR02', 'candidate': 36, 'variant': 'TR64-BODY05',
    'status': 'awaiting_visual_review', 'tool': 'Built-in imagegen',
    'user_request': 'est-ce que tu peux faire plutot les muscles en rouge et les tendons couleurs chaire ?',
    'palette': 'Red muscle bellies, flesh-coloured tendons, ivory bones, red eyes.',
    'previous_trial': 'TR64-BODY04 remains unapproved; preserved in review46.',
    'source_size': list(image.size), 'source_bounds': list(box), 'native_size': [64, 64],
    'native_occupied_slot': [42, 62], 'native_silhouette_iou_to_BODY04': round(iou, 5),
    'no_32px_intermediate': True, 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
    'postprocessing': 'Alpha-bound crop and direct resize only; layout labels outside artwork.',
    'campaign_integration': False, 'mechanics_or_animation_changes': False,
    'verification': 'Alpha and native size checked; native comparison visually reviewed. No runtime change.'
}
(OUT / 'review.json').write_text(json.dumps(metadata, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
assert all((OUT / link).is_file() for link in links)
(OUT / 'package-source.py').write_bytes(Path(__file__).read_bytes())
print(json.dumps({'variant': 'TR64-BODY05', 'native_silhouette_iou': iou, 'local_links': len(links)}))
