"""Review 31: broader pale-lavender facets, same occult drawings and timing."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import prepare_agent_skill_animation_occult_contrast as previous

ROOT=previous.previous.base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-31'
BASE=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-30'


def protected():
    hashes=previous.old_protected()
    for root in [ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-29',BASE]:
        for p in sorted(root.rglob('*')):
            if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def configure():
    previous.BASE=BASE
    previous.OUT=OUT
    previous.VERSION='A03'
    previous.PREVIOUS_VERSION='A02'
    previous.RENDER_OPTION='--occulte-violet-lavande'
    previous.PALETTE_NAME='Violet lavande'
    previous.protected=protected


def prepare():
    previous.prepare()
    p=OUT/'review.json';review=json.loads(p.read_text(encoding='utf-8'))
    review['feedback']='The user asks for a significantly lighter violet because review 30 is still difficult to see.'
    review['revision']='Pale lavender covers more colored violet facets, with almost-white tips. Neutral black cores and red seal remain distinct. Shader color only; raster drawings and alpha unchanged.'
    p.write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--comparisons-only',action='store_true');args=parser.parse_args()
    configure()
    if args.prepare:prepare()
    else:
        previous.package(args.comparisons_only)
        (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
