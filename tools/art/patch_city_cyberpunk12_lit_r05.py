"""Replace the r04 bed within its old footprint in the current review."""
import json

from PIL import Image, ImageChops, ImageDraw

from build_city_cyberpunk12 import ART, DOC, ROOT, normalized, read, sha

OUT = ART / 'lit-r05'
ORIGIN = (736, 1200)
SIZE = (272, 240)
PART = 'V1-I01-logement-D05-lit'


def relative(path):
    return path.relative_to(ROOT).as_posix()


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def main():
    manifest_path = OUT / 'manifest.json'
    if manifest_path.exists():
        assert not read(manifest_path).get('presented_for_user_review'), 'Keep the reviewed revision.'
    parent_path = ART / 'mobilier-r01/manifest.json'
    parent = read(parent_path)
    assert sha(ROOT / parent['candidate']) == parent['candidate_sha256']
    before = Image.open(ROOT / parent['candidate']).convert('RGBA')
    source = OUT / 'lit-source-r05.png'
    artwork, scale = normalized(source, SIZE)

    # Keep the upper wall fixture and the conduit at the east edge. The
    # smaller cot and exposed resin fit inside the old bed and contact shadow.
    footprint = Image.new('L', SIZE)
    ImageDraw.Draw(footprint).polygon([
        (129, 52), (205, 52), (205, 190), (200, 193), (138, 193),
        (129, 190), (128, 181), (128, 77), (129, 69),
    ], fill=255)
    bounds = footprint.getbbox()
    layer = artwork.crop(bounds)
    alpha = footprint.crop(bounds)
    layer.putalpha(alpha)
    at = (ORIGIN[0] + bounds[0], ORIGIN[1] + bounds[1])
    layer_path = OUT / f'{PART}-native-r05.png'
    layer.save(layer_path)
    candidate = before.copy()
    candidate.alpha_composite(layer, at)
    mask = Image.new('L', before.size)
    mask.paste(255, at, alpha)
    diff = ImageChops.difference(before, candidate)
    assert diff.getbbox(alpha_only=False)
    diff.paste((0, 0, 0, 0), mask=mask)
    assert not diff.getbbox(alpha_only=False)
    assert before.getchannel('A').tobytes() == candidate.getchannel('A').tobytes()

    candidate_path = OUT / 'street-candidate-r05.png'
    mask_path = OUT / 'authorized-mask-r05.png'
    candidate.save(candidate_path)
    mask.save(mask_path)
    item, *removals = parent['items']
    assert item['parts'][0]['id'] == PART
    part = dict(id=PART, label='Lit simple compact r05', file=relative(layer_path),
                sha256=sha(layer_path), at_px=list(at), native_size=list(layer.size))
    manifest = dict(
        review='CITY-CYBERPUNK12-LIT-r05', status='artwork-awaiting-user-prevalidation',
        parent_review=relative(parent_path), base=parent['candidate'], base_sha256=parent['candidate_sha256'],
        candidate=relative(candidate_path), candidate_sha256=sha(candidate_path),
        authorized_mask=relative(mask_path), source=relative(source), source_sha256=sha(source),
        source_uniform_scale=scale, prompt=relative(DOC / 'LIT-r05-PROMPT.txt'),
        items=[{'id': item['id'], 'parts': [part]}, *removals], roofs=parent['roofs'],
        presented_for_user_review=False, campaign_integrated=False,
    )
    write(manifest_path, manifest)
    view = dict(id='lit-r05-logement', name='Logement', rect=[384, 1088, 704, 512],
                actor=[12, 21], hide_roofs=[2], item=item['id'])
    roofs = {str(roof['index']): roof['candidate'] for roof in parent['roofs']}
    current = dict(
        review=manifest['review'], status=manifest['status'], manifest=relative(manifest_path),
        title='Première ville · lit simple r05', summary='Un lit simple plus étroit et plus court dans le logement.',
        intro='Le comparatif à taille de jeu permet de juger ses proportions avec le personnage.',
        revision_note='Reprise des proportions du lit.', revision_note_detail='Comparatif r04 et r05.',
        review_doc='LIT-r05.md', before=manifest['base'], after=manifest['candidate'],
        before_label='lit r04', after_label='lit simple r05', default_view=view['id'],
        before_roof_overrides=roofs, roof_overrides=roofs, views=[view],
    )
    write(ART / 'review-current.json', current)


if __name__ == '__main__':
    main()
