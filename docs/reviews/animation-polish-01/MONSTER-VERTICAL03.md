# MONSTER-VERTICAL03-r01 — trois ailés au sol

« parfait on peut continuer » valide la palette harmonisée du démon rouge
MON-144. La règle, l’atlas source et le frontal sont conservés par les quatre
empreintes de `MONSTER-VERTICAL-COLOR01-ACCEPTANCE-r01.json`. Le troisième lot
haut/bas propose trois autres silhouettes, en variante 0 et palette d’origine :

7. MON-039-0 : petit démon ailé, petit gabarit et ailes portées vers le bas.
8. MON-146-0 : ailé doré, corps massif et membranes sombres.
9. MON-151-0 : ailé pâle, ailes bordeaux et brassard unique à l’avant-bras droit.

Huit figures entières par direction, deux pas alternés à 400 ms par pas et
800 ms par cycle. Les ailes restent portées pour l’équilibre au sol. Les
frontaux installés sont copiés exactement au repos ; les figures sont calibrées
uniformément par direction et le cycle entier est ancré sur son appui moyen.
Les têtes et membres ne reçoivent aucune translation indépendante.

Le brassard de MON-151 reste à gauche de l’écran de face, à droite de dos.
L’autre avant-bras reste découvert ; la couleur noire de la main ne constitue
pas un deuxième brassard. Les deux vues sont vérifiées sur les planches.

Les six planches sont produites avec l’outil intégré image_gen. La reprise de
couleur de MON-151 conserve des sources séparées portant `-palette-source.png`.
Pour ses tons ivoire et bruns chauds, une palette préparée avant création de
texture rapproche le cycle du frontal, avec la même règle pour toutes les
phases. Les sources PNG, l’alpha, les positions, les ailes bordeaux et le repos
ne sont pas modifiés par cette préparation. Les autres modèles, dont le rouge
validé de MON-144, restent inchangés.

Le contrôle de couleur relève séparément les tons de peau, les ors et les
membranes ; ses mesures sont dans `MONSTER-VERTICAL03-COLORS.json`. Ce diagnostic
ne remplace pas la comparaison visuelle native. Les couleurs de MON-151 ont
été rapprochées du frontal après un premier écart clair et rosé. Les ombres et
hautes lumières conservent la finition de chaque matière.

Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-03/r01/manifest.json`.
Palette : `MON-151-ivory-palette.json` du même dossier.
Prompts : `MONSTER-VERTICAL03-PROMPTS.json` et
`MONSTER-VERTICAL03-PALETTE-PROMPTS.json`.
La préparation vérifie les 38 empreintes latérales historiques, les 12 du
premier lot vertical, les 12 du deuxième accepté et les 4 du rouge accepté.
Après cet accord, elle vérifie aussi les 13 ressources acceptées du lot 03.

L’aperçu courant garde les neuf sélections et l’inventaire des 129 modèles.
Ce lot est accepté visuellement ; aucun nouveau dessin n’est sélectionné dans
la partie normale. Les captures utilisent le décor accepté, la case de 64 px,
l’éclairage des monstres (gain 1,48), sans charger de partie sauvegardée.

```powershell
python tools/art/prepare_monster_vertical03.py
python tools/art/audit_monster_vertical03_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical03 artifacts/animation-polish-01/monster-vertical-03/r01/current
python tools/art/build_monster_vertical03_review.py
python tools/art/check_monster_vertical03.py
```

Statut : le joueur accepte les trois marches 07–09 par « c'est validé :) ».
L’accord porte sur MON-039/146/151 en variante 0, les deux directions, huit
poses par direction, la palette ivoire préparée et le repos frontal conservé,
à 400 ms par pas et 800 ms par cycle. Les atlas, frontaux, sources retenues et
la règle ivoire sont figés par les treize empreintes de
`MONSTER-VERTICAL03-ACCEPTANCE-r01.json`. L’intégration reste distincte de cet
accord visuel. Neuf modèles sont proposés au total ; 120 autres restent à
reprendre, puis les variantes restantes à contrôler.

Vérification finale du 2 novembre 2026 : les deux tests ciblés des palettes
rouge et ivoire passent. Les 198 captures natives du nouveau lot sont
contrôlées, avec six cellules de repos frontal exactes. Les 38 empreintes
latérales historiques, les 12 sources du premier lot, les 12 du deuxième lot
accepté, les quatre du rouge accepté et les dix atlas latéraux installés
restent inchangés.

L’aperçu a été exécuté dans le navigateur : les neuf sélections chargent
chacune leurs 66 captures, les 129 fiches de l’inventaire sont présentes,
le comparatif mesure 576 × 320 pixels sans redimensionnement CSS, les deux
directions défilent automatiquement à 400 ms par pas et la commande manuelle
retrouve le repos frontal dans les deux sens. Aucun message d’erreur console
n’a été relevé après ouverture de cette révision. Les preuves de face et de
dos et le bilan sont dans
`artifacts/animation-polish-01/monster-vertical-03/r01/browser-proof.png`,
`browser-back-proof.png` et `browser-checks.json`. Ces sorties servent à la
revue ; le jeu ne les charge pas.
