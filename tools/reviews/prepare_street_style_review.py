"""Condition and assemble complete review modules; never paint or replace game art."""
from pathlib import Path
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r03'
OLD = ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r06'


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')


def build():
    origin = (128, 192)
    size = (704, 512)
    floor = Image.open(ART/'STYLE-U01-02-native.png').convert('RGBA')
    assert floor.size == (128, 128)
    tiles = [floor.crop((x, y, x+64, y+64)) for y in (0, 64) for x in (0, 64)]
    canvas = Image.new('RGBA', size)
    for y in range(size[1]//64):
        for x in range(size[0]//64):
            # The separate ruined house starts at world [448,576]. Its entire
            # rectangle stays untouched, including the part beyond this patch.
            if x >= 5 and y >= 6:
                continue
            canvas.alpha_composite(tiles[(x*5+y*3+(x*y)%3) % 4], (x*64, y*64))
    modules = json.loads((OLD/'module-placements.json').read_text(encoding='utf-8'))
    selected = {'warehouse', 'warehouse-crate', 'warehouse-cabinet', 'warehouse-barrel'}
    composition = []
    for module in modules:
        if module['id'] not in selected:
            continue
        file = ART/'STYLE-U01-01-native.png' if module['id'] == 'warehouse' else ROOT/module['file']
        image = Image.open(file).convert('RGBA')
        at = [module['at'][i]-origin[i] for i in range(2)]
        assert all(at[i] >= 0 and at[i]+image.size[i] <= size[i] for i in range(2)), module['id']
        canvas.alpha_composite(image, tuple(at))
        composition.append(dict(id=module['id'], file=file.relative_to(ROOT).as_posix(), at_px=at, size=list(image.size)))
    assert {m['id'] for m in composition} == selected
    # The generated source may contain alpha=1 specks outside its silhouette.
    assert canvas.getchannel('A').crop((320, 384, 704, 512)).point(lambda a: 255 if a > 10 else 0).getbbox() is None
    canvas.save(ART/'STYLE-U01-03-composite.png')
    placements = [
        ('STYLE-U01-01/02', ART/'STYLE-U01-03-composite.png', list(origin), 'Essai de matériaux, emprise technique de revue seulement.'),
        ('GRIM-U01-01', ROOT/'assets/source-art/first-sector-review-01/u01-corrections-r01/GRIM-U01-01-native.png', [4*64, 56*64], 'Défense brisée près des impacts à l’approche de la prison.'),
        ('CORP-U01-01', ART/'CORP-U01-01-native.png', [3*64, 59*64], 'Civil à côté de la clôture, hors du couloir de sortie.'),
        ('CORP-U01-02', ART/'CORP-U01-02-native.png', [9*64, 57*64], 'Cultiste sur l’approche de la défense.'),
        ('CORP-U01-03', ART/'CORP-U01-03-native.png', [6*64, 55*64], 'Militaire près du point de résistance et des impacts.'),
    ]
    items = []
    for identity, file, at, reason in placements:
        im = Image.open(file).convert('RGBA')
        bbox = im.getchannel('A').getbbox()
        assert bbox and bbox[0] >= 0 and bbox[1] >= 0
        items.append(dict(id=identity, file=file.relative_to(ROOT).as_posix(), at_px=at,
                          size=list(im.size), alpha_bbox=list(bbox), reason=reason,
                          sha256=hashlib.sha256(file.read_bytes()).hexdigest()))
    layout = dict(review='FIXES-U01-03', native_cell=64, normal_integration=False,
                  same_lighting=True, living_actors_unchanged=True, ground_layer=True,
                  rejected_for_street=['GRIM-U01-02'], items=items)
    write(ART/'review-layout.json', layout)
    write(ART/'composition.json', dict(origin_px=list(origin), size=list(size), modules=composition,
                                     untouched_world_rect=[448,576,832,704],
                                     complete_modules=True, collision_approval=False,
                                     note='Assemblage technique de tuiles de sol et de modules entiers. Aucun dessin découpé dans une illustration.'))
    print(json.dumps(dict(review=layout['review'], items=len(items), patch=list(size), integrated=False)))


if __name__ == '__main__':
    build()
