"""Index existing isolated scenery, without changing its art or logical visibility.

An observed static sprite can be displayed whole. This index never authorizes
ground, actors or interactive contents under that sprite.
"""
import hashlib
import json
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r06'
WAR = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r04'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def build_scene(name, size, placements):
    atlas = np.zeros((size[1], size[0], 4), dtype=np.uint8)
    objects = []
    for ident, item in enumerate(placements, 1):
        assert ident < 256
        path = ROOT / item['file']
        source = Image.open(path).convert('RGBA')
        x, y = item['at']
        alpha = np.array(source)[:, :, 3]
        h, w = alpha.shape
        assert x >= 0 and y >= 0 and x + w <= size[0] and y + h <= size[1], item
        target = atlas[y:y+h, x:x+w]
        present = alpha > 0
        target[present] = [ident, 0, 0, 255]
        # The alpha silhouette, not a rectangle of neighboring floor.
        target[:, :, 3][present] = alpha[present]
        witnesses = []
        for gy in range(y // 64, (y+h-1) // 64 + 1):
            for gx in range(x // 64, (x+w-1) // 64 + 1):
                part = alpha[max(0,gy*64-y):min(h,(gy+1)*64-y),
                             max(0,gx*64-x):min(w,(gx+1)*64-x)]
                if int((part >= 200).sum()) >= 16:
                    witnesses.append([gx, gy])
        assert witnesses, item
        objects.append(dict(id=ident, source=item['file'], at=item['at'],
                            source_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),
                            witnesses=witnesses, covered_by=item.get('covered_by')))
    texture = ART / f'visibility-surfaces-{name}-03.png'
    Image.fromarray(atlas).save(texture)
    manifest = dict(review='VISION-U01-03', status='candidate_not_integrated',
                    texture=texture.relative_to(ROOT).as_posix(), size=list(size), objects=objects)
    (ART / f'visibility-surfaces-{name}-03.json').write_text(
        json.dumps(manifest, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    return len(objects)


def build():
    # A whole open room is an assembly, not one prop. A covered assembly may
    # retain its exterior silhouette only while its accepted roof is visible.
    street = read(ART/'module-placements.json')
    # A closed roof conceals the assembled room. Only that existing visual
    # state authorizes the complete body silhouette; opening it disables this
    # entry and returns every room cell to the ordinary perception field.
    for m in street:
        if m['id'].endswith('-body'):
            m['covered_by'] = 2 if '02A' in m['id'] else 3
    street += [dict(file=m['file'], at=m['at_px']) for m in read(WAR/'new-buildings.json')]
    catalogue = read(ART/'decor-catalogue.json')
    street += [dict(file=catalogue[m['asset']]['file'], at=m['at_px'])
               for m in read(ART/'decor-placements.json')]
    camp = read(ROOT/'assets/maps/cryogenic-camp-01/camp.json')
    props = [dict(file=p['file'], at=[p['at'][0]*64+p.get('width',1)*32-64,
                                    p['at'][1]*64+p.get('height',1)*64-96])
             for p in camp['props']]
    print(json.dumps(dict(street=build_scene('street',(26*64,66*64),street),
                          camp=build_scene('camp',(camp['width']*64,camp['height']*64),props))))


if __name__ == '__main__':
    build()
