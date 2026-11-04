# PLAYER-MOTION-FIX01 — direction verticale et éclairage du joueur

Le joueur signale la disparition de la marche haut/bas du joueur et un éclairage
différent entre face et côté pour la Traqueuse. L'audit en mémoire de sa sauvegarde
reproduit un Bastion avec arme 67, armure 109 et palette d'objet Blue. Cette
palette suffisait à exclure le personnage de VERT18. Les déplacements verticaux
recevaient alors une pose frontale fixe et utilisaient le vieux rendu générique.
La Traqueuse colorée subissait le même retour à un autre frontal, donnant un
aspect plus clair et une identité différente de sa marche latérale.

La sélection verticale accepte maintenant les palettes d'objets, en conservant
les couleurs peintes des tenues de classe comme le fait déjà le rendu latéral.
Les couleurs et les silhouettes des dessins sources ne changent pas. La teinte
de l'objet, son identité et ses bonus restent dans l'état de jeu. Les clés de
cache continuent à distinguer les couleurs des armes, indépendamment de l'armure.

Les tenues intermédiaires à mains nues étaient également exclues, faute d'une
famille nommée `one`. Elles réutilisent leur corps et leurs mains verticaux déjà
dessinés dans la famille `ranged`, sans ajout d'arme. Les autres associations
d'armes gardent leurs limites de validation C11/C12. Le sélecteur de pose est
partagé entre la carte réelle et le diagnostic, pour tester le même routage.

Le réglage Discret reste commun : gain 1,18, relèvement 0,30, bord 0,012. Le
diagnostic vérifie ces trois valeurs pour chaque orientation et au repos. Les
frontaux colorés de la Traqueuse retrouvent les mêmes pixels de tenue validée
que la palette d'origine, sur cinq degrés d'armure. Aucun filtre supplémentaire
n'est appliqué à une orientation.

La vérification couvre les vingt profils et toutes les palettes, à mains nues
et avec une arme existante. Le diagnostic natif couvre quatorze cas, quatre
directions, six dessins distincts dans chaque direction et 364 contrôles
d'exposition. La sauvegarde réelle est restaurée uniquement en mémoire pour
vérifier les directions -2/+2, avec les paramètres du joueur.

Une ancienne comparaison de pixels VERT18 utilisait encore le fusil antérieur
à FIREARM02 et échouait déjà sur ce changement de géométrie. Elle désactive
désormais explicitement cette évolution lors de la comparaison avec ses
références historiques. Le rendu normal garde FIREARM02 actif. Les comparaisons
actuelles du diagnostic s'effectuent avec le rendu normal.

Les sources et accords des dix monstres intégrés sont préservés. Le joueur
demande aussi une reprise visuelle de certaines marches haut/bas des monstres :
elle reste un chantier séparé, dont les modèles prioritaires sont à identifier.
Cette correction ne redessine pas les monstres.

Capture : `--ui-cold-player-motion-fix OUTPUT`, puis
`tools/art/check_player_motion_fix.py`. Preuves dans
`artifacts/animation-polish-01/player-motion-fix-01/`. Les diagnostics utilisent
un état isolé et ne lancent pas de campagne sur les sauvegardes personnelles.

Les cinq tests verticaux actifs, les dix-neuf tests de déplacement et l'audit
isolé de la sauvegarde passent. Deux comparaisons exhaustives de banques
historiques restent explicitement ignorées. Le contrôle des monstres retrouve
660 captures latérales identiques et 120 vues verticales inchangées. Les deux
compilations, la mise à jour de la distribution et les lanceurs réels passent.
Les sept fichiers personnels contrôlés, sauvegardes et préférences comprises,
restent identiques. Aucune campagne utilisateur n'est lancée par ces contrôles.
