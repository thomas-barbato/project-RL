"""Read-only analysis of tables extracted from the user's installed MPQs.

Raw third-party tables stay in ignored artifacts. Durable output contains
derived counts and selected examples, not a replacement Diablo database.
"""
import csv
import hashlib
import json
import re
import struct
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'artifacts/diablo2-study-2026-10-17'
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/diablo2-study-2026-10-17'


def rows(game, name):
    text = (WORK / 'tables' / game / f'{name}.txt').read_text(encoding='cp1252')
    return list(csv.DictReader(text.splitlines(), delimiter='\t'))


def slots(row, prefix):
    return [value for key, value in row.items()
            if key and re.fullmatch(re.escape(prefix) + r'\d+', key) and value]


def number(row, key):
    return int(row.get(key) or 0)


def analyze(game):
    levels = [r for r in rows(game, 'Levels') if r['Id'].isdigit()]
    stat_rows = [r for r in rows(game, 'MonStats') if r['Id'] != 'Expansion']
    stats = {r['Id']: r for r in stat_rows if r['Id']}
    # Compiled references use physical table order. A dictionary keyed by Id
    # loses earlier records when textual identifiers are duplicated.
    indices = {index: r['Id'] for index, r in enumerate(stat_rows)}
    compiled = (WORK / 'tables' / game / 'Levels.bin').read_bytes()
    record_count = struct.unpack_from('<I', compiled)[0]
    assert len(compiled) == 4 + record_count * 544
    campaign = [r for r in levels if 1 <= number(r, 'Id') <= 132 and number(r, 'Id') != 39]
    monster_bin = (WORK / 'tables' / game / 'MonStats.bin').read_bytes()
    monster_count = struct.unpack_from('<I', monster_bin)[0]
    assert monster_count == len(stat_rows)
    assert len(monster_bin) == 4 + monster_count * 424
    used_ids = {value for row in campaign for prefix in ['mon','nmon','umon'] for value in slots(row,prefix)}
    checked_monsters = 0
    for index, r in enumerate(stat_rows):
        if r['Id'] not in used_ids:
            continue
        base_index = struct.unpack_from('<h', monster_bin, 4 + index * 424 + 2)[0]
        assert indices[base_index] == r['BaseId'], (game, r['Id'], 'BaseId')
        checked_monsters += 1
    checked = []
    for r in campaign:
        offset = 4 + number(r, 'Id') * 544
        assert compiled[offset + 0x32] == number(r, 'NumMon'), r['Id']
        assert struct.unpack_from('<3I', compiled, offset + 0x1c) == tuple(
            number(r, k) for k in ['MonDen', 'MonDen(N)', 'MonDen(H)']), r['Id']
        for prefix, start in [('mon', 0x36), ('nmon', 0x68), ('umon', 0x9a)]:
            ids = [indices[n] for n in struct.unpack_from('<25h', compiled, offset + start) if n >= 0]
            assert ids == slots(r, prefix), (game, r['Id'], prefix)
        checked.append(number(r, 'Id'))
    result = []
    previously_seen = set()
    for act in range(5):
        zones = [r for r in campaign if number(r, 'Act') == act]
        modes = {}
        for mode, prefix in [('normal', 'mon'), ('nightmare_hell', 'nmon')]:
            ids = {v for r in zones for v in slots(r, prefix)}
            assert ids <= stats.keys(), ids - stats.keys()
            families = {stats[i]['BaseId'] or i for i in ids}
            closure = set(ids)
            while True:
                added = {stats[i].get(k) for i in closure for k in ['minion1', 'minion2', 'spawn']}
                added = (added & stats.keys()) - closure
                if not added:
                    break
                closure |= added
            modes[mode] = dict(variant_ids=len(ids), technical_baseids=len(families),
                               baseids=sorted(families),
                               plus_declared_minions_spawns=len({stats[i]['BaseId'] or i for i in closure}))
            if mode == 'normal':
                modes[mode]['baseids_first_seen_in_this_act'] = len(families - previously_seen)
                previously_seen |= families
        result.append(dict(act=act+1, area_rows=len(zones), modes=modes,
                           num_mon_distribution=dict(Counter(number(r, 'NumMon') for r in zones if r['NumMon'])),
                           zones_over_three=[dict(id=number(r, 'Id'), name=r['Name'], num_mon=number(r,'NumMon')) for r in zones if number(r,'NumMon') > 3]))
    examples = []
    for rid in [2, 3, 4, 8, 67, 104, 110, 128, 129, 130, 131]:
        r = next(x for x in levels if number(x,'Id') == rid)
        examples.append(dict(id=rid, name=r['Name'], level_name=r['LevelName'],
                             normal_pool=slots(r,'mon'), nightmare_hell_pool=slots(r,'nmon'),
                             num_mon=number(r,'NumMon'), density=number(r,'MonDen'),
                             unique_min=number(r,'MonUMin'), unique_max=number(r,'MonUMax'),
                             ranged_spawn=number(r,'rangedspawn'), warp_dist=number(r,'WarpDist')))
    companions = {i:{k:stats[i].get(k) for k in ['NameStr','AI','MinGrp','MaxGrp','PartyMin','PartyMax','minion1','minion2','spawn','Rarity']}
                  for i in ['fallen1','fallenshaman1','quillrat1','crownest1','unraveler1','scarab1']}
    return dict(compiled_area_count=record_count, campaign_rows_verified_against_bin=len(checked),
                monster_baseids_verified_against_bin=checked_monsters,
                scope='Ids 1..132 except bonus Cow Level 39; potential random pools only. Includes revisited families. Excludes fixed bosses, preset placements, quest waves and critters.',
                acts=result, examples=examples, group_examples=companions)


def main():
    REVIEW.mkdir(parents=True, exist_ok=True)
    source_dir = Path(r'D:\JEUX\Diablo 2 old\Diablo II')
    inputs = [source_dir/'ProjectD2/pd2data.mpq', source_dir/'patch_d2.mpq']
    manifest = [dict(path=str(p),size=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in inputs]
    result = dict(date='2026-10-17', installation=str(source_dir),
                  method='Installed SFMPQ.dll opened archives with read APIs in 32-bit PowerShell. No write APIs, game launch or save access.',
                  archive_hashes=manifest, LoD=analyze('LoD'), PD2=analyze('PD2'),
                  engine_reference='D2MOO targets LoD 1.10f; used for architecture, not claimed to reproduce PD2 closed engine exactly.',
                  not_observed='No live generated map or gameplay session was inspected.')
    (REVIEW/'counts.json').write_text(json.dumps(result, ensure_ascii=False, indent=2)+'\n',encoding='utf-8')
    for game in ['LoD','PD2']:
        print(game, 'verified campaign areas',result[game]['campaign_rows_verified_against_bin'])
        for act in result[game]['acts']:
            m=act['modes']['normal']
            print(act['act'], 'variants',m['variant_ids'],'baseids',m['technical_baseids'],'first',m['baseids_first_seen_in_this_act'],'including spawn/minions',m['plus_declared_minions_spawns'],act['num_mon_distribution'])


if __name__ == '__main__':
    main()
