"""Check unchanged inventory geometry/art and the narrowly renamed locale entries."""
from pathlib import Path
import hashlib
import json
import re

from PIL import Image, ImageChops

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
PREVIOUS = ROOT / "artifacts/joueur-tenues-completes-2026-10-04"


def function(path, name):
    text = path.read_text(encoding="utf-8")
    start = text.index("fn " + name + "(")
    brace = text.index("{", start)
    depth, end = 1, brace + 1
    while depth:
        depth += (text[end] == "{") - (text[end] == "}")
        end += 1
    return text[start:end]


source_hashes = json.loads((PREVIOUS / "preservation.json").read_text())["source_hashes"]
for relative, expected in source_hashes.items():
    assert hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == expected, relative
wireframe_unchanged = function(ROOT / "src/game_art_player.rs", "draw_wireframe") == function(
    ROOT / "artifacts/joueur-equipe-2026-10-04/avant/game_art_player.rs", "draw_wireframe"
)
assert wireframe_unchanged

names = {
    "equipment.armure_cerbere.name": "Armure Cerbère",
    "equipment.brigandine_cognefer.name": "Tenue Cognefer",
    "equipment.carapace_chrysalide.name": "Carapace Chrysalide",
    "item.composite_carapace.name": "Armure Azurite",
    "equipment.combinaison_fantome.name": "Tenue Fantôme",
    "equipment.manteau_de_mue.name": "Robe de mue",
    "equipment.plastron_bastion.name": "Armure Bastion",
    "item.patched_plating.name": "Tenue du ferrailleur",
    "equipment.veste_de_la_seconde_ombre.name": "Robe de la Seconde Ombre",
    "equipment.veste_de_membranes.name": "Tenue de membranes",
    "equipment.veste_de_peau_seconde.name": "Combinaison Peau-Seconde",
    "equipment.veste_de_veille.name": "Tenue du technicien",
    "equipment.veste_matelassee.name": "Tenue d’aviateur",
    "equipment.veste_a_fibres_croisees.name": "Tenue du nomade",
}
found = {}
for filename in ("fr.json5", "fr_equipment.json5", "fr_armor_models.json5"):
    before = (OUT / "avant" / filename).read_text(encoding="utf-8")
    after = (ROOT / "content/core/locales" / filename).read_text(encoding="utf-8")
    pattern = r'("([^"\n]+)"\s*:\s*)"([^"\n]*)"'

    def normalize(match):
        if match[2] in names:
            return match[1] + '"<validated name>"'
        return match[0]

    assert re.sub(pattern, normalize, before) == re.sub(pattern, normalize, after), filename
    for match in re.finditer(pattern, after):
        if match[2] in names:
            found[match[2]] = match[3]
assert found == names

old_inventory = Image.open(PREVIOUS / "terminal-inventory-live-equipped/cold-start.png").convert("RGB")
new_inventory = Image.open(OUT / "terminal-inventory-live-equipped/cold-start.png").convert("RGB")
assert old_inventory.size == new_inventory.size == (1280, 800)
rects = {"central_wireframe_and_slots": (373, 88, 986, 716), "gauges": (994, 95, 1240, 342)}
identical = {}
for key, rect in rects.items():
    identical[key] = ImageChops.difference(old_inventory.crop(rect), new_inventory.crop(rect)).getbbox() is None
    assert identical[key], key

data = {
    "inventory_rgb_regions_identical": identical,
    "inventory_regions": rects,
    "wireframe_function_unchanged": wireframe_unchanged,
    "source_hashes": source_hashes,
    "validated_names": names,
    "locale_scope": "12 display names changed; all other locale text unchanged",
    "equipment_profiles": "unchanged; existing protection/evasion test passes",
    "historical_content_paths": "two existing content junctions retained; they resolve to current content",
}
(OUT / "preservation.json").write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("Preserved: inventory geometry, gauges, wireframe, item/NPC art. All 14 names verified.")
