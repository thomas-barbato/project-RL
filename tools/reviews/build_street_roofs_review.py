"""Extract new roof layers; never replace approved background pixels."""
from pathlib import Path
import argparse
import hashlib
import json
import shutil

from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-roofs-review-01'
ART = ROOT / 'assets/source-art/first-sector-review-01/street-roofs-r01'
OUT = ROOT / 'artifacts/street-roofs-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def show_capture():
    if (OUT / 'engine-integrated-r01/checks.json').exists():
        from show_street_roofs_integration import build
        build()
        return
    cap = OUT / 'engine-r01'
    if not (cap / 'checks.json').exists():
        return
    checks = json.loads((cap / 'checks.json').read_text(encoding='utf-8'))
    assert checks['render_read_only'] and checks['lighting_production_enabled']
    assert checks['native_cell'] == 64 and checks['source_pixel_scale'] == 1
    assert checks['camera_first'] == [0, 0] and checks['image_origin_px'] == [96, 64]
    frames = {state: Image.open(cap / f'roof-{state}.png').convert('RGB')
              for state in ('closed', 'open', 'inside')}
    assert len({frame.size for frame in frames.values()}) == 1
    mask = Image.new('L', frames['closed'].size)
    layer_mask = Image.open(ART / 'ROOF-U01-01A-r01.png').getchannel('A')
    mask.paste(layer_mask, (96, 64))
    diff = ImageChops.difference(frames['closed'], frames['open'])
    outside = diff.copy()
    outside.paste((0, 0, 0), mask=mask)
    assert outside.getbbox() is None, 'Opening the roof changed street lighting or other pixels'
    assert diff.getbbox() is not None, 'Roof state did not change'
    font = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 18)
    board = Image.new('RGB', (1044, 460), '#10191e')
    labels = [('closed', '01A · Toit visible'), ('open', '01A · Intérieur dévoilé')]
    for i, (state, label) in enumerate(labels):
        crop = frames[state].crop((128, 80, 640, 496))
        crop.save(OUT / f'home-{state}-native.png')
        board.paste(crop, (i * 532, 36))
        ImageDraw.Draw(board).text((i * 532 + 12, 6), label, font=font, fill='#dec898')
    board.save(OUT / 'roofs-comparison-native.png')
    frames['inside'].crop((128, 80, 640, 496)).save(OUT / 'home-inside-native.png')
    frames['closed'].crop((688, 112, 1088, 448)).save(OUT / 'shop-native.png')
    checks.update(outside_roof_pixels_identical=True,
                  framebuffer_size=list(frames['closed'].size),
                  captures={p.name: sha(p) for p in cap.glob('*.png')})
    (DOC / 'render-checks-r01.json').write_text(
        json.dumps(checks, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<title>ROOF-U01-01 · toits et intérieur</title><style>
body{background:#10191e;color:#d7dfdc;font:17px Segoe UI,sans-serif;margin:24px}
h1,h2{color:#dec898}p{max-width:960px;line-height:1.5}.scroll{overflow:auto;border:1px solid #415159;padding:8px;margin:16px 0}
img{display:block;max-width:none}button{padding:10px 18px;background:#26383e;color:#dec898;border:1px solid #63747a;cursor:pointer}</style>
<h1>ROOF-U01-01 · Les toits</h1>
<p>Deux nouveaux toits, avec le même angle et les cases de 64 px. Captures du moteur,
avec la pénombre et les vraies lumières conservées. La scène entière est dévoilée uniquement pour cette revue.</p>
<p>Proposition : ouvrir la porte dévoile l'intérieur ; il reste visible tant que
la porte est ouverte ou que le personnage est dedans. Fermer derrière soi ne
fait donc pas réapparaître le toit au-dessus du personnage. La commande de porte
sera raccordée après validation ; ici on compare les couches du dessin.</p>
<h2>01A · Maison — comparaison à taille native</h2>
<div class="scroll"><img src="roofs-comparison-native.png" alt="Toit visible puis intérieur dévoilé"></div>
<button id="toggle">Dévoiler l'intérieur</button>
<div class="scroll"><img id="home" src="home-closed-native.png" alt="Maison à taille native"></div>
<h2>01A · Personnage à l'intérieur</h2><div class="scroll"><img src="home-inside-native.png" alt="Toit masqué à l'intérieur"></div>
<h2>01B · Second toit</h2><div class="scroll"><img src="shop-native.png" alt="Toit du commerce fermé"></div>
<p>Seuls les pixels du toit changent entre les deux vues de la maison.
Les lumières et les surfaces de la rue restent identiques. Les lampes intérieures
n'éclairent pas le dessus du toit.</p>
<p>Sources et règles conservées dans docs/reviews/refonte-histoire-2026-10-16/street-roofs-review-01.
Dessins produits avec imagegen, extraits à leur taille d'origine, sans remplacer le fond validé.</p>
<script>let open=false;document.getElementById('toggle').onclick=()=>{open=!open;
document.getElementById('home').src=open?'home-open-native.png':'home-closed-native.png';
document.getElementById('toggle').textContent=open?'Afficher le toit':"Dévoiler l'intérieur";};</script></html>'''
    (OUT / 'index.html').write_text(page, encoding='utf-8')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path)
    args = parser.parse_args()
    for path in (DOC, ART, OUT):
        path.mkdir(parents=True, exist_ok=True)
    source = ART / 'ROOF-U01-01-source-r01.png'
    if args.source:
        if source.exists():
            assert sha(source) == sha(args.source), 'Keep existing versioned source intact'
        else:
            shutil.copyfile(args.source, source)
    ink = Image.open(source).convert('RGBA')
    assert ink.size == (1536, 1024), ink.size
    # Technical extraction only: source pixels are copied without recolouring,
    # rescaling or repainting. All exterior front walls and lamps stay original.
    footprints = {
        'ROOF-U01-01A': [(44, 65), (240, 65), (240, 32), (511, 32), (511, 299), (44, 299)],
        'ROOF-U01-01B': [(615, 72), (949, 72), (949, 278), (615, 278)],
    }
    records = []
    for code, polygon in footprints.items():
        alpha = Image.new('L', ink.size)
        ImageDraw.Draw(alpha).polygon(polygon, fill=255)
        layer = ink.copy()
        layer.putalpha(alpha)
        path = ART / f'{code}-r01.png'
        layer.save(path)
        assert layer.size == (1536, 1024)
        assert alpha.getbbox() == layer.getbbox()
        records.append(dict(id=code, file=path.relative_to(ROOT).as_posix(),
                            polygon=polygon, sha256=sha(path), source_pixel_scale=1))
    accepted = (DOC / 'acceptance-r01.json').exists()
    integrated = (DOC / 'integration-render-r01.json').exists()
    manifest = dict(review='ROOF-U01-01-r01', status='accepted_integrated' if integrated else 'accepted_integration_in_progress' if accepted else 'proposed_not_integrated',
                    source=source.relative_to(ROOT).as_posix(), source_sha256=sha(source),
                    style='STYLE-RL-2026-10-18-r01', cell=64, layers=records,
                    approved_background_unchanged=True,
                    behavior='Hide when door open OR player inside; otherwise show',
                    real_door_gameplay_active=accepted,
                    generation=156 if accepted else None,
                    acceptance='acceptance-r01.json' if accepted else None)
    (DOC / 'roofs-r01.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    show_capture()
    print(json.dumps(manifest, ensure_ascii=False))


if __name__ == '__main__':
    main()
