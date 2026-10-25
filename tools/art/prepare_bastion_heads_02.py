"""Compose new headgear over immutable accepted Bastion armor bodies."""
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, OUT, REVIEW, SOURCES, STREET, core_box, font, sha, write_json, gallery
from prepare_bastion_progression_01 import pixels_hash

WAVE = 'bastion-heads-02'
ART, DEST, DOCS = SOURCES/WAVE, OUT/WAVE, REVIEW/WAVE
SEAM = 12
LABELS = ['Très légère','Légère','Interméd.','Lourde','Très lourde']


def frame_overlay(path, spec):
    source = Image.open(path).convert('RGBA')
    x0,y0,x1,y1 = core_box(source)
    pad = max(2, round((y1-y0)*0.02))
    crop = (max(0,x0-pad),max(0,y0-pad),min(source.width,x1+pad),min(source.height,y1+pad))
    scale = min(spec['target_height']/(y1-y0),spec['max_width']/(x1-x0))
    frame = source.crop(crop)
    reduced = frame.resize((round(frame.width*scale),round(frame.height*scale)),Image.Resampling.LANCZOS)
    bx0,by0,bx1,by1 = core_box(reduced)
    dx = round(32-(bx0+bx1)/2)
    dy = spec['bottom_exclusive']-by1
    unclipped = Image.new('RGBA',(64,64))
    unclipped.alpha_composite(reduced,(dx,dy))
    overlay = Image.new('RGBA',(64,64))
    overlay.paste(unclipped.crop((0,0,64,SEAM)),(0,0))
    assert overlay.crop((0,SEAM,64,64)).getbbox() is None
    bounds = core_box(overlay)
    assert bounds and bounds[0]>0 and bounds[2]<64 and bounds[1]>=1 and bounds[3]<=SEAM
    return overlay,{'source_core_box':[x0,y0,x1,y1],'source_crop':list(crop),
        'isotropic_scale':scale,'native_core_box':list(bounds),'placement':[dx,dy],
        'head_only_region':[0,0,64,SEAM],'fit_preserves_source_aspect_ratio':True}


def assemble(original, default_overlay, helmet_override=None):
    sprite = original.copy()
    sprite.alpha_composite(default_overlay if helmet_override is None else helmet_override)
    assert pixels_hash(sprite.crop((0,SEAM,64,64)))==pixels_hash(original.crop((0,SEAM,64,64)))
    return sprite


def boards(originals, sprites, override):
    board = Image.new('RGB',(400,348),'#152025')
    draw = ImageDraw.Draw(board)
    draw.text((8,9),'Bastion · têtes assorties · 64 × 64 natifs',font=font(14),fill='#efddbd')
    for i,label in enumerate(LABELS):
        draw.text((i*80+4,36),f'{i+1:02d} · {label}',font=font(10),fill='#d5dfdc')
    for row,(label,color) in enumerate([('Sol sombre','#343b3f'),('Sol clair','#858478'),('Pénombre','#12181d')]):
        y = 72+row*90
        draw.rectangle((4,y-3,396,y+66),fill=color)
        for i,sprite in enumerate(sprites):
            board.paste(sprite,(8+i*80,y),sprite)
        draw.text((8,y+68),label,font=font(11),fill='#b5c5c3')
    board.save(DEST/'heads-native64.png')
    board = Image.new('RGB',(400,252),'#343b3f')
    draw = ImageDraw.Draw(board)
    draw.text((8,8),'Corps validés · avant et après · taille native',font=font(14),fill='#efddbd')
    for i in range(5):
        draw.text((i*80+32,32),f'{i+1:02d}',font=font(12),fill='#d5dfdc')
        board.paste(originals[i],(i*80+8,54),originals[i])
        board.paste(sprites[i],(i*80+8,162),sprites[i])
    draw.text((8,122),'Avant · tête commune validée',font=font(11),fill='#d5dfdc')
    draw.text((8,230),'Après · tête assortie proposée',font=font(11),fill='#d5dfdc')
    board.save(DEST/'before-after-native64.png')
    board = Image.new('RGB',(1040,263),'#243036')
    draw = ImageDraw.Draw(board)
    draw.text((10,8),'Cinq protections de tête · zoom entier ×3',font=font(17),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        zoom = sprite.resize((192,192),Image.Resampling.NEAREST)
        board.paste(zoom,(8+i*208,38),zoom)
        draw.text((12+i*208,234),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d5dfdc')
    board.save(DEST/'heads-zoom-x3.png')
    board = Image.new('RGB',(640,158),'#343b3f')
    draw = ImageDraw.Draw(board)
    draw.text((8,8),'Détails de tête ×6 · juger aussi la planche native',font=font(14),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        detail = sprite.crop((22,0,42,16)).resize((120,96),Image.Resampling.NEAREST)
        board.paste(detail,(4+i*128,38),detail)
        draw.text((i*128+54,136),f'{i+1:02d}',font=font(12),fill='#d5dfdc')
    board.save(DEST/'head-details-x6.png')
    board = Image.new('RGB',(384,158),'#343b3f')
    draw = ImageDraw.Draw(board)
    draw.text((8,8),'Exemple de priorité du casque · taille native',font=font(14),fill='#efddbd')
    for i,(sprite,label) in enumerate(zip([sprites[0],override,sprites[0]],['Tête du torse 01','Protection 05 à la place','Après retrait'])):
        draw.text((4+i*128,38),label,font=font(10),fill='#d5dfdc')
        board.paste(sprite,(32+i*128,66),sprite)
    board.save(DEST/'helmet-priority-native64.png')
    street = Image.open(STREET).convert('RGBA')
    patch = street.crop((80,160,400,288))
    board = Image.new('RGB',(400,186),'#152025')
    draw = ImageDraw.Draw(board)
    draw.text((9,8),'Même sol · montage de revue à taille native',font=font(14),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        x = i*80
        board.paste(patch.crop((40+i*30,16,120+i*30,144)),(x,36))
        board.paste(sprite,(x+8,68),sprite)
        draw.text((x+32,167),f'{i+1:02d}',font=font(12),fill='#d5dfdc')
    board.save(DEST/'heads-scene-native64.png')


def main():
    if (DOCS/'acceptance.json').exists():
        raise SystemExit('Accepted heads are frozen; prepare a new revision.')
    for folder in (ART,DEST,DOCS):
        folder.mkdir(parents=True,exist_ok=True)
    plan = json.loads((DOCS/'prompts.json').read_text('utf-8'))
    acceptance = json.loads((ROOT/plan['source_body_acceptance']).read_text('utf-8'))
    originals_by_id = {e['candidate_id']:e for e in acceptance['entries'] if e['color']=='original'}
    protected = {plan['source_body_acceptance']:sha(ROOT/plan['source_body_acceptance'])}
    for item in acceptance['entries']:
        for key in ('native','body_layer'):
            protected[item[key]['path']] = item[key]['sha256']
        protected[item['source']] = item['source_sha256']
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    sprites,originals,overlays,entries = [],[],[],[]
    for item in plan['entries']:
        source = ART/(item['candidate_id']+'-overlay-source.png')
        generated = Path(item['generated_file'])
        if generated.exists():
            shutil.copyfile(generated,source)
        else:
            assert source.exists()
        original_entry = originals_by_id[item['body_candidate']]
        original = Image.open(ROOT/original_entry['native']['path']).convert('RGBA')
        overlay,measure = frame_overlay(source,item)
        sprite = assemble(original,overlay)
        native,layer = DEST/(item['candidate_id']+'-assembled-native64.png'),DEST/(item['candidate_id']+'-overlay-native64.png')
        sprite.save(native)
        overlay.save(layer)
        visible_head = sum(original.getpixel((x,y))[3]>=128 and overlay.getpixel((x,y))[3]==0 for y in range(SEAM) for x in range(64))
        entries.append({'number':item['number'],'candidate_id':item['candidate_id'],'label':item['label'],
            'body_candidate':item['body_candidate'],'status':'awaiting_visual_review',
            'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
            'headgear_layer':layer.relative_to(ROOT).as_posix(),'headgear_layer_sha256':sha(layer),
            'approved_body_native':original_entry['native']['path'],
            'body_region':[0,SEAM,64,64],'body_region_rgba_sha256':pixels_hash(sprite.crop((0,SEAM,64,64))),
            'measurements':measure,'unoccluded_original_head_pixels':visible_head,
            'head_occlusion_mask':'alpha channel of the explicit headgear overlay; base face is never redrawn'})
        originals.append(original);sprites.append(sprite);overlays.append(overlay)
    assert len(entries)==5
    override = assemble(originals[0],overlays[0],overlays[4])
    restored = assemble(originals[0],overlays[0])
    assert pixels_hash(restored)==pixels_hash(sprites[0])
    boards(originals,sprites,override)
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    write_json(DOCS/'candidates.json',{'review_id':'BASTION-W01-HEAD02','date':'2026-10-21',
        'status':'awaiting_visual_review','scope':'Five default head appearances belonging to the torso armor models, not five new loot helmet items.',
        'rule':plan['rule'],'native_dimensions':[64,64],'entries':entries,
        'color_variants_generated':0,'runtime_integration':False,
        'checks':{'accepted_body_pixels_preserved_below_y':SEAM,
            'original_base_face_preserved_outside_explicit_overlay_occlusion':True,
            'helmet_override_uses_override_instead_of_default_overlay':True,
            'removing_override_restores_default_exact_rgba':True,
            'all_accepted_bastion_files_preserved':protected,
            'scene_is_montage_not_engine_validation':True},
        'override_demo':'Protection 05 reused purely to illustrate the visual priority rule; no independent helmet item or gameplay rule is created.'})
    (DEST/'section.html').write_text('''<section id="bastion-heads-02"><h2>Bastion · cinq têtes assorties à examiner</h2>
<p>Chaque armure de torse fournit maintenant sa protection de tête par défaut. Du couvre-chef léger au casque fermé, la protection suit la tenue. Les corps validés sont conservés au pixel près.</p>
<p class="note">Ces cinq nouvelles compositions attendent leur validation. Les dessins de tête ne sont pas couverts par l’accord antérieur sur les corps.</p>
<div class="native-scroll"><img class="board" width="400" height="348" src="bastion-heads-02/heads-native64.png" alt="Les cinq armures du Bastion avec leurs nouvelles protections de tête, numérotées et présentées en 64 × 64 sur trois fonds"></div>
<details><summary>Comparer avant et après à taille native</summary><div class="native-scroll"><img class="board" width="400" height="252" src="bastion-heads-02/before-after-native64.png" alt="Avant, tête commune acceptée ; après, nouvelle protection de tête, corps strictement identique"></div></details>
<details><summary>Zoom entier ×3 des compositions</summary><div class="native-scroll"><img class="board" width="1040" height="263" src="bastion-heads-02/heads-zoom-x3.png" alt="Les cinq propositions de tête avec les corps du Bastion en zoom entier ×3"></div></details>
<details><summary>Détails des têtes ×6</summary><div class="native-scroll"><img class="board" width="640" height="158" src="bastion-heads-02/head-details-x6.png" alt="Les cinq nouvelles protections de tête en gros plan entier ×6"></div></details>
<div class="native-scroll"><img class="board" width="400" height="186" src="bastion-heads-02/heads-scene-native64.png" alt="Les cinq nouvelles compositions du Bastion sur le sol de la rue, montage à taille native"></div>
<p class="note">Montage sur une capture acceptée ; l’éclairage moteur reste à vérifier. Le visage de base est conservé sous les protections, qui le masquent selon leur couverture.</p>
<details><summary>Illustrer la priorité d’un casque équipé</summary><p class="note">La protection 05 sert seulement d’exemple : elle remplace visuellement celle du torse 01, puis son retrait rétablit la tête du torse. Aucun objet de casque n’est créé par cette démonstration.</p><div class="native-scroll"><img class="board" width="384" height="158" src="bastion-heads-02/helmet-priority-native64.png" alt="Tête du torse 01, protection 05 prioritaire, puis retour à la tête du torse après retrait"></div></details>
<p>Pas de nouvelle variante de couleur. L’intégration attend les armures complètes, les casques et les bonnes prises d’armes.</p>
<p><a href="../../docs/reviews/class-readability-01/bastion-heads-02/REVIEW.md">Périmètre de la revue</a> · <a href="../../docs/reviews/class-readability-01/bastion-heads-02/prompts.json">Prompts image_gen</a> · <a href="../../docs/reviews/class-readability-01/bastion-heads-02/candidates.json">Sources et contrôles</a></p></section>''','utf-8')
    root_path = REVIEW/'candidates.json'
    root = json.loads(root_path.read_text('utf-8'))
    root['approval_scopes']['default_head_appearances']='Bastion five default headgear proposals awaiting review; other classes pending'
    root['latest_steering']='Torso armor supplies default head appearance; an equipped helmet overrides only that part. Bastion bodies remain accepted; five new headgear overlays await native64 review. Color variants deferred.'
    write_json(root_path,root)
    policy_path = ROOT/plan['rule']
    policy = json.loads(policy_path.read_text('utf-8'))
    policy['bastion_default_head_appearances']='five_proposals_prepared_awaiting_native_visual_review'
    policy['bastion_head_review']='docs/reviews/class-readability-01/bastion-heads-02/candidates.json'
    write_json(policy_path,policy)
    gallery(root['entries'],root['equipment_inventory'])
    print(json.dumps({'new_headgear':5,'body_regions_identical':True,'accepted_art_unchanged':True,
        'original_head_pixels_unoccluded':[e['unoccluded_original_head_pixels'] for e in entries],
        'native_overlay_bounds':[e['measurements']['native_core_box'] for e in entries]}))


if __name__=='__main__':
    main()
