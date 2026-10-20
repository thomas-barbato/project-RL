"""Native scenery catalogue, provenance and alpha export checks for U01 r05."""
import html
import json
from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import build_street_refined_review as scene

ROOT,ART,OUT,DOC=scene.ROOT,scene.ART,scene.OUT,scene.DOC


def build(rows=None,revision='r05'):
    acceptance_file=DOC/'warehouse-south-wall-acceptance-r03.json'
    acceptance=scene.read(acceptance_file) if revision=='r06' and acceptance_file.exists() else {}
    if rows is None:
        rows=scene.read(ART/'module-input.json')
        names=['Générateur','Benne','Végétation','Petit bâtiment','Bâtiment long',
               'Bâtiment bleu','Garage','Bâtiment en retrait']
        rows=[dict(id=r['id'],title=n) for r,n in zip(rows,names)]
        rows += [dict(id=k,title=n) for k,n in [
            ('warehouse','Entrepôt en L'),('collapsed-wall','Mur effondré'),
            ('fallen-light','Lampadaire abattu'),('north-gateway','Portail nord'),
            ('street-post','Luminaire de rue')]]
        rows += [dict(id=f'roof-{i}',title=f'Toit séparé {i+1}') for i in range(4)]
    records=[]
    for index,row in enumerate(rows,1):
        ident=row['id'];native=ROOT/row['file'] if 'file' in row else ART/(ident+'-native.png')
        source=ROOT/row['source'] if 'source' in row else ART/('WAREHOUSE-U01-FIX-source.png' if ident=='warehouse' else ident+'-source.png')
        im=Image.open(native).convert('RGBA');raw=Image.open(source).convert('RGBA')
        bb=raw.getchannel('A').point(lambda a:255 if a>128 else 0).getbbox()
        assert bb and bb[0]>0 and bb[1]>0 and bb[2]<raw.width and bb[3]<raw.height,source
        lo,hi=im.getchannel('A').getextrema()
        assert lo==0 and hi>=240,native
        records.append(dict(row,code=f'KIT-U01-{index:03}',file=scene.rel(native),source=scene.rel(source),
            native_size=list(im.size),source_size=list(raw.size),source_object_bounds=bb,
            source_margins_verified=True,source_method='single_isolated_imagegen_asset',
            anchor_px=[im.width//2,im.height],status='visual_review_pending',
            source_sha256=scene.sha(source),sha256=scene.sha(native)))
        if ident=='warehouse' and acceptance.get('sha256')==records[-1]['sha256'] and acceptance.get('source_sha256')==records[-1]['source_sha256']:
            records[-1]['status']='visual_accepted'
            records[-1]['accepted_review']='MUR-SUD-03'
    scene.write(ART/'scenery-library.json',records)
    bold=ImageFont.truetype('C:/Windows/Fonts/segoeuib.ttf',19)
    normal=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',16)
    player=Image.open(ROOT/'assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png').convert('RGBA')
    heights=[max(Image.open(ROOT/r['file']).height for r in records[i:i+2])+100 for i in range(0,len(records),2)]
    board=Image.new('RGBA',(1440,70+sum(heights)),(17,26,31,255));d=ImageDraw.Draw(board)
    d.text((20,15),'U01 '+revision+' · sources indépendantes · un pixel pour un pixel',font=bold,fill='#dec898')
    y=60
    cards=[]
    for i,rec in enumerate(records):
        x=(i%2)*720+12;h=heights[i//2]
        d.rectangle((x,y,x+696,y+h-8),fill='#28333a',outline='#5c696c')
        d.text((x+12,y+8),rec['code']+' · '+rec['title'],font=bold,fill='#e7cd99')
        w,ih=rec['native_size'];d.text((x+12,y+35),f'{w} × {ih} px',font=normal,fill='#cad4ce')
        im=Image.open(ROOT/rec['file']).convert('RGBA')
        board.alpha_composite(im,(x+18,y+h-24-ih));board.alpha_composite(player,(x+620,y+h-24-player.height))
        escaped=html.escape(rec['title']);url='../../'+rec['file']+'?v='+rec['sha256'][:12]
        note=''
        if revision=='r06' and rec['id']=='warehouse':
            note='<p><b>MUR-SUD-03</b> · façade sud à la même hauteur apparente que celle du garage ; coins aux extrémités et absence de gravats conservés.</p>'
            if rec['status']=='visual_accepted':note+='<p>Visuel validé le 19 octobre 2026.</p>'
            proof=Image.new('RGBA',(w+40,ih+100),(32,43,50,255))
            pd=ImageDraw.Draw(proof)
            pd.text((20,12),rec['code']+' / MUR-SUD-03',font=bold,fill='#e7cd99')
            pd.text((20,40),'Hauteur de façade corrigée · taille native',font=normal,fill='#cad4ce')
            proof.alpha_composite(im,(20,80))
            proof.convert('RGB').save(OUT/'entrepot-mur-sud-03.png')
        cards.append(f'<article id="{rec["code"]}"><h2>{rec["code"]} · {escaped}</h2><p>{w} × {ih} px · source individuelle transparente</p>{note}<div class="sample"><img src="{url}" width="{w}" height="{ih}" alt="{escaped}"></div></article>')
        if i%2 or i==len(records)-1:y+=h
    board.convert('RGB').save(OUT/'modules-native.png')
    # Retained assets are listed separately, not relabelled as newly redrawn.
    retained={}
    for row in scene.read(ART/'module-placements.json'):
        if row['file'] not in {r['file'] for r in records}:retained[row['file']]=row['id']
    for row in scene.read(scene.WAR/'new-buildings.json'):retained[row['file']]=row['code']
    catalogue=scene.read(ART/'decor-catalogue.json')
    for row in scene.read(ART/'decor-placements.json'):
        r=catalogue[row['asset']]
        if r['file'] not in {r['file'] for r in records}:retained[r['file']]=r['code']
    retained_html=[]
    for file,code in retained.items():
        im=Image.open(ROOT/file);w,h=im.size
        version=scene.sha(ROOT/file)[:12]
        retained_html.append(f'<article><h2>{html.escape(code)}</h2><p>Ressource existante réutilisée · {w} × {h} px</p><div class="sample"><img src="../../{file}?v={version}" width="{w}" height="{h}" alt="{html.escape(code)}"></div></article>')
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><title>U01 · bibliothèque des décors</title><style>body{background:#111a1f;color:#d7dfda;font:17px Segoe UI;margin:24px}h1,h2{color:#dec898}h2{font-size:19px}a{color:#e5c693}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(680px,1fr));gap:18px}article{border:1px solid #52636a;padding:14px;overflow:auto}.sample{padding:18px;min-height:140px;background-color:#29343b;background-image:linear-gradient(45deg,#ffffff04 25%,transparent 25%,transparent 75%,#ffffff04 75%),linear-gradient(45deg,#ffffff04 25%,transparent 25%,transparent 75%,#ffffff04 75%);background-size:32px 32px;background-position:0 0,16px 16px}img{display:block;max-width:none}p{max-width:1100px;line-height:1.5}</style>
<h1>U01 r05 · bibliothèque de décors</h1><p><a href="index.html">Retour à la scène assemblée</a></p><p>Éléments repris : une source indépendante par image, puis conditionnement à l’échelle du jeu sans déformation. Les originaux sont conservés. Les images ci-dessous sont affichées à taille native, sans agrandissement. Les toits, boîtiers et émetteurs gardent leurs couches distinctes. Ces nouveaux pixels restent à valider.</p><div class="grid">'''
    page+=''.join(cards)+'</div><h1>Ressources existantes réutilisées</h1><p>Ces éléments conservent leur provenance ; ils ne sont pas présentés comme de nouveaux dessins individuels de cette révision.</p><div class="grid">'+''.join(retained_html)+'</div></html>'
    page=page.replace('U01 r05','U01 '+revision)
    if any(r['status']=='visual_accepted' for r in records):
        page=page.replace('Ces nouveaux pixels restent à valider.','Les validations acquises sont indiquées sur les ressources concernées ; les autres dessins restent en revue.')
    if revision=='r06':
        page=page.replace('<div class="grid">',
            '<p><b>Corrections des contours :</b> <a href="#KIT-U01-009">entrepôt</a>, <a href="#KIT-U01-018">logement</a>, <a href="#KIT-U01-019">boutique</a>. Les accessoires extérieurs sont placés séparément. Conduits en cuivre terni, rouille et acier gris, avec certains réseaux rouges conservés.</p><div class="grid">',1)
    (OUT/'bibliotheque.html').write_text(page,encoding='utf-8')
    scene.write(DOC/('scenery-library-'+revision+'.json'),dict(review='DECORS-PRODUCTION-01 / MAP-U01-EXP-01-'+revision,
        isolated_replacements=records,reused_assets=retained,
        accepted_south_scene_retained=True,source_method_applies_to_all_future_scenery=True,
        notes='The approved southern base is retained; this revision does not claim to redraw that historical scene.'))
    print(json.dumps(dict(isolated_replacements=len(records),reused=len(retained),native_scale=64)))


if __name__=='__main__':
    current=ART.parent/'u01-expansion-r06'
    if (current/'scenery-library.json').exists():
        ART=current
        build(rows=scene.read(ART/'scenery-library.json'),revision='r06')
    else:build()
