# WALK05-DIR01 — correction de l'orientation de l'aperçu

**Rejeté par le joueur le 23 octobre 2026.** « tu vois bien que c'est pas la meme chose ».
Les sens gauche/droite sont distincts, mais cette correction ne reproduit pas
la marche validée du Bastion. Ne pas employer ces sources pour étendre la banque
ou pour réactiver l'animation dans le jeu.

La comparaison des six poses montre que la Traqueuse conserve presque toujours
la même jambe devant ; les variantes replient surtout l'autre jambe et donnent
un effet de sautillement. Le Bastion conserve une présentation du torse plus
frontale et un mouvement plus compact. La qualité du cycle n'a pas été validée
par les contrôles du sens, du format et des commandes de l'aperçu.

La tentative précédente à deux lignes montrait deux orientations vers la droite.
Elle n'est pas retenue. Le nouvel essai traite gauche et droite séparément,
avec leurs propres références de personnage entier ; la tenue n'est pas retournée.

L'essai rejeté ici concerne uniquement CLASS-10, Traqueuse avec fusil.
La première ligne de l'aperçu va à gauche, la seconde à droite. Le sélecteur
permet de retrouver les deux lignes latérales du Bastion d'origine.
Le reste de la banque reste à préparer : cette revue ne revendique aucune
couverture des autres armures, personnages ou monstres.

Sources complètes générées avec l'outil intégré image_gen :
- `assets/source-art/player-walk-review-05/traqueuse-rifle-left-r03.png`
- `assets/source-art/player-walk-review-05/traqueuse-rifle-right-r03.png`

Prompts exacts : `direction-correction-prompts.json` dans ce dossier.
Les personnages entiers sont extraits par appartenance des composantes,
réduits avec une échelle commune et alignés sur une même base des pieds.
Cette extraction évite de couper un canon qui dépasse la cellule théorique.
Aucune jambe n'est déformée, greffée ou remplacée au conditionnement.

Contrôles réalisés : douze images différentes, dimensions natives64,
transparence, silhouettes sur fond sombre et clair, orientation des visages
et canons sur les douze poses. Vérification logique sous Node avec canevas
simulé : sens opposés des deux lignes, lignes correctes de l'atlas, trois
images par case, arrêt après une case et commandes image/durée/référence.
Ce contrôle logique n'est pas une exécution dans un navigateur ni dans le jeu.

Le résultat de la marche est rejeté. Aucune
validation artistique ni règle générale de production n'est enregistrée.
Les essais du moteur restent désactivés (`REVIEW_ACCEPTED = false`).
Les déplacements haut/bas et les règles du jeu ne sont pas modifiés.
