"""Install reviewed names, fixed attributes and starter equipment definitions."""
from pathlib import Path
import json
ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'artifacts/refonte-agent-2026-10-06'
profiles=json.loads((BASE/'review-class-starts-START-ALL01-11.json').read_text('utf-8'))['profiles']
slugs=['bastion','technomancien','traqueur','arcaniste','metamorphe']
roles=['Mêlée et survie','Drones et affaiblissements','Tir et esquive','Magie de dégâts','Transformations']
keys=['power','coordination','resilience','perception','processing','willpower']
bonuses={
'bastion':dict(maximum_hp_percent=110,maximum_energy_percent=90,armor=1),
'technomancien':dict(weapon_damage_percent=90,drone_hp_percent=115,drone_damage_percent=125,weakening_duration_bonus=1),
'traqueur':dict(ranged_damage_percent=120,melee_damage_percent=90,evasion=2),
'arcaniste':dict(maximum_hp_percent=90,spell_damage_percent=115,magic_energy_percent=80),
'metamorphe':dict(transformed_damage_percent=110,transformation_duration_bonus=2,human_skill_energy_percent=110),
}
translations={}
for i,(slug,p) in enumerate(zip(slugs,profiles)):
    key=f'class.{slug}'; equip=p['starting_equipment']; torso=equip['body_armor']; armor=torso['draft_id'].replace('start:','core:')
    start={k:100 for k in ['maximum_hp_percent','maximum_energy_percent','weapon_damage_percent','melee_damage_percent','ranged_damage_percent','spell_damage_percent','magic_energy_percent','drone_hp_percent','drone_damage_percent','transformed_damage_percent','human_skill_energy_percent']}
    start.update(armor=0,evasion=0,transformation_duration_bonus=0,weakening_duration_bonus=0,skills=[s['skill_id'] for s in p['learned_start_skills']]);start.update(bonuses[slug])
    weapon=equip['active_weapon']['reference_model']
    data=dict(id=f'core:{slug}',name_key=key+'.name',role_key=key+'.role',description_key=key+'.description',recommended_attributes=dict(zip(keys,p['attributes'].values())),starting_weapons=[weapon],starting_items=[dict(item=armor,quantity=1),dict(item='core:agent_dose_soin',quantity=2),dict(item='core:agent_cellule_energie',quantity=1)],starting_equipment=[weapon,None,None],agent_start=start)
    (ROOT/f'content/core/classes/{slug}.json5').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf-8')
    translations.update({key+'.name':p['proposed_display_name'],key+'.role':roles[i],key+'.description':p['roleplay']})
    itemkey='item.'+armor.split(':')[1]
    data=dict(id=armor,name_key=itemkey+'.name',description_key=itemkey+'.name',maximum_stack=1,kind='armor',equipment=dict(slot='core:body_armor',armor=torso['armor'],evasion_penalty=torso['evasion_penalty']))
    (ROOT/f'content/core/items/{armor.split(":")[1]}.json5').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf-8')
    translations[itemkey+'.name']=torso['display_name']
for slug,name,effect in [('agent_dose_soin','Dose de soin','restore_integrity'),('agent_cellule_energie','Cellule d’énergie','restore_energy')]:
    key=f'item.{slug}'
    data=dict(id=f'core:{slug}',name_key=key+'.name',description_key=key+'.name',maximum_stack=1,kind='consumable',effects=[dict(type=effect,amount=12)])
    (ROOT/f'content/core/items/{slug}.json5').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n','utf-8')
    translations[key+'.name']=name
path=ROOT/'content/core/locales/fr.json5';text=path.read_text('utf-8');marker='    "class.breche.name"'
new=''.join('    '+json.dumps(k,ensure_ascii=False)+': '+json.dumps(v,ensure_ascii=False)+',\n' for k,v in translations.items())
if '"class.bastion.name"' not in text: text=text.replace(marker,new+marker)
path.write_text(text,'utf-8')
print('Five reviewed starts packaged.')
