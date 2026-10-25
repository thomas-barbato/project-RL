"""Prepare a joint native64 review of five armor models for each player class."""
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, STREET, core_box, native_image, font, sha, write_json, gallery
from prepare_bastion_progression_01 import pixels_hash

WAVE = 'class-armors-all-01'
ART, DEST, DOCS = SOURCES/WAVE, OUT/WAVE, REVIEW/WAVE
LABELS = ['Très légère','Légère','Interméd.','Lourde','Très lourde']
CLASS_NAMES = ['Bastion','Technomancien','Traqueuse','Arcaniste','Métamorphe']


def protect_accepted():
    protected = {}
    for name in ['bastion-wave-01/acceptance.json','bastion-heads-02/acceptance.json',
                 'technomancien-heads-01/acceptance.json']:
        path = REVIEW/name
        data = json.loads(path.read_text('utf-8'))
        protected[path.relative_to(ROOT).as_posix()] = sha(path)
        protected.update(data.get('protected_previous_files',{}))
        protected.update(data.get('checks',{}).get('all_previous_accepted_bastion_files_unchanged',{}))
        for item in data.get('entries',[]):
            for key in ['native','body_layer','headgear_layer']:
                if key in item:
                    protected[item[key]['path']] = item[key]['sha256']
            protected[item['source']] = item['source_sha256']
        for item in list(data.get('files',{}).values())+data.get('review_references',[])+data.get('provenance',[]):
            protected[item['path']] = item['sha256']
        if 'source' in data:
            protected[data['source']] = data['source_sha256']
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    return protected


def import_generated(job):
    suffix = 'body-source' if job['kind']=='body' else 'headgear-source'
    target = ART/(job['candidate_id']+'-'+suffix+'.png')
    generated = Path(job['generated_file'])
    if generated.exists():
        shutil.copyfile(generated,target)
    else:
        assert target.exists(),target
    return target


def frame_gear(source,spec,seam):
    original = Image.open(source).convert('RGBA')
    x0,y0,x1,y1 = core_box(original)
    pad = max(2,round((y1-y0)*0.02))
    crop = (max(0,x0-pad),max(0,y0-pad),min(original.width,x1+pad),min(original.height,y1+pad))
    cropped = original.crop(crop)
    scale = min(spec['target_height']/(y1-y0),spec['max_width']/(x1-x0))
    reduced = cropped.resize((round(cropped.width*scale),round(cropped.height*scale)),Image.Resampling.LANCZOS)
    bx0,by0,bx1,by1 = core_box(reduced)
    dx,dy = round(32-(bx0+bx1)/2),spec['bottom_exclusive']-by1
    full = Image.new('RGBA',(64,64));full.alpha_composite(reduced,(dx,dy))
    overlay = Image.new('RGBA',(64,64));overlay.paste(full.crop((0,0,64,seam)),(0,0))
    bounds = core_box(overlay)
    assert bounds and bounds[0]>0 and bounds[2]<64 and bounds[1]>=1
    return overlay,{'source_core_box':[x0,y0,x1,y1],'source_crop':list(crop),'isotropic_scale':scale,
        'native_core_box':list(bounds),'placement':[dx,dy],'ownership':[0,0,64,seam]}


def save_composition(cid,number,class_name,rank,sprite,seam,status,provenance):
    assert sprite.size==(64,64)
    native = DEST/(cid+'-native64.png')
    head = Image.new('RGBA',(64,64));head.paste(sprite.crop((0,0,64,seam)),(0,0))
    body = Image.new('RGBA',(64,64));body.paste(sprite.crop((0,seam,64,64)),(0,seam))
    head_path,body_path = DEST/(cid+'-head-native64.png'),DEST/(cid+'-body-native64.png')
    sprite.save(native);head.save(head_path);body.save(body_path)
    rebuilt = body.copy();rebuilt.alpha_composite(head)
    assert rebuilt.tobytes()==sprite.tobytes()
    bounds = core_box(sprite)
    assert bounds and bounds[0]>0 and bounds[2]<64 and bounds[1]>=1 and bounds[3]==62
    return {'number':f'{number:02d}','candidate_id':cid,'class_name':class_name,'rank':rank,'label':LABELS[rank-1],
        'status':status,'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
        'head_layer':head_path.relative_to(ROOT).as_posix(),'head_layer_sha256':sha(head_path),
        'body_layer':body_path.relative_to(ROOT).as_posix(),'body_layer_sha256':sha(body_path),
        'head_body_seam_y':seam,'head_region_rgba_sha256':pixels_hash(head),
        'assembled_core_box':list(bounds),'layers_recompose_exact_rgba':True,'provenance':provenance}


def matrix(rows,path,ground,street=False):
    board = Image.new('RGB',(400,600),'#152025')
    d = ImageDraw.Draw(board)
    d.text((8,8),'25 armures · cinq classes · 64 × 64 natifs',font=font(14),fill='#efddbd')
    d.text((8,30),'Numéros 01–25 · comparaison à taille réelle',font=font(11),fill='#b5c5c3')
    for i,label in enumerate(LABELS):
        d.text((i*80+4,52),label,font=font(10),fill='#d5dfdc')
    patch = Image.open(STREET).convert('RGB').crop((80,160,480,256)) if street else None
    for row,(name,sprites) in enumerate(zip(CLASS_NAMES,rows)):
        y = 74+row*104
        d.text((8,y),name,font=font(13),fill='#efddbd')
        if street:
            board.paste(patch.crop((0,0,400,68)),(0,y+18))
        else:
            d.rectangle((4,y+18,396,y+85),fill=ground)
        for i,sprite in enumerate(sprites):
            board.paste(sprite,(8+i*80,y+20),sprite)
            d.text((i*80+32,y+86),f'{row*5+i+1:02d}',font=font(12),fill='#d5dfdc')
    board.save(path)


def class_boards(name,sprites,row,dest=DEST):
    board = Image.new('RGB',(400,348),'#152025');d=ImageDraw.Draw(board)
    d.text((8,9),name+' · cinq armures · 64 × 64 natifs',font=font(14),fill='#efddbd')
    for i in range(5):
        d.text((i*80+4,36),f'{row*5+i+1:02d} · {LABELS[i]}',font=font(10),fill='#d5dfdc')
    for r,(label,color) in enumerate([('Sol sombre','#343b3f'),('Sol clair','#858478'),('Pénombre','#12181d')]):
        y = 72+r*90;d.rectangle((4,y-3,396,y+66),fill=color)
        for i,sprite in enumerate(sprites):board.paste(sprite,(8+i*80,y),sprite)
        d.text((8,y+68),label,font=font(11),fill='#b5c5c3')
    board.save(dest/f'class-{row+1:02d}-native64.png')
    board=Image.new('RGB',(1040,263),'#243036');d=ImageDraw.Draw(board)
    d.text((8,8),name+' · zoom entier ×3',font=font(17),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        z=sprite.resize((192,192),Image.Resampling.NEAREST);board.paste(z,(8+i*208,38),z)
        d.text((12+i*208,234),f'{row*5+i+1:02d} · {LABELS[i]}',font=font(13),fill='#d5dfdc')
    board.save(dest/f'class-{row+1:02d}-zoom-x3.png')


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted overview is frozen; prepare a new revision.')
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    missing=[j['candidate_id'] for j in plan['jobs'] if not j.get('generated_file')]
    if missing:raise SystemExit('Missing generated sources: '+', '.join(missing))
    protected=protect_accepted()
    roots=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    bases={e['candidate_id']:e for e in roots['entries']}
    entries,rows=[],[]
    accepted_bastion=json.loads((REVIEW/'bastion-heads-02/acceptance.json').read_text('utf-8'))
    row=[]
    for i,item in enumerate(accepted_bastion['entries']):
        sprite=Image.open(ROOT/item['native']['path']).convert('RGBA')
        e=save_composition(item['candidate_id'],i+1,'Bastion',i+1,sprite,12,'accepted_visual_reference',
            {'acceptance':'docs/reviews/class-readability-01/bastion-heads-02/acceptance.json','approved_native':item['native']})
        assert Image.open(ROOT/e['native']).convert('RGBA').tobytes()==sprite.tobytes()
        entries.append(e);row.append(sprite)
    rows.append(row)
    tech=json.loads((REVIEW/'technomancien-wave-02/design-candidates.json').read_text('utf-8'))
    pilot=json.loads((REVIEW/'technomancien-heads-01/acceptance.json').read_text('utf-8'))
    cowl=Image.open(ROOT/pilot['files']['cowl_layer']['path']).convert('RGBA')
    approved_pilot=Image.open(ROOT/pilot['files']['native']['path']).convert('RGBA')
    row=[]
    for i,item in enumerate(tech['entries']):
        base=Image.open(ROOT/item['native']).convert('RGBA');sprite=base.copy();sprite.alpha_composite(cowl)
        assert sprite.crop((0,27,64,64)).tobytes()==base.crop((0,27,64,64)).tobytes()
        cid='TE-W02-ID02-r01' if i==2 else f'TE-W02-C0{i+1}-r01'
        if i==2:assert sprite.tobytes()==approved_pilot.tobytes()
        entries.append(save_composition(cid,i+6,'Technomancien',i+1,sprite,14,
            'accepted_visual_reference' if i==2 else 'awaiting_visual_review',
            {'body_candidate':item['candidate_id'],'body_native':item['native'],'body_native_sha256':item['native_sha256'],
             'cowl_acceptance':'docs/reviews/class-readability-01/technomancien-heads-01/acceptance.json',
             'scope':'Approved cowl retained; other four body compositions require review.'}))
        row.append(sprite)
    assert len({e['head_region_rgba_sha256'] for e in entries[5:10]})==1
    rows.append(row)
    source_entries=[]
    for ci,cls in enumerate(plan['classes'],start=2):
        seam=cls['seam'];base_entry=bases[cls['base']]
        base=Image.open(ROOT/base_entry['native']).convert('RGBA')
        protected[base_entry['source']]=base_entry['source_sha256'];protected[base_entry['native']]=base_entry['native_sha256']
        base_head=Image.new('RGBA',(64,64));base_head.paste(base.crop((0,0,64,seam)),(0,0))
        gear_job=next(j for j in plan['jobs'] if j['class_key']==cls['key'] and j['kind']=='headgear')
        gear_source=import_generated(gear_job)
        gear,gear_frame=frame_gear(gear_source,gear_job['framing'],seam)
        head=base_head.copy();head.alpha_composite(gear)
        for y in range(seam):
            for x in range(64):
                if gear.getpixel((x,y))[3]==0:
                    assert head.getpixel((x,y))==base_head.getpixel((x,y))
        head_path=DEST/(gear_job['candidate_id']+'-default-head-native64.png');head.save(head_path)
        gear_path=DEST/(gear_job['candidate_id']+'-overlay-native64.png');gear.save(gear_path)
        source_entries.append({'candidate_id':gear_job['candidate_id'],'kind':'headgear','source':gear_source.relative_to(ROOT).as_posix(),
            'source_sha256':sha(gear_source),'native_layer':gear_path.relative_to(ROOT).as_posix(),'native_layer_sha256':sha(gear_path),
            'framing':gear_frame,'base_head_source':base_entry['native'],'default_head_layer':head_path.relative_to(ROOT).as_posix()})
        row=[]
        for job in [j for j in plan['jobs'] if j['class_key']==cls['key'] and j['kind']=='body']:
            source=import_generated(job)
            framed,measure=native_image(source,62-seam)
            body=Image.new('RGBA',(64,64));body.paste(framed.crop((0,seam,64,64)),(0,seam))
            sprite=body.copy();sprite.alpha_composite(head)
            number=ci*5+job['rank']
            e=save_composition(job['candidate_id'],number,cls['name'],job['rank'],sprite,seam,'awaiting_visual_review',
                {'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),'body_framing':measure,
                 'base_class':cls['base'],'default_head_candidate':gear_job['candidate_id'],
                 'base_face_preserved_outside_explicit_headgear_occlusion':True})
            entries.append(e);row.append(sprite)
        assert len(row)==5 and len({e['head_region_rgba_sha256'] for e in entries[-5:]})==1
        rows.append(row)
    assert len(entries)==25 and len(rows)==5 and all(len(r)==5 for r in rows)
    assert len({e['native_sha256'] for e in entries})==25
    matrix(rows,DEST/'all-classes-native64.png','#343b3f')
    matrix(rows,DEST/'all-classes-light-native64.png','#858478')
    matrix(rows,DEST/'all-classes-street-native64.png',None,street=True)
    for i,(name,row) in enumerate(zip(CLASS_NAMES,rows)):class_boards(name,row,i)
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    write_json(DOCS/'candidates.json',{'review_id':plan['review_id'],'date':'2026-10-21','status':'awaiting_joint_visual_review',
        'native_dimensions':[64,64],'classes':5,'models_per_class':5,'total_compositions':25,
        'previously_accepted_compositions':[e['number'] for e in entries if e['status']=='accepted_visual_reference'],
        'entries':entries,'new_headgear_sources':source_entries,'new_color_variants':0,'runtime_integration':False,
        'head_rule':'docs/reviews/class-readability-01/head-appearance-rule.json',
        'workflow':'Complete all five classes before joint native64 selection of designs to keep or modify.',
        'limits':['Body armor models only; independent inventory helmets and weapon grips remain separate work.',
            'The common default headgear of each new class is a proposal supplied by torso armor, not a new helmet item.',
            'Weights 01–05 are visual review positions, not newly accepted item names or mechanics.',
            'Textured-ground comparisons are review montages, not in-engine validation.'],
        'checks':{'all_twenty_five_native64':True,'all_layers_recompose_exact_rgba':True,
            'new_class_base_faces_preserved_outside_gear_occlusion':True,'foot_baseline_y':61,
            'all_accepted_and_prior_reference_files_preserved':protected}})
    sections=[]
    for row,name in enumerate(CLASS_NAMES):
        nums=f'{row*5+1:02d}–{row*5+5:02d}'
        sections.append(f'<details><summary>{name} · {nums} · trois fonds et zoom</summary><div class="native-scroll"><img class="board" width="400" height="348" src="{WAVE}/class-{row+1:02d}-native64.png" alt="{name}, cinq armures en 64 × 64 sur trois fonds"></div><div class="native-scroll"><img class="board" width="1040" height="263" src="{WAVE}/class-{row+1:02d}-zoom-x3.png" alt="{name}, cinq armures en zoom entier ×3"></div></details>')
    (DEST/'section.html').write_text('''<section id="all-armors-01"><h2>Les cinq classes · 25 armures à comparer</h2>
<p>Cinq tenues par classe, de très légère à très lourde. Chaque personnage ci-dessous occupe exactement 64 × 64 pixels ; les images gardent leur taille native. Les numéros 01 à 25 permettent d’indiquer les modèles à garder ou à modifier.</p>
<p class="note">Déjà validés : Bastion 01–05 et témoin du Technomancien 08. Les dix-neuf autres compositions restent à examiner ensemble. Pas de nouvelle variante colorée ; aucune intégration au jeu.</p>
<h3 id="all-armors-native64">Ensemble à taille réelle</h3>
<div class="native-scroll"><img class="board" width="400" height="600" src="class-armors-all-01/all-classes-native64.png" alt="Vingt-cinq armures, cinq classes et cinq poids, numérotées 01 à 25, en 64 × 64 natifs"></div>
<details><summary>L’ensemble sur sol clair</summary><div class="native-scroll"><img class="board" width="400" height="600" src="class-armors-all-01/all-classes-light-native64.png" alt="Les mêmes vingt-cinq armures à taille native sur fond clair"></div></details>
<details><summary>L’ensemble sur le sol de la rue</summary><p class="note">Montage de revue ; l’éclairage du moteur reste à vérifier.</p><div class="native-scroll"><img class="board" width="400" height="600" src="class-armors-all-01/all-classes-street-native64.png" alt="Les mêmes vingt-cinq armures à taille native sur un sol de rue"></div></details>
'''+''.join(sections)+'''<p>Le torse fournit la tenue complète et sa tête par défaut. Un casque équipé remplacera uniquement cette tête ; les casques équipables et les positions des mains avec les armes restent à préparer. Les armes gardent leurs modèles communs aux classes.</p>
<p><a href="../../docs/reviews/class-readability-01/class-armors-all-01/REVIEW.md">Périmètre et numéros</a> · <a href="../../docs/reviews/class-readability-01/class-armors-all-01/prompts.json">Prompts image_gen</a> · <a href="../../docs/reviews/class-readability-01/class-armors-all-01/candidates.json">Sources et contrôles</a></p></section>''','utf-8')
    roots['latest_steering']='Complete all five class armor ranges before a joint native64 review. Keep accepted Bastion and Technomancien identity pilot exact; nineteen other compositions require selection or revision. No color variants or runtime integration.'
    roots['approval_scopes']['body_armor_variants']='Joint 25-composition review prepared: six exact accepted references and nineteen pending compositions'
    roots['approval_scopes']['default_head_appearances']='Bastion and Technomancien pilot accepted; Traqueuse, Arcaniste and Métamorphe default headgear pending review'
    roots['review_workflow']='all_classes_then_joint_native64_review'
    write_json(REVIEW/'candidates.json',roots)
    plan['status']='prepared_for_joint_native64_review'
    write_json(DOCS/'prompts.json',plan)
    policy_path=REVIEW/'head-appearance-rule.json'
    policy=json.loads(policy_path.read_text('utf-8'))
    policy['visual_status']='bastion_and_technomancien_identity_accepted_three_new_class_headgear_pending_joint_review'
    policy['review_workflow']='all_classes_then_joint_native64_review'
    policy['joint_armor_review']='docs/reviews/class-readability-01/class-armors-all-01/candidates.json'
    write_json(policy_path,policy)
    gallery(roots['entries'],roots['equipment_inventory'])
    print(json.dumps({'compositions':25,'classes':5,'accepted_exact':6,'awaiting_review':19,'new_sources':18,
        'protected_files':len(protected),'native_dimensions':[64,64],'runtime_integration':False}))


if __name__=='__main__':main()
