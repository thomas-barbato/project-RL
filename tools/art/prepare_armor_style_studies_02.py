"""Review targeted face corrections and the simplified copper Bastion."""
import json
import shutil

from PIL import Image, ImageDraw

from prepare_class_readability_review_01 import (
    ROOT, SOURCES, OUT, REVIEW, STREET, native_image, core_box, font, sha,
    write_json, gallery,
)

NAME='armor-style-studies-02'
ART, DEST, DOCS=SOURCES/NAME, OUT/NAME, REVIEW/NAME
OLD=SOURCES/'armor-style-studies-01'


def boards(sprites):
    labels=['01 · avant','01 · visage repris','02 · avant','02 · cuivre']
    board=Image.new('RGB',(512,400),'#152025');draw=ImageDraw.Draw(board)
    draw.text((10,8),'Bastion · avant / reprise · 64 × 64 natifs',font=font(16),fill='#efddbd')
    for i,label in enumerate(labels):
        draw.text((i*128+8,38),label,font=font(12),fill='#d8e2df')
    tile=Image.open(STREET).convert('RGB').crop((80,160,208,232))
    for row,(name,color) in enumerate([('Fond sombre','#343b3f'),('Fond clair','#858478'),('Sol de la rue · montage',None)]):
        y=62+110*row
        for i,sprite in enumerate(sprites):
            x=128*i
            if color:draw.rectangle((x,y,x+127,y+71),fill=color)
            else:board.paste(tile,(x,y))
            board.paste(sprite,(x+32,y+4),sprite)
        draw.text((10,y+76),name,font=font(12),fill='#b5c5c3')
    board.save(DEST/'comparison-native64.png')
    zoom=Image.new('RGB',(512,500),'#243036');draw=ImageDraw.Draw(zoom)
    draw.text((10,8),'Avant / reprise · zoom entier ×3',font=font(16),fill='#efddbd')
    for i,sprite in enumerate(sprites):
        x=24+(i%2)*256;y=34+(i//2)*232
        enlarged=sprite.resize((192,192),Image.Resampling.NEAREST)
        zoom.paste(enlarged,(x,y),enlarged)
        draw.text((x,y+195),labels[i],font=font(13),fill='#d8e2df')
    zoom.save(DEST/'comparison-zoom-x3.png')


def main():
    if (DOCS/'acceptance.json').exists():raise SystemExit('Accepted study is frozen.')
    for folder in (ART,DEST,DOCS):folder.mkdir(parents=True,exist_ok=True)
    prior=json.loads((DOCS/'prior-images.json').read_text('utf-8'))
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    plan=json.loads((DOCS/'prompts.json').read_text('utf-8'))
    entries=[];sprites=[]
    for i,job in enumerate(plan['jobs']):
        source=ART/(job['candidate_id']+'-source.png')
        generated=ROOT/job['generated_file']
        if generated.exists():shutil.copyfile(generated,source)
        assert source.exists()
        assert Image.open(source).convert('RGBA').getchannel('A').getextrema()==(0,255)
        framed,measure=native_image(source,58)
        old_path=OLD/f'BA-STYLE0{i+1}-r01-native64.png'
        previous=Image.open(old_path).convert('RGBA')
        ownership=None
        if i==0:
            # Explicit native head region: retain all armor and shoulders exactly.
            ownership=(27,3,37,12)
            mask=Image.new('L',(64,64));ImageDraw.Draw(mask).rectangle((27,3,36,11),fill=255)
            mask.save(ART/'BA-STYLE01-r02-head-edit-mask-native64.png')
            framed.save(ART/'BA-STYLE01-r02-edited-source-framed-native64.png')
            native=Image.composite(framed,previous,mask)
            for y in range(64):
                for x in range(64):
                    if mask.getpixel((x,y))==0:
                        assert native.getpixel((x,y))==previous.getpixel((x,y))
        else:native=framed
        path=ART/(job['candidate_id']+'-native64.png');native.save(path)
        shutil.copyfile(path,DEST/path.name)
        assert native.size==(64,64) and core_box(native)[3]==62
        assert native.getchannel('A').getextrema()==(0,255)
        sprites.extend([previous,native])
        entries.append({'number':job['number'],'candidate_id':job['candidate_id'],
            'label':job['label'],'status':'awaiting_visual_review',
            'source':source.relative_to(ROOT).as_posix(),'source_sha256':sha(source),
            'native':path.relative_to(ROOT).as_posix(),'native_sha256':sha(path),
            'previous_native':old_path.relative_to(ROOT).as_posix(),
            'previous_native_sha256':sha(old_path),'source_framing':measure,
            'actual_native_core_box':list(core_box(native)),
            'head_edit_ownership_box':list(ownership) if ownership else None,
            'outside_head_edit_exactly_preserved':i==0,
            'foot_anchor':[32,61],'layer_status':'whole_figure_study_not_a_production_rig'})
    boards(sprites)
    write_json(DOCS/'candidates.json',{'review_id':'ARMOR-STYLE-STUDIES02','date':'2026-10-21',
        'status':'awaiting_visual_review','native_dimensions':[64,64],
        'runtime_integration':False,'color_variants':'deferred',
        'previous_feedback':'docs/reviews/class-readability-01/armor-style-studies-01/feedback-01.json',
        'entries':entries})
    section='''<section id="armor-style-studies-02"><h2>Bastion · visages repris et cuivre simplifié</h2>
<p><strong>01</strong> : tenue conservée à l’identique en 64 px, visage humain dégagé.
<br><strong>02</strong> : cuivre plus sobre, petites bandes décoratives retirées, grandes plaques conservées.</p>
<p class="note">Avant / reprise à taille réelle. La plaque faciale et l’œil lumineux disparaissent des deux essais.
Le Briseur utilise désormais la même réduction avec Lanczos que le 01 ; les zooms restent entiers avec Nearest.</p>
<div class="native-scroll"><img class="board" width="512" height="400" src="armor-style-studies-02/comparison-native64.png" alt="Bastion 01 et 02 avant et après reprise, en 64 px sur trois fonds"></div>
<details><summary>Voir les formes au zoom entier ×3</summary><div class="native-scroll"><img class="board" width="512" height="500" src="armor-style-studies-02/comparison-zoom-x3.png" alt="Avant et reprise des deux Bastion au zoom entier trois"></div></details>
<details><summary>Voir l’effet de l’ancienne réduction sur la même source</summary><div class="native-scroll"><img class="board" width="432" height="250" src="armor-style-studies-02/sampling-audit-x3.png" alt="Même ancien Briseur réduit avec Nearest et Lanczos, affiché au zoom entier trois"></div></details>
<p class="note">Études à examiner ; casques séparés, prises d’armes et intégration restent à préparer.
La rue est un montage sur une capture existante.
<a href="../../docs/reviews/class-readability-01/armor-style-studies-02/REVIEW.md">Dossier de reprise</a> ·
<a href="../../docs/reviews/class-readability-01/armor-style-studies-02/prompts.json">Prompts exacts</a></p></section>'''
    (DEST/'section.html').write_text(section,encoding='utf-8')
    root=json.loads((REVIEW/'candidates.json').read_text('utf-8'))
    gallery(root['entries'],root['equipment_inventory'])
    assert all(sha(ROOT/p)==h for p,h in prior.items())
    checks={'review_id':'ARMOR-STYLE-STUDIES02','native_candidates':2,
        'native_dimensions':[64,64],'transparent_backgrounds':True,'foot_baseline_y':61,
        '01_pixels_outside_head_ownership_identical':True,
        '01_head_edit_ownership':[27,3,37,12],'source_reduction':'Lanczos for both revised studies',
        'review_zoom':'integer Nearest x3','prior_images_and_acceptances_unchanged':len(prior),
        'runtime_integration':False,'production_rigs':False,'scene':'montage only'}
    write_json(DOCS/'checks.json',checks)
    print(json.dumps(checks,ensure_ascii=False))


if __name__=='__main__':main()
