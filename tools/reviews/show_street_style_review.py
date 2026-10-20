"""Native engine comparisons for the scenery-first grimdark trial, FIXES-U01-03."""
from pathlib import Path
import hashlib
import json
import re
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'artifacts/street-defects-review-01'
ENGINE = OUT/'engine03'
ART = ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r03'
DOC = ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-defects-review-03'
VIEW = (334, 78, 1552, 782)


def native(file):
    return Image.open(file).convert('RGBA')


def build():
    if (OUT/'engine08/checks.json').exists():
        from show_street_civilians_review import build as latest
        latest()
        return
    if (OUT/'engine06/checks.json').exists():
        from show_street_paving_review import build as latest
        latest()
        return
    if (OUT/'engine04/checks.json').exists():
        from show_street_map_style_review import build as latest
        latest()
        return
    checks = json.loads((ENGINE/'checks.json').read_text(encoding='utf-8'))
    layout = json.loads((ART/'review-layout.json').read_text(encoding='utf-8'))
    assert checks['review'] == layout['review'] == 'FIXES-U01-03'
    assert checks['native_cell'] == 64 and checks['render_read_only']
    assert checks['camp_round_trip'] and not checks['grim_art_integrated']
    assert checks['grim_candidate']['same_lighting'] and checks['grim_candidate']['living_actors_unchanged']
    assert all('GRIM-U01-02' not in item['file'] for item in layout['items'])
    changes = {}
    for prefix, before, after in [('gate', 'street-current', 'street-grim-candidate'),
                                   ('style', 'style-current', 'style-candidate')]:
        old = native(ENGINE/f'{before}.png').convert('RGB').crop(VIEW)
        new = native(ENGINE/f'{after}.png').convert('RGB').crop(VIEW)
        channels = ImageChops.difference(old, new).split()
        difference = ImageChops.lighter(ImageChops.lighter(channels[0], channels[1]), channels[2])
        changed = sum(difference.histogram()[3:])
        assert changed > 100
        changes[prefix] = changed
        old.save(OUT/f'r03-{prefix}-current.png')
        new.save(OUT/f'r03-{prefix}-candidate.png')
    old_warehouse = native(ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r06/warehouse-native.png')
    new_warehouse = native(ART/'STYLE-U01-01-native.png')
    assert old_warehouse.size == new_warehouse.size == (651, 403)
    floor = native(ART/'STYLE-U01-02-native.png')
    player = native(ROOT/'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png')
    # Exclude virtually invisible source alpha noise; retain the native pixels.
    player = player.crop(player.getchannel('A').point(lambda a: 255 if a > 10 else 0).getbbox())
    assert player.height <= 64
    font = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 15)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 13)
    board = Image.new('RGB', (1050, 210), '#111a1f')
    draw = ImageDraw.Draw(board)
    draw.text((14, 10), 'Corps au sol · taille prévue dans le jeu · sans zoom', font=font, fill='#d9c8a6')
    draw.text((14, 36), 'Les trois corps isolés sont nouveaux. La barricade conserve son dessin ; son emplacement change.', font=small, fill='#bbc8c5')
    bg = native(ROOT/'assets/maps/prison-street-expanded-01/street.png').crop((3*64, 59*64, 3*64+128, 59*64+96)).convert('RGB')
    positions = [(35, player, 'Joueur actuel', 'repère inchangé'),
                 (210, native(ART/'CORP-U01-01-native.png'), 'CORP-U01-01 · civil', '80 × 48 px'),
                 (410, native(ART/'CORP-U01-02-native.png'), 'CORP-U01-02 · cultiste', '80 × 48 px'),
                 (620, native(ART/'CORP-U01-03-native.png'), 'CORP-U01-03 · militaire', '80 × 48 px'),
                 (850, native(ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r01/GRIM-U01-01-native.png'), 'GRIM-U01-01', '128 × 64 px · dessin accepté')]
    for x, im, name, size in positions:
        board.paste(bg, (x-8, 70))
        board.paste(im, (x, 156-im.height), im)
        draw.text((x-8, 172), name, font=small, fill='#d9c8a6')
        draw.text((x-8, 191), size, font=small, fill='#bbc8c5')
    board.save(OUT/'r03-corps-natifs.png')
    for image, label in [(old_warehouse, 'current'), (new_warehouse, 'candidate')]:
        plate = Image.new('RGB', (675, 497), '#111a1f')
        d = ImageDraw.Draw(plate)
        d.text((12, 10), 'Entrepôt actuel' if label == 'current' else 'STYLE-U01-01 · essai de matières', font=font, fill='#d9c8a6')
        plate.paste(image, (12, 42), image)
        plate.paste(player, (590, 490-player.height), player)
        d.text((12, 466), 'Même canevas : 651 × 403 px · joueur inchangé', font=small, fill='#bbc8c5')
        plate.save(OUT/f'r03-warehouse-{label}.png')
    floor.save(OUT/'r03-floor.png')
    page = '''<!doctype html>
<html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>Rue · essai de décors usés</title>
<style>
body{margin:24px;background:#111a1f;color:#d2dcd8;font:17px Segoe UI,sans-serif}
h1,h2{color:#d9c8a6}p{max-width:1080px;line-height:1.55}a{color:#e0c693}
section{margin:36px 0}.scroll{overflow:auto;max-width:100%;margin:14px 0;border:1px solid #40525a}
img{display:block;max-width:none}.buttons{display:flex;gap:10px;flex-wrap:wrap}
button{background:#24363d;color:#dfd5bd;border:1px solid #657a80;padding:10px 18px;font:inherit;cursor:pointer}
button[aria-pressed=true]{border-color:#dec793;background:#3c443c}.note{color:#b7c5c3}
</style>
<h1>Décors plus usés · personnages conservés</h1>
<p>FIXES-U01-03 · 19 octobre 2026 · premier essai de style avant d’étendre ce traitement à la ville.</p>
<p>Béton terne, peinture écaillée, tôles marquées et usure irrégulière. La vue reste celle du camp. Les personnages actuels servent de comparaison. Les deux vues du moteur gardent le même éclairage et la même caméra ; les nouveaux dessins sont proposés ici avant intégration.</p>
<section id="style"><h2>1 · Essai dans le jeu</h2>
<p>STYLE-U01-01 : entrepôt. STYLE-U01-02 : sol. Les caisses, le mobilier et le joueur conservent leurs dessins. Ce test porte sur les matériaux ; le nouvel entrepôt demanderait encore un calage précis de son emprise avant remplacement dans le jeu.</p>
<div class="buttons"><button data-for="style-frame" data-src="r03-style-current.png" aria-pressed="false">Décor actuel</button><button data-for="style-frame" data-src="r03-style-candidate.png" aria-pressed="true">Essai de matières</button></div>
<div class="scroll"><img id="style-frame" src="r03-style-candidate.png" width="1218" height="704" alt="Essai de matières rendu par le jeu à taille native"></div>
</section>
<section id="native"><h2>2 · Dessin entier à taille native</h2>
<p>Le bâtiment est créé comme un élément complet sur fond transparent, puis assemblé avec des tuiles de sol. Aucune façade ni accessoire n’est extrait d’une illustration de rue.</p>
<div class="buttons"><button data-for="warehouse" data-src="r03-warehouse-current.png" aria-pressed="false">Entrepôt actuel</button><button data-for="warehouse" data-src="r03-warehouse-candidate.png" aria-pressed="true">Entrepôt proposé</button></div>
<div class="scroll"><img id="warehouse" src="r03-warehouse-candidate.png" width="675" height="497" alt="Entrepôt complet proposé et joueur inchangé"></div>
<details><summary>Sol proposé · quatre cases de 64 px</summary><div class="scroll"><img src="r03-floor.png" width="128" height="128" alt="Tuiles de béton usé"></div></details>
</section>
<section id="corps"><h2>3 · Civils, cultistes et militaires morts</h2>
<div class="scroll"><img src="r03-corps-natifs.png" width="1050" height="210" alt="Trois nouveaux corps et barricade avec repère joueur à taille réelle"></div>
<p>La station médicale de contrainte est retirée de la proposition pour cette rue. La barricade déjà appréciée est rapprochée des impacts devant la prison ; les nouveaux corps se répartissent entre l’approche de la défense et la clôture. Le couloir de sortie reste dégagé.</p>
<div class="buttons"><button data-for="gate" data-src="r03-gate-current.png" aria-pressed="false">Entrée actuelle</button><button data-for="gate" data-src="r03-gate-candidate.png" aria-pressed="true">Placement proposé</button></div>
<div class="scroll"><img id="gate" src="r03-gate-candidate.png" width="1218" height="704" alt="Placement des corps près de la prison dans le moteur"></div>
<p class="note">Les corps sont dessinés au sol, sous les acteurs. Ils ne créent ici aucune nouvelle collision ni interaction. Leurs dessins et ce placement attendent ton avis.</p>
</section>
<details><summary>Corrections de circulation et de visibilité déjà intégrées</summary>
<p>Les correctifs précédents restent en place : vision réelle de six cases, PNJ visibles derrière les murs dans ce cercle, et 61 passages réparés. Les captures r03 vérifient à nouveau le parcours 16,31 → 7,31 en neuf tours et le retour au camp.</p>
<div class="scroll"><img src="r02-npc-comparaison.png" width="796" height="334" alt="Comparaison du correctif de visibilité du PNJ"></div></details>
<script>
document.querySelectorAll('button[data-for]').forEach(button=>button.onclick=()=>{
  document.getElementById(button.dataset.for).src=button.dataset.src;
  document.querySelectorAll('button[data-for="'+button.dataset.for+'"]').forEach(other=>other.setAttribute('aria-pressed',String(other===button)));
});
</script></html>'''
    # Versioned image URLs also refresh an already-open local review after reload.
    for file in OUT.glob('r03-*.png'):
        digest = hashlib.sha256(file.read_bytes()).hexdigest()[:12]
        page = page.replace(file.name+'"', file.name+'?v='+digest+'"')
    (OUT/'index.html').write_text(page, encoding='utf-8')
    for file in re.findall(r'(?:src|data-src)="([^"]+)"', page):
        assert (OUT/file.split('?')[0]).is_file(), file
    checks.update(changed_pixels=changes, review_page=OUT.relative_to(ROOT).as_posix()+'/index.html',
                  source_conditioning='assets/source-art/first-sector-review-01/u01-corrections-r03/conditioning.json')
    DOC.mkdir(parents=True, exist_ok=True)
    (DOC/'capture-checks.json').write_text(json.dumps(checks, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(json.dumps(dict(review=checks['review'], changed_pixels=changes, normal_integration=False)))


if __name__ == '__main__':
    build()
