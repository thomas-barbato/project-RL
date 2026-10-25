# WALK05-STEP03 — appuis et retour au repos

Statut : accepté le 23 octobre 2026 par « tres bien :) on peut passer à la suite :) ».
Voir visual-acceptance-01.json et REGLE-MARCHE-LATERALE.md pour le périmètre.
HEAD02 reste rejeté. La banque complète n'est pas prête : ce candidat couvre
uniquement CLASS-10 avec le fusil existant.

## Différence trouvée dans l'ancien aperçu

Le « 02 » d'origine utilise une planche de marche **et une planche de repos**.
Quand le déplacement se termine, `loc.moving` devient faux et le rendu prend
la pose de repos correspondante. L'aperçu DIR01/HEAD02 gardait au contraire
l'image2 ou5 du cycle durant les520ms de pause. Une jambe pouvait ainsi rester
repliée pendant tout l'arrêt. Les comparaisons précédentes n'avaient donc pas
repris fidèlement le fonctionnement du « 02 », même pour leur Bastion témoin.

## Nouveau candidat

Les six poses gauches ont été dessinées comme personnages entiers, séparément,
avec contacts et passages d'appui distincts. La série droite les reprend dans
l'autre orientation. Deux dessins complets supplémentaires posent les deux
pieds au sol à l'arrêt. Aucun bas de corps n'est greffé et aucune jambe n'est
déformée par le conditionnement. Les fichiers sources sont dans
`assets/source-art/player-walk-review-05/step03/`.

Les deux séries de sources ont des résolutions différentes : chacune reçoit
une échelle uniforme pour le personnage entier et une base de pieds commune,
avec hauteur maximale58px conforme au gabarit de CLASS-10. Les sources, prompts
exacts et appels de l'outil intégré image_gen sont conservés dans
`step03-prompts.json` ; les métadonnées sont dans `step03-packaging.json`.

## Comparaison avec le « 02 »

Les images de marche et repos du Bastion d'origine sont réutilisées sans changer
leurs octets. Les deux acteurs partagent la même horloge : trois images par
case,400ms par défaut,520ms de repos, alternance des deux moitiés du cycle.
Le déplacement, son arrondi, sa courbe et le choix marche/repos reprennent le
code de l'aperçu d'origine. Les choix120/200/400ms restent disponibles.

La vérification rejoue les deux scripts en parallèle et compare601 instants
sur douze secondes. Les régions de l'atlas, positions et états marche/repos
du Bastion sont identiques à ceux du « 02 ». Les contrôles vérifient aussi les
deux directions, six poses, arrêt après une case, second appui et commandes.
Il s'agit d'un contrôle logique sous Node avec DOM/canevas simulés, pas d'une
exécution réelle dans le navigateur ou le jeu. Les PNG et le GIF natifs ont
été composés séparément ; le GIF comporte400ms de marche puis520ms de repos.

Résultats : `step03-renderer-checks.json`. Les planches natives claires/sombres
et les détails×2 sont sous `artifacts/player-walk-review-05/step03-*`.

Le joueur accepte ce candidat et la méthode à adapter aux autres contenus.
Cet accord ne valide pas les futures déclinaisons : une génération par pose
peut introduire des différences de détail qui restent à contrôler. Aucune
intégration ni modification haut/bas. `REVIEW_ACCEPTED` reste faux.
