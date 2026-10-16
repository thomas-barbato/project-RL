"""Prepare a complete, per-object comic art trial; never edit runtime assets."""
from pathlib import Path
import hashlib,json,re
from PIL import Image,ImageDraw,ImageFont

ROOT=Path(__file__).resolve().parents[2];BASE=Path(__file__).resolve().parent
OUT=BASE/'comic-catalogue-review-01';ART=ROOT/'assets/sprites/cyberpunk'
STYLE=BASE/'comic-style-review-01/sources/BD-C01.png'

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def rel(p):return p.relative_to(ROOT).as_posix()
def cell(atlas,index,side,cols=16):
    x=index%cols*side;y=index//cols*side
    return atlas.crop((x,y,x+side,y+side)).convert('RGBA')
def store_ref(cid,im,body=False):
    # Isolate existing sheet content; preserve both members of glove/boot pairs.
    alpha=im.getchannel('A').point(lambda a:255 if a>=128 else 0)
    remaining={i for i,a in enumerate(alpha.tobytes()) if a};groups=[];w,h=im.size
    while remaining:
        seed=min(remaining);remaining.remove(seed);todo=[seed];group=[seed]
        while todo:
            i=todo.pop();x,y=i%w,i//w
            for yy in range(max(0,y-1),min(h,y+2)):
                for xx in range(max(0,x-1),min(w,x+2)):
                    k=yy*w+xx
                    if k in remaining:remaining.remove(k);todo.append(k);group.append(k)
        groups.append(group)
    if groups:
        largest=max(map(len,groups));keep={i for g in groups if len(g)>=max(5,largest*.15) for i in g}
        mask=bytearray(w*h)
        for i in keep:mask[i]=255
        im=im.copy();im.putalpha(Image.frombytes('L',im.size,bytes(mask)))
    if not body and im.getchannel('A').getbbox():im=im.crop(im.getchannel('A').getbbox())
    scale=max(1,512/max(im.size));im=im.resize((round(im.width*scale),round(im.height*scale)),Image.Resampling.NEAREST)
    p=OUT/'references'/f'{cid}.png';p.parent.mkdir(exist_ok=True);im.save(p)
    return {'path':rel(p),'sha256':sha(p)}
def main():
    OUT.mkdir(exist_ok=True);(OUT/'sources').mkdir(exist_ok=True)
    catalog=json.loads((ART/'manifest.json').read_text(encoding='utf-8'))
    models=json.loads((ROOT/'artifacts/sprites-cyberpunk-crawl-review-2026-10-04/equipment-catalog.json').read_text(encoding='utf-8'))['models']
    byid={m['id']:m for m in models};locales={}
    for p in (ROOT/'content/core/locales').glob('fr*.json5'):
        for k,v in re.findall(r'"(equipment\.[^"\n]+)"\s*:\s*("(?:[^"\\]|\\.)*")',p.read_text(encoding='utf-8')):
            locales[k]=json.loads(v)
    def name(i):return locales.get('equipment.'+i.split(':',1)[1]+'.name',byid.get(i,{}).get('name',i))
    legacy=Image.open(ART/'items-128.png').convert('RGBA');bodies=Image.open(ART/'crawl-bodies-32.png').convert('RGBA')
    held_families=Image.open(ART/'agent-rigs/families-48.png').convert('RGBA')
    active_rig=json.loads((ART/'agent-rigs/manifest.json').read_text(encoding='utf-8'))
    active_bodies=Image.open(ART/'agent-rigs/bodies-32.png').convert('RGBA')
    overrides={body:cell(active_bodies,j,32) for j,body in enumerate(active_rig['body_bindings'])}
    item_by_id={e['id']:e for e in catalog['items']};entries=[]
    style_ref={'path':rel(STYLE),'sha256':sha(STYLE)}
    common=('Use case: style-transfer. Asset type: ONE distinct numbered object design for a dark cyberpunk and demonic roguelike. '
        'Image 1 is the existing design reference to preserve the identity of THIS model. Image 2 is ONLY a comic-rendering quality reference; do not copy its armor, helmet, weapon or character. '
        'Redraw only the referenced object in polished 2D bande dessinee: bold dark ink contours, large flat color masses, two or three broad cel shades per material. '
        'Keep its recognizable shape and material/color identity, simplify fine texture into clean substantial forms. Smooth ink contours at large size: do not trace the raster staircase pixels of the reference. No texture noise, tiny rivets, photographic gradients, pink or magenta. '
        'Weapon colors are independent from outfit colors. A charcoal border and clear light edges must survive a 32-pixel thumbnail. '
        'Transparent background, entire subject visible, no text, number, label, logo, watermark, floor, shadow, smoke, particles, decorative aura or hands holding objects. ')
    weapon_spec={
        0:'Short knife. Draw vertically, blade UP, complete straight grip at bottom, sharp edge outside. Distinct blade shape from the reference.',
        1:'One-hand industrial sword. Draw vertically, blade UP, straight grip at bottom. Keep its distinctive blade design.',
        2:'Two-handed industrial sword. Draw vertically blade UP with a long unobstructed straight hilt for TWO separate gloves near the lower end.',
        3:'Two-handed industrial axe. Vertical complete straight haft; blade projects to IMAGE RIGHT, not inward. Two separate grip areas on the lower haft.',
        4:'Industrial club or mace as in reference. Vertical complete straight handle at bottom; no sword crossguard.',
        5:'Industrial pole weapon as in reference. Long straight vertical shaft with graspable lower area, distinct functional head at top; preserve its model shape.',
        6:'Futuristic handgun, strict side view, barrel horizontally to IMAGE RIGHT; a complete substantial grip extends down near one third of the length. No glove, hand or forearm.',
        7:'Futuristic submachine gun, strict side view, barrel horizontally to IMAGE RIGHT; complete pistol grip at the rear third, no hand or forearm.',
        8:'Assault rifle, strict side view, barrel horizontally to IMAGE RIGHT. Full stock, rear pistol grip and separate forward foregrip/support surface; two distinct graspable areas.',
        9:'Shotgun, strict side view, barrel horizontally to IMAGE RIGHT, full stock, rear grip and separate pump/support area.',
        10:'Machine gun matching this reference. Strict horizontal side view, barrel to IMAGE RIGHT, full rear grip and forward support surface. Two separate grasp areas.',
        11:'Energy rifle, strict horizontal side view, barrel to IMAGE RIGHT. Full stock, rear grip and forward support area, preserve this model geometry and broad energy-color accent.',
        12:'Heavy flamethrower, strict horizontal side view, nozzle to IMAGE RIGHT, complete rear grip and forward support area; preserve its distinctive fuel apparatus. No fire jet.',
        13:'Heavy grenade launcher, strict horizontal side view, barrel to IMAGE RIGHT, complete rear grip and forward support area.',
        14:'Heavy rocket launcher, strict horizontal side view, launch tube to IMAGE RIGHT, complete rear grip and forward support area. No launched rocket.',
        15:'Needle launcher, strict horizontal side view, muzzle to IMAGE RIGHT, complete rear grip and separate forward support area.',
        16:'Industrial war hammer, vertical straight shaft with complete grip at bottom and substantial recognizable blunt head at top; no sword crossguard.',
        17:'Precision rifle, strict horizontal side view, long barrel to IMAGE RIGHT. Complete stock, rear pistol grip and forward support surface; preserve its distinct scope and barrel.'}
    for h in sorted(catalog['held_weapons'],key=lambda e:e['index']):
        i=item_by_id[h['id']];cid=f"BD-W{h['index']+1:03}";family=h['family'];source=cell(legacy,i['illustration'],128)
        individual_reference_available=source.getchannel('A').getbbox() is not None
        if not individual_reference_available:source=cell(held_families,family,48)
        ref=store_ref(cid,source)
        variant=''
        if not individual_reference_available:
            recipes=['compact squared receiver with short barrel and straight grip',
                'long slim receiver with extended barrel and slanted grip',
                'broad reinforced receiver with a flat top and substantial trigger guard',
                'vented upper barrel shroud with a stepped steel receiver',
                'heavy rounded front shroud with compact squared rear body',
                'angular long receiver with a broad lower housing']
            variant=' This model has no existing individual drawing; use the family reference for category ONLY. Create its own distinct '+recipes[(h['index']-101)%6]+'. Steel-grey/graphite frame, no same-shape recolor variant.'
        prompt=common+'This model is '+name(h['id'])+'. '+weapon_spec[family]+variant
        if not individual_reference_available:
            prompt=('Use case: stylized-concept. Asset type: ONE distinct futuristic firearm model, '+name(h['id'])+'. '
                'The input image is a comic quality reference ONLY. Draw NO person, armor, sword or shield. '
                'Bold dark ink contour, clean broad flat cel-shaded steel-grey and graphite masses, no texture, tiny details or pink. '
                'This object has its own '+recipes[(h['index']-101)%6]+'. '+weapon_spec[family]+
                ' Entire firearm visible on transparent background, no text, numbers, logo, hand, arm, fire, aura, floor or shadow.')
        entries.append({'review_id':cid,'content_id':h['id'],'name':name(h['id']),'kind':'weapon','family':family,
            'family_name':catalog['weapon_families'][family]['name'],'hold':h['hold'],'source_reference':ref,'style_reference':style_ref,
            'individual_reference_available':individual_reference_available,
            'prompt':prompt,
            'source_path':rel(OUT/'sources'/f'{cid}.png'),'approval':'pending;no_new_comic_art_approved'})
    armor_items=[e for e in catalog['items'] if e['icon']==2]
    for j,e in enumerate(armor_items,1):
        cid=f'BD-A{j:03}';idx=e['illustration'];body_index=idx-98 if 101<=idx<=114 else None
        kind='outfit' if body_index is not None else 'head' if idx<=120 else 'hands' if idx<=126 else 'feet'
        source=overrides.get(body_index,cell(bodies,body_index,32)) if body_index is not None else cell(legacy,idx,128)
        ref=store_ref(cid,source,body=body_index is not None)
        if kind=='outfit':
            spec=('A COMPLETE adult human outfit on a neutral unarmed body, not an isolated chestplate. Preserve the reference helmet, torso, legs, boots, pose, colors and human proportions. '
                'Both relaxed black-gloved hands must remain at the SAME relative positions and each arm is a single anatomically continuous arm. '
                'No weapon, shield, familiar or transformation. The full body is designed to occupy almost all 32 pixels from crown to soles, equipment will be added as separate layers. '
                'Do not make a small head with skinny long legs. Keep the existing anatomy and broad clothing shapes rather than designing another character.')
        elif kind=='head':spec='Only this helmet, hood or face protection, displayed front-on, complete and centered; NO head, neck, shoulders or body. Preserve its distinctive identity.'
        elif kind=='hands':spec='Only this matched PAIR of gloves/gauntlets, separated side by side, knuckles facing viewer, fingers pointing DOWN, cuffs at top. No arms or body.'
        else:spec='Only this matched PAIR of boots, separated side by side, front three-quarter view, cuffs at top and soles visible. No legs or body.'
        entries.append({'review_id':cid,'content_id':e['id'],'name':name(e['id']),'kind':kind,'body_index':body_index,
            'source_reference':ref,'style_reference':style_ref,'prompt':common+'This model is '+name(e['id'])+'. '+spec,
            'source_path':rel(OUT/'sources'/f'{cid}.png'),'approval':'pending;no_new_comic_art_approved'})
    for index in range(3):
        cid=f'BD-B{index+1:03}';source=overrides.get(index,cell(bodies,index,32));ref=store_ref(cid,source,body=True)
        entries.append({'review_id':cid,'content_id':None,'name':f'Tenue de base {index+1}','kind':'outfit','body_index':index,
            'source_reference':ref,'style_reference':style_ref,'prompt':common+
            'A COMPLETE unarmed human body in the same helmet, armor, pants and boots as reference. Preserve anatomy and relaxed empty-hand pose exactly. '
            'Full body from crown to soles, no weapons or shield. Keep the head, torso, hands, hips, knees and feet at the same relative positions. Only rendering style changes; broad colors, substantial limbs.',
            'source_path':rel(OUT/'sources'/f'{cid}.png'),'approval':'pending;no_new_comic_art_approved'})
    series=json.loads((BASE/'review-equipment-series-EQUIP-SERIES01-06.json').read_text(encoding='utf-8'))['series']
    heavy_variants={
        'heavy:machine_gun':[
            'A short broad box-fed heavy machine gun with a single large barrel and armored receiver.',
            'A belt-fed heavy machine gun with a long heavy barrel, large box receiver and bottom ammunition housing.',
            'A compact three-barrel rotary gatling with a chunky motor housing.',
            'A six-barrel rotary gatling with a large cylindrical front barrel cluster and heavy rear power housing.',
            'An industrial twin-barrel heavy machine gun with a broad squared body.',
            'A reinforced heavy machine gun with a long thick barrel shroud and tall rectangular rear housing.'],
        'heavy:plasma_launcher':[
            'A broad industrial plasma launcher with a single large coil chamber and cyan circular muzzle.',
            'A plasma launcher with a long reinforced silver-grey barrel cage and a broad turquoise core block.',
            'A compact heavy plasma launcher with two broad cyan capacitor housings alongside its barrel.',
            'A heavy plasma launcher with a squared armored receiver, orange power housing and bright cyan muzzle insert.',
            'An elongated plasma launcher with a violet energy chamber inside steel braces and a large circular muzzle.',
            'A bulky heavy plasma launcher with a black-and-ivory armored chamber and broad blue energy panels.']}
    aliases=[]
    for s in series:
        fid=s['family_id']
        if fid in ('heavy:flamethrower','heavy:rocket_launcher','heavy:grenade_launcher'):
            ids=s['existing_series_ids_to_preserve'];by_content={e['content_id']:e['review_id'] for e in entries}
            aliases.extend({'series_candidate_id':cid,'review_id':by_content[ident],'content_id':ident} for cid,ident in zip(s['stable_candidate_ids'],ids))
            continue
        for k,oldcid in enumerate(s['stable_candidate_ids']):
            cid='BD-'+oldcid;kind='weapon';family=None;ref=None
            if fid=='caster:focalizer':
                p=BASE/'focalizer-review-01/sources'/f'{oldcid}.png';ref={'path':rel(p),'sha256':sha(p)};family='focalizer'
                spec='A one-handed industrial occult focalizer. Complete short vertical grip below its distinctive fork/core head. Head UP, grip DOWN. Grey metal independent from the core color; keep this exact model identity.'
            elif fid=='caster:channeling_shaft':
                p=BASE/'shaft-review-01/sources'/f'{oldcid}.png';ref={'path':rel(p),'sha256':sha(p)};family='shaft'
                spec='A two-handed industrial channeling shaft. Complete continuous vertical steel shaft with two grasp areas and its distinctive technological head at top. No medieval wooden staff; preserve this exact head shape and color identity.'
            elif fid=='defense:shield':
                rec=json.loads((BASE/'shield-review-07/review-shields-SHIELD-VIS01-07.json').read_text(encoding='utf-8'))['candidates'][k]
                p=ROOT/rec['source']['path'];ref={'path':rel(p),'sha256':sha(p)};kind='shield';family='shield'
                spec='Only this industrial futuristic shield, viewed FLAT FRONT-ON, no perspective tilt, full outline clearly visible with broad silver/steel edges and dark central plate. No gold, medieval decoration, hand, forearm or rear grip. Preserve this model geometry.'
            else:
                family='heavy_machinegun' if fid=='heavy:machine_gun' else 'plasma'
                spec=heavy_variants[fid][k]+' Strict horizontal side view, muzzle pointing RIGHT, substantial rear pistol grip around one third of length and a DISTINCT forward support surface under barrel. Entire object, no hands. Grey steel/graphite neutral frame. Bold readable silhouette and details large enough for native 32-pixel use.'
            references=[ref] if ref else []
            prompt=common+spec if ref else (
                'Use case: stylized-concept. Asset type: ONE isolated heavy-weapon model study for a dark cyberpunk occult roguelike. '
                'Reference image is for comic rendering quality ONLY, do not draw its character, armor, sword or shield. '
                'Polished 2D bande dessinee with bold dark ink contours, broad flat masses and two or three simple cel shades per material. '
                'Transparent background, no text, logos, shadow, fire/projectiles or character. No pink. '+spec)
            entries.append({'review_id':cid,'content_id':None,'series_candidate_id':oldcid,'name':s['family_label']+' '+f'{k+1:02}',
                'kind':kind,'family':family,'family_name':s['family_label'],'hold':'TwoHanded' if family in ('shaft','heavy_machinegun','plasma') else 'Side',
                'source_reference':ref,'style_reference':style_ref,'prompt':prompt,'source_path':rel(OUT/'sources'/f'{cid}.png'),
                'approval':'pending;no_new_comic_art_approved'})
    assert len({e['review_id'] for e in entries})==len(entries)==178
    assert {e['content_id'] for e in entries if e['kind']=='weapon' and e['content_id']}=={e['id'] for e in catalog['held_weapons']}
    assert {e['content_id'] for e in entries if e['kind'] in ('outfit','head','hands','feet') and e['content_id']}=={e['id'] for e in armor_items}
    payload={'review_id':'BD-CATALOGUE01','date':'2026-10-10','approval':'no_new_comic_candidates_approved;review_only',
        'scope':'113 distinct current weapons,32 armor objects,3 base outfits,18 caster/shield models,12 new heavy models',
        'method':'one_builtin_imagegen_call_per_model;separate_model_sources_and_attachment_layers;fixed_bodies',
        'source_game_catalog':{'path':rel(ART/'manifest.json'),'sha256':sha(ART/'manifest.json')},
        'active_rig':{'path':rel(ART/'agent-rigs/manifest.json'),'sha256':sha(ART/'agent-rigs/manifest.json')},
        'counts':{'current_weapons':113,'current_armor_objects':32,'base_outfits':3,'caster_shield_models':18,'additional_heavy_models':12,'total':178},
        'existing_heavy_series_aliases':aliases,'entries':entries,'runtime_integration':'none','mechanical_changes':'none'}
    (OUT/'catalogue.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    f=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',13)
    reference_entries=[e for e in entries if e['source_reference']]
    for page in range((len(reference_entries)+19)//20):
        board=Image.new('RGBA',(1000,870),(17,25,34,255));d=ImageDraw.Draw(board)
        for j,e in enumerate(reference_entries[page*20:(page+1)*20]):
            x=16+j%5*197;y=12+j//5*212
            im=Image.open(ROOT/e['source_reference']['path']).convert('RGBA');im.thumbnail((175,162),Image.Resampling.NEAREST)
            if max(im.size)<80:im=im.resize((im.width*4,im.height*4),Image.Resampling.NEAREST)
            board.alpha_composite(im,(x+(180-im.width)//2,y+8))
            d.text((x,y+173),e['review_id'],font=f,fill=(235,241,245));d.text((x,y+191),e['name'][:27],font=f,fill=(157,180,194))
        board.save(OUT/f'references-page-{page+1:02}.png')
    print(json.dumps(payload['counts']))

if __name__=='__main__':main()
