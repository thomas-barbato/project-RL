from pathlib import Path
import json
from PIL import Image, ImageChops, ImageStat

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
REVIEW = OUT.parent / 'skill-icon-review-48'
captures = json.loads((OUT / 'capture-result.json').read_text('utf-8-sig'))
images = {}
for capture in captures:
    assert capture['process_exit'] == 0
    image = Image.open(capture['file']).convert('RGB')
    assert image.size == (960, 540)
    images[capture['skill']] = image

review = json.loads((REVIEW / 'review.json').read_text('utf-8'))
assert review['status'] == 'accepted'
candidate = review['candidates'][0]
assert candidate['variant'] == 'HL04-C05'
source = Image.open(REVIEW / candidate['files']['64']['file']).convert('RGB')
crop = images['HL04'].crop((239, 270, 303, 334))
error = sum(ImageStat.Stat(ImageChops.difference(crop, source)).mean) / 3
assert error < 15, error
crop.save(OUT / 'HL04-native-icon.png')

before = Image.open(OUT / 'FE01-before-960/cold-start.png').convert('RGB')
grid_and_footer = (200, 0, 605, 540)
grid_changes = ImageChops.difference(before.crop(grid_and_footer), images['FE01'].crop(grid_and_footer)).getbbox()
assert grid_changes is None, grid_changes
# Separate launches can differ by a few channel units on antialiased glyphs.
# Keep a strict low tolerance for navigation and require exact grid pixels.
navigation = (0, 0, 200, 540)
navigation_difference = sum(ImageStat.Stat(ImageChops.difference(before.crop(navigation), images['FE01'].crop(navigation))).mean) / 3
assert navigation_difference < 0.1, navigation_difference
prior_illustration_rect = (669, 234, 861, 426)
detail_difference = sum(ImageStat.Stat(ImageChops.difference(before.crop(prior_illustration_rect), images['FE01'].crop(prior_illustration_rect))).mean) / 3
assert detail_difference > 5, detail_difference
result = {'native_dimensions': [960, 540], 'all_process_exits': 0, 'capture_count': 2,
          'accepted_icon': {'skill': 'HL04', 'variant': 'HL04-C05', 'position': [239, 270],
                            'mean_channel_error': round(error, 3), 'tolerance': 15},
          'FE01_grid_and_footer_pixels_unchanged': True,
          'FE01_navigation_mean_channel_difference': round(navigation_difference, 5),
          'navigation_tolerance': 0.1,
          'FE01_description_changed_mean_channel_difference': round(detail_difference, 3)}
(OUT / 'native-capture-validation.json').write_text(json.dumps(result, indent=2) + '\n', 'utf-8')
page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Compétences · descriptions sans illustrations</title><style>body{margin:0;padding:22px;background:#0b1017;color:#d9e2e8;font:15px system-ui}main{max-width:960px;margin:auto}h1{font-size:23px}h2{font-size:18px;margin-top:28px}p{color:#a6b6c1;line-height:1.6}.capture{overflow-x:auto}img{display:block;width:960px;height:540px}</style><main><h1>Compétences · descriptions sans illustrations</h1><p>L’icône 81 HL04-C05 validée est intégrée. Les grandes images sont retirées des descriptions.</p>'''
for skill, title in [('HL04', 'Maîtrise des armes lourdes'), ('FE01', 'Braise · texte et effets')]:
    page += '<h2>' + title + '</h2><div class="capture"><img src="' + skill + '-960/cold-start.png" width="960" height="540" alt="Capture réelle de la fiche ' + title + '"></div>'
(OUT / 'index.html').write_text(page + '</main></html>', 'utf-8')
print(json.dumps(result))
