"""CIVILS-U01-08: isolated civilian aftermath props, with authored placements.

Only technical conditioning and whole-sprite transforms happen here. No map,
collision or approved production resource is modified.
"""
from pathlib import Path
import hashlib
import json
import shutil
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/first-sector-review-01/u01-civilians-r08'
DOC = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/street-civilians-review-08'
OUT = ROOT / 'artifacts/street-defects-review-01'
MAP = ROOT / 'assets/maps/prison-street-expanded-01'


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build():
    if (DOC / 'acceptance.json').exists():
        print('CIVILS-U01-08 est validé : sources et référence de proposition figées. Utiliser show_street_civilians_review.py pour actualiser la page.')
        return
    DOC.mkdir(parents=True, exist_ok=True)
    OUT.mkdir(parents=True, exist_ok=True)
    source = read(ART / 'prompts.json')
    native = {}
    assets = []
    for item in source['assets']:
        if item['status'] != 'generated_candidate':
            continue
        ident = item['id']
        image = Image.open(ART / f'{ident}-source.png').convert('RGBA')
        alpha = image.getchannel('A')
        box = alpha.point(lambda a: 255 if a > 128 else 0).getbbox()
        assert box and box[0] > 0 and box[1] > 0 and box[2] < image.width and box[3] < image.height, ident
        image = image.crop(box)
        image.putalpha(image.getchannel('A').point(lambda a: 0 if a < 40 else a))
        size = tuple(item['size'])
        limit = (size[0] - 8, size[1] - 6)
        if ident == 'SANG-U01-04':
            limit = (48, 32)
        if ident == 'SANG-U01-05':
            limit = (84, 22)
        image.thumbnail(limit, Image.Resampling.LANCZOS)
        sprite = Image.new('RGBA', size)
        sprite.alpha_composite(image, ((size[0] - image.width) // 2, (size[1] - image.height) // 2))
        sprite.save(ART / f'{ident}-native.png')
        native[ident] = sprite
        assets.append(dict(id=ident, name=item['name'], size=list(size), source_box=list(box),
                           source_sha256=sha(ART / f'{ident}-source.png'),
                           native_sha256=sha(ART / f'{ident}-native.png')))
    # Civilian IDs identify drawings, not new living actors or combat rules.
    bodies = [
        ('CIV-U01-04', (528, 3856), 0, 'fuite vers la prison'),
        ('CIV-U01-05', (414, 3458), 180, 'abords du premier logement'),
        ('CIV-U01-08', (1020, 3658), 0, 'véhicule incendié'),
        ('CIV-U01-06', (1244, 3710), 180, 'ruines près du portail'),
        ('CIV-U01-05', (286, 3290), 0, 'premier logement · intérieur'),
        ('CIV-U01-04', (580, 1296), 0, 'logement nord · intérieur'),
        ('CIV-U01-05', (756, 1340), 180, 'logement nord · intérieur'),
        ('CIV-U01-07', (602, 1858), 0, 'boutique · intérieur'),
        ('CIV-U01-04', (744, 1778), 180, 'boutique · intérieur'),
        ('CIV-U01-06', (842, 540), 0, 'façade détruite près de l’entrepôt'),
        ('CIV-U01-04', (1378, 644), 180, 'porte condamnée au nord'),
        ('CIV-U01-06', (378, 926), 90, 'carrefour et effondrement nord'),
        ('CIV-U01-07', (328, 2068), 180, 'sortie des ruines près de la boutique'),
        ('CIV-U01-08', (812, 2142), 180, 'épave du carrefour central'),
        ('CIV-U01-06', (736, 2456), 0, 'cour et bâtiment effondré'),
        ('CIV-U01-04', (534, 2638), 90, 'abords du local technique'),
        ('CIV-U01-05', (1372, 3128), 0, 'immeuble fermé au sud'),
        ('CIV-U01-07', (696, 2946), 0, 'carrefour sud et traces de violence'),
    ]
    props = []
    variants = {}

    def place(ident, at, angle, context, layer):
        key = (ident, angle)
        if key not in variants:
            turns = {0: None, 90: Image.Transpose.ROTATE_90, 180: Image.Transpose.ROTATE_180, 270: Image.Transpose.ROTATE_270}
            sprite = native[ident] if not angle else native[ident].transpose(turns[angle])
            file = ART / (f'{ident}-native.png' if not angle else f'{ident}-r{angle}-native.png')
            if angle:
                sprite.save(file)
            variants[key] = (sprite, file)
        sprite, file = variants[key]
        room = {'premier logement · intérieur':'ROOF-U01-01A',
                'logement nord · intérieur':'ROOF-U01-02A',
                'boutique · intérieur':'ROOF-U01-02B'}.get(context)
        props.append(dict(id=f'{ident}-{len(props)+1:02}', asset=ident,
                          file=file.relative_to(ROOT).as_posix(), at_px=list(at), size=list(sprite.size),
                          layer=layer, orientation_degrees=angle, context=context, interior_roof=room))

    # Blood belongs beneath the bodies and below all living actors.
    for i, (ident, at, angle, context) in enumerate(bodies):
        if ident in ['CIV-U01-04', 'CIV-U01-05', 'CIV-U01-07']:
            place('SANG-U01-04', (at[0] + 4, at[1] + 9), 180 if i % 2 else 0, context, 'blood')
    for at, angle, context in [
        ((470, 3872), 0, 'fuite vers la prison'),
        ((388, 3488), 180, 'abords du premier logement'),
        ((704, 1370), 180, 'logement nord · intérieur'),
        ((650, 1880), 0, 'boutique · intérieur'),
        ((400, 2082), 180, 'ruines près de la boutique'),
        ((764, 2960), 0, 'carrefour sud'),
        ((1378, 3162), 90, 'immeuble fermé au sud'),
    ]:
        place('SANG-U01-05', at, angle, context, 'blood')
    for ident, at, angle, context in bodies:
        place(ident, at, angle, context, 'body')
    assert all(0 <= p['at_px'][0] and 0 <= p['at_px'][1]
               and p['at_px'][0] + p['size'][0] <= 1664 and p['at_px'][1] + p['size'][1] <= 4064 for p in props)
    # Save the ordered list used directly by the preview renderer. Roof hiding
    # and lighting are the real engine passes, rather than browser simulations.
    write(ART / 'placements.json', props)
    plan = dict(review='CIVILS-U01-08', status='candidate_not_integrated', native_cell=64,
                body_drawings=5, new_body_placements=len(bodies), blood_drawings=2,
                blood_placements=sum(p['layer'] == 'blood' for p in props),
                no_new_collisions=True, new_live_actors=0, production_assets_unchanged=True,
                production_sha256={p.relative_to(MAP).as_posix(): sha(p) for p in MAP.rglob('*') if p.is_file()},
                assets=assets, placements=props,
                absent_asset=dict(id='CIV-U01-09', reason='generation service output rejected; no image produced, no retry'),
                source_directory=ART.relative_to(ROOT).as_posix())
    write(DOC / 'proposal.json', plan)
    shutil.copyfile(ART / 'prompts.json', DOC / 'prompts.json')
    shutil.copyfile(ART / 'generation-results.json', DOC / 'generation-results.json')
    # Native-scale board with all seven deliverables, plus the accepted corpse.
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    board = Image.new('RGBA', (960, 420), '#11191d')
    draw = ImageDraw.Draw(board)
    draw.text((16, 12), 'CIVILS-U01-08 · dessins candidats · taille native · cases de 64 px', font=font, fill='#d9c6a1')
    rows = assets + [dict(id='CORP-U01-01', name='Civil déjà présent · référence', size=[80,48])]
    ground = Image.open(MAP / 'street.png').convert('RGBA').crop((896, 2560, 1088, 2656))
    for i, item in enumerate(rows):
        x, y = 16 + (i % 4) * 236, 48 + (i // 4) * 176
        draw.text((x, y), item['id'], font=font, fill='#d9c6a1')
        draw.text((x, y + 22), item['name'], font=font, fill='#c9d4d2')
        board.alpha_composite(ground, (x, y + 48))
        sprite = native.get(item['id'])
        if sprite is None:
            sprite = Image.open(MAP / 'decor/CORP-U01-01.png').convert('RGBA')
        board.alpha_composite(sprite, (x + 48, y + 70))
        draw.text((x, y + 148), f"{item['size'][0]} × {item['size'][1]} px", font=font, fill='#9eb0ad')
    board.convert('RGB').save(OUT / 'civilians-native-board.png')
    print(json.dumps(dict(review=plan['review'], bodies=len(bodies), blood_decals=plan['blood_placements'],
                          native_assets=len(assets), normal_assets_unchanged=True)))


if __name__ == '__main__':
    build()
