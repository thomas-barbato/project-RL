"""Review the approved shorter bed and remove the five baked riot props."""
import json
import sys

from PIL import Image, ImageChops, ImageStat

from build_city_cyberpunk12 import ART, DOC, ROOT, read, sha

sys.path.insert(0, str(ROOT / 'tools/reviews'))
import build_street_paving_review as paving
from integrate_street_repairs_09 import REMOVED

OUT = ART / 'mobilier-r01'
FLOOR = ROOT / 'assets/source-art/floor-continuous-01'
REPAIR = ROOT / 'assets/source-art/first-sector-review-01/u01-repairs-r09'


def relative(path):
    return path.relative_to(ROOT).as_posix()


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def mirrored(tile):
    w, h = tile.size
    result = Image.new(tile.mode, (2 * w, 2 * h))
    result.paste(tile, (0, 0))
    result.paste(tile.transpose(Image.Transpose.FLIP_LEFT_RIGHT), (w, 0))
    result.paste(tile.transpose(Image.Transpose.FLIP_TOP_BOTTOM), (0, h))
    result.paste(tile.transpose(Image.Transpose.ROTATE_180), (w, h))
    return result


def recompose_frozen_street(size, frozen, *, omit_decor=(), omit_modules=()):
    # Replay the accepted exterior composition from SOL-CONT01, with the same
    # material sampling, rotations and props. Never install the recomposition.
    texture = Image.open(FLOOR / 'material-source.png').convert('RGB')
    atlas = mirrored(texture.resize((512, 512), Image.Resampling.LANCZOS))
    whole = Image.new('RGB', size)
    for y in range(0, size[1], atlas.height):
        for x in range(0, size[0], atlas.width):
            whole.paste(atlas, (x, y))
    tiles = [Image.open(paving.ART / f'{name}-native.png').convert('RGBA')
             for name in ('PAV-U01-06', 'PAV-U01-06B', 'PAV-U01-06C')]
    mean = ImageStat.Stat(whole).mean
    channels = [channel.point([min(255, int(v * old_mean / new_mean)) for v in range(256)])
                for channel, old_mean, new_mean in zip(whole.split(), ImageStat.Stat(tiles[2]).mean, mean)]
    whole = Image.merge('RGB', channels).convert('RGBA')
    turns = [None, Image.Transpose.ROTATE_270, Image.Transpose.ROTATE_180, Image.Transpose.ROTATE_90]
    overrides = {}
    for y in range(size[1] // 64):
        for x in range(size[0] // 64):
            key = ((x * 0x1f123bb5) ^ (y * 0x5f356495) ^ 0x6a09e667) & 0xffffffff
            key ^= key >> 16
            key = (key * 0x45d9f3b) & 0xffffffff
            key ^= key >> 16
            patch = whole.crop((x * 64, y * 64, x * 64 + 64, y * 64 + 64))
            turn = turns[(key >> 12) % 4]
            overrides[(x, y)] = patch if turn is None else patch.transpose(turn)
    road = Image.open(REPAIR / 'ROAD-U01-09-native.png').convert('RGBA')
    options = dict(paving=[tiles[0]] + [tiles[1]] * 6 + [tiles[2]] * 5,
                   grass=read(paving.ART / 'composition-checks.json')['grass_at_px'],
                   warehouse=REPAIR / 'warehouse-native.png',
                   asphalt=mirrored(road.resize((128, 128), Image.Resampling.LANCZOS)),
                   repair_join=True, omit_modules=REMOVED, paving_overrides=overrides,
                   arrival_override=Image.open(FLOOR / 'arrival-source.png').convert('RGBA'))
    rebuilt = paving.compose(**options)
    assert not ImageChops.difference(rebuilt, frozen).getbbox(alpha_only=False)
    options['omit_modules'] = [*REMOVED, *omit_modules]
    return paving.compose(**options, omit_decor=omit_decor)


def view(ident, name, rect, actor, item, hide_roofs=()):
    return dict(id=f'mobilier-r01-{ident}', name=name, rect=rect, actor=actor,
                hide_roofs=list(hide_roofs), item=item)


def main():
    path = OUT / 'manifest.json'
    if path.exists():
        assert not read(path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    OUT.mkdir(exist_ok=True)
    bed = read(ART / 'lit-r04/manifest.json')
    for file, digest in [(bed['candidate'], bed['candidate_sha256']),
                         (bed['parent_candidate'], bed['parent_candidate_sha256'])]:
        assert sha(ROOT / file) == digest, file
    approved = read(ROOT / bed['acceptance'])
    assert approved['candidate_sha256'] == bed['candidate_sha256']
    room = next(item for item in bed['items'] if item['id'] == 'V1-I01-logement')
    bed_part = next(part for part in room['parts'] if part['id'] == bed['edit']['part'])
    assert sha(ROOT / bed_part['file']) == approved['parts_sha256'][bed_part['file']]
    old = Image.open(ROOT / bed['parent_candidate']).convert('RGBA')
    candidate = Image.open(ROOT / bed['candidate']).convert('RGBA')
    allowed = Image.open(ROOT / bed['authorized_mask']).convert('L')

    placements_path = paving.PARTS / 'decor-placements.json'
    placements = [prop for prop in read(placements_path) if prop['asset'] == 'riot']
    assert len(placements) == 5 and all(not p['blocked_cells'] for p in placements)
    decor_code, = {p['code'] for p in placements}
    catalogue = read(paving.PARTS / 'decor-catalogue.json')
    sprite_path = ROOT / catalogue['riot']['file']
    sprite = Image.open(sprite_path).convert('RGBA')
    frozen = Image.open(FLOOR / 'street-0.png').convert('RGBA')
    clean = recompose_frozen_street(old.size, frozen, omit_decor=(decor_code,))
    removals = []
    for index, prop in enumerate(placements, 1):
        x, y = prop['at_px']
        box = (x, y, x + sprite.width, y + sprite.height)
        under = clean.crop(box)
        previous = frozen.crop(box)
        assert candidate.crop(box).tobytes() == previous.tobytes()
        assert under.tobytes() != previous.tobytes()
        mask = sprite.getchannel('A').point(lambda value: 255 if value else 0)
        outside = ImageChops.difference(previous, under)
        outside.paste((0, 0, 0, 0), mask=mask)
        assert not outside.getbbox(alpha_only=False)
        under.putalpha(mask)
        candidate.alpha_composite(under, (x, y))
        allowed.paste(255, (x, y), mask)
        ident = f'V1-RIOT-{index:02}'
        layer_path = OUT / f'{ident}-retrait-native.png'
        under.save(layer_path)
        removals.append(dict(id=ident, parts=[dict(id=ident + '-retrait', label='Retrait du décor antiémeute',
                             at_px=[x, y], file=relative(layer_path), sha256=sha(layer_path))]))

    diff = ImageChops.difference(old, candidate)
    diff.paste((0, 0, 0, 0), mask=allowed)
    assert not diff.getbbox(alpha_only=False)
    assert old.getchannel('A').tobytes() == candidate.getchannel('A').tobytes()
    candidate_path, mask_path = OUT / 'street-candidate-r01.png', OUT / 'authorized-mask-r01.png'
    candidate.save(candidate_path)
    allowed.save(mask_path)
    manifest = dict(review='CITY-CYBERPUNK12-MOBILIER-r01', status='artwork-awaiting-user-prevalidation',
                    base=bed['parent_candidate'], base_sha256=bed['parent_candidate_sha256'],
                    candidate=relative(candidate_path), candidate_sha256=sha(candidate_path),
                    authorized_mask=relative(mask_path), bed_acceptance=bed['acceptance'],
                    items=[{**room, 'parts': [bed_part]}, *removals], roofs=bed['roofs'],
                    removed_decor_source=relative(sprite_path), removed_decor_placements=relative(placements_path),
                    exact_floor_recomposition=True, campaign_integrated=False, presented_for_user_review=False)
    write(path, manifest)
    views = [
        view('logement', 'Logement · lit et trottoir', [384, 1088, 704, 512], [12, 21], room['id'], [2]),
        view('nord', 'Avenue nord', [800, 704, 512, 320], [16, 13], 'V1-RIOT-01'),
        view('cour', 'Cour centrale', [616, 1992, 512, 360], [13, 32], 'V1-RIOT-03'),
        view('est', 'Rue est', [832, 2720, 512, 384], [17, 45], 'V1-RIOT-04'),
        view('depart', 'Rue près du départ', [384, 3512, 576, 392], [9, 57], 'V1-RIOT-05'),
    ]
    overrides = {str(roof['index']): roof['candidate'] for roof in bed['roofs']}
    current = dict(review=manifest['review'], status=manifest['status'], manifest=relative(path),
                   title='Première ville · lit et accessoires au sol',
                   summary='Le comparatif reprend le logement qui était affiché dans l’onglet.',
                   intro='Les vues montrent le lit plus court et le retrait des cinq décors au sol.',
                   revision_note='Lit raccourci r04 et retrait du décor antiémeute.',
                   revision_note_detail='Le béton d’origine réapparaît sous les objets.',
                   review_doc='MOBILIER-r01.md', before=manifest['base'], after=manifest['candidate'],
                   before_label='vue r03', after_label='retouches r01', default_view=views[0]['id'],
                   before_roof_overrides=overrides, roof_overrides=overrides, views=views)
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
