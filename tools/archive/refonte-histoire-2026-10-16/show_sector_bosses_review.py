"""Native large-boss proposals; keeps the initial accepted gallery unchanged."""
import base64
import hashlib
import html
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from show_npc_monsters_review import conditioned_image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT = ROOT/'artifacts/npc-monsters-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    batch = json.loads((REVIEW/'sector-bosses-11.json').read_text(encoding='utf-8'))
    tiles = OUT/'boss-tiles'
    tiles.mkdir(parents=True, exist_ok=True)
    entries = []
    for entry in batch['records']:
        assert sha(ROOT/entry['source']) == entry['source_sha256']
        art, anchor = conditioned_image(entry, 64)
        path = tiles/f"{entry['id']}-{entry['revision']}-native64.png"
        frozen = entry.get('accepted_exports', {}).get('64')
        if frozen:
            path.write_bytes((ROOT/frozen['file']).read_bytes())
        else:
            art.save(path, optimize=True)
        colors = {p[:3] for p in art.get_flattened_data() if p[3]}
        assert len(colors) <= entry['palette_limit']
        assert set(art.getchannel('A').get_flattened_data()) == {0, 255}
        meta = dict(file=path.relative_to(ROOT).as_posix(), cell=64,
                    canvas=art.width, foot_anchor=list(anchor),
                    occupied=list(art.getbbox()), colors=len(colors), sha256=sha(path))
        entries.append({**entry, 'native64_export': meta})
    player = batch['player_reference']
    assert sha(ROOT/player['source']) == player['sha256']
    player_art = Image.open(ROOT/player['source']).convert('RGBA')
    player_height = player_art.getbbox()[3]-player_art.getbbox()[1]
    sheet = Image.new('RGB', (1240, 720), '#10171c')
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 16)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 21)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 14)
    approved = all(e.get('visual_approved') and e.get('scale_approved') for e in entries)
    status = 'apparences et gabarits validés' if approved else 'gabarits en revue'
    draw.text((24, 18), 'BOSS DE SECTEUR · 18 ET 19 · ' + status, font=bold, fill='#e5d2a8')
    draw.text((24, 54), 'Taille native : 1 pixel du fichier = 1 pixel de la planche · même échelle pour tous', font=font, fill='#b4c4c8')
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0, 0, 240, 155))
    sheet.paste(ground.resize((1192, 410), Image.Resampling.NEAREST), (24, 112))
    positions = [170, 570, 1010]
    baseline = 492
    draw.rectangle((positions[0]-32, baseline-64, positions[0]+32, baseline), outline='#d8c28f')
    sheet.paste(player_art, (positions[0]-player['foot_anchor'][0], baseline-player['foot_anchor'][1]), player_art)
    for e, center in zip(entries, positions[1:]):
        art = Image.open(ROOT/e['native64_export']['file']).convert('RGBA')
        anchor = e['native64_export']['foot_anchor']
        sheet.paste(art, (center-anchor[0], baseline-anchor[1]), art)
    labels = [('Joueur · tenue de référence', f'Corps : {player_height} px · case : 64 px')]
    for e in entries:
        box = e['native64_export']['occupied']
        state = 'validé' if e.get('visual_approved') and e.get('scale_approved') else 'proposé'
        labels.append((f"{e['number']:02} · {e['label']} · {e['revision']}", f"Dessin : {box[2]-box[0]} × {box[3]-box[1]} px · {state}"))
    for center, (title, detail) in zip(positions, labels):
        draw.text((center, 548), title, anchor='mm', font=font, fill='#e5d2a8')
        draw.text((center, 578), detail, anchor='mm', font=small, fill='#b4c4c8')
    previous_height = entries[1].get('previous_approved_height64')
    comparison = f"19 : essai à {entries[1]['maximum_body_height64']} px, au lieu de {previous_height} px · dessin conservé" if previous_height else '18 : même source acceptée, nouvel export direct plus grand · 19 : corps haut et bras chirurgicaux'
    draw.text((24, 624), comparison, font=font, fill='#b4c4c8')
    notice = 'Apparences et tailles validées · noms et capacités séparés · aucune collision ou intégration définie' if approved else 'Gabarit du 18 validé · nouveau gabarit du 19 à examiner · aucune collision ou intégration définie'
    draw.text((24, 655), notice, font=font, fill='#b4c4c8')
    draw.text((24, 686), 'Montage sur sol de référence · un seul grand boss par secteur, avec des gabarits différents', font=small, fill='#b4c4c8')
    sheet.save(OUT/'boss-secteurs-18-19.png', optimize=True)

    embedded = []
    cards = []
    for e in entries:
        meta = e['native64_export']
        embedded.append(dict(id=e['id'], meta=meta, uri='data:image/png;base64,'+base64.b64encode((ROOT/meta['file']).read_bytes()).decode()))
        cards.append(f'<article><h2>{e["number"]:02} · {html.escape(e["label"])} · {e["revision"]}</h2><p>{html.escape(e["role"])}</p><canvas data-id="{e["id"]}"></canvas></article>')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><title>Boss de secteur — gabarits</title>
<style>body{background:#10171c;color:#deded4;font:16px/1.5 system-ui;margin:24px}h1{color:#e5d2a8}section{display:flex;gap:24px;flex-wrap:wrap}article{padding:18px;background:#1a242b;border:1px solid #35434b}canvas{display:block;image-rendering:pixelated}select{font:inherit;background:#24333b;color:#efdfbf;padding:6px}a{color:#dcc29a}img{max-width:100%}</style>
<h1>Boss de secteur · 18 et 19 · __STATUS__</h1><p>La case de référence reste de 64 px. Les boss ont des tailles visuelles propres, nettement supérieures. __SIZES__</p>
<label>Zoom <select id="zoom"><option value="1">1× · taille native</option><option value="2">2× · sans lissage</option></select></label><section>''' + ''.join(cards) + '''</section><p>__DETAILS__ Aucune capacité ni collision définie.</p>
<script>const data=''' + json.dumps(embedded, ensure_ascii=False) + ''';
const images=new Map();async function render(){const z=+document.querySelector('#zoom').value;
for(const c of document.querySelectorAll('canvas')){const e=data.find(v=>v.id===c.dataset.id),m=e.meta,b=m.occupied;c.width=Math.max(280,(b[2]-b[0])*z+64);c.height=(b[3]-b[1])*z+80;
let im=images.get(e.id);if(!im){im=new Image;im.src=e.uri;await im.decode();images.set(e.id,im)}
const ctx=c.getContext('2d');ctx.imageSmoothingEnabled=false;const foot=c.height-24;
ctx.strokeStyle='#6c716c';ctx.strokeRect(c.width/2-32*z,foot-64*z,64*z,64*z);
ctx.drawImage(im,c.width/2-m.foot_anchor[0]*z,foot-m.foot_anchor[1]*z,im.width*z,im.height*z)}}
document.querySelector('#zoom').addEventListener('change',render);render();</script></html>'''
    details = ' '.join(f"{e['number']} : gabarit {'validé' if e.get('visual_approved') and e.get('scale_approved') else 'proposé'}, {e['maximum_body_height64']} px de haut." for e in entries)
    page = page.replace('__STATUS__', status).replace('__SIZES__', 'Les apparences et les dimensions présentées sont validées.' if approved else 'Le nouveau gabarit du 19 reste à valider ; celui du 18 est validé.').replace('__DETAILS__', details)
    (OUT/'boss-secteurs.html').write_text(page, encoding='utf-8')
    batch['records'] = entries
    batch['preview'] = 'artifacts/npc-monsters-review-01/boss-secteurs-18-19.png'
    (REVIEW/'sector-bosses-11.json').write_text(json.dumps(batch, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    print('Deux boss exportés ; PNG acceptés réutilisés à l’identique ; galerie initiale et ressources du jeu inchangées.')


if __name__ == '__main__':
    main()
