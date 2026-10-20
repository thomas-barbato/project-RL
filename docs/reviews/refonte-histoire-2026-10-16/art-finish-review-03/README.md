# ART-FINISH-03 — quatre équipements existants

19 octobre 2026. **Lot accepté : « Valider le lot complet ».** Le joueur demande
de mettre aussi à jour les images du menu de création puis de poursuivre la
refonte graphique des personnages et armures. La correction du menu utilise
uniquement les dessins déjà acceptés, consignés dans ART-FINISH-02.

Ce nouveau lot porte sur quatre équipements réellement présents :

- FIN-22 : Tablier de cuir (`core:patched_plating`, tenue 208, torse 108).
- FIN-23 : Tenue de membranes (`core:veste_de_membranes`, tenue 209, torse 110).
- FIN-24 : Combinaison Peau-Seconde (`core:veste_de_peau_seconde`, tenue 210, torse 111).
- FIN-25 : Tenue du technicien (`core:veste_de_veille`, tenue 211, torse 112).

Les noms viennent des traductions courantes du jeu ; les anciens noms de
catalogue ne les remplacent pas. Ce lot ne modifie ni identifiants, ni règles,
ni équipement de départ. Les autres variantes de couleur attendent leur revue.

## Dessins et composition

Le skill imagegen, avec son outil intégré, fournit quatre maîtres et quatre
poses dédiées aux bras au fusil. Le dessin actuel de chaque tenue est la cible
de style ; Bastion accepté sert uniquement de référence de finition. Sources
complètes, prompts et noms de sortie sont dans
`assets/source-art/art-finish-review-03/`. Aucun générateur par API n'est utilisé.

Chaque tenue conserve ses couleurs et éléments distinctifs. La finition reprend
les matières mates, les volumes par masses, les contours nuancés et l'usure
située des lots acceptés. Le visage reste entièrement couvert. Les sujets
entiers sur fond transparent sont réduits sans étirement dans une toile de
128 × 128 px, pied (64,96), pour une case de 64 px. Les marges ne changent pas
l'échelle du personnage.

Le conditionnement reprend la méthode du lot précédent, avec corps, arme et
doigts séparés. Les modèles de pistolet et de fusil sont les dessins acceptés
FIN-13 et FIN-12 ; ils gardent leurs tailles portées d'environ 17 et 34 px.
Les doigts sont extraits des gants sources, sans dessin de remplacement par
script. Tête et bas du corps sont enregistrés exactement entre les prises.
Les points de prise et leur contact avec l'arme sont consignés dans la revue.

## Vérification et aperçu

Deux planches montrent les quatre tenues et les douze configurations, à taille
native, avec comparaison aux anciennes ressources. Les panneaux sont des
captures du GPU sans agrandissement. Deux comparaisons en carte reprennent le
logement et la rue avec l'éclairage réel du moteur, à la même heure.

Les silhouettes supplémentaires de présentation sont uniquement dessinées :
elles ne rejoignent ni simulation, ni sauvegarde, ni rencontres. Les empreintes
de partie sont vérifiées à chaque capture. Le détour de revue est désactivé
par défaut et exclu des builds release. Les fichiers normaux surveillés dans
`production-before.json` restent intacts ; aucune dépendance de production
n'est ajoutée aux sources ou à `artifacts/`.

La page courante est réutilisée : `artifacts/art-quality-audit-01/index.html`.
Elle conserve un accès au lot précédent intégré et aux portraits du menu de
création corrigé. Les sources et preuves utiles sont conservées dans ce dossier.

Commandes de préparation utilisées avant l'accord, conservées comme historique.
Le conditionnement et la capture de revue sont verrouillés depuis la validation :

```text
python tools/reviews/art_finish_batch_03.py --condition
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-03-review artifacts/art-quality-audit-01/finish03-engine
python tools/reviews/show_art_finish_review_03.py
python tools/reviews/validate_art_finish_review_03.py
```

L'accord explicite couvre les quatre tenues FIN-22 à FIN-25, leurs douze
configurations et les deux comparaisons éclairées présentées. `acceptance.json`
fige les empreintes des 28 exports effectivement utilisés : quatre dessins
sans arme et vingt-quatre calques de corps, armes et doigts. Les fichiers
composés de commodité ne sont pas dupliqués dans les ressources de production.

## Intégration

Le rendu normal embarque les PNG exacts depuis `assets/sprites/art-finish-03/`,
pour le plateau et l'inspection, sans dépendance aux dessins sources ou aux
captures. Les identités, noms, règles, variantes non montrées et autres prises
conservent leurs liaisons précédentes. La respiration existante garde les pieds
fixes ; aucune nouvelle animation de combat n'est activée.

Les références de revue ne sont pas réécrites. `integration-checks.json` compare
les seize panneaux du rendu normal et les deux vues éclairées à ces références.
`integration-validation.json` conserve les résultats des contrôles effectués
pour ce passage en production. Les 584 ressources précédentes surveillées sont
vérifiées par empreinte ; le premier et le deuxième lot restent intacts.

```text
python tools/reviews/integrate_art_finish_review_03.py
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-03-integration artifacts/art-quality-audit-01/finish03-integration
python tools/reviews/verify_art_finish_integration_03.py
```

L'ancien conditionnement et la commande de préparation de revue refusent la
réécriture des exports acceptés. L'audit du mobilier demandé, en commençant
par le lit du logement, reste programmé après la refonte complète des acteurs
dans `docs/PLAN_DE_TRAVAIL_CAMPAGNE.md`.
