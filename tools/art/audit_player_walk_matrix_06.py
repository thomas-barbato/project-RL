"""Read-only game mapping audit and native review evidence for WALK06."""
import hashlib,json,re
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2]
DOC=ROOT/'docs/reviews/player-walk-06'
ART=ROOT/'artifacts/player-walk-review-06'
source=(ROOT/'src/game_art_class_armor.rs').read_text('utf-8')
body=source.split('let rank = match equipment[0] {',1)[1].split('};',1)[0]
groups={}
for pattern,stage in re.findall(r'\s*([^\n]+?)\s*=>\s*([0-4]),',body):
    if pattern.strip()=='_':continue
    values=[]
    if 'None' in pattern:values.append(None)
    match=re.search(r'Some\(([^)]+)\)',pattern)
    if match:
        for item in match.group(1).split('|'):
            item=item.strip()
            if '..=' in item:
                lo,hi=map(int,item.split('..='));values.extend(range(lo,hi+1))
            else:values.append(int(item))
    groups[int(stage)]=values
assert len(groups)==5 and sum(map(len,groups.values()))==20
assert '.filter(|v| (115..=120).contains(v))' in source
report=json.loads((DOC/'packaging.json').read_text('utf-8'))
assert len(report['entries'])==25 and not report['errors'] and not report['pending']
assert not any(entry['warnings'] for entry in report['entries'])
rows=[]
for ci in range(5):
    for stage,models in sorted(groups.items()):
        for torso in models:
            for helmet in [None,*range(115,121)]:
                head=0 if helmet is None else helmet-114
                rows.append({'class_body':ci+3,'torso_model':torso,'helmet_model':helmet,
                    'existing_outfit':512+head*32+ci*5+stage,'review_id':f'CLASS-{ci*5+stage:02}',
                    'rifle_review_prepared':helmet is None,'equipment_integration_verified':False})
assert len(rows)==700 and len({r['existing_outfit'] for r in rows})==175
(DOC/'equipment-mapping-audit.json').write_text(json.dumps({'source':'src/game_art_class_armor.rs::outfit_index',
    'cases':rows,'summary':{'physical_combinations':700,'unique_outfits_with_helmets':175,
    'canonical_silhouettes_without_extra_helmets':25,'no_game_mapping_change':True,
    'helmet_variants_prepared':False,'other_weapon_grips_prepared':False}},ensure_ascii=False,indent=2)+'\n','utf-8')
accepted=json.loads((ROOT/'docs/reviews/player-walk-05/visual-acceptance-01.json').read_text('utf-8'))
for asset in accepted['assets']:
    assert hashlib.sha256((ROOT/asset['path']).read_bytes()).hexdigest()==asset['sha256']
atlas=Image.open(ART/'class-armor-cycles-lossless.webp').convert('RGBA')
walk=Image.open(ROOT/accepted['assets'][0]['path']).convert('RGBA')
idle=Image.open(ROOT/accepted['assets'][1]['path']).convert('RGBA')
for side,row in enumerate((7,1)):
    tiles=[idle.crop((row%4*64,row//4*64,row%4*64+64,row//4*64+64))]
    tiles.extend(walk.crop((f*64,row*64,(f+1)*64,(row+1)*64)) for f in range(6))
    for column,tile in enumerate(tiles):
        packed=atlas.crop((column*64,10*128+side*64,(column+1)*64,10*128+side*64+64))
        for actual,expected in zip(packed.getdata(),tile.getdata()):
            assert actual[3]==expected[3]
            if expected[3]:assert actual==expected
font=ImageFont.truetype('C:/Windows/Fonts/arial.ttf',14)
for theme,bg,line in [('light','#b5afa0','#999689'),('textured','#293132','#414b49')]:
    board=Image.new('RGB',(960,800),bg);draw=ImageDraw.Draw(board)
    for entry in report['entries']:
        ox=entry['stage']*192;oy=entry['class_index']*160
        label_color='#eeeade' if theme=='textured' else '#252c2c'
        draw.text((ox+5,oy+4),f'{entry["number"]:02} - Tenue {entry["stage"]+1}',font=font,fill=label_color)
        if entry['stage']==0:draw.text((ox+5,oy+112),entry['label'].split(' · ')[0],font=font,fill=label_color)
        for x in range(ox,ox+192,64):
            draw.rectangle((x,oy+32,x+63,oy+95),outline=line)
            if theme=='textured':draw.line([(x+13,oy+52),(x+25,oy+59),(x+29,oy+55)],fill='#1c2425')
        for sx,column,side in [(ox,0,0),(ox+64,2,0),(ox+128,5,1)]:
            tile=atlas.crop((column*64,entry['index']*128+side*64,(column+1)*64,entry['index']*128+side*64+64))
            board.paste(tile,(sx,oy+32),tile)
    board.save(ART/f'all25-native-{theme}.png')
result={'review_id':'WALK06-CLASS-ARMOR-MATRIX','prepared_silhouettes':25,
    'native_poses':350,'source_figure_extraction_errors':0,'detached_component_warnings':0,
    'accepted_sources_sha256_unchanged':True,'accepted_visible_rgba_unchanged_in_pack':True,
    'physical_equipment_mapping_cases':700,'unique_equipped_outfits':175,
    'equipment_runtime_verified':False,'visual_acceptance':False}
(DOC/'asset-audit.json').write_text(json.dumps(result,indent=2)+'\n','utf-8')
print(json.dumps(result,indent=2))
