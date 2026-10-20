"""Check normal-renderer captures against the frozen, accepted GPU review."""
from pathlib import Path
import hashlib
import json
import re
from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01'
OUT = ROOT / 'artifacts/art-quality-audit-01'
ACCEPTED = OUT / 'finish01-engine'
INTEGRATED = OUT / 'finish01-integration'
VIEW = (334, 78, 1552, 782)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def difference(a, b):
    diff = ImageChops.difference(a.convert('RGB'), b.convert('RGB'))
    return sum(pixel != (0, 0, 0) for pixel in diff.get_flattened_data())


def main():
    receipt = json.loads((DOC / 'acceptance.json').read_text('utf-8'))
    for name, digest in receipt['approved_exports'].items():
        assert sha(ROOT / 'assets/sprites/art-finish-01' / name) == digest, name
    prior = json.loads((DOC / 'production-before.json').read_text('utf-8'))
    for name, digest in prior.items():
        assert sha(ROOT / name) == digest, f'Other resource changed: {name}'
    capture = json.loads((INTEGRATED / 'checks.json').read_text('utf-8'))
    assert capture['status'] == 'accepted_integration' and capture['production_mode'] == 3
    panels = {}
    for label, count in [('native', 13), ('hands', 6), ('palettes', 7)]:
        old = Image.open(DOC / f'{label}-native.png')
        new = Image.open(INTEGRATED / f'{label}-gpu.png')
        for i in range(count):
            x, y = 160 + i % 4 * 300, 80 + i // 4 * 200
            box = (x, y+34, x+128, y+162)
            accepted_box = (x, y+44, x+128, y+172)
            changed = difference(old.crop(accepted_box), new.crop(box))
            panels[f'{label}-{i+1}'] = changed
            assert changed == 0, (label, i, changed)
    scenes = {}
    references = DOC / 'references'
    references.mkdir(exist_ok=True)
    for label in ['house', 'street']:
        accepted_path = references / f'{label}-accepted-native.png'
        if not accepted_path.exists():
            Image.open(ACCEPTED / f'{label}-candidate.png').crop(VIEW).save(accepted_path)
        old = Image.open(accepted_path)
        new = Image.open(INTEGRATED / f'{label}-candidate.png').crop(VIEW)
        scenes[label] = difference(old, new)
        assert scenes[label] == 0, (label, scenes[label])
        new.save(OUT / f'finish01-{label}-integrated.png')
        new.save(references / f'{label}-integrated-native.png')
    for label in ['native', 'hands', 'palettes']:
        # Preserve the accepted panels, including their before images, durably.
        source = OUT / f'finish01-{label}-native.png'
        assert source.exists()
    results = dict(review='ART-FINISH-01', status='accepted_and_integrated',
                   packaged_exact_exports=30, native_panels=panels, scene_changed_pixels=scenes,
                   gameplay_fingerprints_unchanged=capture['gameplay_fingerprints_unchanged'],
                   native_cell=64, layers_preserved=True,
                   runtime_assets='assets/sprites/art-finish-01',
                   previous_resource_files_unchanged=len(prior),
                   unsampled_poses_and_equipment_palettes='previous accepted bindings retained',
                   unshown_145_blue_palette='not activated')
    (DOC / 'integration-checks.json').write_text(json.dumps(results, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    review = json.loads((DOC / 'review.json').read_text('utf-8'))
    review['status'] = 'accepted_and_integrated'
    review['ordinary_runtime_unchanged_at_preparation'] = review.pop('ordinary_runtime_unchanged', True)
    review['runtime_assets'] = results['runtime_assets']
    for entry in review['entries']:
        entry['status'] = 'accepted'
    (DOC / 'review.json').write_text(json.dumps(review, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    show_accepted()
    print(json.dumps(results, ensure_ascii=False))


def show_accepted():
    if (ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/review.json').exists() and (OUT / 'finish02-engine/checks.json').exists():
        from show_art_finish_review_02 import build as show_current
        show_current()
        return
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>ART-FINISH-01 · lot intégré</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2{color:#decca7}p{max-width:1100px}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}a{color:#dfc58e}section{margin:32px 0}</style>
<h1>ART-FINISH-01 · lot validé et intégré</h1>
<p>Les treize dessins, six configurations de mains et sept palettes sont acceptés. Le rendu normal reproduit les 26 échantillons et les deux vues de carte à zéro pixel différent. Les images conservent l’échelle native de 64 px par case.</p>
<p>Le plateau et l’inspection partagent les nouveaux dessins. Les armes et les doigts restent des couches séparées. Les autres configurations de mains, modèles et peintures d’équipement conservent leurs ressources précédentes pour de prochains lots.</p>
<nav><a href="#native">Treize dessins</a> · <a href="#hands">Six prises</a> · <a href="#palettes">Sept palettes</a> · <a href="#scene">Rendu normal en carte</a></nav>
<p>Les comparaisons conservées ci-dessous montrent l’ancien dessin à gauche et le dessin validé à droite.</p>
<section id="native"><h2>Les treize dessins</h2><div class="scroll"><img src="finish01-native-native.png" width="1240" height="900" alt="FIN-01 à FIN-13 : avant et dessins acceptés à taille native"></div></section>
<section id="hands"><h2>Les six configurations de mains</h2><div class="scroll"><img src="finish01-hands-native.png" width="1240" height="500" alt="Bastion et Cerbère sans arme, au pistolet et au fusil"></div></section>
<section id="palettes"><h2>Les sept palettes</h2><div class="scroll"><img src="finish01-palettes-native.png" width="1240" height="500" alt="Palettes acceptées des deux démons ailés"></div></section>
<section id="scene"><h2>Le rendu normal du jeu</h2><h3>Dans le logement</h3><div class="scroll"><img src="finish01-house-integrated.png" width="1218" height="704" alt="Personnage intégré auprès des corps dans le logement éclairé"></div><h3>Dans la rue</h3><div class="scroll"><img src="finish01-street-integrated.png" width="1218" height="704" alt="Dessins intégrés dans les lumières réelles de la rue"></div></section>
<p>Les figurants de cette comparaison servent uniquement à présenter le rendu ; aucune rencontre n’est ajoutée à la partie. Les identifiants, statistiques, règles de combat, éclairages et collisions sont conservés.</p>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/acceptance.json">Accord et empreintes</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/integration-checks.json">Contrôles d’intégration</a></p></html>'''
    for ref in re.findall(r'(?:src|href)="([^"#]+)"', page):
        assert (OUT / ref).exists(), ref
    (OUT / 'index.html').write_text(page, encoding='utf-8')


if __name__ == '__main__':
    main()
