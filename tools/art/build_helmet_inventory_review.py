"""Inventory existing helmet artwork; compose review sheets without changing art."""
from html import escape
from pathlib import Path
import hashlib
import json
import re

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/animation-polish-01'
OUT = ROOT / 'artifacts/animation-polish-01/helmet-inventory/current'
CAPTURES = ROOT / 'artifacts/animation-polish-01/helmets-r01/current'
CLASSES = [('bastion', 'Bastion'), ('traqueuse', 'Traqueuse'),
           ('arcaniste', 'Arcaniste'), ('metamorphe', 'Métamorphe')]
VIEWS = [('front', 'Face'), ('back', 'Dos'), ('left', 'Gauche'), ('right', 'Droite')]
BACKGROUND = (30, 40, 49)
SIZE = (776, 742)


def read_json(path):
    return json.loads(path.read_text(encoding='utf-8'))


def inventory():
    """Read all equipable head items and their runtime bindings, not a hand list."""
    locale = read_json(ROOT / 'content/core/locales/fr_accessory_models.json5')['entries']
    catalog = (ROOT / 'src/game_art_catalog.rs').read_text(encoding='utf-8')
    bindings = dict((item, int(model)) for item, model in re.findall(
        r'"([^"]+)" => ItemArt \{ icon: \d+, illustration: (\d+) \}', catalog))
    icons = read_json(ROOT / 'assets/sprites/equipment-native64/manifest.json')['icon_bindings']
    rows = []
    for path in sorted((ROOT / 'content/core/items').glob('*.json5')):
        source = path.read_text(encoding='utf-8')
        if 'core:head_armor' not in source:
            continue
        item = json.loads(source)
        if item.get('equipment', {}).get('slot') != 'core:head_armor':
            continue
        model = bindings[item['id']]
        assert 115 <= model <= 120, f'Uncaptured helmet model {model}'
        cell, variants = icons[str(model)]
        assert not variants, 'Authored icon variants require explicit inventory'
        rows.append({'number': model - 114, 'model': model, 'id': item['id'],
                     'name': locale[item['name_key']], 'icon_cell': cell,
                     'item_file': path.relative_to(ROOT).as_posix()})
    rows.sort(key=lambda row: row['model'])
    assert [row['model'] for row in rows] == list(range(115, 121))
    return rows


def font(size):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', size)


def icon(atlas, cell):
    columns = atlas.width // 64
    x, y = cell % columns * 64, cell // columns * 64
    return atlas.crop((x, y, x + 64, y + 64))


def figure(model, outfit, view, phase, class_index, fitted=True):
    path = CAPTURES / f'helmet-H{model}-{outfit}-{view}-{phase}.png'
    with Image.open(path) as source:
        assert source.size == (540, 470), f'Unexpected capture layout: {path}'
        # Copy at 1:1, with 16 px horizontal and 8 px upper/lower tile margin.
        x = 12 + class_index * 128
        y = 328 if fitted else 88
        return source.crop((x, y, x + 96, y + 80)).convert('RGB')


def contact_sheet(rows, atlas, class_index, outfit, phase):
    sheet = Image.new('RGB', SIZE, BACKGROUND)
    draw = ImageDraw.Draw(sheet)
    name = CLASSES[class_index][1]
    armor = 'armure lourde' if outfit == 'heavy' else 'tenue de base'
    draw.text((16, 10), f'Casques existants · {name} · {armor}', font=font(18), fill='white')
    draw.text((16, 38), 'Taille native · proportions 04 acceptées · mêmes poses pour tous les casques',
              font=font(13), fill=(174, 191, 200))
    for x, label in [(16, 'Modèle'), (256, 'Icône'), (362, 'Face'), (462, 'Dos'),
                     (556, 'Gauche'), (656, 'Droite')]:
        draw.text((x, 73), label, font=font(14), fill=(174, 191, 200))
    for n, row in enumerate(rows):
        y = 105 + n * 100
        draw.line((16, y - 5, 756, y - 5), fill=(54, 72, 83))
        draw.text((16, y + 12), f'{row["number"]:02} · {row["name"]}', font=font(15), fill='white')
        draw.text((16, y + 40), f'Modèle porté {row["model"]}', font=font(12), fill=(174, 191, 200))
        tile = icon(atlas, row['icon_cell'])
        sheet.paste(tile, (248, y + 8), tile)
        for v, (key, _) in enumerate(VIEWS):
            sheet.paste(figure(row['model'], outfit, key, phase, class_index), (340 + v * 100, y))
    draw.text((16, 718), f'Phase {phase + 1}/6 · dessins actuels · aucune sélection de modèles supprimés',
              font=font(12), fill=(174, 191, 200))
    return sheet


def source_sheet(rows, atlas):
    front = Image.open(ROOT / 'assets/animations/vertical-18/heads/helmets-front.png').convert('RGBA')
    back = Image.open(ROOT / 'assets/animations/vertical-18/heads/helmets-back.png').convert('RGBA')
    sheet = Image.new('RGB', (776, 610), BACKGROUND)
    draw = ImageDraw.Draw(sheet)
    draw.text((16, 12), 'Les six modèles · icônes et coques actuelles', font=font(18), fill='white')
    draw.text((16, 42), 'Icônes natives 64 px ; coques face/dos au zoom entier ×4 pour examiner les détails.',
              font=font(13), fill=(174, 191, 200))
    for n, row in enumerate(rows):
        x, y = 16 + n % 3 * 252, 85 + n // 3 * 255
        draw.text((x, y), f'{row["number"]:02} · {row["name"]}', font=font(15), fill='white')
        tile = icon(atlas, row['icon_cell'])
        sheet.paste(tile, (x + 84, y + 30), tile)
        for v, im in enumerate([front, back]):
            col = (row['model'] - 115) * 32
            # Only enlarge the review display; no resampling into a game asset.
            shell = im.crop((col, 0, col + 32, 32)).resize((128, 128), Image.Resampling.NEAREST)
            sheet.paste(shell, (x - 4 + v * 120, y + 93), shell)
        draw.text((x + 40, y + 218), 'Face ×4', font=font(12), fill=(174, 191, 200))
        draw.text((x + 160, y + 218), 'Dos ×4', font=font(12), fill=(174, 191, 200))
    return sheet


def default_heads():
    sheet = Image.new('RGB', (540, 286), BACKGROUND)
    draw = ImageDraw.Draw(sheet)
    draw.text((12, 9), 'Têtes des tenues · sans objet de casque ajouté', font=font(17), fill='white')
    for class_index, (_, name) in enumerate(CLASSES):
        draw.text((16 + 128 * class_index, 42), name, font=font(13), fill='white')
    for row, (outfit, label) in enumerate([('base', 'Tenue de base'), ('heavy', 'Armure lourde')]):
        y = 68 + 107 * row
        draw.text((12, y), label, font=font(12), fill=(174, 191, 200))
        for class_index in range(4):
            sheet.paste(figure(120, outfit, 'front', 0, class_index, fitted=False),
                        (12 + class_index * 128, y + 22))
    return sheet


def build():
    rows = inventory()
    OUT.mkdir(parents=True, exist_ok=True)
    atlas_path = ROOT / 'assets/sprites/equipment-native64/icons-64.png'
    atlas = Image.open(atlas_path).convert('RGBA')
    for class_index, (key, _) in enumerate(CLASSES):
        for outfit in ['base', 'heavy']:
            for phase in range(6):
                sheet = contact_sheet(rows, atlas, class_index, outfit, phase)
                sheet.save(OUT / f'inventory-{key}-{outfit}-{phase}.png')
    source_sheet(rows, atlas).save(OUT / 'models.png')
    default_heads().save(OUT / 'outfit-heads.png')
    files = ['src/game_art_catalog.rs', 'src/game_art_equipment_catalog.rs',
             'src/game_art_class_armor.rs', 'src/game_art_walk_rig.rs',
             'src/game_art_vertical_geometry.rs',
             'assets/sprites/equipment-native64/icons-64.png',
             'assets/sprites/class-armors-01/helmets.png',
             'assets/animations/vertical-18/heads/helmets-front.png',
             'assets/animations/vertical-18/heads/helmets-back.png']
    report = {'review': 'ANIM-POLISH01-HELM02-INVENTAIRE', 'date': '2026-10-31',
              'status': 'inventory_only_no_model_selection', 'native_tile_px': 64,
              'equipable_helmet_count': len(rows), 'helmets': rows,
              'wearer_profiles': 8, 'phase_count': 6, 'directions': 4,
              'contact_sheets': 48, 'source_capture_count': 288,
              'source_capture_scope': 'C06-HELM01-r01: four classes, base/heavy, original palette; fitted row',
              'retained_model_count': None, 'new_designs_created': False,
              'normal_game_changed': False,
              'view_sources': {'front': 'authored front shell', 'back': 'authored back shell',
                               'left_right': 'front shell projected by turned_helmet; not independent side drawings'},
              'legacy_technomancer': 'existing equipment supports it; outside the current four-class orientation capture pilot',
              'default_outfit_heads': 'separate class/torso appearance; not additional equipable helmet items',
              'inputs': {name: hashlib.sha256((ROOT/name).read_bytes()).hexdigest() for name in files}}
    (REVIEW / 'HELM02-INVENTAIRE.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    rows_html = ''.join(f'<li><strong>{row["number"]:02} · {escape(row["name"])}</strong></li>' for row in rows)
    classes = ''.join(f'<option value="{key}">{escape(name)}</option>' for key, name in CLASSES)
    section = f'''<section id="inventaire-casques"><h2>Inventaire · les six casques actuels</h2>
<p>Référence des six modèles existants, avant le choix de la nouvelle gamme. Le casque porté gardera le dessin de son icône sur tous les personnages, avec des dimensions et un placement adaptés à leur carrure et à leur armure, de face, de dos et des deux côtés.</p>
<ol class="helmet-list">{rows_html}</ol>
<details><summary>Voir les six icônes et les coques de face/dos</summary><div class="native" style="max-width:776px"><img src="helmet-inventory/current/models.png" width="776" height="610" style="display:block;max-width:none;image-rendering:pixelated" alt="Six modèles de casque : icônes natives et coques face/dos agrandies ×4"></div></details>
<div class="controls"><label>Classe<select id="inventory-class">{classes}</select></label><label>Tenue<select id="inventory-outfit"><option value="heavy">Armure lourde</option><option value="base">Tenue de base</option></select></label><label>Phase de marche<select id="inventory-phase">{''.join(f'<option value="{n}">{n+1} / 6</option>' for n in range(6))}</select></label></div>
<div class="native" style="max-width:776px"><img id="inventory-frame" src="helmet-inventory/current/inventory-bastion-heavy-0.png" width="776" height="742" style="display:block;width:776px;height:742px;image-rendering:pixelated" alt="Les six casques portés par le Bastion, face, dos, gauche et droite, à taille native"></div>
<details><summary>Zoom ×2 de la même planche</summary><div class="native"><img id="inventory-zoom" src="helmet-inventory/current/inventory-bastion-heavy-0.png" width="1552" height="1484" style="display:block;width:1552px;height:1484px;max-width:none;image-rendering:pixelated" alt="Même inventaire au zoom entier ×2"></div></details>
<p class="note">Les personnages et les icônes sont affichés à taille native. Les proportions 04 validées sont appliquées dans cette revue. La palette d’origine est conservée. Les côtés reprennent actuellement une projection de la coque frontale ; le dos possède un dessin distinct. Cette planche sert à examiner leur cohérence, sans présumer que tous les angles sont déjà satisfaisants.</p>
<details><summary>Comparer avec les têtes prévues par les tenues</summary><p class="note">Ces têtes appartiennent aux tenues et ne sont pas des objets de casque supplémentaires. Le casque équipé prend leur place. Le Technomancien historique reste hors de ce comparatif des quatre classes.</p><div class="native" style="max-width:540px"><img src="helmet-inventory/current/outfit-heads.png" width="540" height="286" style="display:block;max-width:none;image-rendering:pixelated" alt="Têtes des quatre classes en tenue de base et armure lourde, sans casque équipable ajouté"></div></details>
<p class="note">La nouvelle gamme HELM04 propose cinq dessins correspondant aux degrés de poids visuel des armures. Le casque restera le même objet reconnaissable sur chaque porteur : mêmes formes, ouvertures, matières et détails asymétriques. Sa taille sera jugée sur la silhouette complète du personnage avec son armure, pour éviter un casque trop petit. Elle restera stable pour un même équipement pendant la marche et entre les angles. Les proportions 04 montrées ici servent de base ; les prochains ajustements selon l’armure seront présentés avant intégration.</p></section>'''
    script = '''function showHelmetInventory(){const stem=`helmet-inventory/current/inventory-${$('inventory-class').value}-${$('inventory-outfit').value}-${$('inventory-phase').value}.png`;const label=$('inventory-class').selectedOptions[0].textContent;for(const id of ['inventory-frame','inventory-zoom']){const el=$(id);el.src=stem;el.alt=`Les six casques portés par ${label}, de face, de dos, à gauche et à droite`;}}for(const id of ['inventory-class','inventory-outfit','inventory-phase'])$(id).addEventListener('change',showHelmetInventory);'''
    return section, script


if __name__ == '__main__':
    build()
    print(json.dumps({'inventory': str(OUT), 'contact_sheets': 48, 'helmet_models': 6}))
