# MONSTER-VERTICAL28-ARMS01-r01 — bras relâchés du démon rouge

Le 3 novembre 2026, le joueur refuse la posture des bras du n°28, MON-060-0,
et valide le reste du lot 10. Les n°29–30 (MON-061/062) restent inchangés.
L’accord partiel et la version refusée du n°28 sont consignés dans
`MONSTER-VERTICAL10-ACCEPTANCE-r01.json` : huit ressources acceptées,
trois ressources refusées conservées et un frontal de référence conservé.

La correction porte sur les bras du seul n°28, de face et de dos. Abaisser
les épaules et les coudes, laisser les avant-bras descendre le long du corps,
garder les poignets neutres et les longues griffes ouvertes vers le bas.
Faible balancement en opposition des jambes. La posture précédente formait
un arc rigide et trop écarté ; sa largeur totale ne doit pas être imposée
au prix d’une articulation étrange. La réduction de largeur vient de la
position des bras ; ne pas compenser en écartant les jambes ou en grossissant
le torse. Garder le petit volume de tête, les cornes, le torse étroit,
le pas compact et les couleurs mates du candidat précédent.

Deux planches de huit figures entières, puis leur neuvième repos de guide,
dessinées avec image_gen intégré, en mode édition. Références : planche de
marche précédente à corriger et référence d’identité de la même vue.
Prompts exacts : `MONSTER-VERTICAL28-ARMS01-PROMPTS.json` ; sorties et
empreintes : `MONSTER-VERTICAL28-ARMS01-GENERATIONS.json`. Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-28-arms/r01/manifest.json`.
Le repos de l’atlas conserve exactement le frontal installé dans les deux
directions ; le neuvième dessin généré n’est pas substitué à cette référence.

Comparatif natif : frontal conservé / bras précédents / bras relâchés.
La colonne centrale utilise l’atlas refusé du lot 10, avec la même phase,
la même taille, le même trajet, les mêmes ombres et la même exposition.
Les textures sont préparées avant affichage sur le thread graphique.
La légende et les choix de colonnes suivent la sélection réelle du lecteur.
Les n°29 et 30 chargent leurs ressources validées, sans retouche.

Préparation et contrôles : 171 empreintes antérieures préservées, dont les
huit ressources acceptées du lot 10 et les références refusées du n°28.
Seize figures de marche distinctes, deux repos exacts, 66 captures natives
et une animation de 2 400 ms comprenant les deux directions et les arrêts.
Cadence : 400 ms par avancée et 800 ms par cycle. Échelle uniforme par
direction, ancrage sur la figure entière, aucune greffe ni déformation de
bande. Les dix atlas latéraux intégrés restent inchangés. Aucune partie,
préférence, règle de jeu ou sauvegarde n’est chargée ou modifiée.

Statut : nouvelle posture du n°28 à examiner avant intégration. Une seule
entrée du n°28 figure dans le lecteur ; la posture refusée reste en colonne
de comparaison. Les trente modèles proposés restent disponibles. Les
99 autres modèles et les variantes restantes attendent toujours leur revue.

```powershell
python tools/art/prepare_monster_vertical28_arms.py --references
python tools/art/prepare_monster_vertical28_arms.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical28-arms artifacts/animation-polish-01/monster-vertical-28-arms/r01/current
python tools/art/build_monster_vertical28_arms_review.py
python tools/art/check_monster_vertical28_arms.py
```

Compilation verrouillée et six tests de palette réussis. Les nouveaux bras
réduisent la largeur médiane de 36 à 21 px de face et de 39 à 23,5 px de dos,
avec une hauteur de référence conservée de 47 px. Ces mesures décrivent la
reprise ; elles ne remplacent pas la revue visuelle des articulations.

Le lecteur a été exécuté : trente sélections uniques chargent leurs
66 captures. La nouvelle posture est l’unique entrée de MON-060 ; les
n°29–30 pointent toujours vers leurs ressources validées du lot 10.
Deux arrêts manuels et l’alternance automatique bas/haut à 400 ms sont
contrôlés. La légende distingue bien frontal, bras précédents et reprise,
puis retrouve les intitulés ordinaires sur les autres modèles. Le canevas
576 × 320 conserve sa taille native ; les 129 cartes d’inventaire sont
présentes. Aucun message d’erreur JavaScript n’est relevé. Le chargement de
MON-014 a nécessité une resélection ; ses 66 PNG sont valides et le second
chargement a réussi. Ce détail figure dans le relevé de contrôle.

Preuves : `artifacts/animation-polish-01/monster-vertical-28-arms/r01/browser-checks.json`,
`browser-proof.png`, `browser-back-proof.png` et `native-proof.png`.
Bilan durable : `MONSTER-VERTICAL28-ARMS01-VALIDATION.json`. Une nouvelle
exécution du contrôle du lot 10 conserve son acceptation partielle et ne
réactive pas les bras refusés du n°28.

Retour suivant du joueur : la lumière semble changer entre marche et repos.
Ce retour ne valide pas encore la reprise des bras. Les mêmes dessins sont
repris pour une correction de palette, sans nouvelle peinture ni changement
de pose : `MONSTER-VERTICAL28-COLOR01.md`.
