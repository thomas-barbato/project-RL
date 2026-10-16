"""Reopen unchanged AF05/AF06 native candidates with compact data and receipts."""
import hashlib
import html
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
SOURCE = BASE / 'skill-animation-review-90'
REVIEW = BASE / 'skill-animation-review-101'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def write(name, value):
    (REVIEW/name).write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')

assert not REVIEW.exists(), 'Use a fresh review; do not overwrite a recorded approval.'
REVIEW.mkdir()
metadata = json.loads((SOURCE/'review.json').read_text(encoding='utf-8'))
assert metadata.get('status') != 'visually_accepted_pending_campaign_integration', 'This pair is already accepted. Do not reopen it for approval.'
entries = json.loads((SOURCE/'animations.json').read_text(encoding='utf-8'))
original = {e['variant']: e for e in json.loads((SOURCE/'validation.json').read_text(encoding='utf-8'))['checks']}
assert sha(ROOT/metadata['source']) == metadata['source_sha256']
paths = list(json.loads((BASE/'skill-animation-review-100/preservation.json').read_text(encoding='utf-8')))
paths += ['artifacts/refonte-agent-2026-10-06/skill-animation-review-100/visual-acceptance-01.json',
          'src/agent_skill_lab_review/fire_painted/ranged_harpoon_vampiric.rs',
          'artifacts/refonte-agent-2026-10-06/skill-animation-review-100/DI05-physical-shots-64.webp',
          'artifacts/refonte-agent-2026-10-06/skill-animation-review-100/DI07-physical-shots-64.webp',
          'artifacts/refonte-agent-2026-10-06/skill-animation-review-90/review.json',
          'artifacts/refonte-agent-2026-10-06/skill-animation-review-90/animations.json',
          metadata['source'], 'assets/skills/agent/AF05-compact.png', 'assets/skills/agent/AF06-compact.png']
paths += ['artifacts/refonte-agent-2026-10-06/'+e['atlas'] for e in entries]
paths = list(dict.fromkeys(paths))
preserved = {p:sha(ROOT/p) for p in paths}
write('preservation.json', preserved)
checks = []
for entry in entries:
    atlas_path = BASE/entry['atlas']
    with Image.open(atlas_path) as atlas:
        for index, expected in enumerate(original[entry['variant']]['frame_rgb_sha256']):
            x,y=index%entry['columns']*576,index//entry['columns']*576
            assert hashlib.sha256(atlas.crop((x,y,x+576,y+576)).convert('RGB').tobytes()).hexdigest()==expected
        if entry['case']=='GROUPE' and entry['direction']==2:
            frames=[]
            for index in entry['indices']:
                x,y=index%entry['columns']*576,index//entry['columns']*576
                frames.append(atlas.crop((x,y,x+576,y+576)).convert('RGB'))
            target=REVIEW/(entry['id']+'-unchanged-native-64.webp')
            durations=[1000*(i+1)//entry['fps']-1000*i//entry['fps'] for i in range(entry['frames'])]
            frames[0].save(target,'WEBP',save_all=True,append_images=frames[1:],duration=durations,loop=0,lossless=True,method=1)
            with Image.open(target) as animation:
                elapsed=0
                for index in range(animation.n_frames):
                    animation.seek(index); animation.load()
                    n=min(entry['frames']-1,(elapsed*entry['fps']+500)//1000)
                    assert animation.convert('RGB').tobytes()==frames[n].tobytes()
                    elapsed+=animation.info['duration']
                assert elapsed==sum(durations)
            board=Image.new('RGB',(1728,576))
            for col,index in enumerate([0,22,54]): board.paste(frames[index],(col*576,0))
            board.save(REVIEW/(entry['id']+'-native-qa.png'))
    checks.append({'variant':entry['variant'],'atlas':entry['atlas'],'sha256':sha(atlas_path),
                   'pixels_identical_to_review90':True,'bytes':atlas_path.stat().st_size})
    timeline=entry.pop('timeline'); states=[]; known={}; indices=[]
    for state in timeline:
        key=json.dumps(state,sort_keys=True)
        if key not in known:known[key]=len(states);states.append(state)
        indices.append(known[key])
    assert [states[i] for i in indices]==timeline
    entry['states']=states;entry['state_indices']=indices

metadata.update(status='awaiting_visual_review',source_review='skill-animation-review-90/review.json',
    reprise_id='ANIM-S43-REVIEW02',reprise_date='2026-10-15',unchanged_candidate_ids=True,
    unchanged_visual_pixels=True,unchanged_timeline=True,preview='../apercu-animations.html?lot=101',
    remaining_unshown=0,still_unshown=[],
    pending_separately=['D66 taunt animation and marker','O03 animation','B03 mechanics','Control capacity and taunt numbers'])
write('review.json',metadata)
write('validation.json',{'method':'Existing native atlas pixels checked against all original review90 hashes; exact timeline reconstruction and standalone lossless checks.',
    'scenes':len(entries),'native_tile':64,'new_native_capture':False,'source_code_unchanged':True,'checks':checks,'preserved_files':len(preserved)})
text=json.dumps(entries,ensure_ascii=False,separators=(',',':'))
(REVIEW/'animations.json').write_text(text,encoding='utf-8')
template=(ROOT/'tools/art/afflictions_area_review_template.html').read_text(encoding='utf-8')
template=template.replace('Malédictions · marqueurs discrets','Désorientation et Contagion · validation visuelle')
template=template.replace('function states(){return entries().map(e=>e.timeline[frame()]);}',
    'function states(){return entries().map(e=>e.states[e.state_indices[frame()]]);}')
template=template.replace('Ce lot présente deux rendus à valider ; les autres compétences de la branche suivront par lots.',
    'Cette reprise présente les deux rendus d’origine, avec leurs identifiants inchangés, pour clore leur validation visuelle.')
template=template.replace('<p class="note">Les règles de zone',
    '<p class="note">L’aperçu montre les marqueurs, leur transmission et leur disparition. Les attaques entre monstres seront testées lors de l’intégration jouable.</p><p class="note">Les règles de zone')
replacements={'__ENTRIES__':text,'__BATCH__':'ANIM-S43 · reprise de revue 101',
    '__SKILL_IDS__':json.dumps([c['id'] for c in metadata['candidates']]),
    '__VARIANTS__':' et '.join(f"{c['number']} / {c['variant']}" for c in metadata['candidates']),
    '__COMMON_NOTE__':html.escape(metadata['common_note']), '__DETAIL_NOTE__':html.escape(metadata['detail_note']),
    '__BEFORE_PHASE__':json.dumps(metadata['before_phase'],ensure_ascii=False),
    '__START_ACTION_LABEL__':json.dumps(metadata['start_action_label'],ensure_ascii=False),
    '__EXTRA_SCENARIOS__':'<option value="DEGATS">Dégâts et interruption</option><option value="CHAINE">Mort sans retransmission</option>'}
for side,candidate,copy in zip(['LEFT','RIGHT'],metadata['candidates'],metadata['preview_copy']):
    replacements[f'__{side}_TITLE__']=html.escape(f"{candidate['number']} · {candidate['title']}")
    replacements[f'__{side}_COPY__']=html.escape(copy)
for key,value in replacements.items(): template=template.replace(key,value)
assert '__' not in template
(BASE/'apercu-animations.html').write_text(template,encoding='utf-8')
for path,expected in preserved.items(): assert sha(ROOT/path)==expected
write('progress-audit.json',{'all_ranged_visuals_accepted':['DI01','DI02','DI03','DI04','DI05','DI07'],
    'reopened_unchanged_for_visual_review':['95/AF05-A01','96/AF06-A01'],
    'pending_other_visuals':['D66-TAUNT-A01','D66-TAUNT-I01','O03-A01'],
    'campaign_integration':False,'bestiary_after_skill_animations':True})
print(json.dumps({'scenes':len(entries),'preserved_files':len(preserved),'compact_metadata_bytes':len(text.encode('utf-8')),
    'reused_atlas_bytes':sum(c['bytes'] for c in checks)},ensure_ascii=False))
