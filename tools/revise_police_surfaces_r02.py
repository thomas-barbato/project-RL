"""Requested room surfaces only; police layouts, objects and seeded choices stay fixed."""
from copy import deepcopy
import hashlib
from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[1]
DIR = ROOT / "assets/maps/north-city-01"


def main():
    master = DIR / "review.json"
    review = json.loads(master.read_text(encoding="utf-8"))
    if review.get("police_surface_revision"):
        raise SystemExit("Room surfaces already reviewed in r02. Target individual rooms for later edits.")
    report = []
    for zone in review["zones"][1:]:
        contexts = zone.get("variants") or [zone]
        for context in contexts:
            path = ROOT / context["file"]
            doc = json.loads(path.read_text(encoding="utf-8"))
            before = deepcopy(doc)
            width = doc["width"]
            rooms = []
            def room(key, name, rect, material):
                l, t, r, b = rect
                for y in range(t, b + 1):
                    for x in range(l, r + 1):
                        doc["floors"][y * width + x] = material
                rooms.append(dict(id=f"{zone['id']}:room:{context.get('number', 0)}:{key}",
                                  building=zone["id"], name=name, rect=list(rect), material=material))
            room("base", "Sol de service", (1, 1, 34, 26), 26)
            if zone["id"] == "review:police_upper":
                for key, name, rect, material in [
                        ("office-a", "Bureaux A", (1, 1, 10, 11), 25),
                        ("office-b", "Bureaux B", (12, 1, 22, 11), 25),
                        ("briefing", "Salle de réunion", (24, 1, 34, 11), 22),
                        ("archive", "Archives et dossiers", (1, 16, 10, 26), 26),
                        ("office-c", "Bureaux C", (12, 16, 22, 26), 25),
                        ("stairs", "Palier", (24, 16, 34, 26), 23),
                        ("hall", "Circulation", (1, 12, 34, 15), 23)]:
                    room(key, name, rect, material)
            elif context["number"] == 1:
                room("hall", "Couloir des cellules et réserves", (25, 1, 34, 26), 23)
            elif context["number"] == 2:
                room("hall", "Couloir central", (15, 1, 20, 26), 23)
                room("landing", "Palier et accès", (22, 21, 34, 25), 23)
            else:
                room("hall", "Distribution latérale", (26, 1, 30, 26), 23)
                room("landing", "Palier", (19, 21, 34, 25), 23)
            context["rooms"] = rooms[1:]
            geometry_before = {k: v for k, v in before.items() if k != "floors"}
            geometry_after = {k: v for k, v in doc.items() if k != "floors"}
            assert geometry_before == geometry_after
            path.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
            report.append(dict(file=context["file"], geometry_and_props_unchanged=True,
                               changed_floor_cells=sum(a != b for a, b in zip(before["floors"], doc["floors"])),
                               materials=sorted(set(doc["floors"])),
                               geometry_sha256=hashlib.sha256(json.dumps(geometry_after, sort_keys=True).encode()).hexdigest()))
    review["police_surface_revision"] = dict(review="CITY-NORTH01-r02", scope="Room materials only; walls, objects, portals, variants and seeds preserved")
    master.write_text(json.dumps(review, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    destination = ROOT / "docs/reviews/north-city-01/SOLS-r02.json"
    destination.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Updated room materials on {len(report)} police plans; all geometry and object properties preserved.")


if __name__ == "__main__":
    main()
