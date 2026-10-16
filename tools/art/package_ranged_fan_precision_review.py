"""ANIM-S50: lossless native shots, compact timelines and the one shared page."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-99'


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def write(name, data):
    (REVIEW / name).write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')



def prepare():
    REVIEW.mkdir(exist_ok=True)
    assert not (REVIEW / 'preservation.json').exists()
    paths = list(json.loads((BASE / 'skill-animation-review-98/preservation.json').read_text(encoding='utf-8')))
    paths += [
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-98/visual-acceptance-01.json',
        'src/agent_skill_lab_review/fire_painted/ranged_opening.rs',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-98/DI01-physical-shots-64.webp',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-98/DI02-physical-shots-64.webp',
        'assets/skills/agent/DI03-compact.png', 'assets/skills/agent/DI04-compact.png',
    ]
    paths = list(dict.fromkeys(paths))
    write('preservation.json', {p: sha(ROOT / p) for p in paths})
    write('review.json', {
        'date': '2026-10-15', 'review_id': 'ANIM-S50', 'status': 'visual_proposals_not_accepted',
        'candidates': [{'number': 111, 'id': 'DI03', 'variant': 'DI03-A01', 'title': 'Volée'},
                       {'number': 112, 'id': 'DI04', 'variant': 'DI04-A01', 'title': 'Tir de précision'}],
        'native_tile': 64, 'native_lab': '--apercu-laboratoire-competences --lot-volee-precision',
        'source': 'src/agent_skill_lab_review/fire_painted/ranged_fan_precision.rs',
        'source_texts': ['review-individual-description-DI03-02.json', 'review-individual-description-DI04-03.json'],
        'scope': 'Prepared visual fixtures, not campaign effects or new balance approval.',
        'fixture_rules': ['DI03:90-degree cone, one projectile per selected target, five nearest eligible targets at effective rank11',
            'DI04:one visible target, short red reticle, one fast finite shot, heavy weapons excluded in accepted specification',
            'Current accepted spatial growth gives the example weapon9 cells and the volley cone6 cells; supersedes old card range example',
            'Fixed sample impacts, no RNG, no new weapon variance, critical multiplier or tie policy',
            'Blinking preview removed on launch; shared hit reaction and physical blood only on living targets'],
        'fixture_hit_values': {'DI03_per_target':29,'DI04_noncritical':22,'maximum_target_hp':240},
        'not_new_rules': ['Prepared misses do not establish an accuracy probability or guaranteed hit',
            'No target dies during the demonstration; early-death rules remain pending',
            'Cosmetic flight and brief reticle do not add a preparation turn',
            'A volley does not inherit magical area damage falloff'],
        'active_catalogue_assets_campaign_rng_or_saves_changed': False,
        'pending_separate': ['D66 taunt animation and marker','O03 animation','AF05/AF06 review90',
                             'B03 mechanics','Control capacity and taunt numbers'],
    })
    print(json.dumps({'prepared':str(REVIEW),'preserved':len(paths)}))

def verify_timeline(e):
    t=e['timeline']; initial=t[0]; last=t[-1]; volley=e['id']=='DI03'
    refused=e['case'] in ['VIDE','MUNITIONS','ENERGIE','MELEE'] or (not volley and e['case']=='CLOISON')
    if refused:
        assert all(s==initial for s in t),e['variant']
        return
    assert all(s['preview'] and not s['started'] for s in t[:18])
    assert all(not s['preview'] and s['started'] for s in t[18:])
    assert all(s['energy']==(85 if volley else 89) for s in t[18:])
    count=(4 if e['case']=='CLOISON' else 5) if volley else 1
    assert last['projectiles']==count and last['ammo']==20-count
    expected=[29,29,29,29,29,0,0] if volley else [22,0,0,0,0,0,0]
    if e['case']=='CLOISON': expected=[29,29,0,29,29,0,0]
    if e['case']=='MANQUE': expected[2 if volley else 0]=0
    assert last['damage']==expected,(e['variant'],last['damage'])
    assert last['reaction_events']==last['hits']
    assert last['blood_events']==(0 if e['case']=='MACHINE' else last['hits'])
    assert all(s['damage']==last['damage'] and s['ammo']==last['ammo'] for s in t[60:])
    assert (last['turn'],last['cooldown'])==((1,6 if volley else 4) if e['case']=='ATTENTE' else (7,0))
    assert last['weapon_range_fixture']==9 and last['cone_depth_fixture']==(6 if volley else 0)

def package():
    entries = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 26
    checks = []
    for e in entries:
        verify_timeline(e)
        count, cols = e['unique_images'], e['columns']; rows = (count + cols - 1) // cols
        with Image.open(REVIEW / '_capture' / e['folder'] / 'atlas.png') as raw:
            atlas = Image.new('RGB', raw.size); hashes = []
            for i in range(count):
                x, y = i % cols * 576, (rows - 1 - i // cols) * 576
                frame = raw.crop((x, y, x + 576, y + 576)).convert('RGB')
                atlas.paste(frame, (x, i // cols * 576))
                hashes.append(hashlib.sha256(frame.tobytes()).hexdigest())
        dest = REVIEW / (e['folder'] + '.webp'); atlas.save(dest, 'WEBP', lossless=True, method=1)
        with Image.open(dest) as packed:
            for i, h in enumerate(hashes):
                x, y = i % cols * 576, i // cols * 576
                assert hashlib.sha256(packed.crop((x, y, x + 576, y + 576)).convert('RGB').tobytes()).hexdigest() == h
            if e['label'] == 'HAUT':
                frames = []
                for i in e['indices']:
                    x, y = i % cols * 576, i // cols * 576
                    frames.append(packed.crop((x, y, x + 576, y + 576)).copy())
                standalone = REVIEW / (e['id'] + '-physical-shots-64.webp')
                frames[0].save(standalone, 'WEBP', save_all=True, append_images=frames[1:],
                    duration=[33 if i % 3 != 2 else 34 for i in range(len(frames))], loop=0, lossless=True, method=1)
                with Image.open(standalone) as animated:
                    elapsed = 0
                    for i in range(animated.n_frames):
                        animated.seek(i); animated.load()
                        n = min(e['frames'] - 1, (elapsed * 30 + 500) // 1000)
                        j = e['indices'][n]; x, y = j % cols * 576, j // cols * 576
                        assert animated.convert('RGB').tobytes() == packed.crop((x, y, x + 576, y + 576)).convert('RGB').tobytes()
                        elapsed += animated.info['duration']
                    assert elapsed == 4000
                board = Image.new('RGB', (1728, 576))
                for col, n in enumerate([18, 22, 28]): board.paste(frames[n], (col * 576, 0))
                board.save(REVIEW / (e['id'] + '-native-qa.png'))
        e['atlas'] = 'skill-animation-review-99/' + dest.name
        # Keep distinct states, not 120 redundant copies of each snapshot.
        states = []; known = {}; state_indices = []
        for state in e.pop('timeline'):
            key = json.dumps(state, sort_keys=True)
            if key not in known: known[key] = len(states); states.append(state)
            state_indices.append(known[key])
        e['states'] = states; e['state_indices'] = state_indices
        checks.append({'variant': e['variant'], 'path': dest.name, 'sha256': sha(dest),
                       'bytes': dest.stat().st_size, 'lossless_pixels': True, 'unique_frames': count})
        print(e['variant'], dest.stat().st_size, flush=True)
    preservation = json.loads((REVIEW / 'preservation.json').read_text(encoding='utf-8'))
    for p, h in preservation.items(): assert sha(ROOT / p) == h, p
    text = json.dumps(entries, ensure_ascii=False, separators=(',', ':'))
    (REVIEW / 'animations.json').write_text(text, encoding='utf-8')
    template = (ROOT / 'tools/art/ranged_fan_precision_review_template.html').read_text(encoding='utf-8')
    (BASE / 'apercu-animations.html').write_text(template.replace('__ENTRIES__', text), encoding='utf-8')
    write('validation.json', {'method': 'Native Macroquad capture and exact lossless decoded-pixel checks; no browser render verification.',
        'scenes': 26, 'native_tile': 64, 'frames': 120, 'fps': 30, 'directions': 4,
        'preview_removed_on_confirmation': True, 'walls_stop_shot': True, 'five_target_cap': True,
        'one_projectile_per_selected_target': True, 'single_precision_shot':True, 'brief_red_reticle':True, 'no_repeat_from_wait_or_later_turns': True,
        'machine_no_blood': True, 'shared_hit_reaction': True, 'costs_once_in_isolated_fixture': True,
        'refusals_atomic_in_fixture': True, 'no_early_death_rules_inferred': True,
        'previous_accepted_assets_preserved': len(preservation), 'checks': checks})
    write('progress-audit.json', {'previous_unshown': 4, 'newly_shown': ['DI03', 'DI04'],
        'remaining_unshown': 2, 'still_unshown': ['DI05', 'DI07'],
        'new_visuals_status': 'awaiting_user_review', 'campaign_integration': False})
    raw_files = sorted(p for p in (REVIEW / '_capture').rglob('*') if p.is_file())
    assert len(raw_files) == 27
    write('cleanup-plan.json', {'files': [
        {'path': p.relative_to(REVIEW / '_capture').as_posix(), 'sha256': sha(p), 'bytes': p.stat().st_size}
        for p in raw_files], 'bytes': sum(p.stat().st_size for p in raw_files),
        'reason': 'Raw capture data already decoded and verified in compact lossless previews.'})
    print('Atlas total MiB', round(sum(c['bytes'] for c in checks) / 2**20, 2))


def main():
    parser = argparse.ArgumentParser(description=__doc__); parser.add_argument('mode', choices=['prepare', 'package']); args = parser.parse_args()
    assert not (REVIEW / 'visual-acceptance-01.json').exists(), 'Do not overwrite an accepted review.'
    prepare() if args.mode == 'prepare' else package()


if __name__ == '__main__': main()


