"""Review direct exports from existing detailed art; never modify runtime banks.

One reusable HTML contains its icon atlases. Worn compositions reference the
preserved source art instead of duplicating hundreds of PNGs under artifacts/.
"""
from pathlib import Path
import base64
import hashlib
import html
import io
import json
import re

from PIL import Image, ImageDraw, ImageFont
import numpy as np

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT / 'docs/reviews/refonte-agent-2026-10-06/equipment-native64-01'
OUTPUT = ROOT / 'artifacts/apercu-equipement64.html'
SOURCES = ROOT / 'assets/source-art/refonte-agent-2026-10-06'
SPRITES = ROOT / 'assets/sprites'
RETIRED = {
    'core:integrity_blade', 'core:needle_launcher', 'core:flamethrower',
    'core:composite_carapace', 'core:brigandine_cognefer',
    # Explicitly rejected by the player; not restored by this visual review.
    'core:veste_matelassee',
}
FLAT = {'core:tenue_assaut': 0, 'core:tenue_technique': 1,
        'core:tenue_traque': 2, 'core:tenue_rituelle': 3,
        'core:tenue_metamorphe': 4, 'core:patched_plating': 8,
        'core:veste_de_membranes': 9, 'core:veste_de_peau_seconde': 10,
        'core:veste_de_veille': 11}


def read(path):
    return json.loads(path.read_text('utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def cell(sheet, index, side):
    x, y = index % 16 * side, index // 16 * side
    return sheet.crop((x, y, x + side, y + side))


def data_uri(image):
    stream = io.BytesIO()
    image.save(stream, format='PNG', optimize=True)
    return 'data:image/png;base64,' + base64.b64encode(stream.getvalue()).decode()


def atlas(images):
    result = Image.new('RGBA', (1024, ((len(images) + 15) // 16) * 64))
    for index, image in enumerate(images):
        assert image.size == (64, 64)
        result.alpha_composite(image, (index % 16 * 64, index // 16 * 64))
    return data_uri(result)


def fitted(image):
    """Downsample detailed source directly, with no intermediate 32px image."""
    cut = image.crop(image.getbbox())
    assert min(cut.size) > 32, cut.size
    cut.thumbnail((60, 60), Image.Resampling.LANCZOS)
    result = Image.new('RGBA', (64, 64))
    result.alpha_composite(cut, ((64 - cut.width) // 2, (64 - cut.height) // 2))
    return result


def current_default(image, mask, illustration):
    """Mirror the current runtime's default item material treatment."""
    if 150 <= illustration <= 155:
        ramps = {1: [[38, 43, 48], [161, 171, 181], [232, 239, 245]]}
    elif 35 <= illustration <= 46:
        ramps = {1: [[8, 9, 10], [56, 54, 58], [146, 157, 169]],
                 2: [[0, 0, 1], [56, 54, 58], [67, 84, 118]]}
    else:
        return image
    pixels = np.array(image)
    labels = np.asarray(mask)[:, :, 0]
    value = pixels[:, :, :3].max(axis=2) / 255.0
    t = np.where(value < .53, np.clip((value - .12) / .41, 0, 1),
                 np.clip((value - .53) / .47, 0, 1))[:, :, None]
    for label, ramp in ramps.items():
        low = np.where((value < .53)[:, :, None], ramp[0], ramp[1])
        high = np.where((value < .53)[:, :, None], ramp[1], ramp[2])
        color = np.floor(low * (1 - t) + high * t + .5).astype('uint8')
        pixels[:, :, :3][labels == label] = color[labels == label]
    return Image.fromarray(pixels)


def main():
    catalog = (ROOT / 'src/game_art_catalog.rs').read_text('utf-8')
    mappings = {ident: int(index) for ident, index in re.findall(
        r'"([^"]+)" => ItemArt \{ icon: \d+, illustration: (\d+) \}', catalog)}
    held = {ident: int(index) for ident, index in re.findall(
        r'"([^"]+)" => (\d+),', catalog.split('fn held_weapon')[1].split('\n}')[0])}
    flat = read(SPRITES / 'flat-agents/manifest.json')
    comic = {e['content_id']: e for e in read(REVIEW.parent / 'comic-catalogue-review-01/catalogue.json')['entries'] if e.get('content_id')}
    armor = read(REVIEW.parent / 'armor-flat-review-01/catalogue.json')['entries']
    outfits = {e['content_id']: e for e in armor if e['kind'] == 'outfit' and e.get('content_id')}
    locale = {}
    for path in sorted((ROOT / 'content/core/locales').glob('fr*.json5')):
        locale.update({json.loads(k): json.loads(v) for k, v in re.findall(
            r'("(?:[^"\\]|\\.)*")\s*:\s*("(?:[^"\\]|\\.)*")', path.read_text('utf-8'))})
    names = {}
    for path in (ROOT / 'content/core/items').glob('*.json5'):
        text = path.read_text('utf-8-sig')
        ident = re.search(r'(?<!\w)(?:"id"|id)\s*:\s*"([^"]+)"', text)
        name = re.search(r'(?<!\w)(?:"name_key"|name_key)\s*:\s*"([^"]+)"', text)
        if ident and name:
            if name[1] in locale:
                names[ident[1]] = locale[name[1]]
    def name(ident, fallback):
        return names.get(ident, fallback)
    old_items = Image.open(SPRITES / 'cyberpunk/crawl-items-32.png').convert('RGBA')
    old_masks = Image.open(SPRITES / 'cyberpunk/crawl-item-materials-32.png').convert('RGBA')
    old_flat = Image.open(SPRITES / 'flat-agents/outfit-icons-32.png').convert('RGBA')
    old_bodies = Image.open(SPRITES / 'cyberpunk/crawl-bodies-32.png').convert('RGBA')
    detailed_items = Image.open(SPRITES / 'cyberpunk/items-128.png').convert('RGBA')
    before, after, entries = [], [], []
    def add(ident, title, kind, old, source_image, source_path, variant=0):
        new = fitted(source_image)
        old = old.resize((64, 64), Image.Resampling.NEAREST)
        assert new.tobytes() != old.tobytes(), ident
        number = len(entries)
        before.append(old)
        after.append(new)
        entries.append(dict(number=number + 1, id=ident, name=title, kind=kind,
                            review_id=f'EQ64-I{number + 1:03}', variant=variant,
                            source=str(source_path.relative_to(ROOT)).replace('\\', '/'),
                            source_sha256=sha(source_path),
                            rgba_sha256=hashlib.sha256(new.tobytes()).hexdigest(),
                            status='pending_native64_icon_review'))
    for ident, model in sorted(held.items(), key=lambda e: e[1]):
        if ident in RETIRED:
            continue
        entry = comic[ident]
        source = ROOT / entry['source_path']
        index = mappings[ident]
        old = current_default(cell(old_items, index, 32), cell(old_masks, index, 32), index)
        add(ident, name(ident, entry['name']), 'weapon', old,
            Image.open(source).convert('RGBA'), source)
    for ident, index in sorted(mappings.items(), key=lambda e: e[1]):
        if not 101 <= index <= 132 or ident in RETIRED:
            continue
        if ident in FLAT:
            continue
        if index <= 114:
            entry = outfits[ident]
            for variant in range(3):
                source = SOURCES / f'actor-resolution-review-01/native64/{entry["id"]}-{variant}-00.png'
                add(ident, name(ident, entry['name']), 'armor', cell(old_items, index, 32),
                    Image.open(source).convert('RGBA'), source, variant)
            continue
        else:
            source = SPRITES / 'cyberpunk/items-128.png'
            detailed = cell(detailed_items, index, 128)
        add(ident, name(ident, comic[ident]['name']), 'armor', cell(old_items, index, 32), detailed, source)
    for ident, index in FLAT.items():
        review_id = flat['outfits'][index]['review_id']
        for variant in range(3):
            source = SOURCES / f'actor-resolution-review-01/native64/{review_id}-{variant}-00.png'
            add(ident, name(ident, review_id), 'armor', cell(old_flat, index * 3 + variant, 32),
                Image.open(source).convert('RGBA'), source, variant)
    before_uri, after_uri = atlas(before), atlas(after)
    worn = []
    old_worn = []
    for index, e in enumerate(flat['outfits']):
        ident = e['review_id']
        entry = next(x for x in read(REVIEW.parent / f'armor-flat-review-{e["origin"]:02}/review.json')['entries'] if x['id'] == ident)
        content_id = next((k for k, v in FLAT.items() if v == index), None)
        old = cell(old_flat, index * 3, 32).resize((64, 64), Image.Resampling.NEAREST)
        old_worn.append(old)
        worn.append(dict(id=ident, content_id=content_id, name=name(content_id, entry['name']),
                         status='accepted_native64_reference', number=len(worn) + 1,
                         review_id=f'EQ64-{ident}', icon_index=index, bound=content_id is not None))
    for ident, entry in outfits.items():
        if ident in RETIRED or ident in FLAT:
            continue
        old_worn.append(cell(old_bodies, mappings[ident] - 98, 32).resize((64, 64), Image.Resampling.NEAREST))
        worn.append(dict(id=entry['id'], content_id=ident, name=name(ident, entry['name']),
                         status='pending_native64_worn_review', number=len(worn) + 1,
                         review_id=f'EQ64-{entry["id"]}', bound=True))
    for entry in worn:
        entry['sources'] = []
        for variant in range(3):
            for config in flat['configurations']:
                source = SOURCES / f'actor-resolution-review-01/native64/{entry["id"]}-{variant}-{config["number"]}.png'
                assert source.is_file(), source
                entry['sources'].append(dict(path=str(source.relative_to(ROOT)).replace('\\', '/'), sha256=sha(source)))
    worn_uri = atlas(old_worn)
    terrain = cell(Image.open(SPRITES / 'cyberpunk/terrain-32.png').convert('RGBA'), 0, 32).resize((64, 64), Image.Resampling.NEAREST)
    terrain_uri = data_uri(terrain)
    counts = dict(weapons=sum(e['kind'] == 'weapon' for e in entries),
                  armor_models=len({e['id'] for e in entries if e['kind'] == 'armor'}),
                  icon_exports=len(entries), accepted_outfits=12, pending_outfits=len(worn) - 12)
    report = dict(review_id='EQUIPMENT-NATIVE64-01', status='review_only',
                  method='Direct downsampling of existing detailed sources, never 32px upscaling; no redraw claimed',
                  runtime_assets_modified=False, counts=counts, icons=entries, worn=worn,
                  excluded_ids=sorted(RETIRED),
                  runtime_gap='core:veste_matelassee still has a legacy runtime binding; explicitly rejected outfit not restored here')
    REVIEW.mkdir(parents=True, exist_ok=True)
    (REVIEW / 'review.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', 'utf-8')
    cards = []
    for i, e in enumerate(entries):
        pos = f'{-(i % 16) * 64}px {-(i // 16) * 64}px'
        cards.append(f'<article class="card" data-kind="{e["kind"]}" data-name="{html.escape(e["name"].lower(), quote=True)}"><div class="number">{e["review_id"]}</div><h3>{html.escape(e["name"])}</h3><p class="id">{e["id"]}{" · couleur " + str(e["variant"] + 1) if e["id"] in outfits or e["id"] in FLAT else ""}</p><div class="pair"><figure><div class="tile before" style="background-position:{pos},0 0"></div><figcaption>32 px × 2 actuel</figcaption></figure><figure><div class="tile after" style="background-position:{pos},0 0"></div><figcaption>Export direct 64 px</figcaption></figure></div></article>')
    page = '''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Armes et armures · 64 px</title><style>
*{box-sizing:border-box}body{margin:0;background:#101518;color:#e9e5dc;font:15px/1.5 system-ui,sans-serif}main{max-width:1240px;margin:auto;padding:24px}h1{font-size:28px;margin:0 0 8px}h2{font-size:22px}p{margin:6px 0}header>p{max-width:900px;color:#b8c4ca}button,select,input{font:inherit;color:inherit;background:#233039;border:1px solid #52616a;border-radius:6px;padding:8px 12px}button,select{cursor:pointer}button.active{background:#8f4832;border-color:#d89b69}.toolbar{position:sticky;top:0;z-index:3;background:#101518ee;backdrop-filter:blur(8px);display:flex;gap:8px;flex-wrap:wrap;padding:12px 0}.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(270px,1fr));gap:12px}.card{background:#192126;border:1px solid #37464e;border-radius:9px;padding:14px;min-width:0}.card h3{font-size:16px;margin:4px 0;min-height:48px}.number{font:12px ui-monospace,monospace;color:#edb584}.id{font:11px ui-monospace,monospace;color:#9db0ba;overflow-wrap:anywhere;min-height:30px}.pair{display:flex;justify-content:space-around;gap:12px}.pair figure{margin:8px 0 0;text-align:center}figcaption{font-size:11px;color:#bac7ce;margin-top:6px}.tile{width:64px;height:64px;background-repeat:no-repeat,repeat;image-rendering:pixelated;margin:auto;position:relative}.tile{isolation:isolate}.tile:after{content:"";position:absolute;inset:0;border:1px solid #60768155;pointer-events:none}.before{background-image:url(BEFORE),url(TERRAIN)}.after{background-image:url(AFTER),url(TERRAIN)}.oldworn{background-image:url(OLDWORN),url(TERRAIN)}.wornfigure{position:relative;width:128px;height:128px;background-image:url(TERRAIN);isolation:isolate}.wornfigure img{width:128px;height:128px;image-rendering:pixelated;position:absolute;left:0;top:0}.wornfigure:after{content:"";position:absolute;left:32px;top:32px;width:64px;height:64px;border:1px solid #9cb0bd88;pointer-events:none}.oldspace{width:128px;height:128px;padding:32px;background-image:url(TERRAIN)}.badge{font-size:12px;color:#bdd6c5;min-height:36px}.pending{color:#f4bc85}.note{padding:12px 14px;border-left:3px solid #b17b50;background:#1e292e;margin:16px 0}.section[hidden],.card[hidden]{display:none!important}.count{margin-left:auto;align-self:center;font-size:13px;color:#b5c6ce}footer{margin-top:28px;color:#b6c6ce;font-size:13px}
</style><main><header><h1>Armes et armures · comparaison en 64 px</h1><p>COUNTS. Les dessins détaillés sont déjà conservés dans le projet. Ces candidats utilisent directement ces sources : aucun passage par une image de 32 px, aucun nouveau dessin annoncé.</p><div class="note">Les icônes font réellement 64 × 64 px. Pour les tenues portées, le carré central représente une case de 64 px ; le canevas de 128 px autorise les débordements. Garder le navigateur à 100 % pour juger la taille réelle.</div></header><nav class="toolbar"><button data-section="icons" class="active">Inventaire et objets au sol</button><button data-section="worn">Tenues portées</button><select id="kind"><option value="all">Armes et armures</option value="weapon">Armes</option><option value="armor">Armures</option></select><input id="search" type="search" placeholder="Chercher un nom"><span class="count" id="count"></span></nav><section id="icons" class="section"><p class="note">Cadrages et résolution à examiner. Les numéros EQ64 restent stables. Les trois couleurs sont montrées pour les seize tenues associées à un objet torse. La comparaison de gauche garde la couleur d’origine. Les modèles supprimés auparavant sont exclus.</p><div class="grid">CARDS</div></section><section id="worn" class="section" hidden><p class="note">Les douze premières tenues sont des références 64 px déjà acceptées, conservées sans retouche. Les sept dernières proposent de remplacer le rendu de corps encore en 32 px. Trois références acceptées n’ont actuellement aucun objet torse associé.</p><div class="toolbar"><label>Couleur <select id="variant"><option value="0">D’origine</option><option value="1">Variante 2</option><option value="2">Variante 3</option></select></label><label>Équipement <select id="config">CONFIGS</select></label></div><div class="grid" id="worncards"></div></section><footer>Revue EQUIPMENT-NATIVE64-01 · aucun remplacement graphique actif à ce stade. La comparaison de gauche montre le corps actuel désarmé ; les équipements choisis à droite servent à contrôler les prises et les débordements. Cette page unique remplace ses propres sorties lors d’une nouvelle génération.</footer></main><script>
const outfits=OUTFITS;const base='../assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/';
const worncards=document.querySelector('#worncards');
for(let i=0;i<outfits.length;i++){const e=outfits[i];const card=document.createElement('article');card.className='card';card.dataset.name=e.name.toLowerCase();card.innerHTML=`<div class="number">${e.review_id}</div><h3></h3><p class="id"></p><p class="badge ${i>=12?'pending':''}"></p><div class="pair"><figure><div class="oldspace"><div class="tile oldworn" style="background-position:${-(i%16)*64}px ${-Math.floor(i/16)*64}px,0 0"></div></div><figcaption>Corps actuel · 32 px × 2</figcaption></figure><figure><div class="wornfigure"><img loading="lazy" alt=""></div><figcaption>Source directe · case 64 px</figcaption></figure></div>`;card.querySelector('h3').textContent=e.name;card.querySelector('.id').textContent=e.content_id||'Référence sans objet torse associé';card.querySelector('.badge').textContent=i<12?'Rendu 64 px déjà accepté':'Remplacement de la tenue portée à valider';worncards.appendChild(card);}
function refresh(){const variant=document.querySelector('#variant').value;const config=document.querySelector('#config').value;document.querySelectorAll('#worncards img').forEach((img,i)=>{img.src=base+outfits[i].id+'-'+variant+'-'+config+'.png';img.alt=outfits[i].name;});}refresh();
document.querySelector('#variant').onchange=refresh;document.querySelector('#config').onchange=refresh;
let selected='icons';function filter(){let count=0;const kind=document.querySelector('#kind').value;const q=document.querySelector('#search').value.toLowerCase();document.querySelectorAll('#'+selected+' .card').forEach(card=>{const show=(!q||card.dataset.name.includes(q))&&(selected==='worn'||kind==='all'||card.dataset.kind===kind);card.hidden=!show;if(show)count++;});document.querySelector('#count').textContent=count+' candidats affichés';}
document.querySelector('#search').oninput=filter;document.querySelector('#kind').onchange=filter;
document.querySelectorAll('[data-section]').forEach(button=>button.onclick=()=>{selected=button.dataset.section;document.querySelectorAll('.section').forEach(s=>s.hidden=s.id!==selected);document.querySelectorAll('[data-section]').forEach(b=>b.classList.toggle('active',b===button));document.querySelector('#kind').hidden=selected==='worn';filter();});filter();
</script></html>'''
    for key, value in {'TERRAIN': terrain_uri, 'BEFORE': before_uri, 'AFTER': after_uri,
                       'OLDWORN': worn_uri, 'COUNTS': f'{counts["weapons"]} armes, {counts["armor_models"]} armures et {len(worn)} tenues portées',
                       'CARDS': ''.join(cards), 'OUTFITS': json.dumps([{k: v for k, v in e.items() if k != 'sources'} for e in worn], ensure_ascii=False),
                       'CONFIGS': ''.join(f'<option value="{c["number"]}">{html.escape(c["label"])}</option>' for c in flat['configurations'])}.items():
        page = page.replace(key, value)
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(page, 'utf-8')
    contact_sheet(entries, before, after)
    worn_sheet(worn)
    print(json.dumps(dict(counts=counts, html_bytes=OUTPUT.stat().st_size, runtime_assets_modified=False), ensure_ascii=False))


def contact_sheet(entries, before, after):
    """Small native-size deliverable, not a browser screenshot."""
    selected = ['core:modele_brenek_a80', 'core:fusil_de_guetteur',
                'core:hache_de_combat', 'core:melee_epee_longue',
                'core:melee_couteau_de_camp', 'core:modele_cogneur_p01',
                'core:modele_crache_fer_pm01', 'core:marteau_de_guerre',
                'core:armure_cerbere', 'core:manteau_de_mue',
                'core:patched_plating', 'core:veste_de_membranes',
                'core:tenue_assaut', 'core:tenue_rituelle',
                'core:casque_cerbere', 'core:bottes_cerbere']
    font_path = Path('C:/Windows/Fonts/segoeui.ttf')
    font = ImageFont.truetype(str(font_path), 14)
    small = ImageFont.truetype(str(font_path), 12)
    board = Image.new('RGB', (992, 704), '#101518')
    draw = ImageDraw.Draw(board)
    draw.text((18, 8), 'ARMES ET ARMURES — 64 px réels · sélection de contrôle', font=font, fill='#f0d9bb')
    for j, ident in enumerate(selected):
        e = next(x for x in entries if x['id'] == ident and x['variant'] == 0)
        i = e['number'] - 1
        x, y = 12 + j % 4 * 248, 38 + j // 4 * 165
        draw.rounded_rectangle((x, y, x + 236, y + 155), radius=6, fill='#192126', outline='#37464e')
        draw.text((x + 10, y + 5), e['review_id'], font=small, fill='#edb584')
        title = e['name']
        while draw.textlength(title, font=font) > 218:
            title = title[:-2] + '…'
        draw.text((x + 10, y + 24), title, font=font, fill='#e9e5dc')
        for k, tile in enumerate((before[i], after[i])):
            background = Image.new('RGBA', (64, 64), '#263035')
            background.alpha_composite(tile)
            board.paste(background.convert('RGB'), (x + 24 + k * 116, y + 51))
        draw.text((x + 21, y + 123), 'Actuel 32 × 2', font=small, fill='#b5c6ce')
        draw.text((x + 139, y + 123), 'Direct 64', font=small, fill='#e1b185')
    board.save(OUTPUT.parent / 'equipement64-controle.png')


def worn_sheet(entries):
    pending = [e for e in entries if e['status'] == 'pending_native64_worn_review']
    board = Image.new('RGB', (832, 768), '#101518')
    draw = ImageDraw.Draw(board)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 14)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    draw.text((16, 8), 'TENUES PORTÉES — candidats 64 px · trois couleurs', font=font, fill='#f0d9bb')
    for i, entry in enumerate(pending):
        x, y = 8 + i % 2 * 416, 38 + i // 2 * 180
        draw.rounded_rectangle((x, y, x + 400, y + 170), radius=6, fill='#192126', outline='#37464e')
        draw.text((x + 12, y + 5), entry['review_id'] + ' · ' + entry['name'], font=font, fill='#edb584')
        for variant in range(3):
            path = SOURCES / f'actor-resolution-review-01/native64/{entry["id"]}-{variant}-00.png'
            image = Image.open(path).convert('RGBA')
            assert image.size == (128, 128)
            left, top = x + 8 + variant * 128, y + 22
            board.paste(image, (left, top), image)
            draw.rectangle((left + 32, top + 32, left + 95, top + 95), outline='#607681')
            draw.text((left + 30, top + 130), ['Origine', 'Variante 2', 'Variante 3'][variant], font=small, fill='#b5c6ce')
    board.save(OUTPUT.parent / 'equipement64-tenues.png')


if __name__ == '__main__':
    main()
