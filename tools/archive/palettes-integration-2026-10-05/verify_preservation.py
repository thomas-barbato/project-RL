"""Compare this step with its source snapshot; never restore or mutate code."""
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
changed = []
checked = 0
for before, current in ((HERE / "avant/src", ROOT / "src"),
                        (HERE / "avant/cyberpunk", ROOT / "assets/sprites/cyberpunk")):
    for source in sorted(before.rglob("*")):
        if not source.is_file():
            continue
        path = current / source.relative_to(before)
        checked += 1
        if not path.is_file() or source.read_bytes() != path.read_bytes():
            changed.append(str(path.relative_to(ROOT)).replace("\\", "/"))
new = sorted(str(p.relative_to(ROOT)).replace("\\", "/")
             for p in (ROOT / "src").rglob("*.rs")
             if not (HERE / "avant/src" / p.relative_to(ROOT / "src")).exists())
result = {"checked_files": checked, "modified_since_step_start": changed, "new_source_files": new}
(HERE / "preservation-final.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
print(json.dumps(result, indent=2))
