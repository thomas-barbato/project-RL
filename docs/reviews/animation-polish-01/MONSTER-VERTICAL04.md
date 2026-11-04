# MONSTER-VERTICAL04-r01 — possédés et hybrides

Après l’accord sur les ailés 07–09, le joueur demande « ok on peut continuer
:) ». Le quatrième lot propose trois autres marches haut/bas, en variante 0 :

10. MON-006-0 : possédé voûté, crête orange, bras longs et pantalon sombre.
11. MON-007-0 : hybride massif, bras mécanique droit et bras organique gauche.
12. MON-010-0 : hybride au canon droit, gilet sombre et pantalon orange.

Ce sont des libellés de revue ; les identités de jeu restent inchangées.
Huit figures entières par direction, deux pas alternés à 400 ms par pas et
800 ms par cycle. Conserver la posture voûtée de 006, la masse de 007 et le
canon solidaire du bras de 010. Le bras mécanique et le canon sont à gauche
de l’écran de face, à droite de dos. De dos, la bouche du canon pointe à
l’opposé du spectateur et ne doit pas se retourner vers lui.

Ces modèles n’ont pas d’atlas latéral récemment accepté. Leur repos est donc
repris exactement de la cellule 12 du VERT18 installé, avec son ancrage et son
ombre réels. Les deux cellules de repos des propositions sont identiques à
ce frontal. Les neuf références déjà présentées restent disponibles.

Sources, frontaux et atlas :
`assets/source-art/animation-polish-01/monster-vertical-04/r01/manifest.json`.
Six planches retenues dessinées par l’outil intégré image_gen ; prompts et
références : `MONSTER-VERTICAL04-PROMPTS.json`. Les deux planches de MON-006 ont
été reprises après un premier contrôle natif : elles étaient trop étroites et
redressées. Les reprises entières portent `-broad-source.png` et leurs prompts
sont dans `MONSTER-VERTICAL04-CROUCH-PROMPTS.json`.

La peau beige de MON-006 est harmonisée avec le frontal par la palette
`MON-006-ivory-palette.json`, préparée avant création de la texture comme pour
le démon rouge et l’ailé pâle déjà revus. Cette préparation ne modifie pas les
PNG sources, l’alpha, les formes, le repos ni les matériaux hors sélection.
Les autres modèles de ce lot gardent leurs couleurs peintes ; leurs relevés
comparatifs sont dans `MONSTER-VERTICAL04-COLORS.json`. Les chiffres décrivent
les matières et ne remplacent pas la comparaison visuelle native.

L’assemblage garde une échelle uniforme par
direction et une translation de la figure entière pour l’ancrage moyen du
cycle. Aucun membre ni bande de pixels ne reçoit de déplacement séparé.

La préparation conserve les 38 empreintes latérales historiques, les 12 du
premier lot vertical, les 12 du deuxième accepté, les quatre du rouge accepté
et les 13 ressources du troisième accepté. Les dix atlas latéraux installés
restent inchangés. Les lectures et la préparation des textures de cette revue
ont lieu avant les captures, dans le thread graphique pour leur création.

Statut : le joueur accepte le lot 10–12 par « on peut continuer ».
L’accord porte sur les variantes 0, les deux directions, huit poses par
direction, la posture basse et la palette préparée de MON-006, le bras
mécanique droit de MON-007 et le canon droit de MON-010, à 400 ms par pas.
Les treize ressources retenues sont figées dans
`MONSTER-VERTICAL04-ACCEPTANCE-r01.json`. Aucune intégration dans
la partie normale. Douze modèles proposés au total, 117 autres à reprendre,
puis les variantes restantes à contrôler. Les captures sont indépendantes
des parties et préférences de l’utilisateur.

```powershell
python tools/art/prepare_monster_vertical04.py --references
python tools/art/prepare_monster_vertical04.py
python tools/art/audit_monster_vertical04_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical04 artifacts/animation-polish-01/monster-vertical-04/r01/current
python tools/art/build_monster_vertical04_review.py
python tools/art/check_monster_vertical04.py
```

Vérification finale du 3 novembre 2026 : trois tests de palette passent
(MON-144, MON-151 et MON-006), avec alpha, contours, repos et matériaux hors
sélection conservés. Les 198 captures natives du nouveau lot contiennent six
retours au frontal exact. Les 79 empreintes antérieures et les dix atlas
latéraux installés restent inchangés.

Le lecteur a été exécuté dans le navigateur après le dernier rendu : douze
sélections chargent chacune 66 captures, l’inventaire conserve ses 129 fiches,
le comparatif natif mesure 576 × 320 pixels sans redimensionnement CSS, les
trois nouveaux modèles reviennent au repos dans chaque direction, et
l’aller-retour automatique fonctionne à 400 ms par pas. Aucun message
d’erreur console après le rechargement final. Bilan dans
`MONSTER-VERTICAL04-VALIDATION.json`. Preuves et compte rendu navigateur :
`artifacts/animation-polish-01/monster-vertical-04/r01/browser-proof.png`,
`browser-back-proof.png` et `browser-checks.json`. Ces fichiers de revue ne
sont pas des dépendances du jeu.
