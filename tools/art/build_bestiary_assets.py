"""Pack the approved bestiary, companions and transformations without repainting.

Native review PNGs are copied byte for byte. This bank defines no species,
statistics, spawns or talents. Legacy actor indices remain save-compatible.
"""
import argparse
import hashlib
import io
import json
import math
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OLD = ROOT / 'artifacts/bestiaire-proposition-2026-10-05'
LATEST = ROOT / 'artifacts/allies-transformations-proposition-2026-10-06'
OUT = ROOT / 'assets/sprites/bestiary'
REVIEW = ROOT / 'artifacts/bestiaire-integration-2026-10-06'
COLS = 16


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load(path):
    return json.loads(path.read_text(encoding='utf-8'))


def encoded(value):
    return (json.dumps(value, ensure_ascii=False, indent=2) + '\n').encode('utf-8')


def pack(images, side):
    atlas = Image.new('RGBA', (COLS * side, math.ceil(len(images) / COLS) * side))
    for index, image in enumerate(images):
        # Copy all RGBA channels, including transparent pixels, without compositing.
        atlas.paste(image, ((index % COLS) * side, (index // COLS) * side))
    stream = io.BytesIO()
    atlas.save(stream, 'PNG', optimize=True)
    return stream.getvalue()


def build():
    approval = load(REVIEW / 'integration-authorization.json')
    originals = {e['number']: e for e in load(OLD / 'manifest.json')}
    revisions = {e['number']: e for e in load(LATEST / 'manifest.json')}
    numbers = sorted(originals.keys() | revisions.keys())
    assert numbers == list(range(1, 60)) + list(range(61, 67))
    assert approval['accepted_numbers'] == numbers
    legacy_atlas = Image.open(ROOT / 'assets/sprites/cyberpunk/actors-32.png').convert('RGBA')
    legacy_contacts = load(ROOT / 'assets/sprites/cyberpunk/manifest.json')['actor_contacts']
    images = {(side, kind): [] for side in (32, 64) for kind in ('living', 'corpse')}
    outputs, entries, legacy = {}, [], [0] * 27
    for number in numbers:
        original = originals.get(number, {})
        revision = revisions.get(number, {})
        entry = original | revision
        folder = LATEST if number in revisions else OLD
        approved = approval['approved_art'][str(number)]
        assert approved['source_review'] == folder.relative_to(ROOT).as_posix()
        assert approved['source'] == entry['source']
        assert approved['source_sha256'] == sha((folder / entry['source']).read_bytes()), f'Unreviewed source: {number}'
        side = entry['tile_size']
        assert side in (32, 64)
        actor = original.get('actor_index')
        slot = len(images[side, 'living'])
        art_id = f'rl-art:{number:03d}'
        record = dict(
            number=number, art_id=art_id, name=entry['name'],
            name_status=entry.get('name_status', 'existing'), family=entry['family'],
            usage=entry.get('usage', 'enemy' if number < 33 else 'existing-actor'),
            tile_size=side, atlas_index=slot, atlas_columns=COLS,
            bleeds=entry['corpse_includes_blood'],
            corpse_includes_blood=entry['corpse_includes_blood'],
            content_id=original.get('content_id', entry.get('content_id')),
            legacy_actor_index=actor,
            status='integrated-visual',
            gameplay_status='existing actor unchanged' if actor is not None else 'visual resource; gameplay not defined',
            approval='integration-authorization.json', source_review=folder.relative_to(ROOT).as_posix(),
            source=entry['source'], source_sha256=sha((folder / entry['source']).read_bytes()),
        )
        for kind in ('living', 'corpse'):
            path = folder / entry[kind]
            raw = path.read_bytes()
            assert sha(raw) == approved[kind + '_sha256'], f'Unreviewed {kind}: {number}'
            image = Image.open(io.BytesIO(raw)).convert('RGBA')
            assert image.size == (side, side), path
            assert image.getbbox(), path
            pixels = image.tobytes()
            assert set(pixels[3::4]) <= {0, 255}, path
            colors = {pixels[i:i + 3] for i in range(0, len(pixels), 4) if pixels[i + 3]}
            assert len(colors) <= (32 if side == 64 else 24), (path, len(colors))
            filename = f'tiles/{number:03d}-{kind}-{side}.png'
            outputs[filename] = raw
            record[kind] = filename
            record[kind + '_sha256'] = sha(raw)
            record[kind + '_bounds'] = list(image.getbbox())
            left, top, right, bottom = image.getbbox()
            record[kind + '_shadow_contact'] = (
                legacy_contacts[actor] if actor is not None and kind == 'living' else
                [(left + right) // 2, side - 1 if kind == 'living' else bottom - 1,
                 max(8, min(side - 2, round((right - left) * (0.78 if kind == 'living' else 0.95))))]
            )
            images[side, kind].append(image)
            if actor is not None and kind == 'living':
                cut = legacy_atlas.crop((actor % 16 * 32, actor // 16 * 32,
                                         actor % 16 * 32 + 32, actor // 16 * 32 + 32))
                assert cut.tobytes() == image.tobytes(), f'Legacy living actor changed: {number}'
        if actor is not None:
            assert legacy[actor] == 0
            legacy[actor] = number
        entries.append(record)
    assert len(entries) == 65
    assert sum(e['usage'] == 'player-companion' for e in entries) == 5
    assert sum(e['usage'] == 'player-transformation' for e in entries) == 5
    assert len([e for e in entries if e['legacy_actor_index'] is not None]) == 23
    for (side, kind), cells in images.items():
        outputs[f'{kind}-{side}.png'] = pack(cells, side)
    manifest = dict(version=1, native_tile=32, atlas_columns=COLS,
                    authorization_date=approval['date'], entries=entries,
                    retired_numbers=[60], review_aliases={'60': 66},
                    bosses='64 by 64 native images; visual footprint only',
                    corpse_layer='below loot; native authored blood or dry wreck',
                    simulation='No species, stats, collisions, encounters or talents added')
    outputs['manifest.json'] = encoded(manifest)
    lines = ['// Generated by tools/art/build_bestiary_assets.py. Native art only.',
             f'pub(super) const ENTRIES: [BankEntry; {len(entries)}] = [']
    for e in entries:
        live_shadow = str(e['living_shadow_contact'])
        dead_shadow = str(e['corpse_shadow_contact'])
        lines.append(f'    BankEntry {{ number: {e["number"]}, side: {e["tile_size"]}, index: {e["atlas_index"]}, bleeds: {str(e["bleeds"]).lower()}, live_shadow: {live_shadow}, dead_shadow: {dead_shadow} }},')
    lines += ['];', f'pub(super) const LEGACY_CORPSES: [u8; {len(legacy)}] = [{", ".join(map(str, legacy))}];']
    outputs['catalog.rs'] = ('\n'.join(lines) + '\n').encode('utf-8')
    outputs['README.md'] = (
        '# Banque visuelle intégrée — 6 octobre 2026\n\n'
        '65 références : 32 nouveaux ennemis (dont 4 boss de 64 px), 23 acteurs existants, '
        '5 drones et 5 transformations. Chaque référence possède son vivant et son cadavre/épave. '
        'Le numéro 60 est retiré et renvoie au 66 dans les métadonnées de revue.\n\n'
        'Les PNG natifs sont copiés sans modification. `manifest.json` conserve numéros, '
        'identifiants visuels, sources et empreintes. Les noms proposés restent provisoires. '
        'Aucune nouvelle espèce, statistique, apparition, collision ou capacité n’est définie. '
        'Les 23 acteurs déjà présents utilisent leur cadavre dessiné dans le rendu actif. '
        'Les machines ne saignent pas. Une ombre de contact séparée accompagne chaque vivant et chaque cadavre ; '
        'les sources PNG restent intactes. Les autres modèles sont disponibles pour leurs futures règles.\n\n'
        'Reconstruction : `python tools/art/build_bestiary_assets.py`. '
        'Vérification : même commande avec `--check`. '
        'Autorisation : `artifacts/bestiaire-integration-2026-10-06/integration-authorization.json`.\n'
    ).encode('utf-8')
    return outputs


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    outputs = build()
    if args.check:
        stale = [name for name, data in outputs.items()
                 if not (OUT / name).is_file() or (OUT / name).read_bytes() != data]
        if stale:
            raise SystemExit('Out of date: ' + ', '.join(stale))
    else:
        for name, data in outputs.items():
            target = OUT / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    print(f'Bestiary: 65 references, 130 exact native PNGs, 4 atlases; {"reproducible" if args.check else "packaged"}')


if __name__ == '__main__':
    main()
