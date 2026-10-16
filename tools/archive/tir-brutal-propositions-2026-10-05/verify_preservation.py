"""Verify review outputs and that active assets/renderers were not replaced."""
from pathlib import Path
import hashlib
import json
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
before = json.loads((OUT / "runtime-avant.json").read_text(encoding="utf-8"))
changed = [name for name, digest in before.items()
           if hashlib.sha256((ROOT / name).read_bytes()).hexdigest() != digest]
checks = json.loads((OUT / "controle-taille.json").read_text(encoding="utf-8"))
assert all(check["body_height"] == 30 for check in checks), checks
outputs = {}
for name in ("dix-armes-source-v1.png", "personnages-tir-source-v1.png",
             "dix-armes-proposees.png", "comparatif-armes-portees.png"):
    path = OUT / name
    im = Image.open(path).convert("RGBA")
    outputs[name] = {"size": im.size, "alpha": im.getchannel("A").getextrema(),
                     "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
    if "source" in name:
        assert im.getchannel("A").getextrema()[0] == 0, name
report = {"checked_runtime_files": len(before), "changed_runtime_files": changed,
          "body_heights": checks, "outputs": outputs}
(OUT / "preservation.json").write_text(json.dumps(report, indent=2),encoding="utf-8")
assert not changed, changed
print(f"{len(before)} active files preserved; 6 bodies at 30px; two transparent sources; review only")
