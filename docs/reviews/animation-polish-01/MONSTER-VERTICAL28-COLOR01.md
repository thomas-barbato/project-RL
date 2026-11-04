# MONSTER-VERTICAL28-COLOR01-r01 — continuité des couleurs du démon rouge

Le 3 novembre 2026, le joueur signale : « aussi je sais pas pourquoi il change
de lumiere quand il bouge et qunad il est statique ». La reprise des bras
du n°28 n’est pas encore validée ; les n°29–30 restent validés et inchangés.

Le diagnostic distingue éclairage et peinture. Les trois colonnes du moteur
contrôlent le même gain d’acteur de 1,48, les mêmes lumières et le même trajet.
Les PNG de marche avaient une chair plus claire et moins rouge que le repos.
Sur la chair de face, les médianes RGB étaient 103/59/48 contre 94/42/25 au
repos ; de dos, 108/56/42. Les matériaux sombres étaient également plus clairs,
surtout de dos. Le saut apparaît au retour au frontal d’origine.

Correction : préparer des courbes de couleur fixes par matériau et direction,
communes aux huit poses de chaque direction. La chair rouille et les parties
sombres sont alignées sur les niveaux d’ombre, de milieu et de lumière du
frontal conservé. Aucun gain variant par phase et aucun changement des
réglages d’éclairage. Les PNG de la reprise des bras sont réutilisés tels
quels. Les couleurs sont préparées sur CPU avant création de la texture,
hors dessin. Alpha, silhouettes, poses, contours très sombres et deux repos
restent exacts. Aucun nouveau dessin ni outil de génération d’image utilisé.

Règle de palette :
`assets/source-art/animation-polish-01/monster-vertical-28-color/r01/MON-060-material-palette.json`.
Le manifeste de ce dossier référence les quatre sources inchangées du dossier
`monster-vertical-28-arms/r01`. Les empreintes et quantiles avant/après sont
dans `MONSTER-VERTICAL28-COLOR01-MATERIALS.json`. Les ombres/milieux/lumières
RGB des deux matériaux correspondent au repos dans les deux directions.

Comparatif : frontal conservé / couleurs précédentes / couleurs harmonisées.
Les deux colonnes mobiles montrent les mêmes poses avec les bras relâchés.
La colonne centrale garde la palette précédente pour juger le changement.
400 ms par avancée, 800 ms par cycle ; retour au frontal exact à l’arrêt.
Les trente modèles restent disponibles, avec une seule entrée du n°28 et
les n°29–30 toujours reliés aux ressources validées du lot 10. Aucune nouvelle
animation n’est activée en partie normale. Parties, préférences, sauvegardes
et règles de jeu restent indépendantes de cette revue.

Le test Rust sur les vraies ressources compare les deux directions et les
deux matériaux aux quantiles 10/50/90 % de chaque canal. Il vérifie également
chaque alpha et les pixels protégés, notamment les repos et les contours.
Les sept tests de palette passent. Les 171 ressources antérieures protégées
et les quatre sources de forme sont conservées.

```powershell
python tools/art/prepare_monster_vertical28_color.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical28-color artifacts/animation-polish-01/monster-vertical-28-color/r01/current
python tools/art/build_monster_vertical28_color_review.py
python tools/art/check_monster_vertical28_color.py
```

Statut : correction de couleur et reprise des bras du n°28 à examiner avant
intégration. Cet essai ne vaut pas validation des poses par le joueur.

Vérification finale : compilation verrouillée réussie ; sept tests de palette
réussis, format Rust et `git diff --check` réussis. La texture préparée modifie
11 161 pixels RGB en conservant les formes et tous les alphas. Le contrôle
natif vérifie seize poses distinctes, deux repos exacts, 66 images et une
animation de 2 400 ms avec les deux directions et arrêts. Le gain d’acteur
contrôlé reste 1,48 pour chaque colonne.

Exécution dans le navigateur : trente sélections uniques chargent leurs
66 captures sans nouvelle tentative de décodage ; deux arrêts manuels et
le passage automatique bas/haut à 400 ms sont contrôlés. Le canevas 576 × 320
garde sa taille native, les 129 cartes d’inventaire sont présentes et aucune
erreur JavaScript n’est relevée. Les n°29–30 chargent toujours les ressources
validées du lot 10. La légende distingue correctement les mêmes poses avant
et après harmonisation.

Preuves : `artifacts/animation-polish-01/monster-vertical-28-color/r01/browser-checks.json`,
`browser-proof.png`, `browser-back-proof.png` et `native-proof.png`.
Bilan durable : `MONSTER-VERTICAL28-COLOR01-VALIDATION.json`.

Retour suivant : « pendant son déplacement on ne voit plus son arme :) ».
La lame de la main droite avait été prise pour une griffe lors du dessin des
poses. La correction de couleur n’est pas une validation des poses sans
arme. Restaurer l’équipement original en conservant les bras relâchés et
les couleurs harmonisées : `MONSTER-VERTICAL28-WEAPON01.md`.
