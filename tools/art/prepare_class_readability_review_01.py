"""Package generated silhouette proposals; never modifies active game resources.

Image operations are limited to alpha-preserving framing, native-size reduction
and review layouts. Semantic drawing changes are performed with image_gen.
"""
from pathlib import Path
import csv
import hashlib
import html
import json
import re
import shutil

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'artifacts/class-armor-review-01'
REVIEW = ROOT / 'docs/reviews/class-readability-01'
SOURCES = ROOT / 'assets/source-art/class-readability-01'
GENERATED = Path(r'C:\Users\User\.codex\generated_images\01a15e4b-a136-72f0-b79d-bca96fc58aaa')
STREET = ROOT / 'assets/source-art/first-sector-review-01/u01-style-r07/references/street-native-lighting.png'

PROFILES = [
    ('01', 'bastion', 'Bastion', 'FIN-01', 58,
     'exec-e17db00a-c6bc-4469-92f3-068af79e6d1b.png', 'r04',
     'Large et robuste ; bras puissants, appuis solides, augmentation faciale proposée en r04.'),
    ('02', 'technomancien', 'Technomancien', 'FIN-14', 58,
     'exec-1ce635b0-88f2-4a38-9551-7be0d9b46914.png', 'r04',
     'Corps presque entièrement mécanique ; crâne partiel, torse-machine et membres articulés.'),
    ('03', 'traqueur', 'Traqueuse', 'FIN-15', 56,
     'exec-4f22bb8d-8999-44d8-8d8a-e34db2bcef12.png', 'r03',
     'La traqueuse : compacte et athlétique ; optique, avant-bras et jambe cybernétiques.'),
    ('04', 'arcaniste', 'Arcaniste', 'FIN-16', 60,
     'exec-be185b51-f8a5-40e9-896c-5de2eee7d33c.png', 'r03',
     'Élancé ; avant-bras de canalisation, implants cuivre et céramique.'),
    ('05', 'metamorphe', 'Métamorphe', 'FIN-17', 56,
     'exec-c8ec75c8-8597-41fb-b2c2-91bd3622be86.png', 'r02',
     'La métamorphe : nerveuse, musclée et ramassée ; forme humaine au repos.'),
]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def core_box(image):
    return image.getchannel('A').point(lambda a: 255 if a >= 128 else 0).getbbox()


def font(size):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', size)


def native_image(source, height):
    """Measure solid figure, retain source alpha and edges, preserve aspect ratio."""
    original = Image.open(source).convert('RGBA')
    x0, y0, x1, y1 = core_box(original)
    pad = max(2, round((y1 - y0) * 0.015))
    frame = (max(0, x0-pad), max(0, y0-pad),
             min(original.width, x1+pad), min(original.height, y1+pad))
    cropped = original.crop(frame)
    scale = height / (y1-y0)
    size = (round(cropped.width*scale), round(cropped.height*scale))
    reduced = cropped.resize(size, Image.Resampling.LANCZOS)
    bx0, by0, bx1, by1 = core_box(reduced)
    native = Image.new('RGBA', (64, 64))
    dx = round(32 - (bx0+bx1)/2)
    dy = 62-by1
    native.alpha_composite(reduced, (dx, dy))
    b = core_box(native)
    assert b and b[0] > 0 and b[2] < 64 and b[1] >= 1 and b[3] <= 62
    return native, {'source_core_box': [x0,y0,x1,y1], 'source_crop': list(frame),
                    'isotropic_scale': scale, 'native_core_box': list(b),
                    'native_core_height': b[3]-b[1], 'native_core_width': b[2]-b[0],
                    'foot_baseline_y': 61, 'framing': '64x64, isotropic, Lanczos, source alpha retained'}


def current_image(code):
    folder = 'art-finish-01' if code == 'FIN-01' else 'art-finish-02'
    path = ROOT / f'assets/sprites/{folder}/{code}.png'
    original = Image.open(path).convert('RGBA')
    assert original.size == (128, 128)
    b = core_box(original)
    # Runtime draws the 128px composition at native scale over a 64px logical cell.
    # This fixed crop is its central logical cell, not a downsample of the actor.
    assert b[0] >= 32 and b[1] >= 32 and b[2] <= 96 and b[3] <= 96, (code, b)
    return original.crop((32,32,96,96)), path


def field(text, name):
    found = re.search(r'(?<!\w)(?:"'+name+r'"|'+name+r')\s*:\s*"([^"\n]+)"', text)
    return found.group(1) if found else None


def inventory():
    translations = {}
    for path in (ROOT/'content/core/locales').glob('fr*.json5'):
        for key, value in re.findall(r'"([^"\n]+)"\s*:\s*("(?:[^"\\]|\\.)*")', path.read_text('utf-8')):
            translations[key] = json.loads(value)
    items = []
    for path in sorted((ROOT/'content/core/items').glob('*.json5')):
        text = path.read_text('utf-8')
        slot = field(text, 'slot')
        if slot not in ('core:body_armor', 'core:head_armor'):
            continue
        key = field(text, 'name_key')
        items.append({'id':field(text,'id'), 'name':translations.get(key,key), 'slot':slot,
                      'source':path.relative_to(ROOT).as_posix(),
                      'visual_family':None, 'class_variants':'pending_after_base_review',
                      'historical_starter':path.stem.startswith('tenue_')})
    return items


def diablo_reference():
    reference = OUT/'diablo2-reference'
    inspection = json.loads((reference/'inspection.json').read_text('utf-8-sig'))
    rows = list(csv.DictReader((reference/'patch_d2-Armor.txt').read_text('cp1252').splitlines(), delimiter='\t'))
    examples = []
    for row in rows:
        if row.get('code') not in ('qui','lea','hla','ful','aar'):
            continue
        examples.append({k:row[k] for k in ('name','code','Torso','rArm','lArm','Legs','rSPad','lSPad')})
    result = {**inspection, 'selected_armor_rows':examples,
              'arm_types':list(csv.DictReader((reference/'d2data-ArmType.txt').read_text().splitlines(), delimiter='\t')),
              'observed':'Armor.txt selects body-component variants; separate BA and NE torso DCC banks exist for lit/med/hvy.',
              'limits':'Table and component-bank inspection only; DCC animations were not decoded or visually compared. No Diablo II sprites copied into the game.'}
    write_json(REVIEW/'diablo2-reference.json', result)


def native_board(current, candidates):
    compact = Image.new('RGB', (640, 168), '#242f34')
    compact_draw = ImageDraw.Draw(compact)
    compact_draw.text((20,10),'CLASS-READ01 · candidats en 64 × 64 natifs',font=font(19),fill='#eddfc4')
    for index, profile in enumerate(PROFILES):
        x=20+index*124
        compact_draw.text((x,46),f'{profile[0]} · {profile[2]}',font=font(13),fill='#eddfc4')
        compact.paste(candidates[index],(x+25,82),candidates[index])
    compact.save(OUT/'candidats-native64.png')
    board = Image.new('RGB', (1000, 400), '#111a1e')
    draw = ImageDraw.Draw(board)
    draw.text((20,12), 'CLASS-READ01 · Bases des cinq classes · candidats', font=font(21), fill='#eddfc4')
    draw.text((20,44), '64 × 64 natifs — sans arme ni armure équipée — aucun changement dans le jeu',font=font(14),fill='#b7c4c7')
    for index, profile in enumerate(PROFILES):
        number, slug, name, *_ = profile
        x = 20+index*196
        draw.text((x,80),f'{number} · {name}', font=font(16), fill='#eddfc4')
        for y, label, sprite, color in [(115,'Actuel',current[index],'#343b3f'),(245,'Proposition',candidates[index],'#343b3f')]:
            draw.text((x,y-14),label,font=font(12),fill='#b7c4c7')
            draw.rectangle((x,y+8,x+175,y+105),fill=color)
            board.paste(sprite,(x+56,y+24),sprite)
    board.save(OUT/'classes-native64.png')


def scene_board(current, candidates):
    street = Image.open(STREET).convert('RGBA')
    # A real empty road patch: no actor or object is removed from the screenshot.
    patch = street.crop((80,160,400,288))
    board = Image.new('RGB',(1000,400),'#111a1e')
    d = ImageDraw.Draw(board)
    d.text((20,12),'Même sol du jeu, même échelle · montage de revue',font=font(21),fill='#eddfc4')
    d.text((20,44),'Actuels en haut, propositions en bas — éclairage de la capture, sans nouvelle simulation moteur',font=font(14),fill='#b7c4c7')
    for index, profile in enumerate(PROFILES):
        x=20+index*196
        d.text((x,78),f'{profile[0]} · {profile[2]}',font=font(15),fill='#eddfc4')
        for y,sprite in [(108,current[index]),(248,candidates[index])]:
            crop=patch.crop((index*48,0,index*48+128,128))
            board.paste(crop,(x+20,y))
            board.paste(sprite,(x+52,y+32),sprite)
    board.save(OUT/'classes-scene-native64.png')


def female_revision_board(candidates):
    board=Image.new('RGB',(640,480),'#1c272c')
    draw=ImageDraw.Draw(board)
    draw.text((20,12),'CLASS-READ01 · La traqueuse et la métamorphe',font=font(20),fill='#eddfc4')
    draw.text((20,43),'Cases de 64 × 64 natives, puis zoom entier ×3 des mêmes pixels',font=font(13),fill='#b7c4c7')
    for x,index,old_file,title in [
        (20,2,'CL03-r02-source.png','03 · La traqueuse'),
        (340,4,'CL05-r01-source.png','05 · La métamorphe'),
    ]:
        before,_=native_image(SOURCES/old_file,56)
        draw.text((x,83),title,font=font(18),fill='#eddfc4')
        draw.text((x,114),'Essai précédent',font=font(12),fill='#b7c4c7')
        draw.text((x+150,114),'Nouvelle candidate',font=font(12),fill='#b7c4c7')
        for px,sprite in [(x+20,before),(x+170,candidates[index])]:
            draw.rectangle((px-10,138,px+74,216),fill='#343b3f')
            board.paste(sprite,(px,145),sprite)
        zoom=candidates[index].resize((192,192),Image.Resampling.NEAREST)
        draw.rectangle((x+32,243,x+250,442),fill='#343b3f')
        board.paste(zoom,(x+45,247),zoom)
    draw.text((20,456),'Silhouettes à examiner · les trois autres candidats restent identiques',font=font(12),fill='#b7c4c7')
    board.save(OUT/'femmes-native64.png')


def gallery(entries, items):
    bastion_accepted = (REVIEW/'bastion-wave-01/acceptance.json').exists()
    type_inventory_path=REVIEW/'armor-types.json'
    type_inventory=json.loads(type_inventory_path.read_text('utf-8')) if type_inventory_path.exists() else None
    retained_ids={i['id'] for i in type_inventory['items'] if i['status'] in ('generated_catalogue','accessory')} if type_inventory else {i['id'] for i in items}
    armor_section_path=OUT/'armor-types-section.html'
    armor_section=armor_section_path.read_text('utf-8') if armor_section_path.exists() else ''
    wave_section_path=OUT/'bastion-wave-01/section.html'
    wave_section=wave_section_path.read_text('utf-8') if wave_section_path.exists() else ''
    next_wave_path=OUT/'technomancien-wave-02/section.html'
    tech_head_path=OUT/'technomancien-heads-01/section.html'
    if next_wave_path.exists():
        previous_tech=next_wave_path.read_text('utf-8')
        if tech_head_path.exists():
            previous_tech='<details><summary>Anciennes propositions du Technomancien · non validées</summary>'+previous_tech+'</details>'
        wave_section=previous_tech+wave_section
    head_section_path=OUT/'bastion-heads-02/section.html'
    if head_section_path.exists():
        wave_section=head_section_path.read_text('utf-8')+wave_section
    if tech_head_path.exists():
        wave_section=tech_head_path.read_text('utf-8')+wave_section
    all_armor_path=OUT/'class-armors-all-01/section.html'
    if all_armor_path.exists():
        wave_section=all_armor_path.read_text('utf-8')+'<details><summary>Références et revues précédentes</summary>'+wave_section+'</details>'
    revised_armor_path=OUT/'class-armors-all-02/section.html'
    if revised_armor_path.exists():
        previous_sections=wave_section.replace('id="all-armors-native64"','id="all-armors-native64-r01"')
        wave_section=revised_armor_path.read_text('utf-8')+'<details><summary>Première planche et références précédentes</summary>'+previous_sections+'</details>'
    style_studies_path=OUT/'armor-style-studies-01/section.html'
    if style_studies_path.exists():
        wave_section=style_studies_path.read_text('utf-8')+'<details><summary>Anciennes armures · direction visuelle remise en revue</summary>'+wave_section+'</details>'
    revised_style_path=OUT/'armor-style-studies-02/section.html'
    if revised_style_path.exists():
        wave_section=revised_style_path.read_text('utf-8')+'<details><summary>Premières études et anciennes armures</summary>'+wave_section+'</details>'
    copper_wave_path=OUT/'bastion-copper-wave-01/section.html'
    if copper_wave_path.exists():
        wave_section=copper_wave_path.read_text('utf-8')+'<details><summary>Études de référence et anciennes armures</summary>'+wave_section+'</details>'
    machine_priest_path=OUT/'technomancien-machine-priest-01/section.html'
    no_robe_path=OUT/'technomancien-no-robe-01/section.html'
    if machine_priest_path.exists():
        previous_machine_priest=machine_priest_path.read_text('utf-8')
        if no_robe_path.exists():
            previous_machine_priest='<details><summary>Gamme précédente du Technomancien · avec robe</summary>'+previous_machine_priest+'</details>'
        wave_section=previous_machine_priest+wave_section
    if no_robe_path.exists():
        wave_section=no_robe_path.read_text('utf-8')+wave_section
    traqueuse_sniper_path=OUT/'traqueuse-sniper-wave-01/section.html'
    if traqueuse_sniper_path.exists():
        wave_section=traqueuse_sniper_path.read_text('utf-8')+wave_section
    arcaniste_channeler_path=OUT/'arcaniste-channeler-wave-01/section.html'
    if arcaniste_channeler_path.exists():
        wave_section=arcaniste_channeler_path.read_text('utf-8')+wave_section
    metamorphe_druid_path=OUT/'metamorphe-biodruid-wave-03/section.html'
    if not metamorphe_druid_path.exists():
        metamorphe_druid_path=OUT/'metamorphe-biodruid-study-02/section.html'
    if not metamorphe_druid_path.exists():
        metamorphe_druid_path=OUT/'metamorphe-druid-study-01/section.html'
    if metamorphe_druid_path.exists():
        wave_section=metamorphe_druid_path.read_text('utf-8')+wave_section
    cards=[]
    for e in entries:
        cards.append(f'''<article><h2>{e['number']} · {html.escape(e['name'])}</h2>
<p>{html.escape(e['direction'])}</p><p class="skills">{html.escape(' · '.join(s['title'] for s in e['skills']))}</p>
<div class="comparison"><figure><img src="current-{e['number']}.png" alt="Apparence actuelle"><figcaption>Actuel · taille native</figcaption></figure>
<figure><img src="{e['candidate_id']}-native64.png" alt="Candidat {e['name']}"><figcaption>Candidat · 64 × 64</figcaption></figure></div>
<div class="zoom"><img width="192" height="192" src="{e['candidate_id']}-native64.png" alt="Zoom entier ×3"></div>
<p class="meta">{e['candidate_id']} · <a href="../../{e['source']}">Dessin source</a></p></article>''')
    catalogue=''.join(f"<li>{html.escape(i['name'])}</li>" for i in items if i['id'] in retained_ids)
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>CLASS-READ01 — revue des classes</title><style>
:root{color-scheme:dark}*{box-sizing:border-box}body{margin:0;background:#10181c;color:#dfdcd2;font:16px/1.5 system-ui,sans-serif}main{max-width:1200px;padding:28px;margin:auto}h1{font-size:29px;color:#f0dec0}h2{font-size:18px;color:#e6c895;margin:0 0 12px}.intro{max-width:900px;color:#c3cdce}.status{display:inline-block;padding:6px 12px;background:#493f2c;color:#f4db9c;border-radius:5px}a{color:#b8d3dd}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:18px;margin:28px 0}article{background:#1c272c;border:1px solid #334349;border-radius:8px;padding:18px}article p{min-height:48px}.skills{font-size:13px;color:#b4c6c8}.comparison{display:flex;gap:16px}.comparison figure{margin:0;flex:1;background:#393e3f;padding:18px 8px;text-align:center}.comparison img{width:64px;height:64px;image-rendering:pixelated}figcaption{font-size:11px;color:#bdc8c6;margin-top:12px}.zoom{background:#343b3f;margin-top:14px;text-align:center}.zoom img{display:inline-block;image-rendering:pixelated}.meta{font-size:12px;min-height:0}section{margin-top:30px}.board{max-width:100%;height:auto;border:1px solid #334349}button,select{font:inherit;padding:5px 10px;background:#293c45;color:#eee;border:1px solid #60727b;border-radius:4px}.note{color:#b9c4c5;font-size:14px}ul{columns:2;padding-left:22px}@media(max-width:650px){ul{columns:1}}
.native-scroll{max-width:100%;overflow-x:auto}.native-scroll .board{display:block;max-width:none;box-sizing:content-box}.native-scroll canvas{max-width:none!important}
</style><main><span class="status">Propositions à examiner · aucun dessin intégré</span>
<h1>Cinq classes, cinq corps</h1><p class="intro">Première revue des silhouettes humaines sans armure équipée. Les augmentations visibles font partie du corps proposé : elles ne donnent aucun nouveau bonus. Les modèles d’armes restent communs aux classes.</p>
<p class="intro">Les armures de torse et de tête seront déclinées après le choix des bases. Cette revue ne valide ni toutes les armures, ni les prises d’armes, ni les transformations.</p>
'''+wave_section+armor_section+'''
<section><h2>Révision · la traqueuse et la métamorphe</h2><p class="note">Les classes 03 et 05 sont désormais proposées comme des femmes. Les noms féminins sont retenus ; leurs identifiants et compétences sont conservés.</p><div class="native-scroll"><img class="board" src="femmes-native64.png" alt="La traqueuse et la métamorphe : candidates natives et zoom entier"></div></section>
<section><h2>Candidats à taille native</h2><p class="note">Chaque sprite occupe une case de 64 × 64. Les planches gardent leur taille réelle ; faites défiler horizontalement si nécessaire.</p><div class="native-scroll"><img class="board" src="candidats-native64.png" alt="Cinq candidats en 64 × 64 natifs"></div></section>
<section><h2>Comparaison avec les apparences actuelles</h2><div class="native-scroll"><img class="board" src="classes-native64.png" alt="Comparaison des cinq classes à taille native"></div></section>
<div class="grid">'''+''.join(cards)+'''</div><section><h2>Lecture sur un sol réel</h2><p class="note">Montage sur une zone vide d’une capture de la rue acceptée. Ce n’est pas une nouvelle capture moteur ; le nouvel éclairage des acteurs reste à valider dans le moteur.</p><div class="native-scroll"><img class="board" src="classes-scene-native64.png" alt="Comparaison sur le sol de la rue"></div></section>
<section><h2>Tester le fond à taille native</h2><label>Fond <select id="ground"><option value="#343b3f">Asphalte sombre</option><option value="#858478">Sol clair</option><option value="#12181d">Pénombre</option></select></label><p class="note">Les dessins restent inchangés ; ce contrôle vérifie uniquement leur contraste.</p><div class="native-scroll"><canvas id="lineup" width="640" height="96" style="image-rendering:pixelated"></canvas></div></section>
<section><h2>Torses et têtes retenus pour la suite</h2><p>Modèles du catalogue retenu. Les anciennes tenues de départ et les modèles retirés, dont le Tablier de cuir, sont exclus des prochaines déclinaisons.</p><ul>'''+catalogue+'''</ul><p class="note">Les monstres feront l’objet d’une revue de lisibilité distincte. Aucun affichage modulaire de leur équipement n’est prévu.</p></section>
<p class="note"><a href="../../docs/reviews/class-readability-01/REVIEW.md">Dossier et périmètre de revue</a> · <a href="../../docs/reviews/class-readability-01/prompts.json">Prompts exacts</a></p>
</main><script>const paths='''+json.dumps([e['candidate_id']+'-native64.png' for e in entries])+''';const images=paths.map(p=>{const i=new Image;i.src=p;return i});const c=document.getElementById('lineup'),ctx=c.getContext('2d');function draw(){ctx.fillStyle=document.getElementById('ground').value;ctx.fillRect(0,0,c.width,c.height);ctx.imageSmoothingEnabled=false;images.forEach((im,i)=>{if(im.complete)ctx.drawImage(im,32+i*128,16,64,64)})}images.forEach(im=>im.onload=draw);document.getElementById('ground').onchange=draw;draw();</script></html>'''
    if bastion_accepted:
        page = page.replace('Propositions à examiner · aucun dessin intégré',
                            'Bastion validé · autres propositions en revue · aucun dessin intégré')
    if head_section_path.exists():
        page = page.replace('Bastion validé · autres propositions en revue · aucun dessin intégré',
                            'Bastion : corps validés · nouvelles têtes à examiner · aucun dessin intégré')
    if (REVIEW/'bastion-heads-02/acceptance.json').exists():
        page = page.replace('Bastion : corps validés · nouvelles têtes à examiner · aucun dessin intégré',
                            'Bastion : corps et têtes validés · autres classes en revue · aucun dessin intégré')
    if style_studies_path.exists():
        page = page.replace('Bastion : corps et têtes validés · autres classes en revue · aucun dessin intégré',
                            'Direction visuelle à revoir · deux études du Bastion · aucun dessin intégré')
    if revised_style_path.exists():
        page = page.replace('Direction visuelle à revoir · deux études du Bastion · aucun dessin intégré',
                            'Bastion · visages repris et cuivre simplifié · aucun dessin intégré')
    if copper_wave_path.exists():
        page = page.replace('Bastion · visages repris et cuivre simplifié · aucun dessin intégré',
                            'Bastion · cinq degrés de protection cuivre à examiner · aucun dessin intégré')
    if (REVIEW/'bastion-copper-wave-01/acceptance.json').exists():
        page = page.replace('Bastion · cinq degrés de protection cuivre à examiner · aucun dessin intégré',
                            'Bastion · cinq armures et têtes par défaut validées · intégration différée')
    if machine_priest_path.exists():
        page = page.replace('Bastion · cinq armures et têtes par défaut validées · intégration différée',
                            'Bastion validé · Technomancien à examiner · intégration différée')
    if no_robe_path.exists():
        page = page.replace('Bastion validé · Technomancien à examiner · intégration différée',
                            'Bastion validé · Technomancien : essai sans robe · intégration différée')
    if (REVIEW/'technomancien-no-robe-heads-02/acceptance.json').exists():
        page = page.replace('Bastion validé · Technomancien : essai sans robe · intégration différée',
                            'Bastion et Technomancien validés · autres classes en revue · intégration différée')
    if traqueuse_sniper_path.exists():
        page = page.replace('Bastion et Technomancien validés · autres classes en revue · intégration différée',
                            'Bastion et Technomancien validés · Traqueuse à examiner · intégration différée')
    if (REVIEW/'traqueuse-sniper-wave-01/acceptance.json').exists():
        page = page.replace('Bastion et Technomancien validés · Traqueuse à examiner · intégration différée',
                            'Bastion, Technomancien et Traqueuse validés · intégration différée')
    if arcaniste_channeler_path.exists():
        page = page.replace('Bastion, Technomancien et Traqueuse validés · intégration différée',
                            'Trois classes validées · Arcaniste à examiner · intégration différée')
    if (REVIEW/'arcaniste-red-robe-wave-05/acceptance.json').exists():
        page = page.replace('Trois classes validées · Arcaniste à examiner · intégration différée',
                            'Quatre classes validées · Métamorphe à préparer · intégration différée')
    if metamorphe_druid_path.exists():
        page = page.replace('Quatre classes validées · Métamorphe à préparer · intégration différée',
                            'Quatre classes validées · Métamorphe à examiner · intégration différée')
    (OUT/'index.html').write_text(page,encoding='utf-8')


def main():
    for folder in (OUT,REVIEW,SOURCES):
        folder.mkdir(parents=True,exist_ok=True)
    if (REVIEW/'acceptance.json').exists():
        raise SystemExit('Accepted review is frozen; create a new revision.')
    catalogue=json.loads((ROOT/'assets/skills/agent/catalogue.json').read_text('utf-8'))
    skill_by_id={s['id']:s for s in catalogue['skills']}
    current=[]; proposed=[]; entries=[]; unchanged={}
    for number,slug,name,code,height,generated,revision,direction in PROFILES:
        candidate=f'CL{number}-{revision}'
        source=SOURCES/(candidate+'-source.png')
        shutil.copyfile(GENERATED/generated,source)
        native,measurements=native_image(source,height)
        native.save(OUT/(candidate+'-native64.png'))
        before,path=current_image(code)
        unchanged[path.relative_to(ROOT).as_posix()]=sha(path)
        before.save(OUT/f'current-{number}.png')
        current.append(before); proposed.append(native)
        definition=ROOT/f'content/core/classes/{slug}.json5'
        data=json.loads(definition.read_text('utf-8'))
        skills=[{'id':i,'title':skill_by_id[i]['title'],'description':skill_by_id[i]['roleplay']} for i in data['agent_start']['skills']]
        entries.append({'number':number,'candidate_id':candidate,'class_id':data['id'],'name':name,
                        'status':'awaiting_visual_approval','direction':direction,'skills':skills,
                        'presentation_gender':'female' if number in ('03','05') else 'unspecified',
                        'class_source':definition.relative_to(ROOT).as_posix(),
                        'class_source_sha256':sha(definition),'current_art':path.relative_to(ROOT).as_posix(),
                        'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
                        'native':f'artifacts/class-armor-review-01/{candidate}-native64.png',
                        'native_sha256':sha(OUT/(candidate+'-native64.png')),
                        'measurements':measurements,'layer_status':'silhouette_reference_only_not_a_production_rig',
                        'attachment_points':'foot_baseline_measured; head/shoulders/hands_pending_after_silhouette_review'})
    items=inventory()
    if (REVIEW/'armor-types.json').exists():
        type_items={i['id']:i for i in json.loads((REVIEW/'armor-types.json').read_text('utf-8'))['items']}
        for item in items:
            status=type_items[item['id']]['status']
            item['catalogue_status']=status
            item['planned_for_class_variants']=status in ('generated_catalogue','accessory')
    native_board(current,proposed)
    scene_board(current,proposed)
    female_revision_board(proposed)
    gallery(entries,items)
    diablo_reference()
    data={'review_id':'CLASS-READ01','date':'2026-10-21','status':'awaiting_visual_approval',
          'scope':'Five player base silhouettes and a separate Bastion full-body armor wave; no runtime integration; monsters excluded from equipment system.',
          'latest_steering':'One class per wave. Torso armor defines the whole outfit except the fixed head. Bastion goes from very light to very heavy melee armor. A simplified heavy pilot and red, blue, green plate palettes await review.',
          'tile_size':64,'active_game_resources_modified':False,'entries':entries,'equipment_inventory':items,
          'approval_scopes':{'base_silhouettes':'pending','body_armor_variants':'Bastion wave 01 draft, awaiting visual review','head_armor_variants':'not_produced',
                             'weapon_grips':'not_produced','new_effects':'none','monster_readability':'later_review'}}
    if (REVIEW/'bastion-wave-01/acceptance.json').exists():
        data['approval_scopes']['body_armor_variants']='Bastion wave 01 accepted: five models and fifteen color variants; other classes pending'
        data['latest_steering']='Bastion wave 01 accepted at native64. Preserve the exact twenty views; future drawing revisions require their own review.'
    if (REVIEW/'technomancien-wave-02/candidates.json').exists():
        data['approval_scopes']['body_armor_variants']='Bastion wave 01 accepted; Technomancien wave 02 awaiting visual review; remaining classes not started'
        data['latest_steering']='One class per wave; Technomancien wave 02 follows the accepted Bastion. Integration waits for all armors, helmets and weapon hand positions.'
    if (REVIEW/'technomancien-wave-02/rejection.json').exists():
        data['approval_scopes']['body_armor_variants']='Bastion wave 01 accepted; first Technomancien series rejected; identity revision requires visual review'
        data['latest_steering']='Five distinct class identities readable at native64. The equipped torso item supplies the outfit color. Review a single revised Technomancien identity before another range.'
    if (REVIEW/'technomancien-wave-02/design-candidates.json').exists():
        data['approval_scopes']['body_armor_variants']='Bastion accepted; first Technomancien series rejected; five new coat-based designs awaiting native review; color variants deferred'
        data['latest_steering']='Focus on beautiful distinct class armor designs in native64. No new color variants. Integration waits for armor designs, helmets and weapon hand positions.'
        data['color_variants']='deferred_by_user'
    if (REVIEW/'bastion-heads-02/candidates.json').exists():
        data['approval_scopes']['default_head_appearances']='Bastion five default headgear proposals awaiting review; other classes pending'
        data['latest_steering']='Torso armor supplies default head appearance; an equipped helmet overrides only that part. Bastion bodies remain accepted; five new headgear overlays await native64 review. Color variants deferred.'
    if (REVIEW/'bastion-heads-02/acceptance.json').exists():
        data['approval_scopes']['default_head_appearances']='Bastion five default head appearances accepted; other classes pending'
        data['latest_steering']='Bastion bodies and five default heads accepted at native64. Other class designs, separate helmets and weapon grips still require review. Color variants deferred; no game integration yet.'
    if (REVIEW/'technomancien-heads-01/identity-candidate.json').exists():
        data['approval_scopes']['default_head_appearances']='Bastion accepted; first Technomancien head progression rejected; one draped cowl-and-collar identity pilot awaiting review'
        data['latest_steering']='Change Technomancien geometry, not only colors. Review one asymmetrical soft cowl and collar pilot before another range. Preserve Bastion acceptance; no new colors or runtime integration.'
    if (REVIEW/'technomancien-heads-01/acceptance.json').exists():
        data['approval_scopes']['default_head_appearances']='Bastion accepted; Technomancien cowl identity pilot accepted; additional variations require review'
        data['latest_steering']='Preserve the accepted Technomancien draped cowl pilot and Bastion references; future variations require review.'
    if (REVIEW/'class-armors-all-01/candidates.json').exists():
        data['approval_scopes']['body_armor_variants']='Joint 25-composition review prepared: six exact accepted references and nineteen pending compositions'
        data['approval_scopes']['default_head_appearances']='Bastion and Technomancien pilot accepted; Traqueuse, Arcaniste and Métamorphe default headgear pending review'
        data['latest_steering']='Complete all five class armor ranges before a joint native64 review. Keep accepted Bastion and Technomancien identity pilot exact; nineteen other compositions require selection or revision. No color variants or runtime integration.'
        data['review_workflow']='all_classes_then_joint_native64_review'
    if (REVIEW/'class-armors-all-02/candidates.json').exists():
        data['approval_scopes']['body_armor_variants']='Bastion preserved; revised machine-priest and sniper/ninja outfits, corrected arcanist and metamorph neck framing pending joint review'
        data['approval_scopes']['default_head_appearances']='Bastion accepted; fifteen new Technomancien, Traqueuse and Arcaniste head protections pending review'
        data['latest_steering']='Revise unpopular Technomancien and Traqueuse, compact Arcaniste helmets and fix long neck joins. Technomancien choice: dark machine priest with a mechanical face and sober coat. Keep numbers 01–25; no integration.'
    write_json(REVIEW/'candidates.json',data)
    assert len(entries)==5 and len({e['class_id'] for e in entries})==5
    assert len({e['native_sha256'] for e in entries})==5
    assert all(sha(ROOT/path)==digest for path,digest in unchanged.items())
    checks={'five_distinct_native_candidates':True,'native_dimensions':[64,64],
            'original_alpha_preserved':True,'framing_preserves_aspect_ratio':True,
            'existing_class_sprites_unchanged':unchanged,'all_skills_read_from_live_catalogue':True,
            'game_code_or_rules_modified_by_this_review':False,'game_not_launched':True,
            'scene_is_a_review_montage_not_engine_validation':True,
            'remaining':'Visual acceptance, final layers/anchors, armor variants, weapon grip validation, in-engine lighting and readability.'}
    write_json(REVIEW/'checks.json',checks)
    print(json.dumps({'review':'CLASS-READ01','candidates':len(entries),'body_definitions':sum(i['slot']=='core:body_armor' for i in items),
                      'head_definitions':sum(i['slot']=='core:head_armor' for i in items),
                      'native_core_sizes':{e['name']:[e['measurements']['native_core_width'],e['measurements']['native_core_height']] for e in entries}},ensure_ascii=False))


if __name__=='__main__':
    main()
