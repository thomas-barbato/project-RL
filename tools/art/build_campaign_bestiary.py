"""Package the requested 64px campaign bank; runtime never reads artifacts/.

First import freezes existing native review exports without touching their
pixels. Subsequent builds and --check use only the durable assets/ bank.
The only new source conditioning is the requested brown-cloth frost revision.
"""
import argparse
import hashlib
import json
import math
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT = ROOT / 'assets/sprites/campaign-monsters'
REMOVED = {13, 23, 27, 91, 163}
DEFERRED_BOSSES = {18, 19}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def frost_revision():
    path = ROOT / 'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/DEM-F01-r02.png'
    image = Image.open(path).convert('RGBA')
    image.putalpha(image.getchannel('A').point(lambda a: 255 if a >= 128 else 0))
    body = image.crop(image.getbbox())
    ratio = min(92 / body.height, 88 / body.width)
    body = body.resize((round(body.width * ratio), round(body.height * ratio)), Image.Resampling.LANCZOS)
    alpha = body.getchannel('A').point(lambda a: 255 if a >= 128 else 0)
    body = body.convert('RGB').quantize(24, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE).convert('RGBA')
    body.putalpha(alpha)
    result = Image.new('RGBA', (128, 128))
    result.alpha_composite(body, (64 - body.width // 2, 96 - body.height))
    return path, result


def authored_role(record, source):
    n, group = record['number'], record['section']
    if group.startswith('Nécromanciens') or n in {11, 15, 17, 25, 117}:
        return 'ReservedCaster'
    if group == 'Cultistes humains':
        return 'CultistRanged' if n in {14, 24, 164} else 'CultistMelee'
    if group in {'Possédés et hybrides', 'Cultistes possédés'}:
        return 'PossessedRanged' if n in {10, 96, 98} else 'PossessedMelee'
    if group == 'Esprits torturés':
        return 'Spirit'
    role = (source.get('role', '') + ' ' + record['label']).lower()
    if any(term in role for term in ('distance', 'ranged', 'cracheur', 'projectile', 'vomisseur', 'archer', 'tireur')):
        return 'DemonRanged'
    if any(term in role for term in ('soutien', 'support', 'invoc', 'caster', 'magicien', 'sorcier', 'prophète')):
        return 'ReservedCaster'
    return 'DemonMelee'


def import_review():
    catalog = json.loads((REVIEW / 'monster-candidate-list.json').read_text(encoding='utf-8'))
    (OUT / 'tiles').mkdir(parents=True, exist_ok=True)
    entries = []
    batches = {}
    for record in catalog['records']:
        number = record['number']
        if number in REMOVED | DEFERRED_BOSSES or record.get('runtime_authorized') is False:
            continue
        batch_file = record['review']
        if batch_file not in batches:
            batches[batch_file] = json.loads((REVIEW / batch_file).read_text(encoding='utf-8'))
        source_record = next((r for r in batches[batch_file].get('records', []) if r.get('number') == number), {})
        role = authored_role(record, source_record)
        generic_name = ('Cultiste' if role.startswith('Cultist') or record['section'] == 'Nécromanciens scientifiques' else
                        'Possédé' if role.startswith('Possessed') else 'Zombie' if record['section'] == 'Zombies' else
                        'Squelette' if record['section'] == 'Squelettes' else 'Esprit torturé' if role == 'Spirit' else 'Démon')
        for variant, info in enumerate(record['variants']):
            if info.get('runtime_palette'):
                continue
            if number == 156:
                if variant:
                    continue
                source_path, image = frost_revision()
                native = OUT / 'tiles/156-r02-A.png'
                image.save(native, optimize=True)
                source = source_path.relative_to(ROOT).as_posix()
                revision = 'r02'
            else:
                original = ROOT / info['file']
                assert sha(original) == info['sha256'], original
                native = OUT / 'tiles' / f'{number:03d}-{record["revision"]}-{info["variant"]}.png'
                native.write_bytes(original.read_bytes())
                source = record['source']
                revision = record['revision']
            image = Image.open(native).convert('RGBA')
            assert set(image.getchannel('A').get_flattened_data()) <= {0, 255}
            entries.append(dict(number=number, variant=variant, code=record['code'], revision=revision,
                review_label=record['label'], display_name=generic_name, section=record['section'],
                role=role, cell=64, side=image.width, foot_anchor=info['foot_anchor'],
                tile=native.relative_to(OUT).as_posix(), sha256=sha(native),
                source=source, source_sha256=sha(ROOT/source), bounds=list(image.getbbox())))
    manifest = dict(version=1, native_tile=64, atlas_columns=8,
        authorization='docs/reviews/refonte-histoire-2026-10-16/bestiary-integration-06/integration.json', removed_numbers=sorted(REMOVED),
        deferred_boss_numbers=sorted(DEFERRED_BOSSES),
        note='Artwork integration requested; reserved caster art has no invented abilities. Review labels are not canonical monster names.',
        entries=entries)
    (OUT/'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    return manifest


def build(manifest, check):
    entries = manifest['entries']
    generated = []
    for side in sorted({e['side'] for e in entries}):
        family = [e for e in entries if e['side'] == side]
        atlas = Image.new('RGBA', (side*8, side*math.ceil(len(family)/8)))
        for index, entry in enumerate(family):
            path = OUT/entry['tile']
            assert sha(path) == entry['sha256'], path
            atlas.alpha_composite(Image.open(path).convert('RGBA'), ((index % 8)*side, (index//8)*side))
            entry['atlas_index'] = index
            x, y = entry['foot_anchor']
            width = min(63, max(12, round((entry['bounds'][2]-entry['bounds'][0])*.6)))
            generated.append(f'    MonsterArt {{ number: {entry["number"]}, variant: {entry["variant"]}, side: {side}, index: {index}, foot: [{x}, {y}], shadow: {width}, role: ArtRole::{entry["role"]}, name: {json.dumps(entry["display_name"],ensure_ascii=False)} }},')
        path = OUT / f'living-{side}.png'
        if check:
            with Image.open(path) as old:
                assert old.size == atlas.size and old.convert('RGBA').tobytes() == atlas.tobytes(), path
        else:
            atlas.save(path, optimize=True)
    code = '// Generated by tools/art/build_campaign_bestiary.py; stable review identities.\n'
    code += 'pub const MONSTER_ART: &[MonsterArt] = &[\n' + '\n'.join(generated) + '\n];\n'
    path = OUT / 'catalog.rs'
    if check:
        assert path.read_text(encoding='utf-8') == code, path
    else:
        path.write_text(code, encoding='utf-8')
        (OUT/'manifest.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(f'{len(set(e["number"] for e in entries))} models, {len(entries)} native variants; {"checked" if check else "packaged"}.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--import-review', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    manifest = import_review() if args.import_review else json.loads((OUT/'manifest.json').read_text(encoding='utf-8'))
    build(manifest, args.check)
