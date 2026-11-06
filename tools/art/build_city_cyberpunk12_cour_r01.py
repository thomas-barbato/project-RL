"""Review the court maintenance station on the playable street."""
from PIL import Image, ImageChops

from build_city_cyberpunk12 import ART, DOC, ROOT, normalized, read, sha
from patch_city_cyberpunk12_lit_r05 import relative, write

OUT = ART / 'cour-r01'
ORIGIN = (656, 2464)
BASE = ROOT / 'assets/maps/prison-street-expanded-01/street-with-sewer-entry-lits-r01.png'
MODULE = ROOT / 'assets/source-art/first-sector-review-01/u01-expansion-r05/court-machines-native.png'
MODULE_AT = (688, 2500)
IDENT = 'V1-C06-station-cour'
PANELS = [
    ('flexible', 'Cassette et flexible de diagnostic', (73, 78, 134, 114)),
    ('ecran', 'Panneau de diagnostic fêlé', (76, 114, 132, 138)),
]


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    approval = read(DOC / 'ACCORD-LITS-r01.json')
    assert sha(BASE) == approval['active_files_sha256'][relative(BASE)]
    before = Image.open(BASE).convert('RGBA')
    candidate = before.copy()
    scope = Image.new('L', before.size)
    module = Image.open(MODULE).convert('RGBA').getchannel('A')
    source = OUT / 'station-source.png'
    drawing, scale = normalized(source, (208, 224))
    parts = []
    for key, label, box in PANELS:
        at = (ORIGIN[0] + box[0], ORIGIN[1] + box[1])
        x, y = at[0] - MODULE_AT[0], at[1] - MODULE_AT[1]
        layer = drawing.crop(box)
        w, h = layer.size
        assert module.crop((x, y, x + w, y + h)).getextrema()[0] >= 250
        assert layer.getchannel('A').getextrema() == (255, 255)
        file = OUT / f'{IDENT}-{key}-native.png'
        layer.save(file)
        candidate.alpha_composite(layer, at)
        scope.paste(255, (at[0], at[1], at[0] + w, at[1] + h))
        parts.append(dict(id=f'{IDENT}-{key}', label=label, file=relative(file),
                          sha256=sha(file), at_px=list(at), native_size=[w, h]))
    diff = ImageChops.difference(before, candidate)
    assert diff.getbbox(alpha_only=False)
    diff.paste((0, 0, 0, 0), mask=scope)
    assert not diff.getbbox(alpha_only=False)
    assert before.getchannel('A').tobytes() == candidate.getchannel('A').tobytes()
    picture = OUT / 'street-candidate-r01.png'
    mask = OUT / 'authorized-mask-r01.png'
    candidate.save(picture)
    scope.save(mask)
    manifest = dict(review='CITY-CYBERPUNK12-COUR-r01', status='artwork-awaiting-user-prevalidation',
                    base=relative(BASE), base_sha256=sha(BASE), candidate=relative(picture),
                    candidate_sha256=sha(picture), authorized_mask=relative(mask),
                    source=relative(source), source_sha256=sha(source), source_uniform_scale=scale,
                    prompt=relative(DOC / 'COUR-r01-PROMPT.txt'),
                    items=[dict(id=IDENT, original=relative(MODULE), original_sha256=sha(MODULE), parts=parts)],
                    roofs=[], presented_for_user_review=False, campaign_integrated=False)
    write(manifest_path, manifest)
    views = [
        dict(id='cour-r01-station', name='Station de maintenance', rect=[560, 2384, 512, 400],
             actor=[13, 41], hide_roofs=[], item=IDENT),
        dict(id='cour-r01-secteur', name='Cour centrale', rect=[448, 2176, 768, 688],
             actor=[13, 41], hide_roofs=[], item=IDENT),
    ]
    current = dict(review=manifest['review'], status=manifest['status'], manifest=relative(manifest_path),
                   title='Première ville · station de maintenance r01', summary='La machine de la cour centrale.',
                   intro='Deux cadrages à 64 px par case avec le personnage de référence.',
                   revision_note='Proposition pour la station de maintenance de la cour.',
                   revision_note_detail='Le flexible et le panneau fêlé occupent les deux logements du boîtier.',
                   review_doc='COUR-r01.md', before=manifest['base'], after=manifest['candidate'],
                   before_label='carte actuelle', after_label='proposition r01', default_view=views[0]['id'],
                   before_roof_overrides={}, roof_overrides={}, views=views)
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
