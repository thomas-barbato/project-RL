"""Review 32: vivid saturated purple, same occult drawings and timing."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import prepare_agent_skill_animation_occult_contrast as previous

ROOT=previous.previous.base.ROOT
OUT=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-32'
BASE=ROOT/'artifacts/refonte-agent-2026-10-06/skill-animation-review-31'


def protected():
    hashes=previous.old_protected()
    for number in (29,30,31):
        root=ROOT/f'artifacts/refonte-agent-2026-10-06/skill-animation-review-{number}'
        for p in sorted(root.rglob('*')):
            if p.is_file():hashes[p.relative_to(ROOT).as_posix()]=hashlib.sha256(p.read_bytes()).hexdigest()
    return hashes


def configure():
    previous.BASE=BASE
    previous.OUT=OUT
    previous.VERSION='A04'
    previous.PREVIOUS_VERSION='A03'
    previous.RENDER_OPTION='--occulte-violet-franc'
    previous.PALETTE_NAME='Violet franc'
    previous.protected=protected


def prepare():
    previous.prepare()
    p=OUT/'review.json';review=json.loads(p.read_text(encoding='utf-8'))
    review['feedback']='The user asks for true light purple rather than lavender. A different color and icon revisions are only a possible later decision.'
    review['revision']='Saturated purple replaces colored violet facets, including their tips, without pastel or near-white lifting. Neutral black cores and red seal remain distinct. Shader color only; source drawings and alpha unchanged.'
    review['icons']='Unchanged; no broader palette decision or visual approval is inferred.'
    p.write_text(json.dumps(review,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--comparisons-only',action='store_true');args=parser.parse_args()
    configure()
    if args.prepare:prepare()
    else:
        previous.package(args.comparisons_only)
        (OUT/'package-source.py').write_bytes(Path(__file__).read_bytes())
