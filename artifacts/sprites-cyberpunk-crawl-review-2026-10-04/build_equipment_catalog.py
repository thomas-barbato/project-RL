"""Read the local equipment definitions to build a review-only model catalogue."""
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
locale_paths = sorted((ROOT / 'content/core/locales').glob('en*.json5')) + sorted((ROOT / 'content/core/locales').glob('fr*.json5'))
for path in locale_paths:
    for match in re.finditer(r'(' + STRING + r')\s*:\s*(' + STRING + r')', path.read_text(encoding='utf-8-sig')):
        locales[json.loads(match.group(1))] = json.loads(match.group(2))

families = {}
for path in sorted((ROOT / 'content/core/equipment_loot').glob('*.json5')):
    source = path.read_text(encoding='utf-8-sig')
    for match in re.finditer(r'(?:"item"|\bitem\b)\s*:\s*"([^"]+)"\s*,\s*(?:"family"|\bfamily\b)\s*:\s*"([^"]+)"', source):
        families[match.group(1)] = match.group(2)

records = []
for folder in ['weapons', 'items']:
    for path in sorted((ROOT / f'content/core/{folder}').glob('*.json5')):
        source = path.read_text(encoding='utf-8-sig')
        if folder == 'items' and field(source, 'kind') != 'armor':
            continue
        ident = field(source, 'id')
        if not ident:
            raise ValueError(f'Missing id: {path}')
        name_key = field(source, 'name_key')
        if not name_key:
            raise ValueError(f'Missing display-name key: {path}')
        display_name = locales.get(name_key, ident.split(':', 1)[1].replace('_', ' ').title())
        family = families.get(ident, '')
        if folder == 'items':
            slot = field(source, 'slot') or 'core:body_armor'
            category = {'core:head_armor':'head','core:hand_armor':'hands','core:foot_armor':'feet'}.get(slot,'body')
            group = 'armor'
        else:
            name = display_name.lower()
            if ident == 'core:integrity_blade':
                name = 'lame d’intégrité'
            melee = (family in ['core:knives','core:swords','core:spears','core:axes','core:maces'] or
                     bool(re.search(r'couteau|lame|dague|stylet|poignard|épée|glaive|fauchon|espadon|lance$|épieu|pique|vouge|pertuisane|hallebarde|hache|bardiche|gourdin|masse|marteau|corbin',name)))
            group = 'melee' if melee else 'firearms'
            category = family.removeprefix('core:') or group
            slot = 'weapon'
        records.append(dict(id=ident,name=display_name,name_key=name_key,name_localized=name_key in locales,group=group,category=category,
                            family=family,slot=slot,source=path.relative_to(ROOT).as_posix(),
                            visual_status='proposal_pending',affixes_included=False))

order = ['firearms','melee','armor']
slot_order = {'body':0,'head':1,'hands':2,'feet':3}
records.sort(key=lambda r:(order.index(r['group']),slot_order.get(r['category'],0) if r['group']=='armor' else r['category'],r['name']))
for group in order:
    members = [r for r in records if r['group']==group]
    for index, record in enumerate(members):
        atlas = f'equipment-{group}-v1.png'
        atlas_index = index
        rows = (len(members)+7)//8
        if group == 'firearms':
            atlas = f'equipment-firearms-{index//32+1}-v2.png'
            atlas_index = index % 32
            rows = 4
        record.update(atlas=atlas,index=atlas_index,columns=8,rows=rows,
                      visual_status='generated_candidate_pending_review')

assert len({r['id'] for r in records}) == len(records), 'Duplicate base IDs'
catalogue = dict(version=1,scope='all local core weapon definitions and armor item definitions; base models only',
                 counts={group:sum(r['group']==group for r in records) for group in order},models=records)
(OUT/'equipment-catalog.json').write_text(json.dumps(catalogue,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
(OUT/'equipment-catalog.js').write_text('window.equipmentCatalogData = '+json.dumps(catalogue,ensure_ascii=False)+';\n',encoding='utf-8')
print(json.dumps(catalogue['counts'],ensure_ascii=False))
for group in order:
    print('\n'+group.upper())
    for record in records:
        if record['group']==group:
            print(f"{record['index']+1:02} {record['name']} [{record['category']}] ({record['id']})")
