"""Pack the replacement rear walk of MON-074; retain the rest of batch 14."""
import hashlib
import json
from shutil import copyfile

from PIL import Image
import prepare_monster_vertical01 as base


def main():
    folder = base.ROOT / 'assets/source-art/animation-polish-01/monster-vertical-14'
    old, new = folder / 'r01', folder / 'r02'
    manifest_path = new / 'manifest.json'

    def read(path):
        return json.loads(path.read_text(encoding='utf-8'))

    def write(path, value):
        path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')

    old_manifest = read(old / 'manifest.json')
    previous = next(p for p in old_manifest['profiles'] if p['number'] == 74)
    write(new / 'references.json', [r for r in read(old / 'references.json') if r['number'] == 74])
    copyfile(base.ROOT / previous['sources'][0], new / 'MON-074-down-source.png')
    base.OUT = new
    base.REVIEW = 'MONSTER-VERTICAL14-r02'
    base.ALIGN_SUPPORT_ROOT = True
    base.main()
    manifest = read(manifest_path)
    packed = manifest['profiles'][0]
    atlas_path = base.ROOT / packed['atlas']
    before = Image.open(base.ROOT / previous['atlas']).convert('RGBA')
    after = Image.open(atlas_path).convert('RGBA')
    for region in [(0, 0, 1440, 128), (1280, 128, 1440, 256)]:
        assert before.crop(region).tobytes() == after.crop(region).tobytes()
    assert all(
        before.crop((i*160, 128, (i+1)*160, 256)).tobytes()
        != after.crop((i*160, 128, (i+1)*160, 256)).tobytes()
        for i in range(8)
    )
    palette_path = new / 'MON-074-material-palette.json'
    palette = read(base.ROOT / previous['material_palette'])
    palette.update(
        review=base.REVIEW, source_atlas=packed['atlas'],
        source_sha256=hashlib.sha256(atlas_path.read_bytes()).hexdigest(),
        references=[ref for ref in palette['references'] if ref['direction'] == 'down'],
        method='Retain r01 front material curves; preserve the drawn rear colors in all eight poses.',
    )
    # Keep the prior front palette and preserve the newly drawn dorsal colors.
    palette['material_luts'][3:] = [[[i, i, i] for i in range(256)] for _ in range(3)]
    write(palette_path, palette)
    packed.update(
        name=previous['name'], review_number=42, locomotion='biped', locomotor_legs=2,
        sources=[previous['sources'][0], packed['sources'][1]],
        material_palette=palette_path.relative_to(base.ROOT).as_posix(),
    )
    manifest['profiles'] = [packed if p['number'] == 74 else p for p in old_manifest['profiles']]
    write(manifest_path, manifest)


if __name__ == '__main__':
    main()
