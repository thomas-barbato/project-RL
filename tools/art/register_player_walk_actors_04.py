"""Finish actor registration against their actual visible source height.

Class gear poses share anatomical registration. Single-loadout actors instead
use their complete source bounds, preventing a tilted head outside the central
band from making a historical figure larger than its current world sprite.
The whole painted object receives one uniform scale; weapons are never sheared.
"""
import json
from PIL import Image
from build_player_walk_review_04 import DOC,ART,review_record,review_front,normalize,pack

data=json.loads((DOC/'baseline-data.json').read_text('utf-8'));records={r['id']:r for r in data['records']}
report=json.loads((DOC/'packaging.json').read_text('utf-8'));changed={}
for entry in report['entries']:
    key=entry['id'];record=records[key]
    if key.startswith(('CLASS-','ARMOR-')) or record['rig'] not in ['biped','heavy','robe'] or entry.get('registration')=='whole-visible-height':continue
    record=review_record(record);front=review_front(records[key]);side=entry['size'];atlas=Image.open(ART/entry['atlas']).convert('RGBA')
    views=[]
    for direction in range(3):
        pose=atlas.crop((8*side,direction*side,9*side,(direction+1)*side))
        views.append(front if direction==1 else normalize(pose,front,side,record['foot'],humanoid=False))
    entry.update(pack(record,[views],entry['labels']));entry['registration']='whole-visible-height';changed[key]=entry
(DOC/'packaging.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),'utf-8')
cachepath=DOC/'package-cache.json';cache=json.loads(cachepath.read_text('utf-8'))
for value in cache.values():value['entries']=[changed.get(e['id'],e) for e in value['entries']]
cachepath.write_text(json.dumps(cache,ensure_ascii=False,indent=2),'utf-8')
print(json.dumps({'actor_registrations':len(changed),'uniform_scaling_only':True,'world_tile':64}))
