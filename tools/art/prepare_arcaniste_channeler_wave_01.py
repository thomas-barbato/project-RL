"""Frame whole image_gen Arcaniste studies; preserve all accepted ranges."""
import json
import shutil
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,STREET,native_image,core_box,font,sha,write_json,gallery
from prepare_traqueuse_sniper_wave_01 import reference_sprites

NAME='arcaniste-channeler-wave-01'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
LABELS=['Très légère','Légère','Interméd.','Lourde','Très lourde']
HEAD_LABELS=['Tête nue','Bandeau protecteur','Coiffe renforcée','Casque ouvert','Casque intégral']
HEAD_POLYGONS={i:[(29,1),(35,1),(38,4),(38,8),(35,12),(29,12),(26,8),(26,4)] for i in range(1,6)}
HEAD_POLYGONS[5]=[(29,1),(35,1),(38,4),(38,8),(35,10),(29,10),(26,8),(26,4)]


def split_head(sprite,rank,cid):
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(HEAD_POLYGONS[rank],fill=255)
    head=Image.new('RGBA',(64,64));head.paste(sprite,(0,0),mask)
    body=sprite.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
    rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==sprite.tobytes()
    refs={}
    for part,im in [('head',head),('body',body),('head-mask',mask)]:
        path=ART/f'{cid}-{part}-native64.png';im.save(path)
        refs[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
    return refs,head,body


def boards(sprites,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS,
           class_name='Arcaniste',descriptor='canalisation',review_label=None,
           comparison_filename='four-classes-native64.png',
           head_crop_box=(24,0,40,16),audit_crop_box=(20,0,44,17)):
    board=Image.new('RGB',(400,400),'#152025');draw=ImageDraw.Draw(board)
    draw.text((10,8),f'{class_name} · {descriptor} · 64 × 64',font=font(15),fill='#efddbd')
    for i,label in enumerate(LABELS):
        draw.text((i*80+6,30),f'{i+1:02d}',font=font(11),fill='#d8e2df')
        draw.text((i*80+6,46),label,font=font(10),fill='#d8e2df')
    tile=Image.open(STREET).convert('RGB').crop((80,160,160,232))
    for row,(name,color) in enumerate([('Fond sombre','#343b3f'),('Fond clair','#858478'),('Sol de la rue · montage',None)]):
        y=62+110*row
        for i,actor in enumerate(sprites):
            x=i*80
            if color:draw.rectangle((x,y,x+79,y+71),fill=color)
            else:board.paste(tile,(x,y))
            board.paste(actor,(x+8,y+4),actor)
        draw.text((10,y+76),name,font=font(12),fill='#b5c5c3')
    board.save(destination/'progression-native64.png')
    zoom=Image.new('RGB',(624,500),'#243036');draw=ImageDraw.Draw(zoom)
    draw.text((10,8),f'{class_name} · cinq protections · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,actor in enumerate(sprites):
        x=8+(i%3)*208;y=34+(i//3)*232;z=actor.resize((192,192),Image.Resampling.NEAREST)
        zoom.paste(z,(x,y),z);draw.text((x,y+195),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d8e2df')
    zoom.save(destination/'progression-zoom-x3.png')
    count=len(references)+1
    comparison=Image.new('RGB',(400,48+112*count),'#152025');draw=ImageDraw.Draw(comparison)
    count_label={4:'Quatre',5:'Cinq'}.get(count,str(count))
    draw.text((10,8),f'{count_label} classes · même grille de 64 px',font=font(15),fill='#efddbd')
    for row,(name,actors) in enumerate([*references,(review_label or f'{class_name} · proposé',sprites)]):
        y=40+row*112;draw.text((10,y),name,font=font(13),fill='#d8e2df')
        draw.rectangle((0,y+23,399,y+94),fill='#343b3f')
        for i,actor in enumerate(actors):comparison.paste(actor,(8+i*80,y+27),actor)
    comparison.save(destination/comparison_filename)
    head_board=Image.new('RGB',(400,224),'#243036');draw=ImageDraw.Draw(head_board)
    draw.text((10,8),'Têtes du torse · natif puis zoom entier ×4',font=font(14),fill='#efddbd')
    for i,head in enumerate(heads):
        x=i*80;crop=head.crop(head_crop_box);head_board.paste(crop,(x+32,40),crop)
        draw.text((x+6,70),f'{i+1:02d}',font=font(11),fill='#d8e2df')
        z=crop.resize((64,64),Image.Resampling.NEAREST);head_board.paste(z,(x+8,96),z)
        draw.text((x+4,176),head_labels[i].replace(' ','\n',1),font=font(10),fill='#d8e2df')
    head_board.save(destination/'head-progression-native-x4.png')
    audit=Image.new('RGB',(960,474),'#243036');draw=ImageDraw.Draw(audit)
    for i in range(5):
        for row,im in enumerate([sprites[i],heads[i],bodies[i]]):
            crop=im.crop(audit_crop_box).resize((192,136),Image.Resampling.NEAREST)
            audit.paste(crop,(i*192,28+row*144),crop)
        draw.text((i*192+8,8),f'{i+1:02d} · entier / tête / corps',font=font(11),fill='#e1d4bc')
    audit.save(destination/'head-layers-audit-x8.png')


def main():
    if (REVIEW/'arcaniste-mage-heads-02/candidates.json').exists():raise SystemExit('Later head review exists; do not downgrade live preview.')
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted range frozen; create a revision.')
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'));assert len(plan['jobs'])==5
    sprites=[];heads=[];bodies=[];entries=[]
    for job in sorted(plan['jobs'],key=lambda j:j['rank']):
        rank,cid=job['rank'],job['candidate_id'];source=ROOT/job['source']
        sprite,measure=native_image(source,60);bounds=core_box(sprite)
        assert sprite.size==(64,64) and bounds[1]==2 and bounds[3]==62
        assert sprite.getchannel('A').getextrema()==(0,255)
        native=ART/f'{cid}-native64.png';sprite.save(native);shutil.copyfile(native,DEST/native.name)
        layers,head,body=split_head(sprite,rank,cid)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'class_name':'Arcaniste','class_id':'core:arcaniste','presentation_gender':'male',
            'status':'awaiting_native64_visual_review','source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),'native_dimensions':[64,64],
            'framing':measure,'native_core_box':list(bounds),'native_core_width':bounds[2]-bounds[0],'native_core_height':60,
            'solid_silhouette_pixels':sum(sprite.getchannel('A').histogram()[128:]),'foot_anchor':[32,61],
            'head_label':HEAD_LABELS[rank-1],'head_ownership_polygon':HEAD_POLYGONS[rank],'layers':layers,
            'head_body_recomposition_exact':True,'rig_status':'study_layers_only; independent helmets and weapon grips not prepared'})
        sprites.append(sprite);heads.append(head);bodies.append(body)
    references=[];acceptances=[]
    for folder,label in [('bastion-copper-wave-01','Bastion · validé'),('technomancien-no-robe-heads-02','Technomancien · validé'),('traqueuse-sniper-wave-01','Traqueuse · validée')]:
        actors,approval=reference_sprites(folder);references.append((label,actors));acceptances.append(approval)
    boards(sprites,heads,bodies,references)
    write_json(DOCS/'candidates.json',{'review_id':'ARCANISTE-CANALISATION-W01','date':'2026-10-22',
        'status':'awaiting_native64_visual_review','class_id':'core:arcaniste','class_name':'Arcaniste',
        'native_dimensions':[64,64],'native_height':60,'runtime_integration':False,'color_variants':'deferred',
        'reference_acceptances':acceptances,'entries':entries})
    section='''<section id="arcaniste-channeler-wave-01" style="max-width:760px"><h2>Arcaniste · cinq protections de canalisation</h2>
<p>Une silhouette élancée, un manteau prune et des protections de céramique claire.
Les avant-bras cuivrés rappellent la canalisation ; les coiffes puis les casques reprennent les matières du torse.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="arcaniste-channeler-wave-01/progression-native64.png" alt="Cinq tenues de l’Arcaniste avec leurs têtes compactes et progressives, en 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Chaque sprite occupe une case de 64 × 64.
L’Arcaniste garde ses 60 px de hauteur ; les autres silhouettes conservent leurs proportions validées.</p>
<details><summary>Comparer les quatre classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="496" src="arcaniste-channeler-wave-01/four-classes-native64.png" alt="Trois classes validées et cinq Arcanistes proposés dans une même grille de 64 px"></div></details>
<details><summary>Examiner les coiffes et les casques</summary><div class="native-scroll"><img class="board" width="400" height="224" src="arcaniste-channeler-wave-01/head-progression-native-x4.png" alt="Cinq têtes par défaut de l’Arcaniste à taille native puis au zoom entier quatre"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="arcaniste-channeler-wave-01/progression-zoom-x3.png" alt="Les cinq tenues de l’Arcaniste au zoom entier trois"></div></details>
<p class="note">Propositions à examiner. Les têtes appartiennent aux torses ; elles ne créent pas encore de casques équipables.
La rue est un montage sur une capture existante. Variantes, prises d’armes et intégration restent différées.
<a href="../../docs/reviews/class-readability-01/arcaniste-channeler-wave-01/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/arcaniste-channeler-wave-01/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'ARCANISTE-CANALISATION-W01','candidate_count':5,'native_dimensions':[64,64],
        'native_height':60,'native_widths':[e['native_core_width'] for e in entries],
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],'foot_baseline_y':61,
        'transparent_backgrounds':True,'head_body_recomposition_exact':True,
        'previous_images_and_acceptances_unchanged':len(prior),'approved_compositions_unchanged':15,
        'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
