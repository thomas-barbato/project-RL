"""Frame image_gen robe-free range; no runtime changes or semantic drawing."""
import json
import shutil
from PIL import Image, ImageDraw
from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, native_image, core_box, font, sha,
    write_json, gallery,
)
NAME='technomancien-no-robe-01'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
LABELS=['Très légère','Légère','Interméd.','Lourde','Très lourde']
HEAD_POLYGON=[(29,3),(34,3),(36,6),(36,10),(34,13),(29,13),(27,10),(27,6)]


def split_head(sprite,cid):
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(HEAD_POLYGON,fill=255)
    head=Image.new('RGBA',(64,64));head.paste(sprite,(0,0),mask)
    body=sprite.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
    rebuilt=body.copy();rebuilt.alpha_composite(head)
    assert rebuilt.tobytes()==sprite.tobytes()
    files={}
    for part,im in [('head',head),('body',body),('head-mask',mask)]:
        path=ART/f'{cid}-{part}-native64.png';im.save(path)
        files[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
    return files,head,body


def boards(sprites,bastions,heads,bodies,destination=DEST,cell_width=96):
    board=Image.new('RGB',(5*cell_width,400),'#152025');draw=ImageDraw.Draw(board)
    draw.text((10,8),'Technomancien · sans robe · 64 × 64',font=font(15),fill='#efddbd')
    for i,label in enumerate(LABELS):
        if cell_width==96:draw.text((i*96+5,38),f'{i+1:02d} · {label}',font=font(11),fill='#d8e2df')
        else:
            draw.text((i*cell_width+6,30),f'{i+1:02d}',font=font(11),fill='#d8e2df')
            draw.text((i*cell_width+6,46),label,font=font(10),fill='#d8e2df')
    tile=Image.open(STREET).convert('RGB').crop((80,160,80+cell_width,232))
    for row,(name,color) in enumerate([('Fond sombre','#343b3f'),('Fond clair','#858478'),('Sol de la rue · montage',None)]):
        y=62+110*row
        for i,sprite in enumerate(sprites):
            x=cell_width*i
            if color:draw.rectangle((x,y,x+cell_width-1,y+71),fill=color)
            else:board.paste(tile,(x,y))
            board.paste(sprite,(x+(cell_width-64)//2,y+4),sprite)
        draw.text((10,y+76),name,font=font(12),fill='#b5c5c3')
    board.save(destination/'progression-native64.png')
    zoom=Image.new('RGB',(624,500),'#243036');draw=ImageDraw.Draw(zoom)
    draw.text((10,8),'Technomancien · coques mécaniques · zoom ×3',font=font(16),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        x=8+(i%3)*208;y=34+(i//3)*232
        enlarged=sprite.resize((192,192),Image.Resampling.NEAREST);zoom.paste(enlarged,(x,y),enlarged)
        draw.text((x,y+195),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d8e2df')
    zoom.save(destination/'progression-zoom-x3.png')
    comparison=Image.new('RGB',(5*cell_width,272),'#152025');draw=ImageDraw.Draw(comparison)
    draw.text((10,8),'Deux identités · même échelle native',font=font(16),fill='#efddbd')
    for row,(name,actors) in enumerate([('Bastion · validé',bastions),('Technomancien sans robe · proposé',sprites)]):
        y=40+row*112;draw.text((10,y),name,font=font(13),fill='#d8e2df')
        draw.rectangle((0,y+23,5*cell_width-1,y+94),fill='#343b3f')
        for i,sprite in enumerate(actors):comparison.paste(sprite,((cell_width-64)//2+cell_width*i,y+27),sprite)
    comparison.save(destination/'bastion-comparison-native64.png')
    audit=Image.new('RGB',(960,474),'#243036');draw=ImageDraw.Draw(audit)
    for i in range(5):
        for row,im in enumerate([sprites[i],heads[i],bodies[i]]):
            crop=im.crop((20,1,44,18)).resize((192,136),Image.Resampling.NEAREST)
            audit.paste(crop,(i*192,28+row*144),crop)
        draw.text((i*192+8,8),f'{i+1:02d} · entier / tête / corps',font=font(11),fill='#e1d4bc')
    audit.save(destination/'head-layers-audit-x8.png')


def main():
    if (REVIEW/'technomancien-no-robe-heads-02/candidates.json').exists():raise SystemExit('Current head revision exists; do not downgrade the live review.')
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted range frozen; create a revision.')
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    assert len(plan['jobs'])==5
    anchor_sha=sha(ART/'TE-NR01-P03-r01-native64.png')
    sprites=[];heads=[];bodies=[];entries=[]
    for job in sorted(plan['jobs'],key=lambda j:j['rank']):
        rank,cid=job['rank'],job['candidate_id'];source=ROOT/job['source']
        sprite,measure=native_image(source,58);bounds=core_box(sprite)
        assert bounds[1]==4 and bounds[3]==62 and sprite.size==(64,64)
        assert sprite.getchannel('A').getextrema()==(0,255)
        native=ART/f'{cid}-native64.png';sprite.save(native);shutil.copyfile(native,DEST/native.name)
        layers,head,body=split_head(sprite,cid)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'status':'awaiting_native64_visual_review','source':source.relative_to(ROOT).as_posix(),
            'source_sha256':sha(source),'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
            'framing':measure,'native_core_box':list(bounds),'native_core_width':bounds[2]-bounds[0],
            'native_core_height':58,'solid_silhouette_pixels':sum(sprite.getchannel('A').histogram()[128:]),
            'foot_anchor':[32,61],'head_ownership_polygon':HEAD_POLYGON,'layers':layers,
            'head_body_recomposition_exact':True,'rig_status':'study_layers_only; equippable helmets and weapon grips not yet prepared'})
        sprites.append(sprite);heads.append(head);bodies.append(body)
    assert sha(ART/'TE-NR01-P03-r01-native64.png')==anchor_sha
    accepted=json.loads((REVIEW/'bastion-copper-wave-01/acceptance.json').read_text('utf-8'))
    bastions=[]
    for e in accepted['entries']:
        ref=e['native'];assert sha(ROOT/ref['path'])==ref['sha256']
        bastions.append(Image.open(ROOT/ref['path']).convert('RGBA'))
    boards(sprites,bastions,heads,bodies)
    write_json(DOCS/'candidates.json',{'review_id':'TECHNOMANCIEN-NOROBE01','date':'2026-10-22',
        'status':'awaiting_native64_visual_review','class_id':'core:technomancien','runtime_integration':False,
        'scope':'Five robe-free protection stages; stage03 identity witness preserved.',
        'native_dimensions':[64,64],'production_color_variants':'deferred',
        'future_color_rule':'Red-cloth-only tint intent retained historically; no extension to this steel chassis inferred.',
        'bastion_comparison_acceptance':'docs/reviews/class-readability-01/bastion-copper-wave-01/acceptance.json',
        'entries':entries})
    section='''<section id="technomancien-no-robe-01" style="max-width:760px"><h2>Technomancien · cinq protections sans robe</h2>
<p>Du châssis découvert à la carapace renforcée : cinq paliers sans robe ni capuche.
Les coques épaississent autour des articulations mécaniques ; la tête reste compacte.</p>
<div class="native-scroll"><img class="board" width="480" height="400" src="technomancien-no-robe-01/progression-native64.png" alt="Cinq protections du Technomancien sans robe ni capuche, en 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère, légère, intermédiaire, lourde et très lourde.
Chaque personnage occupe une case de 64 × 64, avec la même hauteur de 58 px.</p>
<details><summary>Comparer avec le Bastion à taille réelle</summary><div class="native-scroll"><img class="board" width="480" height="272" src="technomancien-no-robe-01/bastion-comparison-native64.png" alt="Bastion cuivre accepté et Technomancien mécanique sans robe, cinq paliers à même échelle"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="technomancien-no-robe-01/progression-zoom-x3.png" alt="Cinq protections mécaniques du Technomancien au zoom entier trois"></div></details>
<p class="note">Nouveaux dessins à examiner. Le Bastion accepté reste conservé. La rue est un montage sur une capture existante.
Les têtes montrées appartiennent aux torses. Casques équipables, prises d’armes, variantes et intégration restent différés.
<a href="../../docs/reviews/class-readability-01/technomancien-no-robe-01/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/technomancien-no-robe-01/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'TECHNOMANCIEN-NOROBE01','candidate_count':5,'native_dimensions':[64,64],
        'native_height':58,'native_widths':[e['native_core_width'] for e in entries],
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],
        'foot_baseline_y':61,'transparent_backgrounds':True,'head_body_recomposition_exact':True,
        'stage03_witness_native_preserved':True,'source_reduction':'isotropic Lanczos; alpha retained',
        'previous_images_and_acceptances_unchanged':len(prior),'bastion_approved_compositions_unchanged':5,
        'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
