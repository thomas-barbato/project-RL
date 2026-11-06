"""Inspect or patch one stable CITY-NORTH01 element, preserving every sibling.

Examples:
  python tools/edit_north_city_review.py --zone north_city --kind scenery --list
  python tools/edit_north_city_review.py --zone north_city --kind scenery \
    --id north_city:scenery:001 --set '{"at_px":[2688,6400]}'
Changes are previewed unless --apply is supplied. Generators are never called.
"""
from copy import deepcopy
from pathlib import Path
import argparse
import difflib
import json
import os

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "assets/maps/north-city-01/review.json"
COLLECTIONS = dict(building="buildings", scenery="scenery", prop="props",
                   structure="structures", marker="markers")


def merge_properties(target, patch):
    """A change to details.offset_x must keep all other details intact."""
    for key, value in patch.items():
        if key == "id" or key not in target:
            raise ValueError(f"Propriété inconnue ou identifiant protégé : {key}")
        if isinstance(value, dict):
            if not isinstance(target[key], dict):
                raise ValueError(f"Type incompatible pour {key}")
            merge_properties(target[key], value)
        else:
            target[key] = value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--zone", required=True)
    parser.add_argument("--variant", type=int, choices=(1, 2, 3))
    parser.add_argument("--kind", choices=COLLECTIONS, required=True)
    parser.add_argument("--id")
    parser.add_argument("--list", action="store_true")
    parser.add_argument("--set", dest="patch")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    zones = [z for z in manifest["zones"] if z["id"].split(":", 1)[1] == args.zone]
    if len(zones) != 1:
        parser.error("Zone inconnue ou ambiguë.")
    zone = zones[0]
    if zone.get("variants") and args.variant is None:
        parser.error("Préciser --variant : chaque sous-sol se retouche séparément.")
    context = zone
    if args.variant is not None:
        variants = [v for v in zone.get("variants", []) if v["number"] == args.variant]
        if len(variants) != 1:
            parser.error("Variante absente de cette zone.")
        context = variants[0]
    collection = COLLECTIONS[args.kind]
    if args.kind in ("building", "scenery"):
        path, document = MANIFEST, manifest
        if args.kind == "building":
            context = zone
        items = context.get(collection, [])
    else:
        path = (ROOT / context["file"]).resolve()
        path.relative_to(ROOT / "assets/maps/north-city-01")
        document = json.loads(path.read_text(encoding="utf-8"))
        items = document[collection]
    if args.list:
        print(json.dumps(items, ensure_ascii=False, indent=2))
        return
    matches = [item for item in items if item.get("id") == args.id]
    if len(matches) != 1:
        parser.error("Identifiant absent ou ambigu. Utiliser --list.")
    if args.patch is None:
        print(json.dumps(matches[0], ensure_ascii=False, indent=2))
        return
    patch = json.loads(args.patch)
    if not isinstance(patch, dict) or "id" in patch or not patch:
        parser.error("Fournir un objet de propriétés sans modifier l'identifiant stable.")
    if any(key not in matches[0] for key in patch):
        parser.error("Propriété inconnue : la structure et les identifiants restent conservés.")
    original_document = deepcopy(document)
    target = matches[0]
    original_target = deepcopy(target)
    merge_properties(target, patch)
    # Prove that removing this one edit restores the entire input document.
    edited_target = deepcopy(target)
    target.clear()
    target.update(original_target)
    assert document == original_document, "Une donnée extérieure à la cible a changé."
    target.clear()
    target.update(edited_target)
    before = path.read_text(encoding="utf-8")
    after = json.dumps(document, ensure_ascii=False, indent=2) + "\n"
    changes = list(difflib.unified_diff(before.splitlines(), after.splitlines(),
                                      fromfile=str(path.relative_to(ROOT)),
                                      tofile=f"{path.relative_to(ROOT)} — {args.id}", lineterm=""))
    print("\n".join(changes) if changes else "Aucune modification.")
    if args.apply and changes:
        temporary = path.with_suffix(f".{os.getpid()}.tmp")
        try:
            temporary.write_text(after, encoding="utf-8")
            os.replace(temporary, path)
        finally:
            temporary.unlink(missing_ok=True)
        print("Une cible modifiée ; autres données conservées. Relancer les contrôles du prototype.")


if __name__ == "__main__":
    main()
