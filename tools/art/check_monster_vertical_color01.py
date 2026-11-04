"""Native scope and source immutability for the color-only renderer correction."""
import hashlib
import json
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-color-01/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-color-01/r01/current'
base.REVIEW='MONSTER-VERTICAL-COLOR01-r01'
base.NUMBERS=[144]
base.RECORD='MONSTER-VERTICAL-COLOR01-VALIDATION.json'

if __name__=='__main__':
    data=json.loads((base.OUT/'checks.json').read_text(encoding='utf-8'))
    palette=json.loads((base.SOURCE/'red-material-palette.json').read_text(encoding='utf-8'))
    for file,key in [('source_atlas','source_sha256'),('reference','reference_sha256')]:
        assert hashlib.sha256((base.ROOT/palette[file]).read_bytes()).hexdigest()==palette[key]
    assert len(data['items'])==1
    row=data['items'][0]; assert row['geometry_and_alpha_unchanged'] and row['palette_changed_pixels']>30_000
    base.main()
