"""One grouped native-size review, using actual GPU output and real lighting."""
from pathlib import Path
import hashlib
import html
import json
import re
from PIL import Image, ImageChops, ImageDraw, ImageFont
from art_finish_batch_02 import ROOT, ART, DOC, OUT, ENTRIES, sha, write

GPU = OUT/'finish02-engine'
VIEW = (334,78,1552,782)

def font(size):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)

def difference(a,b):
    diff=ImageChops.difference(a.convert('RGB'),b.convert('RGB'))
    return sum(p!=(0,0,0) for p in diff.get_flattened_data())

def board(name,rows,title,controls=False):
    raw=Image.open(GPU/f'{name}-gpu.png').convert('RGB')
    assert raw.size==(1240,900)
    result=Image.new('RGB',(1240,700),'#11191d')
    draw=ImageDraw.Draw(result)
    draw.text((24,14),title,font=font(23),fill='#decca7')
    draw.text((24,48),'64 px par case · actuel à gauche · candidat à droite · sprites sans agrandissement',font=font(16),fill='#bdc9c4')
    count=0
    for i,(label,identity) in enumerate(rows):
        x,y=20+(i%4)*300,80+(i//4)*200
        draw.text((x,y+2),label,font=font(16),fill='#decca7')
        for column,caption in enumerate(['Actuel','Candidat']):
            xx=x+column*140
            image=raw.crop((xx,y+34,xx+128,y+162))
            result.paste(image,(xx,y+44))
            assert difference(image,result.crop((xx,y+44,xx+128,y+172)))==0
            draw.text((xx+4,y+24),caption,font=font(12),fill='#a7b6b0')
            count+=1
        draw.text((x,y+177),identity,font=font(12),fill='#aebfb9')
    if controls:
        approved=Image.open(ROOT/'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/native-native.png')
        for i,(label,box) in enumerate([
            ('FIN-01 · Bastion validé',(160,124,288,252)),
            ('FIN-02 · Cerbère validée',(460,124,588,252)),
            ('Corps civil validé',(320,724,448,852)),
            ('PNJ de référence',(620,724,748,852)),
        ]):
            x,y=20+i*300,480
            draw.text((x,y+2),label,font=font(16),fill='#b8c9bf')
            draw.text((x,y+24),'Référence conservée',font=font(12),fill='#a7b6b0')
            result.paste(approved.crop(box),(x+70,y+44))
    result.save(OUT/f'finish02-{name}-native.png')
    result.save(DOC/f'{name}-native.png')
    return count

def build():
    latest=DOC.parent/'art-finish-review-03/review.json'
    if latest.exists() and (OUT/'finish03-engine/checks.json').exists():
        from show_art_finish_review_03 import build as show_current
        show_current()
        return
    if (DOC/'integration-checks.json').exists():
        show_integrated()
        return
    assert not (DOC/'acceptance.json').exists(), 'Frozen review: run integration verification first'
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    capture=json.loads((GPU/'checks.json').read_text('utf-8'))
    assert review['review']==capture['review']=='ART-FINISH-02'
    assert len(review['entries'])==8 and capture['configuration_samples']==24
    baseline=json.loads((DOC/'production-before.json').read_text('utf-8'))
    for name,digest in baseline.items(): assert sha(ROOT/name)==digest,name
    preserved=board('native',[(f'{e[0]} · {e[1]}',e[2]) for e in ENTRIES],
                    'ART-FINISH-02 · quatre classes et quatre armures',True)
    for label,selection,title in [
        ('classes-hands',ENTRIES[:4],'Classes · corps, pistolet et fusil'),
        ('armors-hands',ENTRIES[4:],'Armures · corps, pistolet et fusil'),
    ]:
        rows=[(f'{e[0]} · {pose}',e[1]) for e in selection for pose in ['sans arme','pistolet','fusil']]
        preserved+=board(label,rows,title)
    scenes={}
    refs=DOC/'references';refs.mkdir(exist_ok=True)
    for label in ['class-house','classes-street','armor-house','armors-street']:
        images=[]
        for suffix in ['before','candidate']:
            image=Image.open(GPU/f'{label}-{suffix}.png').crop(VIEW)
            image.save(OUT/f'finish02-{label}-{suffix}.png')
            image.save(refs/f'{label}-{suffix}-native.png')
            images.append(image)
        scenes[label]=difference(*images)
        assert scenes[label]>0,label
    checks=dict(capture,production_files_unchanged=len(baseline),native_panels_preserved=preserved,
                scene_changed_pixels=scenes,source_master_count=16,
                fixed_core_regions='head rows 0..44 and lower body rows 80..128 identical between configurations',
                prompts='assets/source-art/art-finish-review-02/prompts.json',
                page='artifacts/art-quality-audit-01/index.html')
    write(DOC/'checks.json',checks)
    rows=''.join(f'<tr><th>{e[0]}</th><td>{html.escape(e[1])}</td><td>{e[2]}</td></tr>' for e in ENTRIES)
    scene_html=''
    for label,title in [('class-house','Technomancien dans le logement'),('classes-street','Les quatre classes dans la rue'),
                        ('armor-house','Chrysalide dans le logement'),('armors-street','Les quatre armures dans la rue')]:
        scene_html+=f'<h3>{title}</h3><div class="buttons"><button data-scene="{label}" data-mode="before">Actuel</button><button data-scene="{label}" data-mode="candidate" aria-pressed="true">Candidat</button></div><div class="scroll"><img id="scene-{label}" src="finish02-{label}-candidate.png" width="1218" height="704" alt="{title}, dans les lumières calculées du moteur"></div>'
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Personnages et armures · ART-FINISH-02</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav,.buttons{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}button{font:inherit;color:#ddd9cd;background:#263932;border:1px solid #718078;padding:8px 12px;cursor:pointer}button[aria-pressed=true]{background:#4b503a;border-color:#d5c39a}table{border-collapse:collapse;width:min(100%,1240px)}td,th{padding:10px 14px;border:1px solid #43514e;text-align:left}section{margin:36px 0}.note{color:#aebfb8}</style>
<h1>ART-FINISH-02 · personnages et armures</h1>
<p><strong>Huit candidats à valider : FIN-14 à FIN-21.</strong> Les quatre autres tenues de classe et quatre armures rejoignent la finition mate et usée du lot accepté. Les noms, identités et règles du jeu sont conservés. Les nouvelles images restent dans ce prototype de revue.</p>
<nav><a href="#all">Les huit tenues</a><a href="#classes">Prises des classes</a><a href="#armor">Prises des armures</a><a href="#scene">Dans la carte</a><a href="#codes">Codes</a><a href="#previous">Premier lot accepté</a></nav>
<section id="all"><h2>Huit tenues et quatre références, à taille réelle</h2><p>L’actuel est à gauche, le candidat à droite. La case reste de 64 px et le personnage d’environ 60 px de haut. Les références du premier lot et le corps civil restent à la même échelle. Affichage du navigateur à 100 % pour conserver cette échelle.</p><div class="scroll"><img src="finish02-native-native.png" width="1240" height="700" alt="FIN-14 à FIN-21 comparés aux dessins actuels, avec quatre références conservées"></div></section>
<section id="classes"><h2>Les prises en main des quatre classes</h2><p>Sans arme, au Cogneur P01 et au fusil d’assaut : douze configurations. Le casque ou la capuche et le bas du corps sont fixes ; les bras de la prise à deux mains viennent d’une source dédiée. Le corps, l’arme et les doigts sont dessinés en couches séparées.</p><div class="scroll"><img src="finish02-classes-hands-native.png" width="1240" height="700" alt="Douze configurations des quatre tenues de classe"></div></section>
<section id="armor"><h2>Les prises en main des quatre armures</h2><p>Douze autres configurations, avec les mêmes armes et leur taille réelle. Les plaques, les tissus et leurs couleurs permettent de distinguer les équipements.</p><div class="scroll"><img src="finish02-armors-hands-native.png" width="1240" height="700" alt="Douze configurations de Chrysalide, de la combinaison fantôme, du manteau de mue et du plastron Bastion"></div></section>
<section id="scene"><h2>Avec les vraies lumières du jeu</h2><p>Mêmes positions, mêmes décors, même horloge et mêmes lumières avant/après. Les silhouettes supplémentaires servent uniquement à présenter les candidats dans la scène ; elles ne sont pas ajoutées à la partie.</p>SCENES</section>
<section id="codes"><h2>Codes pour ton retour</h2><table><thead><tr><th>Code</th><th>Tenue</th><th>Identité conservée</th></tr></thead><tbody>ROWS</tbody></table><p>Tu peux valider le lot complet ou préciser les codes et les prises à corriger. Cette revue porte sur huit dessins et les vingt-quatre configurations montrées ; elle ne modifie pas les autres poses ou palettes.</p></section>
<section id="previous"><h2>Premier lot accepté</h2><p>ART-FINISH-01 reste intégré et sert de référence. <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/README.md">Accord et bilan du premier lot</a>.</p></section>
<p class="note">Fabrication avec le mode intégré imagegen : <a href="../../assets/source-art/art-finish-review-02/prompts.json">prompts des huit tenues</a>, <a href="../../assets/source-art/art-finish-review-02/pose-prompts.json">prompts des poses</a>, <a href="../../assets/source-art/art-finish-review-02/correction-prompts.json">corrections</a>. <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/checks.json">Contrôles</a>.</p>
<script>for(const b of document.querySelectorAll('[data-scene]'))b.addEventListener('click',()=>{const {scene,mode}=b.dataset;document.querySelector('#scene-'+scene).src='finish02-'+scene+'-'+mode+'.png';for(const o of document.querySelectorAll('[data-scene="'+scene+'"]'))o.setAttribute('aria-pressed',String(o===b));});</script></html>'''.replace('SCENES',scene_html).replace('ROWS',rows)
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')
    print(json.dumps(dict(review='ART-FINISH-02',drawings=8,configurations=24,scene_pairs=4,panels_preserved=preserved)))

def show_integrated():
    checks=json.loads((DOC/'integration-checks.json').read_text('utf-8'))
    assert checks['status']=='accepted_and_integrated'
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-02 · lot intégré</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}section{margin:36px 0}</style>
<h1>ART-FINISH-02 · lot validé et intégré</h1>
<p>FIN-14 à FIN-21 : huit tenues et vingt-quatre configurations acceptées le 19 octobre 2026. Le rendu normal du jeu reproduit les 32 échantillons et les quatre vues en carte à zéro pixel différent.</p>
<p>Les nouvelles tenues servent au plateau comme à l’inspection. Le corps, l’arme et les doigts restent des couches séparées. La case conserve ses 64 px. Les autres prises et palettes gardent leurs ressources précédentes pour les prochains lots.</p>
<nav><a href="#all">Huit tenues</a><a href="#classes">Prises des classes</a><a href="#armor">Prises des armures</a><a href="#scene">En carte</a></nav>
<p>Les planches conservent la comparaison présentée : ancien dessin à gauche, dessin accepté à droite, à taille native.</p>'''
    for anchor,label,title in [('all','native','Quatre classes et quatre armures'),
                               ('classes','classes-hands','Douze configurations des classes'),
                               ('armor','armors-hands','Douze configurations des armures')]:
        page+=f'<section id="{anchor}"><h2>{title}</h2><div class="scroll"><img src="finish02-{label}-native.png" width="1240" height="700" alt="{title}, comparaison avant et après"></div></section>'
    page+='<section id="scene"><h2>Le rendu normal avec les lumières du jeu</h2><p>Les figurants de présentation servent seulement à comparer les tenues. Ils ne sont pas ajoutés aux rencontres ou aux sauvegardes.</p>'
    for label,title in [('class-house','Technomancien dans le logement'),('classes-street','Les classes dans la rue'),
                        ('armor-house','Chrysalide dans le logement'),('armors-street','Les armures dans la rue')]:
        page+=f'<h3>{title}</h3><div class="scroll"><img src="finish02-{label}-integrated.png" width="1218" height="704" alt="{title}"></div>'
    page+='</section><p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/README.md">Accord et bilan du lot</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/integration-checks.json">Contrôles</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/README.md">Premier lot intégré</a></p></html>'
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')

if __name__=='__main__': build()
