"""Condition approved material/attachment layers for the active Rust renderer."""
from pathlib import Path
from copy import deepcopy
import hashlib
import json
import runpy
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
REVIEW = BASE / "weapon-silhouette-review-03"
OUT = ROOT / "assets/sprites/cyberpunk/agent-rigs"
AUDIT = ROOT / "artifacts/weapon-integration-2026-10-09"
H = runpy.run_path(str(BASE / "build_weapon_silhouette_review_01.py"), run_name="layer_helpers")
ART = ROOT / "assets/sprites/cyberpunk"


def refs(path):
    return {"path": path.relative_to(ROOT).as_posix(), "binary_sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def mask_for(image, mask, remap=None):
    result = Image.new("RGBA", image.size)
    for y in range(image.height):
        for x in range(image.width):
            if image.getpixel((x, y))[3]:
                label = mask.getpixel((x, y))[0]
                result.putpixel((x, y), ((remap or {}).get(label, label), 0, 0, 255))
    return result


def atlas(images, side):
    result = Image.new("RGBA", (16 * side, ((len(images) + 15) // 16) * side))
    for index, image in enumerate(images):
        H["copy_layer"](result, image, ((index % 16) * side, (index // 16) * side))
    return result


def main():
    assert not OUT.exists()
    OUT.mkdir()
    AUDIT.mkdir(exist_ok=True)
    bank = json.loads((REVIEW / "review-weapon-silhouettes-WEAPON-SIL01-03.json").read_text(encoding="utf-8"))
    frozen = {s["path"]: s["binary_sha256"] for s in bank["sources"]}
    for path, expected in frozen.items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, path
    asset = {name: Image.open(ART / name).convert("RGBA") for name in ["crawl-body-materials-32.png", "crawl-weapon-materials-48.png", "crawl-arms-48.png", "crawl-arm-materials-48.png", "crawl-fingers-48.png"]}
    profiles = [{**row, "body": Image.open(ROOT / row["body_reference"]["path"]).convert("RGBA")} for row in bank["classes"]]
    native_weapons = [Image.open(REVIEW / f"weapon-family-{f:02d}-native48.png").convert("RGBA") for f in range(18)]
    bodies, body_maps = [], []
    weapons, weapon_maps, sleeves, sleeve_maps, gloves = [], [], [], [], []
    matrices = {}
    ordered_sides = [("R", f) for f in range(18)] + [("L", f) for f in H["ONE"]]
    for profile in profiles:
        body = profile["body"]
        bodies.append(body)
        number = profile["number"]
        if number == 3:
            # Explicit palette-to-material assignments for this authored 12-color source.
            primary = {(0, 74, 201), (8, 36, 98), (21, 52, 103), (88, 129, 181)}
            secondary = {(234, 224, 198)}
            mask = Image.new("RGBA", (32, 32))
            for y in range(32):
                for x in range(32):
                    pixel = body.getpixel((x, y))
                    if pixel[3]:
                        mask.putpixel((x, y), (1 if pixel[:3] in primary else 2 if pixel[:3] in secondary else 0, 0, 0, 255))
        else:
            mask = mask_for(body, H["cell"](asset["crawl-body-materials-32.png"], profile["body_source"]["source_atlas_index"], 32))
        body_maps.append(mask)
        for side, family in ordered_sides:
            weapon = native_weapons[family]
            raw_weapon_map = mask_for(weapon, H["cell"](asset["crawl-weapon-materials-48.png"], family, 48))
            empty = Image.new("RGBA", (64, 64))
            if family in H["ONE"]:
                w, _ = H["one_hand"](profile, family, side, weapon, bank["families"][family]["hands"][0])
                wm, _ = H["one_hand"](profile, family, side, raw_weapon_map, bank["families"][family]["hands"][0])
                arm, am, front = empty.copy(), empty.copy(), empty.copy()
            else:
                arms = H["cell"](asset["crawl-arms-48.png"], family, 48)
                raw_arm_map = H["cell"](asset["crawl-arm-materials-48.png"], family, 48)
                fingers = H["cell"](asset["crawl-fingers-48.png"], family, 48)
                arm, w, front, record = H["two_hands"](profile, family, weapon, arms, raw_arm_map, fingers, bank["families"][family]["hands"])
                wm = empty.copy()
                H["copy_layer"](wm, raw_weapon_map, (8, 8))
                sleeve_label = 2 if number == 2 else 1
                raw_am = mask_for(arms, raw_arm_map, {1: 0, 2: sleeve_label})
                am = empty.copy()
                for limb in record["forearms"]:
                    part = Image.new("RGBA", (48, 48))
                    for y in range(48):
                        for x in range(48):
                            if (x < 25) == (limb["hand"] == "L"):
                                part.putpixel((x, y), raw_am.getpixel((x, y)))
                    posed, _ = H["forearm_warp"](part, limb["source_elbow"], limb["source_grip"], limb["target_elbow"], limb["target_grip"])
                    H["copy_layer"](am, posed)
                approved = next(row for row in bank["frames"] if row["class_number"] == number and row["left_family"] is None and row["right_family"] == family)
                for group in approved["qa"]["removed_affine_sleeve_sampling_fragments"]:
                    for point in group:
                        arm.putpixel(tuple(point), (0, 0, 0, 0))
                        am.putpixel(tuple(point), (0, 0, 0, 0))
            matrices[(number, side, family)] = (arm, w, front)
            weapons.append(w); weapon_maps.append(wm); sleeves.append(arm); sleeve_maps.append(am); gloves.append(front)
    # Verify this layer export reproduces every previously shown configuration.
    for row in bank["frames"]:
        profile = profiles[row["class_number"] - 1]
        body = profile["body"].copy()
        image = Image.new("RGBA", (64, 64))
        right, left = row["right_family"], row["left_family"]
        if right is not None and right not in H["ONE"]:
            for x0, y0, x1, y1 in profile["forearm_masks"]:
                for y in range(y0, y1):
                    for x in range(x0, x1):
                        body.putpixel((x, y), (0, 0, 0, 0))
            H["copy_layer"](image, body, (16, 16))
            for layer in matrices[(profile["number"], "R", right)]:
                H["copy_layer"](image, layer)
        else:
            for side, family in [("L", left), ("R", right)]:
                if family is not None:
                    H["copy_layer"](image, matrices[(profile["number"], side, family)][1])
            H["copy_layer"](image, body, (16, 16))
        expected = Image.open(ROOT / row["native_image"]["path"]).convert("RGBA")
        assert image.tobytes() == expected.tobytes(), row["review_id"]
    outputs = []
    for name, images, side in [("bodies-32.png", bodies, 32), ("body-materials-32.png", body_maps, 32),
                               ("weapons-64.png", weapons, 64), ("weapon-materials-64.png", weapon_maps, 64),
                               ("arms-64.png", sleeves, 64), ("arm-materials-64.png", sleeve_maps, 64), ("fingers-64.png", gloves, 64),
                               ("families-48.png", native_weapons, 48)]:
        image = atlas(images, side)
        path = OUT / name
        image.save(path)
        outputs.append({**refs(path), "dimensions": list(image.size), "frame_side": side, "frame_count": len(images)})
    manifest = {"version": 1, "review_id": "WEAPON-SIL01", "review_revision": 3, "user_acceptance": "oui je valide :)",
                "body_bindings": [9, 2, 1, 11, 5], "profiles": [{k: deepcopy(v) for k, v in row.items() if k != "body"} for row in profiles],
                "columns": 16, "layers_per_profile": 25, "side_family_order": ordered_sides, "files": outputs,
                "native_body": [32, 32], "composition_canvas": [64, 64], "approved_configuration_pixel_differences": 0,
                "model_count": 113, "family_count": 18, "mechanical_changes": "none", "class_gameplay_scope": "graphics_bank_only;existing_class_catalog_ids_and_rules_unchanged"}
    (OUT / "manifest.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")
    (AUDIT / "approved-layer-export.json").write_text(json.dumps({"accepted_review": refs(REVIEW / "review-weapon-silhouettes-WEAPON-SIL01-03.json"), "manifest": refs(OUT / "manifest.json"), "approved_configurations_compared": 375, "pixel_differences": 0, "existing_source_asset_changes": 0}, indent=2) + "\n", encoding="utf-8")
    assert frozen == {path: hashlib.sha256((ROOT / path).read_bytes()).hexdigest() for path in frozen}
    print(json.dumps({"profiles": 5, "posed_layers": 125, "approved_configurations": 375, "pixel_differences": 0}))


if __name__ == "__main__":
    main()
