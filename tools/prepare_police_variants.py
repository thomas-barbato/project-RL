"""Add only the requested seeded police basement variants to CITY-NORTH01.

The street, city, ground-floor police, upper floor and their art stay byte-for-byte
unchanged. Existing variants are never regenerated after a review edit.
"""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import random

from build_north_city_review import District, OUT


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def place_pool(district, pool, variant):
    """Same furniture and counts, placed around walls with connected aisles."""
    rng = random.Random(20261105 + variant * 97 + len(district.key))
    candidates = [(x, y) for y in range(2, 25) for x in range(2, 33)
                  if (x, y) not in district.walls
                  and not (26 <= x <= 32 and (y >= 21 or y <= 5))]
    rng.shuffle(candidates)
    candidates.sort(key=lambda p: -sum((p[0] + dx, p[1] + dy) in district.walls
                                      for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0))))
    for prop in pool:
        if not prop["blocking"]:
            continue
        for x, y in candidates:
            before = deepcopy(district.claimed)
            if not district.prop(x, y, prop["sprite"], prop["rotation"],
                                 (prop["details"]["width"], prop["details"]["height"])):
                continue
            # Reject furniture which seals a doorway or encloses walkable floor.
            if connected(district):
                break
            district.doc["props"].pop()
            district.claimed = before
        else:
            raise ValueError(f"No connected furniture placement for {district.key}")


def connected(district):
    from collections import deque
    blocked = set(district.walls) | district.claimed
    start = (29, 23)
    seen, queue = {start}, deque([start])
    while queue:
        x, y = queue.popleft()
        for p in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
            if 0 < p[0] < 35 and 0 < p[1] < 27 and p not in blocked and p not in seen:
                seen.add(p)
                queue.append(p)
    expected = {(x, y) for x in range(1, 35) for y in range(1, 27) if (x, y) not in blocked}
    return seen == expected


def make_variant(key, name, variant, pool, gore_pool, lower):
    district = District(key, name, 36, 28, (29, 23))
    district.floor((1, 1, 34, 26), 26)
    if variant == 1:
        for y in (7, 16):
            district.partition((0, 0, 35, 27), "y", y, [27, 28, 29, 30], 5)
        for x in (8, 16, 24):
            for t, b in ((0, 7), (7, 16), (16, 27)):
                district.partition((0, t, 35, b), "x", x, [t + 3, t + 4], 7)
    elif variant == 2:
        # A central north/south service corridor; broad rooms at each side.
        for x in (14, 21):
            district.partition((0, 0, 35, 27), "x", x, [4, 5, 12, 13, 21, 22], 7)
        for y in (8, 17):
            district.partition((0, 0, 14, 27), "y", y, [6, 7], 5)
            district.partition((21, 0, 35, 27), "y", y, [28, 29], 5)
    else:
        # Offset T-shaped corridors and a large central evidence store.
        for y in (9, 19):
            district.partition((0, 0, 35, 27), "y", y, [5, 6, 28, 29], 5)
        district.partition((0, 0, 35, 9), "x", 17, [4, 5], 7)
        district.partition((0, 9, 35, 19), "x", 11, [13, 14], 7)
        district.partition((0, 9, 35, 19), "x", 24, [14, 15], 7)
        district.partition((0, 19, 35, 27), "x", 18, [23, 24], 7)
    place_pool(district, pool, variant)
    district.prop(29, 23, 81, blocking=False)
    district.marker((29, 23), "Monter — accueil" if lower == 1 else "Monter — cellules",
                    "review:north_city" if lower == 1 else "review:police_basement")
    if lower == 1:
        district.prop(29, 3, 80, blocking=False)
        district.marker((29, 3), "Descendre — réserves sécurisées", "review:police_deep")
    free = [(x, y) for x in range(2, 33) for y in range(2, 25)
            if (x, y) not in district.walls and (x, y) not in district.claimed]
    random.Random(variant * 111 + lower).shuffle(free)
    for i, original in enumerate(gore_pool):
        x, y = free[i]
        item = deepcopy(original)
        item.update(id=f"{key}:scenery:{i+1:03d}", at_px=[x * 64, y * 64], building=None)
        district.scenery.append(item)
    district.view("08-cellules" if lower == 1 else "10-reserves", (29, 14))
    return district.save()


def main():
    manifest_path = OUT / "review.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if any(z.get("variants") for z in manifest["zones"]):
        raise SystemExit("Police variants already exist. Edit their stable IDs; regeneration is disabled.")
    protected = [OUT / "north_city.json", OUT / "police_upper.json"]
    before = {str(p): digest(p) for p in protected}
    basement = manifest["zones"][1]
    base = json.loads((OUT / "police_basement.json").read_text(encoding="utf-8"))
    pools = (base["props"], basement["scenery"])
    additions = []
    for lower, key, name, salt in [
            (1, "police_basement", "Commissariat — cellules et réserves", 0x504f4c4943453031),
            (2, "police_deep", "Commissariat — réserves sécurisées", 0x504f4c4943453032)]:
        variants = []
        for variant in (1, 2, 3):
            path = OUT / f"{key}_layout{variant:02d}.json"
            if path.exists():
                raise SystemExit(f"Refusing to replace an existing variant: {path}")
            data = make_variant(f"{key}_layout{variant:02d}", name, variant, *pools, lower)
            variants.append(dict(number=variant, file=data["file"], scenery=data["scenery"],
                                 views=data["views"], sealed=data["sealed"]))
        if lower == 1:
            zone = basement
        else:
            zone = dict(id="review:police_deep", name=name, buildings=[], scenery=[],
                        views=[], sealed=[], file=variants[0]["file"])
            additions.append(zone)
        zone.update(variants=variants, variant_salt=salt)
    manifest["zones"].extend(additions)
    manifest["links"].append(dict(source="review:police_basement", at=[29, 3],
                                  destination="review:police_deep", arrival=[29, 23]))
    manifest["randomization"] = dict(
        decision="At new-game construction, from game seed and a dedicated stable per-floor salt.",
        policy="Different visits, saves and cache eviction never reroll a floor.",
        basement_levels=2, variants_per_level=3,
        retained="Same materials and same furniture/decal inventories per floor.",
        persistent_selection="Selected variant is included in the zone ID stored by WorldState.")
    assert before == {str(p): digest(p) for p in protected}, "Unrelated map changed"
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print("Added 3 layouts on each of 2 basement levels; city and police upper floor unchanged.")


if __name__ == "__main__":
    main()
