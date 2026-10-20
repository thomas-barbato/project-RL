"""Compose a fixed U01 review from source art; no game or map-editor writes."""

from collections import deque
import argparse
import hashlib
import html
import json
from pathlib import Path
import shutil

from PIL import Image, ImageDraw, ImageFont, ImageOps


ROOT = Path(__file__).resolve().parents[2]
DOC = ROOT / "docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01"
ART = ROOT / "assets/source-art/first-sector-review-01/u01-r01"
DETAIL_ART = ROOT / "assets/source-art/first-sector-review-01/u01-r02"
TOP_ART = ROOT / "assets/source-art/first-sector-review-01/u01-r03"
ILLUSTRATION = ROOT / "assets/source-art/first-sector-review-01/u01-scene-r01/SCENE-U01-04-r01.png"
PROJECTION = ROOT / "assets/source-art/first-sector-review-01/u01-scene-r03/SCENE-U01-06-r01.png"
OUT = ROOT / "artifacts/first-sector-review-01"
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--revision", choices=("r01", "r02", "r03"), default="r03")
REV = parser.parse_args().revision
DETAIL = REV != "r01"
TOPDOWN = REV == "r03"
spec = json.loads((DOC / f"u01-layout-{REV}.json").read_text(encoding="utf-8"))
W, H = spec["size"]
CELL = spec["cell_px"]
assert CELL == 64 and spec["status"] == "proposal_not_integrated"
ART.mkdir(parents=True, exist_ok=True)
OUT.mkdir(parents=True, exist_ok=True)


def dump(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cells(rect):
    x, y, w, h = rect
    return {(a, b) for a in range(x, x + w) for b in range(y, y + h)}


def path(start, goal, allowed):
    pending, previous = deque([start]), {start: None}
    while pending:
        point = pending.popleft()
        if point == goal:
            result = []
            while point is not None:
                result.append(point)
                point = previous[point]
            return result[::-1]
        for dx, dy in ((0, -1), (1, 0), (0, 1), (-1, 0)):
            neighbor = (point[0] + dx, point[1] + dy)
            if neighbor in allowed and neighbor not in previous:
                previous[neighbor] = point
                pending.append(neighbor)
    return []


def reachable(start, allowed):
    pending, visited = deque([start]), {start}
    while pending:
        x, y = pending.popleft()
        for q in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
            if q in allowed and q not in visited:
                visited.add(q)
                pending.append(q)
    return visited


names = [
    "Asphalte", "Asphalte fissuré", "Trottoir", "Trottoir abîmé",
    "Marquage horizontal", "Marquage vertical", "Bordure horizontale",
    "Bordure verticale", "Façade nord", "Façade sud", "Façade ouest",
    "Façade est", "Bloc de barricade", "Débris de béton", "Grille de drainage",
    "Seuil ouvert",
]
source = Image.open(ART / "DEC-U01-atlas-source-r01.png").convert("RGBA")
assert source.size == (1295, 1214), "Unreviewed source atlas dimensions"
# Cell boxes exclude the black separators produced by the image model.
columns = [(3, 320), (329, 643), (652, 966), (975, 1292)]
rows = [(3, 297), (304, 599), (607, 887), (895, 1211)]
tiles, manifest = {}, []
for index, name in enumerate(names, 1):
    left, right = columns[(index - 1) % 4]
    top, bottom = rows[(index - 1) // 4]
    tile = source.crop((left, top, right, bottom)).resize((64, 64), Image.Resampling.LANCZOS)
    file = ART / f"DEC-U01-{index:02}-r01.png"
    tile.save(file)
    tiles[index] = tile
    manifest.append({"id": f"DEC-U01-{index:02}-r01", "name": name,
                     "file": file.relative_to(ROOT).as_posix(), "size": list(tile.size),
                     "crop_box": [left, top, right, bottom], "sha256": digest(file),
                     "status": "candidate_not_accepted"})

top_source = Image.open(TOP_ART / "DEC-U01-topdown-source-r01.png").convert("RGBA") if TOPDOWN else None
if TOPDOWN:
    assert top_source.size == (1536, 1024)
car_source = top_source.crop((927, 665, 1480, 920)) if TOPDOWN else Image.open(ART / "DEC-U01-car-source-r02.png").convert("RGBA")
assert car_source.getchannel("A").getextrema()[0] == 0, "Car source lost transparency"
car_size = (128, 64) if DETAIL else (192, 128)
car_revision = "r04" if TOPDOWN else "r03" if DETAIL else "r02"
car = Image.new("RGBA", car_size)
contained = ImageOps.contain(car_source, car.size, Image.Resampling.LANCZOS)
car.alpha_composite(contained, ((car.width - contained.width) // 2, (car.height - contained.height) // 2))
car_file = (TOP_ART if TOPDOWN else DETAIL_ART if DETAIL else ART) / f"DEC-U01-17-{car_revision}.png"
car.save(car_file)
manifest.append({"id": f"DEC-U01-17-{car_revision}", "name": "Voiture abandonnée",
                 "file": car_file.relative_to(ROOT).as_posix(), "size": list(car_size),
                 "source": "DEC-U01-topdown-source-r01.png" if TOPDOWN else "DEC-U01-car-source-r02.png", "footprint_proposal": [v // CELL for v in car_size],
                 "sha256": digest(car_file), "status": "candidate_not_accepted"})
facades = {}
if REV == "r02":
    buildings_source = Image.open(DETAIL_ART / "DEC-U01-buildings-source-r01.png").convert("RGBA")
    assert buildings_source.size == (1266, 1243)
    # Native door scale is checked against the approved 65px-high actor.
    # 256px facade canvases keep the approximately 70px human doorways readable.
    parts = [(18, "Façade d’habitation", (25, 15, 626, 588), (256, 256)),
             (19, "Garage fermé", (650, 220, 1245, 588), (256, 192)),
             (20, "Façade condamnée", (22, 608, 629, 1183), (256, 256)),
             (21, "Surface de toiture", (690, 645, 1216, 1168), (64, 64))]
    for index, name, box, size in parts:
        cropped = buildings_source.crop(box)
        sprite = Image.new("RGBA", size)
        fitted = ImageOps.contain(cropped, size, Image.Resampling.LANCZOS)
        sprite.alpha_composite(fitted, ((size[0] - fitted.width) // 2, size[1] - fitted.height))
        facades[index] = sprite
        file = DETAIL_ART / f"DEC-U01-{index:02}-r01.png"
        sprite.save(file)
        manifest.append({"id": f"DEC-U01-{index:02}-r01", "name": name,
                         "file": file.relative_to(ROOT).as_posix(), "size": list(size),
                         "crop_box": list(box), "sha256": digest(file),
                         "status": "candidate_not_accepted"})

if TOPDOWN:
    for index, name, box, size in [
        (18, "Immeuble vu de dessus", (68, 30, 856, 512), (384, 256)),
        (19, "Garage vu de dessus", (903, 111, 1476, 546), (256, 192)),
        (20, "Bâtiment à toiture effondrée", (68, 534, 857, 986), (384, 256)),
    ]:
        sprite = Image.new("RGBA", size)
        fitted = ImageOps.contain(top_source.crop(box), size, Image.Resampling.LANCZOS)
        sprite.alpha_composite(fitted, ((size[0] - fitted.width) // 2, size[1] - fitted.height))
        facades[index] = sprite
        file = TOP_ART / f"DEC-U01-{index:02}-r02.png"
        sprite.save(file)
        manifest.append({"id": f"DEC-U01-{index:02}-r02", "name": name,
                         "file": file.relative_to(ROOT).as_posix(), "size": list(size),
                         "crop_box": list(box), "sha256": digest(file),
                         "projection": "overhead", "status": "candidate_not_accepted"})

flame_atlas = Image.open(ROOT / "assets/animations/agent/skill-animation-review-06/flame-atlas.png").convert("RGBA")
assert flame_atlas.size == (1024, 128)


def burning_car(frame):
    result = car.copy()
    for offset, size, phase in [((91, 0), (32, 48), 0), ((72, 8), (28, 42), 3), ((104, 20), (20, 34), 5)]:
        image_index = (frame + phase) % 8
        flame = flame_atlas.crop((image_index * 128, 0, (image_index + 1) * 128, 128))
        result.alpha_composite(flame.resize(size, Image.Resampling.LANCZOS), offset)
    assert result.size == (128, 64)
    return result


dump(DOC / f"u01-decor-manifest-{REV}.json", {
    "review_id": "DEC-U01-03-r01" if TOPDOWN else "DEC-U01-02-r01" if DETAIL else "DEC-U01-01-r01", "tool": "built-in image_gen",
    "conditioning": f"Crop atlas cells and resize to 64px; contain car in {car.width}x{car.height} without stretching. Original sources preserved.",
    "source_atlas_sha256": digest(ART / "DEC-U01-atlas-source-r01.png"),
    "source_car_sha256": digest(TOP_ART / "DEC-U01-topdown-source-r01.png") if TOPDOWN else digest(ART / "DEC-U01-car-source-r02.png"),
    "source_buildings_sha256": digest(TOP_ART / "DEC-U01-topdown-source-r01.png") if TOPDOWN else digest(DETAIL_ART / "DEC-U01-buildings-source-r01.png") if DETAIL else None,
    "items": manifest,
})

road = cells(spec["road_rect"])
pavement = set().union(*(cells(r) for r in spec["sidewalk_rects"] + spec["plaza_rects"]))
access = cells(spec["refuge_access_rect"])
portals = set().union(*(cells(p["opening"]) for p in spec["portals"]))
floor = road | pavement | access | portals
obstacles = set().union(*(cells(p["rect"]) for p in spec["props"]))
assert obstacles <= floor
walkable = floor - obstacles
start = tuple(spec["start"])
assert reachable(start, walkable) == walkable
routes = {p["id"]: path(start, tuple(p["at"]), walkable) for p in spec["portals"]}
assert all(routes.values())
assert not set().union(*(cells(b["rect"]) for b in spec["buildings"])) & floor
wide_centres = {p for p in walkable if cells((p[0] - 1, p[1] - 1, 3, 3)) <= walkable}
assert path((3, 15), (36, 15), wide_centres), "No continuous 3-cell-wide street passage"
assert path((18, 2), (18, 12), wide_centres), "Refuge approach lost width"
logical_rows = ["".join("." if (x, y) in walkable else "#" for x in range(W)) for y in range(H)]
(DOC / f"u01-layout-{REV}.txt").write_text("\n".join(logical_rows) + "\n", encoding="utf-8")

# Draw only the material kit at native size. Dark building volumes are schematic.
scene = Image.new("RGBA", (W * CELL, H * CELL), "#10191e")
draw = ImageDraw.Draw(scene)
for building in spec["buildings"]:
    if TOPDOWN:
        continue
    x, y, w, h = building["rect"]
    draw.rectangle((x * CELL, y * CELL, (x + w) * CELL - 1, (y + h) * CELL - 1), fill="#1d292e")
    for n in range(1, w):
        draw.line(((x + n) * CELL, y * CELL, (x + n) * CELL, (y + h) * CELL), fill="#233137")
    for n in range(1, h):
        draw.line((x * CELL, (y + n) * CELL, (x + w) * CELL, (y + n) * CELL), fill="#233137")
    if REV == "r02":
        for px, py in sorted(cells(building["rect"])):
            scene.alpha_composite(facades[21], (px * CELL, py * CELL))

for x, y in sorted(floor):
    paved = (x, y) in pavement | access
    # Stable local variation; never consumes or touches the game's RNG.
    noise = ((x * 73856093) ^ (y * 19349663) ^ 0x9E3779B9) & 0xFFFFFFFF
    noise = ((noise ^ (noise >> 16)) * 0x45D9F3B) & 0xFFFFFFFF
    noise ^= noise >> 16
    index = (4 if noise % 11 == 0 else 3) if paved else (2 if noise % 13 == 0 else 1)
    if not paved and y == 14 and 4 <= x <= 36 and x % 6 < 3:
        index = 5
    if (x, y) in {(7, 18), (28, 11), (18, 7)}:
        index = 15
    texture = tiles[index]
    if index in (1, 2):
        # Floor UV variations only; never rotate facades, shadows or props.
        if noise & 1:
            texture = texture.transpose(Image.Transpose.FLIP_LEFT_RIGHT)
        if noise & 2:
            texture = texture.transpose(Image.Transpose.FLIP_TOP_BOTTOM)
    scene.alpha_composite(texture, (x * CELL, y * CELL))

# Curb strips follow the sidewalk boundary without filling the whole road cell.
curb = tiles[7].crop((0, 24, 64, 37))
for x in range(1, 39):
    scene.alpha_composite(curb, (x * CELL, 11 * CELL - 7))
    scene.alpha_composite(curb, (x * CELL, 19 * CELL - 7))

wall_n = tiles[9].crop((0, 0, 64, 23))
wall_s = tiles[10].crop((0, 40, 64, 64))
wall_w = tiles[11].crop((0, 0, 23, 64))
wall_e = tiles[12].crop((41, 0, 64, 64))
for x, y in sorted(floor):
    if (x, y - 1) not in floor and y > 0:
        scene.alpha_composite(wall_n, (x * CELL, y * CELL - wall_n.height))
    if (x, y + 1) not in floor and y < H - 1:
        scene.alpha_composite(wall_s, (x * CELL, (y + 1) * CELL))
    if (x - 1, y) not in floor and x > 0:
        scene.alpha_composite(wall_w, (x * CELL - wall_w.width, y * CELL))
    if (x + 1, y) not in floor and x < W - 1:
        scene.alpha_composite(wall_e, ((x + 1) * CELL, y * CELL))

if REV == "r02":
    by_id = {b["id"]: b for b in spec["buildings"]}
    for placement in spec["facade_placements"]:
        building = by_id[placement["building"]]
        bx, by, bw, bh = building["rect"]
        sprite = facades[placement["id"]]
        assert bx <= placement["x"] and placement["x"] + sprite.width // CELL <= bx + bw
        scene.alpha_composite(sprite, (placement["x"] * CELL, (by + bh) * CELL - sprite.height))

if TOPDOWN:
    groups = {b["id"]: cells(b["rect"]) for b in spec["buildings"]}
    occupied_buildings = set()
    for instance in spec["building_instances"]:
        footprint = cells(instance["rect"])
        assert footprint <= groups[instance["block"]]
        assert not footprint & (floor | occupied_buildings)
        occupied_buildings |= footprint
        sprite = facades[instance["id"]]
        x, y, width, height = instance["rect"]
        assert sprite.size == (width * CELL, height * CELL)
        scene.alpha_composite(sprite, (x * CELL, y * CELL))

for prop in spec["props"]:
    x, y, w, h = prop["rect"]
    if prop["tile"] == 17:
        assert (w * CELL, h * CELL) == car_size
        scene.alpha_composite(burning_car(0) if DETAIL else car, (x * CELL, y * CELL))
    else:
        for px, py in sorted(cells(prop["rect"])):
            scene.alpha_composite(tiles[prop["tile"]], (px * CELL, py * CELL))

player_path = ROOT / spec["scale_actor"]["file"]
player = Image.open(player_path).convert("RGBA")
assert player.size == (128, 128)
assert digest(player_path) == "39fc854b865dac6a20d4dabc8b7a06d942aa7b99cb4b0a019022568594836c24"
scene.alpha_composite(player, (start[0] * CELL - 32, start[1] * CELL - 32))
scene.convert("RGB").save(OUT / f"u01-map-native-{REV}.png")
crop_y = (0, 17) if TOPDOWN else (4, 18) if DETAIL else (7, 21)
scene.crop((0, crop_y[0] * CELL, 22 * CELL, crop_y[1] * CELL)).convert("RGB").save(OUT / f"u01-arrival-native-{REV}.png")
scene.crop((14 * CELL, crop_y[0] * CELL, 36 * CELL, crop_y[1] * CELL)).convert("RGB").save(OUT / f"u01-junction-native-{REV}.png")

fonts = Path("C:/Windows/Fonts")
def font(size, bold=False):
    return ImageFont.truetype(str(fonts / ("segoeuib.ttf" if bold else "segoeui.ttf")), size)


# Diagram has its own scale. No player or prop artwork is shrunk into this view.
S, ox, oy = 22, 28, 120
overview = Image.new("RGB", (1280, 850), "#10191e")
d = ImageDraw.Draw(overview)
d.text((28, 20), "U01 · les abords de la prison", font=font(30, True), fill="#e5dfcd")
d.text((29, 66), f"MAP-U01-01-{REV} · plan candidat · 40 × 28 cases", font=font(19), fill="#aebdc0")
for x, y in floor:
    color = "#6c7065" if (x, y) in pavement else "#35434a"
    if (x, y) in access:
        color = "#41675b"
    d.rectangle((ox + x * S, oy + y * S, ox + (x + 1) * S - 2, oy + (y + 1) * S - 2), fill=color)
for b in spec["buildings"]:
    x, y, w, h = b["rect"]
    box = (ox + x * S, oy + y * S, ox + (x + w) * S - 1, oy + (y + h) * S - 1)
    d.rectangle(box, fill="#243038", outline="#526069", width=2)
    d.text(((box[0] + box[2]) / 2, (box[1] + box[3]) / 2), b["id"], anchor="mm", font=font(21, True), fill="#7d9098")
for p in spec["props"]:
    x, y, w, h = p["rect"]
    d.rectangle((ox + x * S, oy + y * S, ox + (x + w) * S - 2, oy + (y + h) * S - 2), fill="#a4875e")
for a, b in zip(routes["U02"], routes["U02"][1:]):
    d.line((ox + (a[0] + .5) * S, oy + (a[1] + .5) * S, ox + (b[0] + .5) * S, oy + (b[1] + .5) * S), fill="#e7c47d", width=3)
for p, color in zip(spec["portals"], ("#9bd1b0", "#9bd1b0", "#91c5d8")):
    x, y = p["at"]
    d.ellipse((ox + x * S + 4, oy + y * S + 4, ox + x * S + S - 5, oy + y * S + S - 5), fill=color)
    label_pos = {"P00": (ox + 4, oy + 12 * S), "F00": (ox + 17 * S, oy - 27), "U02": (ox + 36 * S, oy + 10 * S)}[p["id"]]
    d.text(label_pos, p["id"], font=font(17, True), fill=color)
legend = ["P00  Retour au camp", "F00  Vers le refuge", "U02  Suite résidentielle", "", "Chaussée : 8 cases", "Trottoirs : 2 cases", "Accès refuge : 4 cases", "", "Ocre : obstacles proposés", "B01–04 : bâtiments", "Voiture : 2 × 1 cases" if REV == "r02" else "encore schématiques", "", f"Trajet montré : {len(routes['U02']) - 1} pas", "sur cette grille de revue"]
for i, line in enumerate(legend):
    d.text((934, 156 + i * 30), line, font=font(18), fill="#b9c8ca")
d.text((28, 768), "Façades et décors candidats · passages ouverts · aucun ennemi, quête ou boss placé", font=font(18), fill="#c8d0cd")
d.text((28, 800), "Le plan et son rendu restent à examiner avant leur raccordement au jeu.", font=font(18), fill="#9caeb4")
overview.save(OUT / f"u01-layout-overview-{REV}.png")

board = Image.new("RGB", (1280, 790), "#10191e")
d = ImageDraw.Draw(board)
d.text((24, 16), "DEC-U01 · décors urbains candidats", font=font(27, True), fill="#dec898")
d.text((25, 57), f"01–16 : 64 × 64 px · 17 : {car.width} × {car.height} px · joueur conservé à son échelle", font=font(17), fill="#aebdc0")
for i, name in enumerate(names):
    x, y = 24 + (i % 4) * 310, 98 + (i // 4) * 110
    d.rectangle((x, y, x + 298, y + 100), fill="#1a282e", outline="#3d515a")
    board.paste(tiles[i + 1], (x + 12, y + 18), tiles[i + 1])
    d.text((x + 88, y + 17), f"DEC-U01-{i + 1:02}", font=font(17, True), fill="#dec898")
    d.text((x + 88, y + 47), name, font=font(16), fill="#d5dddc")
d.line((24, 557, 1250, 557), fill="#3d515a")
d.text((24, 573), f"DEC-U01-17 {car_revision} · voiture abandonnée · emprise : {car.width // CELL} × {car.height // CELL} cases", font=font(20, True), fill="#dec898")
for x in range(4):
    for y in range(2):
        board.paste(tiles[1], (24 + x * 64, 620 + y * 64))
board.paste(burning_car(0) if DETAIL else car, (24, 652 if DETAIL else 620), burning_car(0) if DETAIL else car)
board.paste(player, (240, 618), player)
d.text((400, 634), "Voiture et joueur à la taille prévue", font=font(21, True), fill="#d5dddc")
d.text((400, 674), "Sources intactes · dessins non intégrés au jeu", font=font(18), fill="#aebdc0")
board.save(OUT / f"u01-decor-board-native-{REV}.png")

if REV == "r02":
    buildings_board = Image.new("RGB", (1280, 660), "#10191e")
    d = ImageDraw.Draw(buildings_board)
    d.text((24, 18), "Bâtiments urbains · candidats 18–21 · taille native", font=font(27, True), fill="#dec898")
    d.text((25, 60), "Façades de 4 cases de large · joueur inchangé · portes humaines d’environ 70 px", font=font(17), fill="#aebdc0")
    for column, (index, label) in enumerate([(18, "Habitation"), (19, "Garage fermé"), (20, "Habitation condamnée")]):
        x = 24 + column * 416
        d.rectangle((x, 100, x + 392, 520), fill="#1a282e", outline="#3d515a")
        d.text((x + 14, 114), f"DEC-U01-{index:02}-r01", font=font(20, True), fill="#dec898")
        d.text((x + 14, 148), label, font=font(18), fill="#c6d2d3")
        for gx in range(6):
            buildings_board.paste(tiles[3], (x + 4 + gx * 64, 412))
        sprite = facades[index]
        buildings_board.paste(sprite, (x + 12, 430 - sprite.height), sprite)
        buildings_board.paste(player, (x + 250, 334), player)
        d.text((x + 14, 486), f"Canevas : {sprite.width} × {sprite.height} px", font=font(16), fill="#aebdc0")
    buildings_board.paste(facades[21], (24, 558), facades[21])
    d.text((110, 558), "DEC-U01-21-r01 · matériau de toiture · 64 × 64 px", font=font(19, True), fill="#dec898")
    d.text((110, 594), "Dessins proposés avant intégration · façades alignées sur leur pied", font=font(17), fill="#aebdc0")
    buildings_board.save(OUT / "u01-buildings-native-r01.png")

if TOPDOWN:
    buildings_board = Image.new("RGB", (1640, 540), "#10191e")
    d = ImageDraw.Draw(buildings_board)
    d.text((24, 18), "Bâtiments · vue de dessus · candidats 18–20 r02", font=font(29, True), fill="#dec898")
    d.text((25, 62), "Toitures dominantes · grille orthogonale de 64 px · joueur à sa taille habituelle", font=font(19), fill="#aebdc0")
    for column, (index, label) in enumerate([(18, "Immeuble"), (19, "Garage"), (20, "Toiture effondrée · accès condamné")]):
        x = 20 + column * 540
        d.rectangle((x, 106, x + 520, 520), fill="#1a282e", outline="#3d515a")
        d.text((x + 14, 119), f"DEC-U01-{index:02}-r02", font=font(21, True), fill="#dec898")
        d.text((x + 14, 155), label, font=font(18), fill="#c6d2d3")
        for gx in range(8):
            buildings_board.paste(tiles[3], (x + 4 + gx * CELL, 432))
        sprite = facades[index]
        buildings_board.paste(sprite, (x + 14, 448 - sprite.height), sprite)
        buildings_board.paste(player, (x + 374, 352), player)
        d.text((x + 14, 486), f"{sprite.width // CELL} × {sprite.height // CELL} cases · {sprite.width} × {sprite.height} px", font=font(17), fill="#aebdc0")
    buildings_board.save(OUT / "u01-buildings-native-r02.png")

if DETAIL:
    animation_frames = []
    for frame in range(8):
        preview = Image.new("RGB", (640, 352), "#10191e")
        d = ImageDraw.Draw(preview)
        d.text((24, 15), "Voiture en feu · 2 × 1 cases", font=font(26, True), fill="#dec898")
        d.text((25, 54), f"DEC-U01-17-{car_revision} · vue de dessus" if TOPDOWN else "DEC-U01-17-r03 / FX-U01-CAR-01-r01 · taille native", font=font(16), fill="#aebdc0")
        for px in range(9):
            for py in range(3):
                preview.paste(tiles[1], (32 + px * CELL, 96 + py * CELL))
        for px in range(10):
            d.line((32 + px * CELL, 96, 32 + px * CELL, 288), fill="#536166")
        for py in range(4):
            d.line((32, 96 + py * CELL, 608, 96 + py * CELL), fill="#536166")
        preview.paste(player, (128, 160), player)
        burning = burning_car(frame)
        preview.paste(burning, (352, 192), burning)
        d.text((24, 310), "Carrosserie et flammes contenues dans 128 × 64 px", font=font(18), fill="#c6d2d3")
        animation_frames.append(preview)
    animation_frames[0].save(OUT / f"u01-car-fire-native-{car_revision}.png")
    # Shared GIF palette keeps the static car, player and scenery stable.
    palette_source = Image.new("RGB", (640 * 8, 352))
    for index, frame in enumerate(animation_frames):
        palette_source.paste(frame, (index * 640, 0))
    palette = palette_source.quantize(colors=256, dither=Image.Dither.NONE)
    gif_frames = [frame.quantize(palette=palette, dither=Image.Dither.NONE) for frame in animation_frames]
    gif_frames[0].save(OUT / f"u01-car-fire-native-{car_revision}.gif", save_all=True,
                       append_images=gif_frames[1:], duration=[120, 130] * 4,
                       loop=0, optimize=False, disposal=2)

checks = {
    "review_id": spec["review_id"], "scope": "Fixed Python review grid only; not the game generator",
    "walkable_cells": len(walkable), "all_walkable_cells_connected": True,
    "portal_paths_steps": {key: len(value) - 1 for key, value in routes.items()},
    "continuous_three_cell_wide_street_path": True, "continuous_three_cell_wide_refuge_approach": True,
    "player_scale": 1, "player_source_sha256": digest(player_path),
    "native_cell_px": CELL, "native_map_size": list(scene.size),
    "obstacle_cells": len(obstacles), "game_or_map_editor_modified": False,
    "car_canvas_size": list(car_size), "car_max_cells": [v // CELL for v in car_size],
    "fire_effect": "Decorative proposal only, no combat rule" if DETAIL else None,
    "facade_source_scale": "Whole overhead buildings; door openings indicated in plan view" if TOPDOWN else "256px-wide native modules with approximately 70px doorways" if DETAIL else None,
    "limits": ["Building interiors not created by this facade review", "Collision/render interaction not exercised in the engine", "New art and concrete street layout are not yet accepted"],
    "layout_source_sha256": digest(DOC / f"u01-layout-{REV}.json"),
    "outputs": {name: digest(OUT / name) for name in [f"u01-map-native-{REV}.png", f"u01-arrival-native-{REV}.png", f"u01-junction-native-{REV}.png", f"u01-layout-overview-{REV}.png", f"u01-decor-board-native-{REV}.png"]},
}
if DETAIL:
    for name in ["u01-buildings-native-r02.png" if TOPDOWN else "u01-buildings-native-r01.png", f"u01-car-fire-native-{car_revision}.gif", f"u01-car-fire-native-{car_revision}.png"]:
        checks["outputs"][name] = digest(OUT / name)
dump(DOC / f"u01-layout-checks-{REV}.json", checks)
page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>U01 · abords de la prison</title>
<style>body{margin:0;background:#10191e;color:#d7dfdc;font:17px Segoe UI,sans-serif}main{padding:24px}h1{color:#dec898}h2{font-size:21px}a{color:#94cedd}.scroll{overflow:auto;border:1px solid #415159;padding:8px;margin:18px 0}img{display:block;max-width:none}p{max-width:1050px;line-height:1.6}</style>
<main><h1>U01 · les abords de la prison</h1><p>Agencement du chapitre accepté. Plan détaillé, nouveaux dessins et animation proposés séparément. Le joueur reste à sa taille prévue. Aucune nouvelle carte n’est activée dans le jeu.</p>
<nav><a href="#plan">Plan</a> · <a href="#arrivee">Arrivée à taille native</a> · <a href="#carrefour">Carrefour à taille native</a> · <a href="#decors">Décors numérotés</a> · <a href="#ensemble">Carte complète</a></nav>'''
if TOPDOWN and PROJECTION.exists():
    corrected = Image.open(PROJECTION).convert("RGBA")
    native = corrected.copy()
    native.alpha_composite(player, (384 - 32, 896 - 32))
    native.alpha_composite(car, (832, 448))
    native.convert("RGB").save(OUT / "u01-projection-native-r06.png")
    home = corrected.copy()
    home.alpha_composite(player, (320 - 32, 192 - 32))
    home.crop((0, 0, 640, 448)).convert("RGB").save(OUT / "u01-projection-home-native-r06.png")
    accepted = (DOC / "visual-acceptance-r06.json").exists()
    status = "Perspective acceptée ; intégration en génération 155. Le camp reste la référence stricte des dessins suivants." if accepted else "Perspective corrigée proposée avant activation."
    page += f'<h2 id="projection">SCENE-U01-06-r01 · perspective recalée sur le camp</h2><p>{status} Axes parallèles, portail sud ouvert, première maison à gauche visitable. Personnage à sa taille réelle et voiture de 128 × 64 px.</p><div class="scroll"><img src="u01-projection-native-r06.png" alt="Perspective corrigée avec le personnage et la voiture à taille native"></div><h2>Intérieur de la première maison · taille native</h2><div class="scroll"><img src="u01-projection-home-native-r06.png" alt="Maison de gauche en coupe avec le personnage à son échelle réelle"></div>'
    for name, label in [("gate", "Arrivée au portail sud"), ("road", "Rue"), ("home-entry", "Passage vers la maison"), ("home", "Dans la maison"), ("north", "Rue au nord")]:
        file = f"engine-r06/street-{name}.png"
        if (OUT / file).exists():
            page += f'<h2>{label} · capture réelle du jeu</h2><div class="scroll"><img src="{file}" alt="{label} dans le moteur avec interface et visibilité réelles"></div>'
    page += '<h2>Prototypes antérieurs · hors de la revue courante</h2>'
elif TOPDOWN and ILLUSTRATION.exists():
    shutil.copyfile(ILLUSTRATION, OUT / "u01-street-illustration-r01.png")
    page += '<h2 id="illustration">SCENE-U01-04-r01 · illustration d’intention</h2><p>Nouvel agencement asymétrique : volumes, toitures, dommages et mobilier varient selon les lieux. Cette illustration propose l’ambiance et les futurs décors ; les assemblages à taille native ci-dessous appartiennent au prototype précédent.</p><div class="scroll"><img src="u01-street-illustration-r01.png" alt="Rue futuriste dévastée vue de dessus, aux bâtiments et aux décors variés"></div>'
if DETAIL:
    building_board = "u01-buildings-native-r02.png" if TOPDOWN else "u01-buildings-native-r01.png"
    page += f'<h2>Bâtiments candidats · vue de dessus</h2><div class="scroll"><img src="{building_board}" alt="Bâtiments candidats à la même échelle que le joueur"></div><h2>Voiture de deux cases, flammes comprises</h2><div class="scroll"><img src="u01-car-fire-native-{car_revision}.gif" alt="Voiture animée à sa taille native"></div>'
for anchor, title, file in [("plan", "Plan de circulation", f"u01-layout-overview-{REV}.png"), ("arrivee", "Rue à l’arrivée du camp · 64 px par case", f"u01-arrival-native-{REV}.png"), ("carrefour", "Jonction et passage autour des obstacles · 64 px par case", f"u01-junction-native-{REV}.png"), ("decors", "Décors candidats à taille native", f"u01-decor-board-native-{REV}.png"), ("ensemble", "Assemblage complet · défilement à taille native", f"u01-map-native-{REV}.png")]:
    page += f'<h2 id="{anchor}">{html.escape(title)}</h2><div class="scroll"><img src="{file}" alt="{html.escape(title)}"></div>'
page += '</main></html>'
if TOPDOWN and (DOC / "visual-acceptance-r06.json").exists():
    page = page.replace("Agencement du chapitre accepté. Plan détaillé, nouveaux dessins et animation proposés séparément. Le joueur reste à sa taille prévue. Aucune nouvelle carte n’est activée dans le jeu.", "Perspective de la rue acceptée sur la référence du camp. Portail sud et première maison à gauche accessibles dans les nouvelles parties de génération 155. Les anciennes parties conservent leur monde. Les quartiers suivants restent à construire.")
(OUT / "index.html").write_text(page, encoding="utf-8")
print(json.dumps(checks, ensure_ascii=False))
