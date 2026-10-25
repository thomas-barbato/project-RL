"""Revise helmets, machine priest and sniper outfits with anatomical neck joins."""
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, native_image, core_box, font, sha, write_json, gallery
from prepare_all_class_armors_01 import protect_accepted, frame_gear, matrix, class_boards, CLASS_NAMES, LABELS
from prepare_bastion_progression_01 import pixels_hash

WAVE='class-armors-all-02'
ART,DEST,DOCS=SOURCES/WAVE,OUT/WAVE,REVIEW/WAVE


def import_source(job):
    target=ART/(job['candidate_id']+('-body-source.png' if job['kind']=='body' else '-headgear-source.png'))
    generated=Path(job['generated_file'])
    if generated.exists():shutil.copyfile(generated,target)
    else:assert target.exists(),target
    return target


def save_entry(cid,number,cls,rank,body,head,status,provenance):
    sprite=body.copy();sprite.alpha_composite(head)
    assert sprite.size==body.size==head.size==(64,64)
    b=core_box(sprite)
    assert b and b[0]>0 and b[2]<64 and b[1]>=1 and b[3]==62
    saved={}
    for kind,img,suffix in [('native',sprite,'native64'),('body_layer',body,'body-native64'),('head_layer',head,'head-native64')]:
        p=DEST/(cid+'-'+suffix+'.png');img.save(p)
        saved[kind]=p.relative_to(ROOT).as_posix();saved[kind+'_sha256']=sha(p)
    rebuilt=Image.open(ROOT/saved['body_layer']).convert('RGBA')
    rebuilt.alpha_composite(Image.open(ROOT/saved['head_layer']).convert('RGBA'))
    assert rebuilt.tobytes()==sprite.tobytes()
    return {'number':f'{number:02d}','candidate_id':cid,'class_name':cls['name'],'rank':rank,
        'label':LABELS[rank-1],'status':status,'native_dimensions':[64,64],**saved,
        'head_ownership_bottom_y':cls['seam'],'assembled_core_box':list(b),
        'layer_method':'Body includes upper shoulders behind an explicit head silhouette; head is composited above it. No full-width horizontal shoulder crop.',
        'layers_recompose_exact_rgba':True,'provenance':provenance},sprite


def comparison_boards(previous,current):
    # All avatars are kept at one native pixel per image pixel in this board.
    board=Image.new('RGB',(800,600),'#152025');d=ImageDraw.Draw(board)
    d.text((8,8),'Avant · 64 × 64 natifs',font=font(14),fill='#efddbd')
    d.text((408,8),'Reprise · mêmes numéros · 64 × 64 natifs',font=font(14),fill='#efddbd')
    for row,name in enumerate(CLASS_NAMES):
        y=56+row*106
        for side,rows in enumerate([previous,current]):
            x=side*400;d.text((x+8,y),name,font=font(13),fill='#efddbd')
            d.rectangle((x+4,y+19,x+396,y+86),fill='#343b3f')
            for i,sprite in enumerate(rows[row]):
                board.paste(sprite,(x+8+i*80,y+21),sprite)
                d.text((x+i*80+32,y+87),f'{row*5+i+1:02d}',font=font(12),fill='#d5dfdc')
    board.save(DEST/'before-after-native64.png')
    board=Image.new('RGB',(900,430),'#343b3f');d=ImageDraw.Draw(board)
    d.text((8,8),'Têtes et épaules · avant / reprise · zoom entier ×6',font=font(16),fill='#efddbd')
    for i,name in enumerate(CLASS_NAMES):
        d.text((i*180+4,34),name+' · '+f'{i*5+3:02d}',font=font(13),fill='#efddbd')
        for row,rows in enumerate([previous,current]):
            crop=rows[i][2].crop((18,0,46,28)).resize((168,168),Image.Resampling.NEAREST)
            board.paste(crop,(i*180+6,62+row*180),crop)
    d.text((8,414),'En haut : avant · en bas : reprise',font=font(11),fill='#d5dfdc')
    board.save(DEST/'head-shoulder-comparison-x6.png')
    board=Image.new('RGB',(600,480),'#343b3f');d=ImageDraw.Draw(board)
    d.text((8,8),'15 protections de tête · détails en zoom entier ×6',font=font(16),fill='#efddbd')
    for row,ci in enumerate([1,2,3]):
        y=42+row*142;d.text((8,y),CLASS_NAMES[ci],font=font(13),fill='#efddbd')
        for i,sprite in enumerate(current[ci]):
            detail=sprite.crop((23,0,41,18)).resize((108,108),Image.Resampling.NEAREST)
            board.paste(detail,(i*120+6,y+20),detail)
            d.text((i*120+51,y+130),f'{ci*5+i+1:02d}',font=font(12),fill='#d5dfdc')
    board.save(DEST/'fifteen-heads-x6.png')


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted revision is frozen; prepare a new revision.')
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    missing=[j['candidate_id'] for j in plan['jobs'] if not j.get('generated_file')]
    if missing:raise SystemExit('Missing generated sources: '+', '.join(missing))
    old_path=ROOT/plan['source_review'];old=json.loads(old_path.read_text('utf-8'))
    protected=protect_accepted()
    protected[plan['source_review']]=sha(old_path)
    for e in old['entries']:
        for k in ('native','head_layer','body_layer'):protected[e[k]]=e[k+'_sha256']
        if 'source' in e['provenance']:protected[e['provenance']['source']]=e['provenance']['source_sha256']
    for e in old['new_headgear_sources']:protected[e['source']]=e['source_sha256']
    for p in (SOURCES/'class-armors-all-01').glob('*.png'):
        protected[p.relative_to(ROOT).as_posix()]=sha(p)
    assert all(sha(ROOT/p)==s for p,s in protected.items())
    roots=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    bases={e['candidate_id']:e for e in roots['entries']}
    for base in bases.values():
        for key in ('source','native'):
            assert sha(ROOT/base[key])==base[key+'_sha256']
            protected[base[key]]=base[key+'_sha256']
    previous=[[Image.open(ROOT/e['native']).convert('RGBA') for e in old['entries'][r*5:r*5+5]] for r in range(5)]
    current,entries,head_sources=[],[],[]
    for ci,cls in enumerate(plan['classes']):
        row=[]
        for rank in range(1,6):
            number=ci*5+rank;prior=old['entries'][number-1]
            if cls['key']=='bastion':
                body=Image.open(ROOT/prior['body_layer']).convert('RGBA')
                head=Image.open(ROOT/prior['head_layer']).convert('RGBA')
                cid=prior['candidate_id'];status='accepted_visual_reference'
                provenance={'previous_candidate':cid,'accepted_pixels_preserved':True,
                    'neck_audit':'Accepted join is already short; no visual change.'}
            else:
                bjob=next((j for j in plan['jobs'] if j['class_key']==cls['key'] and j['rank']==rank and j['kind']=='body'),None)
                source=import_source(bjob) if bjob else ROOT/prior['provenance']['source']
                body,measure=native_image(source,cls['body_height'])
                # Keep shoulder/collar pixels that legitimately overlap the head's rows.
                # Only the head silhouette owns face occlusion; rows are not erased wholesale.
                assert body.getpixel((0,0))[3]==0
                hjob=next((j for j in plan['jobs'] if j['class_key']==cls['key'] and j['rank']==rank and j['kind']=='headgear'),None)
                if hjob:
                    hsource=import_source(hjob);gear,hmeasure=frame_gear(hsource,hjob['framing'],cls['seam'])
                    if hjob['head_mode']=='overlay':
                        base=Image.open(ROOT/bases[cls['base_class']]['native']).convert('RGBA')
                        base_head=Image.new('RGBA',(64,64));base_head.paste(base.crop((0,0,64,cls['seam'])),(0,0))
                        head=base_head.copy();head.alpha_composite(gear)
                        for y in range(cls['seam']):
                            for x in range(64):
                                if gear.getpixel((x,y))[3]==0:assert head.getpixel((x,y))==base_head.getpixel((x,y))
                    else:
                        head=gear.copy()
                    hp=DEST/(hjob['candidate_id']+'-gear-native64.png');gear.save(hp)
                    head_sources.append({'candidate_id':hjob['candidate_id'],'number':f'{number:02d}',
                        'source':hsource.relative_to(ROOT).as_posix(),'source_sha256':sha(hsource),
                        'native_layer':hp.relative_to(ROOT).as_posix(),'native_layer_sha256':sha(hp),
                        'head_mode':hjob['head_mode'],'framing':hmeasure})
                    head_provenance={'candidate_id':hjob['candidate_id'],'source':hsource.relative_to(ROOT).as_posix(),
                        'mode':hjob['head_mode'],'framing':hmeasure}
                else:
                    head=Image.open(ROOT/prior['head_layer']).convert('RGBA')
                    head_provenance={'previous_head_layer':prior['head_layer'],'preserved_exact_rgba':True}
                cid=bjob['candidate_id'] if bjob else prior['candidate_id'].replace('-r01','-r02')
                status='awaiting_visual_review'
                provenance={'previous_candidate':prior['candidate_id'],'body_source':source.relative_to(ROOT).as_posix(),
                    'body_source_sha256':sha(source),'body_framing':measure,'head':head_provenance,
                    'neck_fix':'Upper shoulders framed closer to the jaw; body and head overlap only under the head silhouette.',
                    'raw_body_art_reused':bjob is None}
            e,sprite=save_entry(cid,number,cls,rank,body,head,status,provenance)
            if cls['key']=='bastion':assert sprite.tobytes()==previous[ci][rank-1].tobytes()
            entries.append(e);row.append(sprite)
        current.append(row)
    assert len(entries)==25 and len({e['candidate_id'] for e in entries})==25
    matrix(current,DEST/'all-classes-native64.png','#343b3f')
    matrix(current,DEST/'all-classes-light-native64.png','#858478')
    matrix(current,DEST/'all-classes-street-native64.png',None,street=True)
    for i,(name,row) in enumerate(zip(CLASS_NAMES,current)):class_boards(name,row,i,dest=DEST)
    comparison_boards(previous,current)
    assert all(sha(ROOT/p)==s for p,s in protected.items())
    write_json(DOCS/'candidates.json',{'review_id':plan['review_id'],'date':'2026-10-21','status':'awaiting_joint_visual_review',
        'native_dimensions':[64,64],'classes':5,'models_per_class':5,'entries':entries,'head_protections':head_sources,
        'previously_accepted_unchanged':['01','02','03','04','05'],'revised_pending_review':[f'{n:02d}' for n in range(6,26)],
        'technomancien_direction':plan['technomancien_choice'],'previous_review':plan['source_review'],
        'new_color_variants':0,'runtime_integration':False,
        'checks':{'all_twenty_five_native64':True,'all_layers_recompose_exact_rgba':True,'foot_baseline_y':61,
            'all_prior_sources_and_accepted_files_unchanged':protected,
            'body_shoulder_rows_not_cropped_at_head_boundary':True,
            'closed_helmet_replaces_base_head_instead_of_showing_hair_outside_it':True,
            'textured_ground_is_montage_not_engine_validation':True}})
    details=[]
    for i,name in enumerate(CLASS_NAMES):
        details.append(f'<details><summary>{name} · {i*5+1:02d}–{i*5+5:02d} · trois fonds et zoom</summary><div class="native-scroll"><img class="board" width="400" height="348" src="{WAVE}/class-{i+1:02d}-native64.png" alt="{name}, cinq tenues révisées à taille native sur trois fonds"></div><div class="native-scroll"><img class="board" width="1040" height="263" src="{WAVE}/class-{i+1:02d}-zoom-x3.png" alt="{name}, cinq tenues révisées en zoom entier ×3"></div></details>')
    (DEST/'section.html').write_text('''<section id="all-armors-02"><h2>Reprise · casques, silhouettes et raccords du cou</h2>
<p>Technomancien : prêtre-machine sombre, visage mécanique et manteau sobre. Traqueuse : tireuse d’élite / ninja, avec cinq protections de tête complètes. Arcaniste : cinq coiffes compactes accordées à la tenue. Les épaules sont recalées pour raccourcir les raccords du cou ; la Métamorphe conserve ses dessins avec ce nouveau cadrage.</p>
<p class="note">Même numérotation 01–25. Le Bastion 01–05 reste exactement validé. Les compositions 06–25 sont de nouvelles révisions à examiner ; l’ancien témoin du Technomancien reste archivé et son accord ne valide pas cette reprise.</p>
<h3 id="all-armors-native64">Ensemble à taille réelle · reprise</h3>
<div class="native-scroll"><img class="board" width="400" height="600" src="class-armors-all-02/all-classes-native64.png" alt="Les vingt-cinq armures après reprise des casques et des raccords du cou, en 64 × 64 natifs"></div>
<details><summary>Avant / reprise à taille réelle</summary><div class="native-scroll"><img class="board" width="800" height="600" src="class-armors-all-02/before-after-native64.png" alt="Les vingt-cinq compositions précédentes et révisées, comparées en taille native"></div></details>
<details><summary>Comparer les têtes et les épaules ×6</summary><div class="native-scroll"><img class="board" width="900" height="430" src="class-armors-all-02/head-shoulder-comparison-x6.png" alt="Avant et reprise des têtes et épaules de chaque classe en zoom entier ×6"></div></details>
<details><summary>Les quinze protections de tête ×6</summary><div class="native-scroll"><img class="board" width="600" height="480" src="class-armors-all-02/fifteen-heads-x6.png" alt="Les cinq protections de tête distinctes du Technomancien, de la Traqueuse et de l’Arcaniste, numérotées comme leurs tenues, en zoom entier ×6"></div></details>
<details><summary>L’ensemble sur sol clair</summary><div class="native-scroll"><img class="board" width="400" height="600" src="class-armors-all-02/all-classes-light-native64.png" alt="Les vingt-cinq compositions révisées à taille native sur sol clair"></div></details>
<details><summary>L’ensemble sur le sol de la rue</summary><p class="note">Montage de revue ; l’éclairage du moteur reste à vérifier.</p><div class="native-scroll"><img class="board" width="400" height="600" src="class-armors-all-02/all-classes-street-native64.png" alt="Les vingt-cinq compositions révisées à taille native sur le sol de la rue"></div></details>
'''+''.join(details)+'''<p>Les protections présentées appartiennent à la tête fournie par le torse. Un casque équipé remplacera cette tête. Les objets de casque séparés et les prises d’armes restent à préparer avant l’intégration. Pas de variantes colorées supplémentaires.</p>
<p><a href="../../docs/reviews/class-readability-01/class-armors-all-02/REVIEW.md">Périmètre de la reprise</a> · <a href="../../docs/reviews/class-readability-01/class-armors-all-02/prompts.json">Prompts image_gen</a> · <a href="../../docs/reviews/class-readability-01/class-armors-all-02/candidates.json">Sources et contrôles</a></p></section>''','utf-8')
    roots['approval_scopes']['body_armor_variants']='Bastion preserved; revised machine-priest and sniper/ninja outfits, corrected arcanist and metamorph neck framing pending joint review'
    roots['approval_scopes']['default_head_appearances']='Bastion accepted; fifteen new Technomancien, Traqueuse and Arcaniste head protections pending review'
    roots['latest_steering']='Revise unpopular Technomancien and Traqueuse, compact Arcaniste helmets and fix long neck joins. Technomancien choice: dark machine priest with a mechanical face and sober coat. Keep numbers 01–25; no integration.'
    write_json(REVIEW/'candidates.json',roots)
    policy_path=REVIEW/'head-appearance-rule.json';policy=json.loads(policy_path.read_text('utf-8'))
    policy['joint_armor_review']='docs/reviews/class-readability-01/class-armors-all-02/candidates.json'
    policy['visual_status']='bastion_accepted_revised_technomancien_traqueuse_arcaniste_heads_pending_joint_review'
    policy['technomancien_default_head_appearances']='dark_machine_priest_revision_requested_previous_accepted_pilot_archived'
    policy['technomancien_head_review']='docs/reviews/class-readability-01/class-armors-all-02/candidates.json'
    policy['technomancien_identity_acceptance_status']='historical_reference_superseded_by_user_requested_revision'
    policy['traqueuse_head_review']='docs/reviews/class-readability-01/class-armors-all-02/candidates.json'
    policy['arcaniste_head_review']='docs/reviews/class-readability-01/class-armors-all-02/candidates.json'
    write_json(policy_path,policy)
    plan['status']='prepared_for_joint_native64_review';write_json(DOCS/'prompts.json',plan)
    gallery(roots['entries'],roots['equipment_inventory'])
    print(json.dumps({'compositions':25,'new_head_protections':15,'unchanged_accepted':5,'revised_pending':20,
        'protected_prior_files':len(protected),'native_dimensions':[64,64],'runtime_integration':False}))


if __name__=='__main__':main()
