"""Native review sheets, palette comparisons and a self-contained gallery."""
from pathlib import Path
import base64
import difflib
import hashlib
import json
import runpy
from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
OUT = BASE / "weapon-silhouette-review-02"
H = runpy.run_path(str(BASE / "build_weapon_silhouette_review_01.py"), run_name="weapon_helpers")
H["save_png"].__globals__["OUT"] = OUT
cell, copy_layer, save_png = H["cell"], H["copy_layer"], H["save_png"]
FONT = H["H"]["HELP"]["font"]


def ref(path):
    return {"reference": path.relative_to(ROOT).as_posix(), "reference_sha256": hashlib.sha256(path.read_text(encoding="utf-8").encode("utf-8")).hexdigest()}


def save_json(path, value):
    assert not path.exists()
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8", newline="\n")


def main(bank_revision=2):
    global OUT
    OUT = BASE / f"weapon-silhouette-review-{bank_revision:02d}"
    save_png.__globals__["OUT"] = OUT
    bank_path = OUT / f"review-weapon-silhouettes-WEAPON-SIL01-{bank_revision:02d}.json"
    bank = json.loads(bank_path.read_text(encoding="utf-8"))
    assert not bank["qa"]["new_detached_component_cases"]
    frames = {(r["class_number"], r["left_family"], r["right_family"]): r for r in bank["frames"]}
    atlas_path = ROOT / bank["atlas"]["path"]
    families = bank["families"]
    sheets = []
    for page in range(3):
        selected = families[page * 6:page * 6 + 6]
        board = Image.new("RGB", (710, 680), "#0c131b")
        draw = ImageDraw.Draw(board)
        draw.text((16, 12), f"WEAPON-SIL01 · Familles {page * 6 + 1:02d}–{page * 6 + 6:02d}", font=FONT(22), fill="#eef0f2")
        draw.text((16, 47), "Corps natifs 32 × 32 · débordements conservés · vues 1:1", font=FONT(14), fill="#c2ced5")
        names = ["1. Bastion", "2. Techno.", "3. Traqueur", "4. Arcaniste", "5. Métamorphe"]
        for i, name in enumerate(names):
            draw.text((202 + i * 100, 86), name, font=FONT(12), fill="#d7e1e8")
        for row, family in enumerate(selected):
            y = 116 + row * 90
            draw.rectangle((14, y, 697, y + 80), fill="#182430")
            draw.text((22, y + 14), f"{family['index'] + 1:02d}. {family['name']}", font=FONT(13), fill="#d7e1e8")
            draw.text((22, y + 39), "Une main" if family["index"] in H["ONE"] else "Deux mains", font=FONT(12), fill="#aec1cc")
            for i in range(5):
                record = frames[(i + 1, None, family["index"])]
                image = Image.open(ROOT / record["native_image"]["path"]).convert("RGBA")
                board.paste(image, (211 + i * 100, y + 5), image)
                draw.text((229 + i * 100, y + 65), f"{i + 1}–{family['index'] + 1:02d}", font=FONT(10), fill="#aec1cc")
        sheets.append(save_png(f"families-native-page-{page + 1}.png", board))
    # Exact approved blue-pistol orientation versus the requested gray material.
    old = Image.open(BASE / "classe-equipee-2-START-VIS01-06.png").convert("RGBA")
    old64 = Image.new("RGBA", (64, 64))
    copy_layer(old64, old, (8, 8))
    new = Image.open(ROOT / frames[(2, None, 6)]["native_image"]["path"]).convert("RGBA")
    assert old64.getchannel("A").tobytes() == new.getchannel("A").tobytes()
    pistol_board = Image.new("RGB", (640, 440), "#0c131b")
    pd = ImageDraw.Draw(pistol_board)
    pd.text((18, 12), "2. Technomancien · Pistolet gris clair", font=FONT(23), fill="#eef0f2")
    pd.text((18, 48), "Orientation validée conservée · bras et gant inchangés", font=FONT(14), fill="#c2ced5")
    for x, image, label in [(18, old64, "Avant · bleu"), (338, new, "Proposition · gris clair")]:
        pd.rectangle((x, 82, x + 281, 425), fill="#182430", outline="#3b4b59")
        pd.text((x + 12, 93), label, font=FONT(16), fill="#d7e1e8")
        zoom = image.resize((192, 192), Image.Resampling.NEAREST)
        pistol_board.paste(zoom, (x + 45, 125), zoom)
        pd.text((x + 12, 326), "Zoom ×3 / taille native 1:1", font=FONT(12), fill="#aec1cc")
        pistol_board.paste(image, (x + 108, 348), image)
    pistol_ref = save_png("pistol-light-gray-comparison.png", pistol_board)
    # Show the actual legacy Pink-to-Gray material rule on equipment shapes.
    art = ROOT / "assets/sprites/cyberpunk"
    body = cell(Image.open(art / "crawl-bodies-32.png").convert("RGBA"), 9, 32)
    body_map = cell(Image.open(art / "crawl-body-materials-32.png").convert("RGBA"), 9, 32)
    weapon = cell(Image.open(art / "crawl-weapons-48.png").convert("RGBA"), 6, 48)
    weapon_map = cell(Image.open(art / "crawl-weapon-materials-48.png").convert("RGBA"), 6, 48)
    pink_body = H["recolor_labels"](body, body_map, {1: H["PINK"], 2: H["BLUE"]})
    gray_body = H["recolor_labels"](body, body_map, {1: H["GRAY"], 2: H["GRAY"]})
    pink_weapon = H["recolor_labels"](weapon, weapon_map, {1: H["PINK"], 2: H["BLUE"]})
    gray_weapon = H["recolor_labels"](weapon, weapon_map, {1: H["GRAY"], 2: H["GRAY"]})
    palette = Image.new("RGB", (640, 390), "#0c131b")
    dd = ImageDraw.Draw(palette)
    dd.text((18, 12), "COLOR-GRAY01 · Le gris remplace le rose", font=FONT(22), fill="#eef0f2")
    dd.text((18, 48), "Même équipement · mêmes statistiques et bonus · nouvelles couleurs proposées", font=FONT(13), fill="#c2ced5")
    for x, images, label in [(18, [pink_body, pink_weapon], "Ancienne palette · rose"), (338, [gray_body, gray_weapon], "Palette proposée · gris neutre")]:
        dd.rectangle((x, 80, x + 281, 373), fill="#182430", outline="#3b4b59")
        dd.text((x + 12, 90), label, font=FONT(15), fill="#d7e1e8")
        for image, dx in zip(images, [30, 110]):
            zoom = image.resize((image.width * 3, image.height * 3), Image.Resampling.NEAREST)
            palette.paste(zoom, (x + dx, 135), zoom)
            palette.paste(image, (x + dx + 20, 308), image)
    gray_ref = save_png("pink-to-neutral-gray-comparison.png", palette)
    floors = cell(Image.open(art / "terrain-materials-32.png").convert("RGBA"), 4, 32)
    floor_ref = save_png("review-floor-native32.png", floors)
    # Preserve serialization position and RNG draw count in a concrete patch.
    names = ["src/entity/equipment_palette.rs", "src/game_art_palette.rs", "src/game/equipment_colors.rs", "src/game_art_worn.rs", "src/game/game_state.rs"]
    patch, changed_files = [], []
    for name in names:
        original = (ROOT / name).read_text(encoding="utf-8")
        revised = original.replace("EquipmentPalette::Pink", "EquipmentPalette::Gray")
        if name.endswith("entity/equipment_palette.rs"):
            revised = revised.replace("    Pink,", '    #[serde(alias = "Pink")]\n    Gray,').replace("Self::Pink", "Self::Gray").replace('=> "Rose"', '=> "Gris"')
            revised = revised.replace("[[104, 28, 82], [221, 67, 156], [255, 160, 216]]", "[[38, 43, 48], [161, 171, 181], [232, 239, 245]]")
            revised += '''
#[cfg(test)]
mod palette_compatibility_tests {
    use super::EquipmentPalette;

    #[test]
    fn legacy_pink_json_and_binary_position_resolve_to_gray() {
        let palette: EquipmentPalette = serde_json::from_str(r#""Pink""#).unwrap();
        assert_eq!(palette, EquipmentPalette::Gray);
        assert_eq!(EquipmentPalette::Gray.index(), 6);
        assert_eq!(serde_json::to_string(&palette).unwrap(), r#""Gray""#);
    }
}
'''
        elif name.endswith("game_art_palette.rs"):
            revised = revised.replace("Pink => Blue", "Gray => Gray")
        assert revised != original
        patch.extend(difflib.unified_diff(original.splitlines(keepends=True), revised.splitlines(keepends=True), fromfile="a/" + name, tofile="b/" + name))
        changed_files.append({"path": name, "current_content_sha256": hashlib.sha256(original.encode("utf-8")).hexdigest()})
    patch_path = OUT / "replace-pink-with-gray-proposed.patch"
    assert not patch_path.exists()
    patch_path.write_text("".join(patch), encoding="utf-8", newline="\n")
    data = {"frames": [{"c": r["class_number"], "l": r["left_family"], "r": r["right_family"], "i": r["sprite_index"]} for r in bank["frames"]],
            "classes": [{"number": p["number"], "name": p["class"]} for p in bank["classes"]],
            "families": [{"id": p["index"], "name": p["name"], "two": p["index"] not in H["ONE"]} for p in families],
            "atlas": "data:image/png;base64," + base64.b64encode(atlas_path.read_bytes()).decode("ascii"),
            "floor": "data:image/png;base64," + base64.b64encode((ROOT / floor_ref["path"]).read_bytes()).decode("ascii")}
    template = (BASE / "weapon-silhouette-gallery-template-01.html").read_text(encoding="utf-8")
    gallery_path = OUT / "weapon-silhouette-gallery.html"
    assert not gallery_path.exists()
    gallery_path.write_text(template.replace("__REVIEW_DATA__", json.dumps(data, ensure_ascii=False).replace("<", "\\u003c")), encoding="utf-8", newline="\n")
    assert gallery_path.stat().st_size < 1_000_000
    result = {"date": "2026-10-09", "review_id": "WEAPON-SIL01-REVIEW", "weapon_bank": ref(bank_path),
              "native_sheets": sheets, "light_gray_pistol": pistol_ref, "neutral_gray_replacement": gray_ref,
              "gallery": ref(gallery_path), "proposed_palette_code_patch": ref(patch_path), "palette_code_patch_sources": changed_files,
              "palette_patch_scope": "replace_Pink_variant_with_Gray_at_same_enum_index;serde_alias_reads_legacy_Pink;ALL_length7_and_rng_draw_count_unchanged;secondary_Gray_neutral",
              "code_patch_validation": "prepared_only;not_applied_or_compiled;visual_gray_review_required",
              "orientation_acceptance": "user_oui_c_est_mieux_accepts_prior_diagonal_pistol_orientation;gray_material_new_pending",
              "gray_direction_acceptance": "user_requests_light_gray_pistol;rose_items_become_neutral_gray;exact_shown_palette_pending_visual_review",
              "traqueur_layer_scope": "unarmed_layer_prepared_from_existing_supporting_source;requires_review_as_part_of_new_family_compositions",
              "mechanical_changes": "none", "active_asset_or_code_changes": 0, "runtime_integration": "not_integrated"}
    save_json(OUT / "weapon-silhouette-review-presentation-01.json", result)
    print(json.dumps({"native_pages": 3, "configurations": 375, "gallery_bytes": gallery_path.stat().st_size,
                      "pistol_geometry_changes": 0, "palette_patch": "prepared_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
