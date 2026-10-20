"""Present the frozen ART-FINISH-04 review and its checked runtime integration."""
import json
import re
from PIL import Image,ImageDraw
from art_finish_batch_04 import ROOT,ART,DOC,OUT,ENTRIES,sha,write
from show_art_finish_review_03 import board,font
from show_art_finish_review_02 import difference,VIEW

GPU=OUT/'finish04-engine'


def build():
    latest=DOC.parent/'art-finish-review-05/review.json'
    if latest.exists() and (OUT/'finish05-engine/checks.json').exists():
        from show_art_finish_review_05 import build as show_current
        show_current()
        return
    if (DOC/'integration-checks.json').exists():
        show_integrated()
        return
    assert not (DOC/'acceptance.json').exists(), 'Preserve the accepted review'
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    capture=json.loads((GPU/'checks.json').read_text('utf-8'))
    assert review['drawings']==2 and capture['configuration_samples']==6
    baseline=json.loads((DOC/'production-before.json').read_text('utf-8'))
    for name,digest in baseline.items(): assert sha(ROOT/name)==digest,name
    count=board('native',[(f'{e[0]} · {e[1]}',e[2]) for e in ENTRIES],
        'ART-FINISH-04 · deux équipements existants',gpu=GPU,doc=DOC,prefix='finish04',height=300)
    # Existing accepted red/orange outfits make their identities comparable.
    result=Image.open(DOC/'native-native.png')
    draw=ImageDraw.Draw(result)
    accepted=Image.open(DOC.parent/'art-finish-review-02/native-native.png')
    for x,label,box in [(620,'FIN-16 · Arcaniste validé',(760,124,888,252)),
                        (920,'FIN-20 · Mue validé',(760,324,888,452))]:
        draw.text((x,82),label,font=font(16),fill='#b8c9bf')
        draw.text((x,104),'Référence conservée',font=font(12),fill='#a7b6b0')
        result.paste(accepted.crop(box),(x+70,124))
    result.save(DOC/'native-native.png')
    result.save(OUT/'finish04-native-native.png')
    count+=board('hands',[(f'{e[0]} · {pose}',e[1]) for e in ENTRIES for pose in ['sans arme','pistolet','fusil']],
        'ART-FINISH-04 · six configurations de mains',gpu=GPU,doc=DOC,prefix='finish04',height=500)
    refs=DOC/'references';refs.mkdir(exist_ok=True)
    scenes={}
    for label in ['house','street']:
        images=[]
        for suffix in ['before','candidate']:
            image=Image.open(GPU/f'{label}-{suffix}.png').crop(VIEW)
            image.save(refs/f'{label}-{suffix}-native.png')
            image.save(OUT/f'finish04-{label}-{suffix}.png')
            images.append(image)
        scenes[label]=difference(*images)
        assert scenes[label]>0,label
    write(DOC/'checks.json',dict(capture,production_files_unchanged=len(baseline),
        native_panels_preserved=count,scene_changed_pixels=scenes,source_masters=4))
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-04 · deux tenues</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav,.buttons{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}button{font:inherit;color:#ddd9cd;background:#263932;border:1px solid #718078;padding:8px 12px;cursor:pointer}button[aria-pressed=true]{background:#4b503a;border-color:#d5c39a}section{margin:36px 0}</style>
<h1>ART-FINISH-04 · deux tenues à examiner</h1>
<p>FIN-26 : Robe de la Seconde Ombre. FIN-27 : Tenue du nomade. Deux équipements existants, chacun sans arme, au Cogneur P01 et au fusil d’assaut. Leurs couleurs et silhouettes conservent leurs identités ; la finition reprend les matières mates et l’usure des lots acceptés. Ces candidats attendent ton accord.</p>
<nav><a href="#outfits">Deux tenues</a><a href="#hands">Six prises</a><a href="#scene">Dans la carte</a><a href="#previous">Lots intégrés</a></nav>
<section id="outfits"><h2>À taille native, avec deux références acceptées</h2><p>Ancien dessin à gauche, candidat à droite. La case garde 64 px et la silhouette son échelle humaine. Les références permettent de comparer la robe au masque sombre avec l’Arcaniste et le manteau du nomade avec celui de Mue.</p><div class="scroll"><img src="finish04-native-native.png" width="1240" height="300" alt="FIN-26 et FIN-27 à taille native, avec deux références acceptées"></div></section>
<section id="hands"><h2>Six configurations : corps, arme et doigts séparés</h2><p>La tête et le bas du corps restent identiques entre les prises ; les bras au fusil viennent d’un dessin dédié. Le pistolet et le fusil gardent leurs tailles portées et leurs dessins acceptés.</p><div class="scroll"><img src="finish04-hands-native.png" width="1240" height="500" alt="Six configurations des deux tenues, à taille native"></div></section>
<section id="scene"><h2>Dans les vraies lumières de la carte</h2><p>Mêmes lieux, positions et horloge avant/après. Les figures de présentation sont dessinées sans ajout à la partie.</p>SCENES</section>
<section id="previous"><h2>Déjà intégré</h2><p>Les trois premiers lots et les portraits du menu de création sont intégrés. Ce lot ne modifie ni les personnages actuels, ni les noms, statistiques et autres palettes. L’audit du mobilier, en commençant par le lit signalé, reste planifié après la refonte des acteurs.</p><p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-03/README.md">Bilan du lot précédent</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/creation-menu-portraits.png">Menu de création actualisé</a> · <a href="../../docs/PLAN_DE_TRAVAIL_CAMPAGNE.md">Plan de travail</a></p></section>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04/README.md">Dossier de revue</a> · <a href="../../assets/source-art/art-finish-review-04/prompts.json">Prompts des tenues</a> · <a href="../../assets/source-art/art-finish-review-04/pose-prompts.json">Prompts des poses</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04/validation.json">Contrôles</a></p>
<script>for(const b of document.querySelectorAll('[data-scene]'))b.addEventListener('click',()=>{const {scene,mode}=b.dataset;document.querySelector('#scene-'+scene).src='finish04-'+scene+'-'+mode+'.png';for(const o of document.querySelectorAll('[data-scene="'+scene+'"]'))o.setAttribute('aria-pressed',String(o===b));});</script></html>'''
    scene_html=''
    for label,title in [('house','Robe de la Seconde Ombre dans le logement'),('street','Les deux tenues dans la rue')]:
        scene_html+=f'<h3>{title}</h3><div class="buttons"><button data-scene="{label}" data-mode="before">Actuel</button><button data-scene="{label}" data-mode="candidate" aria-pressed="true">Candidat</button></div><div class="scroll"><img id="scene-{label}" src="finish04-{label}-candidate.png" width="1218" height="704" alt="{title}"></div>'
    page=page.replace('SCENES',scene_html)
    # validation.json is produced immediately afterwards; other targets exist now.
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page):
        if not ref.endswith('/art-finish-review-04/validation.json'): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')
    print('ART-FINISH-04: 2 drawings, 6 hand configurations, 2 lit scene comparisons')


def show_integrated():
    checks=json.loads((DOC/'integration-checks.json').read_text('utf-8'))
    assert checks['status']=='accepted_and_integrated'
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-04 · lot intégré</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}section{margin:36px 0}</style>
<h1>ART-FINISH-04 · lot validé et intégré</h1>
<p>FIN-26 : Robe de la Seconde Ombre. FIN-27 : Tenue du nomade. Les deux tenues, leurs six configurations et les deux comparaisons en carte ont été acceptées le 19 octobre 2026.</p>
<p>Le rendu normal reproduit les huit échantillons et les deux vues de carte à zéro pixel différent. Ces dessins servent au plateau et à l’inspection, avec le corps, l’arme et les doigts en couches séparées.</p>
<nav><a href="#outfits">Deux tenues</a><a href="#hands">Six prises</a><a href="#scene">Dans la carte</a><a href="#creation">Création de personnage</a></nav>
<section id="outfits"><h2>Deux tenues à taille native</h2><p>Les planches conservent la comparaison présentée : ancien dessin à gauche, dessin accepté à droite, sur des cases de 64 px. Les deux références de classe restent inchangées.</p><div class="scroll"><img src="finish04-native-native.png" width="1240" height="300" alt="FIN-26 et FIN-27, avant et après à taille native"></div></section>
<section id="hands"><h2>Six configurations de mains</h2><div class="scroll"><img src="finish04-hands-native.png" width="1240" height="500" alt="Les deux tenues sans arme, au Cogneur P01 et au fusil d’assaut"></div></section>
<section id="scene"><h2>Le rendu normal avec les lumières du jeu</h2><p>Les figures de présentation servent uniquement à comparer les tenues et ne sont pas ajoutées à la partie.</p>
<h3>Dans le logement</h3><div class="scroll"><img src="finish04-house-integrated.png" width="1218" height="704" alt="Robe de la Seconde Ombre intégrée dans le logement éclairé"></div>
<h3>Dans la rue</h3><div class="scroll"><img src="finish04-street-integrated.png" width="1218" height="704" alt="Les deux tenues intégrées dans la rue éclairée"></div></section>
<section id="creation"><h2>Menu de création actualisé</h2><p>Les cinq vignettes et le grand portrait de classe conservent les dessins validés des lots précédents.</p><div class="scroll"><img src="finish02-creation-menu.png" width="1240" height="340" alt="Les cinq portraits de classe intégrés dans le menu de création"></div></section>
<p>Les autres armes, prises et palettes conservent leurs ressources précédentes pour les prochains lots. L’audit de l’échelle du mobilier reste planifié après la refonte des acteurs, en commençant par le lit signalé.</p>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04/README.md">Accord et bilan du lot</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04/integration-checks.json">Contrôles d’intégration</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-03/README.md">Lot précédent intégré</a> · <a href="../../docs/PLAN_DE_TRAVAIL_CAMPAGNE.md">Plan de travail</a></p></html>'''
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')


if __name__=='__main__': build()
