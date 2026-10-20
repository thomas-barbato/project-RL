"""Publish the current native gallery and verify embedded rendering coverage."""
import json,math,html,shutil
from pathlib import Path
from PIL import Image,ImageDraw,ImageFont
from art_finish_completion import ROOT,ART,DOC,OUT
from art_finish_batch_02 import sha,write

def native_worn_boards():
    engine=OUT/'completion-engine'
    for path in engine.glob('worn-*.png'):
        src=Image.open(path).convert('RGB');page=Image.new('RGB',(960,60+4*158),'#11191d');d=ImageDraw.Draw(page);font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',18)
        d.text((16,14),path.stem+' · mains et palettes · taille native',font=font,fill='#decca7')
        for i in range(23):
            x=12+i%6*318;y=64+i//6*244;xx=i%6*160;yy=60+i//6*158
            page.paste(src.crop((x+89,y+110,x+217,y+238)),(xx+16,yy+22));d.text((xx+12,yy),str(i+1),font=font,fill='#decca7')
        page.save(OUT/f'completion-{path.stem}-native.png')
    rigs=json.loads((DOC/'worn-rigs.json').read_text('utf-8'))['outfits']
    for start in range(0,len(rigs),7):
        part=rigs[start:start+7];page=Image.new('RGB',(640,50+len(part)*160),'#11191d');d=ImageDraw.Draw(page);font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',17)
        d.text((12,12),'Origine · rouge · bleu · gris — matériaux protégés',font=font,fill='#decca7')
        for k,r in enumerate(part):
            src=Image.open(engine/f'worn-{r["outfit"]}.png');d.text((10,k*160+45),f'{r["code"]} / {r["outfit"]}',font=font,fill='#decca7')
            for j,i in enumerate([12,13,14,18]):
                x=12+i%6*318+89;y=64+i//6*244+110;page.paste(src.crop((x,y,x+128,y+128)),(j*160+16,k*160+75))
        page.save(OUT/f'completion-worn-palettes-{start//7+1}.png')
    for path in sorted(engine.glob('weapons-*.png')):
        index=int(path.stem.split('-')[-1]);count=min(24,113-index*24);src=Image.open(path).convert('RGB');page=Image.new('RGB',(960,60+4*158),'#11191d');d=ImageDraw.Draw(page);font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',18)
        d.text((16,14),'Modèles d’armes · prises actuelles · taille native',font=font,fill='#decca7')
        for i in range(count):
            x=12+i%6*318+89;y=64+i//6*244+110;xx=i%6*160;yy=60+i//6*158;page.paste(src.crop((x,y,x+128,y+128)),(xx+16,yy+22));d.text((xx+12,yy),str(index*24+i),font=font,fill='#decca7')
        page.save(OUT/f'completion-{path.stem}-native.png')
    page=Image.new('RGB',(800,520),'#11191d');d=ImageDraw.Draw(page);font=ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',16)
    d.text((16,14),'Icônes actuelles des tenues · 64 px',font=font,fill='#decca7')
    icons=[p for p in sorted((ROOT/'assets/sprites/art-finish-completion-06/worn').glob('icon-*.png')) if '-mask' not in p.stem]
    for i,p in enumerate(icons):
        x=i%5*160;y=60+i//5*115;im=Image.open(p).convert('RGBA');page.paste(im,(x+48,y+25),im);d.text((x+20,y),p.stem,font=font,fill='#decca7')
    page.save(OUT/'completion-outfit-icons-native.png')
    sheets=[Image.open(p).convert('RGB') for p in sorted(OUT.glob('completion-palettes-*.png'))]
    full=Image.new('RGB',(1920,sum(p.height for p in sheets)),'#11191d');top=0
    for sheet in sheets:full.paste(sheet,(0,top));top+=sheet.height
    full.save(OUT/'completion-bestiaire-native.png')

def verify():
    data=json.loads((DOC/'exports.json').read_text('utf-8'));rows=data['exports'];assert len(rows)==471 and not data['missing']
    old=json.loads((DOC/'production-before.json').read_text('utf-8'));assert all(sha(ROOT/p)==s for p,s in old.items())
    engine=OUT/'completion-engine';checks=json.loads((engine/'checks.json').read_text('utf-8'));native=[];page=None
    for i,r in enumerate(rows):
        if i%24==0:page=Image.open(engine/f'native-{i//24:02}.png').convert('RGB')
        file=ROOT/r['file'];im=Image.open(file).convert('RGBA');assert sha(file)==r['sha256'];base=Image.open(ROOT/next(v['file'] for v in rows if v['number']==r['number'] and v['variant']==0)).convert('RGBA');assert im.getchannel('A').tobytes()==base.getchannel('A').tobytes()
        x=12+(i%24)%6*318;y=64+(i%24)//6*244;ox=x+153-r['foot_anchor'][0];oy=y+206-r['foot_anchor'][1]
        compared=bad=0;delta=0
        for sy in range(im.height):
            for sx in range(im.width):
                p=im.getpixel((sx,sy))
                if p[3]!=255:continue
                q=page.getpixel((ox+sx,oy+sy));diff=max(abs(a-b) for a,b in zip(p[:3],q));compared+=1;bad+=diff>1;delta=max(delta,diff)
        assert compared and not bad,(r['number'],r['variant'],compared,bad,delta)
        native.append(dict(number=r['number'],variant=r['variant'],opaque_pixels=compared,max_channel_error=delta,pixels_over_tolerance=bad))
    assert len(list(engine.glob('worn-*.png')))==19
    assert len(list(engine.glob('weapons-*.png')))==5
    native_worn_boards()
    for label in ['cultists','hybrids','zombies','spirits','necromancers','worms','wings','materials']:
        source=engine/f'{label}.png';Image.open(source).crop((334,78,1552,782)).save(OUT/f'completion-{label}-viewport.png')
    write(DOC/'integration-checks.json',dict(review='ART-FINISH-COMPLETE-06',status='engine_verified_visual_review_in_progress',previous_resources_unchanged=len(old),native_appearances=471,native_gpu_pages=20,opaque_pixel_checks=native,real_lighting_scenes=checks['real_lighting_scenes'],worn_outfits=19,worn_samples=19*23,additional_weapon_samples=113,current_icons=19,gameplay_fingerprints_unchanged=checks['gameplay_fingerprints_unchanged'],source_art_not_runtime_dependency=True))
    print('Verified:',len(rows),'native appearances;',len(old),'previous resources unchanged; 19 worn outfits / 113 weapon models')

def publish():
    rows=json.loads((DOC/'exports.json').read_text('utf-8'))['exports'];models=sorted({r['number'] for r in rows});parts=[]
    h=html.escape
    parts.append('''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-06 · bestiaire et équipements</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.5}h1,h2,h3{color:#decca7}p{max-width:1200px}a{color:#dfc58e}nav{display:flex;gap:20px;flex-wrap:wrap}.scroll{overflow:auto;max-width:100%;border:1px solid #43514e}img{max-width:none;display:block}section{margin:36px 0}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(700px,1fr));gap:14px}.card{background:#1b272d;border:1px solid #43514e;padding:12px;overflow:auto}.card h3{margin:0}.variants{display:flex;min-height:190px}.specimen{position:relative;flex:0 0 136px;height:190px;background:#343d3b;margin:8px 4px 0 0;overflow:hidden}.specimen img{position:absolute;pointer-events:none}.specimen span{position:absolute;left:8px;top:5px;color:#becac3}.muted{color:#a9bab3}button,select,input{font:inherit;color:#decca7;background:#1b272d;border:1px solid #6f8278;padding:7px}details{margin:16px 0}summary{cursor:pointer;color:#decca7}</style>
<h1>ART-FINISH-06 · finition du bestiaire et des équipements</h1>
<p>119 dessins de monstres refaits, un modèle conservé après refus de génération. Les 129 modèles existants et leurs 471 apparences sont raccordés au jeu. Contrôle et validation délégués par le joueur, à taille native et dans les lumières réelles de la rue.</p>
<p>Les 19 tenues conservent désormais leur dessin lors des changements d’armes et de mains. Les silhouettes d’équipement sont reprises dans leurs 19 icônes. Les portraits de création déjà acceptés sont conservés. Les 13 lanceurs de sorts réservés attendent toujours leurs pouvoirs ; les boss de secteur seront faits plus tard.</p>
<nav><a href="#scene">Dans la rue</a><a href="#catalogue">129 modèles et palettes</a><a href="#worn">Mains et équipements</a><a href="#furniture">Audit du mobilier</a><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-completion-06/README.md">Bilan durable</a></nav>
<section id="scene"><h2>Dans le moteur du jeu</h2><p>Figurants dessinés pour le contrôle ; aucune créature supplémentaire n’est ajoutée à la partie. Les images gardent leur taille native.</p>''')
    for label,title in [('cultists','Cultistes'),('hybrids','Possédés et hybrides'),('zombies','Zombies'),('spirits','Esprits'),('necromancers','Nécromanciens'),('worms','Vers et larves'),('wings','Démons ailés'),('materials','Peau et vêtements distincts')]:
        parts.append(f'<details {"open" if label=="wings" else ""}><summary>{title}</summary><div class="scroll"><img src="completion-{label}-viewport.png" width="1218" height="704" loading="lazy" alt="{title} dans les lumières de la rue"></div></details>')
    full=Image.open(OUT/'completion-bestiaire-native.png')
    parts.append(f'</section><section id="catalogue"><h2>Catalogue complet · échelle native</h2><p>Les codes identifient les dessins ; ils ne changent pas les noms affichés aux joueurs. Les petites créatures restent petites. A–E désignent les palettes existantes. FIN-106 conserve son dessin précédent.</p><details><summary>Grande planche réunissant les 129 modèles et leurs couleurs</summary><p><a href="completion-bestiaire-native.png" download>Enregistrer la planche complète</a></p><div class="scroll"><img src="completion-bestiaire-native.png" width="{full.width}" height="{full.height}" loading="lazy" alt="Bestiaire complet à taille de jeu, noms et codes"></div></details><label>Rechercher un nom ou un code <input id="search" type="search"></label><div class="grid">')
    for number in models:
        rr=[r for r in rows if r['number']==number];r=rr[0];label=f'{r["code"]} · {number:03} · {r["label"]}';parts.append(f'<article class="card" data-search="{h(label.casefold())}"><h3>{h(label)}</h3>')
        if r['role']=='ReservedCaster':parts.append('<p class="muted">Visuel terminé · pouvoirs encore réservés</p>')
        parts.append('<div class="variants">')
        for r in rr:
            side=r['side'];foot=r['foot_anchor'];url='../../'+r['file'];parts.append(f'<div class="specimen"><span>{chr(65+r["variant"])}</span><img src="{url}" width="{side}" height="{side}" loading="lazy" style="left:{68-foot[0]}px;top:{170-foot[1]}px" alt="{h(label)} palette {chr(65+r["variant"])}"></div>')
        parts.append('</div></article>')
    parts.append('</div></section><section id="worn"><h2>Tenues, mains et palettes</h2><p>1 : sans arme ; 2–7 : armes à une et deux mains ; 8–12 : main gauche et paires ; 13–21 : couleurs de tenue ; 22–23 : couleurs indépendantes des armes. La tête, les jambes et les pieds restent ceux de la tenue. Les formes d’armes déjà lisibles à 64 px sont conservées.</p>')
    for outfit in [200,212,4,6,15,201,202,203,204,213,214,215,216,208,209,210,211,217,218]:parts.append(f'<details><summary>Tenue {outfit} · 23 configurations</summary><div class="scroll"><img src="completion-worn-{outfit}-native.png" width="960" height="692" loading="lazy" alt="Tenue {outfit} à taille native"></div></details>')
    for page in range(5):parts.append(f'<details><summary>Modèles d’armes · planche {page+1}/5</summary><div class="scroll"><img src="completion-weapons-{page:02}-native.png" width="960" height="692" loading="lazy" alt="Modèles d’armes à taille native, planche {page+1}"></div></details>')
    parts.append('<details><summary>Icônes des tenues en inventaire</summary><img src="completion-outfit-icons-native.png" width="800" height="520" loading="lazy" alt="Les 19 icônes de tenues actuelles à taille native"></details><details><summary>PNJ du camp conservés après comparaison</summary><img src="completion-npc-control.png" width="960" height="220" loading="lazy" alt="Quatre PNJ du camp, personnage et cultiste actuels"></details>')
    parts.append('</section><section id="furniture"><h2>Mobilier · audit terminé</h2><p>Les lits des logements sont surdimensionnés. Le lit du camp et ses autres meubles sont cohérents avec les humains actuels. Les cartes n’ont pas été redimensionnées pendant cette refonte. Le remplacement des deux lits doit se faire avec des objets isolés, puis un contrôle de l’emprise au sol.</p>')
    for name,title in [('camp-native','Mobilier du camp'),('ROOF-U01-01A','Premier logement'),('ROOF-U01-02A','Logement nord'),('ROOF-U01-02B','Boutique')]:
        im=Image.open(OUT/f'furniture-audit/{name}.png');parts.append(f'<details><summary>{title}</summary><div class="scroll"><img src="furniture-audit/{name}.png" width="{im.width}" height="{im.height}" loading="lazy" alt="{title}, audit natif"></div></details>')
    parts.append('''<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-completion-06/AUDIT_MOBILIER.md">Rapport d’échelle</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-completion-06/integration-checks.json">Contrôles d’intégration</a> · <a href="../../assets/source-art/art-finish-completion-06/prompts-used.json">Prompts des dessins · outil imagegen intégré</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-completion-06/delegation.json">Délégation</a></p></section>
<script>document.getElementById('search').addEventListener('input',e=>{const q=e.target.value.toLocaleLowerCase('fr');for(const c of document.querySelectorAll('.card'))c.hidden=!c.dataset.search.includes(q)});</script></html>''')
    (OUT/'index.html').write_text('\n'.join(parts),encoding='utf-8');print('Published current gallery:',len(models),'models')

if __name__=='__main__':
    import sys
    if '--verify' in sys.argv:verify()
    if '--publish' in sys.argv:publish()
