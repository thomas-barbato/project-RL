from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
REVIEW = OUT.parent / 'skill-icon-review-43'
captures = json.loads((OUT / 'capture-result.json').read_text(encoding='utf-8-sig'))
review = json.loads((REVIEW / 'review.json').read_text(encoding='utf-8'))
assert review['status'] == 'accepted'
images = {}
for capture in captures:
    assert capture['process_exit'] == 0
    image = Image.open(capture['file']).convert('RGB')
    assert image.size == (960, 540)
    images[capture['skill']] = image
expected = {'AF06': ('AF06', 367, 270), 'B01': ('B01', 239, 122),
            'BD04': ('BD04', 239, 122), 'BM01': ('BM01', 239, 122),
            'BM03': ('BM01', 495, 122), 'BM05': ('BM01', 239, 270)}
matches = []
for candidate in review['candidates']:
    assert candidate['visual_status'] == 'accepted'
    source = Image.open(REVIEW / candidate['files']['64']['file']).convert('RGB')
    scene, x, y = expected[candidate['id']]
    crop = images[scene].crop((x, y, x + 64, y + 64))
    error = sum(ImageStat.Stat(ImageChops.difference(crop, source)).mean) / 3
    assert error < 15, (candidate['id'], error)
    crop.save(OUT / (candidate['id'] + '-native-icon.png'))
    matches.append({'skill': candidate['id'], 'variant': candidate['variant'],
                    'scene': scene, 'position': [x, y], 'mean_channel_error': round(error, 3),
                    'tolerance': 15, 'accepted_source_match': True})
result = {'native_dimensions': [960, 540], 'matches': matches, 'capture_count': 4, 'all_process_exits': 0}
(OUT / 'native-icon-match.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
sections = ''.join('<h2>' + name + '</h2><div class="capture"><img src="' + skill + '-960/cold-start.png" width="960" height="540" alt="Capture réelle des compétences ' + name + '"></div>'
                   for skill, name in [('B01', 'Mêlée'), ('AF06', 'Affaiblissements'), ('BM01', 'Buffs de mêlée'), ('BD04', 'Buffs à distance')])
(OUT / 'index.html').write_text('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Icônes 72–77 intégrées · jeu</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}h2{font-size:18px;margin-top:28px}p{color:#a6b6c1;line-height:1.6}a{color:#b49ce5}.capture{overflow-x:auto}img{display:block;width:960px;height:540px}</style><main><h1>72–77 · Six icônes intégrées</h1><p>Les six dessins exacts validés sont utilisés sans retouche.<br>Captures réelles du jeu à 960 × 540 pixels. <a href="../skill-icon-review-44/index.html">Voir les propositions Armes lourdes</a></p>''' + sections + '</main></html>', encoding='utf-8')
(OUT / 'README.md').write_text('''# SKILLS-NATIVE17 · Mêlée, affaiblissements et buffs

Le joueur valide « ok on peut continuer » en réponse à la planche IC43 présentant les dessins 72–77. Ces six compactes sont intégrées sans retouche depuis leurs exports 64 px validés. Aucun changement de texte, règle, animation ou type de dégâts.

Accord dans ../skill-icon-review-43/visual-acceptance-01.json. La banque compte 75 compactes et 16 illustrations pour 94 compétences. integration-validation.json contrôle les 30 exports, les six masters, les fichiers actifs exacts et la préservation de 703 fichiers antérieurs.

Tests ciblés dans tests.log, compilation dans build.log, format dans format.log. Quatre captures réelles dans B01-960/, AF06-960/, BM01-960/ et BD04-960/ ; correspondance des six icônes contrôlée dans native-icon-match.json. Les propositions Armes lourdes 78–83 d’IC44 attendent leur propre validation.
''', encoding='utf-8')
print(json.dumps(result))
