"""Prepare native64 machine-priest studies; preserve all Bastion approvals."""
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, native_image, core_box, font, sha,
    write_json, gallery,
)

NAME='technomancien-machine-priest-01'
ART, DEST, DOCS=SOURCES/NAME, OUT/NAME, REVIEW/NAME
LABELS=['Très légère','Légère','Interméd.','Lourde','Très lourde']
HEAD_POLYGONS={rank:[(29,3),(35,3),(37,6),(37,11),(34,14),(29,14),(26,11),(26,6)] for rank in range(1,6)}


def boards(sprites,bastions):
    board=Image.new('RGB',(480,400),'#152025');draw=ImageDraw.Draw(board)
    draw.text((10,8),'Technomancien · prêtre-machine · 64 × 64',font=font(15),fill='#efddbd')
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
    draw.text((10,8),'Technomancien · volumes · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        x=8+(i%3)*208;y=34+(i//3)*232
        enlarged=sprite.resize((192,192),Image.Resampling.NEAREST)
        zoom.paste(enlarged,(x,y),enlarged)
        draw.text((x,y+195),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d8e2df')
    zoom.save(DEST/'progression-zoom-x3.png')
    comparison=Image.new('RGB',(480,272),'#152025');draw=ImageDraw.Draw(comparison)
    draw.text((10,8),'Deux identités · même échelle native',font=font(16),fill='#efddbd')
    for row,(name,actors) in enumerate([('Bastion · validé',bastions),('Technomancien · proposé',sprites)]):
        y=40+row*112
        draw.text((10,y),name,font=font(13),fill='#d8e2df')
        draw.rectangle((0,y+23,479,y+94),fill='#343b3f')
        for i,sprite in enumerate(actors):comparison.paste(sprite,(16+96*i,y+27),sprite)
    comparison.save(DEST/'bastion-comparison-native64.png')


def split_head(sprite,rank,cid):
    polygon=HEAD_POLYGONS[rank]
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(polygon,fill=255)
    head=Image.new('RGBA',(64,64));head.paste(sprite,(0,0),mask)
    body=sprite.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
    rebuilt=body.copy();rebuilt.alpha_composite(head)
    assert rebuilt.tobytes()==sprite.tobytes()
    files={}
    for part,image in [('head',head),('body',body),('head-mask',mask)]:
        path=ART/f'{cid}-{part}-native64.png';image.save(path)
        files[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
    return files,polygon


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted wave is frozen; create a revision.')
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    assert len(plan['jobs'])==5 and all(j.get('generated_file') for j in plan['jobs'])
    accepted=json.loads((REVIEW/'bastion-copper-wave-01/acceptance.json').read_text('utf-8'))
    bastions=[]
    for e in accepted['entries']:
        ref=e['native'];assert sha(ROOT/ref['path'])==ref['sha256']
        bastions.append(Image.open(ROOT/ref['path']).convert('RGBA'))
    sprites=[];entries=[]
    for job in sorted(plan['jobs'],key=lambda j:j['rank']):
        rank,cid=job['rank'],job['candidate_id']
        source=ART/(cid+'-source.png');generated=ROOT/job['generated_file']
        if generated.exists():shutil.copyfile(generated,source)
        assert source.exists()
        assert Image.open(source).convert('RGBA').getchannel('A').getextrema()==(0,255)
        native,measure=native_image(source,58)
        bounds=core_box(native)
        assert native.size==(64,64) and bounds[1]==4 and bounds[3]==62
        path=ART/(cid+'-native64.png');native.save(path)
        shutil.copyfile(path,DEST/path.name)
        layers,polygon=split_head(native,rank,cid)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'class_name':'Technomancien',
            'rank':rank,'label':LABELS[rank-1],'status':'awaiting_native64_visual_review',
            'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':path.relative_to(ROOT).as_posix(),'native_sha256':sha(path),
            'framing':measure,'native_core_box':list(bounds),
            'native_core_width':bounds[2]-bounds[0],'native_core_height':bounds[3]-bounds[1],
            'solid_silhouette_pixels':sum(native.getchannel('A').histogram()[128:]),
            'foot_anchor':[32,61],'head_ownership_polygon':polygon,'layers':layers,
            'head_body_recomposition_exact':True,
            'rig_status':'study_layers_only_arms_grips_and_equippable_helmet_replacement_not_yet_validated'})
        sprites.append(native)
    boards(sprites,bastions)
    write_json(DOCS/'candidates.json',{'review_id':'TECHNOMANCIEN-MP01','date':'2026-10-22',
        'status':'awaiting_native64_visual_review','native_dimensions':[64,64],
        'class_id':'core:technomancien','runtime_integration':False,'color_variants':'deferred',
        'bastion_comparison_acceptance':'docs/reviews/class-readability-01/bastion-copper-wave-01/acceptance.json',
        'entries':entries})
    section='''<section id="technomancien-machine-priest-01" style="max-width:760px"><h2>Technomancien · prêtre-machine sombre</h2>
<p>Cinq protections du léger au lourd : capuche compacte, visage mécanique et manteau fendu.
Le col, le châssis et les pans blindés gagnent en épaisseur ; la silhouette reste verticale.</p>
<div class="native-scroll"><img class="board" width="480" height="400" src="technomancien-machine-priest-01/progression-native64.png" alt="Cinq armures du Technomancien prêtre-machine sombre en 64 px sur trois fonds"></div>
<p class="note">01–05 sont les paliers de cette nouvelle gamme du Technomancien. Les anciens essais restent archivés.
Chaque personnage occupe une case de 64 × 64, à la même hauteur que le Bastion validé.</p>
<details><summary>Comparer les deux classes à taille réelle</summary><div class="native-scroll"><img class="board" width="480" height="272" src="technomancien-machine-priest-01/bastion-comparison-native64.png" alt="Bastion validé et Technomancien proposé, cinq paliers chacun à même échelle native"></div></details>
<details><summary>Examiner les volumes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="technomancien-machine-priest-01/progression-zoom-x3.png" alt="Les cinq tenues du Technomancien au zoom entier trois"></div></details>
<p class="note">Propositions à examiner. Les têtes montrées appartiennent au torse ; les casques équipables et prises d’armes restent à préparer.
La rue est un montage sur une capture existante. L’intégration et les variantes de couleur restent différées.
<a href="../../docs/reviews/class-readability-01/technomancien-machine-priest-01/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/technomancien-machine-priest-01/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'TECHNOMANCIEN-MP01','native_compositions':5,'native_dimensions':[64,64],
        'native_widths':[e['native_core_width'] for e in entries],'same_native_height':58,
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],
        'foot_baseline_y':61,'source_reduction':'isotropic Lanczos, source alpha retained',
        'head_body_recomposition_exact':True,'transparent_backgrounds':True,
        'bastion_accepted_compositions_unchanged':5,'prior_images_and_acceptances_unchanged':len(prior),
        'runtime_integration':False,'scene_validation':'static montage; engine lighting not yet reviewed'}
    write_json(DOCS/'checks.json',checks)
    print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
