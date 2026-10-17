"""Condition and show NPC-MONSTERS-01 proposals; never writes runtime atlases.

Original drawings come from image_gen. This script only trims transparent
padding, exports each resolution directly, indexes without dithering and lays
out the review. It does not paint, regenerate anatomy or approve candidates.
"""
import base64
import hashlib
import html
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT = ROOT / 'artifacts/npc-monsters-review-01'


def png_url(path):
    return 'data:image/png;base64,' + base64.b64encode(path.read_bytes()).decode()


def conditioned_image(entry, cell):
    assert cell == 64, 'Only native64 review exports are maintained.'
    frozen = entry.get('accepted_exports', {}).get(str(cell))
    if frozen:
        source = ROOT / entry['source']
        assert hashlib.sha256(source.read_bytes()).hexdigest() == entry['accepted_source_sha256']
        path = ROOT / frozen['file']
        assert hashlib.sha256(path.read_bytes()).hexdigest() == frozen['sha256']
        return Image.open(path).convert('RGBA'), frozen['foot_anchor']
    original = Image.open(ROOT / entry['source']).convert('RGBA')
    # Discard near-transparent output noise before measuring the solid actor.
    original.putalpha(original.getchannel('A').point(lambda a: 255 if a >= 128 else 0))
    bbox = original.getbbox()
    assert bbox, entry['id']
    crop = original.crop(bbox)
    ratio = min(entry['maximum_body_height64'] * cell / 64 / crop.height,
                entry['maximum_body_width64'] * cell / 64 / crop.width)
    size = (max(1, round(crop.width * ratio)), max(1, round(crop.height * ratio)))
    body = crop.resize(size, Image.Resampling.LANCZOS)
    alpha = body.getchannel('A').point(lambda a: 255 if a >= 128 else 0)
    limit = entry.get('palette_limit', 24)
    if entry.get('preserve_cold_blue_panels'):
        # Keep the source's cold material readable on very small flesh bodies.
        pixels = [p[:3] for p in body.get_flattened_data() if p[3] >= 128]
        cold = lambda p: p[2] > p[0]*1.1 and p[2] > p[1]*1.03
        groups = [([p for p in pixels if not cold(p)], limit-3),
                  ([p for p in pixels if cold(p)], 3)]
        colors = []
        for samples, count in groups:
            assert samples, entry['id']
            sample = Image.new('RGB', (len(samples), 1))
            sample.putdata(samples)
            reduced = sample.quantize(count, method=Image.Quantize.MEDIANCUT,
                                      dither=Image.Dither.NONE)
            colors += reduced.getpalette()[:count*3]
        palette = Image.new('P', (1, 1))
        palette.putpalette(colors + colors[:3]*(256-limit))
        body = body.convert('RGB').quantize(palette=palette, dither=Image.Dither.NONE).convert('RGBA')
    elif entry.get('preserve_mask_and_violet_focus'):
        # Reserve source-derived colors for the small but meaningful mask/focus.
        # A global quantizer otherwise loses them to the much larger cloth areas.
        pixels = [p[:3] for p in body.get_flattened_data() if p[3] >= 128]
        violet = lambda p: p[2] > p[0]+10 and p[2] > p[1]+15
        ivory = lambda p: min(p) > 145 and max(p)-min(p) < 45
        groups = [([p for p in pixels if not violet(p) and not ivory(p)], limit-4),
                  ([p for p in pixels if violet(p)], 2),
                  ([p for p in pixels if ivory(p)], 2)]
        colors = []
        for samples, count in groups:
            assert samples, entry['id']
            sample = Image.new('RGB', (len(samples), 1))
            sample.putdata(samples)
            reduced = sample.quantize(count, method=Image.Quantize.MEDIANCUT,
                                      dither=Image.Dither.NONE)
            colors += reduced.getpalette()[:count*3]
        palette = Image.new('P', (1, 1))
        palette.putpalette(colors + colors[:3]*(256-limit))
        body = body.convert('RGB').quantize(palette=palette, dither=Image.Dither.NONE).convert('RGBA')
    else:
        body = body.convert('RGB').quantize(limit, method=Image.Quantize.MEDIANCUT,
                                          dither=Image.Dither.NONE).convert('RGBA')
    body.putalpha(alpha)
    side = cell * entry.get('canvas_cells', 4 if entry['id'] == 'MON-05' else 2)
    ground_cell = entry.get('ground_cell', 3 if entry['id'] == 'MON-05' else 1.5)
    anchor = (side // 2, round(cell * ground_cell))
    offset = (anchor[0] - body.width // 2, anchor[1] - body.height)
    assert min(offset) >= 0
    image = Image.new('RGBA', (side, side))
    image.alpha_composite(body, offset)
    return image, anchor


def export(entry, cell):
    image, anchor = conditioned_image(entry, cell)
    side = image.width
    path = OUT / 'tiles' / f"{entry['id']}-cell{cell}-canvas{side}.png"
    frozen = entry.get('accepted_exports', {}).get(str(cell))
    if frozen:
        path.write_bytes((ROOT / frozen['file']).read_bytes())
    else:
        image.save(path, optimize=True)
    colors = {p[:3] for p in image.get_flattened_data() if p[3]}
    assert len(colors) <= entry.get('palette_limit', 24)
    assert set(image.getchannel('A').get_flattened_data()) == {0, 255}
    return image, dict(file=path.relative_to(ROOT).as_posix(), cell=cell,
                      canvas=side, foot_anchor=list(anchor), occupied=list(image.getbbox()),
                      colors=len(colors), sha256=hashlib.sha256(path.read_bytes()).hexdigest())


def mission_review(entry):
    """Native64, previous refused design and ground samples; no game capture."""
    sheet = Image.new('RGB', (960, 560), '#152127')
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 19)
    draw.text((20, 16), '01 · Responsable des missions — ' + entry['revision'], font=bold, fill='#e5d2a8')
    status = 'Apparence validée' if entry.get('visual_approved') else 'Nouvelle proposition'
    draw.text((20, 47), status + ' : uniforme court, casque à visière, arme au holster.', font=font, fill='#b4c4c8')
    previous = dict(entry, accepted_exports={}, source='assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/NPC-01-r02.png')
    for x, source, cell, zoom, label in [
        (100, entry, 64, 1, '64 px · natif'),
        (285, entry, 64, 2, '64 px · zoom ×2'),
        (510, entry, 64, 3, '64 px · zoom ×3'),
        (805, previous, 64, 1, 'Ancien r02 · refusé')]:
        art, anchor = conditioned_image(source, cell)
        art = art.resize((art.width*zoom, art.height*zoom), Image.Resampling.NEAREST)
        sheet.paste(art, (x-anchor[0]*zoom, 310-anchor[1]*zoom), art)
        draw.text((x-72, 82), label, font=font, fill='#b4c4c8')
    ground_path = ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png'
    ground = Image.open(ground_path).convert('RGB').crop((0, 0, 240, 155))
    art, anchor = conditioned_image(entry, 64)
    for i, (label, color) in enumerate([('Sol clair', '#aea994'), ('Sol sombre', '#111719'), ('Sol texturé', None)]):
        x = 25+i*310
        panel = Image.new('RGB', (290, 155), color) if color else ground.resize((290, 155), Image.Resampling.NEAREST)
        # This is a layout sample, not a runtime screenshot or a resized sprite.
        sheet.paste(panel, (x, 360))
        sheet.paste(art, (x+145-anchor[0], 495-anchor[1]), art)
        draw.text((x+10, 370), label, font=font, fill='#10171c' if i == 0 else '#e6dfca')
    draw.text((25, 530), 'Montage de revue · même taille humaine · rendu 64 px uniquement · sans intégration.', font=font, fill='#b4c4c8')
    sheet.save(OUT/'NPC-01-revue.png', optimize=True)


def trio_review(items, ids, title, filename, zoom=3):
    entries = [e for e in items if e['id'] in ids]
    if not entries:
        return
    service_review = all(e['id'].startswith('NPC') for e in entries)
    sheet = Image.new('RGB', (1230, 650), '#10171c')
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 19)
    draw.text((20, 15), title, font=bold, fill='#e5d2a8')
    scope = 'noms et services à examiner séparément' if service_review else 'noms et capacités à examiner séparément'
    draw.text((20, 45), f'64 px natifs et zoom ×{zoom} sans lissage · {scope}', font=font, fill='#b4c4c8')
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0, 0, 240, 155))
    for i, e in enumerate(entries):
        x = 15+i*405
        draw.rectangle((x, 80, x+390, 620), fill='#1a242b', outline='#35434b')
        draw.text((x+12, 94), f"{e['number']:02} · {e['label']}", font=bold, fill='#e5d2a8')
        draw.text((x+12, 124), e['role'], font=font, fill='#b4c4c8')
        art = Image.open(ROOT/e['exports'][0]['file']).convert('RGBA')
        anchor = e['exports'][0]['foot_anchor']
        for center, scale in [(x+73, 1), (x+250, zoom)]:
            sprite = art.resize((art.width*scale, art.height*scale), Image.Resampling.NEAREST)
            sheet.paste(sprite, (center-anchor[0]*scale, 365-anchor[1]*scale), sprite)
        draw.text((x+32, 379), 'Taille native', font=font, fill='#b4c4c8')
        draw.text((x+205, 379), f'Zoom ×{zoom}', font=font, fill='#b4c4c8')
        sheet.paste(ground.resize((366, 180), Image.Resampling.NEAREST), (x+12, 420))
        sheet.paste(art, (x+195-anchor[0], 580-anchor[1]), art)
    notice = 'Montages sur sol de référence · aucun nouveau PNJ, service ou effet intégré au jeu.' if service_review else 'Montages sur sol de référence · aucune nouvelle créature ou capacité intégrée au jeu.'
    draw.text((20, 627), notice, font=font, fill='#b4c4c8')
    sheet.save(OUT/filename, optimize=True)


def depths_review(items):
    """Two large creatures and the accepted human scale reference."""
    entries = [e for e in items if e['id'] in ('MON-04', 'MON-05')]
    if len(entries) != 2:
        return
    sheet = Image.new('RGB', (1230, 760), '#10171c')
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 19)
    status = 'apparences validées' if all(e.get('visual_approved') for e in entries) else 'validation partielle' if any(e.get('visual_approved') for e in entries) else 'révisions proposées'
    draw.text((20, 15), '08–09 · Démon des profondeurs et créature du plan glacé · ' + status, font=bold, fill='#e5d2a8')
    draw.text((20, 45), 'Case de référence 64 px · taille native et zoom ×2 · noms et capacités à examiner séparément', font=font, fill='#b4c4c8')
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0, 0, 240, 155))
    human = next(e for e in items if e['id'] == 'NPC-01')
    human_art = Image.open(ROOT/human['exports'][0]['file']).convert('RGBA')
    human_anchor = human['exports'][0]['foot_anchor']
    for i, e in enumerate(entries):
        x = 15+i*605
        draw.rectangle((x, 80, x+590, 710), fill='#1a242b', outline='#35434b')
        draw.text((x+12, 94), f"{e['number']:02} · {e['label']} · {e['revision']}", font=bold, fill='#e5d2a8')
        draw.text((x+12, 124), e['role'], font=font, fill='#b4c4c8')
        art = Image.open(ROOT/e['exports'][0]['file']).convert('RGBA')
        anchor = e['exports'][0]['foot_anchor']
        for center, scale in [(x+125, 1), (x+400, 2)]:
            sprite = art.resize((art.width*scale, art.height*scale), Image.Resampling.NEAREST)
            sheet.paste(sprite, (center-anchor[0]*scale, 430-anchor[1]*scale), sprite)
        draw.text((x+65, 444), 'Taille native', font=font, fill='#b4c4c8')
        draw.text((x+365, 444), 'Zoom ×2', font=font, fill='#b4c4c8')
        sheet.paste(ground.resize((566, 210), Image.Resampling.NEAREST), (x+12, 480))
        draw.text((x+24, 490), 'Même échelle · humain de référence à gauche', font=font, fill='#e6dfca')
        sheet.paste(human_art, (x+125-human_anchor[0], 680-human_anchor[1]), human_art)
        sheet.paste(art, (x+400-anchor[0], 680-anchor[1]), art)
    draw.text((20, 730), 'Montages de revue · le 09 est une créature du plan glacé, pas un boss de secteur · aucun raccord au jeu.', font=font, fill='#b4c4c8')
    sheet.save(OUT/'profondeurs-glace-08-09.png', optimize=True)


def readability_review(items):
    entries = [e for e in items if e['id'] in ('MON-06', 'MON-07', 'MON-08')]
    sheet = Image.new('RGB', (1230, 670), '#10171c')
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 19)
    draw.text((20, 15), '10–12 · Même identité, dessin simplifié · comparaison de revue', font=bold, fill='#e5d2a8')
    draw.text((20, 45), 'Chaque ligne : taille native 64 px à gauche, zoom ×2 à droite. Silhouettes à examiner.', font=font, fill='#b4c4c8')
    for i, e in enumerate(entries):
        x = 15+i*405
        draw.rectangle((x, 80, x+390, 640), fill='#1a242b', outline='#35434b')
        draw.text((x+12, 94), f"{e['number']:02} · {e['label']}", font=bold, fill='#e5d2a8')
        old = dict(e, accepted_exports={}, source=f"assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/{e['id']}-r01.png", palette_limit=24)
        current_label = 'Version simplifiée validée · r02' if e.get('visual_approved') else 'Proposition simplifiée · r02'
        for source, foot, label in [(old, 335, 'Avant · r01'), (e, 605, current_label)]:
            draw.text((x+12, foot-200), label, font=font, fill='#b4c4c8')
            art, anchor = conditioned_image(source, 64)
            for center, zoom in [(x+78, 1), (x+260, 2)]:
                sprite = art.resize((art.width*zoom, art.height*zoom), Image.Resampling.NEAREST)
                sheet.paste(sprite, (center-anchor[0]*zoom, foot-anchor[1]*zoom), sprite)
    draw.text((20, 647), 'Anciens dessins conservés · moins de textures et de traits internes · aucune intégration au jeu.', font=font, fill='#b4c4c8')
    sheet.save(OUT/'comparaison-lisibilite.png', optimize=True)


def main():
    data = json.loads((REVIEW / 'review.json').read_text(encoding='utf-8'))
    assert data['status'] in ('proposal_not_integrated', 'partially_approved_not_integrated')
    (OUT / 'tiles').mkdir(parents=True, exist_ok=True)
    items = []
    for entry in data['records']:
        a, meta64 = export(entry, 64)
        items.append({**entry, 'exports': [meta64],
                      'source_sha256': hashlib.sha256((ROOT / entry['source']).read_bytes()).hexdigest(),
                      'uri64': png_url(ROOT / meta64['file'])})

    # Each native view uses one image pixel per sheet pixel. Zoom is integer x2.
    sheet = Image.new('RGB', (1410, 90+340*((len(items)+2)//3)), '#10171c')
    draw = ImageDraw.Draw(sheet)
    font_path = Path('C:/Windows/Fonts/segoeui.ttf')
    bold_path = Path('C:/Windows/Fonts/segoeuib.ttf')
    font = ImageFont.truetype(str(font_path), 16)
    bold = ImageFont.truetype(str(bold_path), 18)
    small = ImageFont.truetype(str(font_path), 14)
    draw.text((22, 15), 'PNJ ET MONSTRES / REVUE COURANTE · 64 PX', font=bold, fill='#e6dfca')
    draw.text((22, 44), 'À gauche : affichage 64 px. À droite : zoom ×2. Les gabarits relatifs sont conservés.', font=font, fill='#a8b9bc')
    for i, entry in enumerate(items):
        x, y = 20 + i % 3 * 465, 82 + i // 3 * 340
        draw.rounded_rectangle((x, y, x + 450, y + 326), radius=5, fill='#1a242b', outline='#35434b')
        draw.text((x + 14, y + 10), f"{entry['number']:02} · {entry['label']}", font=bold, fill='#e5d2a8')
        draw.text((x + 14, y + 37), entry['depth'], font=small, fill='#a8b9bc')
        source = Image.open(ROOT / entry['exports'][0]['file']).convert('RGBA')
        foot = entry['exports'][0]['foot_anchor']
        for center, zoom in [(x + 100, 1), (x + 305, 2)]:
            art = source.resize((source.width * zoom, source.height * zoom), Image.Resampling.NEAREST)
            sheet.paste(art, (center - foot[0] * zoom, y + 305 - foot[1] * zoom), art)
    sheet.save(OUT / 'planche-01.png', optimize=True)
    mission_review(data['records'][0])
    depths_review(items)
    service_ids = ('NPC-02', 'NPC-03', 'NPC-04')
    service_entries = [e for e in items if e['id'] in service_ids]
    service_status = 'apparences r02 validées' if len(service_entries) == len(service_ids) and all(e.get('visual_approved') for e in service_entries) else 'révisions r02 proposées'
    trio_review(items, service_ids, '02–04 · PNJ de services · ' + service_status, 'services-02-04.png')
    hybrid_ids = ('MON-01', 'MON-02', 'MON-03')
    hybrid_entries = [e for e in items if e['id'] in hybrid_ids]
    hybrid_status = 'apparences r02 validées' if len(hybrid_entries) == len(hybrid_ids) and all(e.get('visual_approved') for e in hybrid_entries) else 'révisions r02 proposées'
    trio_review(items, hybrid_ids, '05–07 · Possédé et hybrides · ' + hybrid_status, 'possedes-hybrides-05-07.png', zoom=2)
    trio_review(items, ('MON-06', 'MON-07', 'MON-08'), '10–12 · Hybrides simplifiés · rencontres ultérieures', 'monstres-10-12.png')
    trio_review(items, ('CULT-01', 'CULT-02', 'CULT-03'), '13–15 · Cultistes humains · premières rencontres proposées', 'cultistes-13-15.png')
    elite_ids = ('ELITE-01', 'ELITE-02', 'BOSS-01')
    elite_entries = [e for e in items if e['id'] in elite_ids]
    elite_status = 'apparences validées' if len(elite_entries) == len(elite_ids) and all(e.get('visual_approved') for e in elite_entries) else 'propositions'
    trio_review(items, elite_ids, '16–18 · Deux élites humaines et un premier boss · ' + elite_status, 'elites-boss-16-18.png', zoom=2)
    readability_review(items)

    cards = []
    for e in items:
        cards.append(f'''<article data-kind="{'npc' if e['id'].startswith('NPC') else 'monster'}" data-group="{'cultist' if e['id'].startswith('CULT') else 'other'}">
          <h2><span>{e['number']:02}</span> {html.escape(e['label'])}</h2>
          <p class="role">{html.escape(e['role'])} · {html.escape(e['depth'])}</p>
          <canvas width="280" height="296" data-id="{e['id']}" aria-label="{html.escape(e['label'])}"></canvas>
          <p class="caption">{e['id']} · {'apparence validée' if e.get('visual_approved') else 'apparence proposée'} · {'gabarit de boss' if e['id'] == 'BOSS-01' else 'case de référence : 64 px'}</p>
          <details><summary>Voir le dessin source</summary><img class="source" src="{png_url(ROOT / e['source'])}" alt="Source {e['id']}"></details>
        </article>''')
    embedded = [{k: e[k] for k in ['id', 'uri64', 'exports']} for e in items]
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>PNJ et monstres — première revue</title>
<style>
*{box-sizing:border-box}body{margin:0;background:#10171c;color:#deded4;font:16px/1.5 system-ui,sans-serif}main{max-width:1460px;margin:auto;padding:28px}h1{font-size:28px;margin:0 0 10px}header p{max-width:1000px;color:#aab9bd}nav{display:flex;gap:12px;align-items:center;flex-wrap:wrap;position:sticky;top:0;background:#10171cee;padding:12px 0;z-index:2}label{display:flex;gap:8px;align-items:center}select,button{font:inherit;background:#24333b;color:#efdfbf;border:1px solid #59646a;padding:7px 12px;border-radius:4px}section{display:grid;grid-template-columns:repeat(auto-fit,minmax(300px,1fr));gap:16px}article{background:#19242b;border:1px solid #36444b;padding:14px;border-radius:6px;min-width:0}h2{font-size:18px;margin:0;min-height:28px}h2 span{color:#d9ad70}.role{font-size:13px;min-height:38px;color:#a8bbbd}canvas{display:block;width:280px;height:296px;margin:auto;background:linear-gradient(#1c2931,#121c23);image-rendering:pixelated}.caption{font-size:12px;color:#a9b9bc}.source{display:block;max-width:100%;max-height:700px;margin:auto}summary{cursor:pointer;color:#dcc29a}footer{margin-top:24px;max-width:1100px;color:#aab9bd}article[hidden]{display:none}
</style><main><header><h1>PNJ et monstres · Première revue</h1>
<p>Les premières rencontres sont des cultistes humains ; les possédés et hybrides apparaissent ensuite. Rendu 64 px uniquement. Les apparences 01–07 et 09–18 sont validées ; le 08 reste en revue. Le 09 est une créature du plan glacé, pas un boss de secteur. Les numéros sont stables et ne donnent pas l'ordre des rencontres. Les intitulés sont provisoires.</p></header>
<nav><label>Afficher <select id="filter"><option value="all">Tous les candidats</option><option value="npc">PNJ 01–04</option><option value="monster">Ennemis 05–18</option><option value="cultist">Cultistes du départ 13–15</option></select></label>
<label>Échelle <select id="zoom"><option value="1">1× · pixels du fichier</option><option value="2">2× · zoom sans lissage</option></select></label></nav>
<section>''' + '\n'.join(cards) + '''</section>
<footer><p>Les exports 64 px proviennent directement des grands dessins sources. La variante 32 px est abandonnée. Les gabarits relatifs sont conservés : le coureur est bas, l’élite plus large et le boss plus massif.</p>
<p>Revue des apparences uniquement. Le responsable 01 est une étude de pose armée ; les PNJ 02–04 sont des tenues avec accessoires de métier dont les apparences r02 sont validées. Les couches séparées de bras, d’armes et de doigts, les cadavres et les animations restent à préparer. Aucun comportement ni nom définitif n’est fixé par cette planche.</p></footer>
<script>const data=''' + json.dumps(embedded, ensure_ascii=False) + ''';
const images=new Map();
async function render(){const cell=64,zoom=+document.querySelector('#zoom').value;
for(const c of document.querySelectorAll('canvas')){const e=data.find(v=>v.id===c.dataset.id),meta=e.exports.find(v=>v.cell===cell),key=e.id+'-'+cell;
let im=images.get(key);if(!im){im=new Image;im.src=e.uri64;await im.decode();images.set(key,im)}
const ctx=c.getContext('2d');ctx.clearRect(0,0,c.width,c.height);ctx.imageSmoothingEnabled=false;
const anchor=meta.foot_anchor;ctx.strokeStyle='#46575c';ctx.setLineDash([3,4]);ctx.strokeRect(140-cell*zoom/2,276-cell*zoom,cell*zoom,cell*zoom);ctx.setLineDash([]);
ctx.drawImage(im,140-anchor[0]*zoom,276-anchor[1]*zoom,im.width*zoom,im.height*zoom);}}
document.querySelector('#zoom').addEventListener('change',render);
document.querySelector('#filter').addEventListener('change',e=>{for(const a of document.querySelectorAll('article'))a.hidden=e.target.value==='cultist'?a.dataset.group!=='cultist':e.target.value!=='all'&&a.dataset.kind!==e.target.value});render();
</script></main></html>'''
    (OUT / 'apercu.html').write_text(page, encoding='utf-8')
    validation = dict(review_id=data['review_id'], status=data['status'],
                      candidate_count=len(items), native_export_count=len(items),
                      palette_limit=24, solid_alpha='binary', resolutions_direct_from_source=True,
                      runtime_code_changed=False, runtime_atlases_changed=False,
                      items=[{k:v for k,v in e.items() if not k.startswith('uri') and k not in ('prompt','framing_edit_prompt')} for e in items])
    (REVIEW / 'exports.json').write_text(json.dumps(validation, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print(f"{len(items)} candidates, {len(items)} checked native64 exports. Preview: {OUT / 'apercu.html'}")


if __name__ == '__main__':
    main()
