"""Show complete current enemy candidates at their unchanged native world size."""
import json
import re
from PIL import Image, ImageDraw
from art_finish_batch_05 import ROOT, ART, DOC, OUT, ENTRIES, sha, write
from show_art_finish_review_03 import board, font
from show_art_finish_review_02 import VIEW, difference

GPU=OUT/'finish05-engine'


def build():
    if (DOC/'integration-checks.json').exists():
        show_integrated()
        return
    assert not (DOC/'acceptance.json').exists(), 'Preserve the accepted review'
    review=json.loads((DOC/'review.json').read_text('utf-8'))
    capture=json.loads((GPU/'checks.json').read_text('utf-8'))
    before=json.loads((DOC/'production-before.json').read_text('utf-8'))
    assert capture['drawings']==4 and capture['background_samples']==12
    for name,digest in before.items(): assert sha(ROOT/name)==digest,name
    rows=[(f'{code} · {name}',f'{number:03} A · {detail}') for code,number,name,detail,_ in ENTRIES]
    board('native',rows,'ART-FINISH-05 · deux cultistes, un possédé et un zombie',gpu=GPU,doc=DOC,prefix='finish05',height=300)
    board('native',rows*3,'Même taille · sol clair, sol sombre et sol de la rue',gpu=GPU,doc=DOC,prefix='finish05-backgrounds',height=700)
    # Keep the compact board as a separate durable image; the generic helper writes one name.
    (DOC/'backgrounds-native.png').write_bytes((DOC/'native-native.png').read_bytes())
    board('native',rows,'ART-FINISH-05 · deux cultistes, un possédé et un zombie',gpu=GPU,doc=DOC,prefix='finish05',height=300)
    (OUT/'finish05-backgrounds-native.png').write_bytes((DOC/'backgrounds-native.png').read_bytes())
    refs=DOC/'references'; refs.mkdir(exist_ok=True)
    scenes={}
    for label in ['street','north']:
        pair=[]
        for suffix in ['before','candidate']:
            im=Image.open(GPU/f'{label}-{suffix}.png').crop(VIEW)
            im.save(refs/f'{label}-{suffix}-native.png')
            im.save(OUT/f'finish05-{label}-{suffix}.png')
            pair.append(im)
        scenes[label]=difference(*pair)
        assert scenes[label]>0,label
    accepted=Image.open(DOC.parent/'art-finish-review-01/native-native.png')
    controls=Image.new('RGB',(1240,280),'#11191d')
    d=ImageDraw.Draw(controls)
    d.text((24,16),'Références déjà acceptées · même échelle',font=font(23),fill='#decca7')
    for i,(label,box) in enumerate([
        ('Bastion · FIN-01',(160,124,288,252)),
        ('Cultiste · FIN-06',(460,324,588,452)),
        ('Possédé · FIN-08',(1060,324,1188,452)),
        ('Corps civil de référence',(320,724,448,852)),
    ]):
        x=20+i*300
        d.text((x,70),label,font=font(16),fill='#decca7')
        controls.paste(accepted.crop(box),(x+70,100))
    controls.save(DOC/'references-native.png'); controls.save(OUT/'finish05-references-native.png')
    write(DOC/'checks.json',dict(capture,production_files_unchanged=len(before),
        native_panels_preserved=24,scene_changed_pixels=scenes,source_masters=4,
        repaired_source='FIN-30: complete mechanical gripper, revised source retains transparent lower margin'))
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-05 · quatre ennemis</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav,.buttons{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}button{font:inherit;color:#ddd9cd;background:#263932;border:1px solid #718078;padding:8px 12px;cursor:pointer}button[aria-pressed=true]{background:#4b503a;border-color:#d5c39a}section{margin:36px 0}</style>
<h1>ART-FINISH-05 · quatre ennemis à examiner</h1>
<p>FIN-28 : Cultiste au pistolet (014 A). FIN-29 : Cultiste au bouclier et à la masse (016 A). FIN-30 : Possédé hybride quadrupède (006 A). FIN-31 : Zombie, détenu réanimé (120 A). Les noms affichés en jeu restent « Cultiste », « Possédé » et « Zombie » ; les précisions ci-dessus servent à désigner les dessins.</p>
<p>Cette revue reprend quatre designs existants avec la finition mate et usée des personnages acceptés. Même échelle et mêmes ancrages ; leurs règles de combat et leur répartition restent inchangées. Ces quatre candidats attendent ton accord.</p>
<nav><a href="#native">Quatre dessins</a><a href="#reference">Références acceptées</a><a href="#grounds">Trois sols</a><a href="#scene">Dans la carte</a></nav>
<section id="native"><h2>À taille prévue dans le jeu</h2><p>Ancien dessin à gauche, candidat à droite. Une case mesure 64 px. Le cadrage de chaque monstre reprend ses dimensions actuelles ; les toiles transparentes ne sont pas agrandies à l’écran.</p><div class="scroll"><img src="finish05-native-native.png" width="1240" height="300" alt="Quatre ennemis avant et après, à taille native"></div></section>
<section id="reference"><h2>Auprès des dessins validés</h2><div class="scroll"><img src="finish05-references-native.png" width="1240" height="280" alt="Bastion, cultiste, possédé et corps civil déjà acceptés, à la même échelle"></div></section>
<section id="grounds"><h2>Lisibilité sur trois fonds</h2><p>De haut en bas : sol clair, sol sombre, sol texturé de la rue. Les ombres de contact restent celles du moteur.</p><div class="scroll"><img src="finish05-backgrounds-native.png" width="1240" height="700" alt="Les quatre comparaisons sur trois sols, sans changement d’échelle"></div></section>
<section id="scene"><h2>Dans l’éclairage réel de la rue</h2><p>Bastion garde son dessin accepté. Les quatre ennemis sont uniquement dessinés pour cette présentation, sur des cases praticables et visibles. Les mêmes positions, lumières et horloge sont utilisées avant et après.</p>SCENES</section>
<p>Le lot porte sur les variantes A montrées. Les autres couleurs restent pour les lots suivants. Les quatre premiers lots et le menu de création restent intégrés ; l’audit du mobilier est toujours planifié après la refonte des acteurs.</p>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-05/README.md">Dossier du lot</a> · <a href="../../assets/source-art/art-finish-review-05/prompts.json">Prompts exacts · imagegen intégré</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04/README.md">Dernier lot intégré</a> · <a href="../../docs/PLAN_DE_TRAVAIL_CAMPAGNE.md">Plan de travail</a></p>
<script>for(const b of document.querySelectorAll('[data-scene]'))b.addEventListener('click',()=>{const {scene,mode}=b.dataset;document.querySelector('#scene-'+scene).src='finish05-'+scene+'-'+mode+'.png';for(const o of document.querySelectorAll('[data-scene="'+scene+'"]'))o.setAttribute('aria-pressed',String(o===b));});</script></html>'''
    scenes_html=''
    for label,title in [('street','Près de l’entrepôt'),('north','Plus au nord')]:
        scenes_html+=f'<h3>{title}</h3><div class="buttons"><button data-scene="{label}" data-mode="before">Actuel</button><button data-scene="{label}" data-mode="candidate" aria-pressed="true">Candidat</button></div><div class="scroll"><img id="scene-{label}" src="finish05-{label}-candidate.png" width="1218" height="704" alt="{title} : quatre ennemis et le personnage accepté"></div>'
    page=page.replace('SCENES',scenes_html)
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')
    print('ART-FINISH-05: 4 existing designs, 12 background samples, 2 lit comparisons; candidates only')


def show_integrated():
    checks=json.loads((DOC/'integration-checks.json').read_text('utf-8'))
    assert checks['status']=='accepted_and_integrated'
    page='''<!doctype html><html lang="fr"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>ART-FINISH-05 · lot intégré</title>
<style>body{margin:24px;background:#11191d;color:#d5ddda;font:17px Segoe UI,sans-serif;line-height:1.55}h1,h2,h3{color:#decca7}p{max-width:1120px}a{color:#dfc58e}nav{display:flex;gap:16px;flex-wrap:wrap;margin:16px 0}.scroll{overflow:auto;max-width:1240px;border:1px solid #43514e}img{display:block;max-width:none}section{margin:36px 0}</style>
<h1>ART-FINISH-05 · quatre ennemis validés et intégrés</h1>
<p>FIN-28 : Cultiste au pistolet (014 A). FIN-29 : Cultiste au bouclier et à la masse (016 A). FIN-30 : Possédé hybride quadrupède (006 A). FIN-31 : Zombie, détenu réanimé (120 A). Le lot complet est accepté le 19 octobre 2026.</p>
<p>Les dessins sont utilisés par le rendu normal et l’inspection. Les douze échantillons sur trois fonds et les deux vues éclairées correspondent aux références acceptées à zéro pixel différent. Les identités, règles de jeu, tailles de case et ombres de contact sont conservées.</p>
<nav><a href="#native">Quatre dessins</a><a href="#grounds">Trois sols</a><a href="#scene">Dans la carte</a><a href="#previous">Lots précédents</a></nav>
<section id="native"><h2>Comparaison acceptée à taille native</h2><p>Ancien dessin à gauche, dessin accepté à droite. Cases de 64 px, sans zoom.</p><div class="scroll"><img src="finish05-native-native.png" width="1240" height="300" alt="Quatre ennemis avant et après, dans leur taille de jeu"></div></section>
<section id="grounds"><h2>Lisibilité sur trois fonds</h2><div class="scroll"><img src="finish05-backgrounds-native.png" width="1240" height="700" alt="Comparaisons acceptées sur fond clair, sombre et sol de la rue"></div></section>
<section id="scene"><h2>Le rendu normal dans les lumières de la rue</h2><p>Les figurants de présentation sont uniquement dessinés pour la comparaison ; ils ne sont pas ajoutés à la partie.</p>
<h3>Près de l’entrepôt</h3><div class="scroll"><img src="finish05-street-integrated.png" width="1218" height="704" alt="Les quatre ennemis intégrés près de l’entrepôt"></div>
<h3>Plus au nord</h3><div class="scroll"><img src="finish05-north-integrated.png" width="1218" height="704" alt="Les quatre ennemis intégrés plus au nord de la rue"></div></section>
<section id="previous"><h2>Références déjà acceptées</h2><div class="scroll"><img src="finish05-references-native.png" width="1240" height="280" alt="Bastion, cultiste, possédé et corps civil acceptés à la même échelle"></div><p>Les quatre premiers lots et les portraits du menu de création restent intégrés. Les autres variantes de couleur conservent leurs dessins précédents. L’audit du mobilier reste planifié après la refonte des acteurs, avec le lit signalé en premier.</p></section>
<p><a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-05/README.md">Accord et bilan du lot</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-05/integration-checks.json">Contrôles d’intégration</a> · <a href="../../assets/source-art/art-finish-review-05/prompts.json">Prompts exacts · imagegen intégré</a> · <a href="../../docs/reviews/refonte-histoire-2026-10-16/art-finish-review-04/README.md">Lot précédent</a> · <a href="../../docs/PLAN_DE_TRAVAIL_CAMPAGNE.md">Plan de travail</a></p></html>'''
    for ref in re.findall(r'(?:src|href)="([^"#]+)"',page): assert (OUT/ref).exists(),ref
    (OUT/'index.html').write_text(page,encoding='utf-8')


if __name__=='__main__': build()
