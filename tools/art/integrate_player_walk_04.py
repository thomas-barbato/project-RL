"""Install the WALK04 drawings approved for in-game use on 23 October 2026.

No runtime path points to a review capture, authoring source, Site or browser.
"""
import hashlib,json,shutil
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-04'
SOURCE=ROOT/'artifacts/player-walk-review-04'
OUTPUT=ROOT/'assets/animations/walk-04'

def main():
    report=json.loads((DOC/'packaging.json').read_text('utf-8'))
    baseline=json.loads((DOC/'baseline-data.json').read_text('utf-8'))
    records={r['id']:r for r in baseline['records']};entries=[]
    OUTPUT.mkdir(parents=True,exist_ok=True)
    for pose in report['entries']:
        record=records[pose['id']];source=SOURCE/pose['atlas'];destination=OUTPUT/(pose['id']+'.png')
        content=source.read_bytes()
        if not destination.exists() or destination.read_bytes()!=content:shutil.copy2(source,destination)
        entries.append({'id':pose['id'],'file':destination.relative_to(ROOT).as_posix(),
            'side':pose['size'],'foot':pose['foot'],'configurations':pose['rows']//3,
            'shadow':max(12,min(32,record['bounds'][2]-record['bounds'][0]))*.55,
            'sha256':hashlib.sha256(content).hexdigest()})
    assert len(entries)==659 and len({e['id'] for e in entries})==659
    manifest={'format_version':1,'review':'WALK04-r01','native_tile':64,'frames':9,
        'approval':'User explicitly authorized game integration on 2026-10-23.',
        'default_duration_ms':400,'duration_choices_ms':[120,200,400],'entries':entries}
    (OUTPUT/'manifest.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n','utf-8')
    # Keep the corrected canvas-02 gait and modular equipment on every rebuild;
    # reinstalling the old static review must never revive the frontal fallback.
    from build_walk_rigs_02 import main as install_rigs
    from rebuild_walk_cycle_02 import main as install_cycles
    install_rigs()
    install_cycles()
    print(json.dumps({'installed':len(entries),'bytes':sum(p.stat().st_size for p in OUTPUT.glob('*.png')),'runtime_root':str(OUTPUT)}))

if __name__=='__main__':main()
