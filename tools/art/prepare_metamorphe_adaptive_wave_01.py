"""Frame human female Metamorphe outfit proposals, preserving 20 approved references."""
import json
import shutil
import sys
from PIL import Image,ImageDraw
from prepare_class_readability_review_01 import ROOT,SOURCES,OUT,REVIEW,STREET,core_box,font,sha,write_json,native_image,gallery
from prepare_traqueuse_sniper_wave_01 import reference_sprites
from prepare_arcaniste_channeler_wave_01 import boards

NAME='metamorphe-adaptive-wave-01'
RID='METAMORPHE-ADAPTIVE-W01'
ART,DEST,DOCS=SOURCES/NAME,OUT/NAME,REVIEW/NAME
BASE=SOURCES/'CL05-r02-source.png'
FINISH=SOURCES/'traqueuse-sniper-wave-01/accepted/sources/TR-SN01-P03-r01-source.png'
LABELS=['Très légère','Légère','Interméd.','Lourde','Très lourde']
HEAD_LABELS=['Tête nue','Bandeau','Coiffe souple','Casque ouvert','Casque renforcé']

COMMON='''Use case: stylized-concept.
Asset: ONE whole METAMORPHE FEMALE HUMAN player character outfit for a cyberpunk roguelike, designed to read as a 56-pixel-high figure inside a 64x64 game cell. One outfit from a five-stage very-light to very-heavy progression. Entire figure from crown to soles, all hands and both feet, genuinely transparent RGBA, generous empty margin. No floor, cast shadow, glow, aura, effects, text or board.
Identity: the same adult HUMAN WOMAN through all outfits, warm light olive skin, short tousled dark-brown hair, strong brows and determined feminine face, natural muscular arms and thick athletic thighs. Compact robust proportions, moderate height, lower center of gravity, slightly flexed knees and a springy balanced stance. Her strength is muscular and human, not giant bodybuilder proportions. Practical fully covered adult clothing, no exposed midriff, bikini, exaggerated breasts or pinup pose. Small normal head seated low into the shoulders, short natural neck, never a separate tall head or bulky helmet. Head including protection about 8-9 native pixels high.
Starting skills are Mue prédatrice and Puissance métamorphe: they inspire physical mobility and curved protective forms only. This is her HUMAN RESTING FORM, not a transformation. No beast anatomy, claws, horns, antlers, fangs, animal ears, tails, wings, digitigrade legs, fur coat, skull trophies, gore or living growth from her body. All protective parts are clearly WORN equipment, not a new biological power.
Class identity: close-fitting flexible MOSS/SAGE GREEN textile and dark umber elastic/leather joints, wearing a few broad warm PALE BONE-COLORED BIOCOMPOSITE shells. These are engineered biomimetic shells, not real bones or medieval steel. The shell shapes are smooth rounded crescents and curved overlapping ribs, hugging the strong torso and limbs and leaving obvious flexible gaps. Use a few large plates and quiet surfaces, not dozens of scales. Curved shoulders and forearm wraps, a stable short torso, compact hips and powerful legs make her distinct. NO broad square copper brawler pauldrons, ivory knight armor, grey robot chassis, sniper blue uniform, robe, cape, long coat, loincloth skirt, druid vines or fantasy ornaments.
Palette: medium muted moss and sage-green cloth, lighter olive cloth planes, broad warm chalk/ivory shell highlights, muted ochre/umber straps, dark brown-charcoal joints and boots. Keep the green visible at every stage, and the head readable on a dark floor. No gold trim, scattered orange LEDs, neon veins, magic lights or multicolor accents. Two or three broad calm values per material, subtle matte wear without scratch/rivet noise. Protective volume must read at 56px, not rely on high-resolution microdetail.
Finish: polished restrained painterly game-sprite illustration, clean whole silhouette and readable material masses with economical folds, subtle upper-left light, slightly elevated nearly frontal game view. No anime portrait rendering, photo textures or thick comic outline around every small plate. Same human height, face and stance across the five outfits; armor thickness, coverage and headwear evolve. Two EMPTY HUMAN hands slightly away from hips, relaxed natural fingers, feet separated and grounded. No weapon, shield, staff, gadgets or attack animation.
The torso outfit covers torso, shoulders, arms/gloves, trousers/legs and boots, and provides default headwear. Every region evolves with the armor. Propose visuals only, no new equipment mechanics, transformation behavior or stats.
'''

STAGES={
    1:'''STAGE 01 / VERY LIGHT. Bare compact female head with short brown tousled hair. Practical sleeveless moss-green cropped-at-hip vest over a quiet pale cloth undershirt, no midriff. Bare muscular upper arms and forearms, simple dark hand wraps and thin gloves. No hard chest shell and no shoulder plates. Dark olive flexible trousers, small fabric knee pads and light worn brown ankle boots. One small pale existing wrist protector is enough. Minimal coverage; clearly much lighter than a full suit. About 25-27 native pixels wide.''',
    2:'''STAGE 02 / LIGHT. Same feminine face and short hair, low pale protective forehead band, no full cap. Fitted moss-green short-sleeve jacket with a small curved pale sternum insert and light flexible ribs at the sides. Modest curved forearm cuffs, light elbow wraps, reinforced cloth gloves, olive trousers with small rounded knee pads and stronger ankle boots. Shoulders and much of the arms remain lightly protected. No shoulder bulk, no full chest shell. About 27-28 native pixels wide.''',
    3:'''STAGE 03 / MEDIUM. Fully worn fitted green flexible suit with a broad smooth pale curved chest protector and two restrained smaller curved side ribs. Short rounded shoulder caps, curved ivory forearm shells and muted ochre fastening cuffs, sturdy gloves. Olive thighs with small outer thigh guards, rounded knees and pale wraparound shin guards above practical boots. Keep large dark elastic gaps at shoulders, elbows, groin and knees. HEAD: fitted moss-green fabric skullcap with one low broad ivory brow/temple protector, entire feminine face visible, no crown or helmet ear discs. Short hair fringe may show. Medium protection, clearly not the heaviest. About 29-31 native pixels wide.''',
    4:'''STAGE 04 / HEAVY. A thicker rounded biocomposite chest shell hugging the torso, overlapping broad crescent side plates, fuller curved upper-arm caps and substantial forearm shells over green flexible sleeves. Broad guarded gloves, curved outer-hip/thigh protection, large rounded knee and shin shells, heavy green/brown boots with pale protective toes. Flexible dark joints remain readable and the human frame stays compact. HEAD: compact olive/ivory open-face protective cap, smooth pale brow arch and short temple/cheek guards, no snout, no large crown or neck stalk. Same feminine eyes, nose and mouth visible inside. About 32-34 native pixels wide. No square warrior shoulder blocks or plate skirt.''',
    5:'''STAGE 05 / VERY HEAVY. Maximum protective volume in a compact rounded biological-inspired engineered suit. Thick broad PALE curved thorax shell, two overlapping crescent side ribs, rounded layered shoulder/upper-arm cups and large wraparound forearm shells with strong gloved hands. Green flexible waist and dark elastic gaps stay visible, guarded hips and thighs without a skirt, thick rounded knees and deep shin/boot protection. Increase shell thickness and coverage unmistakably beyond stage 04 while retaining a crouch-ready athletic woman, not a giant knight or robot. HEAD: compact close-fitting moss-green and ivory full protective cap with a low curved brow and simple short cheek/jaw protection; narrow dark eye opening may partly obscure her face. Human head size, no beast skull, ears, horns, fangs or visor glow. Head seated close to shoulders. About 35-37 native pixels wide.'''}


def plan():
    for p in (ART,DEST,DOCS):p.mkdir(parents=True,exist_ok=True)
    assert not (DOCS/'prompts.json').exists()
    pilot={'rank':3,'candidate_id':'ME-AD01-P03-r01','tool':'built_in_image_gen','transparent_background':True,
        'prompt':COMMON+'\n'+STAGES[3]+'''\nInput image 1 is the earlier FEMALE identity/silhouette reference: keep her adult feminine facial identity, brown hair and muscular human build only. Replace the old ragged clothes with the stated new adaptive outfit. Input image 2 is a FINISH/readability reference only: use its restrained game-sprite volumes, not its body, blue palette, tactical armor, visor or cybernetic limbs. Build this distinctly different class.''',
        'references':[{'path':BASE.relative_to(ROOT).as_posix(),'sha256':sha(BASE),'role':'female human identity and build only; old clothes not retained'},
            {'path':FINISH.relative_to(ROOT).as_posix(),'sha256':sha(FINISH),'role':'approved game finish only; no sniper gear or robotic limb designs'}],
        'source':None}
    write_json(DOCS/'prompts.json',{'review_id':RID,'date':'2026-10-22','mode':'built_in_image_gen',
        'source_user_request':'parfait on continue vers la métamorphe','pilot_rank':3,'common_prompt':COMMON,
        'stage_descriptions':{str(k):v for k,v in STAGES.items()},'jobs':[pilot],
        'class_reference':'content/core/classes/metamorphe.json5','class_reference_sha256':sha(ROOT/'content/core/classes/metamorphe.json5'),
        'starting_skills':['M01','M03'],'native_height':56,'human_resting_form_only':True})
    protected=[]
    for root in (SOURCES,OUT,REVIEW):protected.extend(p for p in root.rglob('*.png') if NAME not in p.parts)
    protected.extend(REVIEW.rglob('acceptance.json'))
    write_json(DOCS/'prior-images.json',{p.relative_to(ROOT).as_posix():sha(p) for p in protected})
    print(json.dumps({'pilot':3,'protected_files':len(protected)}))


HEAD_POLYGON=[(29,4),(35,4),(38,8),(38,13),(35,17),(29,17),(26,13),(26,8)]


def main():
    if (DOCS/'rejection.json').exists():raise SystemExit('User changed direction to druid; do not publish superseded adaptive armor.')
    assert not (DOCS/'acceptance.json').exists(),'Accepted range frozen; create a revision.'
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'));assert len(plan['jobs'])==5
    actors=[];heads=[];bodies=[];entries=[]
    for job in sorted(plan['jobs'],key=lambda j:j['rank']):
        rank,cid=job['rank'],job['candidate_id'];source=ROOT/job['source']
        assert sha(source)==job['source_sha256']
        actor,framing=native_image(source,56);bounds=core_box(actor)
        assert actor.size==(64,64) and bounds[1]==6 and bounds[3]==62
        assert actor.getchannel('A').getextrema()==(0,255)
        mask=Image.new('L',(64,64));ImageDraw.Draw(mask).polygon(HEAD_POLYGON,fill=255)
        head=Image.new('RGBA',(64,64));head.paste(actor,(0,0),mask)
        body=actor.copy();body.paste((0,0,0,0),(0,0,64,64),mask)
        rebuilt=body.copy();rebuilt.alpha_composite(head);assert rebuilt.tobytes()==actor.tobytes()
        layers={}
        for part,im in [('head',head),('body',body),('head-mask',mask)]:
            p=ART/f'{cid}-{part}-native64.png';im.save(p);layers[part]={'path':p.relative_to(ROOT).as_posix(),'sha256':sha(p)}
        native=ART/f'{cid}-native64.png';actor.save(native);shutil.copyfile(native,DEST/native.name)
        entries.append({'number':f'{rank:02d}','candidate_id':cid,'rank':rank,'label':LABELS[rank-1],
            'class_name':'Métamorphe','class_id':'core:metamorphe','presentation_gender':'female','form':'human_resting',
            'status':'awaiting_native64_visual_review','source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),'native_dimensions':[64,64],
            'framing':framing,'native_core_box':list(bounds),'native_core_width':bounds[2]-bounds[0],
            'native_core_height':bounds[3]-bounds[1],'solid_silhouette_pixels':sum(actor.getchannel('A').histogram()[128:]),
            'head_label':HEAD_LABELS[rank-1],'head_ownership_polygon':HEAD_POLYGON,'layers':layers,
            'head_body_recomposition_exact':True,'foot_anchor':[32,61],
            'rig_status':'study layers only; helmet override, weapon grips and transformation forms not modified'})
        actors.append(actor);heads.append(head);bodies.append(body)
    references=[];approvals=[]
    for folder,label in [('bastion-copper-wave-01','Bastion · validé'),('technomancien-no-robe-heads-02','Technomancien · validé'),('traqueuse-sniper-wave-01','Traqueuse · validée'),('arcaniste-red-robe-wave-05','Arcaniste · validé')]:
        ref,a=reference_sprites(folder);references.append((label,ref));approvals.append(a)
    boards(actors,heads,bodies,references,destination=DEST,head_labels=HEAD_LABELS,class_name='Métamorphe',
        descriptor='protections courbes',review_label='Métamorphe · proposée',comparison_filename='five-classes-native64.png',
        head_crop_box=(24,4,40,20),audit_crop_box=(20,4,44,21))
    write_json(DOCS/'candidates.json',{'review_id':RID,'date':'2026-10-22','class_id':'core:metamorphe',
        'status':'awaiting_native64_visual_review','presentation_gender':'female','form':'human_resting',
        'native_dimensions':[64,64],'native_height':56,'starting_skill_ids':['M01','M03'],
        'reference_acceptances':approvals,'runtime_integration':False,'color_variants':'deferred','entries':entries})
    section=f'''<section id="{NAME}" style="max-width:760px"><h2>Métamorphe · cinq protections aux formes organiques</h2>
<p>Une femme musclée, une posture ramassée et des protections claires asymétriques sur du tissu vert mousse.
La couverture augmente du vêtement léger à la tenue renforcée, tout en gardant des articulations souples.</p>
<div class="native-scroll"><img class="board" width="400" height="400" src="{NAME}/progression-native64.png" alt="Cinq tenues féminines de la Métamorphe du très léger au très lourd, en 64 px sur trois fonds"></div>
<p class="note">01–05 : très légère à très lourde. Cases de 64 × 64 ; silhouette humaine de 56 px.
La Métamorphe est montrée au repos, sans transformation ni arme.</p>
<details><summary>Voir les 25 armures des cinq classes à taille réelle</summary><div class="native-scroll"><img class="board" width="400" height="608" src="{NAME}/five-classes-native64.png" alt="Vingt armures validées des quatre autres classes et cinq Métamorphes proposées, toutes en grille native 64 px"></div></details>
<details><summary>Examiner les coiffes et les casques</summary><div class="native-scroll"><img class="board" width="400" height="224" src="{NAME}/head-progression-native-x4.png" alt="Tête nue, bandeau, coiffe et casques progressifs de la Métamorphe, natif puis zoom entier quatre"></div></details>
<details><summary>Examiner les cinq silhouettes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="500" src="{NAME}/progression-zoom-x3.png" alt="Cinq tenues de la Métamorphe au zoom entier trois"></div></details>
<p class="note">Propositions à examiner. Les vingt références des autres classes sont conservées à l’identique.
Les têtes appartiennent aux torses ; casques indépendants, variantes, prises d’armes et intégration restent différés.
La rue est un montage. Aucun effet de transformation n’est modifié.
<a href="../../docs/reviews/class-readability-01/{NAME}/REVIEW.md">Dossier de la gamme</a> ·
<a href="../../docs/reviews/class-readability-01/{NAME}/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'));gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':RID,'candidate_count':5,'native_dimensions':[64,64],'native_height':56,
        'native_widths':[e['native_core_width'] for e in entries],
        'solid_silhouette_pixels':[e['solid_silhouette_pixels'] for e in entries],'foot_baseline_y':61,
        'head_body_recomposition_exact':True,'joint_native_review_dimensions':[400,608],'joint_armor_count':25,
        'other_accepted_compositions_unchanged':20,'prior_images_and_acceptances_unchanged':len(prior),
        'human_resting_form_only':True,'runtime_integration':False,'engine_execution':False,'ui_execution':'pending_browser_check'}
    write_json(DOCS/'checks.json',checks);print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':
    if '--plan' in sys.argv:plan()
    else:main()
