"""Read-only terrain study of installed Diablo II/PD2 tables and compiled bins.

Full third-party tables and reference code stay in ignored artifacts. Only
derived counts, selected facts and provenance are saved in docs/reviews.
"""
import argparse
import csv
import hashlib
import json
import struct
from collections import Counter
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
WORK=ROOT/'artifacts/diablo2-study-2026-10-17'
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/world-generation-study-01'
REVISION='0c0bbd1e6bef9181a3da1c046523af622a925416'
MODES={1:'maze',2:'preset',3:'outdoor'}
FIELDS=['DrlgType','LevelType','SubType','SubTheme','SubWaypoint','SubShrine',
        'Waypoint','Depend','OffsetX','OffsetY',
        'SizeX','SizeY','SizeX(N)','SizeY(N)','SizeX(H)','SizeY(H)']
FIELDS += [f'{prefix}{i}' for prefix in ['Vis','Warp'] for i in range(8)]
EXAMPLES=[1,2,3,8,25,28,35,37,40,46,67,75,83,102,103,108,109,110,129,131,132]

def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path): return json.loads(path.read_text(encoding='utf-8'))
def rows(game,name):
    return list(csv.DictReader((WORK/'tables'/game/f'{name}.txt').read_text(encoding='cp1252').splitlines(),delimiter='\t'))
def number(row,key): return int(row.get(key) or 0)

def analyze(game):
    levels=[r for r in rows(game,'Levels') if r['Id'].isdigit() and 1<=int(r['Id'])<=132 and r['Id']!='39']
    assert len(levels)==131
    definitions=(WORK/'tables'/game/'LevelDefs.bin').read_bytes()
    compiled=(WORK/'tables'/game/'Levels.bin').read_bytes()
    count=struct.unpack_from('<I',definitions)[0]
    assert len(definitions)==4+count*156
    assert len(compiled)==4+count*544
    # Exact offsets from pinned D2MOO LevelsTbls.h and its compiler descriptors.
    scalar_offsets={'OffsetX':0x24,'OffsetY':0x28,'Depend':0x2c,'DrlgType':0x30,
        'LevelType':0x34,'SubType':0x38,'SubTheme':0x3c,'SubWaypoint':0x40,'SubShrine':0x44}
    checked=0
    for row in levels:
        at=4+number(row,'Id')*156
        for field,offset in scalar_offsets.items():
            assert struct.unpack_from('<I',definitions,at+offset)[0]==number(row,field)&0xffffffff,(game,row['Id'],field)
            checked+=1
        for prefix,offset in [('SizeX',0x0c),('SizeY',0x18)]:
            values=tuple(number(row,prefix+s)&0xffffffff for s in ['', '(N)', '(H)'])
            assert struct.unpack_from('<3I',definitions,at+offset)==values
            checked+=3
        for prefix,offset in [('Vis',0x48),('Warp',0x68)]:
            assert struct.unpack_from('<8I',definitions,at+offset)==tuple(number(row,f'{prefix}{i}')&0xffffffff for i in range(8))
            checked+=8
        assert compiled[4+number(row,'Id')*544+0xe4]==number(row,'Waypoint')
        checked+=1
    maze=[r for r in rows(game,'LvlMaze') if r['Level'].isdigit()]
    maze_bin=(WORK/'tables'/game/'LvlMaze.bin').read_bytes()
    assert struct.unpack_from('<I',maze_bin)[0]==len(maze)
    assert len(maze_bin)==4+len(maze)*28
    for index,row in enumerate(maze):
        assert struct.unpack_from('<7I',maze_bin,4+index*28)==tuple(number(row,k)&0xffffffff for k in ['Level','Rooms','Rooms(N)','Rooms(H)','SizeX','SizeY','Merge'])
    presets=[r for r in rows(game,'LvlPrest') if r['Def'].isdigit()]
    preset_bin=(WORK/'tables'/game/'LvlPrest.bin').read_bytes()
    assert len(preset_bin)==4+len(presets)*432
    assert struct.unpack_from('<I',preset_bin)[0]==len(presets)
    modes=Counter(MODES[number(r,'DrlgType')] for r in levels)
    examples=[]
    for row in levels:
        if number(row,'Id') not in EXAMPLES:continue
        item={k:row[k] for k in ['Id','Name','LevelName','DrlgType','Waypoint']}
        item['generator']=MODES[number(row,'DrlgType')]
        item['sizes_by_difficulty']=[[number(row,'SizeX'+s),number(row,'SizeY'+s)] for s in ['','(N)','(H)']]
        item['declared_passages']=[dict(slot=i,visible_level=number(row,f'Vis{i}'),warp=number(row,f'Warp{i}')) for i in range(8) if number(row,f'Vis{i}') or number(row,f'Warp{i}')>=0]
        matching=[m for m in maze if number(m,'Level')==number(row,'Id')]
        if matching:
            item['maze_parameters']={k:number(matching[0],k) for k in ['Rooms','Rooms(N)','Rooms(H)','SizeX','SizeY','Merge']}
        preset=[p for p in presets if number(p,'LevelId')==number(row,'Id')]
        if preset:item['preset_definitions']=[dict(definition=number(p,'Def'),files=number(p,'Files')) for p in preset]
        examples.append(item)
    return dict(campaign_rows=131,compiled_definition_rows=count,generator_modes=dict(sorted(modes.items())),
        waypoint_rows=sum(number(r,'Waypoint')!=255 for r in levels),
        structural_field_comparisons_against_bin=checked,maze_parameter_rows_verified=len(maze),
        preset_definition_rows_verified=len(presets),level_type_rows=sum(r['Id'].isdigit() for r in rows(game,'LvlTypes')),
        examples=examples),{r['Id']:r for r in levels}

def main(check):
    original=read(ROOT/'docs/reviews/refonte-histoire-2026-10-16/diablo2-study-2026-10-17/counts.json')
    for archive in original['archive_hashes']:
        assert sha(Path(archive['path']))==archive['sha256'],archive['path']
    analyzed={}; raw={}
    for game in ['LoD','PD2']:analyzed[game],raw[game]=analyze(game)
    differences=[dict(level=int(n),field=k,lod=number(r,k),pd2=number(raw['PD2'][n],k)) for n,r in raw['LoD'].items() for k in FIELDS if number(r,k)!=number(raw['PD2'][n],k)]
    references=[]
    for filename in ['DrlgDrlg.cpp','DrlgMaze.cpp','DrlgPreset.cpp','DrlgOutdoors.cpp','DrlgOutRoom.cpp','DrlgDrlgWarp.cpp','D2DrlgDrlg.h','LevelsTbls.h','LevelsTbls.cpp']:
        local=WORK/'world-code'/filename
        relative=('source/D2Common/include/Drlg/' if filename=='D2DrlgDrlg.h' else 'source/D2Common/include/DataTbls/' if filename=='LevelsTbls.h' else 'source/D2Common/src/DataTbls/' if filename=='LevelsTbls.cpp' else 'source/D2Common/src/Drlg/')+filename
        references.append(dict(file=local.relative_to(ROOT).as_posix(),sha256=sha(local),url=f'https://github.com/ThePhrozenKeep/D2MOO/blob/{REVISION}/{relative}'))
    result=dict(review_id='MONDE-PM01-01',date='2026-10-17',scope='131 campaign table rows per archive, Ids 1..132 except bonus Cow Level 39; not all playable map layouts.',
        archives_unchanged_from_encounter_study=True,archive_hashes=original['archive_hashes'],
        input_tables=[dict(file=p.relative_to(ROOT).as_posix(),sha256=sha(p)) for game in ['LoD','PD2'] for p in sorted((WORK/'tables'/game).glob('*')) if p.stem in ['Levels','LevelDefs','LvlMaze','LvlPrest','LvlTypes','LvlSub']],
        lod=analyzed['LoD'],pd2=analyzed['PD2'],compared_structural_fields=FIELDS,campaign_field_differences=differences,
        reference_revision=REVISION,reference_sources=references,
        caveats=['D2MOO is a 1.10f reverse-engineered reference, not PD2 closed engine source.',
                 'Equal structural fields do not prove identical maps, DS1 assets, quest scripts or generation algorithms.',
                 'Room parameters are declarations; special cases add, reshape or multiply rooms.',
                 'Preset definition count is not a count of distinct playable maps.',
                 'Vis/Warp table entries do not fully describe all hardcoded outdoor links or quest transitions.'])
    validation=dict(archives_unchanged=True,game_not_launched=True,saves_not_accessed=True,
        compiled_structural_fields_checked_per_archive=analyzed['LoD']['structural_field_comparisons_against_bin'],
        structural_fields_match_compiled_bin=True,maze_rows_match_compiled_bin_per_archive=81,
        preset_counts_match_compiled_bin=True,reference_sources_pinned_and_hashed=True,
        game_code_content_and_saves_changed=False,design_proposals_integrated=False)
    REVIEW.mkdir(parents=True,exist_ok=True)
    for name,data in [('counts.json',result),('validation.json',validation)]:
        path=REVIEW/name
        if check:assert read(path)==data,path
        else:path.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(lod_modes=analyzed['LoD']['generator_modes'],pd2_modes=analyzed['PD2']['generator_modes'],
        campaign_field_differences=len(differences),verified_bin_fields_per_archive=validation['compiled_structural_fields_checked_per_archive'],
        preset_definitions=[analyzed[g]['preset_definition_rows_verified'] for g in ['LoD','PD2']],
        game_code_content_and_saves_changed=False)))

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--check',action='store_true')
    main(parser.parse_args().check)
