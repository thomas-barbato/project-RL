# MONSTER-VERTICAL10-r01 — trois démons bipèdes

Le 3 novembre 2026, « je valide :) », puis « et on continue », accepte les
n°25–27, MON-035/037/038. Leurs treize ressources, dont la palette de chair
préparée de MON-035, sont figées dans `MONSTER-VERTICAL09-ACCEPTANCE-r01.json`.

Le dixième lot propose les variantes 0 de :

28. MON-060-0 : démon rouge élancé, torse sombre et longues griffes.
29. MON-061-0 : démon cornu aux sabots, chair ocre et épaules sombres.
30. MON-062-0 : démon aux plaques osseuses, chair rouge et longues griffes.

Ce sont des libellés de revue. Les faces et dos installés confirment deux
jambes porteuses et deux bras libres pour chacun. Conserver les proportions
propres à chaque démon, le petit volume de la tête, les cornes recourbées,
les sabots fendus de MON-061 et la répartition des plaques de MON-062.
Les griffes appartiennent aux mains ; elles ne deviennent ni armes ni appuis.
De dos, montrer les véritables matériaux dorsaux et occulter visage et torse.

Les guides de MON-034 validé servent seulement pour les appuis alternés,
sans transférer ses couleurs orangées, ses poings, ses cornes ou son gabarit.
Huit figures entières par direction, pas courts et genoux lisibles à 400 ms
par avancée, 800 ms par cycle. La tête et le torse restent solidaires,
les bras oscillent peu et aucun membre n’est greffé ou déformé par bandes.
Une échelle uniforme par direction et une translation du cycle alignent
les appuis. Les repos conservent exactement les frontaux installés,
leurs ancrages et leurs ombres.

Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-10/r01/manifest.json`.
Planches dessinées avec l’outil intégré image_gen, en mode édition avec
conservation d’identité. Prompts et références exacts dans
`MONSTER-VERTICAL10-PROMPTS.json`. Conserver les sources originales,
la transparence et les couleurs mates de référence.

Les deux premières planches de MON-060 rapprochaient trop les bras et
réduisaient sa largeur à environ 70–76 % du frontal. Deux reprises ciblées
restaurent l’écartement des bras sans élargir le pas. Les fichiers
`MON-060-down-broad-source.png` et `MON-060-up-broad-source.png` sont retenus ;
les deux essais étroits restent tracés comme non sélectionnés. Au total,
huit générations, dont six sélectionnées, et seize références d’entrée sont
consignées dans les relevés `PROMPTS` et `GENERATIONS` du lot.

`MONSTER-VERTICAL10-COLORS.json` relève les matières visibles de chaque
direction sans modifier les PNG. Les différences de surfaces visibles ne
mesurent pas à elles seules un changement d’éclairage ; vérifier les captures
des trois colonnes avec le décor, l’exposition et la taille native identiques.

La préparation doit vérifier les 159 empreintes antérieures, dont les
dessins bipèdes rejetés de MON-032 conservés pour traçabilité et sa correction
quadrupède validée. Les dix atlas latéraux intégrés restent inchangés.
Préparer les ressources avant affichage ; créer les textures sur le thread
graphique et ne pas lire les planches dans le chemin de dessin.

Statut lors de la proposition : candidats 28–30 en revue avant intégration.
Les 27 références précédentes restent disponibles, avec une seule entrée
quadrupède du n°21. Trente modèles proposés après ce lot ; 99 autres restent
à reprendre, puis les variantes restantes à contrôler. Les scènes de revue
ne chargent aucune partie ni préférence et n’activent pas ces cycles en
partie normale. Les identifiants, rôles et règles de jeu restent inchangés.

```powershell
python tools/art/prepare_monster_vertical10.py --references
python tools/art/prepare_monster_vertical10.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical10 artifacts/animation-polish-01/monster-vertical-10/r01/current
python tools/art/build_monster_vertical10_review.py
python tools/art/check_monster_vertical10.py
```

Vérification du 3 novembre 2026 : compilation verrouillée réussie ; six tests
de palette existants réussis. Les captures finales valident 48 figures de
marche distinctes, six repos exactement identiques aux frontaux installés,
198 images et trois animations natives de 2 400 ms incluant les deux sens
et les arrêts. Les largeurs médianes des silhouettes mobiles représentent
0,84 à 1,00 fois le frontal ; aucune échelle variable par phase n’est utilisée.
Les 159 empreintes antérieures, les huit sorties de génération, leurs seize
références et les dix atlas latéraux intégrés sont vérifiés.

Le lecteur a été exécuté dans le navigateur : trente sélections uniques
chargent chacune leurs 66 captures, six arrêts manuels sont contrôlés et
les trois nouveaux modèles passent du bas au haut à 400 ms. Le canevas
576 × 320 reste affiché à sa taille native, sans lissage. Les 129 cartes
d’inventaire sont présentes. Les quadrupèdes conservent leurs libellés de
pattes ; les nouveaux bipèdes affichent leurs phases de pieds. Une seule
entrée de MON-032 pointe vers sa correction quadrupède acceptée. Aucune
erreur JavaScript n’est relevée.

Preuves : `artifacts/animation-polish-01/monster-vertical-10/r01/browser-checks.json`,
`browser-proof.png`, `browser-back-proof.png` et `native-proof.png`.
Bilan durable : `MONSTER-VERTICAL10-VALIDATION.json`. Les candidats 28–30
restent à examiner par le joueur avant leur intégration en partie normale.

Retour du 3 novembre 2026 : le joueur valide le reste du lot mais refuse la
position des bras du n°28. Seuls les n°29–30 (MON-061/062) sont acceptés,
avec huit ressources figées dans `MONSTER-VERTICAL10-ACCEPTANCE-r01.json`.
Le n°28 conserve son frontal d’origine et demande une reprise des bras.
Sa version refusée reste traçable et sert de comparaison, sans être proposée
comme un second modèle accepté : `MONSTER-VERTICAL28-ARMS01.md`.
