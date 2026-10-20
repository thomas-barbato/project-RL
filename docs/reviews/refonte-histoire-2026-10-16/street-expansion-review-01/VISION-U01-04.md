# Tout le décor dans le cercle — VISION-U01-04

19 octobre 2026 · activé pour essai en jeu à la demande du joueur.

Après présentation, le joueur demande : « ok integre la map pour l'instant
j'ai envie de la tester en jeu :) ». Le rendu ci-dessous est désormais utilisé
pour le camp et la rue dans le lancement habituel. Il garde les paramètres
examinés, sans nouvelle règle de découverte du terrain. Cet essai ne vaut pas
acceptation artistique définitive. Voir `integration-assets.json` et le README.

Les paragraphes suivants décrivent la préparation et la vérification du prototype.

Le joueur propose : « on éclairera tout sur la map mais dans le cercle de
vision du joueur », puis précise : « ca me semble bien plus facile du coup ».
Cette direction est préparée comme variante visuelle, sans activation normale.

Le cercle détermine l'affichage de la scène statique. Les murs n'y créent plus
de poches noires, dedans comme dehors. Une frange de 1,2 case adoucit la limite,
sans agrandir la portée. La pénombre et les lumières restent calculées par le
moteur ; afficher le décor ne signifie pas l'éclairer en plein jour.

Les obstacles continuent à bloquer les déplacements, la ligne de vue des acteurs
et les attaques. Les entités, objets interactifs et effets de combat restent
filtrés par la perception réelle. Les toits suivent les portes et la présence
du joueur comme auparavant. Leur exposition n'est plus découpée par les murs
dans cette variante : le masque circulaire final limite leur affichage.

Le sol déjà connu hors du cercle reste assombri à 45 %. L'espace inconnu au-delà
reste noir. Le prototype ne découvre pas de cellules de jeu : afficher du décor
derrière un mur n'autorise pas automatiquement le ciblage ou un déplacement sur
cette case. La mémorisation éventuelle de ce décor seulement aperçu par le
cercle devra être raccordée explicitement lors de l'intégration ; elle n'est pas
simulée en modifiant une sauvegarde dans cet aperçu.

Le mode circulaire n'utilise pas les index d'objets de VISION-U01-03. Il supprime
les deux passages de brouillard précédents et applique un seul masque final,
après les lumières et les toits. Le chemin de partie normale reste inchangé.
Le mode est réservé au diagnostic debug `--vision-circle`.

Huit comparaisons réelles du moteur conservent le même état de partie, la même
position, les mêmes portes et lumières. Les cases restent à 64 px, sans zoom.
Le rendu vérifie l'absence de mutation de simulation et le retour au camp.
Les deux côtés incluent la correction de grille de VISION-U01-02, afin de
comparer seulement l'affichage. Voir `vision-checks-04.json`.

Contrôles réussis : compilation debug, contrôle release, formatage et vérification
des espaces du diff ; deux tests du masque de revue et le test de non-révélation
de terrain dans la mémoire ; huit paires GPU à la même position, rendu en lecture
seule, retour réel au camp. Les captures ont été examinées sur la rue, la cour,
le logement ouvert et le camp. Ces résultats ne valent pas validation visuelle.

```powershell
cargo build --locked
target/debug/project-rl.exe --ui-cold-street-expansion artifacts/street-expansion-review-01/vision-04/engine --vision-layout --vision-circle
python tools/reviews/show_world_visibility_review.py --circle
```

L'accord sur le principe ne vaut pas acceptation des pixels de ce prototype.
AGENTS.md demande : « Ils doivent être validés explicitement par le joueur. »
Cette revue doit être examinée avant son activation dans le lancement normal.
