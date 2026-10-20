# Contrôles des contours U01 r06

19 octobre 2026 · `CONTOURS-U01-06 / MAP-U01-EXP-01-r06`.

Reprise ciblée de l'entrepôt, du logement et de la boutique signalés par le
joueur. Les objets autour des façades ont été retirés des architectures.
Les accessoires replacés sont trois images complètes indépendantes. Les
variantes cuivre, rouille et acier concernent les conduits décoratifs ; les
voyants fonctionnels sont préservés.

Le dernier export conserve le prolongement de l'aile gauche, conformément
à la clarification du joueur. Le toit bleu couvre cette partie jusqu'au
mur sud complet ; les coins sont replacés aux extrémités et les gravats
inutiles sont retirés. Le raccourcissement précédent est abandonné.
MUR-SUD-03 corrige ensuite la hauteur de cette façade pour l'accorder à
celle du garage. Après présentation, le joueur valide MUR-SUD-03 le
19 octobre 2026 : « parfait là c'est bien :) ». L'accord et les empreintes
exactes sont dans `warehouse-south-wall-acceptance-r03.json`.

Contrôle artistique effectué sur fond contrasté, à taille native, puis dans
la scène : coins rentrants de l'entrepôt, base des murs, absence de caisse et
de boîtier tronqués, ancien feuillage coupé des intérieurs, seuils ouverts et
fermés. Les 6 ruines et 15 accessoires réutilisés ont aussi été examinés ;
aucune coupe comparable n'a été identifiée. Ce constat ne remplace pas
l'accord artistique du joueur.

Contrôles techniques :

- Construction du candidat : 695 cases libres reliées, aucun sol derrière
  la prison, trois bâtiments visitables, lentilles non recouvertes et
  empreintes de la carte active inchangées.
- Huit tests ciblés `prison_street` réussis ; le test de circulation du
  candidat est rejoué avec succès après les derniers exports des intérieurs
  puis de l'entrepôt.
- `cargo build --locked`, `cargo fmt --all -- --check`,
  `cargo check --locked --release` et `git diff --check` réussis.
- Capture isolée du moteur : 21 vues à 64 px par case, toits et portes,
  trois animations de 24 images, absence de mutation par le rendu et
  retour réel au camp. Les résultats exacts sont dans `render-r06.json`.
- Planche de revue dimensionnée depuis les images ; chaque source entière
  est contenue dans son encadré, sans masque de bâtiment ni découpe partielle.

Les dessins restent uniquement dans le candidat ; l'entrepôt MUR-SUD-03 est
validé, les autres ressources modifiées restent en revue. Les sources
r05 et les ressources de partie normale sont conservées.
