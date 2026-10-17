"""Native-only TROUPES-17 board, indexed palette variants and contact shadows.

Bodies originate from image_gen; this script only conditions native exports,
maps their indexed palettes and composes the review. No runtime atlas writes.
"""
import colorsys
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

from review_contact_shadows import contact_shadow
from show_npc_monsters_review import conditioned_image

ROOT = Path(__file__).resolve().parents[2]
REVIEW = ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01'
ART = ROOT/'assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01'
OUT = ROOT/'artifacts/npc-monsters-review-01'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def alternate(rgb, number):
    h, s, v = colorsys.rgb_to_hsv(*(c/255 for c in rgb))
    degrees = h*360
    new = None
    if v < .19 or s < .12:
        return rgb
    if number <= 24 and (degrees < 20 or degrees > 335) and s > .3:
        new = (75/360, s*.78, v)
    elif number == 25 and 135 < degrees < 225 and s > .2:
        new = (72/360, s*.8, v)
    elif number == 26 and 30 < degrees < 70 and s > .35:
        new = (8/360, s*.8, v*.9)
    elif number == 27 and 25 < degrees < 80 and s > .25:
        new = (207/360, s*.65, v*.9)
    elif number == 28 and 45 < degrees < 170 and s > .2:
        new = (12/360, s*.8, v)
    elif number == 29 and degrees < 55 and rgb[1] >= rgb[0]*.65:
        new = (85/360, min(s,.23), v*.9)
    elif number == 30 and (degrees < 65 or degrees > 340) and s > .25:
        new = (210/360, min(s,.48), v*.9)
    elif number == 31 and 145 < degrees < 235 and s > .2:
        new = (23/360, s*.95, v)
    elif number == 32 and 205 < degrees < 300 and s > .17:
        new = (150/360, s*.8, v)
    elif number == 33 and 175 < degrees < 260 and s > .15:
        new = (178/360, s*.8, v*.8)
    elif number == 34 and (degrees < 70 or degrees > 340) and s > .25:
        new = (215/360, min(s,.48), v*.92)
    elif number == 35 and degrees < 55 and rgb[1] >= rgb[0]*.65:
        new = (85/360, min(s,.23), v*.92)
    elif number == 36 and 165 < degrees < 270 and s > .15:
        new = (120/360, s*.8, v)
    elif number == 37 and degrees < 65 and rgb[1] >= rgb[0]*.65:
        new = (195/360, min(s,.22), v*.92)
    elif number == 38 and 135 < degrees < 245 and s > .17:
        new = (280/360, s*.8, v)
    elif number == 39 and (degrees < 45 or degrees > 270) and s > .17:
        new = (170/360, min(s,.32), v)
    if new is None:
        return rgb
    return tuple(round(c*255) for c in colorsys.hsv_to_rgb(*new))


def variant(body, number):
    colors = sorted({p[:3] for p in body.get_flattened_data() if p[3]})
    mapping = {rgb: alternate(rgb, number) for rgb in colors}
    result = body.copy()
    result.putdata([(*mapping[p[:3]], p[3]) if p[3] else p for p in body.get_flattened_data()])
    changed = sum(a != b for a,b in zip(body.get_flattened_data(),result.get_flattened_data()))
    assert changed > 0, f'No palette region selected: {number}'
    assert result.getchannel('A').tobytes() == body.getchannel('A').tobytes()
    return result, dict(changed_pixels=changed, alpha_and_geometry_identical=True,
        palette_map=[dict(source=list(a), target=list(b)) for a,b in mapping.items() if a != b])


def main():
    path = REVIEW/'petites-troupes-17.json'
    batch = json.loads(path.read_text(encoding='utf-8-sig'))
    (OUT/'troupes-tiles').mkdir(parents=True, exist_ok=True)
    (ART/'troupes-shadows-17').mkdir(parents=True, exist_ok=True)
    ready = []
    layers = {}
    for entry in batch['records']:
        if not entry['in_current_demon_batch'] and entry['number'] != 25:
            continue
        source = ROOT/entry['source']
        if not source.exists():
            continue
        assert sha(source) == entry['source_sha256']
        body, anchor = conditioned_image(entry, 64)
        other, recolor = variant(body, entry['number'])
        entry['palette_variant'] = recolor
        entry['variants'] = []
        shadow, contact = contact_shadow(body, anchor)
        shadow_path = ART/'troupes-shadows-17'/f"{entry['id']}-{entry['revision']}-shadow64.png"
        shadow.save(shadow_path, optimize=True)
        entry['contact_shadow'] = dict(file=shadow_path.relative_to(ROOT).as_posix(),
            sha256=sha(shadow_path), foot_anchor=list(anchor), contact=contact, visual_approved=False)
        for suffix, image in [('A',body),('B',other)]:
            output = OUT/'troupes-tiles'/f"{entry['id']}-{entry['revision']}-{suffix}-native64.png"
            image.save(output, optimize=True)
            entry['variants'].append(dict(variant=suffix, file=output.relative_to(ROOT).as_posix(),
                canvas=image.width, cell=64, occupied=list(image.getbbox()), foot_anchor=list(anchor),
                colors=len({p[:3] for p in image.get_flattened_data() if p[3]}), sha256=sha(output), visual_approved=False))
        layers[entry['id']] = (body, other, shadow, anchor)
        if entry['in_current_demon_batch']:
            ready.append(entry)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',17)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0,0,240,155))
    player = batch['player_reference']
    assert sha(ROOT/player['source']) == player['sha256']
    player_body = Image.open(ROOT/player['source']).convert('RGBA')
    player_shadow,_ = contact_shadow(player_body,player['foot_anchor'])
    for start in [0,6]:
        subset = ready[start:start+6]
        if not subset:
            continue
        sheet = Image.new('RGB',(920,650),'#10171c')
        draw = ImageDraw.Draw(sheet)
        draw.text((18,12),'Petits démons · 64 px par case · taille native · aucun zoom',font=bold,fill='#e5d2a8')
        draw.text((18,42),'A et B : même dessin, palettes différentes · joueur à droite comme repère',font=font,fill='#c3cdd0')
        for i,e in enumerate(subset):
            x,y = 12+(i%2)*454, 76+(i//2)*181
            draw.rectangle((x,y,x+442,y+168),fill='#1a242b',outline='#35434b')
            draw.text((x+12,y+7),f"{e['number']} · {e['label']}",font=bold,fill='#e5d2a8')
            body,other,shadow,anchor = layers[e['id']]
            b = body.getbbox()
            draw.text((x+12,y+33),f"{e['family']} · {b[2]-b[0]} × {b[3]-b[1]} px",font=small,fill='#b4c4c8')
            # Tile the existing floor at its own resolution; no actor resizing.
            sheet.paste(ground.crop((0,0,216,103)),(x+10,y+55))
            sheet.paste(ground.crop((0,0,216,103)),(x+226,y+55))
            for center,art,label in [(x+84,body,'A'),(x+204,other,'B')]:
                for layer in [shadow,art]:
                    sheet.paste(layer,(center-anchor[0],y+140-anchor[1]),layer)
                draw.text((center,y+145),label,anchor='mt',font=small,fill='#e5e2d2')
            for layer in [player_shadow,player_body]:
                sheet.paste(layer,(x+351-player['foot_anchor'][0],y+140-player['foot_anchor'][1]),layer)
            draw.text((x+351,y+145),'Joueur',anchor='mt',font=small,fill='#e5e2d2')
        draw.text((18,627),'Propositions de monstres ordinaires · ombres séparées · aucun raccord au jeu',font=font,fill='#b4c4c8')
        last = 33 if start == 0 else 39
        sheet.save(OUT/f'petits-demons-{28+start}-{last}.png',optimize=True)
    if 'TROUP-04' in layers:
        body,other,shadow,anchor = layers['TROUP-04']
        mage = Image.new('RGB',(460,210),'#10171c')
        md = ImageDraw.Draw(mage)
        md.text((12,10),'25 · Magicien possédé · conservé à part',font=bold,fill='#e5d2a8')
        md.text((12,38),'Taille native · dessin apprécié · sorts à définir',font=small,fill='#b4c4c8')
        mage.paste(ground.crop((0,0,216,103)),(10,62))
        mage.paste(ground.crop((0,0,216,103)),(226,62))
        for center,art,label in [(84,body,'A'),(204,other,'B')]:
            for layer in [shadow,art]:
                mage.paste(layer,(center-anchor[0],147-anchor[1]),layer)
            md.text((center,153),label,anchor='mt',font=small,fill='#e5e2d2')
        for layer in [player_shadow,player_body]:
            mage.paste(layer,(351-player['foot_anchor'][0],147-player['foot_anchor'][1]),layer)
        md.text((351,153),'Joueur',anchor='mt',font=small,fill='#e5e2d2')
        md.text((12,187),'Deux palettes proposées · aucune intégration',font=small,fill='#b4c4c8')
        mage.save(OUT/'magicien-possede-25.png',optimize=True)
    batch['preview_scale'] = 1
    batch['zoom_previews_generated'] = False
    path.write_text(json.dumps(batch,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(f'{len(ready)} démons et {len(ready)} variantes natives ; magicien possédé conservé à part.')


if __name__ == '__main__':
    main()
