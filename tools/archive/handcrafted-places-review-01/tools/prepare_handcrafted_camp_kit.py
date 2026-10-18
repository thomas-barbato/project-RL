"""Condition the r04 illustrated modular kit, preserving all source paintings.

The generated pieces are sampled onto fixed engine footprints. End profiles
are registered to a common port, as in an ordinary autotile packing pipeline.
No character art or gameplay geometry is changed.
"""
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r04'
DEST = ROOT / 'assets/prototypes/handcrafted-camp-01/r04'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def opaque(image):
    image = image.convert('RGBA')
    image.putalpha(image.getchannel('A').point(lambda a: 255 if a >= 160 else 0))
    return image


def cell(image, col, row, cols, rows):
    return image.crop((col * image.width // cols, row * image.height // rows,
                       (col + 1) * image.width // cols, (row + 1) * image.height // rows))


def compact(image, colors=32):
    alpha = image.getchannel('A')
    result = image.convert('RGB').quantize(colors=colors, dither=Image.Dither.NONE).convert('RGBA')
    result.putalpha(alpha)
    return result


def largest_component(image):
    width,height=image.size
    alpha=image.getchannel('A').tobytes()
    seen=bytearray(width*height)
    largest=[]
    for start,a in enumerate(alpha):
        if not a or seen[start]:continue
        pending=[start];component=[];seen[start]=1
        while pending:
            i=pending.pop();component.append(i);x=i%width;y=i//width
            for n in ([i-1] if x else [])+([i+1] if x+1<width else [])+([i-width] if y else [])+([i+width] if y+1<height else []):
                if alpha[n] and not seen[n]:seen[n]=1;pending.append(n)
        if len(component)>len(largest):largest=component
    mask=bytearray(width*height)
    for i in largest:mask[i]=255
    image.putalpha(Image.frombytes('L',(width,height),bytes(mask)))
    return image


def fit(image, bounds, single_piece=False):
    image = opaque(image)
    if single_piece:image=largest_component(image)
    art_bounds = image.getbbox()
    assert art_bounds
    x, y, right, bottom = bounds
    art = opaque(image.crop(art_bounds).resize((right-x, bottom-y), Image.Resampling.LANCZOS))
    result = Image.new('RGBA', (64, 64))
    result.alpha_composite(art, (x, y))
    return result


def profile(piece, vertical):
    # Use an interior section of the connecting arm, excluding the painted cap.
    samples = [piece.getpixel((x, 8)) for x in range(64)] if vertical else [piece.getpixel((8, y)) for y in range(64)]
    return samples


def register_ports(piece, mask, north_south, east_west):
    for t in range(4):
        for i in range(64):
            if mask & 1: piece.putpixel((i, t), north_south[i])
            if mask & 4: piece.putpixel((i, 63-t), north_south[i])
            if mask & 8: piece.putpixel((t, i), east_west[i])
            if mask & 2: piece.putpixel((63-t, i), east_west[i])


def main():
    DEST.mkdir(parents=True, exist_ok=True)
    source = Image.open(SOURCE / 'walls-source-r04.png').convert('RGBA')
    pieces = []
    for m in range(16):
        target = (0 if m & 8 else 16, 0 if m & 1 else 16,
                  64 if m & 2 else 48, 64 if m & 4 else 48)
        pieces.append(compact(fit(cell(source, m % 4, m // 4, 4, 4), target, single_piece=True)))
    ns = profile(pieces[5], True)
    ew = profile(pieces[10], False)
    assert ns[32][3] == 255 and ew[32][3] == 255
    atlas = Image.new('RGBA', (256, 256))
    for m, piece in enumerate(pieces):
        register_ports(piece, m, ns, ew)
        atlas.alpha_composite(piece, ((m % 4)*64, (m // 4)*64))
    atlas.save(DEST / 'walls-native64.png', optimize=True)
    # The source floor contains 6x6 slabs. Select four individual slabs, not
    # multi-slab quadrants, to keep one slab about the size of a logical cell.
    floor_source = Image.open(SOURCE / 'floor-source-r04.png').convert('RGBA')
    floors = Image.new('RGBA', (128, 128))
    for i, (x, y) in enumerate([(1, 1), (2, 1), (3, 3), (4, 4)]):
        tile = cell(floor_source, x, y, 6, 6).resize((64, 64), Image.Resampling.LANCZOS)
        tile.putalpha(255)
        floors.alpha_composite(tile, ((i % 2)*64, (i // 2)*64))
    compact(floors, 32).save(DEST / 'floors-native64.png', optimize=True)
    furniture_source = Image.open(SOURCE / 'furniture-source-r04.png').convert('RGBA')
    furniture = []
    for i, (label, size) in enumerate(zip(['Établi', 'Siège', 'Armoire', 'Étagère', 'Lit', 'Banc'], [(54,38),(20,28),(32,56),(40,52),(34,68),(46,24)])):
        item = opaque(cell(furniture_source, i % 3, i // 3, 3, 2))
        crop = item.getbbox()
        art = opaque(item.crop(crop).resize(size, Image.Resampling.LANCZOS))
        canvas = Image.new('RGBA', (128, 128))
        canvas.alpha_composite(compact(art), (64-size[0]//2, 96-size[1]))
        path = DEST / f'DEC-CAMP-{i+1:02}-r04.png'
        canvas.save(path, optimize=True)
        furniture.append(dict(id=f'DEC-CAMP-{i+1:02}', label=label, file=path.relative_to(ROOT).as_posix(),
                              native_body_size=list(size), canvas=[128,128], foot_anchor=[64,96], sha256=sha(path)))
    doors_source = Image.open(SOURCE / 'doors-source-r04.png').convert('RGBA')
    doors = []
    for i in range(4):
        piece = compact(fit(cell(doors_source, i % 2, i // 2, 2, 2), (0,0,64,64)))
        # Register artwork to the actual door geometry. An open door consists
        # only of its two jambs, with no crossbeam through the walkable center.
        for y in range(64):
            for x in range(64):
                allowed = (16 <= x < 48 and (i == 1 or y < 16 or y >= 48)) if i < 2 else (16 <= y < 48 and (i == 3 or x < 16 or x >= 48))
                if not allowed: piece.putpixel((x,y),(0,0,0,0))
        register_ports(piece, 5 if i < 2 else 10, ns, ew)
        doors.append(piece)
    door_atlas = Image.new('RGBA',(128,128))
    for i,piece in enumerate(doors):door_atlas.alpha_composite(piece,((i%2)*64,(i//2)*64))
    door_atlas.save(DEST / 'doors-native64.png', optimize=True)
    lamp=opaque(Image.open(SOURCE/'lamp-source-r04.png').convert('RGBA'))
    lamp=opaque(lamp.crop(lamp.getbbox()).resize((36,12),Image.Resampling.LANCZOS))
    compact(lamp).save(DEST/'lamp-native.png',optimize=True)
    # Validate actual packed pixels, including every T's three physical ports.
    for m,piece in enumerate(pieces):
        for bit,edge in [(1,[piece.getpixel((i,0)) for i in range(64)]),(4,[piece.getpixel((i,63)) for i in range(64)]),(2,[piece.getpixel((63,i)) for i in range(64)]),(8,[piece.getpixel((0,i)) for i in range(64)])]:
            if m & bit: assert edge == (ns if bit in (1,4) else ew)
        assert piece.getpixel((32,32))[3] == 255
    for i in [0,2]:
        assert all(doors[i].getpixel((x,y))[3] == 0 for y in range(16,48) for x in range(16,48))
    manifest = dict(place='PLACE-CAMP-01', revision='r04', art_approved=False, runtime_campaign_connected=False,
        source_images={p.name:sha(p) for p in SOURCE.glob('*source-r04.png')},
        method='Built-in imagegen paintings; fixed slot sampling, native conditioning, footprint registration and common edge ports. Sources preserved.',
        cell=64, wall_masks=list(range(16)), painted_T_masks=[7,11,13,14], wall_rotation_operations=0,
        all_wall_ports_registered=True, open_door_center_transparent=True,
        furniture=furniture, outputs={p.name:sha(p) for p in DEST.glob('*.png')})
    (REVIEW / 'kit-r04.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(wall_shapes=16,painted_Ts=4,native_furniture=6,wall_rotations=0,ports_verified=True)))


if __name__ == '__main__':
    main()
