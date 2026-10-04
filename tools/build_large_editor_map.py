"""Build the fixed test landscape in the standalone editor's Document format.

No campaign generation or user map is modified. The seed only makes the saved
scenery reproducible; the resulting JSON is an ordinary, fully editable map.
Run native validation/captures with --map PATH --capture-landscape NEW_DIRECTORY.
"""
from pathlib import Path
import json
import math
import random

ROOT = Path(__file__).resolve().parents[1]
DESTINATION = ROOT / "artifacts/map-editor/maps/grande-ville-et-nature-2026-10-02.json"
W, H = 160, 112
rng = random.Random(20261002)
doc = dict(version=1, width=W, height=H, spawn=dict(x=41, y=32),
           floors=[7] * (W * H), structures=[], props=[], paint=[], markers=[])
occupied = set()
buildings = []
paths = []


def ground(rect, material):
    l, t, r, b = rect
    for y in range(t, b + 1):
        for x in range(l, r + 1):
            doc["floors"][y * W + x] = material


def stroke(points, material, diameter, clip=None, reserve=False):
    doc["paint"].append(dict(material=material, diameter=diameter,
                             points=[dict(x=max(0, min(W * 64 - 1, round(x * 64))),
                                          y=max(0, min(H * 64 - 1, round(y * 64)))) for x, y in points],
                             **({"clip": clip} if clip else {})))
    if reserve:
        paths.append((points, diameter / 128 + 1.1))


def clear_paint(rect):
    l, t, r, b = rect
    points = []
    for i, x in enumerate(range(l, r + 2, 4)):
        x = min(x + 0.5, r + 0.5)
        points.extend([(x, t + 0.5), (x, b + 0.5)] if i % 2 == 0
                      else [(x, b + 0.5), (x, t + 0.5)])
    stroke(points, None, 384, dict(left=l * 64, top=t * 64, right=(r + 1) * 64,
                                  bottom=(b + 1) * 64, offset_x=0, offset_y=0, turns=0))


def object_at(x, y, sprite, rotation=0, *, size=None, blocking=None,
              decoration=False, scale=100, jitter=False, optional=False):
    w, h = size or ((2, 1) if sprite in (4, 65) else (1, 2)
                   if 72 <= sprite <= 75 else (1, 1))
    rw, rh = (h, w) if rotation % 2 else (w, h)
    if blocking is None:
        blocking = sprite not in (67, 80, 81, 82, 111, 112, 114, 116, 117)
    claims = [(xx, yy) for yy in range(y, y + rh) for xx in range(x, x + rw)]
    if any(not (0 <= xx < W and 0 <= yy < H) for xx, yy in claims):
        raise ValueError((x, y, sprite, "outside"))
    if (blocking or not decoration) and any(p in occupied for p in claims):
        if optional:
            return False
        raise ValueError((x, y, sprite, "occupied"))
    if blocking or not decoration:
        occupied.update(claims)
    doc["props"].append(dict(pos=dict(x=x, y=y), sprite=sprite, furniture=True,
                              rotation=rotation, blocking=blocking,
                              details=dict(width=w, height=h, offset_x=rng.randint(-8, 8) if jitter else 0,
                                           offset_y=rng.randint(-8, 8) if jitter else 0,
                                           decoration=decoration, scale=scale,
                                           opaque=blocking and sprite in (2, 3, 15, 18, 21, 56, 57,
                                                                          58, 59, 104, 105, 106, 110, 118))))
    return True


def building(name, rect, floor, style, door_side="south", offset=None):
    l, t, r, b = rect
    door = dict(south=((l + r) // 2, b), north=((l + r) // 2, t),
                east=(r, (t + b) // 2), west=(l, (t + b) // 2))[door_side]
    if offset is not None:
        door = (offset, door[1]) if door_side in ("south", "north") else (door[0], offset)
    ground(rect, floor)
    for y in range(t, b + 1):
        for x in range(l, r + 1):
            if x not in (l, r) and y not in (t, b):
                continue
            corner = {(l, t): 6, (r, t): 12, (l, b): 3, (r, b): 9}.get((x, y))
            is_door = (x, y) == door
            part = dict(pos=dict(x=x, y=y), door="Open" if is_door else None,
                        rotation=(int(door_side in ("east", "west")) if is_door else
                                  0 if corner else 3 if x == l else 1 if x == r else 2 if y == b else 0),
                        style=style)
            if not is_door:
                part["fixed_connections"] = corner or 10
            doc["structures"].append(part)
            occupied.add((x, y))
    # Interior flooring stays clean even where a soft path approaches the door.
    clear_paint(rect)
    buildings.append(dict(name=name, rect=list(rect), door=list(door)))
    return l, t, r, b


def furnish(rect, kind, variant=0):
    l, t, r, b = rect
    x, y = l + 1, t + 1
    width, height = r - l - 1, b - t - 1
    # Keep a clear central aisle to every door, and room around large footprints.
    if kind == "home":
        for dx, dy, s in [(0, 0, 72 + variant % 4), (2, 0, 2),
                           (width - 1, 0, 18), (width - 2, 0, 16),
                           (width - 1, 1, 17), (0, height - 2, 14)]:
            object_at(x + dx, y + dy, s)
        object_at(x + 3, y + height - 3, 4)
        object_at(x + 3, y + height - 2, 1, 2)
        object_at(x + width - 1, y + height - 1, 19)
    elif kind == "shop":
        for dx in range(0, width, 2):
            object_at(x + dx, y, 11)
        for dy in range(2, height - 2, 3):
            object_at(x, y + dy, 9)
            object_at(r - 1, y + dy, 10)
        for dx in (1, 2, 3):
            object_at(x + dx, b - 2, 8)
        object_at(r - 1, b - 2, 34)
    elif kind == "clinic":
        for dx, dy, s in [(0, 0, 12), (2, 0, 21), (width - 1, 0, 69),
                           (width - 2, 0, 68), (0, 3, 20), (width - 1, 3, 20),
                           (0, 6, 20), (width - 1, 6, 20), (2, height - 2, 13),
                           (width - 2, height - 2, 13)]:
            object_at(x + dx, y + dy, s)
    elif kind == "servers":
        for dx in (0, 2, 5, 7, 10):
            for dy in (0, 3, 6):
                object_at(x + dx, y + dy, 56 + (dx // 2 + dy // 3) % 4)
        object_at(x, b - 2, 65)
        object_at(x + 3, b - 2, 60)
        object_at(r - 2, b - 2, 61)
    elif kind == "armory":
        for dx, s in enumerate((88, 89, 90, 91, 97, 98)):
            object_at(x + dx * 2, y, s)
        for dx, dy, s in [(0, 3, 94), (0, 5, 95), (width - 1, 3, 93),
                           (width - 1, 5, 100), (2, height - 2, 101),
                           (width - 2, height - 2, 96)]:
            object_at(x + dx, y + dy, s)
    elif kind == "workshop":
        for dx, dy, s in [(0, 0, 0), (2, 0, 23), (width - 1, 0, 53),
                           (1, 3, 36), (width - 1, 3, 39),
                           (0, height - 2, 37), (width - 1, height - 2, 55)]:
            object_at(x + dx, y + dy, s)
    elif kind == "restaurant":
        for dx, s in enumerate((18, 16, 17, 19, 8, 8)):
            object_at(x + dx, y, s)
        for dx in (0, width - 3):
            for dy in (3, 7):
                object_at(x + dx, y + dy, 4)
                object_at(x + dx, y + dy + 1, 1, 2)
                object_at(x + dx + 1, y + dy - 1, 1)
    elif kind == "inn":
        for dx in (0, 3, 7, 10):
            object_at(x + dx, y, 72 + dx % 4)
            object_at(x + dx, y + 4, 72 + (dx + 1) % 4)
        object_at(x, b - 2, 8)
        object_at(r - 1, b - 2, 85)
    elif kind == "depot":
        for dx in range(0, width, 3):
            for dy in range(0, height - 2, 3):
                px = x + dx + int(dx == 0 and dy == 3)
                object_at(px, y + dy, (36, 38, 39, 76, 77, 78)[(dx + dy) % 6])
        object_at(r - 1, b - 2, 0)


# Organic ground breaks up the meadows; authored patches are stored as brush strokes.
for _ in range(85):
    x, y = rng.uniform(3, 156), rng.uniform(3, 108)
    if 7 < x < 84 and 7 < y < 78:
        continue
    if 117 < x < 148 and 51 < y < 83:
        continue
    stroke([(x, y), (min(157, x + rng.uniform(2, 7)), min(109, y + rng.uniform(1, 5)))],
           6, rng.choice((192, 256, 320, 384)))

# River, shallow margins and a separate lake with a walkable shore.
river = [(100 + 7 * math.sin(y / 16) + y * 0.045, y) for y in range(0, H + 1, 4)]
stroke(river, 6, 384)
stroke(river, 8, 352)
stroke(river, 9, 176)
lake_center = (143, 96)
clear_paint((128, 84, 157, 108))
for y in range(84, 109):
    for x in range(128, 158):
        if ((x + .5 - 143) / 11) ** 2 + ((y + .5 - 96) / 9) ** 2 < 1:
            doc["floors"][y * W + x] = 9
def lake_outline(rx, ry):
    points = []
    for i in range(33):
        angle = i * math.tau / 32
        wobble = .7 * math.sin(3 * angle) + .4 * math.cos(5 * angle)
        points.append((143 + (rx + wobble) * math.cos(angle),
                       96 + (ry + wobble) * math.sin(angle)))
    return points


shore = lake_outline(11, 9)
stroke(shore, 6, 320)
stroke(shore, 8, 176)
# Join the deeper basin to the shallows without an artificial dry inner ring.
stroke(lake_outline(8.5, 6.5), 9, 192)

# The main road, curved footpaths, and two concrete bridges.
main_road = [(76, 32), (85, 35), (92, 43), (101, 43), (110, 43), (118, 52), (132, 68)]
stroke(main_road, 6, 320)
stroke(main_road, 5, 192, reserve=True)
south_trail = [(41, 72), (41, 81), (53, 84), (67, 88), (83, 95), (97, 90), (114, 87), (126, 82), (132, 74)]
stroke(south_trail, 6, 224)
stroke(south_trail, 5, 112, reserve=True)
quarry_trail = [(41, 81), (36, 85), (31, 89), (30, 96)]
stroke(quarry_trail, 6, 224)
stroke(quarry_trail, 5, 128, reserve=True)
forest_trail = [(113, 43), (123, 37), (130, 30), (139, 24), (139, 21)]
stroke(forest_trail, 6, 192)
stroke(forest_trail, 5, 96, reserve=True)
lake_trail = [(132, 77), (128, 87), (124, 97), (129, 105), (142, 109), (153, 105)]
stroke(lake_trail, 6, 160)
stroke(lake_trail, 5, 80, reserve=True)
stroke([(91, 43), (113, 43)], 26, 256, reserve=True)
stroke([(98, 89), (121, 85)], 26, 128, reserve=True)

# City: three districts, eight homes, functional public buildings and two yards.
city = (10, 10, 80, 75)
ground(city, 18)
for road in [(10, 30, 80, 33), (10, 49, 80, 52), (39, 10, 43, 75), (61, 10, 64, 75)]:
    ground(road, 16)
ground((46, 35, 59, 47), 19)
ground((13, 72, 37, 74), 20)
clear_paint(city)

for name, rect, floor, style, kind in [
    ("Épicerie", (13, 13, 24, 27), 22, 1, "shop"),
    ("Clinique", (27, 13, 37, 27), 24, 2, "clinic"),
    ("Centre de données", (46, 13, 59, 27), 0, 4, "servers"),
    ("Armurerie", (66, 13, 79, 27), 26, 0, "armory"),
    ("Atelier", (13, 36, 24, 47), 26, 1, "workshop"),
    ("Restaurant", (27, 36, 37, 47), 23, 2, "restaurant"),
    ("Auberge", (66, 36, 79, 47), 25, 2, "inn"),
]:
    furnish(building(name, rect, floor, style), kind)

for row, (top, bottom) in enumerate(((55, 62), (65, 72))):
    for col, (left, right) in enumerate(((13, 24), (27, 37), (46, 59), (66, 79))):
        variant = row * 4 + col
        rect = (left, top, right, bottom)
        furnish(building(f"Logement {variant + 1}", rect, 25 if variant % 2 else 22,
                          1 if col % 2 else 2, "north" if row == 0 else "south"), "home", variant)

for x in (47, 51, 55):
    object_at(x, 36, 32)
    object_at(x, 39, 34)
for x, y, s in [(48, 44, 35), (57, 43, 28), (57, 46, 25), (46, 46, 26),
                (52, 46, 48), (54, 39, 33), (60, 16, 42), (60, 22, 55),
                (65, 17, 66), (60, 39, 43), (65, 44, 47), (38, 17, 45),
                (38, 41, 30), (38, 24, 46), (44, 28, 50), (44, 48, 49)]:
    object_at(x, y, s)
for x in (11, 25, 38, 44, 60, 65, 80):
    for y in (11, 29, 34, 48, 53, 74):
        object_at(x, y, 24 if y in (29, 53) else 27, optional=True)
for x in (15, 29, 48, 68):
    object_at(x, 34, 25)
    object_at(x + 2, 34, 26)
for x, y, s in [(12, 31, 40), (44, 31, 40), (65, 50, 40),
                (11, 51, 31), (80, 31, 31), (60, 51, 31)]:
    object_at(x, y, s, optional=True)

# Freight yards just outside the city, with a maintenance stairwell.
for name, rect in [("Entrepôt", (13, 78, 25, 86)), ("Maintenance", (67, 76, 79, 83))]:
    ground((rect[0] - 1, rect[1] - 1, rect[2] + 1, rect[3] + 1), 26)
    furnish(building(name, rect, 26, 0, "north"), "depot")
object_at(73, 82, 81, blocking=False, optional=True)
stroke([(19, 75), (19, 77)], 26, 128, reserve=True)

# Hamlet: houses around an irregular dirt square, workshop and a grain store.
hamlet = (119, 55, 147, 81)
stroke([(121, 67), (127, 65), (133, 68), (140, 66), (144, 69)], 6, 384, reserve=True)
for i, (name, rect, kind) in enumerate([
    ("Maison du hameau 1", (120, 55, 129, 63), "home"),
    ("Maison du hameau 2", (134, 54, 144, 63), "home"),
    ("Maison du hameau 3", (120, 72, 129, 80), "home"),
    ("Maison du hameau 4", (134, 72, 144, 80), "home"),
    ("Atelier du hameau", (148, 62, 158, 71), "workshop"),
    ("Réserve du hameau", (148, 74, 158, 82), "depot"),
]):
    side = "south" if i < 2 else "north" if i < 4 else "west"
    furnish(building(name, rect, 25 if kind == "home" else 26, 1, side), kind, i)
    door = buildings[-1]["door"]
    endpoint = (door[0] + .5, door[1] + .5)
    if i < 4:
        stroke([(door[0] + .5, 67), endpoint], 6, 160, reserve=True)
        clear_paint(rect)
for x, y, s in [(131, 66, 28), (135, 66, 25), (128, 68, 32), (139, 68, 33),
                (142, 69, 34), (130, 77, 38), (146, 65, 37), (145, 76, 76)]:
    object_at(x, y, s)

# Two visually different dungeon entries, each with a metadata destination.
object_at(138, 19, 117, size=(3, 3), blocking=False)
for x, y, sprite, scale in [(135, 18, 118, 170), (143, 19, 110, 140),
                            (137, 17, 110, 130), (143, 22, 108, 110), (136, 23, 111, 90)]:
    object_at(x, y, sprite, scale=scale)
object_at(28, 94, 116, size=(3, 3), blocking=False)
object_at(29, 96, 81, blocking=False, decoration=True, scale=65)
for x, y, s, scale in [(24, 92, 118, 170), (26, 94, 110, 160),
                       (32, 92, 118, 160), (33, 96, 119, 130),
                       (26, 98, 78, 100), (32, 98, 76, 100), (34, 95, 39, 100)]:
    object_at(x, y, s, scale=scale)
doc["markers"].extend([
    dict(pos=dict(x=139, y=21), width=1, height=1, kind="Cave",
         name="Grotte du bois", target="test/grotte-du-bois", count=0),
    dict(pos=dict(x=29, y=96), width=1, height=1, kind="Cave",
         name="Ancienne carrière", target="test/ancienne-carriere", count=0),
    dict(pos=dict(x=156, y=104), width=3, height=3, kind="Exit",
         name="Chemin vers la zone suivante", target="test/zone-suivante", count=0),
    dict(pos=dict(x=120, y=7), width=14, height=12, kind="Enemies",
         name="Bois du nord", target="", count=8),
    dict(pos=dict(x=45, y=93), width=18, height=13, kind="Enemies",
         name="Friches du sud", target="", count=6),
])


def segment_distance(x, y, a, b):
    dx, dy = b[0] - a[0], b[1] - a[1]
    length = dx * dx + dy * dy
    t = max(0, min(1, ((x - a[0]) * dx + (y - a[1]) * dy) / length)) if length else 0
    return math.hypot(x - a[0] - t * dx, y - a[1] - t * dy)


def reserved(x, y):
    if any(l <= x <= r and t <= y <= b for l, t, r, b in
           [(8, 8, 82, 87), (117, 52, 159, 84), (22, 89, 36, 101), (133, 16, 145, 25)]):
        return True
    if any(segment_distance(x, y, a, b) < radius
           for points, radius in paths for a, b in zip(points, points[1:])):
        return True
    if min(segment_distance(x, y, a, b) for a, b in zip(river, river[1:])) < 3.7:
        return True
    return ((x - 143) / 13) ** 2 + ((y - 96) / 11) ** 2 < 1


# Woods have irregular edges and different tree species; paths stay open.
forests = [(27, 4, 26, 8), (128, 18, 28, 19), (16, 95, 16, 15),
           (76, 100, 30, 13), (131, 44, 23, 9), (88, 64, 9, 19)]
for y in range(2, H - 2, 3):
    for x in range(2, W - 2, 3):
        xx, yy = x + rng.randint(-1, 1), y + rng.randint(-1, 1)
        density = min(((xx - cx) / rx) ** 2 + ((yy - cy) / ry) ** 2
                      for cx, cy, rx, ry in forests)
        if density < rng.uniform(.7, 1.1) and not reserved(xx + .5, yy + .5):
            object_at(xx, yy, rng.choices((104, 105, 106, 107), (4, 4, 3, 1))[0],
                      scale=rng.randint(150, 195), jitter=True, optional=True)
for _ in range(1800):
    x, y = rng.randrange(2, W - 2), rng.randrange(2, H - 2)
    if reserved(x + .5, y + .5):
        continue
    sprite = rng.choices((108, 109, 110, 111, 113, 114, 115), (4, 3, 1, 2, 1, 7, 1))[0]
    object_at(x, y, sprite, scale=rng.randint(65, 125), jitter=True,
              decoration=sprite in (111, 114), optional=True)
# Reeds follow the shore rather than being scattered through the deep water.
for points in (river, shore):
    for i, (x, y) in enumerate(points):
        if i % 2 == 0:
            dx, dy = (2.6 if points is river else 1.5), rng.choice((-1, 1))
            xx, yy = int(x + dx), int(y + dy)
            if 0 <= xx < W and 0 <= yy < H and not any(
                    segment_distance(xx, yy, a, b) < radius
                    for path, radius in paths for a, b in zip(path, path[1:])):
                object_at(xx, yy, 112, blocking=False, decoration=True, scale=95)

# Low vegetation and shade trees at the city entrances and by the market.
for x, y in [(9, 18), (9, 43), (81, 22), (81, 58), (45, 36), (60, 45), (45, 73)]:
    object_at(x, y, 104, scale=160, optional=True)

DESTINATION.parent.mkdir(parents=True, exist_ok=True)
if DESTINATION.exists():
    raise SystemExit(f"Refusing to overwrite an existing map: {DESTINATION}")
DESTINATION.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
manifest = dict(map=str(DESTINATION.relative_to(ROOT)), dimensions=[W, H],
                seed=20261002, buildings=buildings, landmarks=[
                    dict(name="Ville", rect=list(city)), dict(name="Hameau", rect=list(hamlet)),
                    dict(name="Grotte du bois", cell=[139, 21]),
                    dict(name="Ancienne carrière", cell=[29, 96]), dict(name="Lac", cell=[143, 96])],
                counts={k: len(doc[k]) for k in ("structures", "props", "paint", "markers")})
artifact = ROOT / "artifacts/grande-carte-test-2026-10-02"
artifact.mkdir(parents=True, exist_ok=True)
(artifact / "composition.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
print(f"Created {DESTINATION} — {len(buildings)} buildings, {len(doc['props'])} objects")
