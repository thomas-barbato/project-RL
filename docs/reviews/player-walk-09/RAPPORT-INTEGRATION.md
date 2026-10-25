# WALK09 — intégration et observation des cultistes

Les cinq classes et leurs25 tenues utilisent maintenant la démarche commune
validée dans le lancement normal. Les350 poses gardent les figures entières,
les protections de jambes et les armes réellement équipées. La hachette67
utilise AX67-r02 au manche de bois affiné. Les mouvements latéraux parcourent
une seule case, en400ms par défaut, avec les choix120/200/400ms conservés.
Les dessins des déplacements verticaux restent ceux déjà acceptés.

L'accord exact et les empreintes sont dans `acceptance.json`. L'exécutable
Release utilisé par `Jouer.cmd` a été reconstruit ; `integration.json` conserve
son empreinte et le résultat des vérifications.

## Vérifications réalisées

Les40 tests ciblés passent :350 poses et possession des protections,113 modèles
d'armes sur les cinq classes avec le lecteur normal, hachette67, durée, appuis,
repos à l'arrivée et répétition des commandes. Le contrôle dans le moteur
exécute un vrai déplacement d'une case dans chaque sens, avec la hachette
équipée, puis compare les25 tenues sur fonds clair et sombre avec six
configurations d'armes. Les captures sont dans `integration/`.

La simulation ne change pas pendant le dessin. Les empreintes de la campagne,
des préférences graphiques et des contrôles du joueur restent identiques.
Les140 fichiers protégés des cycles de monstres et de l'éditeur de cartes
restent identiques. L'inventaire de distribution inclut les51 fichiers WALK09.
La banque rejetée WALK04 reste désactivée.

## Cultistes : défauts confirmés dans le rendu normal

L'observation utilise une scène de jeu isolée avec des cultistes en déplacement,
puis des cases natives64 montrant le repos, la gauche et la droite. Elle utilise
le lecteur normal, sans activer le contexte du catalogue. Elle ne modifie pas
la campagne du joueur.

| Modèles canoniques | Constat | Cause |
| --- | --- | --- |
| MON-022-1 et MON-024-1 | Glissent sans changer de pose ni d'orientation | Leur variante1 ne passe pas la sélection des cycles, réservée aux variantes0 dans le jeu normal. |
| MON-132-1,133-1,134-1,135-1 | Même absence d'animation latérale | Même restriction de variante ; les cycles de base existent. |
| MON-022-0 et022-1 | Hauteur visible49px | Le dessin et son cycle ont été préparés à cette petite taille. |
| MON-024-0 et024-1 | Hauteur visible48px | Même problème de gabarit. |

Les autres cultistes mesurés font59 à66px de haut à cette échelle. Les petits
modèles sont bien centrés verticalement ; le défaut concerne leur taille,
pas un décalage temporaire pendant la marche. Les versions0 de022 et024 sont
animées, tout en conservant cette petite taille. Les six variantes1 ci-dessus
restent statiques.

L'audit des471 couples de campagne relève129 bases animées et342 variantes
sans cycle actif dans le lecteur normal. Ce sont des couples modèle/couleur,
pas342 silhouettes différentes ni342 monstres présents dans une partie.
`integration/monster-audit.json` donne le détail, les tailles et les liaisons.
Les anciennes identités de type `Actor`, les PNJ, transformations et autres
drones ne font pas partie de cet inventaire de campagne.

## Suite identifiée

Raccorder les variantes aux cycles de base et vérifier leurs teintes ; remettre
les figures entières022/024 à un gabarit comparable aux autres cultistes, en
conservant un calage fixe au repos et pendant la marche. Ces changements de
monstres restent à présenter avant intégration, conformément à la revue
demandée. Les propositions de variantes et les neuf nouveaux cycles de
formes/drones WALK08 restent séparés de l'accord sur les personnages WALK09.

Les sources sélectionnées, consignes, règles et21 fichiers de preuves utiles
sont conservés. Les98 captures intermédiaires reproductibles du catalogue et
du diagnostic WALK09 sont passées à la Corbeille après copie de ces preuves,
soit52,7Mo retirés du dossier de travail. La suppression définitive a été
refusée par le contrôle automatique ; la Corbeille n'a pas été vidée.
`cleanup.json` conserve les chemins et empreintes des exports rangés.
