"""ANIM-FLOW-01: publish native whole-effect comparisons in the current preview."""
import json
from pathlib import Path

from PIL import Image, ImageChops
import build_animation_readability_review as previous

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "artifacts/refonte-agent-2026-10-06/animation-shape-01"
CURRENT = OUT.parent / "apercu-animations.html"
DOC = ROOT / "docs/reviews/animation-shape-01"

SCRIPT = r'''
'use strict';
const data=__DATA__,folder='./animation-shape-01/',el=id=>document.getElementById(id);
const contexts=[el('before').getContext('2d'),el('after').getContext('2d')];
contexts.forEach(c=>c.imageSmoothingEnabled=false);
for(const row of data.cases){const option=document.createElement('option');option.value=row.number;option.textContent=`${String(row.number).padStart(2,'0')} · ${row.title}`;el('case').append(option);}
let row=data.cases[0],sheets=null,playing=true,age=0,last=performance.now(),generation=0;
function notes(){const duration=Math.round(row.seconds*1000*(el('slower').checked?row.duration_scale:1));document.querySelector('.pane h2').textContent=row.before_label||'Découpe actuelle';document.querySelector('.pane.proposed h2').textContent=row.after_label||'Forme naturelle';el('before-note').textContent=row.loop_seconds?`${duration} ms par cycle visuel · intensité actuelle`:`${duration} ms · dessin limité aux cases`;el('after-note').textContent=row.loop_seconds?`${duration} ms par cycle visuel · neige et glace plus claires`:`${duration} ms · dessin entier, proportions conservées`;el('scrub').max=row.loop_seconds?Math.round(row.loop_seconds*(el('slower').checked?row.duration_scale:1)*3000):2100;el('rationale').textContent=row.rationale||(row.physical_wall?'Le dessin conserve son ampleur sur le sol libre. La cloison physique arrête sa partie droite ; aucun éclat ne traverse le mur.':'Le dessin suit les contours peints de l’effet. Il peut dépasser dans les cases voisines sans y appliquer de dégâts.');}
async function readImage(name){const img=new Image();img.src=(row.folder?'./'+row.folder+'/':folder)+name;await img.decode();if(img.naturalWidth!==data.frame_width*data.columns||img.naturalHeight!==data.frame_height*Math.ceil(data.frames/data.columns))throw Error('Dimensions de planche incorrectes : '+name);return img;}
async function load(){const ticket=++generation;row=data.cases.find(r=>r.number===Number(el('case').value));const light=el('light').value,n=String(row.file_number||row.number).padStart(2,'0');sheets=null;el('status').className='muted';el('status').textContent='Chargement des captures du moteur…';notes();try{const images=await Promise.all([readImage(`${n}-${light}-0.png`),readImage(`${n}-${light}-1.png`)]);if(ticket!==generation)return;sheets=images;age=0;last=performance.now();el('status').textContent=`${row.review_id||'ANIM-FLOW-01-'+n} · ${row.title} · ${light==='sombre'?'sol sombre':'sol plus éclairé'} · cases de 64 px`;draw();}catch(error){if(ticket!==generation)return;el('status').className='error';el('status').textContent='Aperçu indisponible : '+error.message;}}
function paint(context,img,time){if(row.loop_seconds)time%=row.loop_seconds;const frame=Math.max(0,Math.min(data.frames-1,Math.floor(time*data.fps)));context.drawImage(img,(frame%data.columns)*data.frame_width,Math.floor(frame/data.columns)*data.frame_height,data.frame_width,data.frame_height,0,0,data.frame_width,data.frame_height);if(el('zone').checked){context.save();context.strokeStyle='#9cd8ee';context.lineWidth=1;context.setLineDash([4,4]);for(const [x,y] of row.affected_cells)context.strokeRect(x*64+.5,y*64+.5,63,63);context.restore();}}
function draw(){if(!sheets)return;const t=age/(el('slower').checked?row.duration_scale:1);paint(contexts[0],sheets[0],t);paint(contexts[1],sheets[1],t);el('scrub').value=Math.round(age*1000);el('clock').value=Math.round(age*1000)+' ms';}
function tick(now){const dt=Math.min(.1,(now-last)/1000);last=now;if(playing&&sheets){const period=row.loop_seconds?row.loop_seconds*(el('slower').checked?row.duration_scale:1)*3:2.1;age=(age+dt)%period;draw();}requestAnimationFrame(tick);}
el('case').onchange=load;el('light').onchange=load;
el('pause').onclick=()=>{playing=!playing;el('pause').textContent=playing?'Pause':'Lire';};
el('replay').onclick=()=>{age=0;playing=true;last=performance.now();el('pause').textContent='Pause';draw();};
el('scrub').oninput=()=>{playing=false;el('pause').textContent='Lire';age=Number(el('scrub').value)/1000;draw();};
for(const id of ['zone','slower'])el(id).onchange=()=>{notes();draw();};
load();requestAnimationFrame(tick);
'''


def html(data):
    page = previous.HTML.split('<script>')[0]
    page = page.replace('Rythme et lisibilité · ANIM-READ-01', 'Formes naturelles · ANIM-FLOW-01')
    page = page.replace('ANIM-READ-01 · PROPOSITION À VALIDER', 'ANIM-FLOW-01 · NOUVELLE PROPOSITION')
    page = page.replace('Un rythme plus posé, des effets plus lisibles', 'Des effets qui respirent hors des cases')
    page = page.replace('Mêmes dessins, mêmes cases de <strong>64 px</strong>. À gauche le rendu actuel ; à droite une proposition qui laisse davantage de temps pour lire l’action et renforce certains effets discrets.', 'À gauche, les dessins découpés par les cases touchées. À droite, les mêmes dessins entiers : les éclats, flammes et volutes peuvent déborder naturellement sur le sol libre. Les cases de jeu restent de <strong>64 px</strong>.')
    page = page.replace('À gauche, les dessins découpés par les cases touchées. À droite, les mêmes dessins entiers :', 'La version actuelle découpe le dessin selon les cases touchées. La proposition conserve le dessin entier :')
    page = page.replace('</style>', '.comparison{margin-inline:auto}@media(max-width:980px){.comparison{grid-template-columns:448px;width:448px}}</style>')
    page = page.replace('<div class="checks"><label><input type="checkbox" id="phase">Comparer le contraste à phase identique</label><label><input type="checkbox" id="contrast" checked>Appliquer le contraste proposé</label></div>', '<div class="checks"><label><input type="checkbox" id="zone">Montrer les cases touchées</label><label><input type="checkbox" id="slower" checked>Rythme ralenti des deux côtés</label></div>')
    page = page.replace('<h2>Actuel</h2>', '<h2>Découpe actuelle</h2>').replace('<h2>Proposition</h2>', '<h2>Forme naturelle</h2>')
    page = page.replace('Animation proposée, taille native', 'Animation entière proposée, taille native')
    page = page.replace('max="2400"', 'max="2100"')
    page = page.replace('Lecture en boucle avec une pause entre les actions.', 'Même phase et même intensité dans les deux versions pour comparer la forme. Le ralentissement peut être désactivé.')
    page = page.replace('sur une petite fenêtre, faire défiler la comparaison horizontalement', 'sur une petite fenêtre, les deux versions se placent l’une sous l’autre')
    start = page.index('<footer>')
    page = page[:start] + '''<footer><p class="muted">L’animation peut dépasser la zone qui inflige les dégâts. Le repère « cases touchées » montre que cette zone est conservée dans les deux colonnes. Les murs physiques et les zones hors de vue restent protégés.</p><p class="muted">Trois effets et un exemple contre une cloison, à taille native sur deux éclairages de revue. Cette proposition utilise les dessins existants, avec une taille stable, sans découpe par case ni étirement. Elle reste isolée du jeu normal. Les choix précédents de vitesse et de contraste restent en attente de validation.</p></footer>'''
    return page + '<script>' + SCRIPT.replace('__DATA__', json.dumps(data, ensure_ascii=False)) + '</script></body></html>'


def frame(sheet, data, index):
    x = index % data['columns'] * data['frame_width']
    y = index // data['columns'] * data['frame_height']
    return sheet.crop((x, y, x + data['frame_width'], y + data['frame_height'])).convert('RGB')


def main():
    data = json.loads((OUT / 'capture.json').read_text(encoding='utf-8'))
    assert data['review'] == 'ANIM-FLOW-01' and not data['normal_game_activation']
    size = (data['frame_width'] * data['columns'], data['frame_height'] * ((data['frames'] + data['columns'] - 1) // data['columns']))
    checks = []
    for row in data['cases']:
        for light in ('sombre', 'clair'):
            paths = [OUT / f'{row["number"]:02d}-{light}-{panel}.png' for panel in range(2)]
            for path in paths:
                raw = path.with_suffix('.rgba')
                if raw.exists():
                    payload = raw.read_bytes()
                    assert len(payload) == size[0] * size[1] * 4, raw
                    with Image.frombytes('RGBA', size, payload).transpose(Image.Transpose.FLIP_TOP_BOTTOM) as sheet:
                        sheet.save(path, compress_level=1)
            with Image.open(paths[0]) as before, Image.open(paths[1]) as after:
                assert before.size == after.size == size
                rest = frame(before, data, data['frames'] - 1)
                assert ImageChops.difference(rest, frame(after, data, data['frames'] - 1)).getbbox() is None, (row['title'], light, 'rest changed')
                changed = animated = outside_pixels = 0
                area = Image.new('L', (data['frame_width'], data['frame_height']), 255)
                from PIL import ImageDraw
                paint = ImageDraw.Draw(area)
                for x, y in row['affected_cells']:
                    paint.rectangle((x*64, y*64, x*64+63, y*64+63), fill=0)
                for index in range(data['frames']):
                    a, b = frame(before, data, index), frame(after, data, index)
                    changed += ImageChops.difference(a, b).getbbox() is not None
                    effect = ImageChops.difference(b, rest)
                    animated += effect.getbbox() is not None
                    bands = effect.split()
                    visible = ImageChops.lighter(ImageChops.lighter(bands[0], bands[1]), bands[2]).point(lambda v: 255 if v else 0)
                    outside_pixels += ImageChops.multiply(visible, area).histogram()[255]
                    for x, y in row['excluded_visual_cells']:
                        box = (x*64, y*64, x*64+64, y*64+64)
                        assert ImageChops.difference(b.crop(box), rest.crop(box)).getbbox() is None, (row['title'], light, index, x, y, 'obstacle or hidden cell leak')
                assert changed > 0 and animated > 0 and outside_pixels > 0, row['title']
                checks.append({'number':row['number'],'light':light,'changed_phases':changed,'animated_phases':animated,'visual_overhang_pixel_samples':outside_pixels,'excluded_cell_leaks':0,'rest_identical':True})
    DOC.mkdir(parents=True, exist_ok=True)
    (DOC / 'proposal.json').write_text(json.dumps(data, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    (OUT / 'pixel-checks.json').write_text(json.dumps(checks, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    CURRENT.write_text(html(data), encoding='utf-8')
    (OUT / 'controller-check.js').write_text(SCRIPT.replace('__DATA__', json.dumps(data, ensure_ascii=False)), encoding='utf-8')
    for row in data['cases']:
        for light in ('sombre', 'clair'):
            for panel in range(2):
                raw = OUT / f'{row["number"]:02d}-{light}-{panel}.rgba'
                if raw.exists():
                    raw.unlink()
    print(CURRENT)


if __name__ == '__main__':
    main()
