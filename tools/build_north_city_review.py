"""CITY-NORTH01: reproducible authored maps for an isolated engine review.

Only this candidate's assets are written. No campaign, editor recovery, save,
accepted drawing or distribution manifest is changed.
"""
from collections import deque
from pathlib import Path
import json
import random

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "assets/maps/north-city-01"
RNG = random.Random(20261105)
CORPSES = "assets/maps/prison-street-expanded-01/decor"


class District:
    def __init__(self, key, name, width, height, spawn):
        self.key, self.name = key, name
        self.w, self.h = width, height
        self.doc = dict(version=1, width=width, height=height,
                        spawn=dict(x=spawn[0], y=spawn[1]), floors=[18] * (width * height),
                        structures=[], props=[], paint=[], markers=[])
        self.walls, self.claimed = {}, set()
        self.buildings, self.scenery, self.views, self.sealed = [], [], [], []
        self.rectangle((0, 0, width - 1, height - 1), 5)

    def floor(self, rect, material):
        l, t, r, b = rect
        for y in range(t, b + 1):
            for x in range(l, r + 1):
                self.doc["floors"][y * self.w + x] = material

    def wall(self, x, y, style=5, door=False, rotation=0):
        self.walls[x, y] = dict(id=f"{self.key}:wall:{x}:{y}", pos=dict(x=x, y=y), door="Closed" if door else None,
                                rotation=rotation, style=style)

    def rectangle(self, rect, style):
        l, t, r, b = rect
        for x in range(l, r + 1):
            self.wall(x, t, style)
            self.wall(x, b, style, rotation=2)
        for y in range(t + 1, b):
            self.wall(l, y, style, rotation=3)
            self.wall(r, y, style, rotation=1)

    def building(self, key, name, rect, material=22, style=1, side="south", accessible=True):
        l, t, r, b = rect
        self.floor(rect, material)
        self.rectangle(rect, style)
        door = dict(south=((l + r) // 2, b), north=((l + r) // 2, t),
                    west=(l, (t + b) // 2), east=(r, (t + b) // 2))[side]
        if accessible:
            self.wall(*door, style, door=True, rotation=int(side in ("west", "east")))
        else:
            for y in range(t + 1, b):
                for x in range(l + 1, r):
                    self.wall(x, y, style)
                    self.sealed.append([x, y])
        building = dict(id=key, name=name, rect=list(rect), doors=[list(door)] if accessible else [],
                        accessible=accessible, roof_floor=23 if style == 1 else 26)
        self.buildings.append(building)
        return building

    def partition(self, rect, axis, at, openings, style=2):
        l, t, r, b = rect
        cells = [(at, y) for y in range(t + 1, b)] if axis == "x" else [(x, at) for x in range(l + 1, r)]
        for x, y in cells:
            if (y if axis == "x" else x) not in openings:
                self.wall(x, y, style)

    def prop(self, x, y, sprite, rotation=0, size=None, blocking=True):
        w, h = size or ((2, 1) if sprite in (4, 65) else (1, 2) if 72 <= sprite <= 75 else (1, 1))
        rw, rh = (h, w) if rotation % 2 else (w, h)
        cells = {(xx, yy) for yy in range(y, y + rh) for xx in range(x, x + rw)}
        if any(p in self.walls or p in self.claimed for p in cells):
            return False
        assert all(0 < xx < self.w - 1 and 0 < yy < self.h - 1 for xx, yy in cells)
        if blocking:
            self.claimed.update(cells)
        self.doc["props"].append(dict(id=f"{self.key}:prop:{x}:{y}:{sprite}", pos=dict(x=x, y=y), sprite=sprite, furniture=True,
                                     rotation=rotation, blocking=blocking,
                                     details=dict(width=w, height=h, opaque=False)))
        return True

    def gore(self, x, y, kind="civilian", roof=None):
        if kind == "blood":
            file = f"{CORPSES}/civilians-08/SANG-U01-0{RNG.choice((4, 5))}-native.png"
        elif kind == "civilian":
            file = f"{CORPSES}/civilians-08/CIV-U01-0{RNG.choice((4, 5, 6, 7, 8))}-native.png"
        else:
            file = f"{CORPSES}/CORP-U01-{RNG.choice(('01', '02', '03'))}.png"
        self.scenery.append(dict(id=f"{self.key}:scenery:{len(self.scenery) + 1:03d}", file=file, at_px=[x * 64 + RNG.randrange(-10, 11),
                                                  y * 64 + RNG.randrange(-8, 9)],
                                 kind=kind, building=roof))

    def marker(self, at, name, target):
        self.doc["markers"].append(dict(id=f"{self.key}:passage:{at[0]}:{at[1]}", pos=dict(x=at[0], y=at[1]), width=1, height=1,
                                        kind="Exit", name=name, target=target, count=0))

    def view(self, key, at):
        self.views.append(dict(id=key, at=list(at)))

    def save(self):
        self.doc["structures"] = list(self.walls.values())
        # Check every accessible floor, considering ordinary closed doors open.
        walls = {p for p, value in self.walls.items() if value["door"] is None}
        start = tuple(self.doc["spawn"].values())
        reachable, queue = {start}, deque([start])
        while queue:
            x, y = queue.popleft()
            for p in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
                if (0 < p[0] < self.w - 1 and 0 < p[1] < self.h - 1
                        and p not in walls and p not in self.claimed and p not in reachable):
                    reachable.add(p)
                    queue.append(p)
        expected = {(x, y) for y in range(1, self.h - 1) for x in range(1, self.w - 1)
                    if (x, y) not in walls and (x, y) not in self.claimed}
        assert reachable == expected, (self.key, sorted(expected - reachable)[:20])
        for marker in self.doc["markers"]:
            assert tuple(marker["pos"].values()) in reachable
        for view in self.views:
            assert tuple(view["at"]) in reachable, view
        for prop in self.scenery:
            assert (ROOT / prop["file"]).is_file(), prop["file"]
        (OUT / f"{self.key}.json").write_text(json.dumps(self.doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        return dict(id=f"review:{self.key}", name=self.name, file=f"assets/maps/north-city-01/{self.key}.json",
                    buildings=self.buildings, scenery=self.scenery, views=self.views,
                    sealed=self.sealed, walkable_cells=len(reachable))


def home(city, key, name, rect, side="south", ruined=False):
    building = city.building(key, name, rect, 25, 1, side)
    l, t, r, b = rect
    city.partition(rect, "x", l + 5, [b - 3, b - 2], 1)
    for x, y, sprite in [(l + 1, t + 1, 72), (r - 2, t + 1, 18),
                          (l + 7, t + 1, 14), (l + 7, b - 3, 4), (r - 1, b - 2, 19)]:
        city.prop(x, y, sprite)
    city.gore(l + 2, b - 3, roof=key)
    city.gore(l + 3, b - 3, "blood", key)
    if ruined:
        for x in range(r - 3, r):
            city.walls.pop((x, t), None)
        city.prop(r - 2, t + 2, 111, blocking=False)
        city.prop(r - 2, b - 2, 119)
    return building


def build():
    # Bootstrap only. Review data becomes authoritative once created; editing
    # one element must never silently regenerate its neighbours.
    if (OUT / "review.json").exists():
        raise SystemExit("Candidate already exists. Edit its stable IDs; regeneration is disabled to preserve review changes.")
    OUT.mkdir(parents=True, exist_ok=True)
    city = District("north_city", "Quartier nord — route de la caserne", 80, 112, (41, 108))
    city.floor((37, 1, 45, 110), 17)
    for top in (26, 57, 87):
        city.floor((1, top, 78, top + 5), 17)
    city.floor((5, 1, 8, 110), 17)
    city.floor((71, 1, 74, 110), 17)

    home(city, "south_west", "Immeuble éventré", (12, 96, 31, 106), "east", True)
    home(city, "south_east", "Logements abandonnés", (51, 96, 67, 106), "west")
    home(city, "apartments", "Immeuble d'habitation", (12, 66, 31, 82), "east")
    city.building("collapsed", "Tour effondrée — inaccessible", (50, 65, 66, 72), accessible=False)
    home(city, "shop_home", "Ancien commerce et logement", (50, 76, 66, 83), "west", True)

    # Fire station: equipment store, sleeping room, open garage bays.
    rect = (11, 35, 31, 51)
    city.building("fire_station", "Caserne de pompiers", rect, 26, 5, "east")
    city.partition(rect, "y", 43, [20, 21, 29, 30], 5)
    city.partition((11, 35, 31, 43), "x", 21, [40, 41], 5)
    for x in (12, 15, 18):
        city.prop(x, 36, 75)
    for x in (23, 25, 27, 29):
        city.prop(x, 36, 85)
    for x in (13, 19, 25):
        city.prop(x, 46, 65)
    city.prop(32, 45, 28)
    city.gore(17, 47, "military", "fire_station")
    city.gore(20, 46, "blood", "fire_station")

    # School: central hallway, classrooms, staff room, playground outside.
    rect = (50, 35, 67, 52)
    city.building("school", "École abandonnée", rect, 24, 2, "west")
    city.partition(rect, "x", 57, [42, 43, 44, 45], 2)
    city.partition(rect, "y", 43, [51, 52, 55, 56, 57, 58, 59], 2)
    for x in (51, 54, 60, 63):
        for y in (37, 40, 46, 49):
            city.prop(x, y, 12)
            city.prop(x, y + 1, 1)
    city.prop(51, 36, 15)
    city.prop(61, 36, 15)
    for x in (53, 59, 65):
        city.prop(x, 54, 25)
    # Existing adult civilian artwork only; no new child imagery.
    city.gore(53, 44, roof="school")
    city.gore(56, 45, "blood", "school")

    # Police ground floor: reception, offices, custody and stairs.
    rect = (12, 7, 32, 23)
    city.building("police", "Commissariat — rez-de-chaussée", rect, 24, 5, "east")
    city.partition(rect, "x", 23, [14, 15, 16], 5)
    city.partition((12, 7, 23, 23), "y", 15, [20, 21], 5)
    for x in (14, 18, 25, 28):
        city.prop(x, 9, 12)
        city.prop(x, 10, 1)
    for x in (13, 16, 19):
        city.prop(x, 17, 85)
    city.prop(26, 20, 80, blocking=False)
    city.prop(29, 20, 81, blocking=False)
    city.gore(26, 15, "military", "police")
    city.gore(28, 16, "blood", "police")
    city.gore(16, 18, roof="police")

    city.building("checkpoint", "Ancien poste militaire", (51, 10, 66, 22), 26, 5, "west")
    for x in (52, 55, 58, 61, 64):
        city.prop(x, 11, 92)
    city.prop(54, 18, 97)
    city.gore(57, 18, "military", "checkpoint")
    city.gore(60, 18, "blood", "checkpoint")
    city.building("sealed_block", "Immeuble incendié — inaccessible", (13, 1, 30, 4), accessible=False)

    # Barracks is the end of this map: a perimeter and a future passage,
    # not an invented completed military base beyond it.
    for x in range(34, 49):
        if x not in (40, 41, 42):
            city.wall(x, 6, 7)
    for x, y in [(35, 8), (47, 8), (35, 11), (47, 11)]:
        city.prop(x, y, 31)
    city.marker((41, 4), "Entrée de la caserne militaire", "core:military_barracks")
    city.marker((41, 108), "Retour — X 18 ; Y 1", "core:prison_street")
    city.marker((41, 109), "Retour — X 18 ; Y 2", "core:prison_street")
    city.marker((26, 20), "Commissariat — sous-sol", "review:police_basement")
    city.marker((29, 20), "Commissariat — étage", "review:police_upper")

    # Clustered aftermath: evacuation, emergency response and military retreat.
    for cx, cy, count, kind in [(42, 101, 9, "civilian"), (39, 90, 6, "civilian"),
                                (34, 70, 6, "civilian"), (45, 60, 8, "military"),
                                (34, 45, 6, "civilian"), (46, 29, 9, "military"),
                                (40, 12, 12, "military")]:
        for _ in range(count):
            x, y = cx + RNG.randint(-2, 2), cy + RNG.randint(-3, 3)
            if (x, y) not in city.walls and (x, y) not in city.claimed:
                city.gore(x, y, kind)
                city.gore(x + 1, y, "blood")
    # Street furnishings stay outside the uninterrupted central travel lane.
    for y in (18, 39, 69, 98):
        city.prop(35, y, 24)
        city.prop(47, y + 1, 24)
        city.prop(34, y + 3, 26)
    for x, y in [(33, 96), (49, 86), (33, 67), (49, 54), (33, 33), (48, 20)]:
        city.prop(x, y, 111, blocking=False)
    for key, at in [("01-arrivee", (41, 102)), ("02-logement", (26, 102)),
                     ("03-carrefour", (41, 60)), ("04-pompiers", (25, 48)),
                     ("05-ecole", (55, 44)), ("06-commissariat", (27, 15)),
                     ("07-caserne", (41, 10))]:
        city.view(key, at)

    basement = District("police_basement", "Commissariat — cellules et réserves", 36, 28, (29, 23))
    basement.floor((1, 1, 34, 26), 26)
    for y in (7, 16):
        basement.partition((0, 0, 35, 27), "y", y, [27, 28, 29, 30], 5)
    for x in (8, 16, 24):
        for t, b in ((0, 7), (7, 16), (16, 27)):
            basement.partition((0, t, 35, b), "x", x, [t + 3, t + 4], 7)
    for x in (2, 10, 18):
        basement.prop(x, 2, 74)
        basement.prop(x, 9, 74)
        basement.gore(x + 1, 12)
        basement.gore(x + 2, 12, "blood")
        for xx in (x, x + 2, x + 4):
            basement.prop(xx, 18, 92)
            basement.prop(xx, 20, 94)
    basement.prop(29, 23, 81, blocking=False)
    basement.marker((29, 23), "Monter — accueil", "review:north_city")
    basement.view("08-cellules", (20, 12))

    upper = District("police_upper", "Commissariat — bureaux et archives", 36, 28, (29, 23))
    upper.floor((1, 1, 34, 26), 24)
    for x in (11, 23):
        upper.partition((0, 0, 35, 27), "x", x, [12, 13, 14, 15], 2)
    upper.partition((0, 0, 35, 27), "y", 12, [6, 7, 17, 18, 29, 30], 2)
    for x in (2, 6, 14, 18, 26, 30):
        for y in (3, 7, 18):
            upper.prop(x, y, 12)
            upper.prop(x, y + 1, 1)
    for x in (2, 4, 6, 8):
        upper.prop(x, 24, 15)
    for x, y in ((7, 19), (18, 8), (27, 15)):
        upper.gore(x, y, "military")
        upper.gore(x + 1, y, "blood")
    upper.prop(29, 23, 80, blocking=False)
    upper.marker((29, 23), "Descendre — accueil", "review:north_city")
    upper.view("09-archives", (7, 20))

    zones = [zone.save() for zone in (city, basement, upper)]
    manifest = dict(review="CITY-NORTH01-r01", status="candidate-awaiting-visual-review",
                    native_cell=64, campaign_integrated=False, seed=20261105,
                    street_links=[dict(at=[18, 1], arrival=[41, 108]),
                                  dict(at=[18, 2], arrival=[41, 109])],
                    links=[dict(source="review:north_city", at=[26, 20],
                                destination="review:police_basement", arrival=[29, 23]),
                           dict(source="review:north_city", at=[29, 20],
                                destination="review:police_upper", arrival=[29, 23])],
                    future_exit=dict(zone="review:north_city", at=[41, 4], target="core:military_barracks"),
                    zones=zones,
                    scope_notes=["Exploration prototype using the real map/door/passage/snapshot engine.",
                                 "No new combat stats, loot, narrative quest or barracks interior.",
                                 "Civilian and armoured adult corpses reuse accepted native artwork.",
                                 "Small monster corpse artwork remains to draw and review.",
                                 "Buildings are cutaway footprints; final facade identities require review."])
    (OUT / "review.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(dict(dimensions=[80, 112], buildings=len(city.buildings),
                          zones=len(zones), scenery=sum(len(z["scenery"]) for z in zones),
                          walkable=[z["walkable_cells"] for z in zones])))


if __name__ == "__main__":
    build()
