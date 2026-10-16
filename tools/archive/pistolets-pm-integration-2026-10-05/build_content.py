"""Author native JSON5 content using the approved art/model plan."""
from pathlib import Path
import json
ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
plan = json.loads((HERE / "modeles.json").read_text(encoding="utf-8"))
def save(path, value):
    (ROOT / path).write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
locale = {"locale": "fr", "entries": {}}
bases = []
for family in plan["families"]:
    description = f'equipment.{family["family"]}.description'
    locale["entries"][description] = family["description"]
    for tier, model in enumerate(family["models"], 1):
        key = f'equipment.{model["id"]}.name'
        locale["entries"][key] = model["name"]
        weapon = {"id": "core:" + model["id"], "name_key": key,
                  "description_key": description, "mass_grams": family["mass_grams"],
                  "attack": {"range": family["range"], "distance_metric": "euclidean",
                             "requires_line_of_sight": True, "accuracy_modifier": family["accuracy"],
                             "damage": {"amount": model["damage"], "damage_type": "piercing", "penetration": 0}},
                  "supply": {"kind": "matter", "amount": family["matter"]}}
        if family["shots"] > 1:
            weapon["burst"] = {"shots": family["shots"]}
        save(f'content/core/weapons/{model["id"]}.json5', weapon)
        save(f'content/core/visuals/{model["id"]}.json5', {
            "id": weapon["id"], "terminal": {
                "frames": ["ballistic"] * family["shots"], "color": [231, 217, 173, 255],
                "accent_colors": [[111, 123, 130, 255], [255, 244, 208, 255]],
                "frame_millis": 35, "step_millis": 25, "linger_millis": 12}})
        bases.append({"item": weapon["id"], "family": "core:" + family["family"],
            "tier": tier, "grammar": "ms", "form": "humanoid_equipment",
            "sources": ["humanoid_site"], "minimum_depth": tier - 1,
            "maximum_depth": tier - 1 if tier < 6 else None,
            "effects": [{"effect": "core:affix_braise", "minimum_tier": 1, "weight": 20},
                        {"effect": "core:affix_decharge", "minimum_tier": 1, "weight": 8},
                        {"effect": "core:affix_saignement", "minimum_tier": 1, "weight": 16},
                        {"effect": "core:affix_poison", "minimum_tier": 1, "weight": 10}]})
stats = [("power", 100), ("coordination", 100), ("resilience", 100), ("perception", 80),
         ("processing", 80), ("accuracy", 70), ("armor_penetration", 35), ("vitality", 100),
         ("energy_reserve", 70), ("heat_dissipation", 35)]
save("content/core/equipment_loot/one_hand_firearms.json5", {
    "id": "core:one_hand_firearms", "stats": [{"affix": "affix:" + key, "weight": weight} for key, weight in stats],
    "bases": bases})
save("content/core/locales/fr_one_hand_firearms.json5", locale)
print("12 one-hand firearms + loot families, French text and native attack cues written")
