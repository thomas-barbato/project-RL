"""Frame two whole-figure studies and present them; no runtime art mutation."""
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, core_box, native_image, font, sha,
    write_json, gallery,
)

NAME = 'armor-style-studies-01'
ART, DEST, DOCS = SOURCES/NAME, OUT/NAME, REVIEW/NAME


def pixel_frame(source):
    """Technical isotropic framing of the pixel-oriented source, no repainting."""
    original = Image.open(source).convert('RGBA')
    x0,y0,x1,y1 = core_box(original)
    pad = max(2,round((y1-y0)*0.015))
    box = (max(0,x0-pad),max(0,y0-pad),min(original.width,x1+pad),min(original.height,y1+pad))
    crop = original.crop(box)
    scale = 58/(y1-y0)
    reduced = crop.resize((round(crop.width*scale),round(crop.height*scale)),Image.Resampling.NEAREST)
    bx0,by0,bx1,by1 = core_box(reduced)
    result = Image.new('RGBA',(64,64))
    result.alpha_composite(reduced,(round(32-(bx0+bx1)/2),62-by1))
    bounds=core_box(result)
    assert bounds and bounds[0]>0 and bounds[2]<64 and bounds[1]>=1 and bounds[3]==62
    return result, {'source_core_box':[x0,y0,x1,y1], 'source_crop':list(box),
                    'isotropic_scale':scale, 'native_core_box':list(bounds),
                    'framing':'isotropic, Nearest, source alpha retained',
                    'foot_baseline_y':61}


def boards(sprites):
    names=['Ancien Bastion 03','01 · Vétéran','02 · Briseur']
    board=Image.new('RGB',(444,400),'#152025')
    d=ImageDraw.Draw(board)
    d.text((10,8),'Deux études · personnages en 64 × 64',font=font(16),fill='#efddbd')
    for i,label in enumerate(names):
        d.text((i*148+10,37),label,font=font(12),fill='#d8e2df')
    patch=Image.open(STREET).convert('RGB').crop((80,160,480,256))
    # Reuse the same empty ground patch behind each candidate.
    tile=patch.crop((0,0,148,72))
    for row,(label,color) in enumerate([('Fond sombre','#343b3f'),('Fond clair','#858478'),('Sol de la rue · montage',None)]):
        y=62+row*110
        for i,sprite in enumerate(sprites):
            x=i*148
            if color is None:board.paste(tile,(x,y))
            else:d.rectangle((x,y,x+147,y+71),fill=color)
            board.paste(sprite,(x+42,y+4),sprite)
        d.text((10,y+76),label,font=font(12),fill='#b5c5c3')
    board.save(DEST/'comparison-native64.png')
    zoom=Image.new('RGB',(648,264),'#243036');d=ImageDraw.Draw(zoom)
    d.text((10,8),'Contrôle des formes · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        enlarged=sprite.resize((192,192),Image.Resampling.NEAREST)
        zoom.paste(enlarged,(12+i*216,38),enlarged)
        d.text((12+i*216,235),names[i],font=font(13),fill='#d8e2df')
    zoom.save(DEST/'comparison-zoom-x3.png')


def main():
    if (DOCS/'acceptance.json').exists():
        raise SystemExit('Accepted study is frozen; create a revision.')
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    previous=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==digest for p,digest in previous.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    old_path=OUT/'class-armors-all-02/BA-W01-H03-r01-native64.png'
    old=Image.open(old_path).convert('RGBA')
    sprites=[old];entries=[]
    for i,job in enumerate(plan['jobs']):
        source=ART/(job['candidate_id']+'-source.png')
        generated=ROOT/job['generated_file']
        if generated.exists():shutil.copyfile(generated,source)
        assert source.exists(),source
        original=Image.open(source).convert('RGBA')
        assert original.getchannel('A').getextrema()==(0,255)
        native,measure=(native_image(source,58) if i==0 else pixel_frame(source))
        native_path=ART/(job['candidate_id']+'-native64.png');native.save(native_path)
        shutil.copyfile(native_path,DEST/native_path.name)
        assert native.size==(64,64) and native.getchannel('A').getextrema()==(0,255)
        sprites.append(native)
        entries.append({'number':job['number'],'candidate_id':job['candidate_id'],'label':job['label'],
            'status':'awaiting_visual_approval', 'source':source.relative_to(ROOT).as_posix(),
            'source_sha256':sha(source), 'native':native_path.relative_to(ROOT).as_posix(),
            'native_sha256':sha(native_path), 'measurements':measure,
            'layer_status':'whole_figure_style_study_not_an_equippable_rig',
            'foot_anchor':[32,61], 'head_and_grip_anchors':'to_measure_after_style_selection',
            'production_layers':'body, torso default head, arms, weapon and hand occlusions to prepare after review'})
    boards(sprites)
    candidate_data={'review_id':'ARMOR-STYLE-STUDIES01','date':'2026-10-21',
        'status':'awaiting_visual_approval','native_dimensions':[64,64],
        'runtime_integration':False,'color_variants':'deferred',
        'scope':'Two alternative whole Bastion designs, not armor progression or production rigs.',
        'historical_control':{'path':old_path.relative_to(ROOT).as_posix(),'sha256':sha(old_path)},
        'prior_acceptances':'preserved; latest feedback reopens the overall aesthetic including Bastion',
        'entries':entries}
    write_json(DOCS/'candidates.json',candidate_data)
    section='''<section id="armor-style-studies-01"><h2>Reprise du style · deux études du Bastion</h2>
<p>Le retour porte désormais sur l’esthétique de l’ensemble, y compris le Bastion.
Deux dessins complets permettent de choisir une direction avant de refaire les gammes.</p>
<p><strong>01 · Vétéran industriel</strong> : métal usé, cuir et tissu, épaule asymétrique.
<br><strong>02 · Briseur cybernétique</strong> : plaques anguleuses, grandes masses ocre et ardoise.</p>
<p class="note">Les deux propositions changent aussi les formes et les matières : cette comparaison ne mesure pas seulement le filtrage.
L’ancien Bastion 03 reste le témoin. Chaque personnage occupe exactement 64 × 64 px.</p>
<div class="native-scroll"><img class="board" width="444" height="400" src="armor-style-studies-01/comparison-native64.png" alt="Ancien Bastion et études 01 et 02 à taille native, sur fond sombre, clair et sur la rue"></div>
<details><summary>Voir le zoom entier ×3</summary><div class="native-scroll"><img class="board" width="648" height="264" src="armor-style-studies-01/comparison-zoom-x3.png" alt="Zoom entier des trois Bastion"></div></details>
<p class="note">Études de style : les calques de casque et les prises d’armes seront préparés après le choix du dessin.
La rue est un montage sur une capture existante ; l’éclairage moteur reste à tester.
<a href="../../docs/reviews/class-readability-01/armor-style-studies-01/REVIEW.md">Dossier des études</a> ·
<a href="../../docs/reviews/class-readability-01/armor-style-studies-01/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root_data=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    gallery(root_data['entries'],root_data['equipment_inventory'])
    assert all(sha(ROOT/p)==digest for p,digest in previous.items())
    checks={'review_id':'ARMOR-STYLE-STUDIES01','native_candidates':2,'native_dimensions':[64,64],
        'alpha_backgrounds_transparent':True,'foot_baseline_y':61,
        'native_review_sprite_scale':1,'zoom_scale':3,
        'prior_images_and_acceptances_unchanged':len(previous),
        'runtime_integration':False,'production_rigs':False,
        'scene_validation':'static montage; engine lighting and grips deferred'}
    write_json(DOCS/'checks.json',checks)
    print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
