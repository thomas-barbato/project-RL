"""Render the current hand-authored place plan, without creating game art.

The small overview is a circulation diagram with no actor artwork. The native
view reuses the accepted player PNG byte-for-byte at 64 pixels per map cell.
"""
import hashlib
import html
import json
from collections import deque
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from review_contact_shadows import contact_shadow

ROOT=Path(__file__).resolve().parents[2]
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01'
OUT=ROOT/'artifacts/handcrafted-places-review-01'

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def write(path,data):path.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

def main():
    record=json.loads((REVIEW/'review.json').read_text(encoding='utf-8'))['records'][0]
    grid=(REVIEW/record['layout']).read_text(encoding='utf-8').splitlines()
    width,height=record['width_cells'],record['height_cells']
    assert len(grid)==height and all(len(row)==width for row in grid)
    assert set(''.join(grid))<={'#','.','+'}
    start=tuple(record['start']);seen=set();queue=deque([start])
    while queue:
        x,y=queue.popleft()
        if (x,y) in seen or not(0<=x<width and 0<=y<height) or grid[y][x]=='#':continue
        seen.add((x,y));queue.extend([(x+1,y),(x-1,y),(x,y+1),(x,y-1)])
    floor={(x,y) for y,row in enumerate(grid) for x,tile in enumerate(row) if tile!='#'}
    assert seen==floor
    reserved=[record['sector_connection'],record['player_at'],record['travel_reserved_at']]+[s['at'] for s in record['service_slots']]
    assert all(tuple(at) in seen for at in reserved)
    source=ROOT/record['player_reference']['source']
    assert sha(source)==record['player_reference']['sha256']
    player=Image.open(source).convert('RGBA');foot=record['player_reference']['foot_anchor']
    shadow,_=contact_shadow(player,foot)
    OUT.mkdir(parents=True,exist_ok=True)
    normal='C:/Windows/Fonts/segoeui.ttf';bold='C:/Windows/Fonts/segoeuib.ttf'
    outputs=[];placement=None
    for overview,cell in [(True,20),(False,64)]:
        margin=24;top=108;bottom=146
        image=Image.new('RGB',(width*cell+margin*2,height*cell+top+bottom),'#10171c')
        draw=ImageDraw.Draw(image)
        title=ImageFont.truetype(bold,19 if overview else 24)
        font=ImageFont.truetype(normal,13 if overview else 18)
        small=ImageFont.truetype(normal,12 if overview else 16)
        draw.text((margin,14),'PLACE-CAMP-01 · r01 · Premier camp',font=title,fill='#e5d2a8')
        draw.text((margin,48),'Plan proposé · circulation et emplacements à examiner',font=font,fill='#d4dfe4')
        draw.text((margin,74),'Schéma sans acteur' if overview else '64 px par case · joueur à taille native · décors à créer',font=small,fill='#b8cad2')
        for y,row in enumerate(grid):
            for x,tile in enumerate(row):
                px,py=margin+x*cell,top+y*cell
                color='#242e34' if tile=='#' else '#5b6468' if tile=='.' else '#819497'
                draw.rectangle((px,py,px+cell-1,py+cell-1),fill=color)
                if tile!='#':
                    draw.line((px,py,px+cell-1,py),fill='#697276',width=1)
                    draw.line((px,py,px,py+cell-1),fill='#697276',width=1)
        for s in record['service_slots']:
            x,y=s['at'];cx,cy=margin+(x+.5)*cell,top+(y+.5)*cell
            radius=9 if overview else 20
            draw.ellipse((cx-radius,cy-radius,cx+radius,cy+radius),fill='#b9d2d9',outline='#15272d',width=1)
            draw.text((cx,cy),s['code'][-1] if overview else s['code'],font=small,fill='#17282f',anchor='mm')
            tx,ty=s['room_label_at']
            label=('Info / missions' if s['code']=='S1' else s['label'])
            draw.text((margin+(tx+.5)*cell,top+(ty+.2)*cell),label,font=font,fill='#f2ead8',anchor='mt')
        for label,at in [('A',record['start']),('B',record['sector_connection']),('V',record['travel_reserved_at'])]:
            x,y=at;cx,cy=margin+(x+.5)*cell,top+(y+.5)*cell
            radius=9 if overview else 20
            draw.rectangle((cx-radius,cy-radius,cx+radius,cy+radius),fill='#e3c286')
            draw.text((cx,cy),label,font=small,fill='#29241c',anchor='mm')
        x,y=record['player_at'];cx,baseline=margin+(x+.5)*cell,top+(y+1)*cell
        if overview:
            draw.text((cx,baseline-cell*.45),'J',font=small,fill='#e5d2a8',anchor='mm')
        else:
            px,py=round(cx-foot[0]),round(baseline-foot[1])
            image.paste(shadow,(px,py),shadow);image.paste(player,(px,py),player)
            placement=dict(position=[px,py],source=record['player_reference']['source'],source_sha256=sha(source),actor_resize_operations=0,scale=1)
        below=top+height*cell+16
        labels=['A · Arrivée     B · Vers le secteur hostile     V · Voyage réservé',
                'S1 · Information / missions     S2 · Commerce' if not overview else '1 · Information / missions     2 · Commerce',
                'S3 · Soins     S4 · Coffre' if not overview else '3 · Soins     4 · Coffre',
                'Marqueurs = emplacements proposés ; aucun PNJ ou décor validé ici.',
                'Gabarit candidat : 28 × 18 cases. Plans et dessins se valident séparément.']
        for i,label in enumerate(labels):draw.text((margin,below+i*24),label,font=small,fill='#c7d3d9')
        path=OUT/('camp-01-overview.png' if overview else 'camp-01-native64.png')
        image.save(path,optimize=True)
        outputs.append(dict(file=path.relative_to(ROOT).as_posix(),sha256=sha(path),size=list(image.size),diagram_only=overview))
    native=Image.open(ROOT/outputs[1]['file']).convert('RGB')
    px,py=placement['position'];crop=native.crop((px,py,px+player.width,py+player.height))
    for source_pixel,actual in zip(player.get_flattened_data(),crop.get_flattened_data()):
        if source_pixel[3]==255:assert source_pixel[:3]==actual
    (OUT/'index.html').write_text('<!doctype html><html lang="fr"><meta charset="utf-8"><title>Premier camp — revue du plan</title><style>body{margin:0;background:#10171c;color:#dce4e4;font:15px Segoe UI,sans-serif}header{padding:16px 24px;max-width:1040px}h1{font-size:22px;color:#e5d2a8}a{color:#b9d3e9;margin-right:20px}p{line-height:1.5}section{padding:12px 0;scroll-margin-top:12px}h2{font-size:18px;margin-left:24px}img{display:block;max-width:none;width:auto;height:auto;image-rendering:pixelated}</style><header><h1>PLACE-CAMP-01 · r01 — plan à examiner</h1><p>Vue d’ensemble schématique, puis vue à 64 px par case avec le joueur à sa taille réelle. Les surfaces unies représentent le plan ; les décors définitifs restent à créer et à valider.</p><p><a href="#overview">Vue d’ensemble</a><a href="#native">Taille native</a></p></header><section id="overview"><h2>Vue d’ensemble — implantation</h2><img src="camp-01-overview.png" alt="Plan du camp : quatre emplacements de services autour d’un passage central, arrivée à gauche et accès au secteur à droite."></section><section id="native"><h2>Taille native — même plan avec le joueur</h2><img src="camp-01-native64.png" width="1840" height="1406" alt="Plan du camp à 64 pixels par case, joueur non redimensionné et emplacements proposés des services."></section></html>\n',encoding='utf-8')
    validation=dict(review_id='HANDCRAFTED-PLACES-01',place='PLACE-CAMP-01',revision='r01',
        hand_authored_layout_sha256=sha(REVIEW/record['layout']),walkable_cells=len(seen),all_floor_connected=True,
        all_reserved_positions_reachable=True,player_native_pixels_verified=True,player_placement=placement,
        proposed_dimensions=[width,height],dimensions_approved=False,layout_approved=False,art_approved=False,
        runtime_connected=False,new_gameplay_effects=False,outputs=outputs,
        scope='Schematic review only: no game renderer, NPC behavior, generated campaign map or final art validated.')
    write(REVIEW/'validation.json',validation)
    print(json.dumps({k:v for k,v in validation.items() if k not in ['outputs','player_placement']},ensure_ascii=False))

if __name__=='__main__':main()
