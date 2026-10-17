"""Demon reviews: varied native body sizes, indexed recolors and separate shadows.

Original sources come from built-in image_gen. No anatomy is painted here,
and no runtime resource or acceptance registry is written.
"""
import argparse
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
OUT = ROOT/'artifacts/npc-monsters-review-01/demons-18'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def recolor(body, rule):
    mapping = {}
    for rgb in sorted({p[:3] for p in body.get_flattened_data() if p[3]}):
        h,s,v = colorsys.rgb_to_hsv(*(c/255 for c in rgb))
        target = rgb
        if rule['hue_min'] <= h*360 <= rule['hue_max'] and s > rule['minimum_saturation'] and v > rule['minimum_value']:
            target = tuple(round(c*255) for c in colorsys.hsv_to_rgb(rule['target_hue']/360,
                min(max(s*rule['saturation_factor'],rule.get('saturation_floor',0)),.6),
                v*rule['value_factor']))
        mapping[rgb] = target
    result = body.copy()
    result.putdata([(*mapping[p[:3]],p[3]) if p[3] else p for p in body.get_flattened_data()])
    changed = sum(a!=b for a,b in zip(body.get_flattened_data(),result.get_flattened_data()))
    assert changed > 0, 'No source color selected for alternate palette'
    assert body.getchannel('A').tobytes() == result.getchannel('A').tobytes()
    return result, dict(changed_pixels=changed,alpha_and_geometry_identical=True,
        palette_map=[dict(source=list(a),target=list(b)) for a,b in mapping.items() if a!=b])


def main():
    global OUT
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--review', default='demons-varied-18.json')
    parser.add_argument('--output', default='demons-18')
    args = parser.parse_args()
    OUT = ROOT/'artifacts/npc-monsters-review-01'/args.output
    path = REVIEW/args.review
    batch = json.loads(path.read_text(encoding='utf-8'))
    shadow_dir = ART/f"demons-shadows-{batch['review_id'].rsplit('-',1)[-1]}"
    (OUT/'tiles').mkdir(parents=True,exist_ok=True)
    shadow_dir.mkdir(parents=True,exist_ok=True)
    ready = []
    layers = {}
    for entry in batch['records']:
        if entry.get('status') in {'rejected_deleted', 'generation_blocked_no_image'}:
            continue
        source = ROOT/entry['source']
        if not source.exists():
            continue
        assert sha(source) == entry['source_sha256']
        body,anchor = conditioned_image(entry,64)
        other,mapping = recolor(body,entry['palette_transfer'])
        shadow,contact = contact_shadow(body,anchor)
        shadow_path = shadow_dir/f"{entry['id']}-{entry['revision']}-shadow64.png"
        shadow.save(shadow_path,optimize=True)
        entry['contact_shadow'] = dict(file=shadow_path.relative_to(ROOT).as_posix(),
            sha256=sha(shadow_path),foot_anchor=list(anchor),contact=contact,visual_approved=False)
        entry['palette_variant'] = mapping
        entry['variants'] = []
        for suffix,image in [('A',body),('B',other)]:
            output = OUT/'tiles'/f"{entry['id']}-{entry['revision']}-{suffix}-native64.png"
            image.save(output,optimize=True)
            entry['variants'].append(dict(variant=suffix,file=output.relative_to(ROOT).as_posix(),
                canvas=image.width,cell=64,foot_anchor=list(anchor),occupied=list(image.getbbox()),
                colors=len({p[:3] for p in image.get_flattened_data() if p[3]}),sha256=sha(output),visual_approved=False))
        layers[entry['id']] = (body,other,shadow,anchor)
        ready.append(entry)
    font = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',14)
    bold = ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',17)
    small = ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    ground = Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0,0,240,155))
    player = batch['player_reference']
    assert sha(ROOT/player['source']) == player['sha256']
    player_body = Image.open(ROOT/player['source']).convert('RGBA')
    player_shadow,_ = contact_shadow(player_body,player['foot_anchor'])
    preview_sheets = []
    for start in range(0,len(ready),4):
        subset = ready[start:start+4]
        sheet = Image.new('RGB',(920,550),'#10171c')
        draw = ImageDraw.Draw(sheet)
        title = batch.get('preview_title','Démons · tailles variées · 64 px par case · aucun zoom')
        if batch.get('family_sheet_titles'):
            title = f"{subset[0]['family']} · taille native · 64 px par case"
        draw.text((18,12),title,font=bold,fill='#e5d2a8')
        draw.text((18,42),'A et B : même silhouette · joueur à droite, même échelle · sélection ultérieure',font=font,fill='#c3cdd0')
        for i,e in enumerate(subset):
            x,y = 12+(i%2)*454,76+(i//2)*218
            draw.rectangle((x,y,x+442,y+206),fill='#1a242b',outline='#35434b')
            heading = f"{e['number']} · {e['id']}-{e['revision']}" if batch.get('show_codes') else f"{e['number']} · {e['label']}"
            draw.text((x+12,y+7),heading,font=bold,fill='#e5d2a8')
            body,other,shadow,anchor = layers[e['id']]
            b = body.getbbox()
            subheading = e['label'] if batch.get('show_codes') else e['family']
            draw.text((x+12,y+34),f"{subheading} · {b[2]-b[0]} × {b[3]-b[1]} px",font=small,fill='#b4c4c8')
            sheet.paste(ground.crop((0,0,216,139)),(x+10,y+58))
            sheet.paste(ground.crop((0,0,216,139)),(x+226,y+58))
            for center,art,label in [(x+84,body,'A'),(x+218,other,'B')]:
                for layer in [shadow,art]:
                    sheet.paste(layer,(center-anchor[0],y+175-anchor[1]),layer)
                draw.text((center,y+181),label,anchor='mt',font=small,fill='#e5e2d2')
            for layer in [player_shadow,player_body]:
                sheet.paste(layer,(x+365-player['foot_anchor'][0],y+175-player['foot_anchor'][1]),layer)
            draw.text((x+365,y+181),'Joueur',anchor='mt',font=small,fill='#e5e2d2')
        draw.text((18,527),'Galerie de candidats · ombres séparées · aucun acteur ni pouvoir ajouté au jeu',font=font,fill='#b4c4c8')
        first_number = subset[0]['number']
        last_number = subset[-1]['number']
        preview_path = OUT/f'demons-{first_number}-{last_number}.png'
        sheet.save(preview_path,optimize=True)
        preview_sheets.append(preview_path.relative_to(ROOT).as_posix())
    old = json.loads((REVIEW/'petites-troupes-17.json').read_text(encoding='utf-8'))
    e = next(e for e in old['records'] if e['number']==28)
    images = [Image.open(ROOT/m['file']).convert('RGBA') for m in e['variants']]
    shadow = Image.open(ROOT/e['contact_shadow']['file']).convert('RGBA')
    anchor = e['variants'][0]['foot_anchor']
    panel = Image.new('RGB',(460,230),'#10171c')
    draw = ImageDraw.Draw(panel)
    draw.text((12,10),'28 · Démon voûté · stature du joueur',font=bold,fill='#e5d2a8')
    draw.text((12,39),'65 px de haut · même source · export natif révisé',font=small,fill='#b4c4c8')
    panel.paste(ground.crop((0,0,216,132)),(10,64))
    panel.paste(ground.crop((0,0,216,132)),(226,64))
    for center,art,label in [(84,images[0],'A'),(218,images[1],'B')]:
        for layer in [shadow,art]:
            panel.paste(layer,(center-anchor[0],177-anchor[1]),layer)
        draw.text((center,183),label,anchor='mt',font=small,fill='#e5e2d2')
    for layer in [player_shadow,player_body]:
        panel.paste(layer,(365-player['foot_anchor'][0],177-player['foot_anchor'][1]),layer)
    draw.text((365,183),'Joueur',anchor='mt',font=small,fill='#e5e2d2')
    draw.text((12,209),'Taille native · ombre séparée · sélection ultérieure',font=small,fill='#b4c4c8')
    panel.save(OUT/'demon-28-taille-joueur.png',optimize=True)
    batch['preview_scale'] = 1
    batch['zoom_previews_generated'] = False
    batch['preview_sheets'] = preview_sheets
    if batch.get('combined_preview') and preview_sheets:
        boards = [Image.open(ROOT/p).convert('RGB') for p in preview_sheets]
        combined = Image.new('RGB', (boards[0].width, sum(b.height for b in boards)), '#10171c')
        cursor = 0
        for board in boards:
            combined.paste(board, (0, cursor))
            cursor += board.height
        combined.save(ROOT/batch['combined_preview'], optimize=True)
    if batch.get('native_floor_review'):
        # A composed review scene: bodies remain at one source pixel per pixel.
        # Solid backgrounds are diagnostic underlays; the texture is reused.
        floors = Image.new('RGB', (920, 1748), '#10171c')
        fd = ImageDraw.Draw(floors)
        assert len(ready) <= 24
        for panel, (label, fill) in enumerate([
            ('Fond clair', '#b4afa4'), ('Fond sombre', '#202930'),
            ('Sol texturé de référence', None),
        ]):
            top = panel*582
            fd.text((18, top+10), f'{label} · corps natifs · joueur à droite', font=bold, fill='#e5d2a8')
            floor_y = top+46
            if fill:
                fd.rectangle((12, floor_y, 907, floor_y+511), fill=fill)
            else:
                for tx in range(12, 908, ground.width):
                    for ty in range(floor_y, floor_y+512, ground.height):
                        patch = ground.crop((0, 0, min(ground.width, 908-tx), min(ground.height, floor_y+512-ty)))
                        floors.paste(patch, (tx, ty))
            for i, e in enumerate(ready):
                body, _, shadow, anchor = layers[e['id']]
                assert body.size == (128, 128)
                x, y = 12+(i%6)*128, floor_y+(i//6)*128
                for layer in [shadow, body]:
                    floors.paste(layer, (x+64-anchor[0], y+112-anchor[1]), layer)
                fd.text((x+7, y+2), f"{e['number']} · {e['id']}", font=small, fill='#10171c' if fill == '#b4afa4' else '#eee5d3')
            for row in range(4):
                y = floor_y+row*128
                for layer in [player_shadow, player_body]:
                    floors.paste(layer, (844-player['foot_anchor'][0], y+112-player['foot_anchor'][1]), layer)
                fd.text((800, y+2), 'Joueur', font=small, fill='#10171c' if fill == '#b4afa4' else '#eee5d3')
            fd.text((18, top+564), 'Scène composée pour la revue · taille 1:1 · aucun acteur ajouté au jeu', font=small, fill='#b4c4c8')
        floors.save(ROOT/batch['native_floor_review'], optimize=True)
    path.write_text(json.dumps(batch,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(f'{len(ready)} nouveaux dessins et variantes ; 28 corrige a la stature du joueur.')


if __name__ == '__main__':
    main()
