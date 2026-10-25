"""New red-robed Arcaniste range, retaining the selected legacy sprite at 04."""
import json
import shutil
import sys
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,core_box,font,sha,write_json,native_image,gallery
from prepare_arcaniste_channeler_wave_01 import boards,LABELS
from prepare_traqueuse_sniper_wave_01 import reference_sprites

NAME='arcaniste-red-robe-wave-05'
RID='ARCANISTE-RED-ROBE-W05'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
HEAD_LABELS=['Tête nue','Capuche légère','Capuche doublée','Masque ancien','Masque renforcé']
LEGACY=ROOT/'assets/sprites/art-finish-02/FIN-16.png'
CONCEPT=ROOT/'assets/source-art/art-finish-review-02/FIN-16-source.png'
FACE=SOURCES/'arcaniste-same-face-03/AR-FACE03-identity-reference.png'


def plan():
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    assert not (DOCS/'prompts.json').exists()
    legacy_native=ART/'AR-RR05-legacy-reference-native64.png'
    # Keep all original sprite pixels; only remove its transparent 128px margins
    # and place the feet on the same y=61 baseline as the other review sprites.
    Image.open(LEGACY).convert('RGBA').crop((32,34,96,98)).save(legacy_native)
    descriptions={
        1:'''STAGE 01 / VERY LIGHT. A spare worn brick-red short sleeveless mage tunic, asymmetric red cloth tab reaching to the knee over charcoal trousers. Upper arms exposed, dark fabric wrist wraps, small practical gloves, light worn leather ankle boots. No torso shell, no shoulder plates, no long robe yet. White-haired pale adult man's bare compact head and small copper temple implant from the face reference. No hood or mask. Slender but adult and grounded, no broad warrior muscles. About 26 native pixels wide. This is the light underlayer from which the legacy red robe will grow.''',
        2:'''STAGE 02 / LIGHT. Worn brick-red coat extending just below the knees, narrow practical sleeves and simple reinforced leather cuffs. Small red soft cloth hood seated directly over the shoulders, pale face from the identity reference partly in hood shadow, short white fringe allowed but no head enlargement. Charcoal inner clothes, dark belt, leather gloves, slightly sturdier knee and boot guards. No hard breastplate, no broad shoulder mantle and no full mask. About 29 native pixels wide. Clear red mage clothing, not a tactical uniform.''',
        3:'''STAGE 03 / MEDIUM. A long ankle-length split RED robe, substantial but narrower and lighter than the legacy stage 04 reference. Layered charcoal inner tunic and dark leather chest/waist reinforcement, broad belt and one quiet pale central cloth panel. Robe sleeves now widen toward leather-reinforced cuffs, hands still fully visible. Hood has a thick folded cloth rim and covers temples; dark cloth wraps the lower face while the eye region stays in the hood shadow. No ivory full face mask yet, no scalp shell. Boots and knees gain dark protective covers visible under the open hem. About 33 native pixels wide. Protection reads as layered red cloth and dark leather, not a suit of ivory plates.''',
        5:'''STAGE 05 / VERY HEAVY. Build beyond the legacy stage 04 RED hooded masked robe. Thicker short layered shoulder cowl in deep brick red, a broad folded hood around the same warm ivory faceted full-face mask with ONE dark vertical slit. Keep the mask the same normal human-head size as the legacy, not a giant helmet. The robe is now a broad double layered protective gown, heavy overlapping red panels over charcoal inner armor, large leather belt, broad reinforced sleeves, thick dark forearm guards with a few matte aged steel edges, protective gloves and heavy leather/steel-toed boots. A small dark breast reinforcement is partly concealed by the robe. Shoulder volume comes from padded cloth/cowl; no giant hard melee pauldrons, no segmented ivory knight cuirass. Make the heavy protection and increased thickness visibly stronger than stage 04, especially upper arms, cuffs and layered robe flanks. About 43 native pixels wide. Retain the distinctive red robe, hood and ivory slit mask as the dominant identity.'''}
    common='''Use case: stylized-concept.
Asset: ONE whole full-body ARCANISTE mage outfit for Project RL, to be reduced to a 60-pixel-high figure inside a 64x64 game cell. This is one member of a five-stage armor progression.
Reference image 1 is the LEGACY STYLE reference selected by the user: worn deep red robe, dark inner clothes and belts, red hood and simple warm ivory full face mask with one vertical dark slot. Its hands and feet give the characteristic grounded mage pose. It is a late armor, stage 04, not the starting outfit. Ignore its black background, aura, grunge speckles and microtexture: draw a clean complete transparent figure.
Reference image 2 is the exact existing 64px legacy SPRITE: use its economical broad value masses, slightly elevated front-facing game view, compact proportions, and balanced pose. Avoid a side or three-quarter portrait and avoid a superhero/anime rendering.
Adult human male, lean torso under the cloth, normal compact head seated low between the shoulders, short neck hidden naturally by collar/hood. The uncovered face where needed is the same pale short-white-haired man across the range, not a different person. At high stages the mask hides him completely. Forearms remain the channeling focus, with protection in dark leather, muted aged metal or discreet implant cuffs. No new powers or devices.
Keep a consistent nearly frontal stance: two feet apart, both arms slightly bent and held a little away from the torso at waist level, hands empty and fully drawn. Same anatomy and body height across the range. The figure's added width must come from clothing thickness and protection, not different poses or oversized anatomy. Full crown and soles within generous transparent margins.
Palette and finish: matte worn BRICK RED / OXBLOOD cloth with broad reddish lit folds, charcoal black-brown underclothes and old dark leather straps, limited dull steel, warm ivory only in selected mask or quiet cloth panels. A little restrained dull copper may appear on existing forearm fittings. No shiny gold edging, no orange flecks, no neon lights, no bright magical effects. Restrained polished painterly game-sprite finish, large readable cloth folds and material masses. No scratch noise, embroidery, runes, rivets, chains or tiny gadgets. No big thick black outlines around every fold. The red cloth must remain legible against a dark grey floor.
Each armor changes torso, arms, gloves, legs and boots together. Headwear grows with the outfit but stays in the same compact head area. Do not lengthen the neck or perch the head above a tall collar. Mage identity must remain different from the Bastion's copper melee plates and the Technomancien's exposed robot machinery.
No weapon, staff, wand, book, orb, flame, particles, aura, cast shadow, ground, text or presentation board. One whole figure on genuine transparent RGBA only.
'''
    jobs=[]
    for rank in (1,2,3,5):
        refs=[{'path':CONCEPT.relative_to(ROOT).as_posix(),'sha256':sha(CONCEPT),'role':'selected late-armor style only; exclude opaque background and effects'},
            {'path':legacy_native.relative_to(ROOT).as_posix(),'sha256':sha(legacy_native),'role':'exact legacy sprite silhouette, pose, palette and native-size readability'}]
        prompt=common+'\n'+descriptions[rank]
        if rank in (1,2):
            refs.append({'path':FACE.relative_to(ROOT).as_posix(),'sha256':sha(FACE),'role':'same underlying human face identity only; do not copy the old plum outfit or portrait scale'})
            prompt+='\nReference image 3 is ONLY the underlying human facial identity for exposed areas: same white hair, pale face, brow, nose and small copper temple implant. Match the legacy painterly finish and requested red outfit, not the crop lighting or portrait size. Hood may partially cover the face in 02.'
        jobs.append({'rank':rank,'candidate_id':f'AR-RR05-P{rank:02d}-r01','prompt':prompt,'tool':'built_in_image_gen',
            'transparent_background':True,'references':refs,'generated_file':None})
    write_json(DOCS/'prompts.json',{'review_id':RID,'date':'2026-10-22','mode':'built_in_image_gen',
        'selected_reference_answer':'01 — Ancien personnage en jeu : robe rouge, capuche et masque',
        'latest_user_request':"cette version robe rouge, capuche et masque pourrait etre l'une des dernieres armures :)",
        'reused_rank':4,'reused_native':legacy_native.relative_to(ROOT).as_posix(),
        'reused_native_framing':'FIN-16 crop (32,34,96,98), original pixels unchanged, feet y=61',
        'provisional_previous_approval':'bon je valide pour l\'instant — superseded by explicit preference for the old in-game style; no acceptance manifest created.',
        'jobs':jobs})
    protected=[]
    for root in (SOURCES,OUT,REVIEW):protected.extend(p for p in root.rglob('*.png') if NAME not in p.parts)
    protected.extend(REVIEW.rglob('acceptance.json'))
    write_json(DOCS/'prior-images.json',{p.relative_to(ROOT).as_posix():sha(p) for p in protected})
    print(json.dumps({'generated_jobs':[1,2,3,5],'legacy_reused_rank':4,'protected_files':len(protected)}))


HEAD_POLYGONS={
    1:[(29,0),(35,0),(37,4),(37,9),(35,12),(29,12),(27,9),(27,4)],
    2:[(29,0),(35,0),(39,6),(39,11),(36,14),(28,14),(25,11),(25,6)],
    3:[(29,0),(35,0),(39,6),(40,11),(37,15),(27,15),(24,11),(25,6)],
    4:[(29,0),(35,0),(39,6),(40,11),(37,15),(27,15),(24,11),(25,6)],
    5:[(29,0),(35,0),(39,6),(40,11),(37,15),(27,15),(24,11),(25,6)]}


def main():
    assert not (DOCS/'acceptance.json').exists(),'Accepted range frozen; prepare a revision.'
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    jobs={j['rank']:j for j in plan['jobs']}
    actors=[];heads=[];bodies=[];entries=[]
    for rank in range(1,6):
        cid=f'AR-RR05-P{rank:02d}-r01';source=ART/f'{cid}-source.png'
        if rank==4:
            shutil.copyfile(LEGACY,source)
            actor=Image.open(ROOT/plan['reused_native']).convert('RGBA')
            framing={'source_core_box':list(core_box(Image.open(source).convert('RGBA'))),
                'source_crop':[32,34,96,98],'isotropic_scale':1,'native_core_box':list(core_box(actor)),
                'native_core_height':60,'native_core_width':40,'foot_baseline_y':61,
                'framing':'Original 128px runtime sprite cropped to 64px logical cell and feet aligned; no resampling.'}
            assert actor.tobytes()==Image.open(LEGACY).convert('RGBA').crop((32,34,96,98)).tobytes()
        else:
            assert sha(source)==jobs[rank]['source_sha256']
            actor,framing=native_image(source,60)
        assert actor.size==(64,64) and actor.getchannel('A').getextrema()==(0,255)
        assert core_box(actor)[3]==62
        mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(HEAD_POLYGONS[rank],fill=255)
        head=Image.new('RGBA',(64,64));head.paste(actor,(0,0),mask)
        body=actor.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
        rebuilt=body.copy();rebuilt.alpha_composite(head)
        # The legacy PNG stores RGB in eleven fully transparent head pixels.
        # Preserve its exact raw pixels using the binary ownership mask too.
        pasted=body.copy();pasted.paste(head,(0,0),mask);assert pasted.tobytes()==actor.tobytes()
        assert rebuilt.getchannel('A').tobytes()==actor.getchannel('A').tobytes()
        assert all(rebuilt.getpixel((x,y))==actor.getpixel((x,y))
            for y in range(64) for x in range(64) if actor.getpixel((x,y))[3]>0)
        layers={}
        for label,im in [('head',head),('body',body),('head-mask',mask)]:
            p=ART/f'{cid}-{label}-native64.png';im.save(p);layers[label]={'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}
        native=ART/f'{cid}-native64.png';actor.save(native);shutil.copyfile(native,DEST/native.name)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'class_name':'Arcaniste','class_id':'core:arcaniste','presentation_gender':'male',
            'status':'awaiting_native64_visual_review','source':source.relative_to(ROOT).as_posix(),
            'source_sha256':sha(source),'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
            'native_dimensions':[64,64],'framing':framing,'native_core_box':list(core_box(actor)),
            'native_core_width':core_box(actor)[2]-core_box(actor)[0],
            'native_core_height':core_box(actor)[3]-core_box(actor)[1],
            'solid_silhouette_pixels':sum(actor.getchannel('A').histogram()[128:]),'foot_anchor':[32,61],
            'head_label':HEAD_LABELS[rank-1],'head_ownership_polygon':HEAD_POLYGONS[rank],
            'layers':layers,'head_body_recomposition_exact':True,'recomposition_method':'binary ownership paste; alpha composite exact at every visible pixel',
            'legacy_sprite_reused_exactly':rank==4,
            'rig_status':'whole-outfit study only; independent helmet replacement, weapon grips and animation pending'})
        actors.append(actor);heads.append(head);bodies.append(body)
    references=[];approvals=[]
    for folder,label in [('bastion-copper-wave-01','Bastion · validé'),('technomancien-no-robe-heads-02','Technomancien · validé'),('traqueuse-sniper-wave-01','Traqueuse · validée')]:
        ref,a=reference_sprites(folder);references.append((label,ref));approvals.append(a)
    boards(actors,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS)
    witness=Image.new('RGB',(544,400),'#243036');draw=ImageDraw.Draw(witness)
    draw.text((10,8),'Ancienne silhouette et palier très lourd',font=font(16),fill='#efddbd')
    for i,(label,actor) in enumerate([('04 · ancienne silhouette exacte',actors[3]),('05 · protection renforcée',actors[4])]):
        x=8+i*272;draw.text((x,36),label,font=font(13),fill='#d8e2df')
        witness.paste(actor,(x+96,62),actor)
        zoom=actor.resize((256,256),Image.Resampling.NEAREST);witness.paste(zoom,(x,136),zoom)
    witness.save(DEST/'legacy-and-heavy-native-x4.png')
    write_json(DOCS/'candidates.json',{'review_id':RID,'date':'2026-10-22','status':'awaiting_native64_visual_review',
        'class_id':'core:arcaniste','native_dimensions':[64,64],'legacy_reference_rank':4,
        'reference_acceptances':approvals,'runtime_integration':False,'color_variants':'deferred','entries':entries})
    section=f'''<section id="arcaniste-channeler-wave-01" style="max-width:760px"><h2 id="{NAME}">Arcaniste · retour à la robe rouge</h2>
<p>La progression rejoint l’ancien mage à capuche rouge et masque ivoire.
Le palier 04 reprend exactement son ancien sprite ; le 05 épaissit la tenue et ses protections.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="{NAME}/progression-native64.png" alt="Cinq Arcanistes en tenue rouge du très léger au très lourd, avec l’ancienne silhouette au palier quatre, en 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Cases de 64 × 64. La robe, la capuche puis le masque prennent progressivement leur place.</p>
<details><summary>Comparer l’ancienne robe et le palier très lourd</summary><div class="native-scroll"><img class="board" width="544" height="400" src="{NAME}/legacy-and-heavy-native-x4.png" alt="Ancien sprite exact au palier quatre et proposition très lourde au cinq, en natif puis au zoom entier quatre"></div></details>
<details><summary>Comparer les quatre classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="496" src="{NAME}/four-classes-native64.png" alt="Trois classes validées et nouvelle progression rouge de l’Arcaniste à taille réelle 64 px"></div></details>
<details><summary>Examiner les coiffes et les masques</summary><div class="native-scroll"><img class="board" width="400" height="224" src="{NAME}/head-progression-native-x4.png" alt="Progression de la tête nue à la capuche et au masque de l’Arcaniste, natif puis zoom entier quatre"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="{NAME}/progression-zoom-x3.png" alt="Progression des cinq tenues rouges de l’Arcaniste au zoom entier trois"></div></details>
<p class="note">Nouvelle gamme à examiner. Les précédentes tenues prune restent archivées.
Les têtes appartiennent aux torses ; casques indépendants, variantes, prises d’armes et intégration restent différés.
La rue est un montage.
<a href="../../docs/reviews/class-readability-01/{NAME}/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    (OUT/'arcaniste-channeler-wave-01/section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':RID,'candidate_count':5,'new_drawings':4,'legacy_sprite_reused_rank':4,
        'legacy_native_pixels_exact':True,'native_dimensions':[64,64],
        'native_widths':[e['native_core_width'] for e in entries],
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],
        'head_body_recomposition_exact':True,'foot_baseline_y':61,
        'prior_images_and_acceptances_unchanged':len(prior),'approved_other_class_compositions_unchanged':15,
        'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':
    if '--plan' in sys.argv:plan()
    else:main()
