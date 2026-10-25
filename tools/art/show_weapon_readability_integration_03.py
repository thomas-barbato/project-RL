"""Show actual engine evidence in the existing approved weapon review."""
from pathlib import Path
import json
import re

from prepare_weapon_readability_review_01 import ROOT, OUT, read, write


def main():
    path = OUT/'index.html'
    html = path.read_text('utf-8')
    html = re.sub(r'<section id="weapon-integration-03">.*?</section>', '', html, flags=re.S)
    html = html.replace('Armes lourdes · revue 02 · 64 px', 'Armes intégrées · 64 px')
    html = html.replace('WEAPON-READABILITY02 · ARMES LOURDES À VALIDER', 'WEAPON-READABILITY03 · INTÉGRÉ AU JEU')
    html = html.replace('Armes lourdes : quatre silhouettes à comparer', 'Six dessins d’armes intégrés au jeu')
    html = html.replace('03 à 06 : quatre familles d’armes lourdes sur les cinq classes. La hache et le fusil (01–02) sont validés.',
                        'La hache, le fusil et les quatre armes lourdes présentées utilisent désormais les dessins et les prises validés.')
    html = html.replace('"approved": false', '"approved": true')
    html = html.replace('Les propositions 03–06 attendent ta validation. Les dessins et prises 01–02 sont validés et conservés.',
                        'Les six dessins sont validés et intégrés. Les comparaisons originales sont conservées ci-dessus.')
    if '<a href="#weapon-integration-03">' not in html:
        html = html.replace('<p id="description"></p>', '<p><a href="#weapon-integration-03">Voir le rendu intégré dans le moteur ↓</a></p><p id="description"></p>')
    html = html.replace('00 · Actuel','00 · Avant').replace('00 : actuel','00 : avant')
    section = '''<section id="weapon-integration-03">
<h2>Rendu intégré · captures du moteur</h2>
<p>Les cinq classes en armure 05, à taille native. Ces images proviennent du rendu ordinaire du joueur dans le jeu.</p>
<div class="scroll"><img class="full" src="integration/heavy-engine-native.png" width="432" height="548" alt="Mitrailleuse, lance-flammes, lance-grenades et lance-roquettes intégrés sur les cinq classes en 64 px"></div>
<details><summary>Hache et fusil intégrés</summary><div class="scroll"><img class="full" src="integration/axe-rifle-engine-native.png" width="256" height="548" alt="Hache et fusil intégrés, cinq classes à taille native"></div></details>
<h2 id="melee-cuffs-03">Mêlée à deux mains · manches corrigées</h2>
<p>Les anciennes mains et les fragments de manche restés sous l’arme ont été retirés du montage. Comparaison à taille native sur les cinq armures du Bastion et de l’Arcaniste.</p>
<div class="scroll"><img class="full" src="integration/melee-63-native.png" width="584" height="454" alt="Hache de combat, avant et après correction, Bastion et Arcaniste"></div>
<details><summary>Autre hache et espadon</summary><div class="scroll"><img class="full" src="integration/melee-64-native.png" alt="Autre hache, avant et après correction à taille native"><img class="full" src="integration/melee-88-native.png" alt="Espadon, avant et après correction à taille native"></div></details>
<details><summary>Captures complètes : cinq niveaux d’armure et nuances de gris</summary>GALLERIES</details>
<p class="note">Contrôles : 3 100 compositions ; UNCHANGED compositions des autres configurations identiques ; 150 cas des six dessins conformes à la revue ; CLEANUP montages de mêlée corrigés. Palettes existantes conservées. Identités et statistiques des objets inchangées.</p>
</section>'''
    checks = read(ROOT/'docs/reviews/weapon-readability-02/integration-03-cpu-checks.json')
    section = section.replace('UNCHANGED',str(checks['unrelated_compositions_byte_identical']))
    section = section.replace('CLEANUP',str(checks['melee_arm_cleanup_changes']))
    names = [('two-handed-axe','Hache'),('rifle','Fusil'),('heavy-machinegun','Mitrailleuse'),
             ('heavy-flamethrower','Lance-flammes'),('heavy-grenade','Lance-grenades'),
             ('heavy-rocket','Lance-roquettes'),('rifle-gray','Fusil en gris'),
             ('heavy-machinegun-gray','Mitrailleuse en gris'),
             ('two-handed-axe-combat','Hache de combat · manches corrigées'),
             ('two-handed-sword','Espadon · manches corrigées')]
    section = section.replace('GALLERIES',''.join(
        f'<h3>{title}</h3><div class="scroll"><img class="full" src="integration/gpu/native-{page}.png" alt="{title}, capture du moteur sans réduction"></div>'
        for page,title in names))
    html = html.replace('<hr><p class="note">',section+'<hr><p class="note">',1)
    path.write_text(html,'utf-8')
    for folder in ['weapon-readability-01','weapon-readability-02']:
        doc = ROOT/'docs/reviews'/folder
        record = read(doc/'review.json')
        record['status'] = 'accepted_and_integrated'
        record['runtime_integration'] = True
        record['runtime_manifest'] = 'assets/sprites/weapon-readability-03/manifest.json'
        write(doc/'review.json',record)
    (ROOT/'docs/reviews/weapon-readability-02/INTEGRATION-03.md').write_text(
        '# WEAPON-READABILITY03 — intégration\n\n'
        'Accord : « ok ca me semble bien visible :) on peut donc tout integrer :) ».\n\n'
        'Les six modèles présentés (2, 17, 23, 31, 51, 66) utilisent leurs pixels natifs, prises, tailles par classe et ombre de séparation validés.\n'
        'Les armes, doigts et ombre restent des couches ; les têtes, visages ouverts et bottes gardent leurs sources.\n'
        'Le même chemin sert au plateau et aux portraits. Le cache final évite de reconstruire les couches à chaque image.\n'
        'Les banques embarquées viennent de assets/sprites/weapon-readability-03/ ; aucune dépendance du jeu à artifacts/.\n'
        'Les palettes d’équipement existantes restent prises en charge avec le même classement de matériaux ; les couleurs originales restent exactes.\n'
        'Correction demandée ensuite : les armes de mêlée à deux mains réutilisent la découpe complète des anciennes mains/manches, déjà employée pour les armes longues. Cela retire les fragments du Bastion et de l’Arcaniste sans repeindre leurs armures ni leurs armes.\n'
        'Les dessins des autres modèles du catalogue, leurs identités, statistiques et sauvegardes restent inchangés.\n\n'
        'Contrôles chiffrés : integration-03-cpu-checks.json et integration-03-gpu-checks.json.\n'
        'Aperçu courant : artifacts/weapon-readability-review-01/index.html#weapon-integration-03.\n', 'utf-8')
    print(json.dumps(dict(page=str(path),runtime_integration=True)))


if __name__ == '__main__':
    main()
