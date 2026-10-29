"""EG-CAN02: connect existing channels; keep this layout a review candidate."""
from collections import deque
import copy
import hashlib
import heapq
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
ART = ROOT / "assets/source-art/sewer-water-02"
DOC = ROOT / "docs/reviews/sewer-water-01"
DIRS = ((1, 0), (0, 1), (-1, 0), (0, -1))


def neighbours(p):
    return [(p[0] + dx, p[1] + dy) for dx, dy in DIRS]


def flood(cells, start):
    seen = {start} if start in cells else set()
    todo = deque(seen)
    while todo:
        for q in neighbours(todo.popleft()):
            if q in cells and q not in seen:
                seen.add(q)
                todo.append(q)
    return seen


def components(cells):
    left, result = set(cells), []
    while left:
        part = flood(left, min(left))
        result.append(part)
        left -= part
    return sorted(result, key=lambda s: (-len(s), min(s)))


def route(allowed, starts, targets, cost):
    queue, distance, previous = [], {}, {}
    for p in sorted(starts):
        state = (p, -1)
        distance[state] = 0
        heapq.heappush(queue, (0, p, -1))
    end = None
    while queue:
        value, p, facing = heapq.heappop(queue)
        state = (p, facing)
        if distance.get(state) != value:
            continue
        if p in targets:
            end = state
            break
        for direction, q in enumerate(neighbours(p)):
            if q not in allowed:
                continue
            next_value = value + cost(q) + (4 if facing not in (-1, direction) else 0)
            next_state = (q, direction)
            if next_value < distance.get(next_state, float("inf")):
                distance[next_state] = next_value
                previous[next_state] = state
                heapq.heappush(queue, (next_value, q, direction))
    if end is None:
        return None
    path = [end[0]]
    while end in previous:
        end = previous[end]
        path.append(end[0])
    return path[::-1]


def read(path):
    return json.loads(path.read_text(encoding="utf-8"))


def build(index):
    source = ROOT / f"assets/maps/prison-sewer-01/variant-{index}/plan.json"
    base = read(source)
    art = read(ROOT / f"assets/source-art/sewer-review-04/seed-{index}/plan.json")
    grid = base["layout"]
    floor = {(x, y) for y, row in enumerate(grid) for x, c in enumerate(row) if c != "#"}
    old_water = {(x, y) for y, row in enumerate(grid) for x, c in enumerate(row) if c == "~"}
    old_bridges = {(x, y) for y, row in enumerate(grid) for x, c in enumerate(row) if c == "="}
    blocked = set(map(tuple, base["blocked"]))
    actors = {tuple(m["at"]) for g in base["groups"] for m in g["members"]}
    views = {tuple(v["at"]) for v in base["views"]}
    spawn = tuple(base["spawn"])
    arrival = {p for p in floor if abs(p[0] - spawn[0]) + abs(p[1] - spawn[1]) <= 6}
    protected = blocked | arrival
    # Protect the full source footprints, not just each furniture base cell.
    for prop in art["props"] + art.get("decorative_bodies", []):
        image = Image.open(ROOT / prop["file"])
        box = image.getbbox()
        if box is None:
            continue
        x, y = prop["at_px"]
        for cy in range((y + box[1]) // 64, (y + box[3] - 1) // 64 + 1):
            for cx in range((x + box[0]) // 64, (x + box[2] - 1) // 64 + 1):
                protected.add((cx, cy))
    channels = old_water | old_bridges
    allowed = (floor - protected) | channels
    paths = []
    while True:
        parts = components(channels)
        if len(parts) <= 1:
            break
        path = None
        for joined in parts:
            path = route(allowed, joined, channels - joined,
                         lambda q: 2 if q in channels else 12 + (20 if q in actors | views else 0)
                         + 6 * sum(n not in floor for n in neighbours(q)))
            if path is not None:
                break
        if path is None:
            break
        paths.append(path)
        channels.update(path)
    # Feed the longest still-dry galleries as well as joining old basins.
    corridor_cells = {tuple(p) for corridor in art["corridors"] for p in corridor["line"]}
    for _ in range(2):
        choices = (corridor_cells & allowed) - channels
        if not choices:
            break
        target = max(sorted(choices), key=lambda p: min(abs(p[0]-q[0])+abs(p[1]-q[1]) for q in channels))
        if min(abs(target[0]-q[0])+abs(target[1]-q[1]) for q in channels) < 5:
            break
        path = route(allowed, channels, {target},
                     lambda q: 10 + (20 if q in actors | views else 0) + (0 if q in corridor_cells else 3))
        if path is None:
            break
        paths.append(path)
        channels.update(path)
    new_cells = channels - old_water - old_bridges
    # New crossings preserve every existing floor region, actor and view.
    # Only new channel cells may become additional bridges.
    bridges = set(old_bridges)
    bridges.update(new_cells & (actors | views))
    for rat in base["rats"]:
        for a, b in zip(rat["path"], rat["path"][1:]):
            length = max(abs(b[0] - a[0]), abs(b[1] - a[1]))
            for step in range(int(length // 16) + 2):
                t = min(1, step * 16 / max(length, 1))
                p = (int((a[0] + (b[0] - a[0]) * t) // 64), int((a[1] + (b[1] - a[1]) * t) // 64))
                if p in new_cells:
                    bridges.add(p)
    old_dry = floor - blocked - old_water
    if len(flood(old_dry, spawn)) != len(old_dry):
        raise ValueError(f"Baseline {index} has disconnected dry ground")
    while True:
        dry = floor - blocked - (channels - bridges)
        reached = flood(dry, spawn)
        if reached == dry:
            break
        crossing = route(old_dry, reached, dry - reached, lambda q: 100 if q in new_cells else 1)
        if crossing is None:
            raise ValueError("No crossing can preserve access")
        bridges.update(p for p in crossing if p in new_cells)
    water = channels - bridges
    assert len(components(channels)) == 1, "Sewer construction left an isolated water section"
    assert new_cells.isdisjoint(protected)
    assert old_water <= water and old_bridges <= bridges
    assert actors | views | {spawn} <= dry
    assert len(flood(dry, spawn)) == len(dry)
    proposal = copy.deepcopy(base)
    proposal["layout"] = ["".join("~" if (x, y) in water else "=" if (x, y) in bridges else c
                                   for x, c in enumerate(row)) for y, row in enumerate(grid)]
    assert all(proposal[key] == base[key] for key in base if key != "layout")
    output = ART / f"variant-{index}-plan.json"
    output.write_text(json.dumps(proposal, indent=2) + "\n", encoding="utf-8")
    return {"variant": index, "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
            "candidate_sha256": hashlib.sha256(output.read_bytes()).hexdigest(),
            "water_before": len(old_water), "water_after": len(water),
            "bridges_before": len(old_bridges), "bridges_after": len(bridges),
            "channel_components_before": len(components(old_water | old_bridges)),
            "channel_components_after": len(components(channels)),
            "new_channel_cells": len(new_cells), "new_water_cells": sorted(water - old_water),
            "new_bridge_cells": sorted(bridges - old_bridges), "paths": paths,
            "all_dry_ground_connected": True, "unchanged_actor_positions": len(actors),
            "actors_views_and_arrival_clear": True, "source_footprints_avoided": True,
            "existing_channels_preserved": True, "only_layout_changed": True,
            "dry_cells_after": len(dry), "width": base["width"], "height": base["height"]}


if __name__ == "__main__":
    ART.mkdir(parents=True, exist_ok=True)
    DOC.mkdir(parents=True, exist_ok=True)
    checks = [build(i) for i in range(1, 4)]
    report = {"review": "EG-CAN02-r01", "status": "candidate_not_integrated",
              "native_cell": 64, "normal_game_activation": False, "variants": checks}
    (DOC / "canals-plan-validation.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    for row in checks:
        print({k: v for k, v in row.items() if k not in ("paths", "new_water_cells", "new_bridge_cells")})
