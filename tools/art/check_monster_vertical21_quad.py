"""Review 21 is a quadruped; no biped acceptance or duplicate roster entry."""
import json
import check_monster_vertical01 as base

base.SOURCE=base.ROOT/'assets/source-art/animation-polish-01/monster-vertical-21-quad/r01'
base.OUT=base.ROOT/'artifacts/animation-polish-01/monster-vertical-21-quad/r01/current'
base.REVIEW='MONSTER-VERTICAL21-QUAD01-r01'
base.NUMBERS=[32]
base.RECORD='MONSTER-VERTICAL21-QUAD01-VALIDATION.json'

if __name__=='__main__':
    manifest=json.loads((base.SOURCE/'manifest.json').read_text(encoding='utf-8'))
    p=manifest['profiles'][0];assert p['locomotion']=='quadruped' and p['locomotor_legs']==4
    accepted=json.loads((base.REV/'MONSTER-VERTICAL07-ACCEPTANCE-r01.json').read_text(encoding='utf-8'))
    assert accepted['models']==[26,28] and accepted['withdrawn_models']==[32]
    base.main()
    browser=base.OUT.parent/'browser-checks.json'
    if browser.exists():
        b=json.loads(browser.read_text(encoding='utf-8'))
        assert len(b['checked_models'])==24 and len({p['id'] for p in b['checked_models']})==24
        assert len(b['manual_quad_phases'])==18 and len(b['manual_stops'])==2
        assert b['rejected_biped_selector_entries']==0 and b['automatic_both_directions']
        assert 'quadruped' in b['corrected_inventory_caption']
