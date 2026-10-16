"""Compare the stage's real files, never traversing older junction checkouts."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
STAGE = Path(__file__).resolve().parent
ALLOW = {
    "src/ascii_app.rs", "src/suspension.rs", "src/equipment_commerce.rs",
    "src/equipment_generation.rs", "src/game_art_catalog.rs", "src/cyberpunk_capture.rs",
    "src/game_art_palette.rs", "src/game_art_worn.rs", "src/loot/equipment_tests.rs",
    "src/firearm_models_tests.rs",
    "assets/sprites/cyberpunk/manifest.json", "assets/sprites/cyberpunk/README.md",
}

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

checked, changed, missing, unexpected, added = [], [], [], [], []
for before, live in [("src", "src"), ("core", "content/core"),
                     ("cyberpunk", "assets/sprites/cyberpunk")]:
    baseline = STAGE / "avant" / before
    actual = ROOT / live
    for path in sorted(baseline.rglob("*")):
        if not path.is_file():
            continue
        relative = Path(live) / path.relative_to(baseline)
        target = ROOT / relative
        name = relative.as_posix()
        checked.append(name)
        if not target.is_file():
            missing.append(name)
        elif sha(path) != sha(target):
            changed.append(name)
            if name not in ALLOW:
                unexpected.append(name)
    for path in sorted(actual.rglob("*")):
        if path.is_file() and not (baseline / path.relative_to(actual)).exists():
            added.append((Path(live) / path.relative_to(actual)).as_posix())
result = {
    "checked_count": len(checked), "changed": changed, "missing": missing,
    "unexpected_changes": unexpected, "added": added,
    "png_changes": [name for name in changed if name.endswith(".png")],
    "preexisting_content_changed": [name for name in changed if name.startswith("content/")],
}
(STAGE / "preservation.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print(json.dumps(result, indent=2))
if missing or unexpected or result["png_changes"] or result["preexisting_content_changed"]:
    raise SystemExit("Unexpected preservation difference")
