"""One complete native64 monster review image, with stable names and codes.

This reuses frozen/previous body exports without resizing or repainting them.
Only missing older native exports and separate contact shadows are conditioned.
No runtime atlas, content rule or acceptance record is written.
"""
import hashlib
import html
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from review_contact_shadows import contact_shadow

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT = ROOT/'artifacts/npc-monsters-review-01/catalogue'
SECTIONS = [
    'Cultistes humains', 'Cultistes possédés', 'Nécromanciens scientifiques',
    'Nécromanciens démoniaques', 'Zombies', 'Esprits torturés',
    'Squelettes', 'Créatures enflammées', 'Possédés et hybrides',
    'Vers et larves démoniaques', 'Démons ailés', 'Minotaures ailés',
    'Démons de peste', 'Démons du froid', 'Démons humanoïdes',
    'Démons et créations', 'Machines détournées', 'Élites démoniaques',
    'Boss de secteur',
]
EXCLUDED = {13, 23, 27, 91, 94, 95, 97, 110, 114, 163}


def read(name):
    return json.loads((REVIEW/name).read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')


def write_viewer(output, size, sections):
    links = ''.join(f'<a href="#s{i}">{html.escape(s["name"])} ({s["count"]})</a>'
                    for i, s in enumerate(sections))
    anchors = ''.join(f'<span id="s{i}" style="top:{s["y"]}px"></span>'
                      for i, s in enumerate(sections))
    (OUT/'index.html').write_text(
        '<!doctype html><html lang="fr"><meta charset="utf-8">'
        '<title>Bestiaire — taille native</title><style>'
        'body{margin:0;background:#10171c;color:#dce4e4;font:14px Segoe UI,sans-serif}'
        'header{position:sticky;top:0;z-index:2;background:#10171cf5;'
        f'border-bottom:1px solid #35434b;padding:12px 18px;width:{size[0]-36}px}}'
        'h1{font-size:19px;color:#e5d2a8;margin:0 0 8px}'
        'nav{display:flex;gap:6px 14px;flex-wrap:wrap}a{color:#d5c39f}'
        f'main{{position:relative;width:{size[0]}px}}'
        'main span{position:absolute;left:0;scroll-margin-top:210px}'
        f'img{{display:block;width:{size[0]}px;max-width:none;height:auto;image-rendering:pixelated}}'
        '</style><header><h1>Bestiaire complet — affichage à 100 %, 64 px par case</h1>'
        f'<nav>{links}<a href="{output.name}">Ouvrir le PNG complet</a></nav></header>'
        f'<main>{anchors}<img src="{output.name}" width="{size[0]}" height="{size[1]}" '
        'alt="Grande planche des monstres, codes et noms, sans agrandissement des personnages">'
        '</main></html>\n', encoding='utf-8')


def section(e):
    number, ident = e['number'], e['id']
    if e.get('catalogue_section'):
        return e['catalogue_section']
    if ident.startswith('NEC-C'):
        return 'Nécromanciens scientifiques'
    if ident.startswith('NEC-D'):
        return 'Nécromanciens démoniaques'
    if ident.startswith('CULT-'):
        return 'Cultistes humains' if number < 116 else 'Cultistes possédés'
    if number in {16, 17, 22, 23, 24}:
        return 'Cultistes humains'
    if ident.startswith('ZOM-'):
        return 'Zombies'
    if ident.startswith('ESP-'):
        return 'Esprits torturés'
    if ident.startswith('SQUE-'):
        return 'Squelettes'
    if ident.startswith('BRA-'):
        return 'Créatures enflammées'
    if number in {18, 19}:
        return 'Boss de secteur'
    if number in {20, 21}:
        return 'Élites démoniaques'
    if number == 27:
        return 'Machines détournées'
    if number in {5, 6, 7, 10, 11, 12, 25, 26, 96, 98, 99}:
        return 'Possédés et hybrides'
    return 'Démons et créations'


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT/'shadows').mkdir(exist_ok=True)
    listing = read('monster-candidate-list.json')
    existing = {e['number']: e for e in listing['records']}
    exports = {e['id']: e for e in read('exports.json')['items']}
    bodies, raw = {}, {}

    def include(e, review):
        e = dict(e, **existing.get(e['number'], {}))
        if 'section' in e:
            e['catalogue_section'] = e['section']
        if e['number'] in EXCLUDED or e.get('status') in {'rejected_deleted', 'generation_blocked_no_image'}:
            return
        if not (ROOT/e['source']).is_file():
            return
        raw[e['number']] = (e, review)

    for listed in listing['records']:
        source_review = read(listed['review'])
        original = next(e for e in source_review['records'] if e['number'] == listed['number'])
        include(original, listed['review'])
    for e in read('review.json')['records']:
        if 5 <= e['number'] <= 17:
            include(e, 'review.json')
    review_names = ['sector-bosses-11.json', 'elite-monsters-16.json', 'petites-troupes-17.json', 'cultists-zombies-spirits-24.json']
    if listing.get('latest_batch') and listing['latest_batch'] not in review_names:
        review_names.append(listing['latest_batch'])
    for name in review_names:
        for e in read(name)['records']:
            include(e, name)

    normalized = []
    for number, (e, review) in sorted(raw.items()):
        source = ROOT/e['source']
        expected_source = e.get('source_sha256', e.get('accepted_source_sha256'))
        if expected_source:
            assert sha(source) == expected_source, (number, 'source')
        variants = e.get('variants')
        if not variants:
            meta = e.get('accepted_exports', {}).get('64') or e.get('native64_export')
            if not meta:
                meta = next((m for m in exports.get(e['id'], {}).get('exports', []) if m['cell'] == 64), None)
            if not meta:
                raise ValueError(f"Missing durable native export for {number}")
            variants = [dict(meta, variant='A')]
        variants = [dict(v) for v in variants]
        for v in variants:
            path = ROOT/v['file']
            assert sha(path) == v['sha256'], (number, 'body')
            art = Image.open(path).convert('RGBA')
            assert v['cell'] == 64
            assert list(art.getbbox()) == v['occupied']
            v['colors'] = len({p[:3] for p in art.get_flattened_data() if p[3]})

        art = Image.open(ROOT/variants[0]['file']).convert('RGBA')
        shadow, contact = contact_shadow(art, variants[0]['foot_anchor'])
        shadow_path = OUT/'shadows'/f"{e['id']}-{e['revision']}-shadow64.png"
        shadow.save(shadow_path, optimize=True)
        box = art.getbbox()
        result = dict(existing.get(number, {}))
        result.update(number=number, id=e['id'], revision=e['revision'],
            code=f"{e['id']}-{e['revision']}", label=e['label'], review=review,
            source=e['source'], source_sha256=sha(source),
            native_body_size=[box[2]-box[0], box[3]-box[1]],
            variants=variants, variant_files=[v['file'] for v in variants],
            contact_shadow=dict(file=shadow_path.relative_to(ROOT).as_posix(), sha256=sha(shadow_path),
                foot_anchor=variants[0]['foot_anchor'], contact=contact),
            section=section(e), visual_approved=e.get('visual_approved', False),
            name_approved=e.get('name_approved', False), rules_approved=e.get('rules_approved', False),
            scale_approved=e.get('scale_approved', bool(e.get('visual_approved') and e.get('accepted_exports', {}).get('64'))),
            status='visually_approved' if e.get('visual_approved') else 'listed_pending_later_individual_selection',
            source_status=e.get('status', 'proposal'), runtime_connected=e.get('runtime_connected', False))
        if e.get('lore_status'):
            result['lore_status'] = e['lore_status']
        normalized.append(result)
        bodies[number] = ([(Image.open(ROOT/v['file']).convert('RGBA'), v) for v in variants], shadow)

    player = read('cultists-zombies-spirits-24.json')['player_reference']
    assert sha(ROOT/player['source']) == player['sha256']
    player_art = Image.open(ROOT/player['source']).convert('RGBA')
    player_shadow, _ = contact_shadow(player_art, player['foot_anchor'])
    groups = [(name, [e for e in normalized if e['section'] == name]) for name in SECTIONS]
    cursor, rows, sections = 100, [], []
    for name, entries in groups:
        if not entries:
            continue
        sections.append(dict(name=name, y=cursor, count=len(entries)))
        cursor += 44
        for start in range(0, len(entries), 2):
            pair = entries[start:start+2]
            height = max(202, max(e['native_body_size'][1] for e in pair)+112)
            rows.append((cursor, height, pair))
            cursor += height+12
        cursor += 12
    extra=max(0,max(len(e['variants']) for e in normalized)-4)*130
    card_width=662+extra
    sheet = Image.new('RGB', (1360+extra*2, cursor+42), '#10171c')
    draw = ImageDraw.Draw(sheet)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf', 17)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 14)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf', 12)
    draw.text((18, 12), 'Bestiaire complet · codes et noms · gabarits natifs du jeu', font=bold, fill='#e5d2a8')
    draw.text((18, 40), f"{len(normalized)} modèles · base de 64 px par case · aucun zoom ni ajustement au cadre", font=font, fill='#c3cdd0')
    draw.text((18, 64), 'A/B/C/D/E : palettes du même dessin · joueur à droite · pouvoirs des lanceurs de sorts et boss de secteur en réserve', font=small, fill='#b4c4c8')
    for s in sections:
        draw.text((18, s['y']+5), f"{s['name']} · {s['count']}", font=bold, fill='#e5d2a8')
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0, 0, 240, 155))
    placements = []

    def paste_native(art, center, baseline, anchor, tag):
        box = art.getbbox()
        position = (center-anchor[0]+box[0], baseline-anchor[1]+box[1])
        crop = art.crop(box)
        sheet.paste(crop, position, crop)
        if tag:
            placements.append(dict(tag, position=list(position), source_crop=list(box), size=list(crop.size), scale=1))

    for y, height, pair in rows:
        for column, e in enumerate(pair):
            x = 12+column*(card_width+12)
            draw.rectangle((x, y, x+card_width, y+height), fill='#1a242b', outline='#35434b')
            draw.text((x+12, y+7), f"{e['number']:03} · {e['code']}", font=bold, fill='#e5d2a8')
            draw.text((x+12, y+32), e['label'], font=font, fill='#e3e6df')
            w, h = e['native_body_size']
            state = 'Intégré' if e.get('runtime_connected') else 'Dessin conservé en réserve' if e.get('runtime_authorized') else 'À sélectionner'
            if e.get('palette_review_status') == 'native_review_pending':
                state = 'Base intégrée · nouvelles couleurs en revue'
            if e['number'] in {19, 20, 21}:
                state = 'Dessin apprécié · gabarit proposé'
            draw.text((x+12, y+54), f'{state} · {w} × {h} px', font=small, fill='#b4c4c8')
            floor_box = (x+10, y+76, x+card_width-10, y+height-12)
            for tx in range(floor_box[0], floor_box[2], ground.width):
                for ty in range(floor_box[1], floor_box[3], ground.height):
                    sheet.paste(ground.crop((0, 0, min(ground.width, floor_box[2]-tx), min(ground.height, floor_box[3]-ty))), (tx, ty))
            arts, shadow = bodies[e['number']]
            baseline = y+height-34
            centers = [x+76+i*130 for i in range(len(arts))] if len(arts) > 2 else [x+148, x+378] if len(arts) > 1 else [x+270]
            assert len(arts) <= len(centers)
            for center, (art, meta) in zip(centers, arts):
                box = art.getbbox()
                visible = (center-meta['foot_anchor'][0]+box[0], baseline-meta['foot_anchor'][1]+box[1], center-meta['foot_anchor'][0]+box[2], baseline-meta['foot_anchor'][1]+box[3])
                assert visible[0] >= floor_box[0] and visible[2] <= floor_box[2] and visible[1] >= floor_box[1], (e['number'], visible, floor_box)
                paste_native(shadow, center, baseline, meta['foot_anchor'], None)
                paste_native(art, center, baseline, meta['foot_anchor'], dict(number=e['number'], variant=meta['variant'], file=meta['file'], sha256=meta['sha256']))
                variant_label=meta['variant']+((' · '+meta['label']) if meta.get('label') else '')
                draw.text((center, baseline+5), variant_label, anchor='mt', font=small, fill='#e5e2d2')
            for layer in [player_shadow, player_art]:
                paste_native(layer, x+592+extra, baseline, player['foot_anchor'], None)
            draw.text((x+592+extra, baseline+5), 'Joueur', anchor='mt', font=small, fill='#e5e2d2')
    draw.text((18, cursor+8), 'Sources et identifiants conservés · intégration limitée aux rôles et règles déjà disponibles', font=small, fill='#b4c4c8')
    output = OUT/'monstres-candidats-native64.png'
    sheet.save(output, optimize=True)
    write_viewer(output, sheet.size, sections)
    layout = dict(review_id=listing.get('catalogue_review_id', 'CATALOGUE-24'), image=output.relative_to(ROOT).as_posix(), image_sha256=sha(output),
        size=list(sheet.size), scale=1, actor_resize_operations=0, cell=64, count=len(normalized),
        player_reference=player, sections=sections, placements=placements,
        excluded_deleted=[13, 23, 27, 91, 94, 95, 97, 163], excluded_unrendered=[110, 114],
        older_style_trials_kept_separately=list(range(40, 60)), runtime_connected=True)
    layout_name = listing.get('catalogue_layout', 'catalogue-layout-24.json')
    write_json(REVIEW/layout_name, layout)
    listing.update(candidate_count=len(normalized), records=normalized, complete_catalogue=True,
        catalogue_image=output.relative_to(ROOT).as_posix(), catalogue_layout=layout_name,
        catalogue_viewer=(OUT/'index.html').relative_to(ROOT).as_posix(),
        older_reviews_retained_separately=['demons-varied-18.json'],
        catalogued_reviews=sorted({e['review'] for e in normalized}),
        visual_approved_count=sum(e['visual_approved'] for e in normalized),
        pending_visual_count=sum(not e['visual_approved'] for e in normalized),
        note='Unified current review, including older cultists, hybrids, demons, elites and bosses. Superseded style trials remain separate; removed models never return.')
    write_json(REVIEW/'monster-candidate-list.json', listing)
    print(json.dumps(dict(models=len(normalized), image=str(output), pixels=list(sheet.size), cultists=[14, 15, 16, 17, 22, 24, 164, 165], sections={s['name']:s['count'] for s in sections}), ensure_ascii=True))


if __name__ == '__main__':
    main()
