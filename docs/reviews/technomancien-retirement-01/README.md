# TECHNOMANCIEN-RETIREMENT01 — 26 octobre 2026

Le joueur demande de supprimer le Technomancien et précise qu'il s'agit de la
classe jouable. Il demande aussi de retirer le grand personnage à droite de
l'écran de création. Ces deux changements sont intégrés.

Les nouvelles parties proposent Bastion, Traqueuse, Arcaniste et Métamorphe,
à la souris comme au clavier. Le grand portrait est supprimé et la description
utilise la largeur libérée. Les petites vignettes64px, l'équipement de départ,
les attributs, les commandes et les règles des quatre classes sont conservés.

Le départ historique du Technomancien reste résoluble par son identifiant, avec
`available_at_creation: false`. Ses valeurs n'ont pas changé. Les sauvegardes
existantes gardent leurs règles, apparence, compétences et barre d'actions ;
elles ne sont ni converties ni supprimées. Les compétences communes de drones
et d'affaiblissement sont conservées. Les identifiants graphiques ne sont pas
renumérotés. Les dessins C13 du Technomancien sont retirés de la revue active,
sans validation ni intégration ; ceux du Bastion restent en attente d'avis.

Les18 tests ciblés portent sur les départs, les armes et armures, les choix de
création, le catalogue commun et la compatibilité des sauvegardes. Le test
spécifique de retrait vérifie une reprise du Technomancien par cache et par
rejeu après changement d'empreinte de compilation. Le camp, la rue et l'accès
aux égouts sont aussi vérifiés pour les quatre départs conservés.

Le dernier changement d'affichage est compilé en Debug et Release puis contrôlé
dans le moteur à1280×800 et960×540 : quatre choix, textes lisibles, grand
portrait absent, vignettes conservées. Captures reproductibles :
`artifacts/technomancien-retirement-01/creation-final/cold-start.png` et
`creation-small-final/cold-start.png`. Ces diagnostics utilisent une partie
isolée ; aucune sauvegarde du joueur n'est lue ou modifiée.

`cargo fmt --all -- --check`, `cargo check --locked --all-targets` et
`git diff --check` passent. Le Release utilisé par `Jouer.cmd` est reconstruit.
Les commandes, résultats et empreinte sont consignés dans `validation.json`.
La commande `--verify-assets` exige un manifeste de distribution et ne s'applique
pas au dossier de développement brut ; son refus de ce dossier n'est pas une
preuve d'absence de ressources. Aucun paquet portable n'est reconstruit ici.

Le diagnostic refuse d'écraser une capture existante. La suppression de la
première capture a été rejetée par le contrôle automatique ; elle est conservée,
et les captures finales sont dans les deux sous-dossiers dédiés ci-dessus.
