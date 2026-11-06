"""Condition imagegen technology into independent layers over the intact city.

No architecture, ground, doors, collision or active asset is repainted here.
The generated whole-module studies supply only the selected device pixels.
"""
from pathlib import Path
import argparse
import hashlib
import json
from PIL import Image, ImageChops, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / 'assets/source-art/city-cyberpunk12'
DOC = ROOT / 'docs/reviews/city-cyberpunk12'
OUT = ROOT / 'artifacts/city-cyberpunk12'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def polygon_mask(size, polygons):
    mask = Image.new('L', (size[0] * 4, size[1] * 4))
    draw = ImageDraw.Draw(mask)
    for polygon in polygons:
        draw.polygon([(x * 4, y * 4) for x, y in polygon], fill=255)
    return mask.resize(size, Image.Resampling.LANCZOS)


def normalized(source, size):
    """Uniform scale, with at most one rounded raster pixel of padding."""
    image = Image.open(source).convert('RGBA')
    scale = min(size[0] / image.width, size[1] / image.height)
    wh = tuple(round(v * scale) for v in image.size)
    result = Image.new('RGBA', size)
    result.alpha_composite(image.resize(wh, Image.Resampling.LANCZOS),
                           ((size[0] - wh[0]) // 2, (size[1] - wh[1]) // 2))
    return result, scale


def device(study, code, polygons, *, shift=(0, 0), box=None, at=None, width=None):
    layer = study.copy()
    mask = polygon_mask(study.size, polygons)
    layer.putalpha(ImageChops.multiply(layer.getchannel('A'), mask))
    if box is not None:
        layer = layer.crop(box)
        factor = width / layer.width
        layer = layer.resize((width, round(layer.height * factor)), Image.Resampling.LANCZOS)
        target = Image.new('RGBA', study.size)
        target.alpha_composite(layer, at)
        layer = target
    elif shift != (0, 0):
        target = Image.new('RGBA', study.size)
        target.alpha_composite(layer, shift)
        layer = target
    return code, layer


def additions(study, number):
    if number == 1:
        return [
            device(study, 'D01-relais', [[(135,102),(203,102),(213,110),(213,143),(201,149),(155,150),(153,145),(135,145)]]),
            device(study, 'D02-reseau', [[(156,139),(171,139),(172,150),(180,153),(242,153),(253,158),(256,166),(256,207),(250,217),(242,217),(242,207),(245,200),(245,166),(239,163),(177,163),(164,158)]]),
            device(study, 'D03-camera', [[(184,205),(205,209),(207,217),(198,225),(184,221)]]),
            device(study, 'D04-acces', [[(228,225),(247,225),(247,260),(235,265),(228,261)]], box=(228,225,248,266), at=(231,219), width=12),
        ]
    if number == 2:
        return [
            device(study, 'D01-relais', [[(123,176),(144,174),(146,190),(150,192),(163,192),(164,176),(183,186),(183,206),(175,207),(175,248),(168,253),(143,253),(138,246),(138,218),(130,212),(130,205),(123,203)]], shift=(6,20)),
            device(study, 'D02-reseau', [[(151,244),(163,244),(165,255),(174,266),(178,274),(178,310),(170,326),(160,333),(160,347),(151,352),(128,352),(124,346),(67,347),(60,351),(55,346),(55,337),(62,328),(78,326),(126,326),(128,316),(154,314),(158,317),(165,307),(165,276),(161,267),(153,261)]], shift=(6,20)),
            device(study, 'D03-acces', [[(140,418),(158,418),(158,448),(140,448)]], box=(140,418,159,449), at=(146,419), width=13),
        ]
    return [
        device(study, 'D01-relais', [[(69,32),(139,32),(139,82),(69,82)]]),
        device(study, 'D02-reseau', [[(69,49),(77,49),(77,205),(71,213),(64,217),(70,226),(65,231),(60,220),(55,229),(51,227),(53,216),(48,212),(38,213),(38,243),(28,243),(28,203),(34,197),(57,197),(57,54)]]),
        device(study, 'D03-plaque', [[(145,207),(182,207),(182,237),(145,237)]]),
        device(study, 'D04-acces', [[(149,306),(175,306),(175,345),(149,345)]], box=(149,306,176,346), at=(160,310), width=18),
    ]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--compose-only', action='store_true', help='Recompose existing independent layers without generating or changing them.')
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    manifest = read(ART / 'manifest.json')
    assert manifest['status'] in ('artwork-awaiting-user-prevalidation', 'artwork-prevalidated')
    if not args.compose_only:
        assert not manifest.get('presented_for_user_review'), 'Preserve presented artwork; edit one numbered layer in a sibling revision, then use --compose-only.'
    scope = read(DOC / 'SCOPE.json')
    for path, digest in scope['baseline_sha256'].items():
        assert sha(ROOT / path) == digest, ('Baseline changed', path)
    baseline_path = ROOT / 'assets/maps/prison-street-expanded-01/street.png'
    baseline = Image.open(baseline_path).convert('RGBA')
    candidate = baseline.copy()
    authorized = Image.new('L', baseline.size)
    for number, item in enumerate(manifest['items'], 1):
        assert sha(ROOT / item['original']) == item['original_sha256']
        if args.compose_only:
            layers = [(part['id'], Image.open(ROOT / part['file']).convert('RGBA')) for part in item['parts']]
            study = Image.new('RGBA', tuple(item['native_size']))
            scale = item['source_uniform_scale']
        else:
            study, scale = normalized(ROOT / item['source'], tuple(item['native_size']))
            layers = additions(study, number)
        combined = Image.new('RGBA', study.size)
        records = []
        original = Image.open(ROOT / item['original']).convert('RGBA')
        protected = {1: [(44,222,171,295),(178,225,230,304)], 2: [(76,416,143,477)], 3: [(85,289,158,357)]}[number]
        for index, (code, layer) in enumerate(layers):
            if not args.compose_only:
                # Condition once; multiplying alpha again would alter reviewed pixels.
                layer.putalpha(ImageChops.multiply(layer.getchannel('A'), original.getchannel('A')))
                alpha = layer.getchannel('A')
                draw = ImageDraw.Draw(alpha)
                for rect in protected:
                    draw.rectangle(rect, fill=0)
                layer.putalpha(alpha)
            if args.compose_only:
                part = item['parts'][index]
                path = ROOT / part['file']
                assert sha(path) == part['sha256'], ('Update the edited layer record explicitly', path)
                # A composed layer is immutable: conditioning belongs to its own revision.
                assert Image.open(path).convert('RGBA').tobytes() == layer.tobytes()
                ident = code
            else:
                path = ART / (item['id'] + '-' + code + '-native-r01.png')
                if path.exists() and item.get('parts'):
                    previous = next(p for p in item['parts'] if p['id'] == item['id'] + '-' + code)
                    assert sha(path) == previous['sha256'], ('Preserve manual edit', path)
                layer.save(path)
                ident = item['id'] + '-' + code
            combined.alpha_composite(layer)
            records.append({'id': ident, 'file': path.relative_to(ROOT).as_posix(), 'sha256': sha(path), 'native_bbox': layer.getchannel('A').getbbox()})
        combined.save(ROOT / item['native'])
        item.update(status=manifest['status'],source_sha256=sha(ROOT / item['source']),native_sha256=sha(ROOT / item['native']),source_uniform_scale=scale,parts=records,protected_doors=protected,composition='Technology layers only. Existing architecture and openings retained.')
        at = tuple(item['at_px'])
        candidate.alpha_composite(combined, at)
        authorized.paste(ImageChops.lighter(authorized.crop((at[0], at[1], at[0]+combined.width, at[1]+combined.height)), combined.getchannel('A')), at)
    assert candidate.getchannel('A').tobytes() == baseline.getchannel('A').tobytes()
    changed = ImageChops.difference(candidate, baseline).convert('RGB')
    outside = changed.copy()
    outside.paste((0,0,0), mask=authorized.point(lambda value: 255 if value else 0))
    assert outside.getbbox() is None
    candidate.save(ART / 'street-candidate-r01.png')
    authorized.save(ART / 'authorized-mask-r01.png')
    manifest.update(candidate=(ART/'street-candidate-r01.png').relative_to(ROOT).as_posix(),candidate_sha256=sha(ART/'street-candidate-r01.png'),viewpoint=[0,2112],view_size=[1664,1024],actor_cell=[7,43],active_assets_unchanged=True,outside_device_pixels_unchanged=True,architecture_ground_and_doors_preserved=True,campaign_integrated=False)
    (ART/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'modules':len(manifest['items']),'independent_devices':sum(len(i['parts']) for i in manifest['items']),'active_assets_unchanged':True,'outside_device_pixels_unchanged':True}))


if __name__ == '__main__':
    main()
