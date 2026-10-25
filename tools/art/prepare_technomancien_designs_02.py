"""Review five new Technomancien designs; keep rejected and accepted art intact."""
import json
import shutil
from pathlib import Path

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import ROOT, REVIEW, STREET, font, core_box, sha, write_json, gallery
from prepare_technomancien_wave_02 import ART, DEST, DOCS, SEAM, compose, pixels_hash

LABELS = ['Très légère', 'Légère', 'Interméd.', 'Lourde', 'Très lourde']


def boards(sprites, comparison):
    board = Image.new('RGB', (400,348), '#152025')
    draw = ImageDraw.Draw(board)
    draw.text((8,9), 'Technomancien · cinq dessins · 64 × 64 natifs', font=font(14), fill='#efddbd')
    for i,label in enumerate(LABELS):
        draw.text((i*80+4,36), f'{i+1:02d} · {label}', font=font(10), fill='#d5dfdc')
    for row,(label,color) in enumerate([('Sol sombre','#343b3f'),('Sol clair','#858478'),('Pénombre','#12181d')]):
        y = 72+row*90
        draw.rectangle((4,y-3,396,y+66),fill=color)
        for i,sprite in enumerate(sprites):
            board.paste(sprite,(8+i*80,y),sprite)
        draw.text((8,y+68),label,font=font(11),fill='#b5c5c3')
    board.save(DEST/'designs-native64.png')
    board = Image.new('RGB',(1040,263),'#243036')
    draw = ImageDraw.Draw(board)
    draw.text((10,8),'Cinq coupes · même tête · zoom entier ×3',font=font(17),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        zoom = sprite.resize((192,192),Image.Resampling.NEAREST)
        board.paste(zoom,(8+i*208,38),zoom)
        draw.text((12+i*208,234),f'{i+1:02d} · {LABELS[i]}',font=font(13),fill='#d5dfdc')
    board.save(DEST/'designs-zoom-x3.png')
    board = Image.new('RGB',(384,152),'#343b3f')
    draw = ImageDraw.Draw(board)
    draw.text((8,8),'Comparer les identités · taille native',font=font(15),fill='#efddbd')
    for i,(sprite,label) in enumerate(zip(comparison,['Bastion validé','Technomancien refusé','Nouvelle direction'])):
        draw.text((6+i*128,39),label,font=font(11),fill='#d5dfdc')
        board.paste(sprite,(32+i*128,66),sprite)
    board.save(DEST/'design-identity-comparison-native64.png')
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
    board.save(DEST/'designs-scene-native64.png')


def main():
    if (DOCS/'design-acceptance.json').exists():
        raise SystemExit('Accepted designs are frozen; create a new revision.')
    spec = json.loads((DOCS/'design-prompts.json').read_text('utf-8'))
    assert len(spec['entries'])==5 and spec['color_variants']=='deferred_by_user'
    bastion = json.loads((REVIEW/'bastion-wave-01/acceptance.json').read_text('utf-8'))
    rejected = json.loads((DOCS/'rejection.json').read_text('utf-8'))
    protected = {}
    for item in bastion['entries']:
        for key in ('native','body_layer'):
            protected[item[key]['path']] = item[key]['sha256']
        protected[item['source']] = item['source_sha256']
    for item in rejected['entries']:
        for key in ('native','source'):
            protected[item[key]] = item[key+'_sha256']
    for path in ['content/core/classes/technomancien.json5','assets/sprites/art-finish-02/FIN-14.png']:
        protected[path] = sha(ROOT/path)
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    head = Image.open(ROOT/spec['head_layer']).convert('RGBA')
    head_hash = pixels_hash(head.crop((0,0,64,SEAM)))
    sprites,entries = [],[]
    for entry in spec['entries']:
        cid = entry['candidate_id']
        source = ART/(cid+'-body-source.png')
        generated = Path(entry['generated_file'])
        if generated.exists():
            shutil.copyfile(generated,source)
        else:
            assert source.exists(), source
        body,sprite,measure = compose(source,head)
        assert sprite.size==(64,64)
        assert pixels_hash(sprite.crop((0,0,64,SEAM)))==head_hash
        bounds = core_box(sprite)
        assert bounds[0]>0 and bounds[2]<64 and bounds[3]==62
        native,layer = DEST/(cid+'-native64.png'),DEST/(cid+'-body-native64.png')
        sprite.save(native)
        body.save(layer)
        entries.append({'number':entry['number'],'candidate_id':cid,'label':entry['label'],
            'status':'awaiting_native_visual_review','item_id':None,
            'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':native.relative_to(ROOT).as_posix(),'native_sha256':sha(native),
            'body_layer':layer.relative_to(ROOT).as_posix(),'body_layer_sha256':sha(layer),
            'measurements':measure,'assembled_core_box':list(bounds),'head_region_sha256':head_hash})
        sprites.append(sprite)
    bastion_medium = next(e for e in bastion['entries'] if e['candidate_id']=='BA-W01-P03-r01')
    old_tech = next(e for e in rejected['entries'] if e['candidate_id']=='TE-W02-P03-r01')
    comparison = [Image.open(ROOT/bastion_medium['native']['path']).convert('RGBA'),
                  Image.open(ROOT/old_tech['native']).convert('RGBA'),sprites[2]]
    boards(sprites,comparison)
    assert all(sha(ROOT/path)==digest for path,digest in protected.items())
    write_json(DOCS/'design-candidates.json',{'review_id':'TECHNOMANCIEN-W02','revision':'r02-designs-only',
        'date':'2026-10-21','status':'awaiting_native_visual_review','class_id':'core:technomancien',
        'models':5,'native_dimensions':[64,64],'color_variants':'deferred_by_user',
        'new_color_variants_generated':0,'head_layer':spec['head_layer'],
        'head_layer_sha256':sha(ROOT/spec['head_layer']),'head_region':[0,0,64,SEAM],
        'head_region_sha256':head_hash,'entries':entries,'runtime_integration':False,
        'direction_feedback_only':'User likes the new coat design; native views and additional models require review.',
        'integration_gate':'All armor designs, helmets and weapon hand positions must be ready and reviewed.',
        'checks':{'all_five_native64':True,'head_pixels_identical':True,
            'accepted_bastion_and_rejected_tech_sources_preserved':protected,
            'scene_is_review_montage_not_engine_validation':True}})
    details = ''.join(f'<li>{e["number"]} · {e["label"]} · {e["candidate_id"]}</li>' for e in entries)
    (DEST/'section.html').write_text('''<section id="technomancien-wave-02"><span id="technomancien-colors"></span>
<h2>Vague 02 · Technomancien · nouvelle direction</h2>
<p>La première série est refusée : trop proche du Bastion et illisible à 64 px. La reprise s’appuie sur le manteau technique et les prothèses cuivrées que tu apprécies.</p>
<h3 id="technomancien-designs">Cinq dessins · du très léger au très lourd</h3>
<p><strong>Le design d’abord.</strong> Une veste courte, un manteau léger, la tenue témoin, puis deux manteaux plus protecteurs. Ces dessins montrent les corps : bras, mains, jambes et bottes évoluent ensemble. Les têtes assorties au torse restent à préparer selon la nouvelle règle. Les variantes de couleur sont reportées.</p>
<div class="native-scroll"><img class="board" width="400" height="348" src="technomancien-wave-02/designs-native64.png" alt="Cinq nouvelles tenues du Technomancien, numérotées de très légère à très lourde, en 64 × 64 natifs sur trois fonds"></div>
<details><summary>Zoom entier ×3 des cinq dessins</summary><div class="native-scroll"><img class="board" width="1040" height="263" src="technomancien-wave-02/designs-zoom-x3.png" alt="Les cinq nouvelles tenues du Technomancien, zoom entier ×3"></div></details>
<h3>Identité de classe · comparaison à taille réelle</h3>
<div class="native-scroll"><img class="board" width="384" height="152" src="technomancien-wave-02/design-identity-comparison-native64.png" alt="Bastion validé, ancien Technomancien refusé et nouvelle tenue témoin, tous en taille native"></div>
<div class="native-scroll"><img class="board" width="400" height="186" src="technomancien-wave-02/designs-scene-native64.png" alt="Les cinq nouveaux dessins sur le sol de la rue, montage à taille native"></div>
<p class="note">La tête commune de ces cinq vues est provisoire : chaque torse fournira sa tête par défaut, remplacée visuellement lorsqu’un casque sera équipé. Le montage permet de comparer la lecture sur le sol ; l’éclairage moteur reste à vérifier.</p>
<details><summary>Identifiants des cinq propositions</summary><ul>'''+details+'''</ul></details>
<p>Ces cinq dessins attendent leur revue à taille native. Le Bastion validé est conservé. L’intégration attend toutes les armures, les casques et les bonnes positions des mains avec les armes.</p>
<p><a href="../../docs/reviews/class-readability-01/technomancien-wave-02/REVIEW.md">Périmètre corrigé</a> · <a href="../../docs/reviews/class-readability-01/technomancien-wave-02/design-prompts.json">Prompts image_gen des nouveaux dessins</a> · <a href="../../docs/reviews/class-readability-01/technomancien-wave-02/design-candidates.json">Sources et contrôles</a> · <a href="../../docs/reviews/class-readability-01/technomancien-wave-02/rejection.json">Refus de la première série</a></p></section>''','utf-8')
    candidates_path = REVIEW/'candidates.json'
    candidates = json.loads(candidates_path.read_text('utf-8'))
    candidates['approval_scopes']['body_armor_variants']='Bastion accepted; first Technomancien series rejected; five new coat-based designs awaiting native review; color variants deferred'
    candidates['latest_steering']='Focus on beautiful distinct class armor designs in native64. No new color variants. Integration waits for armor designs, helmets and weapon hand positions.'
    candidates['color_variants']='deferred_by_user'
    write_json(candidates_path,candidates)
    gallery(candidates['entries'],candidates['equipment_inventory'])
    print(json.dumps({'new_designs':5,'color_variants_generated':0,'heads_identical':True,'bastion_and_rejected_drawings_preserved':True}))


if __name__=='__main__':
    main()
