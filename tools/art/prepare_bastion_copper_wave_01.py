"""Frame the copper Bastion progression, retaining the two reference studies."""
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, native_image, core_box, font, sha,
    write_json, gallery,
)

NAME='bastion-copper-wave-01'
ART, DEST, DOCS=SOURCES/NAME, OUT/NAME, REVIEW/NAME
LABELS=['Très légère','Légère','Interméd.','Lourde','Très lourde']
HEAD_POLYGONS={
    1:[(29,3),(35,3),(36,6),(36,10),(34,12),(29,12),(27,10),(27,6)],
    2:[(29,3),(35,3),(36,6),(36,10),(34,11),(29,11),(27,10),(27,6)],
    3:[(29,3),(35,3),(37,6),(37,10),(35,12),(29,12),(27,10),(27,6)],
    4:[(29,3),(35,3),(37,6),(37,10),(35,12),(29,12),(27,10),(27,6)],
    5:[(29,3),(35,3),(37,6),(37,10),(35,12),(29,12),(27,10),(27,6)],
}


def boards(sprites):
    board=Image.new('RGB',(480,400),'#152025');draw=ImageDraw.Draw(board)
    draw.text((10,8),'Bastion · cinq degrés de protection · 64 × 64',font=font(15),fill='#efddbd')
    for i,label in enumerate(LABELS):
        draw.text((i*96+5,38),f'{i+1:02d} · {label}',font=font(11),fill='#d8e2df')
    tile=Image.open(STREET).convert('RGB').crop((80,160,176,232))
    for row,(name,color) in enumerate([('Fond sombre','#343b3f'),('Fond clair','#858478'),('Sol de la rue · montage',None)]):
        y=62+110*row
        for i,sprite in enumerate(sprites):
            x=96*i
            if color:draw.rectangle((x,y,x+95,y+71),fill=color)
            else:board.paste(tile,(x,y))
            board.paste(sprite,(x+16,y+4),sprite)
        draw.text((10,y+76),name,font=font(12),fill='#b5c5c3')
    board.save(DEST/'progression-native64.png')
    zoom=Image.new('RGB',(624,500),'#243036');draw=ImageDraw.Draw(zoom)
    draw.text((10,8),'Bastion · contrôle des volumes · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        x=8+(i%3)*208;y=34+(i//3)*232
        enlarged=sprite.resize((192,192),Image.Resampling.NEAREST)
        zoom.paste(enlarged,(x,y),enlarged)
        draw.text((x,y+195),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d8e2df')
    zoom.save(DEST/'progression-zoom-x3.png')


def split_head(native,rank,cid):
    polygon=HEAD_POLYGONS[rank]
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(polygon,fill=255)
    head=Image.new('RGBA',(64,64));head.paste(native,(0,0),mask)
    body=native.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
    result=body.copy();result.alpha_composite(head)
    assert result.tobytes()==native.tobytes()
    output={}
    for part,image in [('head',head),('body',body),('head-mask',mask)]:
        path=ART/f'{cid}-{part}-native64.png';image.save(path)
        output[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
    return output,polygon


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted wave is frozen.')
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    old=json.loads((REVIEW/'armor-style-studies-02/candidates.json').read_text('utf-8'))
    jobs={j['rank']:j for j in plan['jobs']}
    reuse={r['rank']:next(e for e in old['entries'] if e['candidate_id']==r['candidate_id']) for r in plan['reuse']}
    sprites=[];entries=[]
    for rank in range(1,6):
        if rank in reuse:
            ref=reuse[rank];cid=ref['candidate_id']
            source=ROOT/ref['source'];assert sha(source)==ref['source_sha256']
            path=ROOT/ref['native'];assert sha(path)==ref['native_sha256']
            native=Image.open(path).convert('RGBA');measure=ref['source_framing']
            reference={'candidate_id':cid,'native':ref['native'],'native_sha256':ref['native_sha256'],'pixels_preserved':True}
        else:
            job=jobs[rank];cid=job['candidate_id']
            source=ART/(cid+'-source.png');generated=ROOT/job['generated_file']
            if generated.exists():shutil.copyfile(generated,source)
            assert source.exists()
            assert Image.open(source).convert('RGBA').getchannel('A').getextrema()==(0,255)
            native,measure=native_image(source,58)
            path=ART/(cid+'-native64.png');native.save(path)
            reference=None
        bounds=core_box(native)
        assert native.size==(64,64) and bounds[3]==62 and bounds[1]==4
        assert native.getchannel('A').getextrema()==(0,255)
        shutil.copyfile(path,DEST/(cid+'-native64.png'))
        layers,head_polygon=split_head(native,rank,cid)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'class_name':'Bastion',
            'rank':rank,'label':LABELS[rank-1],'status':'new_progression_composition_pending_visual_review',
            'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':path.relative_to(ROOT).as_posix(),'native_sha256':sha(path),
            'framing':measure,'native_core_box':list(bounds),
            'native_core_width':bounds[2]-bounds[0],'native_core_height':bounds[3]-bounds[1],
            'foot_anchor':[32,61],'head_ownership_polygon':head_polygon,
            'layers':layers,'head_body_recompose_exactly':True,'retained_reference':reference,
            'rig_status':'study_layers_only_arms_grips_and_helmet_replacement_not_yet_validated'})
        sprites.append(native)
    widths=[e['native_core_width'] for e in entries]
    assert widths[2]<widths[3]<widths[4],f'Heavy armor must grow without stretching: {widths}'
    boards(sprites)
    write_json(DOCS/'candidates.json',{'review_id':'BASTION-COPPER-W01','date':'2026-10-21',
        'status':'awaiting_native64_visual_review','native_dimensions':[64,64],
        'runtime_integration':False,'color_variants':'deferred','entries':entries})
    section='''<section id="bastion-copper-wave-01"><h2>Bastion · du très léger au blindage brutal</h2>
<p>Cinq degrés de protection, à la même hauteur. Les épaules, le torse, les avant-bras et les jambes deviennent progressivement plus massifs.
Les grandes surfaces cuivre restent sobres.</p>
<div class="native-scroll"><img class="board" width="480" height="400" src="bastion-copper-wave-01/progression-native64.png" alt="Cinq armures du Bastion, de très légère à très lourde, en 64 px sur trois fonds"></div>
<p class="note">02 reprend exactement le Vétéran apprécié ; 03 reprend exactement le Briseur cuivre.
01, 04 et 05 sont de nouveaux dessins. Cette numérotation décrit les cinq paliers de la gamme.</p>
<details><summary>Examiner les volumes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="bastion-copper-wave-01/progression-zoom-x3.png" alt="Les cinq armures du Bastion agrandies trois fois sans lissage"></div></details>
<p class="note">Les coiffes présentées appartiennent à la tête par défaut du torse. Casques équipables et prises d’armes restent à préparer.
La rue est un montage sur une capture existante. Aucun dessin n’est intégré au jeu.
<a href="../../docs/reviews/class-readability-01/bastion-copper-wave-01/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/bastion-copper-wave-01/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'BASTION-COPPER-W01','native_compositions':5,'new_drawings':3,
        'retained_references_unchanged':2,'native_dimensions':[64,64],
        'silhouette_widths':widths,'same_native_height':58,'foot_baseline_y':61,
        'heavy_silhouette_width_increases_without_stretch':True,'head_body_recomposition_exact':True,
        'all_backgrounds_transparent':True,'prior_images_and_acceptances_unchanged':len(prior),
        'runtime_integration':False,'scene_validation':'static montage; engine lighting not yet reviewed'}
    write_json(DOCS/'checks.json',checks)
    print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
