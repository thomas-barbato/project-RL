# WALK06 — les 25 tenues de classe au fusil

Propositions préparées le 23 octobre 2026, après l'accord WALK05-STEP03.
Le joueur accepte ensuite les25 déclinaisons et autorise leur intégration :
« je pense qu'on peut integrer :) ». Voir `acceptance.json` pour le périmètre exact.
La demande « toutes les armures sur tous les personnages » concerne aussi
les équipements et les anciennes banques, pas seulement cette première série.

## Contenu présenté

01–05 Bastion, 06–10 Technomancien, 11–15 Traqueuse, 16–20 Arcaniste,
21–25 Métamorphe. Chaque personnage possède les cinq couvertures d'armure
prévues par le rendu actuel. Le numéro11 réutilise exactement la Traqueuse
déjà acceptée, sans nouvelle génération ni changement de ses pixels visibles.

Chaque tenue comporte deux orientations latérales, six dessins entiers de
marche et une vraie pose de repos par orientation : 350 poses natives64.
Trois dessins par case, puis l'autre moitié au pas suivant ;400ms par défaut,
choix120/200/400. La pause520ms sert à observer l'arrivée dans la démonstration.
Elle n'ajoute aucun délai au jeu. Les déplacements haut/bas restent inchangés.

Le fusil est dessiné avec le sujet entier dans ce lot. Ce lot ne constitue
pas encore une banque de couches interchangeables pour toutes les armes.
Les autres prises, les six casques équipables, les anciennes tenues, les PNJ,
les formes et les monstres restent des adaptations à préparer et présenter.
Les couvre-chefs déjà propres au dessin d'une tenue sont conservés.

## Compatibilité et preuves

`equipment-mapping-audit.json` relit le sélecteur existant
`src/game_art_class_armor.rs::outfit_index` : cinq classes,20 choix de torse
et sept choix de tête donnent700 cas physiques,175 apparences avec casques.
Plusieurs objets sélectionnent la même couverture d'armure. Les correspondances,
identifiants, bonus et règles du jeu ne sont pas modifiés.
Ce relevé vérifie le périmètre ; il ne valide pas le rendu des casques absents
de cette série ni une compatibilité déjà exécutée en jeu.

`asset-audit.json` vérifie les25 découpes sans avertissement de composant détaché,
les empreintes des sources acceptées et leurs pixels visibles dans le nouveau
conditionnement. Les fichiers WebP sont sans perte, avec égalité RGBA après
décodage. `packaging.json` décrit les cinq vues de revue sous1Mo chacune.

`renderer-checks.json` compare601 instants aux animations du lecteur accepté :
deux directions, six phases, arrêt réel, pas unique, durée réglable et limites
du nouvel atlas. Exécution JavaScript dans Node avec DOM/canvas simulés ;
aucune exécution dans un navigateur réel ou dans le jeu n'est revendiquée.

Les planches ont été inspectées à taille native, sur fonds sombre, clair et
texturé. Les cinq vues proposent aussi un zoom entier×2. Cette inspection
technique ne remplace pas la validation visuelle du joueur.

## Sources et statut

Les dessins proposés sont conservés dans
`assets/source-art/player-walk-review-06/CLASS-XX-rifle-r01.png`.
Références, consignes exactes et provenance sont dans `references/`, `prompts/`,
`jobs.json` et les journaux `generation-*.json` de ce dossier. Les sorties
de conditionnement et de comparaison sont dans `artifacts/player-walk-review-06/`.
Fabrication : `prepare_player_walk_matrix_06.py`, `build_player_walk_matrix_06.py`.
Contrôles : `audit_player_walk_matrix_06.py`, `verify_player_walk_matrix_06.cjs`.

Statut : **lot accepté, intégration autorisée** pour le fusil d'assaut modèle2,
les palettes Original et les têtes propres aux25 tenues, sans casque supplémentaire.
Le rendu direct des atlas est raccordé par `src/game_art_walk_approved.rs`.
Le garde `REVIEW_ACCEPTED=false` de l'ancienne banque WALK04 reste faux et ne
concerne pas ces nouvelles ressources. Les autres configurations conservent
le compositeur courant. Les résultats d'exécution sont dans `integration-checks.json`.
Aucun commit ou publication.

## Suite demandée

Le joueur précise : « bien-sur il faudra faire la meme chose pour toutes les
armes :) puis pour les monstres du jeu ». L'ordre est donc armes et prises,
puis bestiaire. Cette demande conserve le périmètre complet de la méthode ;
elle ne valide pas les anciennes animations rejetées. Les nouveaux lots
seront montrés avant leur intégration, avec leur compatibilité armures/casques.
