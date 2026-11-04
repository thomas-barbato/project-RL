# MONSTER-VERTICAL06-r01 — cultistes armés et possédé en haillons

Après l’accord sur les cultistes 13–15, « on continue :) » poursuit la reprise
des marches haut/bas. Ce sixième lot propose uniquement les variantes 0 de :

16. MON-022-0 : cultiste armé 022, capuche rouge et fusil fin.
17. MON-024-0 : cultiste armé 024, capuche rouge et fusil au corps plus large.
18. MON-025-0 : possédé en haillons, tête chauve penchée et longs bras nus.

Ce sont des libellés de revue. Les armes présentes dans les dessins de 022 et
024 sont conservées ; aucun rôle, objet, dégât, rencontre ni capacité n’est
ajouté ou reclassé. Le statut réservé de certaines ressources ne change pas.

Huit figures entières par direction, deux appuis alternés à 400 ms par pas
et 800 ms par cycle. Les deux mains restent reliées aux prises des armes.
Les canons dirigés vers la gauche de l’écran de face conservent leur côté
anatomique et apparaissent à droite de dos. Les fusils sont portés devant
la poitrine : le corps masque leur centre et les mains de dos ; les armes
ne sont pas collées en travers du dos. Seules leurs extrémités qui dépassent
la silhouette restent visibles.

MON-025 garde ses épaules étroites, sa tête penchée solidaire du torse, ses
haillons vert sombre et les différences de couverture ou d’ombre des pieds.
Ses mains pendantes ne deviennent pas des appuis supplémentaires. Les figures
restent entières, sans bandes de jambes déplacées ni tête animée isolément.

Le repos conserve exactement la cellule 12 du VERT18 installé, avec son
ancrage et son ombre réels. Les quinze sélections précédentes restent dans
l’aperçu courant, dont les trois cultistes 13–15 acceptés.

Sources, frontaux et atlas :
`assets/source-art/animation-polish-01/monster-vertical-06/r01/manifest.json`.
Six planches dessinées avec l’outil intégré image_gen ; prompts et références
dans `MONSTER-VERTICAL06-PROMPTS.json`. Les figures sont calibrées avec une
échelle uniforme par direction et une translation de cycle pour l’appui
moyen, sans déplacement séparé des membres. Le diagnostic de couleur dans
`MONSTER-VERTICAL06-COLORS.json` ne modifie aucun pixel source et complète
la comparaison visuelle native.

Après le premier contrôle natif, les tons beiges de MON-025 sont rapprochés
du frontal par `MON-025-ivory-palette.json`, préparée avant création de la
texture. Une même règle sert à toutes les phases mobiles. Cette préparation
conserve l’alpha, les formes, les cellules de repos et les matériaux hors
sélection ; les PNG sources ne sont pas modifiés. Le test de palette contrôle
les tons beiges sélectionnés ; le diagnostic de couleurs décrit des ensembles
plus larges de peau et de tissu.

La préparation vérifie les 104 empreintes antérieures : 38 latérales, 12 du
premier lot vertical, 12 du deuxième accepté, quatre du rouge harmonisé,
13 du troisième, 13 du quatrième et 12 du cinquième acceptés. Les dix atlas
latéraux installés restent inchangés. Les textures de revue sont préparées
avant les captures, avec création dans le thread du contexte graphique.

Statut : lot 16–18 accepté le 3 novembre 2026 par « ok je valide et on continue :) ».
Les treize ressources et la palette de MON-025 sont figées dans
`MONSTER-VERTICAL06-ACCEPTANCE-r01.json`, sans activation dans la partie normale. Dix-huit modèles proposés au total, 111 autres à
reprendre, puis les variantes restantes à contrôler. Les captures ne chargent
aucune partie ni préférence de l’utilisateur.

```powershell
python tools/art/prepare_monster_vertical06.py --references
python tools/art/prepare_monster_vertical06.py
python tools/art/audit_monster_vertical06_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical06 artifacts/animation-polish-01/monster-vertical-06/r01/current
python tools/art/build_monster_vertical06_review.py
python tools/art/check_monster_vertical06.py
```

Vérification finale du 3 novembre 2026 : build verrouillé et quatre tests de
palette réussis (MON-144, MON-151, MON-006 et MON-025), avec alpha, contours,
cellules de repos et matériaux hors sélection conservés. Les 198 captures
natives du nouveau lot contiennent six retours au frontal exact. Les
104 empreintes antérieures et les dix atlas latéraux installés restent
inchangés. Formats Python, formatage Rust ciblé et `git diff --check` passent.

Le lecteur a été exécuté dans l’onglet courant : dix-huit sélections chargent
chacune 66 captures, l’inventaire garde ses 129 fiches, et les trois nouveaux
modèles retrouvent leur repos dans chaque direction avec le curseur de
phase. Le bouton de pose suivante et l’aller-retour automatique à 400 ms
par pas sont également contrôlés. Le comparatif mesure 576 × 320 pixels,
sans redimensionnement CSS. Aucun message d’erreur console après l’ouverture
finale. Bilan dans `MONSTER-VERTICAL06-VALIDATION.json`. Preuve de face du
possédé, preuve de dos du cultiste armé et compte rendu navigateur dans
`artifacts/animation-polish-01/monster-vertical-06/r01/browser-proof.png`,
`browser-back-proof.png` et `browser-checks.json`. Ces sorties ne sont pas
des dépendances du jeu.
