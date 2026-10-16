"""Lossless area-curse playback and actor-binding verification (review number argument)."""
import hashlib
import html
import json
import sys
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
number = int(sys.argv[1]) if len(sys.argv) > 1 else 90
assert number in (88, 89, 90)
REVIEW = BASE / f'skill-animation-review-{number}'
assert not (REVIEW / 'visual-acceptance-01.json').exists()
metadata = json.loads((REVIEW/'review.json').read_text(encoding='utf-8'))
skills = [c['id'] for c in metadata['candidates']]
catalogue = {s['id']:s for s in json.loads((ROOT/'assets/skills/agent/catalogue.json').read_text(encoding='utf-8'))['skills']}
def parameter(id, key):
    if id == 'AF06':
        values = metadata['prepared_values'][id]
        return values[{'duration_rank_turns':'duration','cooldown_turns':'cooldown','energy':'energy'}[key]]
    p = catalogue[id]['parameters'][key]
    return p['value'] if p['kind'] == 'constant' else p['base']
entries = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf-8'))
assert len(entries) == metadata['scenes']
previous = json.loads((BASE/'skill-animation-review-87/animations.json').read_text(encoding='utf-8')) if number == 88 else []
marker_proof = json.loads((REVIEW/'_capture/marker-transparency.json').read_text(encoding='utf-8'))
assert len(marker_proof) == 2
assert all(p['fully_transparent_pixels'] >= 32 and p['visible_pixels'] >= 32 for p in marker_proof)
(REVIEW/'marker-transparency.json').write_text(json.dumps(marker_proof,indent=2)+'\n',encoding='utf-8')
checks = []
for e in entries:
    assert e['id'] in skills
    duration = parameter(e['id'], 'duration_rank_turns')
    cooldown = parameter(e['id'], 'cooldown_turns')
    cost = parameter(e['id'], 'energy')
    if previous:
        old = next(p for p in previous if (p['id'],p['case'],p['direction']) == (e['id'],e['case'],e['direction']))
        assert e['timeline'] == old['timeline'], 'Cosmetic revision changed mechanical timeline'
    before, first, last = (e['timeline'][n] for n in (0, 18, 111))
    passive = e['id'] == 'AF06'
    assert before['energy'] == 100 and before['preview'] == (not passive) and before['applied_count'] == 0
    assert first['cast'] and not first['preview'] and first['area_width'] == (5 if passive else 3)
    assert first['energy'] == last['energy'] == 100 - cost
    assert first['cooldown'] == cooldown
    expected = 1 if e['case'] == 'JOUEUR' else (3 if passive else 2) if e['case'] == 'RESISTE' else (4 if passive and e['case'] not in ['DEPLACEMENT','CHAINE'] else 3)
    if e['case'] == 'MUR':
        expected = first['applied_count']
        assert 1 <= expected <= 2
        assert first['victims'][0]['applied'] and not first['victims'][1]['applied']
        assert first['victims'][1]['inside_area_now'] is False
    assert first['applied_count'] == expected and first['active_count'] == expected
    assert not first['victims'][4]['applied']
    for t in e['timeline']:
        for v in t['victims']:
            hit = e['case'] == 'DEGATS' and t['turn'] >= 1 and v['index'] == 0
            dead = e['case'] == 'CHAINE' and t['turn'] >= 1 and v['index'] == 0
            assert v['hp'] == (0 if dead else 90 if hit else 100)
            assert v['damage'] == (10 if hit else 0)
    assert all(not v['applied'] for t in e['timeline'] for v in t['victims'] if not v['hostile'])
    if e['case'] == 'ATTENTE':
        assert all(t == first for t in e['timeline'][18:])
        assert last['turn'] == 0 and last['active_count'] == expected
    else:
        for n, turn in [(18, 0), (50, 1), (74, 2), (100, 3)]:
            state = e['timeline'][n]
            assert state['turn'] == turn and state['cooldown'] == max(0,cooldown - turn)
            for v in state['victims']:
                interrupted = turn >= 1 and v['index'] == 0 and (e['case'] == 'CHAINE' or (e['case'] == 'DEGATS' and e['id'] == 'AF05'))
                assert v['remaining'] == (max(0,duration-turn) if v['applied'] and not interrupted else 0)
        assert last['active_count'] == 0
    if e['case'] == 'DEPLACEMENT':
        left = e['timeline'][54]['victims'][0]
        entered = e['timeline'][78]['victims'][3]
        assert left['position'] == [1 if passive else 2, 2] and not left['inside_area_now'] and left['icon_visible']
        assert entered['position'] == [5, 3] and entered['inside_area_now'] and not entered['icon_visible']
    if passive:
        assert before['source_alive'] and all(not t['source_alive'] and t['death_procs'] == 1 and t['energy'] == 100 for t in e['timeline'][18:])
        assert all(v['received_by_contagion'] == v['applied'] for t in e['timeline'] for v in t['victims'])
    if e['case'] == 'CHAINE':
        assert not first['victims'][3]['applied'] and not last['victims'][3]['applied']
    cols = e.pop('native_columns')
    count = e['unique_images']
    rows = (count + cols - 1) // cols
    with Image.open(REVIEW / '_capture' / e['folder'] / 'atlas.png') as source:
        source = source.convert('RGB')
        atlas = Image.new('RGB', source.size)
        hashes = []
        for i in range(count):
            x, y = i % cols * 576, (rows - 1 - i // cols) * 576
            frame = source.crop((x, y, x+576, y+576))
            atlas.paste(frame, (x, i // cols * 576))
            hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
    name = 'apercu-animations-data/' + e['variant'] + '.webp'
    atlas.save(BASE / name, 'WEBP', lossless=True, method=1)
    with Image.open(BASE / name) as packed:
        for i, digest in enumerate(hashes):
            x, y = i % cols * 576, i // cols * 576
            assert hashlib.sha256(packed.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest() == digest
    e.update(atlas=name, columns=cols)
    checks.append({'variant':e['variant'],'frames':e['frames'],'unique_images':count,'frame_rgb_sha256':hashes,'pixel_equality':True,'timeline':True,'previous_mechanical_timeline_identical':True if previous else None,'atlas_bytes':(BASE/name).stat().st_size})
    print(e['variant'],count,(BASE/name).stat().st_size,flush=True)
for p in json.loads((REVIEW/'previous-review-preservation.json').read_text(encoding='utf-8')):
    assert hashlib.sha256((ROOT/p['path']).read_bytes()).hexdigest() == p['sha256']
text = json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf-8')
(REVIEW/'validation.json').write_text(json.dumps({'native_tile':64,'scenes':len(entries),'lossless':True,'previous_accepted_art_preserved':True,'checks':checks},indent=2)+'\n',encoding='utf-8')
template=(ROOT/'tools/art/afflictions_area_review_template.html').read_text(encoding='utf-8')
replacements = {'__ENTRIES__':text,'__BATCH__':metadata['batch'],'__SKILL_IDS__':json.dumps(skills),'__VARIANTS__':' et '.join(f"{c['number']} / {c['variant']}" for c in metadata['candidates'])}
replacements.update({
    '__COMMON_NOTE__':html.escape(metadata.get('common_note','')),
    '__DETAIL_NOTE__':html.escape(metadata.get('detail_note','')),
    '__BEFORE_PHASE__':json.dumps(metadata.get('before_phase','Avant lancement · carré de 3 × 3 cases, découpé par les obstacles'),ensure_ascii=False),
    '__START_ACTION_LABEL__':json.dumps(metadata.get('start_action_label','Lancer'),ensure_ascii=False),
    '__EXTRA_SCENARIOS__':'<option value="DEGATS">Dégâts et interruption</option><option value="CHAINE">Mort sans retransmission</option>' if number == 90 else '',
})
for side, candidate, copy in zip(['LEFT','RIGHT'],metadata['candidates'],metadata['preview_copy']):
    replacements[f'__{side}_TITLE__'] = html.escape(f"{candidate['number']} · {candidate['title']}")
    replacements[f'__{side}_COPY__'] = html.escape(copy)
for key, value in replacements.items():
    template = template.replace(key,value)
assert '__' not in template
(BASE/'apercu-animations.html').write_text(template,encoding='utf-8')
print('Playback MiB:',round(sum(c['atlas_bytes'] for c in checks)/2**20,2))
