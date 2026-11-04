# MONSTER-VERTICAL08-r01 — trois bipèdes sans équipement

Le 3 novembre 2026, « je valide et on continue :) » accepte le lot 19–21
de MONSTER-VERTICAL07-r01 et poursuit la revue haut/bas du bestiaire.
Les douze ressources finales acceptées sont figées dans
`MONSTER-VERTICAL07-ACCEPTANCE-r01.json`.

Ce huitième lot propose les variantes 0 de :

22. MON-008-0 : créature rouge à crête osseuse, ramassée, aux longues griffes.
23. MON-034-0 : petit démon orangé, large et bas, aux deux cornes sombres.
24. MON-036-0 : créature bleue effilée, à crête et gorge ocre claire.

Ces libellés servent à la revue, sans ajout ni changement d’ID, rôle,
rencontre, objet, capacité, dégât ou statut de ressource. Ces trois modèles
possèdent deux jambes porteuses. Leurs bras restent des bras, même lorsque
les griffes arrivent très près du sol. Les modèles rampants et à quatre
appuis encore non traités gardent leur statut dans l’inventaire ; aucune
marche bipède ne leur est appliquée.

Conserver les proportions mesurées sur les frontaux installés : 61 × 60 px
pour MON-008, 44 × 38 pour MON-034 et 43 × 50 pour MON-036. Les crêtes sont
solidaires du crâne et du torse ; celle de MON-008 ne devient ni museau ni
appendice supplémentaire. MON-034 garde son corps court orangé, ses cornes
et ses bras minces. La gorge claire de MON-036 appartient au devant et reste
occultée de dos. Son dessin arrière actuel montre encore cette zone claire ;
il ne sert donc pas de référence pour la nouvelle vue arrière.

Huit figures entières distinctes par direction, deux pas à 400 ms chacun
et un cycle de 800 ms. Des contacts, prises d’appui et passages bas sous les
hanches forment les pas. Aucun déplacement séparé de la tête, greffe de
jambes ni déformation de bandes. Échelle uniforme par direction et
translation commune au cycle pour aligner l’appui moyen, sans étirement.
Les largeurs sont contrôlées avec une tolérance d’un pixel de quantification,
en complément de la comparaison visuelle. Les cellules de repos conservent
exactement le frontal VERT18, son ancrage et son ombre réels.

Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-08/r01/manifest.json`.
Planches dessinées avec l’outil intégré image_gen en mode édition avec
conservation d’identité. Les apparences sont distinctes des guides de pas ;
prompts et références sont dans `MONSTER-VERTICAL08-PROMPTS.json`.
La première planche arrière de MON-008 répétait son frontal et le sens de
sa crête : elle est écartée. La reprise prend directement le dos actuel
comme référence principale ; prompt dans `MONSTER-VERTICAL08-BACK-PROMPT.json`.
Le diagnostic `MONSTER-VERTICAL08-COLORS.json` lit les matériaux visibles
par direction, sans modifier les PNG ni ajouter de filtre de lumière.
Les deux premiers dessins de MON-036 sont également écartés : silhouette
trop étroite et gorge frontale encore visible de dos. La reprise restaure
l’envergure des bras sans grossir le torse et ferme le crâne, la nuque et
le haut du dos en bleu-gris. Prompts dans
`MONSTER-VERTICAL08-BLUE-PROMPTS.json`. Le contrôle des huit poses arrière
vérifie aussi l’absence de pixels ocre dans la région de la gorge/haut du dos.
La comparaison native montre ensuite la planche frontale de MON-036 trop
claire dans les matériaux bleu-gris et ocre. Une retouche de palette par
image_gen devient trop sombre et est écartée ; elle est conservée comme
source distincte, avec son prompt dans `MONSTER-VERTICAL08-COLOR-PROMPT.json`.
Les poses précédentes restent sélectionnées. Une palette de matière préparée
avant création de la texture rapproche séparément le bleu-gris et l’ocre du
frontal conservé. Les mêmes gains servent aux huit poses de face ; le dos,
les deux repos, l’alpha, les contours et les matériaux hors sélection restent
exacts. Aucun PNG n’est modifié et aucun filtre d’éclairage du monde n’est
changé. Règle : `MON-036-front-material-palette.json` dans le dossier source.
Le diagnostic des médianes de matière est complété par un test sur les
ressources réelles, avec contrôle du dos et des pixels non sélectionnés.

La préparation vérifie 129 empreintes antérieures : 38 latérales, 12 du
premier lot vertical, 12 du deuxième accepté, quatre du rouge harmonisé,
13 du troisième, 13 du quatrième, 12 du cinquième, 13 du sixième et 12 du
septième acceptés. Les dix atlas latéraux installés restent inchangés.
Les ressources de revue sont préparées avant les captures, avec création
des textures dans le thread du contexte graphique.

Statut : n°22–24 acceptés par « ok je valide :) », puis confirmés par
« je valide sauf le 21 ». Les treize ressources sont figées dans
`MONSTER-VERTICAL08-ACCEPTANCE-r01.json`, sans activation dans la partie normale.
Le n°21 appartient au lot 07 et fait l’objet d’une reprise distincte sur
quatre appuis : `MONSTER-VERTICAL21-QUAD01.md`.
Les vingt et une sélections précédentes restent disponibles. Vingt-quatre
modèles proposés au total, 105 autres à reprendre, puis les variantes
restantes à contrôler. La capture ne charge aucune partie ni préférence
de l’utilisateur et n’active pas les cycles dans la partie normale.

```powershell
python tools/art/prepare_monster_vertical08.py --references
python tools/art/prepare_monster_vertical08.py
python tools/art/audit_monster_vertical08_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical08 artifacts/animation-polish-01/monster-vertical-08/r01/current
python tools/art/build_monster_vertical08_review.py
python tools/art/check_monster_vertical08.py
```

Vérification finale du 3 novembre 2026 : compilation verrouillée et cinq
tests ciblés réussis. Le nouveau test vérifie les médianes de chaque matière
de MON-036 sur les ressources réelles, ainsi que l’alpha, les cellules de
repos, le dos entier et les pixels hors sélection inchangés. Les quatre
palettes antérieures conservent leurs tests et leurs empreintes.

Les 198 captures natives présentent 48 poses mobiles distinctes et six
retours au frontal exact. La région supérieure du dos de MON-036 ne contient
aucun pixel ocre sélectionné. Les 129 empreintes antérieures et les dix atlas
latéraux installés sont conservés. Le diagnostic des deux matières préparées
de face retrouve leurs médianes de référence. Formats Python, formatage Rust
ciblé et `git diff --check` vérifiés.

Dans l’onglet courant, les vingt-quatre sélections chargent chacune 66 images
et l’inventaire conserve ses 129 fiches. Les six arrêts, le bouton de pose
suivante et l’aller-retour automatique de chacun des trois nouveaux modèles
ont été exercés à 400 ms par pas. Le comparatif mesure 576 × 320 pixels,
sans redimensionnement CSS ; aucune erreur console après l’ouverture finale.
Bilan dans `MONSTER-VERTICAL08-VALIDATION.json`.

Les dix générations, leurs empreintes et les six sources sélectionnées sont
consignées dans `MONSTER-VERTICAL08-GENERATIONS.json`. Les trois premières
planches écartées et la retouche trop sombre restent des sources distinctes.
Preuves et bilan d’interaction :
`artifacts/animation-polish-01/monster-vertical-08/r01/browser-proof.png`,
`browser-back-proof.png`, `native-proof.png` et `browser-checks.json`.
Ces sorties ne sont pas des dépendances de la partie normale.
