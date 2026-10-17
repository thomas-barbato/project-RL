"""Elite scale revisions 20-21; separate contact shadows; no runtime changes."""
import base64
import hashlib
import html
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from show_npc_monsters_review import conditioned_image
from review_contact_shadows import contact_shadow

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
ART = ROOT/'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT = ROOT/'artifacts/npc-monsters-review-01'


def load(path):
    return json.loads(path.read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')


def main():
    batch = load(REVIEW/'elite-monsters-16.json')
    (OUT/'elite-tiles').mkdir(parents=True, exist_ok=True)
    for e in batch['records']:
        assert sha(ROOT/e['source']) == e['source_sha256']
        art, anchor = conditioned_image(e, 64)
        path = OUT/'elite-tiles'/f"{e['id']}-{e['revision']}-native64.png"
        art.save(path, optimize=True)
        e['native64_export'] = dict(file=path.relative_to(ROOT).as_posix(), cell=64,
            canvas=art.width, foot_anchor=list(anchor), occupied=list(art.getbbox()),
            colors=len({p[:3] for p in art.get_flattened_data() if p[3]}), sha256=sha(path))
    initial = load(REVIEW/'exports.json')['items']
    previous = load(REVIEW/'sector-bosses-11.json')['records']
    current = [e for e in initial if not e['id'].startswith('NPC')]+previous+batch['records']
    shadows = ART/'contact-shadows-14'
    shadows.mkdir(parents=True, exist_ok=True)
    records = []
    by_key = {}
    for e in current:
        meta = e.get('native64_export') or e['exports'][0]
        body = Image.open(ROOT/meta['file']).convert('RGBA')
        assert sha(ROOT/meta['file']) == meta['sha256']
        shadow, contact = contact_shadow(body, meta['foot_anchor'])
        key = e['id']+'-'+e['revision']
        path = shadows/f'{key}-shadow64.png'
        shadow.save(path, optimize=True)
        record = dict(id=e['id'], number=e['number'], revision=e['revision'],
            body_file=meta['file'], body_sha256=meta['sha256'],
            shadow_file=path.relative_to(ROOT).as_posix(), shadow_sha256=sha(path),
            canvas=body.width, foot_anchor=meta['foot_anchor'], contact=contact,
            shadow_visual_approved=False)
        records.append(record)
        by_key[key] = (body, shadow, meta['foot_anchor'])
        if e in batch['records']:
            e['contact_shadow'] = record
    shadow_review = dict(review_id='CONTACT-SHADOWS14', date='2026-10-17',
        user_statement=batch['shadow_request'], status='direction_confirmed_render_for_review',
        current_elite_review='elite-monsters-16.json',
        direction='Une ombre de contact sous tous les monstres, sur un calque séparé sous le corps.',
        implementation='Même forme à bandes, couleurs et opacités que draw_contact_shadow ; largeur adaptée aux appuis visibles.',
        original_character_pngs_modified=False, runtime_connected=False, records=records)
    write(REVIEW/'contact-shadows-14.json', shadow_review)

    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 15)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 19)
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0, 0, 240, 155))
    player = batch['player_reference']
    assert sha(ROOT/player['source']) == player['sha256']
    player_art = Image.open(ROOT/player['source']).convert('RGBA')
    player_shadow, _ = contact_shadow(player_art, player['foot_anchor'])
    sheet = Image.new('RGB', (1120, 650), '#10171c')
    draw = ImageDraw.Draw(sheet)
    draw.text((20, 16), '20–21 · ÉLITES · dessins conservés, gabarits réduits et ombres séparées', font=bold, fill='#e5d2a8')
    draw.text((20, 48), 'Cases de référence 64 px · apparences appréciées, nouveaux gabarits à examiner', font=font, fill='#b4c4c8')
    embedded = []
    cards = []
    for i, e in enumerate(batch['records']):
        x = 15+i*555
        body, shadow, anchor = by_key[e['id']+'-'+e['revision']]
        draw.rectangle((x, 80, x+535, 610), fill='#1a242b', outline='#35434b')
        draw.text((x+12, 94), f"{e['number']:02} · {e['label']} · {e['revision']}", font=bold, fill='#e5d2a8')
        b = body.getbbox()
        draw.text((x+12, 124), f"{e['role']} · {b[2]-b[0]} × {b[3]-b[1]} px", font=font, fill='#b4c4c8')
        for z, top, height, baseline in [(1, 154, 150, 278), (2, 332, 248, 561)]:
            sheet.paste(ground.resize((511, height), Image.Resampling.NEAREST), (x+12, top))
            draw.text((x+22, top+8), 'Taille native · joueur à gauche' if z == 1 else 'Zoom ×2 · mêmes proportions', font=font, fill='#e6dfca')
            for layers, at, center in [([player_shadow, player_art], player['foot_anchor'], x+105), ([shadow, body], anchor, x+360)]:
                for layer in layers:
                    enlarged = layer.resize((layer.width*z, layer.height*z), Image.Resampling.NEAREST)
                    sheet.paste(enlarged, (center-at[0]*z, baseline-at[1]*z), enlarged)
        draw.text((x+18, 586), 'Ombre séparée · aucune retouche du dessin source', font=font, fill='#b4c4c8')
        meta = e['native64_export']
        embedded.append(dict(id=e['id'], meta=meta,
            uri='data:image/png;base64,'+base64.b64encode((ROOT/meta['file']).read_bytes()).decode(),
            shadow='data:image/png;base64,'+base64.b64encode((ROOT/e['contact_shadow']['shadow_file']).read_bytes()).decode()))
        cards.append(f'<article><h2>{e["number"]} · {html.escape(e["label"])}</h2><canvas data-id="{e["id"]}"></canvas></article>')
    draw.text((20, 626), 'Libellés provisoires · gabarits et ombres en revue · aucun raccord au jeu', font=font, fill='#b4c4c8')
    sheet.save(OUT/'elites-20-21.png', optimize=True)

    overview = Image.new('RGB', (1440, 50+320*((len(records)+3)//4)), '#10171c')
    ov = ImageDraw.Draw(overview)
    ov.text((20, 14), 'Ombres de contact · tous les monstres courants · PNG des corps inchangés', font=bold, fill='#e5d2a8')
    for i, r in enumerate(records):
        x, y = (i%4)*360, 50+(i//4)*320
        ov.text((x+12, y+6), f"{r['number']:02} · {r['id']} · {r['revision']}", font=font, fill='#e5d2a8')
        overview.paste(ground.resize((344, 275), Image.Resampling.NEAREST), (x+8, y+34))
        body, shadow, anchor = by_key[r['id']+'-'+r['revision']]
        for layer in [shadow, body]:
            overview.paste(layer, (x+180-anchor[0], y+300-anchor[1]), layer)
    overview.save(OUT/'ombres-monstres.png', optimize=True)
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Boss 20–21 et ombres</title><style>body{background:#10171c;color:#deded4;font:16px/1.5 system-ui;margin:24px}h1{color:#e5d2a8}section{display:flex;flex-wrap:wrap;gap:20px}article{background:#1a242b;padding:16px;border:1px solid #35434b}canvas{image-rendering:pixelated}select{font:inherit;padding:6px}</style><h1>Boss de secteur · propositions 20–21</h1><p>Apparences, gabarits et ombres à examiner ; aucun effet de combat défini.</p><label>Zoom <select id="zoom"><option value="1">1× · natif</option><option value="2">2×</option></select></label> <label><input type="checkbox" id="shadows" checked> Ombres séparées</label><section>'''+''.join(cards)+'''</section><script>const data='''+json.dumps(embedded, ensure_ascii=False)+''';const cache=new Map();async function render(){const z=+document.querySelector('#zoom').value;for(const c of document.querySelectorAll('canvas')){const e=data.find(v=>v.id===c.dataset.id),m=e.meta,b=m.occupied;c.width=Math.max(300,(b[2]-b[0])*z+64);c.height=(b[3]-b[1])*z+96;const ctx=c.getContext('2d');ctx.imageSmoothingEnabled=false;const foot=c.height-30;for(const kind of ['shadow','uri']){if(kind==='shadow'&&!document.querySelector('#shadows').checked)continue;const key=e.id+kind;let im=cache.get(key);if(!im){im=new Image;im.src=e[kind];await im.decode();cache.set(key,im)}ctx.drawImage(im,c.width/2-m.foot_anchor[0]*z,foot-m.foot_anchor[1]*z,im.width*z,im.height*z)}}}for(const id of ['zoom','shadows'])document.querySelector('#'+id).addEventListener('change',render);render();</script></html>'''
    page = page.replace('Boss 20–21 et ombres', 'Élites 20–21 et ombres').replace('Boss de secteur · propositions 20–21', 'Élites · gabarits proposés 20–21').replace('Apparences, gabarits et ombres à examiner', 'Dessins conservés ; gabarits réduits et ombres à examiner')
    (OUT/'elites-20-21.html').write_text(page, encoding='utf-8')
    write(REVIEW/'elite-monsters-16.json', batch)
    print(f"Deux élites réexportées ; {len(records)} ombres séparées ; aucun dessin source précédent modifié.")


if __name__ == '__main__':
    main()
