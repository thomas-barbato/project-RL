"""Native checks for the possessed batch, retaining prior accepted drawings."""
import hashlib
import json
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-04/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-04/r01/current'
base.REVIEW='MONSTER-VERTICAL04-r01'
base.NUMBERS=[6,7,10]
base.RECORD='MONSTER-VERTICAL04-VALIDATION.json'

if __name__=='__main__':
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    possessed=next(p for p in captures['items'] if p['id']=='MON-006-0')
    assert possessed['geometry_and_alpha_unchanged'] and possessed['palette_changed_pixels']>2000
    palette=json.loads((base.SOURCE/'MON-006-ivory-palette.json').read_text(encoding='utf-8'))
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/palette[file]).read_bytes()).hexdigest()==palette[key]
    colors=json.loads((base.REV/'MONSTER-VERTICAL04-COLORS.json').read_text(encoding='utf-8'))
    skin=next(m for m in colors['materials'] if m['model']==6 and m['material']=='warm_ivory')
    assert max(abs(v) for v in skin['median_rgb_delta'])<=2
    base.main()
