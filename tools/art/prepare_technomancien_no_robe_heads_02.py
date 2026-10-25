"""Composite image_gen head edits only, preserving every non-head native pixel."""
import json
import shutil
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,core_box,font,sha,write_json,gallery
from prepare_technomancien_no_robe_01 import boards,LABELS

NAME='technomancien-no-robe-heads-02'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
HEAD_POLYGON=[(28,3),(35,3),(37,5),(37,9),(35,10),(34,12),(29,12),(28,10),(26,9),(26,5)]


def fixed_project(image,original,framing):
    frame=tuple(framing['source_crop']);scale=framing['isotropic_scale']
    cropped=original.crop(frame);size=(round(cropped.width*scale),round(cropped.height*scale))
    reduced=cropped.resize(size,Image.Resampling.LANCZOS);b=core_box(reduced)
    dx=round(32-(b[0]+b[2])/2);dy=62-b[3]
    output=Image.new('RGBA',(64,64))
    output.alpha_composite(image.crop(frame).resize(size,Image.Resampling.LANCZOS),(dx,dy))
    return output,(dx,dy),size,frame


def head_stats(im,mask):
    values=[im.getpixel((x,y)) for y in range(64) for x in range(64)
            if mask.getpixel((x,y)) and im.getpixel((x,y))[3]>=128]
    lum=[.2126*r+.7152*g+.0722*b for r,g,b,a in values]
    return {'solid_pixels':len(values),'mean_luma_0_255':round(sum(lum)/len(lum),2),
            'pixels_luma_above_120':sum(v>=120 for v in lum)}


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted review frozen; create a revision.')
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    previous=json.loads((REVIEW/'technomancien-no-robe-01/candidates.json').read_text('utf-8'))
    prior_by_rank={e['rank']:e for e in previous['entries']}
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(HEAD_POLYGON,fill=255)
    sprites=[];heads=[];bodies=[];entries=[];originals=[]
    for job in sorted(plan['jobs'],key=lambda j:j['rank']):
        rank,cid=job['rank'],job['candidate_id'];old=prior_by_rank[rank]
        source=Image.open(ROOT/old['source']).convert('RGBA')
        generated=Image.open(ROOT/job['generated_file']).convert('RGBA')
        assert source.size==generated.size,(rank,source.size,generated.size)
        old_native=Image.open(ROOT/old['native']).convert('RGBA')
        projected,offset,size,frame=fixed_project(source,source,old['framing'])
        assert projected.tobytes()==old_native.tobytes()
        edited,_,_,_=fixed_project(generated,source,old['framing'])
        native=old_native.copy();native.paste(edited,(0,0),mask)
        outside_changes=sum(old_native.getpixel((x,y))!=native.getpixel((x,y))
            for y in range(64) for x in range(64) if not mask.getpixel((x,y)))
        assert outside_changes==0
        assert native.crop((0,13,64,64)).tobytes()==old_native.crop((0,13,64,64)).tobytes()
        assert native.size==(64,64) and core_box(native)[3]==62
        # Store a high-resolution composite too. Only an explicit mask selects
        # generated head pixels; no code paints or transforms material colors.
        dx,dy=offset
        local_mask=mask.crop((dx,dy,dx+size[0],dy+size[1]))
        source_mask=Image.new('L',source.size)
        source_mask.paste(local_mask.resize((frame[2]-frame[0],frame[3]-frame[1]),Image.Resampling.NEAREST),(frame[0],frame[1]))
        final_source=source.copy();final_source.paste(generated,(0,0),source_mask)
        source_path=ART/f'{cid}-source.png';final_source.save(source_path)
        source_mask_path=ART/f'{cid}-head-mask-source.png';source_mask.save(source_mask_path)
        native_path=ART/f'{cid}-native64.png';native.save(native_path);shutil.copyfile(native_path,DEST/native_path.name)
        head=Image.new('RGBA',(64,64));head.paste(native,(0,0),mask)
        body=native.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
        old_body=old_native.copy();old_body.paste((0,0,0,0),(0,0,64,64),mask)
        assert body.tobytes()==old_body.tobytes()
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==native.tobytes()
        layers={}
        for part,im in [('head',head),('body',body),('head-mask',mask)]:
            path=ART/f'{cid}-{part}-native64.png';im.save(path)
            layers[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
        before,after=head_stats(old_native,mask),head_stats(native,mask)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'status':'awaiting_native64_visual_review','original_candidate_id':old['candidate_id'],
            'source':source_path.relative_to(ROOT).as_posix(),'source_sha256':sha(source_path),
            'generated_head_edit':job['generated_file'],'generated_head_edit_sha256':sha(ROOT/job['generated_file']),
            'source_head_mask':source_mask_path.relative_to(ROOT).as_posix(),
            'native':native_path.relative_to(ROOT).as_posix(),'native_sha256':sha(native_path),
            'native_dimensions':[64,64],'native_core_box':list(core_box(native)),
            'framing':old['framing'],'head_region_polygon':HEAD_POLYGON,'layers':layers,
            'non_head_native_pixel_changes':outside_changes,'body_below_y13_exact':True,
            'head_body_recomposition_exact':True,'foot_anchor':[32,61],
            'head_luma_before':before,'head_luma_after':after,
            'rig_status':'default_head_study_only; equippable helmet and weapon grips remain pending'})
        sprites.append(native);heads.append(head);bodies.append(body);originals.append(old_native)
    accepted=json.loads((REVIEW/'bastion-copper-wave-01/acceptance.json').read_text('utf-8'))
    bastions=[]
    for e in accepted['entries']:
        ref=e['native'];assert sha(ROOT/ref['path'])==ref['sha256'];bastions.append(Image.open(ROOT/ref['path']).convert('RGBA'))
    boards(sprites,bastions,heads,bodies,DEST,cell_width=80)
    board=Image.new('RGB',(480,280),'#243036');draw=ImageDraw.Draw(board)
    draw.text((10,8),'Têtes · avant / après · zoom entier ×5',font=font(16),fill='#efddbd')
    for row,(name,actors) in enumerate([('Avant · mêmes crânes sombres',originals),('Après · protection progressive',sprites)]):
        y=36+row*118;draw.text((10,y),name,font=font(13),fill='#d8e2df')
        for i,actor in enumerate(actors):
            crop=actor.crop((24,2,40,18)).resize((80,80),Image.Resampling.NEAREST)
            board.paste(crop,(i*96+8,y+22),crop)
            draw.text((i*96+8,y+101),f'{i+1:02d}',font=font(10),fill='#d8e2df')
    board.save(DEST/'heads-before-after-x5.png')
    write_json(DOCS/'candidates.json',{'review_id':'TECHNOMANCIEN-NOROBE-HEADS02','date':'2026-10-22',
        'status':'awaiting_native64_visual_review','class_id':'core:technomancien','runtime_integration':False,
        'native_dimensions':[64,64],'head_scope':'Brighter progressively protected default heads, not independent equippable helmets.',
        'body_preservation':'Every native pixel outside explicit head region is identical to robe-free r01.',
        'entries':entries})
    section='''<section id="technomancien-no-robe-01" style="max-width:760px"><h2 id="technomancien-no-robe-heads-02">Technomancien · têtes plus claires et progressives</h2>
<p>Cinq protections sans robe ni capuche. La tête gagne des surfaces acier claires,
puis des renforts de front, de tempes et de visage jusqu’au casque fermé.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="technomancien-no-robe-heads-02/progression-native64.png" alt="Cinq Technomanciens sans robe avec têtes plus claires et protection progressive, en 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Les corps et leurs couleurs restent identiques.
Chaque sprite occupe une case de 64 × 64 ; les casques montrés font partie de la tête par défaut du torse.</p>
<details><summary>Comparer les têtes avant / après au zoom entier ×5</summary><div class="native-scroll"><img class="board" width="480" height="280" src="technomancien-no-robe-heads-02/heads-before-after-x5.png" alt="Anciennes têtes sombres et cinq nouvelles protections plus claires au zoom cinq"></div></details>
<details><summary>Comparer avec le Bastion à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="272" src="technomancien-no-robe-heads-02/bastion-comparison-native64.png" alt="Bastion accepté et Technomancien sans robe avec têtes révisées en 64 px"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="technomancien-no-robe-heads-02/progression-zoom-x3.png" alt="Cinq Technomanciens avec têtes corrigées au zoom entier trois"></div></details>
<p class="note">Nouvelles têtes à examiner. La rue est un montage sur une capture existante.
Les casques équipables indépendants, prises d’armes, variantes et intégration restent différés.
<a href="../../docs/reviews/class-readability-01/technomancien-no-robe-heads-02/REVIEW.md">Dossier de la correction</a> ·
<a href="../../docs/reviews/class-readability-01/technomancien-no-robe-heads-02/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    (OUT/'technomancien-no-robe-01/section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'TECHNOMANCIEN-NOROBE-HEADS02','candidate_count':5,
        'native_dimensions':[64,64],'head_masks_explicit':True,'head_body_recomposition_exact':True,
        'non_head_native_pixel_changes':[e['non_head_native_pixel_changes'] for e in entries],
        'head_luma_before':[e['head_luma_before'] for e in entries],
        'head_luma_after':[e['head_luma_after'] for e in entries],
        'previous_images_and_acceptances_unchanged':len(prior),'bastion_approved_compositions_unchanged':5,
        'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
