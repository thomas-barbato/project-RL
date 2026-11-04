"""Native checks for the next two gun carriers and possessed walker."""
import hashlib
import json
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-06/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-06/r01/current'
base.REVIEW='MONSTER-VERTICAL06-r01'
base.NUMBERS=[22,24,25]
base.RECORD='MONSTER-VERTICAL06-VALIDATION.json'

if __name__=='__main__':
    captures=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    possessed=next(p for p in captures['items'] if p['id']=='MON-025-0')
    assert possessed['geometry_and_alpha_unchanged'] and possessed['palette_changed_pixels']>2000
    palette=json.loads((base.SOURCE/'MON-025-ivory-palette.json').read_text(encoding='utf-8'))
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/palette[file]).read_bytes()).hexdigest()==palette[key]
    base.main()
