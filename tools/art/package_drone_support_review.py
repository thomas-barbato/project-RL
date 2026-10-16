"""ANIM-S44: lossless native drone deployment and prepared cargo-transfer review."""
import hashlib
import html
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-91'
assert not (REVIEW/'visual-acceptance-01.json').exists()
metadata = json.loads((REVIEW/'review.json').read_text(encoding='utf-8'))
entries = json.loads((REVIEW/'_capture/capture.json').read_text(encoding='utf-8'))
assert len(entries) == metadata['scenes'] == 22
checks = []
for e in entries:
    before, first, last = (e['timeline'][n] for n in [0,18,111])
    assert all(t['enemy_hp'] == 100 and t['drone_attacks'] == 0 for t in e['timeline'])
    refused = e['case'] in ['MUR','OCCUPEE','ENERGIE','EXISTANT']
    if refused:
        assert before['refusal'] and not before['preview']
        assert all(t == before for t in e['timeline'])
    else:
        assert before['preview'] and not before['deployed']
        assert first['launched'] and first['deployed'] and not first['preview']
        assert first['deployments'] == 1 and first['energy'] == 90 and first['cooldown'] == 6
        assert all(t['energy'] == 90 and t['deployed'] and t['actor_identity'] == 'new-drone' and t['deployments'] == 1 for t in e['timeline'][18:])
        if e['case'] == 'ATTENTE':
            assert all(t == first for t in e['timeline'][18:])
        else:
            for frame,turn in [(18,0),(50,1),(74,2),(100,3)]:
                assert e['timeline'][frame]['turn'] == turn
                assert e['timeline'][frame]['cooldown'] == 6-turn
    if e['case'] == 'COLLECTE':
        assert before['ground_item'] and before['loot_visible']
        if e['id'] == 'D57':
            moved, picked = (e['timeline'][n] for n in [50,74])
            assert moved['drone'] == [4,4] and moved['ground_item'] and not moved['cargo']
            assert picked['cargo'] and not picked['ground_item'] and picked['pickup_events'] == 1
            assert last['pickup_events'] == 1 and last['item_identity'] == before['item_identity']
            assert all(int(t['ground_item']) + int(t['cargo']) == 1 for t in e['timeline'])
        else:
            assert all(t['ground_item'] and not t['cargo'] and t['pickup_events'] == 0 for t in e['timeline'])
    if e['case'] == 'CACHE':
        assert not before['loot_visible']
        assert all(t['ground_item'] and not t['cargo'] and t['pickup_events'] == 0 and t['drone'] == t['spawn'] for t in e['timeline'])
        assert all(t['loot_visible'] == (e['id'] == 'D56') for t in e['timeline'][18:])
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
    checks.append({'variant':e['variant'],'pixel_equality':True,'lifecycle_and_cost_checks':True,'frame_rgb_sha256':hashes,'atlas_bytes':(BASE/name).stat().st_size})
    print(e['variant'],count,(BASE/name).stat().st_size,flush=True)
for p in json.loads((REVIEW/'previous-review-preservation.json').read_text(encoding='utf-8')):
    assert hashlib.sha256((ROOT/p['path']).read_bytes()).hexdigest()==p['sha256']
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf-8')
(REVIEW/'validation.json').write_text(json.dumps({'native_tile':64,'scenes':22,'lossless':True,'previous_art_and_curse_source_preserved':True,'checks':checks},indent=2)+'\n',encoding='utf-8')
template=(ROOT/'tools/art/drone_support_review_template.html').read_text(encoding='utf-8')
replacements={'__ENTRIES__':text,'__BATCH__':metadata['batch'],'__SKILL_IDS__':json.dumps([c['id'] for c in metadata['candidates']]),'__VARIANTS__':' et '.join(f"{c['number']} / {c['variant']}" for c in metadata['candidates'])}
for side,c,copy in zip(['LEFT','RIGHT'],metadata['candidates'],metadata['preview_copy']):
    replacements[f'__{side}_TITLE__']=html.escape(f"{c['number']} · {c['title']}")
    replacements[f'__{side}_COPY__']=html.escape(copy)
for key,value in replacements.items():template=template.replace(key,value)
assert '__' not in template
(BASE/'apercu-animations.html').write_text(template,encoding='utf-8')
print('Playback MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
