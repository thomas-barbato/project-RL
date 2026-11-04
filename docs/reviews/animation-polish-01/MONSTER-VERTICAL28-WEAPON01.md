# MONSTER-VERTICAL28-WEAPON01-r01 — lame d’origine rétablie

Le 3 novembre 2026, le joueur signale : « pendant son déplacement on ne voit
plus son arme :) ». La lame claire du n°28, MON-060-0, avait été prise à tort
pour une griffe lors de la reprise des bras. Le dessin frontal installé et
le guide arrière montrent la même lame tenue dans la main droite anatomique.
Au début de cette reprise, les n°29–30 sont validés ; le n°28 reste à valider.

Restaurer cette unique lame dans les huit poses de chaque direction :
à gauche de l’image de face, à droite de dos. Main droite fermée autour du
manche, acier gris froid à pointe abaissée et légèrement éloignée de la
jambe, main gauche libre. Garder les épaules relâchées, les coudes abaissés,
les pas courts, les cornes et le petit volume de tête. Ne pas inventer de
seconde arme, changer son type ou grossir le corps pour la rendre visible.

Deux planches dessinées avec image_gen intégré, en mode édition. Cibles :
les planches des bras relâchés ; références d’identité et d’arme : le frontal
installé et le dos installé du même monstre. Prompts et références exactes :
`MONSTER-VERTICAL28-WEAPON01-PROMPTS.json`. Sorties et empreintes :
`MONSTER-VERTICAL28-WEAPON01-GENERATIONS.json`. Atlas et sources :
`assets/source-art/animation-polish-01/monster-vertical-28-weapon/r01/manifest.json`.
La neuvième figure générée ne remplace pas le repos : les deux cellules
de repos conservent exactement le frontal d’origine, avec son arme.

La chair et les parties sombres gardent le principe de palette harmonisée
avec le repos, par matériau et direction, commun aux huit poses. L’acier
ne subit pas cette transformation : ses pixels restent identiques dans la
texture préparée. Le test Rust vérifie les vrais PNG, chaque alpha, les
repos, les pixels d’acier et les quantiles RGB de chaque matériau. Les huit
tests de palette réussissent. Les 171 ressources antérieures protégées et
les dessins validés des n°29–30 restent conservés.

Le comparatif natif montre frontal conservé / arme absente / arme rétablie.
La colonne centrale utilise les bras relâchés avec leur palette harmonisée
précédente ; le changement de couleur ne fausse donc pas la comparaison.
La préparation lit atlas et palettes avant création des textures sur le
thread graphique ; aucune lecture lourde dans le dessin. Même lumière,
gain d’acteur 1,48 et cadence de 400 ms par avancée, 800 ms par cycle.

Le contrôle des seize poses relève un noyau d’acier clair du bon côté de
la silhouette, à hauteur de main et de jambe. Il reste dans le composant
alpha principal du corps, sans arme flottante. Le seuil de visibilité est
comparé au noyau métallique du frontal retenu. Cette mesure ne remplace
pas la revue native de la prise et du dessin de la lame.

Les identifiants, rôles, règles, butin et équipement mécanique ne changent
pas. Aucune partie ni préférence n’est chargée. La nouvelle marche reste
dans le circuit de revue ; aucune intégration en partie normale. Les trente sélections
restent disponibles, avec une seule entrée du n°28 et les n°29–30 validés.

```powershell
python tools/art/prepare_monster_vertical28_weapon.py --references
python tools/art/prepare_monster_vertical28_weapon.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical28-weapon artifacts/animation-polish-01/monster-vertical-28-weapon/r01/current
python tools/art/build_monster_vertical28_weapon_review.py
python tools/art/check_monster_vertical28_weapon.py
```

Statut : lame rétablie, bras relâchés et couleurs harmonisées validés ensemble
le 3 novembre 2026 par « ok parfait c'est validé :) ».

Vérification finale : compilation verrouillée et huit tests de palette
réussis. Les sources de dessin et les quatre références d’entrée de génération
sont vérifiées par empreinte. Le contrôle natif valide seize poses distinctes,
deux repos exacts, 66 captures et une animation de 2 400 ms. Sur chacune des
seize poses, le noyau d’acier lisible se trouve du bon côté et appartient au
composant alpha principal de la figure. La comparaison applique bien la
palette harmonisée précédente à la colonne centrale sans arme.

Le lecteur a été exécuté : trente sélections uniques chargent leurs captures,
sans erreur JavaScript ni nouvelle tentative de décodage. Deux arrêts manuels
et la lecture automatique bas/haut à 400 ms sont contrôlés. Les n°29–30 sont
toujours reliés aux ressources validées. Les trois colonnes font 576 × 320 px
natifs ; la preuve dans la fenêtre étroite utilise les deux colonnes mobiles
de 384 × 320 px, également à taille native. Les 129 cartes d’inventaire sont
présentes. Format Rust et `git diff --check` réussis.

Preuves : `artifacts/animation-polish-01/monster-vertical-28-weapon/r01/browser-checks.json`,
`browser-proof.png` et `native-proof.png`. Bilan durable :
`MONSTER-VERTICAL28-WEAPON01-VALIDATION.json`.

Cette acceptation porte sur MON-060-0, n°28, dans les deux directions à
400 ms par avancée. Les cinq ressources finales (atlas, frontal, deux
planches sources et palette) sont figées par empreinte dans
`MONSTER-VERTICAL28-WEAPON01-ACCEPTANCE-r01.json`. Les n°29–30 restent
acceptés par le lot 10 ; la posture initiale refusée du n°28 conserve son
statut historique. L’accord visuel n’active pas le cycle en partie normale.
