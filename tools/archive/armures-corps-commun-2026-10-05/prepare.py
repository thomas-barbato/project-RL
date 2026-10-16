"""Read-only baseline; preserve the earlier numbered gallery as a reference."""
from pathlib import Path
import hashlib, json, shutil

ROOT=Path(__file__).resolve().parents[2]
OUT=Path(__file__).resolve().parent
OLD=ROOT/'artifacts/armures-bicolores-completes-2026-10-05'
baseline={}
for directory in ('src','assets/sprites/cyberpunk','content/core','tools/art'):
    for p in (ROOT/directory).rglob('*'):
        if p.is_file() and '__pycache__' not in p.parts:
            baseline[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
(OUT/'baseline.json').write_text(json.dumps(baseline,indent=2),encoding='utf-8')
for name in ('current.rs','proposal.rs','modeles.json'):
    shutil.copy2(OLD/name,OUT/name)
print(f'Baseline: {len(baseline)} active files; previous proposals preserved.')
