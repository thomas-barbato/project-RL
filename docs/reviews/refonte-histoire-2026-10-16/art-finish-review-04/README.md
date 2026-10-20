# ART-FINISH-04 — deux tenues existantes

19 octobre 2026. **Lot validé et intégré : « Valider le lot complet ».** Le joueur accepte
les deux tenues, les six configurations de mains et les deux comparaisons
éclairées. `acceptance.json` fige les empreintes des 14 exports acceptés. Ce lot
reprend deux tenues existantes :

- FIN-26 : Robe de la Seconde Ombre — `core:veste_de_la_seconde_ombre`, tenue 217, torse 109.
- FIN-27 : Tenue du nomade — `core:veste_a_fibres_croisees`, tenue 218, torse 114.

Chaque tenue est présentée sans arme, au Cogneur P01 et au fusil d'assaut :
six configurations au total. Les noms sont ceux des traductions courantes.
Les anciens noms de catalogue ne les remplacent pas. Les trois dessins de
compatibilité sans liaison de contenu aux indices 205 à 207 ne sont pas
réintroduits par ce lot. Il ne constitue pas une refonte des autres armes,
prises, peintures d'équipement ou icônes d'objet.

## Fabrication

Le skill imagegen utilise l'outil intégré pour deux maîtres complets et deux
poses dédiées aux bras au fusil. Le dessin actuel de chaque tenue est la cible
de style ; Bastion accepté sert de référence de finition. Les sources sont
copiées dans `assets/source-art/art-finish-review-04/`, avec les prompts exacts
et les noms des sorties. Aucun générateur par API n'est utilisé.

Les identités restent lisibles : robe cramoisie avec panneau et masque sombres
pour la Seconde Ombre ; manteau rouge ouvert, bande de tissu ivoire et pantalon
sombre pour le nomade. Les matières mates, les volumes par masses, les contours
nuancés et l'usure située reprennent les lots acceptés. Aucune capacité ou
statistique n'est ajoutée. Les variantes de couleur non montrées attendent
leur propre revue.

Le sujet complet est conditionné sans étirement dans une toile transparente
de 128 × 128 px, pied (64,96), pour une case de 64 px et une silhouette humaine
d'environ 60 px. Le filtre du moteur reste Nearest. Il n'existe pas de nouvelle
version d'affichage 32 px.

Le conditionnement réutilise la méthode du lot 03 : corps, arme et doigts en
couches séparées ; bras au fusil issus d'une pose dédiée ; tête et bas du corps
enregistrés exactement entre les prises. Les doigts sont ceux des gants sources.
Le pistolet et le fusil réutilisent les dessins acceptés FIN-13 et FIN-12 avec
leurs tailles portées d'environ 17 et 34 px. Les contacts aux points de prise
sont vérifiés et consignés ; aucune arme n'est peinte dans le corps.

## Préparation de la revue

Les seize panneaux comparatifs proviennent du GPU sans changement d'échelle.
La planche principale comprend deux références acceptées de tenues rouges pour
comparer les identités. Deux vues avant/après montrent le logement et la rue
dans l'éclairage réel, avec les mêmes positions et la même horloge. Les figures
de présentation sont uniquement dessinées ; les empreintes de partie sont
vérifiées à chaque capture.

Le mode de revue est désactivé par défaut et exclu des builds release. Le rendu
normal ne dépend ni de ces sources ni des captures. `production-before.json`
conserve les empreintes des ressources précédentes et `validation.json` compare
le rendu normal aux seize échantillons et aux deux vues acceptées du lot 03.
Les planches et références acceptées des lots précédents ne sont pas réécrites.

```text
python tools/reviews/art_finish_batch_04.py --condition
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-04-review artifacts/art-quality-audit-01/finish04-engine
target/debug/project-rl.exe --ui-cold-art-finish-03-integration artifacts/art-quality-audit-01/finish04-normal-check
python tools/reviews/show_art_finish_review_04.py
python tools/reviews/validate_art_finish_review_04.py
```

Ces commandes décrivent la préparation avant accord. Les scripts de préparation
et de conditionnement refusent désormais de réécrire ce lot accepté. Les
planches et références montrées au joueur restent figées.

## Intégration acceptée

Les 14 exports exacts sont copiés dans `assets/sprites/art-finish-04/` et embarqués
par `src/game_art_finish_catalog_04.rs`. Le fournisseur commun les utilise au
plateau et dans l'inspection. La Robe de la Seconde Ombre conserve sa liaison
tenue 217 / torse 109 ; le nomade conserve tenue 218 / torse 114. Seules les
configurations acceptées sans arme, au Cogneur P01 et au fusil d'assaut, dans
leurs couleurs originales, utilisent ce lot. Le souffle idle existant reste
appliqué, avec les pieds fixes. Noms, statistiques et règles de jeu sont conservés.

Le contrôle d'intégration compare les huit échantillons et les deux vues éclairées
du rendu ordinaire aux références acceptées. Les ressources précédentes sont
vérifiées contre les 612 empreintes de `production-before.json`. Les résultats
sont consignés dans `integration-checks.json` et `integration-validation.json`.
Le jeu ne dépend ni de `artifacts/`, ni des sources de fabrication.

```text
python tools/reviews/integrate_art_finish_review_04.py
cargo test --locked --bin project-rl game_art:: -- --test-threads=1
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-04-integration artifacts/art-quality-audit-01/finish04-integration
python tools/reviews/verify_art_finish_integration_04.py
cargo check --locked --release
cargo fmt --all -- --check
git diff --check
```

La page courante reste `artifacts/art-quality-audit-01/index.html` et présente
le bilan intégré. L'audit du mobilier reste planifié après la refonte des acteurs,
avec le lit signalé comme premier point à vérifier.
