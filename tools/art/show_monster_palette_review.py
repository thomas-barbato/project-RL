"""Current native-size comparison: original, orange-red, green, player."""
import json
import math
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from review_contact_shadows import contact_shadow

ROOT=Path(__file__).resolve().parents[2]
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07'
BANK=ROOT/'assets/sprites/campaign-monsters'
OUT=ROOT/'artifacts/monster-material-palettes-07'

def main():
    review=json.loads((REVIEW/'review.json').read_text(encoding='utf-8'))
    records=[r for r in review['records'] if r['number'] not in {144,150}]
    reference=json.loads((ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01/cultists-zombies-spirits-24.json').read_text(encoding='utf-8'))['player_reference']
    player=Image.open(ROOT/reference['source']).convert('RGBA');player_shadow,_=contact_shadow(player,reference['foot_anchor'])
    ground=Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0,0,240,155))
    bold=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',16)
    font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',13)
    small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    OUT.mkdir(parents=True,exist_ok=True);pages=[];placements=[]
    for start in range(0,len(records),16):
        group=records[start:start+16]; heights=[]
        for i in range(0,len(group),2):
            heights.append(max(202,max(Image.open(BANK/e['source']).getbbox()[3]-Image.open(BANK/e['source']).getbbox()[1] for e in group[i:i+2])+112))
        sheet=Image.new('RGB',(1360,104+sum(h+12 for h in heights)),'#10171c');draw=ImageDraw.Draw(sheet)
        draw.text((18,10),'Variantes de matières · rouge-orangé et vert · taille native',font=bold,fill='#e5d2a8')
        draw.text((18,38),'88 autres modèles · 176 nouvelles palettes · mêmes dessins, tailles, ombres et règles',font=font,fill='#c3cdd0')
        draw.text((18,62),'A : origine · C : rouge-orangé · D : vert · joueur à droite à la même échelle · sans zoom',font=small,fill='#b4c4c8')
        top=94
        for i,e in enumerate(group):
            if i and i%2==0: top+=heights[i//2-1]+12
            x=12+(i%2)*674;h=heights[i//2]
            draw.rectangle((x,top,x+662,top+h),fill='#1a242b',outline='#35434b')
            draw.text((x+12,top+8),f"{e['number']:03} · {e['code']} · {e['label']}",font=bold,fill='#e5d2a8')
            draw.text((x+12,top+34),f"Matière : {e['material']} · autres matières protégées",font=small,fill='#b4c4c8')
            floor=(x+10,top+62,x+652,top+h-10)
            for tx in range(floor[0],floor[2],ground.width):
                for ty in range(floor[1],floor[3],ground.height):
                    sheet.paste(ground.crop((0,0,min(ground.width,floor[2]-tx),min(ground.height,floor[3]-ty))),(tx,ty))
            base=Image.open(BANK/e['source']).convert('RGBA');shadow,_=contact_shadow(base,e['foot_anchor'])
            images=[('A · origine',base),('C · rouge-orangé',Image.open(ROOT/e['variants'][0]['file']).convert('RGBA')),('D · vert',Image.open(ROOT/e['variants'][1]['file']).convert('RGBA'))]
            baseline=top+h-32
            for center,(label,image) in zip([x+86,x+248,x+412],images):
                px,py=center-e['foot_anchor'][0],baseline-e['foot_anchor'][1]
                sheet.paste(shadow,(px,py),shadow);sheet.paste(image,(px,py),image)
                draw.text((center,baseline+5),label,anchor='mt',font=small,fill='#e5e2d2')
                placements.append(dict(number=e['number'],label=label,page=start//16+1,position=[px,py],scale=1))
            px,py=x+592-reference['foot_anchor'][0],baseline-reference['foot_anchor'][1]
            sheet.paste(player_shadow,(px,py),player_shadow);sheet.paste(player,(px,py),player)
            draw.text((x+592,baseline+5),'Joueur',anchor='mt',font=small,fill='#e5e2d2')
        path=OUT/f'palettes-{start//16+1:02}.png';sheet.save(path,optimize=True)
        pages.append(dict(file=path.relative_to(ROOT).as_posix(),width=sheet.width,height=sheet.height,numbers=[e['number'] for e in group]))
    links=' '.join(f'<a href="#p{i+1}">Planche {i+1}</a>' for i in range(len(pages)))
    images=''.join(f'<section id="p{i+1}"><img src="{Path(p["file"]).name}" width="{p["width"]}" height="{p["height"]}" alt="Variantes natives planche {i+1}"></section>' for i,p in enumerate(pages))
    (OUT/'index.html').write_text('<!doctype html><html lang="fr"><meta charset="utf-8"><title>Palettes du bestiaire</title><style>body{margin:0;background:#10171c;color:#dce4e4;font:14px Segoe UI,sans-serif}header{position:sticky;top:0;background:#10171cf5;padding:12px 18px;width:1324px;border-bottom:1px solid #35434b}h1{font-size:19px;margin:0 0 8px;color:#e5d2a8}a{color:#d5c39f;margin-right:18px}img{display:block;width:1360px;height:auto;max-width:none;image-rendering:pixelated}section{scroll-margin-top:78px}</style><header><h1>88 autres modèles · 176 variantes · affichage à 100 %</h1>'+links+'</header>'+images+'</html>\n',encoding='utf-8')
    (REVIEW/'native-preview-layout.json').write_text(json.dumps(dict(scale=1,actor_resize_operations=0,pages=pages,placements=placements,player_reference=reference),ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(f'{len(pages)} native sheets, {len(records)} models, {len(records)*2} new palettes.')

if __name__=='__main__': main()
