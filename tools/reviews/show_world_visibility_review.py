"""Present actual paired captures of the common scenery fog pass."""
import hashlib
import html
import json
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
CIRCLE='--circle' in sys.argv
REVIEW='VISION-U01-04' if CIRCLE else 'VISION-U01-03'
OUT=ROOT/'artifacts/street-expansion-review-01'/('vision-04' if CIRCLE else 'vision-03')
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
LABELS={
    'join-middle':'Rue · champ de vision et décors',
    'court-clean':'Cour · contour de portée',
    'camp':'Camp · même traitement des décors',
    'house-a-open':'Logement · intérieur',
    'house-b-open':'Boutique · intérieur',
    'house-a-closed':'Porte fermée et toit',
    'gate':'Sortie de prison',
    'west-corner':'Angle de l’enceinte',
}


def build():
    checks=json.loads((OUT/'engine/checks.json').read_text(encoding='utf-8'))
    assert checks['review']==REVIEW and checks['unified_review']
    assert checks['candidate_only'] and checks['camp_round_trip'] and checks['native_cell']==64
    captures={r['scene']:r for r in checks['scenes']}
    assert set(captures)==set(LABELS)
    records=[];cards=[]
    for name,title in LABELS.items():
        record=captures[name]
        assert record['render_read_only'] and record['cell'][2:]==[64.,64.]
        before=Image.open(OUT/f'engine/{name}.png').convert('RGB')
        after=Image.open(OUT/f'engine/{name}-unified.png').convert('RGB')
        assert before.size==after.size
        x,y,_,_=record['cell']
        left=max(0,min(before.width-768,round(x-352)))
        top=max(0,min(before.height-512,round(y-320)))
        crop=(left,top,left+768,top+512)
        before=before.crop(crop);after=after.crop(crop)
        before.save(OUT/f'{name}-before.png');after.save(OUT/f'{name}-after.png')
        delta=np.abs(np.asarray(before,dtype=int)-np.asarray(after,dtype=int))
        changed=int(np.any(delta>2,axis=2).sum())
        records.append(dict(record,title=title,crop=list(crop),changed_pixels=changed))
        digest=hashlib.sha256(before.tobytes()+after.tobytes()).hexdigest()[:12]
        card=f'''<section><h2>{html.escape(title)}</h2><div class="controls"><button data-view="before">Masque actuel</button><button data-view="after" class="active">VISION-U01-03</button></div><div class="scene" data-view="after"><img class="before" src="{name}-before.png?v={digest}" width="768" height="512" alt="Masque actuel"><img class="after" src="{name}-after.png?v={digest}" width="768" height="512" alt="Masque unique proposé"></div><details><summary>Voir les deux captures côte à côte</summary><div class="pair"><figure><figcaption>Masque actuel</figcaption><img src="{name}-before.png?v={digest}" width="768" height="512"></figure><figure><figcaption>VISION-U01-03</figcaption><img src="{name}-after.png?v={digest}" width="768" height="512"></figure></div></details></section>'''
        cards.append(card)
    primary=next(r for r in records if r['scene']=='join-middle')
    assert primary['changed_pixels']>3000,'Do not present an unchanged street as the main comparison'
    font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',20)
    board=Image.new('RGB',(1568,562),'#10191e');d=ImageDraw.Draw(board)
    frames=[]
    for x,variant,title in [(12,'before','Masque actuel'),(788,'after',REVIEW+(' · décor dans le cercle' if CIRCLE else ' · masque unique'))]:
        d.text((x,12),title,font=font,fill='#dec898')
        im=Image.open(OUT/f'join-middle-{variant}.png').convert('RGB')
        board.paste(im,(x,44))
        frame=Image.new('RGB',(792,562),'#10191e');fd=ImageDraw.Draw(frame)
        fd.text((12,12),title,font=font,fill='#dec898');frame.paste(im,(12,44));frames.append(frame)
    board.save(OUT/'comparaison-rue.png')
    frames[0].save(OUT/'comparaison-rue.gif',save_all=True,append_images=frames[1:],duration=1200,loop=0)
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>VISION-U01-03</title>
<style>body{margin:24px;background:#10191e;color:#dce3df;font:17px Segoe UI,sans-serif}h1,h2,figcaption{color:#dec898}p{max-width:1050px;line-height:1.5}button{background:#25343b;color:#e4ddd0;border:1px solid #64777d;padding:9px 18px;cursor:pointer}button.active{background:#806d46;color:#fff}.scene{position:relative;width:768px;height:512px;border:1px solid #64777d;margin:10px 0}.scene img{position:absolute;inset:0}.scene[data-view="before"] .after{display:none}img{display:block;max-width:none}.pair{display:flex;gap:12px;overflow:auto}figure{margin:10px 0}figcaption{padding:6px 0}section{margin:30px 0}a{color:#e3c88e}</style>
<h1>VISION-U01-03 · un masque commun pour les décors</h1>
<p>Un objet de décor aperçu conserve sa silhouette complète : le brouillard ne le découpe plus suivant la grille du sol. Cela concerne les ressources isolées de la rue et le mobilier du camp. Les pièces assemblées gardent leurs murs et leur sol propres : voir une façade ne dévoile pas leur contenu.</p>
<p>Le sol inconnu reste masqué. Le décor découvert reste assombri hors de la vue actuelle, avec une transition de portée progressive. La règle ne dépend pas du fait d’être dedans ou dehors. Les ennemis restent soumis à leur visibilité réelle.</p>
<p>Les boutons alternent deux captures du même état de partie, à la même position et avec les mêmes lumières. Cases de 64 px, affichage sans zoom. La portée et la visibilité des acteurs restent celles du moteur. Proposition en revue, non activée dans une partie normale.</p>'''+''.join(cards)+'''<script>document.querySelectorAll('.controls button').forEach(button=>button.onclick=()=>{const section=button.closest('section');section.querySelector('.scene').dataset.view=button.dataset.view;section.querySelectorAll('button').forEach(b=>b.classList.toggle('active',b===button));});</script></html>'''
    if CIRCLE:
        start=page.index('<h1>');end=page.index('<section>')
        page=page[:start]+'''<h1>VISION-U01-04 · tout le décor dans le cercle</h1>
<p>Le cercle détermine l’affichage du décor, à l’intérieur comme à l’extérieur. Les murs ne créent plus de découpes noires à l’intérieur du cercle. La limite de portée est progressive ; les lumières et la pénombre restent celles de la scène.</p>
<p>Les ennemis, les objets interactifs et les attaques restent soumis aux règles réelles de visibilité et d’obstacles. Les toits suivent les portes comme auparavant. Le terrain déjà connu hors du cercle est assombri ; l’espace inconnu au-delà reste noir.</p>
<p>Les boutons alternent deux captures du même état de partie, avec les mêmes lumières, à 64 px par case et sans zoom. Ce prototype ne change pas les sauvegardes et n’est pas activé en partie normale. Il ne découvre pas de cellules pour le ciblage ou le déplacement.</p>'''+page[end:]
    page=page.replace('VISION-U01-03',REVIEW)
    (OUT/'index.html').write_text(page,encoding='utf-8')
    report=dict(review=REVIEW,status='visual_review_pending',native_cell=64,
        candidate_only=True,normal_game_enabled=False,comparison_same_state=True,
        shared_camp_and_street=True,world_state_unchanged_by_render=True,
        memory_brightness=0.45,scenes=records)
    report['circular_scenery_without_wall_occlusion']=CIRCLE
    report['actor_visibility_unchanged']=True
    if not CIRCLE:
        report['complete_static_scenery_surfaces']=True
        report['surface_indexes']={name:len(json.loads((ROOT/'assets/source-art/first-sector-review-01/u01-expansion-r06'/f'visibility-surfaces-{name}-03.json').read_text(encoding='utf-8'))['objects']) for name in ['street','camp']}
    (DOC/('vision-checks-04.json' if CIRCLE else 'vision-checks-03.json')).write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(review=report['review'],pairs=len(records),primary_changed_pixels=primary['changed_pixels'])))


if __name__=='__main__':build()
