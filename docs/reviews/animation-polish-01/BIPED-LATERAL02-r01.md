# BIPED-LATERAL02-r01 — trois déclinaisons à 400 ms

Accord du joueur : « ok on peut continuer », après affichage de ce lot.
Les modèles 005, 014 et 156 sont acceptés dans le périmètre montré ; sources
et atlas figés dans `BIPED-LATERAL02-ACCEPTANCE-r01.json`. Les mentions de
proposition ci-dessous décrivent la présentation avant cet accord.

Le joueur valide MON-144-r03 et précise : « je valide, on mettra ces animations
en 400ms :) ». L'accord est figé séparément dans
`BIPED-LATERAL01-ACCEPTANCE-r03.json` : deux orientations, modèle 144,
palette d'origine, sans arme. La cadence de la suite du chantier est 400 ms
par pas, 800 ms par cycle. L'aperçu r03 et ses exports ont été réglés ainsi.

Le lot suivant propose 01 MON-005-0 (possédé voûté), 02 MON-014-0 (cultiste
en tabard court), 03 MON-156-0 (démon massif). Ils reprennent les vrais dessins
du cycle du Bastion, avec MON-144-r03 comme repère d'adaptation. Chaque modèle
est redessiné entier dans huit poses et un frontal, sur chaque côté. Sa
silhouette, ses vêtements et appendices viennent du frontal courant VERT18.
Le conditionnement conserve des figures entières, une taille native stable
et des appuis ancrés ; aucune jambe, tête ou main n'est déplacée seule.

04 MON-144-0 reste disponible comme référence **déjà validée**. Le lot réutilise
directement son atlas accepté, avec contrôle des empreintes des deux sources
et de l'atlas. Les 66 compositions du démon accepté sont identiques pixel par
pixel à celles de r03 dans la région du personnage ; seuls les en-têtes et
la cadence de lecture de l'export peuvent changer.

Le comparatif synchronise joueur et monstre : mêmes phases relatives, même
trajet d'étude de 40 px par cycle. Les captures contiennent 32 échantillons
par direction et un vrai repos frontal, soit 66 par modèle. Les exports
emploient 25 ms par échantillon de marche (800 ms par cycle), puis 400 ms
de repos de revue ; l'aller-retour complet dure 2 400 ms. Le navigateur
précharge les captures de la sélection et conserve au plus deux banques.

L'exposition habituelle de chaque acteur et la lumière du camp restent
conservées. Le joueur de référence n'est pas agrandi à la taille du monstre.
Le frontal actuel peut être comparé avec la nouvelle marche, et une vue
à trois colonnes ainsi qu'un zoom entier restent disponibles. La synchronisation
contrôle le temps et le déplacement ; l'adaptation des articulations et du
poids du corps demeure une revue visuelle.

État : 01–03 sont de nouvelles propositions à examiner ; 04 conserve son
accord distinct. Ni leurs autres palettes ni les contacts d'armes ne sont
validés par ce lot. Aucune intégration au rendu normal n'est faite ici.
Les règles de jeu, les sauvegardes, la marche verticale et les autres dessins
restent conservés. Le reste du bestiaire reste suivi dans la couverture.

Sources : `assets/source-art/animation-polish-01/biped-lateral-02/r01/`.
Prompts : `BIPED-LATERAL02-PROMPTS-r01.md`. Reproduction :
`--ui-cold-biped-nextlot OUTPUT`, puis `tools/art/build_biped_next02_review.py`.
Captures et contrôle du lecteur :
`artifacts/animation-polish-01/biped-lateral-02/r01/`.

Vérification : compilation Debug et 264 captures natives réussies. Le lecteur
charge les quatre banques de 66 images, conserve 320 × 192 pixels à taille
native et un zoom entier. Cadence sélectionnée à 400 ms, repos frontal et
passage du pied contrôlés manuellement, lecture active et aucune erreur
console observée. Preuves : `browser-checks.json` et `browser-proof.png`.
