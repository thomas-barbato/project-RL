"""CITY-NORTH01-r02: requested square European street plan and room review.

This is a one-shot migration of the inspected r01 candidate, not a general
regenerator. Stable building/prop/decal IDs and all basement layout geometry
and seed rules survive. Only this candidate's city, room surfaces and matching
presentation metadata are edited. Re-running refuses to overwrite r02 edits.
"""
from collections import deque
from copy import deepcopy
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[1]
DIR = ROOT / "assets/maps/north-city-01"
W = H = 112
PLACEMENTS = {
    "south_west": (12, 92), "south_east": (75, 87),
    "apartments": (12, 65), "collapsed": (90, 62),
    "shop_home": (49, 54), "fire_station": (72, 27),
    "school": (8, 34), "police": (17, 10),
    "checkpoint": (83, 10), "sealed_block": (54, 7),
}


def read(path):
    return json.loads(path.read_text(encoding="utf-8"))


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(".r02.tmp")
    temporary.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def xy(item):
    return item["pos"]["x"], item["pos"]["y"]


def inside(p, rect, strict=False):
    x, y = p
    l, t, r, b = rect
    d = int(strict)
    return l + d <= x <= r - d and t + d <= y <= b - d


def moved(item, dx, dy):
    item = deepcopy(item)
    item["pos"]["x"] += dx
    item["pos"]["y"] += dy
    return item


def prop_cells(prop, at=None):
    x, y = at or xy(prop)
    details = prop.get("details", {})
    width, height = details.get("width", 1), details.get("height", 1)
    if prop.get("rotation", 0) % 2:
        width, height = height, width
    return {(xx, yy) for yy in range(y, y + height) for xx in range(x, x + width)}


def new_prop(key, sprite, x, y):
    width, height = ((2, 1) if sprite in (4, 65) else (1, 2) if 72 <= sprite <= 75 else (1, 1))
    return dict(id=f"north_city:prop:{key}", pos=dict(x=x, y=y), sprite=sprite,
                furniture=True, rotation=0, blocking=True,
                details=dict(width=width, height=height, opaque=False))


def reachable(doc, extra=()):
    width, height = doc["width"], doc["height"]
    blocked = {xy(s) for s in doc["structures"] if s["door"] is None}
    blocked.update(extra)
    for prop in doc["props"]:
        if prop["blocking"]:
            blocked.update(prop_cells(prop))
    start = xy({"pos": doc["spawn"]})
    seen, queue = {start}, deque([start])
    while queue:
        x, y = queue.popleft()
        for p in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
            if 0 < p[0] < width - 1 and 0 < p[1] < height - 1 and p not in blocked and p not in seen:
                seen.add(p)
                queue.append(p)
    expected = {(x, y) for y in range(1, height - 1) for x in range(1, width - 1)
                if (x, y) not in blocked}
    return seen, expected - seen


class Interior:
    def __init__(self, doc, building, rooms):
        self.doc, self.b, self.rooms = doc, building, rooms
        self.l, self.t, self.r, self.btm = building["rect"]
        self.width, self.height = self.r - self.l + 1, self.btm - self.t + 1
        self.flip = building["id"] == "south_east"
        self.walls = {xy(p): p for p in doc["structures"]}
        self.original = {xy(p): p for p in doc["structures"] if inside(xy(p), building["rect"], True)}
        for p in self.original:
            self.walls.pop(p)
        self.plan = []

    def point(self, x, y):
        return self.l + (self.width - 1 - x if self.flip else x), self.t + y

    def floor(self, slug, name, rect, material):
        l, t, r, b = rect
        points = [self.point(x, y) for y in range(t, b + 1) for x in range(l, r + 1)]
        for x, y in points:
            self.doc["floors"][y * W + x] = material
        xs, ys = zip(*points)
        self.rooms.append(dict(id=f"north_city:room:{self.b['id']}:{slug}", building=self.b["id"],
                               name=name, rect=[min(xs), min(ys), max(xs), max(ys)], material=material))

    def wall(self, axis, at, start, end, doors=()):
        for index in range(start, end + 1):
            x, y = (at, index) if axis == "x" else (index, at)
            gx, gy = self.point(x, y)
            old = self.original.get((gx, gy))
            self.walls[gx, gy] = dict(
                id=old["id"] if old else f"north_city:wall:{self.b['id']}:room:{x}:{y}",
                pos=dict(x=gx, y=gy), door="Closed" if index in doors else None,
                rotation=int(axis == "x"), style=2 if self.b["id"] == "school" else 5)

    def furniture(self, x, y, sprite, slug):
        self.plan.append((self.point(x, y), sprite, f"{self.b['id']}:{slug}"))

    def finish(self):
        self.doc["structures"] = list(self.walls.values())


def homes(i, large=False):
    w, h = i.width, i.height
    if large:
        i.floor("hall", "Distribution", (1, 7, w - 2, 9), 23)
        i.floor("bedroom", "Chambre", (1, 1, 8, 5), 25)
        i.floor("kitchen", "Cuisine", (10, 1, w - 2, 5), 22)
        i.floor("living", "Séjour", (1, 11, 8, h - 2), 25)
        i.floor("washroom", "Salle d’eau", (10, 11, w - 2, h - 2), 22)
        i.wall("y", 6, 1, w - 2, (4, 13))
        i.wall("y", 10, 1, w - 2, (4, 13))
        i.wall("x", 9, 1, 5)
        i.wall("x", 9, 11, h - 2)
        for x, y, sprite, key in [(2, 2, 72, "bed"), (6, 2, 85, "wardrobe"),
                                  (11, 1, 16, "cooker"), (12, 1, 17, "sink"),
                                  (16, 1, 18, "fridge"), (2, 12, 14, "sofa"),
                                  (5, 13, 4, "table"), (5, 14, 1, "chair"),
                                  (11, 12, 86, "basin"), (16, 12, 85, "linen")]:
            i.furniture(x, y, sprite, key)
    else:
        bath_left = 11 if w >= 19 else 8
        i.floor("hall", "Entrée et dégagement", (w - 5, 1, w - 2, h - 2), 23)
        i.floor("bedroom", "Chambre", (1, 1, 6, 4), 25)
        i.floor("kitchen", "Cuisine", (8, 1, w - 7, 4), 22)
        i.floor("living", "Séjour", (1, 6, bath_left - 2, h - 2), 25)
        i.floor("washroom", "Salle d’eau", (bath_left, 6, w - 7, h - 2), 22)
        i.wall("x", w - 6, 1, h - 2, (3, 7))
        i.wall("y", 5, 1, w - 7, (3, 10))
        i.wall("x", 7, 1, 4, (2,))
        i.wall("x", bath_left - 1, 6, h - 2, (7,))
        for x, y, sprite, key in [(1, 1, 72, "bed"), (5, 1, 85, "wardrobe"),
                                  (9, 1, 16, "cooker"), (10, 1, 17, "sink"),
                                  (w - 8, 1, 18, "fridge"), (1, 7, 14, "sofa"),
                                  (5, 7, 4, "table"), (5, 8, 1, "chair"),
                                  (bath_left, 6, 86, "basin")]:
            i.furniture(x, y, sprite, key)


def public_rooms(i):
    key = i.b["id"]
    if key == "shop_home":
        i.floor("shop", "Ancienne boutique", (1, 1, 7, 6), 23)
        i.floor("lodging", "Arrière-boutique et logement", (9, 1, 15, 3), 25)
        i.floor("store", "Réserve", (9, 5, 15, 6), 26)
        i.wall("x", 8, 1, 6, (3,))
        i.wall("y", 4, 9, 15, (12,))
        for x, y, sprite, slug in [(2, 1, 11, "shelf"), (5, 1, 8, "counter"),
                                    (10, 1, 72, "bed"), (13, 1, 18, "fridge"), (10, 5, 19, "cupboard")]:
            i.furniture(x, y, sprite, slug)
    elif key == "school":
        i.floor("hall", "Couloir de l’école", (1, 7, 16, 9), 23)
        i.floor("class_a", "Salle de classe A", (1, 1, 8, 5), 25)
        i.floor("class_b", "Salle de classe B", (10, 1, 16, 5), 25)
        i.floor("staff", "Salle des professeurs", (1, 11, 8, 16), 25)
        i.floor("vestibule", "Dégagement des sanitaires", (10, 11, 11, 16), 23)
        i.floor("washroom", "Sanitaires", (13, 11, 16, 16), 22)
        i.wall("y", 6, 1, 16, (4, 12))
        i.wall("y", 10, 1, 16, (4, 11))
        i.wall("x", 9, 1, 5)
        i.wall("x", 9, 11, 16)
        i.wall("x", 12, 11, 16, (14,))
        for x in (2, 5, 11, 14):
            for y in (1, 4):
                i.furniture(x, y, 12, f"desk-{x}-{y}")
                i.furniture(x, y + 1, 1, f"chair-{x}-{y}")
        for x, y, sprite, slug in [(2, 12, 12, "staff-desk"), (5, 12, 4, "staff-table"),
                                    (2, 15, 15, "staff-books"), (14, 11, 86, "basin"), (15, 15, 85, "cleaning")]:
            i.furniture(x, y, sprite, slug)
    elif key == "fire_station":
        i.floor("hall", "Circulation de service", (17, 1, 19, 15), 23)
        i.floor("dorm", "Dortoir de garde", (1, 1, 9, 6), 25)
        i.floor("gear", "Vestiaires et équipements", (11, 1, 15, 6), 23)
        i.floor("garage", "Remise et atelier", (1, 9, 15, 15), 26)
        i.wall("x", 16, 1, 15, (5, 12))
        i.wall("x", 10, 1, 6, (3,))
        i.wall("y", 8, 1, 15, (6, 13))
        for x in (2, 5, 8):
            i.furniture(x, 2, 75, f"bed-{x}")
        for x in (11, 13, 15):
            i.furniture(x, 1, 85, f"locker-{x}")
        i.furniture(12, 5, 86, "basin")
        for x in (2, 7, 12):
            i.furniture(x, 14, 65, f"workbench-{x}")
        i.furniture(2, 10, 23, "repair-machine")
        i.furniture(9, 10, 34, "repair-materials")
    elif key == "police":
        i.floor("offices", "Bureaux", (1, 1, 9, 6), 25)
        i.floor("briefing", "Salle de briefing", (11, 1, 19, 5), 23)
        i.floor("waiting", "Attente et dépôt", (1, 8, 9, 15), 22)
        i.floor("reception", "Accueil et distribution", (11, 7, 19, 12), 23)
        i.floor("stairs", "Cage d’escalier", (11, 13, 19, 15), 26)
        i.wall("x", 10, 1, 15, (3, 8, 12))
        i.wall("y", 7, 1, 9, (5,))
        i.wall("y", 6, 11, 19, (15,))
        for x in (2, 6):
            i.furniture(x, 2, 12, f"desk-{x}")
            i.furniture(x, 3, 1, f"chair-{x}")
        for x, y, sprite, slug in [(1, 5, 15, "records"), (14, 2, 4, "briefing-table"),
                                    (14, 3, 1, "briefing-chair"), (18, 2, 12, "briefing-desk"),
                                    (2, 11, 25, "waiting-bench-a"), (6, 11, 25, "waiting-bench-b"),
                                    (16, 9, 8, "front-desk")]:
            i.furniture(x, y, sprite, slug)
    elif key == "checkpoint":
        i.floor("store", "Matériel et munitions", (1, 1, 7, 11), 26)
        i.floor("office", "Poste de contrôle", (9, 1, 14, 5), 23)
        i.floor("reserve", "Réserve", (9, 7, 14, 11), 26)
        i.wall("x", 8, 1, 11, (3, 9))
        i.wall("y", 6, 9, 14, (12,))
        for x in (1, 4, 6):
            i.furniture(x, 1, 92, f"ammo-{x}")
        i.furniture(10, 2, 12, "desk")
        i.furniture(10, 3, 1, "chair")


def nearest(at, allowed):
    return min(allowed, key=lambda p: (abs(p[0] - at[0]) + abs(p[1] - at[1]), p[1], p[0]))


def main():
    review = read(DIR / "review.json")
    if review["review"] != "CITY-NORTH01-r01":
        raise SystemExit("r02 is a one-shot migration from r01; inspected edits must not be regenerated.")
    old = read(DIR / "north_city.json")
    old_zone = deepcopy(review["zones"][0])
    other_files = [p for p in DIR.glob("*.json") if p.name not in ("review.json", "north_city.json")]
    geometry_before = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in other_files}
    doc = dict(version=1, width=W, height=H, spawn=deepcopy(old["spawn"]), floors=[18] * (W * H),
               structures=[], props=[], paint=[], markers=[])
    zone = review["zones"][0]
    zone.update(rooms=[], fabric=[], road_markings=[], streets=[], plazas=[])
    offsets, footprint, preserved = {}, set(), []
    for building in zone["buildings"]:
        prior = deepcopy(building)
        l, t, r, b = prior["rect"]
        nx, ny = PLACEMENTS[building["id"]]
        dx, dy = nx - l, ny - t
        offsets[building["id"]] = dx, dy
        building["rect"] = [l + dx, t + dy, r + dx, b + dy]
        building["doors"] = [[x + dx, y + dy] for x, y in building["doors"]]
        preserved.append(dict(id=building["id"], before=prior["rect"], after=building["rect"]))
        for y in range(t, b + 1):
            for x in range(l, r + 1):
                assert (x + dx, y + dy) not in footprint, "Building overlap"
                footprint.add((x + dx, y + dy))
                doc["floors"][(y + dy) * W + x + dx] = old["floors"][y * old["width"] + x]
        doc["structures"].extend(moved(p, dx, dy) for p in old["structures"] if inside(xy(p), prior["rect"]))
        # Existing internal props are assigned below; source IDs stay stable.
        for marker in old["markers"]:
            if inside(xy(marker), prior["rect"]):
                doc["markers"].append(moved(marker, dx, dy))
    walls = {xy(s): s for s in doc["structures"]}
    for x, y in {(x, y) for x in range(W) for y in range(H) if x in (0, W - 1) or y in (0, H - 1)}:
        walls[x, y] = dict(id=f"north_city:wall:border:{x}:{y}", pos=dict(x=x, y=y), door=None, rotation=0, style=5)

    # Irregular adjoining street fronts, with passage-connected inner courts.
    fabric = {}
    def mass(key, rect):
        l, t, r, b = rect
        for y in range(t, b + 1):
            for x in range(l, r + 1):
                if (x, y) not in footprint:
                    fabric[x, y] = key
    def cut(rect):
        l, t, r, b = rect
        for y in range(t, b + 1):
            for x in range(l, r + 1): fabric.pop((x, y), None)
    for key, rectangles in {
        "F01": [(4, 4, 39, 9), (4, 10, 16, 25)],
        "F02": [(8, 29, 25, 33), (27, 30, 35, 43)],
        "F03": [(4, 59, 11, 86), (12, 59, 31, 64), (32, 65, 38, 69)],
        "F04": [(5, 85, 31, 91), (5, 92, 11, 105)],
        "F05": [(75, 80, 104, 86), (92, 87, 104, 105), (75, 98, 91, 105)],
        "F06": [(81, 53, 106, 61), (81, 62, 89, 74)],
        "F07": [(64, 22, 92, 26), (64, 27, 71, 40)],
        "F08": [(47, 11, 71, 23), (48, 4, 53, 10)],
        "F09": [(43, 67, 63, 90), (64, 73, 68, 85)],
    }.items():
        for rect in rectangles: mass(key, rect)
    for rect in [(8, 12, 13, 21), (3, 18, 8, 20),
                 (5, 65, 9, 77), (3, 70, 5, 72),
                 (6, 94, 9, 101), (4, 97, 6, 99),
                 (95, 89, 101, 99), (101, 94, 105, 96),
                 (49, 73, 58, 82), (42, 77, 49, 79)]:
        cut(rect)
    sealed = []
    for (x, y), key in fabric.items():
        perimeter = any((x + dx, y + dy) not in fabric and (x + dx, y + dy) not in footprint
                        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)))
        walls[x, y] = dict(id=f"north_city:wall:{key}:{x}:{y}", pos=dict(x=x, y=y),
                           door=None, rotation=0, style=1 if key in ("F01", "F02", "F03", "F04") else 2)
        doc["floors"][y * W + x] = 23 if key in ("F01", "F03", "F05", "F08") else 26
        if not perimeter: sealed.append([x, y])
    zone["fabric"] = [dict(id=key, cells=[list(p) for p, value in fabric.items() if value == key])
                      for key in sorted(set(fabric.values()))]
    for p in old_zone["sealed"]:
        owner = next(b for b in old_zone["buildings"] if inside(p, b["rect"]))
        dx, dy = offsets[owner["id"]]
        sealed.append([p[0] + dx, p[1] + dy])
    zone["sealed"] = sealed
    blocked_exterior = footprint | set(fabric)
    doc["structures"] = list(walls.values())

    def road(name, points, width=3):
        path = []
        for a, b in zip(points, points[1:]):
            x, y = a
            while (x, y) != b:
                path.append((x, y))
                if x != b[0]: x += 1 if b[0] > x else -1
                else: y += 1 if b[1] > y else -1
            path.append(b)
        half = width // 2
        for x, y in path:
            for yy in range(y - half, y + half + 1):
                for xx in range(x - half, x + half + 1):
                    if 0 < xx < W - 1 and 0 < yy < H - 1 and (xx, yy) not in blocked_exterior:
                        doc["floors"][yy * W + xx] = 17
        zone["streets"].append(dict(id=f"north_city:street:{len(zone['streets']) + 1:02d}",
                                    name=name, points=points, width=width))
        if width >= 5:
            for index in range(2, len(path) - 1, 5):
                x, y = path[index]
                if (x, y) in blocked_exterior: continue
                zone["road_markings"].append(dict(at=[x, y], horizontal=path[index - 1][1] == y))
    road("Rue des logements", [(41, 109), (41, 104), (36, 104), (36, 85), (35, 85), (35, 73), (36, 73), (36, 60), (40, 60), (40, 52), (44, 52)])
    road("Rue du commissariat", [(44, 52), (44, 44), (40, 44), (40, 30), (41, 30), (41, 18)], 3)
    road("Route de la caserne", [(44, 52), (52, 52), (52, 50), (62, 50), (62, 47), (96, 47), (96, 35), (99, 35), (99, 26), (102, 26), (102, 5), (99, 5)], 5)
    road("Passage de l’école", [(35, 85), (3, 85), (3, 53), (4, 53), (4, 28)], 3)
    road("Rue de la vieille ville", [(4, 28), (41, 28)], 3)
    road("Traverse des commerces", [(44, 52), (44, 64), (71, 64)], 3)
    road("Rue des ruines", [(71, 64), (72, 64), (72, 76), (108, 76), (108, 49), (98, 49)], 3)
    road("Rue des cours", [(41, 104), (58, 104), (58, 96), (70, 96), (70, 82), (72, 82), (72, 76)], 3)
    road("Ruelle de la cour intérieure", [(35, 77), (49, 77), (53, 77)], 3)
    road("Rue des remparts", [(41, 18), (41, 3), (76, 3), (76, 7), (99, 7)], 3)
    road("Accès au poste militaire", [(78, 7), (78, 16), (81, 16)], 3)
    zone["plazas"] = [dict(id="north_city:place:school", name="Cour de l’école", rect=[27, 44, 37, 52]),
                      dict(id="north_city:place:central", name="Petite place des commerces", rect=[38, 47, 49, 61]),
                      dict(id="north_city:place:south", name="Placette d’entrée", rect=[33, 100, 47, 109])]
    # An irregular shop square rather than another equal-width crossroads.
    for y in range(47, 62):
        for x in range(38 + (1 if y < 50 else 0), 50 - (2 if y > 58 else 0)):
            if (x, y) not in blocked_exterior: doc["floors"][y * W + x] = 18

    plans = []
    for building in zone["buildings"]:
        if not building["accessible"]: continue
        interior = Interior(doc, building, zone["rooms"])
        if building["id"] in ("south_west", "south_east", "apartments"):
            homes(interior, building["id"] == "apartments")
        else: public_rooms(interior)
        interior.finish()
        plans.extend(interior.plan)
    # Preserve stairs and reciprocal street anchors; move only the police's
    # street-side anchor together with its building.
    for link in review["links"]:
        if link["source"] == "review:north_city":
            dx, dy = offsets["police"]
            link["at"] = [link["at"][0] + dx, link["at"][1] + dy]
    for marker in old["markers"]:
        if any(inside(xy(marker), b["rect"]) for b in old_zone["buildings"]): continue
        marker = deepcopy(marker)
        if xy(marker) == tuple(review["future_exit"]["at"]):
            marker["pos"] = dict(x=99, y=4)
        doc["markers"].append(marker)
    review["future_exit"]["at"] = [99, 4]
    for x in range(94, 106):
        if x not in (98, 99, 100):
            doc["structures"].append(dict(id=f"north_city:wall:barracks:{x}", pos=dict(x=x, y=8),
                                           door=None, rotation=0, style=7))
    _, unreachable = reachable(doc)
    if unreachable: raise ValueError(("Unconnected room/court before furniture", sorted(unreachable)[:20]))

    structures = {xy(s) for s in doc["structures"]}
    reserved = {xy(m) for m in doc["markers"]} | {(41, 108), (41, 109), (99, 4)}
    for s in doc["structures"]:
        if s["door"] is not None:
            x, y = xy(s)
            reserved.update(((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)))
    claimed = set()
    pool = []
    for prop in old["props"]:
        owner = next((b for b in old_zone["buildings"] if inside(xy(prop), b["rect"])), None)
        dx, dy = offsets[owner["id"]] if owner else (0, 0)
        pool.append((owner["id"] if owner else None, moved(prop, dx, dy)))
    def place(prop, at, allowed, critical=False):
        candidates = sorted(allowed, key=lambda p: (abs(p[0] - at[0]) + abs(p[1] - at[1]), p[1], p[0]))
        for p in candidates:
            cells = prop_cells(prop, p)
            if cells & (structures | claimed) or not cells <= allowed: continue
            if prop["blocking"] and cells & reserved: continue
            candidate = deepcopy(prop)
            candidate["pos"] = dict(x=p[0], y=p[1])
            doc["props"].append(candidate)
            if not prop["blocking"] or not reachable(doc)[1]:
                claimed.update(cells)
                return
            doc["props"].pop()
        if critical: raise ValueError(("No connected prop placement", prop["id"]))
    for at, sprite, key in plans:
        owner_id = key.split(":")[0]
        existing = next(((index, p) for index, (owner, p) in enumerate(pool)
                         if owner == owner_id and p["sprite"] == sprite), None)
        prop = pool.pop(existing[0])[1] if existing else new_prop(key, sprite, *at)
        b = next(b for b in zone["buildings"] if b["id"] == owner_id)
        room = next((r for r in zone["rooms"] if r["building"] == owner_id and inside(at, r["rect"])), None)
        allowed = {p for p in footprint if inside(p, room["rect"] if room else b["rect"], bool(not room))}
        place(prop, at, allowed, True)
    exterior = {(x, y) for x in range(1, W - 1) for y in range(1, H - 1) if (x, y) not in blocked_exterior}
    pavements = {p for p in exterior if doc["floors"][p[1] * W + p[0]] == 18}
    exterior_anchors = [(33, 100), (47, 105), (32, 73), (38, 63), (39, 33), (46, 44),
                        (93, 40), (94, 27), (103, 15), (81, 18), (28, 46), (30, 47),
                        (32, 48), (73, 93), (68, 97), (109, 73)]
    for index, (owner, prop) in enumerate(pool):
        if owner:
            b = next(b for b in zone["buildings"] if b["id"] == owner)
            allowed = {p for p in footprint if inside(p, b["rect"], True)}
            at = xy(prop)
        else:
            allowed, at = pavements, exterior_anchors[index % len(exterior_anchors)]
        place(prop, at, allowed, True)
    # All source decals keep their file, kind, ID and relative cluster offsets.
    clusters = [(42, 101, 39, 104), (39, 90, 62, 98), (34, 70, 36, 74),
                (45, 60, 72, 64), (34, 45, 96, 45), (46, 29, 43, 31), (40, 12, 101, 12)]
    for prop in zone["scenery"]:
        if prop.get("building"):
            dx, dy = offsets[prop["building"]]
            prop["at_px"][0] += dx * 64
            prop["at_px"][1] += dy * 64
            b = next(b for b in zone["buildings"] if b["id"] == prop["building"])
            candidates = {p for p in footprint if inside(p, b["rect"], True)} - structures - claimed
        else:
            cx, cy, nx, ny = min(clusters, key=lambda c: abs(prop["at_px"][1] / 64 - c[1]))
            prop["at_px"][0] += (nx - cx) * 64
            prop["at_px"][1] += (ny - cy) * 64
            candidates = exterior - structures - claimed
        at = prop["at_px"][0] // 64, prop["at_px"][1] // 64
        if at not in candidates:
            p = nearest(at, candidates)
            prop["at_px"] = [p[0] * 64, p[1] * 64]
    for view in zone["views"]:
        owner = {"02-logement": "south_west", "04-pompiers": "fire_station",
                 "05-ecole": "school", "06-commissariat": "police"}.get(view["id"])
        if owner:
            dx, dy = offsets[owner]
            view["at"] = [view["at"][0] + dx, view["at"][1] + dy]
        else:
            view["at"] = {"01-arrivee": [38, 102], "03-carrefour": [43, 53], "07-caserne": [101, 11]}[view["id"]]
    seen, unreachable = reachable(doc)
    assert not unreachable, sorted(unreachable)[:20]
    for view in zone["views"]:
        if tuple(view["at"]) not in seen: view["at"] = list(nearest(view["at"], seen))
    for marker in doc["markers"]: assert xy(marker) in seen, marker
    review["review"] = "CITY-NORTH01-r02"
    review["layout_revision"] = dict(scope="Square European street fabric and requested functional room interiors",
                                      width=112, height=112, preserved_building_ids=[b["id"] for b in zone["buildings"]],
                                      source_review="CITY-NORTH01-r01", basement_selection_unchanged=True)
    review["validation"] = dict(layout_prevalidation="pending-user-inspection", visual_prevalidation="pending-user-inspection",
                                 final_in_game_validation="pending-user-inspection", automated_checks_are_user_approval=False)
    assert {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in other_files} == geometry_before
    assert {p["id"] for p in old["props"]} <= {p["id"] for p in doc["props"]}
    assert {p["id"] for p in old_zone["scenery"]} == {p["id"] for p in zone["scenery"]}
    history = DIR / "history"
    write(history / "r01-city.json", old)
    write(history / "r01-review.json", read(DIR / "review.json"))
    write(DIR / "north_city.json", doc)
    write(DIR / "review.json", review)
    record = dict(review=review["review"], size=[W, H], moves=preserved,
                  protected_files_unchanged=geometry_before, source_props_retained=len(old["props"]),
                  current_props=len(doc["props"]), decal_ids_retained=len(zone["scenery"]),
                  rooms=len(zone["rooms"]), walkable_cells=len(seen), fabric_blocks=len(zone["fabric"]),
                  streets=len(zone["streets"]))
    write(ROOT / "docs/reviews/north-city-01/CHANGEMENTS-r02.json", record)
    print(json.dumps({k: v for k, v in record.items() if k not in ("moves", "protected_files_unchanged")}))


if __name__ == "__main__":
    main()
