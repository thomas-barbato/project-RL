from pathlib import Path
import hashlib
import json

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
REVIEW = OUT.parent / 'skill-icon-review-32'

page = REVIEW / 'index.html'
text = page.read_text(encoding='utf-8')
note = '<p style="color:#c7bbdf">Correction de validation : les cinq autres icônes restent acceptées. Déflagration occulte (60 A02-C01) est retirée. <a href="../skill-icon-review-33/index.html">Nouvelle proposition 60 · explosion seule</a>.</p>'
assert 'Correction de validation :' not in text
assert '<main class="gallery">' in text
page.write_text(text.replace('<main class="gallery">', note + '<main class="gallery">'), encoding='utf-8')
page = REVIEW / 'README.md'
page.write_text(page.read_text(encoding='utf-8') + '\n## Correction du 11 octobre 2026\n\nLes cinq dessins A01, HY05, OC04, OC05 et OC07 sont validés et intégrés. A02-C01 est refusée et retirée. Reçu : visual-acceptance-02.json ; nouvelle proposition 60 A02-C02 dans IC33. Les paragraphes précédents conservent le contexte de préparation initial.\n', encoding='utf-8')
page = OUT / 'index.html'
page.write_text(page.read_text(encoding='utf-8').replace('after-correction/A02-960/cold-start.png', 'after-correction/A02-960-confirmed/cold-start.png'), encoding='utf-8')
capture = json.loads((OUT / 'after-correction/capture-result.json').read_text(encoding='utf-8-sig'))
assert capture['process_exit'] == 0
assert hashlib.sha256(Path(capture['file']).read_bytes()).hexdigest() == capture['sha256']
current = json.loads((OUT / 'integration-validation.json').read_text(encoding='utf-8'))
current['correction_native_capture'] = 'after-correction/A02-960-confirmed/cold-start.png'
current['correction_native_process_exit'] = 0
current['targeted_tests_passed'] = 3
current['debug_build_passed'] = True
current['formatting_and_diff_checks_passed'] = True
(OUT / 'integration-validation.json').write_text(json.dumps(current, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print('Correction documented; rejected icon absent; native capture exit 0')
