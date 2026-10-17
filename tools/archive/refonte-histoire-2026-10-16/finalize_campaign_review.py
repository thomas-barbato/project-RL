"""Freeze review evidence and prepare a scoped cleanup plan. Never deletes files."""
import hashlib
import json
import shutil
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
DURABLE = ROOT/'docs/reviews/refonte-histoire-2026-10-16/bestiary-integration-06'
SOURCES = ROOT/'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01'
BANK = ROOT/'assets/sprites/campaign-monsters'
WORK = ROOT/'artifacts/npc-monsters-review-01'
REMOVED = {13, 23, 27, 91, 163}
PREFIXES = ('CULT-01-', 'TROUP-02-', 'TROUP-06-', 'ARPG-32-', 'DEM-H04-')

def read(path):
    return json.loads(path.read_text(encoding='utf-8'))

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')

def relative(path):
    return path.relative_to(ROOT).as_posix()

def freeze():
    listing = read(REVIEW/'monster-candidate-list.json')
    batch = read(REVIEW/'two-handed-cultists-29.json')
    for record in batch['records']:
        record.update(visual_approved=True, scale_approved=True, runtime_authorized=True,
                      acceptance_statement='Valider les deux', accepted_date='2026-10-17')
    batch.update(status='visually_accepted', acceptance_statement='Valider les deux')
    write(REVIEW/'two-handed-cultists-29.json', batch)
    records = {r['number']:r for r in listing['records'] if r['number'] not in REMOVED}
    for record in batch['records']:
        records[record['number']] = dict(record, review='two-handed-cultists-29.json',
            code=f"{record['id']}-{record['revision']}", section='Cultistes humains')
    manifest = read(BANK/'manifest.json')
    by_number = {}
    for entry in manifest['entries']:
        by_number.setdefault(entry['number'], []).append(entry)
    for number, record in records.items():
        record['runtime_authorized'] = number not in {18,19}
        record['runtime_connected'] = number in by_number and by_number[number][0]['role'] != 'ReservedCaster'
        if record['section'] in {'Cultistes humains', 'Cultistes possédés', 'Possédés et hybrides'}:
            record.setdefault('review_label', record['label'])
            record['label'] = 'Cultiste' if record['section'] == 'Cultistes humains' else 'Possédé'
        if number in by_number:
            entries = by_number[number]
            record['variants'] = [dict(variant=chr(65+e['variant']), file=relative(BANK/e['tile']),
                sha256=e['sha256'], cell=64, canvas=e['side'], foot_anchor=e['foot_anchor'], occupied=e['bounds']) for e in entries]
            if number == 156:
                e = entries[0]
                record.update(source=e['source'], source_sha256=e['source_sha256'], revision='r02', code='DEM-F01-r02')
        else:
            # Sector bosses stay review-only, with durable native pixels.
            for variant in record['variants']:
                original = ROOT/variant['file']
                target = SOURCES/'native64-review-only'/original.name
                if original != target:
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(original,target)
                variant['file'] = relative(target)
                variant['sha256'] = sha(target)
    for number in (144,150):
        record = records[number]
        base = record['variants'][0]
        record['variants'] = [v for v in record['variants'] if not v.get('runtime_palette')]
        for variant in (2,3):
            original = ROOT/f'artifacts/campaign-bestiary-integration-06/{number}-palette-{variant}.png'
            target = BANK/f'tiles/{number}-palette-{variant}.png'
            shutil.copyfile(original,target)
            a, b = Image.open(ROOT/base['file']).convert('RGBA'), Image.open(target).convert('RGBA')
            if a.getchannel('A').tobytes() != b.getchannel('A').tobytes():
                b = b.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
                b.save(target,optimize=True)
            assert a.size == b.size and a.getchannel('A').tobytes() == b.getchannel('A').tobytes()
            assert a.tobytes() != b.tobytes()
            record['variants'].append(dict(base,variant=chr(65+variant),file=relative(target),sha256=sha(target),runtime_palette=True))
    listing.update(records=list(sorted(records.values(),key=lambda r:r['number'])),candidate_count=len(records),
        latest_batch='two-handed-cultists-29.json',runtime_connected=True,status='authorized_integration_with_reserved_powers',
        integration_authorization=relative(DURABLE/'integration.json'),individual_acceptance_deferred=False)
    assert len(records) == 131
    write(REVIEW/'monster-candidate-list.json',listing)
    write(DURABLE/'integration.json',dict(review_id='BESTIARY-INTEGRATION-06',date='2026-10-17',
        user_request='on va integrer ca au jeu avec les monstres mais il faut aussi faire les variantes de couleurs comme définies :) puis tu fera le ménage dans les fichiers de generation etc , une fois qu’on en aura plus besoin',
        scope_answer='Intégrer maintenant les visuels, les couleurs et les règles déjà validées ; compléter les pouvoirs ensuite.',
        additional_acceptance={'numbers':[164,165],'answer':'Valider les deux'},
        removed_numbers=sorted(REMOVED),deferred_sector_bosses=[18,19],
        native_cell=64,source_policy='Original drawings and frozen native exports kept in assets; no runtime dependency on artifacts.',
        palette_policy='Explicit material color maps preserve silhouette, alpha, clothing, metal and horns. Color alone grants no unapproved power.',
        approved_physical_resistance={'review_id':'MM21','percentage_points':30,'cap':75,'order':'after physical Armor','mixed_attacks':'physical components only'},
        deferred='Unquantified damage bonuses, auras, curses, poison powers and caster AI; sector boss patterns.',
        generation=150,legacy_policy='No actor/save layout extension; new appearances and traits only on new generation 150. Model, palette and corpse identity persist.',
        image_generation=[
            {'number':156,'source':'DEM-F01-r02.png','instruction':'Keep the frost demon unchanged. Recolor only its neck wrap and loincloth in muted dark brown; preserve blue flesh, ice, horns, claws, silhouette and transparent background.'},
            {'number':164,'source':'CULT-08-r01.png','instruction':'Ordinary human cultist with grey hooded uniform, muted red panel and opaque ivory mask, no visible face. Hold a straight two-handed rifle correctly, both hands gripping stock and fore-end. Match the outlined cel-shaded player art; readable restrained details, complete body on transparent background.'},
            {'number':165,'source':'CULT-09-r01.png','instruction':'Matching ordinary human cultist with grey hooded uniform, muted red panel and opaque ivory mask. Carry a two-handed axe with a straight shaft, two distinct plausible hand grips. Readable restrained cel shading matching the player, full body, transparent background.'}],
        image_generation_note='Instructions above summarize the three built-in image generation edits; native exports are frozen and hash-verified.'))
    print(f'Frozen {len(records)} catalogue models; two new cultists explicitly accepted.')

def cleanup_plan():
    mapping, targets = [], []
    known = {sha(p):p for p in (BANK/'tiles').glob('*.png')}
    listing = read(REVIEW/'monster-candidate-list.json')
    active = {ROOT/listing['catalogue_image'],ROOT/listing['catalogue_viewer']}
    active.update(ROOT/r['contact_shadow']['file'] for r in listing['records'])
    for path in sorted(WORK.rglob('*')):
        if not path.is_file() or path in active:
            continue
        if path.suffix.lower()=='.png' and not path.name.startswith(PREFIXES):
            with Image.open(path) as image:
                native = image.width == image.height and image.width <= 512 and image.mode == 'RGBA'
            if native:
                digest = sha(path)
                target = known.get(digest, SOURCES/'frozen-review-exports'/path.relative_to(WORK))
                if not target.exists():
                    target.parent.mkdir(parents=True,exist_ok=True)
                    shutil.copyfile(path,target)
                assert sha(target)==digest
                mapping.append(dict(old=relative(path),durable=relative(target),sha256=digest))
                known[digest]=target
        if path.suffix.lower() in {'.json','.md'}:
            target = DURABLE/'legacy-build-metadata'/path.relative_to(WORK)
            target.parent.mkdir(parents=True,exist_ok=True)
            shutil.copyfile(path,target)
        targets.append(dict(path=relative(path),bytes=path.stat().st_size,sha256=sha(path)))
    for path in SOURCES.rglob('*.png'):
        if path.name.startswith(PREFIXES):
            targets.append(dict(path=relative(path),bytes=path.stat().st_size,sha256=sha(path),reason='explicit user refusal'))
    write(DURABLE/'cleanup-plan.json',dict(review_id='BESTIARY-CLEANUP-06',native_export_relocations=mapping,
        targets=targets,total_bytes=sum(t['bytes'] for t in targets),
        preserved=['artifacts/map-editor/','all source drawings except explicitly refused models','current catalogue','compiled game and legacy assets']))
    print(f'Cleanup prepared: {len(targets)} files, {sum(t["bytes"] for t in targets):,} bytes; {len(mapping)} native exports preserved.')

if __name__ == '__main__':
    import sys
    cleanup_plan() if '--cleanup-plan' in sys.argv else freeze()
