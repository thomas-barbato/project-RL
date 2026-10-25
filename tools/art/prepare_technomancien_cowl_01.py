"""Review one draped cowl identity pilot, preserving earlier drawings."""
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, SOURCES, OUT, REVIEW, STREET, core_box, font, sha, write_json, gallery
from prepare_bastion_progression_01 import pixels_hash

WAVE = 'technomancien-heads-01'
ART, DEST, DOCS = SOURCES/WAVE, OUT/WAVE, REVIEW/WAVE
COLLAR_END = 27


def frame_module(source_path, spec):
    source = Image.open(source_path).convert('RGBA')
    x0, y0, x1, y1 = core_box(source)
    pad = max(2, round((y1-y0)*0.02))
    crop = (max(0,x0-pad), max(0,y0-pad), min(source.width,x1+pad), min(source.height,y1+pad))
    scale = min(spec['target_height']/(y1-y0), spec['max_width']/(x1-x0))
    cropped = source.crop(crop)
    reduced = cropped.resize((round(cropped.width*scale),round(cropped.height*scale)),Image.Resampling.LANCZOS)
    bx0, by0, bx1, by1 = core_box(reduced)
    dx = round(spec.get('center_x',32)-(bx0+bx1)/2)
    dy = spec['bottom_exclusive']-by1
    framed = Image.new('RGBA',(64,64))
    framed.alpha_composite(reduced,(dx,dy))
    module = Image.new('RGBA',(64,64))
    module.paste(framed.crop((0,0,64,COLLAR_END)),(0,0))
    bounds = core_box(module)
    assert bounds[0]>0 and bounds[2]<64 and bounds[1]>=1 and bounds[3]<=COLLAR_END
    return module, {'source_core_box':[x0,y0,x1,y1], 'source_crop':list(crop),
        'isotropic_scale':scale, 'native_core_box':list(bounds), 'placement':[dx,dy],
        'layer_ownership':[0,0,64,COLLAR_END], 'source_aspect_ratio_preserved':True}


def boards(base, pilot, bastion):
    board = Image.new('RGB',(384,358),'#152025')
    d = ImageDraw.Draw(board)
    d.text((8,8),'Technomancien · un témoin de forme · taille native',font=font(13),fill='#efddbd')
    for i,label in enumerate(['Bastion accepté','Ancien témoin','01 · Nouvelle coiffe']):
        d.text((4+i*128,36),label,font=font(11),fill='#d5dfdc')
    for row,(label,color) in enumerate([('Sol sombre','#343b3f'),('Sol clair','#858478'),('Pénombre','#12181d')]):
        y = 72+row*92
        d.rectangle((4,y-3,380,y+66),fill=color)
        for i,sprite in enumerate([bastion,base,pilot]):
            board.paste(sprite,(32+i*128,y),sprite)
        d.text((8,y+68),label,font=font(11),fill='#b5c5c3')
    board.save(DEST/'identity-native64.png')
    board = Image.new('RGB',(624,263),'#243036')
    d = ImageDraw.Draw(board)
    d.text((8,8),'Comparer les formes · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,(sprite,label) in enumerate(zip([bastion,base,pilot],['Bastion accepté','Ancien témoin','01 · Capuche et col drapés'])):
        z = sprite.resize((192,192),Image.Resampling.NEAREST)
        board.paste(z,(8+i*208,38),z)
        d.text((8+i*208,235),label,font=font(12),fill='#d5dfdc')
    board.save(DEST/'identity-zoom-x3.png')
    board = Image.new('RGB',(400,210),'#343b3f')
    d = ImageDraw.Draw(board)
    d.text((8,8),'Tête et col ×6 · vérifier aussi à taille native',font=font(14),fill='#efddbd')
    for i,(sprite,label) in enumerate([(base,'Avant'),(pilot,'01 · Proposition')]):
        detail = sprite.crop((18,0,46,27)).resize((168,162),Image.Resampling.NEAREST)
        board.paste(detail,(8+i*200,30),detail)
        d.text((8+i*200,191),label,font=font(12),fill='#d5dfdc')
    board.save(DEST/'identity-head-collar-x6.png')
    street = Image.open(STREET).convert('RGBA')
    board = Image.new('RGB',(384,186),'#152025')
    d = ImageDraw.Draw(board)
    d.text((8,8),'Même sol · montage à taille native',font=font(14),fill='#efddbd')
    patch = street.crop((80,160,464,288))
    board.paste(patch,(0,36))
    for i,(sprite,label) in enumerate(zip([bastion,base,pilot],['Bastion accepté','Ancien témoin','01 · Proposition'])):
        board.paste(sprite,(32+i*128,68),sprite)
        d.text((4+i*128,166),label,font=font(11),fill='#d5dfdc')
    board.save(DEST/'identity-scene-native64.png')


def main():
    if (DOCS/'acceptance.json').exists():
        raise SystemExit('Accepted pilot is frozen; create a new revision.')
    for folder in (ART,DEST,DOCS):
        folder.mkdir(parents=True,exist_ok=True)
    plan = json.loads((DOCS/'identity-prompt.json').read_text('utf-8'))
    body_review_path = REVIEW/'technomancien-wave-02/design-candidates.json'
    body_review = json.loads(body_review_path.read_text('utf-8'))
    base_item = next(e for e in body_review['entries'] if e['candidate_id']==plan['base_body_candidate'])
    protected = dict(body_review['checks']['accepted_bastion_and_rejected_tech_sources_preserved'])
    protected[body_review_path.relative_to(ROOT).as_posix()] = sha(body_review_path)
    for e in body_review['entries']:
        for key in ('source','native','body_layer'):
            protected[e[key]] = e[key+'_sha256']
    accepted = json.loads((REVIEW/'bastion-heads-02/acceptance.json').read_text('utf-8'))
    for e in accepted['entries']:
        for key in ('native','headgear_layer'):
            protected[e[key]['path']] = e[key]['sha256']
        protected[e['source']] = e['source_sha256']
    for e in accepted['review_references']+accepted['provenance']:
        protected[e['path']] = e['sha256']
    for p in [REVIEW/'bastion-wave-01/acceptance.json',REVIEW/'bastion-heads-02/acceptance.json']:
        protected[p.relative_to(ROOT).as_posix()] = sha(p)
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    cid = plan['candidate_id']
    source = ART/(cid+'-cowl-source.png')
    generated = Path(plan['generated_file'])
    if generated.exists():
        shutil.copyfile(generated,source)
    else:
        assert source.exists()
    module, framing = frame_module(source,plan['framing'])
    base = Image.open(ROOT/base_item['native']).convert('RGBA')
    pilot = base.copy()
    pilot.alpha_composite(module)
    assert pilot.size==(64,64)
    assert pilot.crop((0,COLLAR_END,64,64)).tobytes()==base.crop((0,COLLAR_END,64,64)).tobytes()
    native, layer = DEST/(cid+'-native64.png'), DEST/(cid+'-cowl-native64.png')
    pilot.save(native); module.save(layer)
    default_head = Image.new('RGBA',(64,64))
    default_head.paste(pilot.crop((0,0,64,14)),(0,0))
    body_with_collar = Image.new('RGBA',(64,64))
    body_with_collar.paste(pilot.crop((0,14,64,64)),(0,14))
    head_path, body_path = DEST/(cid+'-default-head-native64.png'), DEST/(cid+'-body-collar-native64.png')
    default_head.save(head_path); body_with_collar.save(body_path)
    recomposed = body_with_collar.copy()
    recomposed.alpha_composite(default_head)
    assert recomposed.tobytes()==pilot.tobytes()
    bastion_entry = accepted['entries'][2]
    bastion = Image.open(ROOT/bastion_entry['native']['path']).convert('RGBA')
    boards(base,pilot,bastion)
    write_json(DOCS/'identity-candidate.json',{'review_id':plan['review_id'],'date':'2026-10-21',
        'candidate_id':cid,'number':'01','status':'awaiting_native_visual_review','class_id':'core:technomancien',
        'scope':'One draped hood and collar identity pilot; no range and no assumed acceptance of the earlier Technomancien bodies.',
        'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
        'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
        'cowl_layer':layer.relative_to(ROOT).as_posix(),'cowl_layer_sha256':sha(layer),
        'default_head_layer':head_path.relative_to(ROOT).as_posix(),'default_head_layer_sha256':sha(head_path),
        'body_collar_layer':body_path.relative_to(ROOT).as_posix(),'body_collar_layer_sha256':sha(body_path),
        'head_body_seam_y':14,
        'base_body_candidate':base_item['candidate_id'],'base_native':base_item['native'],
        'native_dimensions':[64,64],'framing':framing,'new_color_variants':0,'runtime_integration':False,
        'checks':{'base_pixels_preserved_below_y':COLLAR_END,
            'base_face_retained_below_explicit_cowl_occlusion':True,
            'head_body_layers_recompose_exact_rgba':True,
            'all_prior_sources_and_accepted_references_preserved':protected,
            'scene_is_montage_not_engine_validation':True},
        'next_step':'Review this silhouette before deriving five class armor and default-head designs.'})
    section = '''<section id="technomancien-heads-01"><h2>Technomancien · 01 · nouvelle forme à examiner</h2>
<p>Un seul témoin avant de refaire une gamme : capuche de tissu profonde, silhouette asymétrique et col drapé autour du visage cybernétique. Le cuivre accompagne les prothèses et les raccords du manteau.</p>
<p class="note">La première progression de coiffes est abandonnée après ton refus : elle reprenait trop les formes du Bastion. Cette nouvelle proposition reste à valider en 64 × 64.</p>
<div class="native-scroll"><img class="board" width="384" height="358" src="technomancien-heads-01/identity-native64.png" alt="Bastion accepté, ancien témoin du Technomancien et proposition 01 à capuche drapée, en 64 × 64 sur trois fonds"></div>
<details><summary>Comparer les silhouettes en zoom entier ×3</summary><div class="native-scroll"><img class="board" width="624" height="263" src="technomancien-heads-01/identity-zoom-x3.png" alt="Comparaison des trois silhouettes en zoom entier ×3"></div></details>
<details><summary>Tête et raccord du col ×6</summary><div class="native-scroll"><img class="board" width="400" height="210" src="technomancien-heads-01/identity-head-collar-x6.png" alt="Ancien témoin et proposition 01, tête et col en zoom entier ×6"></div></details>
<div class="native-scroll"><img class="board" width="384" height="186" src="technomancien-heads-01/identity-scene-native64.png" alt="Bastion, ancien témoin et nouvelle proposition 01 sur le même sol à taille native"></div>
<p class="note">Le col est une nouvelle partie proposée avec la tête. Le reste du manteau et le bas du corps sont conservés. Montage de revue ; le rendu dans l’éclairage moteur reste à vérifier.</p>
<p>Pas de variante colorée. Aucun dessin intégré au jeu. La tête reste fournie par le torse, avec priorité au casque équipé. La tête et le col du corps ont des calques séparés ; les casques équipables restent à dessiner.</p>
<p><a href="../../docs/reviews/class-readability-01/technomancien-heads-01/REVIEW.md">Périmètre de la reprise</a> · <a href="../../docs/reviews/class-readability-01/technomancien-heads-01/identity-prompt.json">Prompt image_gen</a> · <a href="../../docs/reviews/class-readability-01/technomancien-heads-01/identity-candidate.json">Source et contrôles</a></p></section>'''
    (DEST/'section.html').write_text(section,'utf-8')
    root_path = REVIEW/'candidates.json'
    root = json.loads(root_path.read_text('utf-8'))
    root['approval_scopes']['default_head_appearances']='Bastion accepted; first Technomancien head progression rejected; one draped cowl-and-collar identity pilot awaiting review'
    root['latest_steering']='Change Technomancien geometry, not only colors. Review one asymmetrical soft cowl and collar pilot before another range. Preserve Bastion acceptance; no new colors or runtime integration.'
    write_json(root_path,root)
    policy_path = ROOT/'docs/reviews/class-readability-01/head-appearance-rule.json'
    policy = json.loads(policy_path.read_text('utf-8'))
    policy['technomancien_default_head_appearances']='first_head_progression_rejected_one_cowl_identity_pilot_pending_review'
    policy['technomancien_head_review']='docs/reviews/class-readability-01/technomancien-heads-01/identity-candidate.json'
    write_json(policy_path,policy)
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    print(json.dumps({'identity_pilots':1,'native_dimensions':[64,64],'cowl_box':framing['native_core_box'],
        'prior_files_unchanged':len(protected),'base_body_preserved_below_y':COLLAR_END,'runtime_integration':False}))


if __name__=='__main__':
    main()
