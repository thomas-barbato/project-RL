"""Prepare frozen blue material-map proposals; accepted runtime stays unchanged."""
import argparse
import colorsys
import hashlib
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
from review_contact_shadows import contact_shadow

ROOT=Path(__file__).resolve().parents[2]
BANK=ROOT/'assets/sprites/campaign-monsters'
PREVIOUS=ROOT/'docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07'
REVIEW=ROOT/'docs/reviews/refonte-histoire-2026-10-16/monster-blue-palettes-08'
SOURCE=ROOT/'assets/source-art/refonte-histoire-2026-10-16/monster-blue-palettes-08'
OUT=ROOT/'artifacts/monster-blue-palettes-08'
# Native A/B material audit: these 39 models already have a blue option.
EXISTING_A={31,36,38,78,139,145,156,157,158}
EXISTING_B={30,34,37,39,60,61,62,63,67,68,69,71,77,79,82,83,84,85,86,87,88,90,93,140,144,148,151,154,161}

def read(path): return json.loads(path.read_text(encoding='utf-8'))
def write(path,data):
    path.parent.mkdir(parents=True,exist_ok=True)
    path.write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def target(rgb,cloth,number):
    _,light,_=colorsys.rgb_to_hls(*(c/255 for c in rgb))
    color=colorsys.hls_to_rgb((207+number%5*3)/360,light,.27 if cloth else .46)
    old=sum(c*w for c,w in zip(rgb,(.2126,.7152,.0722)))
    new=sum(c*255*w for c,w in zip(color,(.2126,.7152,.0722)))
    factor=min(1,old/max(1,new))
    return [round(c*255*factor) for c in color]

def author():
    path=SOURCE/'material-palettes.json'
    if path.exists(): raise ValueError('Frozen blue proposals already exist; use build or a distinct revision.')
    rows=[]
    bank=read(BANK/'manifest.json')
    for old in read(PREVIOUS/'review.json')['records']:
        n=old['number'];existing=n in EXISTING_A|EXISTING_B|{150}
        if existing:
            index=0 if n in EXISTING_A else 3 if n==150 else 1
            entry=(next(e for e in old['variants'] if e['variant_index']==3) if n==150 else
                   next(e for e in bank['entries'] if e['number']==n and e['variant']==index))
            file=entry['file'] if n==150 else (BANK/entry['tile']).relative_to(ROOT).as_posix()
            row=dict(number=n,code=old['code'],label=old['label'],material=old['material'],
                source=file,source_sha256=sha(ROOT/file),foot_anchor=entry['foot_anchor'],
                existing_blue=True,blue_variant=chr(65+index),blue_file=file)
        else:
            file=(BANK/old['source']).relative_to(ROOT).as_posix()
            row=dict(number=n,code=old['code'],label=old['label'],material=old['material'],
                source=file,source_sha256=sha(ROOT/file),foot_anchor=old['foot_anchor'],side=old['side'],
                atlas_index=old['atlas_index'],existing_blue=False,blue_variant='E',variant_index=4,
                blue_file=(SOURCE/'native64'/f'{n:03}-palette-4.png').relative_to(ROOT).as_posix(),
                colors=[dict(source=c,target=target(c,'robe' in old['material'],n)) for c in old['selected_rgb']])
        rows.append(row)
    assert len(rows)==90 and sum(not r['existing_blue'] for r in rows)==51
    write(path,dict(review_id='MONSTER-BLUE-PALETTES-08',status='native_review_pending',
        visual_approved=False,runtime_connected=False,date='2026-10-17',records=rows))

def build(check):
    review=read(SOURCE/'material-palettes.json')
    changed=protected=0
    for row in review['records']:
        path=ROOT/row['source'];assert sha(path)==row['source_sha256']
        if row['existing_blue']: continue
        image=Image.open(path).convert('RGBA');s=row['side'];i=row['atlas_index']
        atlas=Image.open(BANK/f'materials-{s}.png').convert('RGBA')
        mask=atlas.crop((i%8*s,i//8*s,i%8*s+s,i//8*s+s))
        mapping={tuple(c['source']):tuple(c['target']) for c in row['colors']}
        before=list(image.get_flattened_data())
        after=[(*mapping.get(p[:3],p[:3]),p[3]) if m[0] and p[3] else p for p,m in zip(before,mask.get_flattened_data())]
        result=image.copy();result.putdata(after)
        assert result.getchannel('A').tobytes()==image.getchannel('A').tobytes()
        assert result.getbbox()==image.getbbox()
        for p,q,m in zip(before,after,mask.get_flattened_data()):
            if not m[0]: assert p==q;protected+=int(p[3]!=0)
            changed+=int(p!=q)
        output=ROOT/row['blue_file']
        if check: assert Image.open(output).convert('RGBA').tobytes()==result.tobytes()
        else:
            output.parent.mkdir(parents=True,exist_ok=True);result.save(output,optimize=True)
        row.update(blue_sha256=sha(output),occupied=list(result.getbbox()),canvas=s,scale=1)
    review.update(new_blue_palettes=51,existing_blue_models=39,numbered_models=90,
        actor_resize_operations=0,alpha_geometry_anchors_and_protected_materials_preserved=True,
        protected_opaque_pixels=protected,changed_material_pixels=changed)
    code='// Generated from frozen source-art/monster-blue-palettes-08/material-palettes.json.\n'
    code+=f'pub const BLUE_PALETTES_APPROVED: bool = {str(review["visual_approved"]).lower()};\n'
    code+='pub const BLUE_MATERIAL_PALETTES: &[MaterialPalette] = &[\n'
    if review['visual_approved']:
        assert review['acceptance']['answer']=='Valider et intégrer les 51 bleues'
        for row in review['records']:
            if row['existing_blue']: continue
            packed=lambda c:(c[0]<<16)|(c[1]<<8)|c[2]
            colors=', '.join(f'(0x{packed(c["source"]):06x}, 0x{packed(c["target"]):06x})' for c in row['colors'])
            code+=f'    MaterialPalette {{ number: {row["number"]}, variant: 4, colors: &[{colors}] }},\n'
    code+='];\n'
    path=BANK/'blue-material-palettes.rs'
    if check: assert path.read_text(encoding='utf-8')==code
    else: path.write_text(code,encoding='utf-8')
    if not check: write(REVIEW/'review.json',review)
    return review

def preview(review):
    reference=read(ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01/cultists-zombies-spirits-24.json')['player_reference']
    player=Image.open(ROOT/reference['source']).convert('RGBA');ps,_=contact_shadow(player,reference['foot_anchor'])
    ground=Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/comic64-reference-review-04/five-characters-at-64.png').convert('RGB').crop((0,0,240,155))
    bold=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',16)
    small=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',12)
    OUT.mkdir(parents=True,exist_ok=True);pages=[];placements=[]
    records=sorted(review['records'],key=lambda r:(r['existing_blue'],r['number']))
    for start in range(0,len(records),16):
        group=records[start:start+16]
        heights=[max(202,max(Image.open(ROOT/r['source']).getbbox()[3]-Image.open(ROOT/r['source']).getbbox()[1] for r in group[i:i+2])+112) for i in range(0,len(group),2)]
        sheet=Image.new('RGB',(1360,104+sum(h+12 for h in heights)),'#10171c');draw=ImageDraw.Draw(sheet)
        draw.text((18,10),'Bleus pour le gel · palettes à taille native',font=bold,fill='#b9d3e9')
        draw.text((18,38),'51 nouvelles palettes E '+('validées' if review['visual_approved'] else 'à examiner')+' · 39 bleus déjà présents rappelés après les nouveautés',font=small,fill='#c3cdd0')
        draw.text((18,62),'Dessins, tailles et matériaux protégés identiques · joueur à la même échelle · aucun nouveau pouvoir',font=small,fill='#b4c4c8')
        top=94
        for i,row in enumerate(group):
            if i and i%2==0: top+=heights[i//2-1]+12
            x=12+i%2*674;h=heights[i//2];baseline=top+h-32
            draw.rectangle((x,top,x+662,top+h),fill='#1a242b',outline='#35434b')
            draw.text((x+12,top+8),f"{row['number']:03} · {row['code']} · {row['label']}",font=bold,fill='#e5d2a8')
            draw.text((x+12,top+34),'Bleu existant conservé' if row['existing_blue'] else 'Nouveau bleu · '+('validé' if review['visual_approved'] else 'en attente de validation'),font=small,fill='#b9d3e9')
            floor=(x+10,top+62,x+652,top+h-10)
            for tx in range(floor[0],floor[2],ground.width):
                for ty in range(floor[1],floor[3],ground.height):
                    sheet.paste(ground.crop((0,0,min(ground.width,floor[2]-tx),min(ground.height,floor[3]-ty))),(tx,ty))
            for center,file,label in [(x+128,row['source'],'Origine'),(x+366,row['blue_file'],row['blue_variant']+' · bleu'+(' existant' if row['existing_blue'] else ' proposé'))]:
                image=Image.open(ROOT/file).convert('RGBA');shadow,_=contact_shadow(image,row['foot_anchor'])
                px,py=center-row['foot_anchor'][0],baseline-row['foot_anchor'][1]
                sheet.paste(shadow,(px,py),shadow);sheet.paste(image,(px,py),image)
                draw.text((center,baseline+5),label,anchor='mt',font=small,fill='#dce5ed')
                placements.append(dict(number=row['number'],file=file,page=start//16+1,position=[px,py],scale=1))
            px,py=x+592-reference['foot_anchor'][0],baseline-reference['foot_anchor'][1]
            sheet.paste(ps,(px,py),ps);sheet.paste(player,(px,py),player)
            draw.text((x+592,baseline+5),'Joueur',anchor='mt',font=small,fill='#e5e2d2')
        path=OUT/f'bleus-{start//16+1:02}.png';sheet.save(path,optimize=True)
        pages.append(dict(file=path.relative_to(ROOT).as_posix(),sha256=sha(path),size=list(sheet.size)))
    links=' '.join(f'<a href="#p{i+1}">Planche {i+1}</a>' for i in range(len(pages)))
    images=''.join(f'<section id="p{i+1}"><img src="{Path(p["file"]).name}" alt="Palettes bleues planche {i+1}"></section>' for i,p in enumerate(pages))
    status='validés' if review['visual_approved'] else 'à valider'
    (OUT/'index.html').write_text('<!doctype html><html lang="fr"><meta charset="utf-8"><title>Bleus du bestiaire</title><style>body{margin:0;background:#10171c;color:#dce4e4;font:14px Segoe UI,sans-serif}header{position:sticky;top:0;background:#10171cf5;padding:12px 18px;width:1324px;border-bottom:1px solid #35434b}h1{font-size:19px;margin:0 0 8px;color:#b9d3e9}a{color:#b9d3e9;margin-right:18px}img{display:block;width:1360px;height:auto;max-width:none;image-rendering:pixelated}section{scroll-margin-top:78px}</style><header><h1>51 nouveaux bleus '+status+' · 39 bleus déjà présents · taille native</h1>'+links+'</header>'+images+'</html>\n',encoding='utf-8')
    write(REVIEW/'native-preview-layout.json',dict(scale=1,actor_resize_operations=0,pages=pages,placements=placements,player_reference=reference))

def validate_preview():
    layout=read(REVIEW/'native-preview-layout.json')
    assert layout['actor_resize_operations']==0 and layout['scale']==1
    pages=[]
    for p in layout['pages']:
        assert sha(ROOT/p['file'])==p['sha256']
        pages.append(Image.open(ROOT/p['file']).convert('RGB'))
    for p in layout['placements']:
        source=Image.open(ROOT/p['file']).convert('RGBA');x,y=p['position']
        crop=pages[p['page']-1].crop((x,y,x+source.width,y+source.height))
        for before,after in zip(source.get_flattened_data(),crop.get_flattened_data()):
            if before[3]==255: assert before[:3]==after,p

def publish(review):
    assert review['visual_approved']
    path=ROOT/'docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01/monster-candidate-list.json'
    listing=read(path);records={r['number']:r for r in listing['records']}
    additions=[]
    for row in review['records']:
        if row['existing_blue']: continue
        record=records[row['number']]
        variant=dict(variant='E',variant_index=4,label='Bleu',file=row['blue_file'],sha256=row['blue_sha256'],
            cell=64,canvas=row['canvas'],foot_anchor=row['foot_anchor'],occupied=row['occupied'],
            runtime_palette=True,visual_approved=True,palette_review_id=review['review_id'])
        record['variants']=[v for v in record['variants'] if v['variant']!='E']+[variant]
        record['variant_files']=[v['file'] for v in record['variants']]
        record['blue_palette_review_id']=review['review_id'];record['blue_palette_review_status']='accepted'
        additions.append(dict(number=row['number'],variant=4,preview=row['blue_file'],preview_relative_to='workspace',
            source_variant=0,implementation='src/game_art_campaign.rs',material_map='blue-material-palettes.rs',
            material_mask=f'materials-{row["side"]}.png',
            authorization='docs/reviews/refonte-histoire-2026-10-16/monster-blue-palettes-08/review.json',
            available_since_generation=152))
    listing['current_blue_palette_review']='../monster-blue-palettes-08/review.json';write(path,listing)
    path=BANK/'manifest.json';manifest=read(path)
    manifest['runtime_blue_material_palettes']=additions;write(path,manifest)

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--author',action='store_true');parser.add_argument('--check',action='store_true');parser.add_argument('--publish-catalogue',action='store_true')
    args=parser.parse_args()
    if args.author: author()
    review=build(args.check)
    if not args.check: preview(review)
    validate_preview()
    if args.publish_catalogue: publish(review)
    print('51 blue material maps, 39 existing blues, six native sheets; geometry and protected pixels verified. Approved: '+str(review['visual_approved']))
