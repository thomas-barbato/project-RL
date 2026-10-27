# PLAYER-CONSISTENCY13 — Technomancien et contrôle des cinq classes

**26 octobre2026 — suite abandonnée pour le Technomancien.** Le joueur demande
de supprimer sa classe jouable, puis précise « Supprimer la classe jouable ».
Ses propositions C13 ne sont donc plus en attente d'intégration. Les dessins et
prompts restent des sources historiques non approuvées. Les trois propositions
du Bastion restent en revue ; aucun accord supplémentaire n'est déduit de ce
retrait. Voir `../technomancien-retirement-01/` pour le retrait du catalogue jouable.

26 octobre2026. Le joueur demande de retravailler le Technomancien, trop fin
et moins détaillé de face que pendant sa marche, puis de vérifier les autres
personnages. Les nouveaux dessins restent des candidats à revoir à taille native.

### Révision r02 après retour du joueur

La r01 n'est pas validée : le joueur trouve encore la face 1/5 trop fine et
signale un regard vers la droite pendant les trajets gauche des cinq tenues.
La tenue `TE-C13-01-r02` est redessinée entière : face, torse et membres plus
remplis, puis douze figures de marche au même nouveau gabarit. Aucun étirement
de silhouette, greffe de tête ou remplacement de jambe n'est utilisé. Les quatre
autres tenues du Technomancien et les trois propositions du Bastion sont conservées.

Le sens inversé signalé n'a pas été reproduit dans les fichiers C13 examinés :
les PNG n'ont pas d'orientation EXIF ; les six figures de la rangée supérieure
regardent vers la gauche et celles du bas vers la droite. Ce rangement est
conservé par le conditionnement et les trajets. L'optique ambrée est bien placée
du côté avant de chaque tête sur les60 poses. Ce contrôle de couleur vérifie le
rangement des rangées ; la lecture du crâne, de la mâchoire, du torse et des pieds
est également inspectée sur la planche entière. Aucune cause d'inversion n'est
affirmée sans reproduction et aucune rangée n'est retournée arbitrairement.

Le nouvel aperçu de référence est
[C13 r02 : gauche et droite](Technomancien-C13-r02-directions-native64.gif).
Il ne montre que les candidats : deux pistes simultanées explicitement nommées,
l'une parcourue de droite à gauche, l'autre de gauche à droite. Chaque trajet
couvre exactement une case de64px en400ms, utilise trois phases, puis reprend
la vraie face à l'arrêt. Les trois autres phases servent au pas suivant.
Le fond et les poses sont rendus directement à taille native, sans contrôle du jeu
ou navigateur. Les pauses de300ms sont réservées à cet aperçu.

La [face 1/5 avant/après](Technomancien-C13-r02-face-01-avant-apres.png) montre
la r01 et la r02 à taille native et en zoom x2. Les deux planches de contrôle
`Technomancien-C13-r02-toutes-phases-native64.png` et
`Technomancien-C13-r02-toutes-phases-zoom2.png` montrent les60 poses entières.
Prompts, outil intégré `image_gen` et provenance : [generation-r02.json](generation-r02.json).
Contrôles reproductibles : `tools/art/review_player_consistency_directions_13.py`
et [direction-review-r02.json](direction-review-r02.json).

Les nouveaux corps restent sans équipement ajouté dans cette revue. Les52
ressources runtime de référence sont inchangées ; aucune intégration n'est
autorisée par ce retour demandant des retouches. Revue visuelle toujours en attente.

L'audit couvre les25 tenues, leur face actuelle et les350 poses entières WALK12.
Le Technomancien est le seul groupe dont la silhouette haute médiane pendant
la marche dépasse celle de face sur les cinq tenues : environ+9% aux trois
premiers paliers, malgré une vue de trois quarts. Cette mesure inclut les bras
et n'est pas une mesure du volume physique ou de la qualité artistique.

La nouvelle étude conserve son identité mécanique, ses matières ivoire/acier/
anthracite, sa mâchoire bronze, son optique ambrée et les cinq niveaux de
protection, sans robe. Le châssis doit avoir un torse, une taille et des membres
plus construits, avec le même gabarit et la même finition dans toutes les vues.
Les nouvelles figures restent entières, les équipements portés séparés.

Les autres classes sont contrôlées dans les cinq planches d'audit. La largeur
apparente doit être distinguée de la position des bras et du recouvrement des
volumes en trois quarts ; les ratios seuls ne mesurent pas un volume physique.
Le contrôle des dessins révèle aussi un défaut du Bastion aux paliers3/4/5 :
ses avant-bras et cuisses perdent des plaques ou utilisent une silhouette de
tenue plus légère pendant la marche. Trois cycles complets sont proposés avec
le blindage et la tête exacts de leur face conservée. Les autres classes gardent
leurs dessins actuels après contrôle, sans élargissement artificiel des robes.

## Candidats présentés

- `TE-C13-01` à `TE-C13-05` : cinq nouvelles faces et60 figures de marche.
- `BA-C13-03` à `BA-C13-05` :36 figures de marche, faces actuelles conservées.

Les deux GIF `Technomancien-face-marche-avant-apres-native64.gif` et
`Bastion-blindages-face-marche-native64.gif` comparent l'actuel à gauche et le
candidat à droite. Chaque déplacement couvre une case en400ms, puis revient
de face. Les pauses de300ms servent seulement à lire cet arrêt dans l'aperçu.
Les planches PNG présentent aussi les vues de face, gauche et droite en64px
avec zoom x2. Ces comparaisons sont sans arme ; elles valident la proposition
de corps, sans prétendre valider de nouvelles prises ou une intégration moteur.

Les dessins sont produits par l'outil intégré `image_gen`. Les prompts exacts,
références et chemins de sortie sont conservés dans [les prompts du Technomancien](generation.json),
[ceux du Bastion](bastion-generation.json) et [la reprise du dernier pied](tech05-frame11-repair.json).
Le premier essai de marge globale n'a pas suffi ; seule la dernière figure
droite est remplacée, les onze autres restant celles du cycle initial. La
réduction du correctif complet à la résolution du cycle reste isotrope.
Les contrôles de
conditionnement et empreintes sont dans `candidate-conditioning.json`.

## Résultat du contrôle des autres personnages

Les hauteurs restent proches de leur référence dans les25 tenues : les petits
écarts de silhouette suivent la flexion et le pas, sans redimensionnement XY
par phase. Les vêtements de la Traqueuse, les robes de l'Arcaniste et les
plaques organiques de la Métamorphe sont comparés sur les cinq planches natives.
Aucune retouche n'est proposée pour ces trois classes ni les deux premières
tenues du Bastion dans ce lot. Cette revue ne constitue pas un nouveau contrôle
du bestiaire.

## Suite de la revue

Les huit candidats doivent être validés visuellement avant intégration, selon
`AGENTS.md`. Après cet accord, l'intégration doit réutiliser les vrais objets
équipés, leurs prises rigides et les casques, puis vérifier ces composites à
taille native. Les prévisualisations présentes sont sans équipement ajouté ;
elles ne prétendent pas avoir déjà validé ces nouveaux composites équipés.

Aucun dessin accepté, aucune ressource du jeu et aucune règle ne sont modifiés
par la préparation de cette revue. Les sources des candidats vont sous
`assets/source-art/player-consistency-13/`, les exports reproductibles sous
`artifacts/player-consistency-review-13/`. Les contrôles et décisions restent ici.
