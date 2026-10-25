"""Correct existing head attachment points; no drawing or resampling at native64."""
import json
import shutil
import sys
from PIL import Image,ImageDraw,ImageChops
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,core_box,font,sha,write_json,gallery
from prepare_arcaniste_channeler_wave_01 import boards,LABELS
from prepare_arcaniste_mage_heads_02 import HEAD_LABELS
from prepare_arcaniste_same_face_03 import inverse_mask
from prepare_technomancien_no_robe_heads_02 import fixed_project
from prepare_traqueuse_sniper_wave_01 import reference_sprites

NAME='arcaniste-short-neck-04'
RID='ARCANISTE-SHORT-NECK04'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
OLD=REVIEW/'arcaniste-same-face-03/candidates.json'
DY=2


def shifted(im,dy):
    result=Image.new(im.mode,im.size);result.paste(im,(0,dy));return result


def plan():
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    assert not (DOCS/'recipe.json').exists()
    old=json.loads(OLD.read_text('utf-8'))
    write_json(DOCS/'recipe.json',{'review_id':RID,'date':'2026-10-22',
        'source_user_request':'sa nuque est trop haute ca fait bizarre',
        'method':'Existing modular head-layer attachment correction; no new artwork generated.',
        'native_head_translation':[0,DY],'native_resampling':False,
        'preserve':'Exact existing heads, common visible face, hood occlusion, body outside old/new head footprints, feet and canvas.',
        'raccord':'Old and moved ownership masks remove the old attachment and hide the neck beneath the lowered head. No horizontal body crop.',
        'references':[{'candidate_id':e['candidate_id'],'source':e['source'],'source_sha256':sha(ROOT/e['source']),
            'native':e['native'],'native_sha256':sha(ROOT/e['native'])} for e in old['entries']]})
    protected=[]
    for root in (SOURCES,OUT,REVIEW):protected.extend(p for p in root.rglob('*.png') if NAME not in p.parts)
    protected.extend(REVIEW.rglob('acceptance.json'))
    write_json(DOCS/'prior-images.json',{p.relative_to(ROOT).as_posix():sha(p) for p in protected})
    print(json.dumps({'protected_files':len(protected),'native_head_translation':[0,DY]}))


def main():
    if (REVIEW/'arcaniste-red-robe-wave-05/candidates.json').exists():raise SystemExit('Later red-robe review exists; do not downgrade live preview.')
    assert not (DOCS/'acceptance.json').exists(),'Accepted review is frozen.'
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    old=json.loads(OLD.read_text('utf-8'))
    bases=json.loads((REVIEW/'arcaniste-channeler-wave-01/candidates.json').read_text('utf-8'))['entries']
    actors=[];heads=[];bodies=[];originals=[];entries=[]
    for e in old['entries']:
        rank=e['rank'];cid=f'AR-NC04-P{rank:02d}-r01'
        original=Image.open(ROOT/e['native']).convert('RGBA')
        old_head=Image.open(ROOT/e['layers']['head']['path']).convert('RGBA')
        old_mask=Image.open(ROOT/e['layers']['head-mask']['path']).convert('L')
        head=shifted(old_head,DY);mask=shifted(old_mask,DY)
        union=ImageChops.lighter(old_mask,mask)
        body=original.copy();body.paste((0,0,0,0),(0,0,64,64),union)
        actor=body.copy();actor.alpha_composite(head)
        assert head.tobytes()==shifted(old_head,DY).tobytes()
        changed_outside=sum(actor.getpixel((x,y))!=original.getpixel((x,y))
            for y in range(64) for x in range(64) if not union.getpixel((x,y)))
        assert changed_outside==0
        assert actor.crop((0,13,64,64)).tobytes()==original.crop((0,13,64,64)).tobytes()
        assert core_box(actor)[3]==62
        face=shifted(Image.open(ROOT/e['layers']['face']['path']).convert('RGBA'),DY)
        face_mask=shifted(Image.open(ROOT/e['layers']['face-mask']['path']).convert('L'),DY)
        assert all(actor.getpixel((x,y))==face.getpixel((x,y))
            for y in range(64) for x in range(64) if face_mask.getpixel((x,y)))
        source=Image.open(ROOT/e['source']).convert('RGBA')
        base=Image.open(ROOT/bases[rank-1]['source']).convert('RGBA')
        _,offset,size,frame=fixed_project(base,base,e['framing'])
        source_mask=inverse_mask(old_mask,source,offset,size,frame)
        source_dy=round(DY*(frame[3]-frame[1])/size[1])
        moved_source_mask=shifted(source_mask,source_dy)
        source_union=ImageChops.lighter(source_mask,moved_source_mask)
        source_head=Image.new('RGBA',source.size);source_head.paste(source,(0,0),source_mask)
        moved_source_head=shifted(source_head,source_dy)
        source_body=source.copy();source_body.paste((0,0,0,0),(0,0,*source.size),source_union)
        final_source=source_body.copy();final_source.alpha_composite(moved_source_head)
        restored=final_source.copy();restored.paste(source,(0,0),source_union)
        assert restored.tobytes()==source.tobytes()
        sp=ART/f'{cid}-source.png';final_source.save(sp)
        np=ART/f'{cid}-native64.png';actor.save(np);shutil.copyfile(np,DEST/np.name)
        layers={}
        for label,im in [('head',head),('body',body),('head-mask',mask),('attachment-mask',union),('face',face),('face-mask',face_mask)]:
            p=ART/f'{cid}-{label}-native64.png';im.save(p);layers[label]={'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}
        for label,im in [('head',moved_source_head),('body',source_body),('attachment-mask',source_union)]:
            p=ART/f'{cid}-{label}-source.png';im.save(p);layers[f'{label}-source']={'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==actor.tobytes()
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'status':'awaiting_visual_review','previous_candidate_id':e['candidate_id'],
            'source':sp.relative_to(ROOT).as_posix(),'source_sha256':sha(sp),
            'native':np.relative_to(ROOT).as_posix(),'native_sha256':sha(np),'native_dimensions':[64,64],
            'native_core_box':list(core_box(actor)),'framing':e['framing'],
            'framing_status':'unchanged body framing; head translated afterwards',
            'head_label':HEAD_LABELS[rank-1],'native_head_translation':[0,DY],'source_head_translation':[0,source_dy],
            'native_head_pixels_preserved_by_translation':True,'native_resampling':False,
            'native_pixel_changes_outside_attachment':changed_outside,'all_pixels_from_y13_exact':True,
            'source_pixels_outside_attachment_exact':True,'visible_face_preserved_by_translation':True,
            'head_body_recomposition_exact':True,'hood_occlusion_allowed':True,'layers':layers,
            'foot_anchor':[32,61],'rig_status':'study attachment correction only; independent helmet override and weapon grips pending'})
        actors.append(actor);heads.append(head);bodies.append(body);originals.append(original)
    references=[];acceptances=[]
    for folder,label in [('bastion-copper-wave-01','Bastion · validé'),('technomancien-no-robe-heads-02','Technomancien · validé'),('traqueuse-sniper-wave-01','Traqueuse · validée')]:
        ref,a=reference_sprites(folder);references.append((label,ref));acceptances.append(a)
    boards(actors,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS)
    comparison=Image.new('RGB',(400,304),'#243036');draw=ImageDraw.Draw(comparison)
    draw.text((10,8),'Arcaniste · nuque · avant / après ×4',font=font(14),fill='#efddbd')
    for row,(label,sprites) in enumerate([('Avant · tête trop haute',originals),('Après · tête rapprochée des épaules',actors)]):
        y=36+row*132;draw.text((10,y),label,font=font(12),fill='#d8e2df')
        for i,im in enumerate(sprites):
            crop=im.crop((24,0,40,20)).resize((64,80),Image.Resampling.NEAREST)
            comparison.paste(crop,(i*80+8,y+22),crop);draw.text((i*80+8,y+105),f'{i+1:02d}',font=font(10),fill='#d8e2df')
    comparison.save(DEST/'neck-before-after-x4.png')
    write_json(DOCS/'candidates.json',{'review_id':RID,'date':'2026-10-22','class_id':'core:arcaniste',
        'source_user_request':'sa nuque est trop haute ca fait bizarre','status':'awaiting_visual_review',
        'reference_acceptances':acceptances,'runtime_integration':False,'entries':entries})
    section=f'''<section id="arcaniste-channeler-wave-01" style="max-width:760px"><h2 id="{NAME}">Arcaniste · nuque raccourcie</h2>
<p>Les cinq têtes sont rapprochées des épaules. Les visages et les coiffes gardent leurs dessins ;
les capuches peuvent masquer une partie du visage. Seul le raccord du cou change.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="{NAME}/progression-native64.png" alt="Cinq Arcanistes à la nuque raccourcie, à taille réelle 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Cases de 64 × 64. Les corps gardent leur taille et leurs appuis.</p>
<details><summary>Comparer les nuques avant / après au zoom entier ×4</summary><div class="native-scroll"><img class="board" width="400" height="304" src="{NAME}/neck-before-after-x4.png" alt="Cinq raccords tête et épaules avant et après correction de la nuque au zoom entier quatre"></div></details>
<details><summary>Comparer les quatre classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="496" src="{NAME}/four-classes-native64.png" alt="Trois classes validées et Arcaniste à la nuque raccourcie dans la grille native 64 px"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="{NAME}/progression-zoom-x3.png" alt="Cinq tenues conservées avec une tête rapprochée du col au zoom entier trois"></div></details>
<p class="note">Correction à examiner. La tête appartient au torse. Casques indépendants, prises d’armes,
variantes et intégration restent différés. La rue est un montage.
<a href="../../docs/reviews/class-readability-01/{NAME}/REVIEW.md">Dossier de la correction</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/recipe.json">Composition et points d’attache</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    (OUT/'arcaniste-channeler-wave-01/section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':RID,'candidate_count':5,'native_dimensions':[64,64],
        'head_translation':[0,DY],'native_head_pixels_preserved':True,'native_resampling':False,
        'common_visible_face_preserved':True,'all_pixels_from_y13_exact':True,
        'native_pixel_changes_outside_attachment':[e['native_pixel_changes_outside_attachment'] for e in entries],
        'head_body_recomposition_exact':True,'source_pixels_outside_attachment_exact':True,
        'solid_heights':[core_box(a)[3]-core_box(a)[1] for a in actors],
        'previous_images_and_acceptances_unchanged':len(prior),'approved_compositions_unchanged':15,
        'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':
    if '--plan' in sys.argv:plan()
    else:main()
