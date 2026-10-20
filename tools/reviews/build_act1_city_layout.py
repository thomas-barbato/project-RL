"""Render and check a chapter-layout proposal; never edits game or editor data."""

from collections import deque
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont


ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / "docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01"
SOURCE = REVIEW / "act1-city-layout-r01.json"
OUTPUT = ROOT / "artifacts/first-sector-review-01/act1-city-layout-r01.png"
data = json.loads(SOURCE.read_text(encoding="utf-8"))
nodes = {node["id"]: node for node in data["nodes"]}
assert len(nodes) == len(data["nodes"]), "Duplicate review identifier"
assert data["status"] in {"proposal_not_integrated", "layout_accepted_not_integrated"}
graph = {key: set() for key in nodes}
for edge in data["edges"]:
    graph[edge["from"]].add(edge["to"])
    graph[edge["to"]].add(edge["from"])


def reachable(start, removed=()):
    removed = set(removed)
    seen = set() if start in removed else {start}
    pending = deque(seen)
    while pending:
        for neighbor in graph[pending.popleft()] - seen - removed:
            seen.add(neighbor)
            pending.append(neighbor)
    return seen


optional = {key for key, node in nodes.items() if node["kind"] == "optional"}
assert reachable("P00") == set(nodes), "Unreachable review zone"
assert "X00" in reachable("P00", optional), "Optional area is mandatory"
assert "X00" not in reachable("P00", {"U04"}), "Transit can be bypassed"
assert graph["F00"] == {"U01"}, "Refuge must be off the combat route"
assert graph["D01"] == {"U02", "U03"}, "Courtyard loop lost a connection"
assert all(b in graph[a] for a, b in zip(data["main_route"], data["main_route"][1:]))

colors = {
    "accepted": (115, 190, 157),
    "refuge": (115, 190, 157),
    "main": (116, 179, 197),
    "optional": (187, 156, 105),
    "landmark": (205, 160, 121),
    "next": (152, 158, 176),
}
bg = (16, 23, 28)
ink = (227, 229, 220)
muted = (169, 180, 185)
image = Image.new("RGB", tuple(data["canvas"]), bg)
draw = ImageDraw.Draw(image)
fonts = Path("C:/Windows/Fonts")


def font(size, bold=False):
    return ImageFont.truetype(str(fonts / ("segoeuib.ttf" if bold else "segoeui.ttf")), size)


draw.text((52, 30), data["title"], font=font(33, True), fill=ink)
draw.text((53, 78), data["subtitle"], font=font(20), fill=muted)
draw.text((52, 118), data["review_id"], font=font(16), fill=colors["main"])
for edge in data["edges"]:
    points = [tuple(point) for point in edge["points"]]
    color = colors[edge["kind"]]
    draw.line(points, fill=color, width=3, joint="curve")
    x, y = points[-1]
    if edge["kind"] == "main":
        draw.polygon([(x, y - 3), (x - 7, y - 14), (x + 7, y - 14)], fill=color)
    else:
        draw.ellipse((x - 4, y - 4, x + 4, y + 4), fill=color)

for node in nodes.values():
    x, y, width, height = node["rect"]
    color = colors[node["kind"]]
    draw.rounded_rectangle((x, y, x + width, y + height), radius=7,
                           fill=(24, 34, 41), outline=color, width=2)
    draw.text((x + 15, y + 9), node["id"], font=font(15, True), fill=color)
    title_font = font(23, True)
    assert draw.textlength(node["title"], font=title_font) <= width - 30, node["id"]
    draw.text((x + 15, y + 30), node["title"], font=title_font, fill=ink)
    for index, line in enumerate(node["detail"]):
        detail_font = font(16)
        assert draw.textlength(line, font=detail_font) <= width - 30, node["id"]
        assert 61 + index * 19 + draw.textbbox((0, 0), line, font=detail_font)[3] <= height - 8, node["id"]
        draw.text((x + 15, y + 61 + index * 19), line, font=detail_font, fill=muted)

draw.line((52, 1002, 1388, 1002), fill=(54, 65, 72), width=1)
draw.text((52, 1017), "Bleu : parcours principal     Vert : camp / refuge     Ocre : détour facultatif",
          font=font(18), fill=ink)
draw.text((52, 1048), "Schéma de parcours uniquement · plans de rues, décors et dimensions encore à proposer",
          font=font(18), fill=muted)
OUTPUT.parent.mkdir(parents=True, exist_ok=True)
image.save(OUTPUT)
report = {
    "review_id": data["review_id"],
    "scope": "Proposal graph only; no engine geometry, collisions or campaign activation tested",
    "nodes": len(nodes),
    "edges": len(data["edges"]),
    "all_nodes_reachable": True,
    "optional_branches_not_required": True,
    "transit_cannot_be_bypassed": True,
    "refuge_off_combat_route": True,
    "courtyard_loop_two_connections": True,
    "source_sha256": hashlib.sha256(SOURCE.read_bytes()).hexdigest(),
    "output": str(OUTPUT.relative_to(ROOT)).replace("\\", "/"),
    "output_size": list(image.size),
    "output_sha256": hashlib.sha256(OUTPUT.read_bytes()).hexdigest(),
}
(REVIEW / "act1-city-layout-checks-r01.json").write_text(
    json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
)
print(json.dumps(report, ensure_ascii=False))
