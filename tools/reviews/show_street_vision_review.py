"""Native comparison of the authored interior sight grid, with unchanged movement."""
import hashlib
import html
import json
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'artifacts/street-expansion-review-01/vision-02'
BASELINE=OUT.parent/'vision-01/engine'
DOC=ROOT/'docs/reviews/refonte-histoire-2026-10-16/street-expansion-review-01'
LABELS={
    'gate':'Sortie de prison',
    'join-middle':'Rue et angle de bâtiment',
    'house-a-closed':'Porte fermée et toit',
    'house-a-open':'Intérieur du logement',
    'house-b-open':'Intérieur de la boutique',
    'west-corner':'Angle de l’enceinte',
}


def build():
    checks=json.loads((OUT/'engine/checks.json').read_text(encoding='utf-8'))
    assert checks['review']=='VISION-U01-02' and checks['vision_review']
    assert checks['candidate_only'] and checks['camp_round_trip'] and checks['native_cell']==64
    assert {r['scene'] for r in checks['scenes']}==set(LABELS)
    baseline_checks=json.loads((BASELINE/'checks.json').read_text(encoding='utf-8'))
    baseline_scenes={r['scene']:r for r in baseline_checks['scenes']}
    rows=[];cards=[]
    # Put the two actual corrections first; unchanged controls belong last.
    order=['house-a-open','house-b-open','house-a-closed','join-middle','gate','west-corner']
    for record in sorted(checks['scenes'],key=lambda r:order.index(r['scene'])):
        name=record['scene'];title=LABELS[name]
        assert record['render_read_only'] and record['cell'][2:]==[64.,64.]
        assert record['player']==baseline_scenes[name]['player']
        assert record['cell']==baseline_scenes[name]['cell']
        x,y,_,_=record['cell']
        before=Image.open(BASELINE/f'{name}.png').convert('RGB')
        after=Image.open(OUT/f'engine/{name}.png').convert('RGB')
        assert before.size==after.size
        left=max(0,min(before.width-768,round(x-352)))
        top=max(0,min(before.height-512,round(y-320)))
        crop=(left,top,left+768,top+512)
        before=before.crop(crop);after=after.crop(crop)
        before.save(OUT/f'{name}-before.png');after.save(OUT/f'{name}-after.png')
        delta=np.abs(np.array(after,dtype=int)-np.array(before,dtype=int))
        changed=int(np.any(delta>2,axis=2).sum())
        if name in ('house-a-open','house-b-open'):assert changed>0,name
        rows.append(dict(record,title=title,crop=list(crop),changed_pixels=changed))
        version=hashlib.sha256(after.tobytes()).hexdigest()[:12]
        cards.append(f'<h2>{html.escape(title)}</h2><div class="pair"><figure><figcaption>Avant</figcaption><img src="{name}-before.png?v={version}" width="768" height="512"></figure><figure><figcaption>VISION-U01-02 · grille intérieure corrigée</figcaption><img src="{name}-after.png?v={version}" width="768" height="512"></figure></div>')
    # Two clearly labelled views of the SAME scene, with no rescaling or stitching.
    pair=Image.new('RGB',(1568,562),'#10191e')
    draw=ImageDraw.Draw(pair);font=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',20)
    for offset,variant,title in [(12,'before','Avant'),(788,'after','VISION-U01-02 · grille intérieure corrigée')]:
        draw.text((offset,12),title,font=font,fill='#dec898')
        pair.paste(Image.open(OUT/f'house-a-open-{variant}.png'),(offset,44))
    pair.save(OUT/'comparaison-logement.png')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>VISION-U01-02</title>
<style>body{margin:24px;background:#10191e;color:#dae1dd;font:17px Segoe UI,sans-serif}h1,h2,figcaption{color:#dec898}p{max-width:1080px;line-height:1.5}.pair{display:flex;gap:8px;overflow:auto}figure{margin:0 0 20px}figcaption{padding:10px 0}img{display:block;max-width:none}a{color:#e5c58a}</style>
<h1>VISION-U01-02 · carte logique des intérieurs</h1>
<p><strong>Diagnostic précédent.</strong> La direction demandée ensuite par le joueur est présentée dans <a href="../vision-04/index.html">VISION-U01-04 · tout le décor dans le cercle</a>.</p>
<p>À gauche, le rendu actuel. À droite, quatorze cases de sol ou de mobilier bas cessent de bloquer la vue comme des murs. Même position, mêmes portes, mêmes lumières et mêmes collisions de déplacement. Les murs extérieurs et meubles hauts restent opaques. L’algorithme de vision et le dessin de ses bords sont conservés.</p>
<p>Captures réelles du moteur : un pixel pour un pixel, cases de 64 px. Proposition visuelle à examiner avant activation dans une partie normale.</p>
<p><a href="../index.html">Retour à la revue de la rue</a></p>'''+''.join(cards)+'</html>'
    (OUT/'index.html').write_text(page,encoding='utf-8')
    report=dict(review='VISION-U01-02',status='visual_review_pending',native_cell=64,
        candidate_only=True,normal_game_enabled=False,world_state_unchanged=True,
        comparison_same_positions=True,vision_algorithm_unchanged=True,
        movement_unchanged=True,unknown_cell_gate='existing opaque mask retained',
        corrected_cells=14,scenes=rows)
    (DOC/'vision-checks-02.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps(dict(review=report['review'],same_position_pairs=len(rows),native_cell=64)))


if __name__=='__main__':build()
