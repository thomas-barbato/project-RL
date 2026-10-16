"""ANIM-S45: lossless native prepared combat-drone phases."""
import hashlib
import html
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-92'
assert not (REVIEW/'visual-acceptance-01.json').exists()
metadata = json.loads((REVIEW/'review.json').read_text(encoding='utf-8'))
entries = json.loads((REVIEW/'_capture/capture.json').read_text(encoding='utf-8'))
assert metadata['scenes']==22
partial=len(entries)!=22
assert not partial or (len(entries)==11 and all(e['id']=='D58' for e in entries))
existing=json.loads((REVIEW/'animations-before-palette.json').read_text(encoding='utf-8')) if partial else []
checks = []
for e in entries:
    before, first, last = (e['timeline'][n] for n in [0,18,143])
    refused = e['case'] in ['ENERGIE','EXISTANT']
    if refused:
        assert before['refusal'] and not before['preview']
        assert all(t == before for t in e['timeline'])
    else:
        assert before['preview'] and not before['deployed']
        assert first['launched'] and first['deployed'] and not first['preview']
        assert first['energy']==88 and first['cooldown']==6 and first['deployments']==1
        assert all(t['energy']==88 and t['deployed'] and t['actor_identity']=='new-drone' and t['deployments']==1 for t in e['timeline'][18:])
        for t in e['timeline']:
            assert t['enemy_hp']==100-t['impacts']*t['damage_fixture']
            assert t['blood_events']==(0 if e['case']=='MACHINE' else t['impacts'])
            assert t['attacks_scheduled']<=1+t['turn']
            assert t['impacts']<=t['attacks_scheduled']
        if e['case']=='CLOISON':
            assert not first['line_of_sight']
            assert all(t['enemy_hp']==100 and t['moves']==0 and t['attacks_scheduled']==0 for t in e['timeline'])
        elif e['case']=='ATTENTE':
            assert all(t['turn']==0 and t['cooldown']==6 for t in e['timeline'][18:])
            assert all(t==e['timeline'][36] for t in e['timeline'][36:])
            assert last['impacts']==(0 if e['id']=='D58' else 1)
        else:
            for frame,turn in [(18,0),(50,1),(74,2),(100,3)]:
                assert e['timeline'][frame]['turn']==turn
                assert e['timeline'][frame]['cooldown']==6-turn
            if e['id']=='D58':
                expected=4 if e['case']=='CONTACT' else 1 if e['case']=='DEPLACEMENT' else 2
                assert last['impacts']==expected
                assert last['moves']==(0 if e['case']=='CONTACT' else 3 if e['case']=='DEPLACEMENT' else 2)
            else:
                assert last['impacts']==(3 if e['case']=='DEPLACEMENT' else 4) and last['moves']==(1 if e['case']=='DEPLACEMENT' else 0)
        if e['case']!='CLOISON':
            if e['id']=='D59' or e['case']=='CONTACT':
                assert e['timeline'][24]['impacts']==1 and e['timeline'][24]['turn']==0
            else:
                assert first['moves']==1 and first['turn']==0
    cols = e.pop('native_columns'); count = e['unique_images']; rows = (count+cols-1)//cols
    with Image.open(REVIEW/'_capture'/e['folder']/'atlas.png') as source:
        source = source.convert('RGB'); atlas = Image.new('RGB',source.size); hashes=[]
        for i in range(count):
            x,y=i%cols*576,(rows-1-i//cols)*576
            frame=source.crop((x,y,x+576,y+576));atlas.paste(frame,(x,i//cols*576))
            hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name='apercu-animations-data/'+e['variant']+'.webp';atlas.save(BASE/name,'WEBP',lossless=True,method=1)
    with Image.open(BASE/name) as packed:
        for i,digest in enumerate(hashes):
            x,y=i%cols*576,i//cols*576
            assert hashlib.sha256(packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest()==digest
    e.update(atlas=name,columns=cols)
    checks.append({'variant':e['variant'],'pixel_equality':True,'lifecycle_cost_and_hit_checks':True,'frame_rgb_sha256':hashes,'atlas_bytes':(BASE/name).stat().st_size})
    print(e['variant'],count,(BASE/name).stat().st_size,flush=True)
for p in json.loads((REVIEW/'previous-review-preservation.json').read_text(encoding='utf-8')):
    assert hashlib.sha256((ROOT/p['path']).read_bytes()).hexdigest()==p['sha256']
if partial:
    retained=[e for e in existing if e['id']=='D59']
    assert len(retained)==11
    old_checks=json.loads((REVIEW/'validation-before-palette.json').read_text(encoding='utf-8'))['checks']
    checks.extend(c for c in old_checks if c['variant'].startswith('D59-'))
    entries.extend(retained)
assert len(entries)==22
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf-8')
(REVIEW/'validation.json').write_text(json.dumps({'native_tile':64,'scenes':22,'lossless':True,'previous_art_and_curse_source_preserved':True,'checks':checks},indent=2)+'\n',encoding='utf-8')
template=(ROOT/'tools/art/drone_combat_review_template.html').read_text(encoding='utf-8')
replacements={'__ENTRIES__':text,'__BATCH__':metadata['batch'],'__SKILL_IDS__':json.dumps([c['id'] for c in metadata['candidates']]),'__VARIANTS__':' et '.join(f"{c['number']} / {c['variant']}" for c in metadata['candidates'])}
for side,c,copy in zip(['LEFT','RIGHT'],metadata['candidates'],metadata['preview_copy']):
    replacements[f'__{side}_TITLE__']=html.escape(f"{c['number']} · {c['title']}")
    replacements[f'__{side}_COPY__']=html.escape(copy)
for key,value in replacements.items():template=template.replace(key,value)
assert '__' not in template
(BASE/'apercu-animations.html').write_text(template,encoding='utf-8')
print('Playback MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
