"""Present the actual GPU samples without changing any sprite pixels."""
from pathlib import Path
import hashlib
import html
import json
import shutil
from PIL import Image, ImageDraw, ImageFont, ImageChops

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/art-quality-audit-01'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-quality-audit-01'
CIVIL = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-civilians-review-08'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build():
    if (ROOT / 'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/review.json').exists() and (OUT / 'finish01-engine/checks.json').exists():
        from show_art_finish_review import build as show_latest
        show_latest()
        return
    capture = json.loads((OUT / 'checks.json').read_text(encoding='utf-8'))
    before = json.loads((DOC / 'art-before-audit.json').read_text(encoding='utf-8'))
    assert capture['native_cell'] == 64 and len(capture['samples']) == 20
    assert not capture['gameplay_loaded'] and not capture['art_modified']
    for file, digest in before.items():
        assert sha(ROOT / file) == digest, file
    raw_path = OUT / 'native-comparison.png'
    raw = Image.open(raw_path).convert('RGB')
    assert raw.size == (1240, 900)
    # A captured empty framebuffer must never be accepted as a visual proof.
    assert len(raw.getcolors(raw.width * raw.height)) > 1000
    board = Image.new('RGB', raw.size, '#11191d')
    draw = ImageDraw.Draw(board)
    font_path = Path('C:/Windows/Fonts/segoeui.ttf')
    font = lambda size: ImageFont.truetype(str(font_path), size)
    draw.text((24, 14), 'Qualité graphique · comparaison du rendu actuel', font=font(24), fill='#decca7')
    draw.text((24, 49), 'ART-QUALITY-01 · cases de 64 px · même fond, sans éclairage de scène · aucun dessin retouché', font=font(17), fill='#c5cfcc')
    same_pixels = []
    for index, sample in enumerate(capture['samples']):
        x, y, _, _ = map(int, sample['panel'])
        box = (x + 4, y + 34, x + 236, y + 164)
        stage = raw.crop(box)
        board.paste(stage, box[:2])
        draw.text((x + 8, y + 3), sample['label'], font=font(17), fill='#decca7')
        code = sample['code']
        if index == 18:
            code = 'torse 102 · ancien rig 32 px'
        elif index == 15:
            code = 'fusil d’assaut · icône dans 64 px'
        elif index == 16:
            code = 'pistolet P01 · icône dans 64 px'
        elif index == 17:
            code = 'Cerbère · icône dans 64 px'
        draw.text((x + 8, y + 174), code, font=font(13), fill='#b7c3c0')
        assert ImageChops.difference(stage, board.crop(box)).getbbox() is None
        same_pixels.append(sample['code'])
    board.save(OUT / 'board-native.png')
    shutil.copy2(raw_path, DOC / 'native-renderer.png')
    shutil.copy2(OUT / 'board-native.png', DOC / 'board-native.png')
    checks = dict(capture, checked_source_files=len(before), source_hashes_unchanged=True,
                  unchanged_image_panels=len(same_pixels),
                  raw_capture_sha256=sha(raw_path), native_board_sha256=sha(OUT / 'board-native.png'),
                  presentation_changes='labels only, no sprite scaling or retouch',
                  visual_review='native board and normal integrated house view inspected')
    (DOC / 'checks.json').write_text(json.dumps(checks, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    rows = [
        ('Personnages et armures', 'Haute', 'Contours épais, surfaces régulières et bras très clairs : la finition paraît plus lisse et plus proche d’une figurine que celle des nouveaux corps.'),
        ('Anciennes ressources', 'Haute', 'Trois torses et certaines invocations passent encore par des dessins 32 px agrandis. La Brigandine Cognefer et le drone 57 montrent cet écart technique.'),
        ('Monstres de campagne', 'Moyenne', 'Déjà natifs et lisibles. Les démons ont surtout besoin d’une finition de matières plus cohérente ; leurs designs n’ont pas à être jetés.'),
        ('Armes', 'Moyenne', 'Les icônes échantillonnées sont déjà nettes. Une arme portée est physiquement plus petite ; la retouche doit préserver sa taille, ses attaches et ses mains.'),
        ('PNJ du camp', 'Faible', 'Le PNJ observé est déjà plus sobre et plus proche du nouveau rendu. Pas de refonte générale justifiée par cet exemple.'),
    ]
    table = ''.join('<tr>' + ''.join(f'<td>{html.escape(v)}</td>' for v in row) + '</tr>' for row in rows)
    report_url = '../../' + (DOC / 'AUDIT.md').relative_to(ROOT).as_posix()
    context_url = '../../' + (CIVIL / 'integrated-house-open-native.png').relative_to(ROOT).as_posix()
    rules_url = '../../docs/DIRECTION_ARTISTIQUE_TILES.md'
    page = f'''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Audit de qualité graphique · ART-QUALITY-01</title><style>
body{{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}}h1,h2{{color:#decca7}}p{{max-width:1100px}}a{{color:#dfc58e}}nav{{display:flex;gap:24px;flex-wrap:wrap}}.scroll{{overflow:auto;border:1px solid #43514e;max-width:1240px}}img{{display:block;max-width:none}}table{{border-collapse:collapse;max-width:1240px}}td,th{{border:1px solid #43514e;padding:12px;text-align:left;vertical-align:top}}th{{color:#decca7}}.note{{color:#aebfba}}
</style><h1>Les corps révèlent un écart de finition</h1>
<p>CIVILS-U01-08 est intégré : 18 corps et 19 décors de sang ajoutés, dont cinq corps en intérieur. Les dix vues normales et la carte entière correspondent exactement à la revue acceptée.</p>
<p>Mon constat : les nouveaux corps ont des vêtements, des ombres et des postures plus naturels que plusieurs personnages actuels. Mais tous les anciens dessins ne sont pas de mauvaise qualité : les silhouettes du bestiaire restent utiles, et les icônes d’armes échantillonnées sont déjà nettes.</p>
<nav><a href="#native">Comparaison native</a><a href="#findings">Constats</a><a href="#scene">Dans la carte</a><a href="{report_url}">Rapport complet</a><a href="{rules_url}">Règles de dessin</a><a href="../street-defects-review-01/index.html">Revue des corps intégrés</a></nav>
<h2 id="native">Même échelle, rendu réel du moteur</h2>
<p>Vingt échantillons sur un fond identique sans lumière de scène. Les dessins gardent leur taille prévue ; les grands démons débordent naturellement de leur case. Les icônes sont montrées dans une case de 64 px. Les libellés sont recomposés, les pixels des vingt zones d’image sont conservés à l’identique.</p>
<p class="note">Affichage natif à un zoom de navigateur de 100 %. Fais défiler horizontalement si nécessaire.</p>
<div class="scroll"><img src="board-native.png" width="1240" height="900" alt="Corps, personnages équipés, PNJ, monstres, armes et anciens dessins à échelle constante"></div>
<h2 id="findings">Ce qu’il faudrait harmoniser</h2><table><thead><tr><th>Catégorie</th><th>Priorité</th><th>Constat</th></tr></thead><tbody>{table}</tbody></table>
<p>Je recommande un petit lot pilote : une tenue du joueur, une armure portée, un fusil avec ses mains, un cultiste et un démon connus. Il serait montré à côté d’un corps accepté, puis dans la carte éclairée. Après validation seulement, la finition pourrait être étendue par famille.</p>
<p>Les tenues récentes et les monstres de campagne sont déjà natifs en 64 px. Les anciens torses 102, 104 et 113 suivent un autre chemin. Changer le filtre ou agrandir tous les sprites ne corrigerait pas ces deux causes différentes.</p>
<h2 id="scene">Les corps intégrés dans l’éclairage du jeu</h2>
<p>Vue normale de contrôle : sol usé, vrais luminaires, toit masqué et vision de six cases. Cette partie technique est créée sans choix de classe ; son personnage ne représente pas à lui seul les cinq classes récentes montrées plus haut.</p>
<div class="scroll"><img src="{context_url}" width="1218" height="704" alt="Corps et joueur dans une maison avec le rendu normal du jeu"></div>
<p class="note">Audit représentatif, pas une revue individuelle de toutes les variantes. Aucun dessin d’acteur, pouvoir ou équipement n’a été changé. Les nouvelles règles du lot civil sont consignées ; les retouches proposées restent à montrer et à valider.</p>
</html>'''
    (OUT / 'index.html').write_text(page, encoding='utf-8')
    print(json.dumps(dict(review='ART-QUALITY-01', source_files_unchanged=len(before), native_panels_unchanged=len(same_pixels), report=str(DOC / 'AUDIT.md'))))


if __name__ == '__main__':
    build()
