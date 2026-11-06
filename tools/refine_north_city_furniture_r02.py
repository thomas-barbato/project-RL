"""Keep school desks in classrooms and staff rooms; retain every existing ID."""
from copy import deepcopy
import json
from pathlib import Path
from revise_north_city_r02 import reachable, inside, xy

ROOT = Path(__file__).resolve().parents[1]
DIR = ROOT / "assets/maps/north-city-01"


def main():
    path = DIR / "north_city.json"
    master = DIR / "review.json"
    doc = json.loads(path.read_text(encoding="utf-8"))
    review = json.loads(master.read_text(encoding="utf-8"))
    if review.get("furniture_refinement_r02"):
        raise SystemExit("Furniture r02 is already prepared; target individual IDs for later edits.")
    before = deepcopy(doc)
    zone = review["zones"][0]
    school = next(b for b in zone["buildings"] if b["id"] == "school")
    l, t, _, _ = school["rect"]
    school_props = [p for p in doc["props"] if inside(xy(p), school["rect"], True)]
    desks = [p for p in school_props if p["sprite"] == 12]
    chairs = [p for p in school_props if p["sprite"] == 1]
    student_desks = [(x, 1) for x in (1, 3, 5, 7, 10, 12, 14, 16)]
    student_desks += [(x, 4) for x in (1, 3, 5, 10, 14, 16)]
    desk_targets = student_desks + [(2, 12), (6, 12)]
    chair_targets = [(x, y + 1) for x, y in student_desks] + [(2, 13), (6, 13)]
    assert len(desks) == len(desk_targets) == len(chairs)
    for items, targets in ((desks, desk_targets), (chairs, chair_targets)):
        for prop, (x, y) in zip(items, targets):
            prop["pos"] = dict(x=l + x, y=t + y)
    table = next(p for p in school_props if p["sprite"] == 4)
    table["pos"] = dict(x=l + 4, y=t + 15)
    fire = next(b for b in zone["buildings"] if b["id"] == "fire_station")
    fl, ft, _, _ = fire["rect"]
    extra_locker = next(p for p in doc["props"] if p["sprite"] == 85 and xy(p) == (fl + 18, ft + 1))
    extra_locker["pos"] = dict(x=fl + 14, y=ft + 3)
    # Shapes, sprites, collisions and every unrelated item remain identical.
    edited = []
    for old, new in zip(before["props"], doc["props"]):
        if old == new: continue
        old_without_pos = {k: v for k, v in old.items() if k != "pos"}
        new_without_pos = {k: v for k, v in new.items() if k != "pos"}
        assert old_without_pos == new_without_pos
        edited.append(new["id"])
    assert {k: v for k, v in before.items() if k != "props"} == {k: v for k, v in doc.items() if k != "props"}
    seen, inaccessible = reachable(doc)
    assert not inaccessible, sorted(inaccessible)[:10]
    doc_json = json.dumps(doc, ensure_ascii=False, indent=2) + "\n"
    path.write_text(doc_json, encoding="utf-8")
    review["furniture_refinement_r02"] = dict(changed_ids=edited, scope="School desk/chair placement and one fire-station locker only", other_properties_preserved=True)
    master.write_text(json.dumps(review, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Repositioned {len(edited)} existing objects in their functional rooms; {len(seen)} walkable cells connected.")


if __name__ == "__main__":
    main()
