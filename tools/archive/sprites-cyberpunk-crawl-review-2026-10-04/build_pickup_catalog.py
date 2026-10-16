"""Review-only icon catalogue for every core non-equipment item definition."""
from pathlib import Path
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
STRING = r'"(?:[^"\\]|\\.)*"'

def field(source, key):
    match = re.search(r'(?:"' + re.escape(key) + r'"|\b' + re.escape(key) + r'\b)\s*:\s*(' + STRING + r')', source)
    return json.loads(match.group(1)) if match else None

locales = {}
for path in sorted((ROOT / 'content/core/locales').glob('fr*.json5')):
    for match in re.finditer(r'(' + STRING + r')\s*:\s*(' + STRING + r')', path.read_text(encoding='utf-8-sig')):
        locales[json.loads(match.group(1))] = json.loads(match.group(2))

subjects = {
    'repair_patch': 'Compact medical box, red lid and red corner reinforcements, ivory front and orange medical plus. Red replaces the dark shell so its whole shape is visible on the floor.',
    'weapon_matter': 'Simple green crate, green lid and sides, broad shading to show volume. No cartridges, handles or small accessories.',
    'charged_battery': 'ONE chunky electric battery cartridge, dark graphite block with two large brass contacts on top, cyan charge strip on front, ivory protective corners.',
    'power_regulator': 'ONE industrial power regulator, squat ivory circular casing with cyan center socket, three chunky copper connectors around the rim.',
    'salvaged_component': 'ONE salvaged electronic component, asymmetrical olive circuit block with one large exposed copper coil and a broken connector. Visibly used.',
    'unstable_fragment': 'ONE irregular angular fragment of unusual material, dark violet facets with small teal and ivory highlights, solid physical shard, no particles or magical glow.',
    'repair_parts': 'ONE open shallow parts tray with three large readable pieces: steel gear, bent plate and copper connector. Gray and rust industrial parts, no medical symbol.',
    'tuning_parts': 'ONE tuning interface bundle, two slim ivory rectangular plug modules connected by a short dark cable loop and one cyan contact, no medical symbol.',
    'engineering_tool': 'ONE compact folded industrial multitool, graphite body, orange grips, two short visible chunky diagnostic prongs, rugged instrument silhouette, no medical symbol.',
    'fragmentation_charge': 'ONE compact fragmentation explosive with segmented olive shell, metal safety lever and broad orange safety band, no letters.',
    'breach_charge': 'ONE narrow rectangular breaching charge with two exposed copper focusing strips, graphite backing and small orange firing connector.',
    'proximity_mine': 'ONE flat circular proximity mine, low graphite disc with three ivory feet and one small amber raised sensor at the center, no explosion.',
    'configurable_charge': 'ONE wedge-shaped configurable explosive, ivory angular shell, orange dial on sloping front, graphite receiver stub and thick latches.',
    'structural_charge': 'ONE heavy demolition block, broad stacked rust-colored plates tied together with two dark straps and a single small amber trigger unit.',
    'sound_decoy': 'ONE compact sound emitter, dark upright speaker cylinder with broad circular grille, ivory base and two little orange controls.',
    'camouflage_bundle': 'ONE folded camouflage fabric roll, muted olive textured layers held by two graphite straps, one loose fabric corner, no magical effects.',
    'saturation_beacon': 'ONE squat tripod field beacon, graphite and ivory pedestal with a cyan glass cap and three short spread legs, no projected field or particles.',
}
models = []
for path in sorted((ROOT / 'content/core/items').glob('*.json5')):
    source = path.read_text(encoding='utf-8-sig')
    kind = field(source, 'kind')
    if kind == 'armor':
        continue
    ident = field(source, 'id')
    key = ident.split(':', 1)[1]
    name_key = field(source, 'name_key')
    # The user identified the legacy "repair kit" as a first-aid kit.
    name = 'Boîte de soins' if key == 'repair_patch' else locales.get(name_key, key.replace('_', ' ').title())
    if key not in subjects:
        raise ValueError(f'No art brief for {ident}')
    stack = re.search(r'\bmaximum_stack\s*:\s*(\d+)', source)
    models.append(dict(id=ident, name=name, name_key=name_key, description=locales.get(field(source, 'description_key'), ''),
                       group='supplies', category='consumable' if kind == 'consumable' else 'material',
                       source=path.relative_to(ROOT).as_posix(), maximum_stack=int(stack.group(1)) if stack else 1,
                       atlas=f'pickup-{key}-v{dict(repair_patch=5, weapon_matter=5, charged_battery=2).get(key, 1)}.png', index=0, columns=1, rows=1,
                       search_aliases=['trousse de secours', 'repair kit', 'kit de soins'] if key == 'repair_patch' else [],
                       visual_status='generated_candidate_pending_review', affixes_included=False, art_subject=subjects[key]))
models.sort(key=lambda m: (m['id'] != 'core:repair_patch', m['name']))
assert len({m['id'] for m in models}) == len(models)
equipment = json.loads((OUT / 'equipment-catalog.json').read_text(encoding='utf-8'))['models']
actual = set()
for folder in ['items', 'weapons']:
    for path in (ROOT / f'content/core/{folder}').glob('*.json5'):
        actual.add(field(path.read_text(encoding='utf-8-sig'), 'id'))
assert actual == {m['id'] for m in equipment + models}, 'Incomplete pickup coverage'
catalogue = dict(version=1, scope='all local core non-equipment item definitions', count=len(models), total_pickup_count=len(actual), models=models)
(OUT / 'pickup-catalog.json').write_text(json.dumps(catalogue, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
(OUT / 'pickup-catalog.js').write_text('window.pickupCatalogData = ' + json.dumps(catalogue, ensure_ascii=False) + ';\n', encoding='utf-8')
print(f'{len(models)} supplies; {len(actual)} total base references covered')
