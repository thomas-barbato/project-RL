"""Run native checks for the third batch while preserving earlier approvals."""
import hashlib
import json
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-03/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-03/r01/current'
base.REVIEW='MONSTER-VERTICAL03-r01'
base.NUMBERS=[39,146,151]
base.RECORD='MONSTER-VERTICAL03-VALIDATION.json'

if __name__=='__main__':
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    ivory=next(p for p in captures['items'] if p['id']=='MON-151-0')
    assert ivory['geometry_and_alpha_unchanged'] and ivory['palette_changed_pixels']>5000
    palette=json.loads((base.SOURCE/'MON-151-ivory-palette.json').read_text(encoding='utf-8'))
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/palette[file]).read_bytes()).hexdigest()==palette[key]
    colors=json.loads((base.REV/'MONSTER-VERTICAL03-COLORS.json').read_text(encoding='utf-8'))
    skin=next(m for m in colors['materials'] if m['model']==151 and m['material']=='warm_ivory')
    assert max(abs(v) for v in skin['median_rgb_delta'])<=2
    base.main()
