# Retrait des essais ASCII et correction du cercle de vision

## Retrait des essais

Les trois modules `terminal_ascii*`, leurs deux lanceurs, leurs routes de
lancement et de capture, les deux dossiers de sprites de démonstration, la
police JetBrains Mono et ses documents, ainsi que les trois dossiers de captures
des pilotes ont été supprimés. La maquette comparant A et B et son prompt ont été
retirés du dossier d'essai loot/vision sans toucher à ses autres preuves.
La documentation ne propose plus cette direction ni ses commandes.

Les sources, assets et documents actifs ont été vérifiés : aucune référence aux
pilotes retirés ne reste. Le rendu courant, les textures de l'éditeur et les
autres travaux du projet sont conservés.

## Cause et correction

Le masque original passait à un noir opaque sur les cases visibles de la frange,
alors que les cases déjà connues autour gardaient leur rendu de mémoire. La
capture `avant-memoire/cold-start.png` reproduit la couronne de carrés noirs.

Le fondu passe désormais à la teinte de mémoire là où le terrain extérieur est
connu et au noir là où il reste inconnu. La couleur de mémoire est commune avec
le rendu ordinaire des cases explorées. Des sommets partagés interpolent ce
raccord sans joints par case. Quatre voisins sont échantillonnés afin de garder
la continuation connue d'un couloir d'une seule case de largeur.

Le masque ne dessine que la frange des cases visibles et consulte uniquement les
données de terrain déjà mémorisées. La portée, les murs opaques, la simulation,
les sauvegardes, les acteurs et les effets ne changent pas.

## Vérifications natives

- `rendu-final-memoire/cold-start.png` : mémoire sur tous les côtés, 1280 × 800.
- `rendu-final-memoire-960/cold-start.png` : même cas à 960 × 540.
- `rendu-final-inconnu/cold-start.png` : terrain extérieur inconnu.
- `rendu-final-expedition/cold-start.png` : expédition et couloirs étroits.

Les captures mémoire et inconnu vérifient les pixels du GPU au bord de la portée
contre la case adjacente hors de portée. Les résultats se trouvent dans
`vision-pixels.json`. L'écart accepté est de deux niveaux sur 255, pour les arrondis
du mélange ; en mémoire, l'écart observé est d'un niveau sur le rouge.
Le terrain inconnu reste noir.

La zone de jeu de la capture vierge corrigée (rectangle 300,50–972,662) est
identique pixel pour pixel à la capture initiale du cercle, conservée dans
`artifacts/loot-vision-141-2026-10-02/vision-finale/cold-start.png`.
Dans le couloir de l'expédition, le pixel 636,612 passait à [0,0,0] avec le
raccord initial. Il vaut maintenant [6,12,17], contre [6,12,16] pour la case
mémorisée suivante (636,644). Le trou noir a disparu.

## Contrôles de code

- `cargo fmt --check` et `git diff --check` : succès.
- `cargo check --locked --all-targets` : succès.
- Compilation debug : succès.
- Tests ciblés de géométrie du cercle, de confidentialité du terrain mémorisé
  et de confinement des effets : trois tests réussis, logs `tests-*.log`.
- Les quatre captures natives finales ont réussi et leur texte a passé les
  contrôles de visibilité de l'interface.

La suite complète du moteur n'a pas été relancée. Aucun commit ni push.

La compilation release a réussi (`build-release.log`) et le démarrage du jeu
normal a été contrôlé pendant six secondes sans erreur standard. Seul le
processus créé pour ce contrôle a été arrêté. L'exécutable release est à jour.
