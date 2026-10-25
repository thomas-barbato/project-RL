"""Package whole image_gen sniper/ninja studies; no game resource integration."""
import json
import shutil
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,STREET,native_image,core_box,font,sha,write_json,gallery

NAME='traqueuse-sniper-wave-01'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
LABELS=['Très légère','Légère','Interméd.','Lourde','Très lourde']
HEAD_LABELS=['Tête découverte','Cagoule souple','Demi-casque','Casque fermé','Casque intégral']
HEAD_POLYGONS={i:[(29,5),(35,5),(38,8),(38,13),(35,16),(29,16),(26,13),(26,8)] for i in range(1,6)}
HEAD_POLYGONS[5]=[(29,5),(35,5),(38,8),(38,12),(35,14),(29,14),(26,12),(26,8)]


def split_head(sprite,rank,cid):
    polygon=HEAD_POLYGONS[rank]
    mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(polygon,fill=255)
    head=Image.new('RGBA',(64,64));head.paste(sprite,(0,0),mask)
    body=sprite.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
    rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==sprite.tobytes()
    refs={}
    for part,im in [('head',head),('body',body),('head-mask',mask)]:
        path=ART/f'{cid}-{part}-native64.png';im.save(path)
        refs[part]={'path':path.relative_to(ROOT).as_posix(),'sha256':sha(path)}
    return refs,head,body


def reference_sprites(name):
    acceptance=REVIEW/name/'acceptance.json';data=json.loads(acceptance.read_text('utf-8'))
    sprites=[]
    for e in data['entries']:
        r=e['native'];assert sha(ROOT/r['path'])==r['sha256'];sprites.append(Image.open(ROOT/r['path']).convert('RGBA'))
    return sprites,{'path':acceptance.relative_to(ROOT).as_posix(),'sha256':sha(acceptance)}


def boards(sprites,heads,bodies,bastions,technomanciens):
    board=Image.new('RGB',(400,400),'#152025');draw=ImageDraw.Draw(board)
    draw.text((10,8),'Traqueuse · tireuse / ninja · 64 × 64',font=font(15),fill='#efddbd')
    for i,label in enumerate(LABELS):
        draw.text((i*80+6,30),f'{i+1:02d}',font=font(11),fill='#d8e2df')
        draw.text((i*80+6,46),label,font=font(10),fill='#d8e2df')
    tile=Image.open(STREET).convert('RGB').crop((80,160,160,232))
    for row,(name,color) in enumerate([('Fond sombre','#343b3f'),('Fond clair','#858478'),('Sol de la rue · montage',None)]):
        y=62+110*row
        for i,actor in enumerate(sprites):
            x=80*i
            if color:draw.rectangle((x,y,x+79,y+71),fill=color)
            else:board.paste(tile,(x,y))
            board.paste(actor,(x+8,y+4),actor)
        draw.text((10,y+76),name,font=font(12),fill='#b5c5c3')
    board.save(DEST/'progression-native64.png')
    zoom=Image.new('RGB',(624,500),'#243036');draw=ImageDraw.Draw(zoom)
    draw.text((10,8),'Traqueuse · cinq protections · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,actor in enumerate(sprites):
        x=8+(i%3)*208;y=34+(i//3)*232;z=actor.resize((192,192),Image.Resampling.NEAREST)
        zoom.paste(z,(x,y),z);draw.text((x,y+195),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d8e2df')
    zoom.save(DEST/'progression-zoom-x3.png')
    comparison=Image.new('RGB',(400,384),'#152025');draw=ImageDraw.Draw(comparison)
    draw.text((10,8),'Trois classes · même grille de 64 px',font=font(15),fill='#efddbd')
    for row,(name,actors) in enumerate([('Bastion · validé',bastions),('Technomancien · validé',technomanciens),('Traqueuse · proposée',sprites)]):
        y=40+row*112;draw.text((10,y),name,font=font(13),fill='#d8e2df')
        draw.rectangle((0,y+23,399,y+94),fill='#343b3f')
        for i,actor in enumerate(actors):comparison.paste(actor,(8+i*80,y+27),actor)
    comparison.save(DEST/'three-classes-native64.png')
    heads_board=Image.new('RGB',(400,224),'#243036');draw=ImageDraw.Draw(heads_board)
    draw.text((10,8),'Têtes du torse · natif puis zoom entier ×4',font=font(14),fill='#efddbd')
    for i,head in enumerate(heads):
        x=i*80;crop=head.crop((24,4,40,20));heads_board.paste(crop,(x+32,40),crop)
        draw.text((x+6,70),f'{i+1:02d}',font=font(11),fill='#d8e2df')
        z=crop.resize((64,64),Image.Resampling.NEAREST);heads_board.paste(z,(x+8,96),z)
        draw.text((x+4,176),HEAD_LABELS[i].replace(' ','\n',1),font=font(10),fill='#d8e2df')
    heads_board.save(DEST/'head-progression-native-x4.png')
    audit=Image.new('RGB',(960,474),'#243036');draw=ImageDraw.Draw(audit)
    for i in range(5):
        for row,im in enumerate([sprites[i],heads[i],bodies[i]]):
            crop=im.crop((20,3,44,20)).resize((192,136),Image.Resampling.NEAREST)
            audit.paste(crop,(i*192,28+row*144),crop)
        draw.text((i*192+8,8),f'{i+1:02d} · entier / tête / corps',font=font(11),fill='#e1d4bc')
    audit.save(DEST/'head-layers-audit-x8.png')


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted range frozen; create a revision.')
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'));assert len(plan['jobs'])==5
    sprites=[];heads=[];bodies=[];entries=[]
    for job in sorted(plan['jobs'],key=lambda j:j['rank']):
        rank,cid=job['rank'],job['candidate_id'];source=ROOT/job['source']
        sprite,measure=native_image(source,56);bounds=core_box(sprite)
        assert sprite.size==(64,64) and bounds[1]==6 and bounds[3]==62
        assert sprite.getchannel('A').getextrema()==(0,255)
        native=ART/f'{cid}-native64.png';sprite.save(native);shutil.copyfile(native,DEST/native.name)
        layers,head,body=split_head(sprite,rank,cid)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'class_name':'Traqueuse','class_id':'core:traqueur','presentation_gender':'female',
            'status':'awaiting_native64_visual_review','source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),'native_dimensions':[64,64],
            'framing':measure,'native_core_box':list(bounds),'native_core_width':bounds[2]-bounds[0],'native_core_height':56,
            'solid_silhouette_pixels':sum(sprite.getchannel('A').histogram()[128:]),'foot_anchor':[32,61],
            'head_label':HEAD_LABELS[rank-1],'head_ownership_polygon':HEAD_POLYGONS[rank],'layers':layers,
            'head_body_recomposition_exact':True,'rig_status':'study_layers_only; independent helmets and weapon grips not prepared'})
        sprites.append(sprite);heads.append(head);bodies.append(body)
    bastions,bastion_acceptance=reference_sprites('bastion-copper-wave-01')
    technomanciens,technomancien_acceptance=reference_sprites('technomancien-no-robe-heads-02')
    boards(sprites,heads,bodies,bastions,technomanciens)
    write_json(DOCS/'candidates.json',{'review_id':'TRAQUEUSE-SNIPER-W01','date':'2026-10-22',
        'status':'awaiting_native64_visual_review','class_id':'core:traqueur','class_name':'Traqueuse','presentation_gender':'female',
        'native_dimensions':[64,64],'native_height':56,'runtime_integration':False,'color_variants':'deferred',
        'reference_acceptances':[bastion_acceptance,technomancien_acceptance],'entries':entries})
    section='''<section id="traqueuse-sniper-wave-01" style="max-width:760px"><h2>Traqueuse · cinq protections de tireuse / ninja</h2>
<p>Une femme athlétique, combinaison bleu ardoise, protections asymétriques et augmentations localisées.
Du textile léger à l’exocombinaison blindée, la tête se protège aussi progressivement.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="traqueuse-sniper-wave-01/progression-native64.png" alt="Cinq armures de la Traqueuse tireuse ninja avec leurs têtes progressives, en 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Chaque sprite occupe une case de 64 × 64.
La Traqueuse mesure 56 px ; les deux autres classes gardent leurs 58 px approuvés.</p>
<details><summary>Comparer les trois classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="384" src="traqueuse-sniper-wave-01/three-classes-native64.png" alt="Bastion et Technomancien validés comparés aux cinq Traqueuses proposées, en cases de 64 px"></div></details>
<details><summary>Examiner les têtes et leurs protections</summary><div class="native-scroll"><img class="board" width="400" height="224" src="traqueuse-sniper-wave-01/head-progression-native-x4.png" alt="Cinq têtes par défaut de la Traqueuse, à taille native puis au zoom entier quatre"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="traqueuse-sniper-wave-01/progression-zoom-x3.png" alt="Les cinq tenues de la Traqueuse au zoom entier trois"></div></details>
<p class="note">Propositions à examiner. Les têtes font partie des torses ; elles ne créent pas encore de casques équipables.
La rue est un montage sur une capture existante. Variantes, prises d’armes et intégration restent différées.
<a href="../../docs/reviews/class-readability-01/traqueuse-sniper-wave-01/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/traqueuse-sniper-wave-01/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'TRAQUEUSE-SNIPER-W01','candidate_count':5,'native_dimensions':[64,64],
        'native_height':56,'native_widths':[e['native_core_width'] for e in entries],
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],'foot_baseline_y':61,
        'transparent_backgrounds':True,'head_body_recomposition_exact':True,
        'previous_images_and_acceptances_unchanged':len(prior),'approved_bastion_compositions_unchanged':5,
        'approved_technomancien_compositions_unchanged':5,'runtime_integration':False,
        'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
