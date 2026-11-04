# BIPED-LATERAL01-r03 — référence directe du cycle joueur

**Validation du 2 novembre 2026 :** « je valide, on mettra ces animations en
400ms :) ». Les dessins de MON-144-0 présentés sont acceptés, avec 400 ms par
pas (800 ms par cycle). La cadence de 650 ms ci-dessous décrit la capture
initiale. Le lecteur et son export animé reprennent désormais 400 ms.
Le périmètre exact et les empreintes sont dans `BIPED-LATERAL01-ACCEPTANCE-r03.json`.

Le joueur demande pourquoi les monstres ne reprennent pas simplement la
démarche des personnages joueurs. Après l'explication, « ok continue » autorise
la reprise avec cette référence. R02, sa marche rasante et son trajet réduit
restent une tentative remplacée, sans validation visuelle ni intégration.

La cible actuelle reste MON-144-0, sans arme, en palette d'origine. Les poses
sources viennent directement des pixels du Bastion lourd sans arme de
CLASS-LATERAL02-r02 : huit phases et un frontal, dans chaque direction.
`player-reference.json` conserve le chemin, l'empreinte et les timings.
Les planches de fabrication montrent ces vrais dessins dans une grille 3×3.
Elles ne sont plus les schémas de démarche des deux premières tentatives.

L'outil intégré image_gen redessine le démon entier dans les poses données
par le joueur de référence, avec sa carrure, ses griffes et ses deux ailes.
Les sources sont conditionnées par réduction uniforme sur X/Y de la figure
entière, puis inscrites dans un atlas natif avec couronne et appui stables.
Cette normalisation corrige la dérive de taille des sorties générées avant
la création des textures. Aucun membre n'est déplacé, greffé ou cisaillé ;
le rendu n'effectue aucune correction de taille entre phases.
La reprise de registration générée pour la planche gauche n'améliorait pas
son alignement et n'est pas retenue ; la source gauche initiale est conservée.

Le moteur montre côte à côte le véritable Bastion r02 et le nouveau démon.
Tous deux utilisent le même index de phase, les mêmes durées
180/180/170/120/180/180/170/120 ms, soit 650 ms par pas, et le même trajet de
40 px par cycle dans l'étude. Cette synchronisation est contrôlée sur les
64 échantillons animés. Elle garantit le déroulement temporel et spatial de
la comparaison ; la qualité de l'adaptation des articulations reste à juger
visuellement, ce n'est pas un système de contraintes de squelette.

Les deux retours au repos utilisent les véritables frontaux du rendu normal.
L'ancienne tentative r02 reste disponible dans une autre comparaison. La
lumière du camp est commune ; chaque acteur conserve son traitement habituel
(Discret pour le joueur, exposition de monstre pour le démon). Les pixels du
joueur sont conservés, avec sa taille native ; aucune mise à l'échelle du
joueur dans la comparaison n'est ajoutée pour égaliser sa taille au démon.

Les 66 captures sont préparées avant la lecture, dans le lecteur et son cache
borné. La comparaison par défaut conserve deux colonnes natives (320×192),
avec option trois colonnes, pause, poses, directions et zoom entier ×2.

État : dessins de MON-144-0 acceptés à 400 ms par pas, non intégrés. Les autres
monstres et les prises d'armes restent à reprendre, conformément à la procédure de
revue latérale. Les dessins, règles, sauvegardes et timings du jeu normal
restent conservés. Les anciens éditeurs de l'aperçu republiant les bipèdes
redirigent vers r03, pour ne pas réafficher r01 ou r02 comme candidats actuels.

Reproduction : `--ui-cold-biped-player-r03 OUTPUT`, puis
`tools/art/build_biped_player03_review.py`. Sources sous
`assets/source-art/animation-polish-01/biped-lateral-01/r03/`, preuves sous
`artifacts/animation-polish-01/biped-lateral-01/r03/`. Les prompts complets sont
dans `BIPED-LATERAL01-PROMPTS-r03.md`.
