"""ANIM-S51: lossless native shots, compact timelines and the one shared page."""
import argparse
import hashlib
import json
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'artifacts/refonte-agent-2026-10-06'
REVIEW = BASE / 'skill-animation-review-100'


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def write(name, data):
    (REVIEW / name).write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')




def prepare():
    REVIEW.mkdir(exist_ok=True)
    assert not (REVIEW / 'preservation.json').exists()
    paths=list(json.loads((BASE/'skill-animation-review-99/preservation.json').read_text(encoding='utf-8')))
    paths += [
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-99/visual-acceptance-01.json',
        'src/agent_skill_lab_review/fire_painted/ranged_fan_precision.rs',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-99/DI03-physical-shots-64.webp',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-99/DI04-physical-shots-64.webp',
        'assets/skills/agent/DI05-compact.png','assets/skills/agent/DI07-compact.png',
        'artifacts/refonte-agent-2026-10-06/skill-animation-review-77/regeneration-source.png',
    ]
    paths=list(dict.fromkeys(paths));write('preservation.json',{p:sha(ROOT/p) for p in paths})
    write('review.json',{
        'date':'2026-10-15','review_id':'ANIM-S51','status':'visual_proposals_not_accepted',
        'candidates':[{'number':113,'id':'DI05','variant':'DI05-A01','title':'Tir de harpon'},
                      {'number':114,'id':'DI07','variant':'DI07-A01','title':'Tir vampirique'}],
        'native_tile':64,'native_lab':'--apercu-laboratoire-competences --lot-harpon-vampirique',
        'source':'src/agent_skill_lab_review/fire_painted/ranged_harpoon_vampiric.rs',
        'source_texts':['review-individual-description-DI05-02.json','review-individual-description-DI07-02.json'],
        'scope':'Prepared finite visual fixtures, not campaign effects or new balance approval.',
        'fixture_rules':['DI05:one rapid barbed projectile, short steel cable, three-cell straight pull after successful hit',
            'Anchored target and prepared elite resistance keep shot damage without displacement',
            'DI07:one ordinary physical shot, green agent healing pulse and actual +X PV label',
            'Rank11 life return50% of actual primary HP removed, shared cast cap10% maxHP, missing HP limit',
            'Exact even prepared impacts avoid deciding rounding; maxHP100 is a scene fixture',
            'Current accepted spatial growth: example active normal weapon6 plus3 range cells',
            'Active normal ranged weapon required, heavy weapons excluded; no extra preparation turn',
            'Preview disappears at confirmation, finite impact, accepted physical blood and shared hit reaction'],
        'fixture_hit_values':{'DI05':31,'DI07':36,'reduced_after_defence_either_skill':12,'maximum_target_hp':240,
            'player_max_hp':100,'player_start_hp':40,'almost_full_start_hp':98},
        'not_new_rules':['No new critical, variance, defence, RNG or elite chance law',
            'Prepared elite failure does not replace accepted conditional75% traction chance',
            'Only a straight pull is illustrated; diagonals, ties and general actor interception remain separate',
            'No overkill, shield, machine healing, full-health cast policy or early target death is arbitrated',
            'No healing from indirect/reflected damage, no persistent status and no collision bonus introduced'],
        'active_catalogue_assets_campaign_rng_or_saves_changed':False,
        'pending_separate':['D66 taunt animation and marker','O03 animation','AF05/AF06 review90',
                            'B03 mechanics','Control capacity and taunt numbers'],
    })
    print(json.dumps({'prepared':str(REVIEW),'preserved':len(paths)}))

def verify_timeline(e):
    t=e['timeline']; initial=t[0]; last=t[-1]; harpoon=e['id']=='DI05'
    refused=e['case'] in ['CLOISON','MUNITIONS','ENERGIE','MELEE']
    if refused:
        assert all(s==initial for s in t),e['variant']
        return
    assert all(s['preview'] and not s['started'] for s in t[:18])
    assert all(not s['preview'] and s['started'] for s in t[18:])
    assert all(s['energy']==(87 if harpoon else 88) for s in t[18:])
    assert last['projectiles']==1 and last['ammo']==19
    damage=0 if e['case']=='MANQUE' else 12 if e['case']=='REDUIT' else 31 if harpoon else 36
    assert last['damage']==[damage,0,0],(e['variant'],last['damage'])
    pulled=3 if harpoon and e['case'] not in ['MANQUE','ANCRE','ELITE'] else 0
    assert last['target_cell']==[4,2+pulled] and last['pulled_cells']==pulled
    heal=0 if harpoon or damage==0 else 2 if e['case']=='PV_MANQUANTS' else 6 if damage==12 else 10
    assert last['healed']==heal
    assert last['player_hp']==(98 if e['case']=='PV_MANQUANTS' else 40)+heal
    assert last['reaction_events']==last['blood_events']==last['hits']==int(damage>0)
    assert last['misses']==int(e['case']=='MANQUE')
    assert all(s['damage']==last['damage'] and s['ammo']==last['ammo'] and s['player_hp']==last['player_hp'] for s in t[60:])
    assert (last['turn'],last['cooldown'])==((1,6 if harpoon else 8) if e['case']=='ATTENTE' else (9,0))
    assert last['weapon_range_fixture']==9 and not last['persistent_status_created']

def package():
    entries = json.loads((REVIEW / '_capture/capture.json').read_text(encoding='utf-8'))
    assert len(entries) == 28
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
        e['atlas'] = 'skill-animation-review-100/' + dest.name
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
    template = (ROOT / 'tools/art/ranged_harpoon_vampiric_review_template.html').read_text(encoding='utf-8')
    (BASE / 'apercu-animations.html').write_text(template.replace('__ENTRIES__', text), encoding='utf-8')
    write('validation.json', {'method': 'Native Macroquad capture and exact lossless decoded-pixel checks; no browser render verification.',
        'scenes': 28, 'native_tile': 64, 'frames': 120, 'fps': 30, 'directions': 4,
        'preview_removed_on_confirmation': True, 'walls_stop_shot': True, 'three_cell_prepared_pull': True,
        'one_shot_per_skill': True, 'finite_tether': True, 'green_heal_pulse':True, 'actual_heal_label':True, 'no_repeat_from_wait_or_later_turns': True,
        'no_machine_heal_policy_inferred': True, 'heal_cap_and_missing_hp': True, 'shared_hit_reaction': True, 'costs_once_in_isolated_fixture': True,
        'refusals_atomic_in_fixture': True, 'no_early_death_rules_inferred': True,
        'previous_accepted_assets_preserved': len(preservation), 'checks': checks})
    write('progress-audit.json', {'previous_unshown': 2, 'newly_shown': ['DI05', 'DI07'],
        'remaining_unshown': 0, 'still_unshown': [],
        'new_visuals_status': 'awaiting_user_review', 'campaign_integration': False})
    raw_files = sorted(p for p in (REVIEW / '_capture').rglob('*') if p.is_file())
    assert len(raw_files) == 29
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




