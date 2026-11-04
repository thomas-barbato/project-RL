# BIPED-LATERAL03-r02 — direction des têtes des cultistes

Retour du joueur sur r01 : « quand les cultistes bougent leurs têtes ne
pivotent pas :) ». Les ouvertures de capuche restaient trop frontales ou
ambiguës pour accompagner le changement de direction du corps.

Cette révision concerne uniquement MON-164-0 au fusil et MON-165-0 à la
hache. Le volume du tissu montre son côté arrière et l'ouverture du visage
se place vers l'avant, à gauche ou à droite selon la marche. Les huit poses
de chaque côté sont reprises. La tête reste attachée au cou et suit le buste,
sans oscillation indépendante ni rotation procédurale d'une pièce collée.
La neuvième pose de revue reste frontale ; les captures utilisent le vrai
frontal courant au repos. Le port d'arme, les prises, les pas et l'identité
de chaque cultiste servent de contraintes à cette retouche.

La source entière est éditée par l'outil intégré image_gen, puis conditionnée
comme figure complète selon le même procédé natif. Le résultat de génération
ne garantit pas des pixels inchangés hors de la tête ; les armes et appuis
sont donc également à recontrôler visuellement. Les prompts et corrections
de transparence sont conservés dans `BIPED-LATERAL03-PROMPTS-r02.md`.

Les atlas des cinq autres modèles sont directement réutilisés : MON-145
(proposition r01 encore à examiner), MON-005, MON-014, MON-156 et MON-144
(accords distincts déjà enregistrés). Ils ne sont ni redessinés ni acceptés
par extension. Les deux directions, la palette d'origine, le camp, les cases
de 64 px, la lumière et les 400 ms par pas sont conservés.

L'aperçu courant ouvre le fusilier. Pour 02 et 03, la comparaison par défaut
affiche le cycle r01 puis r02 à la même phase ; l'autre colonne garde le
joueur comme repère de démarche. Pour les cinq autres modèles, le frontal
courant reste disponible. Le lecteur prépare les captures avant lecture et
conserve au plus deux banques de 66 images. Le zoom est entier.

Les sources vont dans `assets/source-art/animation-polish-01/biped-lateral-03/r02/`.
Reproduction : `tools/art/prepare_biped_hoods_r02.py`, puis
`--ui-cold-biped-hoods-r02 OUTPUT`,
`tools/art/build_biped_hoods_r02_review.py` et
`tools/art/check_biped_hoods_r02.py`.

Cette révision reste à présenter au joueur ; aucune intégration normale,
modification de gameplay, de sauvegarde, de réglage ou de marche verticale.

Contrôles réussis : compilation Debug, rustfmt ciblé et compilation Python.
462 captures natives, 448 correspondances de phase ; les 330 captures des cinq
modèles non repris et les 132 panneaux r01 de comparaison conservent exactement
leurs pixels d'acteur. Empreintes des cinq atlas inchangés contrôlées. Les sept
exports gardent exactement 2 400 ms par aller-retour (800 ms de cycle et
400 ms de pause de revue dans chaque direction).

Dans le navigateur : les deux cultistes, les passages des pieds dans les deux
sens, le vrai repos et la comparaison r01/r02 sont contrôlés. Taille native
320 × 192, zoom entier 640 × 384, planche simultanée 512 × 288 ; lecture à
400 ms active et aucune erreur console observée. La sélection d'un modèle
non repris conserve sa comparaison avec le frontal courant. Sources et
capturés inspectés pour la direction des têtes ; le jugement du joueur reste
attendu. Preuves dans `artifacts/animation-polish-01/biped-lateral-03/r02/` :
`native-validation.json`, `browser-checks.json`, `browser-proof.png` et
`hood-direction-summary.png`.
