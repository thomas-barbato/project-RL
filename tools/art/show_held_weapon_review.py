"""Embed the native isolated compositor output in the reusable equipment page.

No drawing or active asset is modified. Regenerate the single capture with:
cargo run --locked --example held_weapon_review
"""
from pathlib import Path
import base64
import hashlib
import json
import re
import struct

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / 'artifacts/apercu-equipement64.html'
CAPTURE = ROOT / 'artifacts/apercu-prises64.png'
REVIEW = ROOT / 'docs/reviews/refonte-agent-2026-10-06/held-weapon-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def integrated_gallery():
    captures = [ROOT / f'artifacts/held64-check-{page}/cold-start.png' for page in (1, 2)]
    if not all(path.exists() for path in captures):
        raise SystemExit('Native captures are absent; keep the existing embedded page or regenerate them.')
    html = OUTPUT.read_text('utf-8')
    html = re.sub(r'<!-- HELD64-BEGIN -->.*?<!-- HELD64-END -->', '', html, flags=re.S)
    html = re.sub(r'// HELD64-SCRIPT-BEGIN.*?// HELD64-SCRIPT-END', '', html, flags=re.S)
    button = '<button data-tab="heldreview" class="active">Prises intégrées</button>'
    if 'data-tab="heldreview"' not in html:
        html = html.replace('<button data-tab="icons" class="active">', button + '<button data-tab="icons">')
    else:
        html = html.replace('Prises à valider</button>', 'Prises intégrées</button>')
    html = html.replace('<section id="icons" class="section">', '<section id="icons" class="section" hidden>')
    html = html.replace("let selected='icons';", "let selected='heldreview';")
    html = html.replace("selected==='runtime'", "(selected==='runtime'||selected==='heldreview')")
    body = '''<!-- HELD64-BEGIN --><section id="heldreview" class="section"><h2>Prises validées et intégrées</h2>
<p class="note">Les <strong>27 armes lourdes</strong> sont plus imposantes et les <strong>7 haches à deux mains</strong> ont une prise horizontale.
Ces captures proviennent du rendu actif du jeu, à 64 px, sur les 19 tenues. Les trois couleurs de chaque tenue sont également contrôlées.
Les fusils ordinaires conservent leur taille.<br>La prise contondante validée reste une référence pour les modèles à deux mains ; les marteaux et masses à une main gardent leurs règles.</p>
<button id="heldzoom" type="button">Voir en zoom ×2</button><p>Affichage initial à taille native. Faire défiler horizontalement si la fenêtre est étroite.</p>'''
    for page, path in enumerate(captures, 1):
        uri = 'data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode()
        body += f'<h3>Tenues {"1 à 10" if page == 1 else "11 à 19"}</h3><div class="capture"><img class="heldcapture" alt="Rendu intégré, page {page}" src="{uri}" width="1280" style="max-width:none;width:1280px;height:auto;image-rendering:pixelated"></div>'
    body += '</section><!-- HELD64-END -->'
    html = html.replace('<section id="icons"', body + '<section id="icons"', 1)
    script = "// HELD64-SCRIPT-BEGIN\ndocument.querySelector('#heldzoom').onclick=()=>{const images=[...document.querySelectorAll('.heldcapture')],big=images[0].style.width==='2560px';images.forEach(i=>i.style.width=big?'1280px':'2560px');document.querySelector('#heldzoom').textContent=big?'Voir en zoom ×2':'Revenir à la taille native';};\n// HELD64-SCRIPT-END\n"
    html = html.replace('</script>', script + '</script>', 1)
    breaths = [ROOT / f'artifacts/breath64-check-{phase}/cold-start.png' for phase in (0, 1)]
    if all(path.exists() for path in breaths):
        frames = ['data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode() for path in breaths]
        animation = '''<section id="breathing" class="section"><h2>Respiration · toutes les armures</h2>
<p class="note">Aperçu animé du rendu du jeu sur les 19 tenues. Le haut du corps, les bras, les mains et l’arme suivent le même mouvement ; les pieds restent fixes.
Le cycle dure 3 secondes, avec deux poses de 1,5 seconde. L’amplitude conserve le mouvement existant : 1 px en 32 px, 2 px en 64 px.
Les portraits et le réglage « mouvements réduits » gardent une pose fixe.</p>
<button id="breathpause" type="button">Pause</button> <button id="breathzoom" type="button">Voir en zoom ×2</button>
<div class="capture"><div id="breathstage" style="position:relative;width:1280px;height:800px"><img alt="Respiration des 19 armures, repos" src="FRAME0"><img class="raised" alt="Deuxième pose du cycle" src="FRAME1"></div></div>
<style>#breathstage img{position:absolute;left:0;top:0;width:100%;height:100%;max-width:none;image-rendering:pixelated}#breathstage .raised{opacity:0;animation:armor-breathing 3s step-end infinite}@keyframes armor-breathing{0%,49.999%{opacity:0}50%,100%{opacity:1}}@media(prefers-reduced-motion:reduce){#breathstage .raised{animation:none}}</style></section>'''.replace('FRAME0', frames[0]).replace('FRAME1', frames[1])
        html = html.replace('<section id="icons"', animation + '<section id="icons"', 1)
        html = html.replace('<button data-tab="heldreview" class="active">', '<button data-tab="breathing" class="active">Respiration</button><button data-tab="heldreview">')
        html = html.replace('<section id="heldreview" class="section">', '<section id="heldreview" class="section" hidden>')
        html = html.replace("let selected='heldreview';", "let selected='breathing';")
        html = html.replace("selected==='heldreview'", "(selected==='heldreview'||selected==='breathing')")
        script = "document.querySelector('#breathpause').onclick=()=>{const image=document.querySelector('#breathstage .raised'),paused=image.style.animationPlayState==='paused';image.style.animationPlayState=paused?'running':'paused';document.querySelector('#breathpause').textContent=paused?'Pause':'Reprendre';};document.querySelector('#breathzoom').onclick=()=>{const s=document.querySelector('#breathstage'),big=s.style.width==='2560px';s.style.width=big?'1280px':'2560px';s.style.height=big?'800px':'1600px';document.querySelector('#breathzoom').textContent=big?'Voir en zoom ×2':'Revenir à la taille native';};"
        html = html.replace('</script>', script + '\n</script>', 1)
    OUTPUT.write_text(html, 'utf-8')
    print(json.dumps({'status': 'integrated', 'native_captures_embedded': 2, 'gallery_bytes': OUTPUT.stat().st_size}))


def main():
    if (REVIEW / 'visual-acceptance-01.json').exists():
        integrated_gallery()
        return
    REVIEW.mkdir(parents=True, exist_ok=True)
    bindings = json.loads((ROOT / 'assets/sprites/flat-agents/manifest.json').read_text('utf-8'))['weapons']
    ids = ['BD-W032', 'BD-W024', 'BD-W016', 'BD-W064', 'BD-W098', 'BD-W003']
    candidates = []
    for number, ident in enumerate(ids, 1):
        binding = next(e for e in bindings if e['review_id'] == ident and e['side'] != 0)
        source = ROOT / f'assets/source-art/refonte-agent-2026-10-06/comic-catalogue-review-01/sources/{ident}.png'
        candidates.append({
            'id': f'PRISES64-{number:02}', 'source_review_id': ident,
            'source': source.relative_to(ROOT).as_posix(), 'source_sha256': sha(source),
            'content_id': binding['id'],
            'change': ('heavy firearm scale 1.55, actual grip/support registration' if number <= 3 else
                       'horizontal haft; source axe rotated exactly 90 degrees without the old oblique rig angle' if number == 4 else
                       'horizontal two-handed blunt pose study only; current one-handed hammer rules unchanged' if number == 5 else
                       'unchanged ordinary rifle control'),
        })
    protected = {p.relative_to(ROOT).as_posix(): sha(p)
                 for folder in ['flat-agents', 'equipment-native64']
                 for p in (ROOT / 'assets/sprites' / folder).iterdir() if p.is_file()}
    # Verify the just-integrated production assets were never replaced by this trial.
    prior = json.loads((ROOT / 'docs/reviews/refonte-agent-2026-10-06/equipment-native64-01/integration-validation-01.json').read_text('utf-8'))
    for entry in prior['active_bank']:
        assert protected[entry['path']] == entry['sha256'], entry['path']
    dimensions = struct.unpack('>II', CAPTURE.read_bytes()[16:24])
    assert dimensions == (1280, 944), dimensions
    review = {
        'review_id': 'HELD-WEAPON-64-01', 'date': '2026-10-16',
        'status': 'candidates pending explicit visual acceptance; no game integration',
        'user_requests': [
            'Make equipped heavy ranged weapons more imposing; ordinary two-handed firearms are satisfactory.',
            'Hold two-handed axes and blunt weapons horizontally.',
            'Correct the tilted axe head.',
        ],
        'candidates': candidates,
        'outfits': ['AR-01 Cerbère', 'AR-N10 Arcaniste', 'AR-03 Chrysalide (olive/bone)'],
        'native_tile': 64, 'composition_canvas': 128,
        'method': 'Isolated Macroquad example; existing source paintings and game sleeve/finger layers with shared registration across outfits. Source-authored opposed gloves for melee. No repainting or generated replacement.',
        'mechanical_changes': False, 'runtime_assets_modified': False,
        'limits': [
            'Five proposed renderings and one unchanged control; not a completed production rollout.',
            'Candidate 05 studies the requested blunt two-handed pose using an existing hammer drawing; it does not convert one-handed weapons to two-handed weapons.',
            'Only the shown candidates are submitted. Numerical scale and hand anchors are review parameters.',
        ],
        'verification': {
            'prototype_build': 'cargo run --locked --example held_weapon_review passed without compiler warning',
            'native_output': 'inspected locally; 18 before/after comparisons',
            'visible_head_pixels_unchanged': True,
            'weapons_not_clipped_by_composition_canvas': True,
            'ordinary_rifle_control_identical': True,
            'active_bank_matches_previous_integration_receipt': True,
            'full_game_suite': 'not rerun; production code and assets untouched',
            'browser_render': 'not observed; file protocol access blocked by browser URL policy; no alternate browser or server workaround used',
        },
        'capture': {'path': CAPTURE.relative_to(ROOT).as_posix(), 'dimensions': dimensions,
                    'bytes': CAPTURE.stat().st_size, 'sha256': sha(CAPTURE)},
        'protected_runtime_files': protected,
        'prototype': {'path': 'examples/held_weapon_review.rs', 'sha256': sha(ROOT / 'examples/held_weapon_review.rs')},
        'cleanup': 'One reusable PNG and its embedded HTML retained for this pending review. No eight-frame series or test CMD generated. Remove temporary capture after acceptance.',
    }
    (REVIEW / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    html = OUTPUT.read_text('utf-8')
    html = re.sub(r'<!-- HELD64-BEGIN -->.*?<!-- HELD64-END -->', '', html, flags=re.S)
    button = '<button data-tab="heldreview" class="active">Prises à valider</button>'
    if 'data-tab="heldreview"' not in html:
        html = html.replace('<button data-tab="icons" class="active">', button + '<button data-tab="icons">')
    html = html.replace('<section id="icons" class="section">', '<section id="icons" class="section" hidden>')
    html = html.replace("let selected='icons';", "let selected='heldreview';")
    html = html.replace("selected==='runtime'", "(selected==='runtime'||selected==='heldreview')")
    uri = 'data:image/png;base64,' + base64.b64encode(CAPTURE.read_bytes()).decode()
    section = '''<!-- HELD64-BEGIN --><section id="heldreview" class="section">
<h2>Prises 64 px · propositions à valider</h2>
<p class="note"><strong>01–03 :</strong> armes lourdes plus imposantes. <strong>04 :</strong> hache tenue à l’horizontale, fer aligné.
<strong>05 :</strong> étude de prise pour une contondante à deux mains ; les armes à une main ne changent pas de règles.
<strong>06 :</strong> fusil ordinaire conservé, pour comparaison.<br>Chaque paire montre l’actuel à gauche et la proposition à droite.
Le carré représente une case de 64 px. Les poses sont communes aux trois armures. Ces propositions ne sont pas encore intégrées au jeu.</p>
<button id="heldzoom" type="button">Voir en zoom ×2</button><p>Faire défiler horizontalement si la fenêtre est étroite. L’affichage initial est à taille native.</p>
<div class="capture"><img id="heldboard" alt="PRISES64-01 à 06 : actuel et proposition sur trois tenues" src="URI" width="1280" style="max-width:none;width:1280px;height:auto;image-rendering:pixelated"></div>
</section><!-- HELD64-END -->'''.replace('URI', uri)
    html = html.replace('<section id="icons"', section + '<section id="icons"', 1)
    zoom = "document.querySelector('#heldzoom').onclick=()=>{const i=document.querySelector('#heldboard'),big=i.style.width==='2560px';i.style.width=big?'1280px':'2560px';document.querySelector('#heldzoom').textContent=big?'Voir en zoom ×2':'Revenir à la taille native';};"
    if "document.querySelector('#heldzoom').onclick=" not in html:
        html = html.replace('</script>', zoom + '\n</script>', 1)
    OUTPUT.write_text(html, 'utf-8')
    print(json.dumps({'review': review['review_id'], 'candidates': len(candidates), 'outfits': 3,
                      'capture_bytes': CAPTURE.stat().st_size, 'gallery_bytes': OUTPUT.stat().st_size,
                      'runtime_assets_unchanged': True}))


if __name__ == '__main__':
    main()
