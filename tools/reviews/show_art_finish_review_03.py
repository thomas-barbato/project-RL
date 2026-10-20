"""Present frozen-size ART-FINISH-03 GPU samples and existing integrated UI."""
import json
import re
from PIL import Image,ImageDraw,ImageFont
from art_finish_batch_03 import ROOT,ART,DOC,OUT,ENTRIES,sha,write
from show_art_finish_review_02 import difference,VIEW

GPU=OUT/'finish03-engine'


def font(size):
    return ImageFont.truetype('C:/Windows/Fonts/segoeui.ttf',size)


def board(label,rows,title,controls=False,*,gpu=GPU,doc=DOC,prefix='finish03',height=None):
    raw=Image.open(gpu/f'{label}-gpu.png').convert('RGB')
    result=Image.new('RGB',(1240,height or (500 if controls else 700)),'#11191d')
    d=ImageDraw.Draw(result)
    d.text((24,14),title,font=font(23),fill='#decca7')
    d.text((24,48),'64 px par case · actuel à gauche · candidat à droite · aucune mise à l’échelle',font=font(16),fill='#bdc9c4')
    for i,(name,identity) in enumerate(rows):
        x,y=20+(i%4)*300,80+(i//4)*200
        d.text((x,y+2),name,font=font(16),fill='#decca7')
        for column,caption in enumerate(['Actuel','Candidat']):
            xx=x+column*140
            sample=raw.crop((xx,y+34,xx+128,y+162))
            result.paste(sample,(xx,y+44))
            assert difference(sample,result.crop((xx,y+44,xx+128,y+172)))==0
            d.text((xx+4,y+24),caption,font=font(12),fill='#a7b6b0')
        d.text((x,y+177),identity,font=font(12),fill='#aebfb9')
    if controls:
        accepted=Image.open(ROOT/'docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/native-native.png')
        for i,(name,box) in enumerate([
            ('FIN-01 · Bastion validé',(160,124,288,252)),
            ('FIN-02 · Cerbère validée',(460,124,588,252)),
            ('Corps civil validé',(320,724,448,852)),
            ('PNJ de référence',(620,724,748,852)),
        ]):
            x,y=20+i*300,280
            d.text((x,y+2),name,font=font(16),fill='#b8c9bf')
            result.paste(accepted.crop(box),(x+70,y+44))
    result.save(doc/f'{label}-native.png')
    result.save(OUT/f'{prefix}-{label}-native.png')
    return len(rows)*2


def build():
    latest=DOC.parent/'art-finish-review-04/review.json'
    if latest.exists() and (OUT/'finish04-engine/checks.json').exists():
        from show_art_finish_review_04 import build as show_current
        show_current()
        return
    if (DOC/'integration-checks.json').exists():
        show_integrated()
        return
    assert not (DOC/'acceptance.json').exists(), 'Preserve the accepted review'
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    capture=json.loads((GPU/'checks.json').read_text('utf-8'))
    assert review['drawings']==4 and capture['configuration_samples']==12
    baseline=json.loads((DOC/'production-before.json').read_text('utf-8'))
    for name,digest in baseline.items(): assert sha(ROOT/name)==digest,name
    count=board('native',[(f'{e[0]} · {e[1]}',e[2]) for e in ENTRIES],
                'ART-FINISH-03 · quatre équipements existants',True)
    count+=board('hands',[(f'{e[0]} · {pose}',e[1]) for e in ENTRIES for pose in ['sans arme','pistolet','fusil']],
                 'ART-FINISH-03 · douze configurations de mains')
    refs=DOC/'references';refs.mkdir(exist_ok=True)
    scenes={}
    for label in ['house','street']:
        samples=[]
        for suffix in ['before','candidate']:
            image=Image.open(GPU/f'{label}-{suffix}.png').crop(VIEW)
            image.save(refs/f'{label}-{suffix}-native.png')
            image.save(OUT/f'finish03-{label}-{suffix}.png')
            samples.append(image)
        scenes[label]=difference(*samples)
        assert scenes[label]>0,label
    write(DOC/'checks.json',dict(capture,production_files_unchanged=len(baseline),
          native_panels_preserved=count,scene_changed_pixels=scenes,source_masters=8))
    rows=''.join(f'<tr><th>{e[0]}</th><td>{e[1]}</td><td>{e[2]}</td></tr>' for e in ENTRIES)
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-03 · quatre armures</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav,.buttons{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}button{font:inherit;color:#ddd9cd;background:#263932;border:1px solid #718078;padding:8px 12px;cursor:pointer}button[aria-pressed=true]{background:#4b503a;border-color:#d5c39a}table{border-collapse:collapse;width:min(100%,1240px)}td,th{padding:10px 14px;border:1px solid #43514e;text-align:left}section{margin:36px 0}</style>
<h1>ART-FINISH-03 · quatre armures à examiner</h1>
<p>FIN-22 à FIN-25 : quatre équipements existants, chacun sans arme, au Cogneur P01 et au fusil d’assaut. Ces candidats poursuivent la finition mate et usée des lots acceptés. Ils restent dans la revue avant ton accord.</p>
<nav><a href="#creation">Menu de création corrigé</a><a href="#outfits">Quatre tenues</a><a href="#hands">Douze prises</a><a href="#scene">En carte</a><a href="#codes">Codes</a></nav>
<section id="creation"><h2>Déjà intégré : les portraits du menu de création</h2><p>Les vignettes et le grand portrait utilisent les cinq dessins de classe validés. Les captures ci-dessous viennent du menu normal. La fiche conserve l’équipement de départ réel.</p><div class="scroll"><img src="finish02-creation-menu.png" width="1240" height="340" alt="Les cinq portraits validés affichés dans le menu de création"></div><p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/creation-menu-full.png">Voir le menu complet</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/creation-menu-checks.json">Contrôles des cinq classes</a></p></section>
<section id="outfits"><h2>Quatre tenues à taille native</h2><p>Ancien dessin à gauche, candidat à droite. La case garde 64 px et la silhouette son échelle humaine. Les quatre références du premier lot restent inchangées.</p><div class="scroll"><img src="finish03-native-native.png" width="1240" height="500" alt="FIN-22 à FIN-25 à taille native, avec références"></div></section>
<section id="hands"><h2>Corps, arme et doigts séparés</h2><p>Douze configurations. Le casque, le tronc et les jambes restent enregistrés sur le maître ; les bras au fusil viennent d’une pose dédiée. Les prises, les tailles d’arme et les chevauchements des doigts sont vérifiés dans le rendu du moteur.</p><div class="scroll"><img src="finish03-hands-native.png" width="1240" height="700" alt="Douze configurations des quatre tenues à taille native"></div></section>
<section id="scene"><h2>Dans les vraies lumières de la carte</h2><p>Mêmes lieux, positions et horloge avant/après. Les figures de présentation sont dessinées sans ajout à la partie.</p>SCENES</section>
<section id="codes"><h2>Les codes du lot</h2><table><thead><tr><th>Code</th><th>Équipement</th><th>Identité conservée</th></tr></thead><tbody>ROWS</tbody></table><p>Le lot porte sur ces quatre tenues et les douze configurations montrées. Les noms, statistiques et autres palettes du jeu sont conservés.</p></section>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-03/README.md">Dossier de revue</a> · <a href="../../assets/source-art/art-finish-review-03/prompts.json">Prompts des tenues</a> · <a href="../../assets/source-art/art-finish-review-03/pose-prompts.json">Prompts des poses</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/README.md">Lot précédent intégré</a></p>
<script>for(const b of document.querySelectorAll('[data-scene]'))b.addEventListener('click',()=>{const {scene,mode}=b.dataset;document.querySelector('#scene-'+scene).src='finish03-'+scene+'-'+mode+'.png';for(const o of document.querySelectorAll('[data-scene="'+scene+'"]'))o.setAttribute('aria-pressed',String(o===b));});</script></html>'''
    scenes_html=''
    for label,title in [('house','Tablier de cuir dans le logement'),('street','Les quatre tenues dans la rue')]:
        scenes_html+=f'<h3>{title}</h3><div class="buttons"><button data-scene="{label}" data-mode="before">Actuel</button><button data-scene="{label}" data-mode="candidate" aria-pressed="true">Candidat</button></div><div class="scroll"><img id="scene-{label}" src="finish03-{label}-candidate.png" width="1218" height="704" alt="{title}"></div>'
    page=page.replace('SCENES',scenes_html).replace('ROWS',rows)
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')
    print('ART-FINISH-03: 4 drawings, 12 hand configurations, 2 scene comparisons')


def show_integrated():
    checks=json.loads((DOC/'integration-checks.json').read_text('utf-8'))
    assert checks['status']=='accepted_and_integrated'
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-03 · lot intégré</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}section{margin:36px 0}</style>
<h1>ART-FINISH-03 · lot validé et intégré</h1>
<p>FIN-22 à FIN-25 : quatre équipements et leurs douze configurations acceptés le 19 octobre 2026. Le rendu normal reproduit les 16 échantillons et les deux vues de carte à zéro pixel différent.</p>
<p>Tablier de cuir, Tenue de membranes, Combinaison Peau-Seconde et Tenue du technicien servent au plateau comme à l’inspection. Le corps, l’arme et les doigts restent des couches séparées, à l’échelle native de 64 px par case.</p>
<nav><a href="#outfits">Quatre tenues</a><a href="#hands">Douze prises</a><a href="#scene">En carte</a><a href="#creation">Création de personnage</a></nav>
<p>Les planches conservent la comparaison présentée : ancien dessin à gauche, dessin accepté à droite. Les autres armes, prises et palettes gardent leurs ressources précédentes pour les prochains lots.</p>
<section id="outfits"><h2>Quatre équipements à taille native</h2><div class="scroll"><img src="finish03-native-native.png" width="1240" height="500" alt="FIN-22 à FIN-25, comparaison avant et après à taille native"></div></section>
<section id="hands"><h2>Douze configurations de mains</h2><div class="scroll"><img src="finish03-hands-native.png" width="1240" height="700" alt="Quatre tenues sans arme, au pistolet et au fusil"></div></section>
<section id="scene"><h2>Le rendu normal avec les lumières du jeu</h2><p>Les figurants de présentation servent uniquement à comparer les tenues. Ils ne sont pas ajoutés aux rencontres ou aux sauvegardes.</p>
<h3>Dans le logement</h3><div class="scroll"><img src="finish03-house-integrated.png" width="1218" height="704" alt="Tablier de cuir intégré dans le logement éclairé"></div>
<h3>Dans la rue</h3><div class="scroll"><img src="finish03-street-integrated.png" width="1218" height="704" alt="Les quatre tenues intégrées dans la rue éclairée"></div></section>
<section id="creation"><h2>Menu de création actualisé</h2><p>Les cinq vignettes et le grand portrait de classe utilisent les dessins acceptés.</p><div class="scroll"><img src="finish02-creation-menu.png" width="1240" height="340" alt="Les cinq portraits de classe intégrés dans le menu de création"></div></section>
<p>L’audit de l’échelle du mobilier, en commençant par le lit signalé, est programmé après la refonte des acteurs dans le plan de travail.</p>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-03/README.md">Accord et bilan du lot</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-03/integration-checks.json">Contrôles d’intégration</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/README.md">Lot précédent intégré</a> · <a href="../../docs/PLAN_DE_TRAVAIL_CAMPAGNE.md">Audit du mobilier planifié</a></p></html>'''
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')

if __name__=='__main__': build()
