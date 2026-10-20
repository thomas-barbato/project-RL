# Audit d’échelle du mobilier — MOB-SCALE-01

20 octobre 2026. Contrôle demandé après la refonte des acteurs. Les planches
montrent les ressources existantes à leur taille de jeu, avec un personnage
actuel debout et un corps humain au sol. Aucune carte, collision ou dimension
de mobilier n’a été modifiée pendant cet audit.

## Constat prioritaire : lits de la rue

Le signalement du joueur est confirmé. Les lits intégrés aux dessins des
logements sont nettement plus longs et plus larges que le lit de l’infirmerie
du camp, à grille identique de 64 px. Le logement nord `ROOF-U01-02A` présente
le plus grand écart. Son matelas utile est environ deux fois plus long que
le corps humain allongé utilisé comme repère ; la tête et la face verticale
du sommier accentuent encore sa hauteur à l’écran. Le lit du premier logement
`ROOF-U01-01A` est également surdimensionné, mais l’écart est moins prononcé.

Il ne faut pas déduire la longueur d’un objet au sol de la seule hauteur du
personnage debout. Ici, la comparaison avec le corps allongé et avec le mobilier
du camp confirme toutefois l’incohérence. Les dimensions sont données comme
constats de lecture, pas comme nouvelles dimensions acceptées.

Correction à préparer séparément : dessiner chaque lit comme un objet isolé,
avec un matelas proche du repère humain allongé et une marge pour le cadre.
Le lit du camp (dessin complet 34 × 72 px) est un point de comparaison utile.
Conserver la projection, les murs et le volume des logements. Ne pas réduire
la maison entière pour corriger son mobilier. Revoir ensuite le sol laissé
libre, les meubles adjacents et l’emprise bloquante avant activation.

## Camp

Dix modèles de mobilier ont été examinés. Le lit médical `DEC-CAMP-10`
(34 × 72 px), les bureaux de 82–90 px de large, les casiers de 30 × 56 px,
le chariot de 26 × 40 px et les coffres de 42 × 34 px sont cohérents entre eux.
Les six cuves de cryogénie partagent un dessin de 44 × 96 px ; le caisson ouvert
fait 48 × 96 px. Leur enveloppe technique explique la marge autour d’un humain.
Ils ne demandent pas la même réduction que les lits de la rue.

Le dessin d’un lit ou d’une cuve déborde visuellement de sa case d’ancrage.
Ce débord ne prouve pas à lui seul une collision erronée : les emprises logiques
sont conservées ici et devront être revues avec le remplacement des lits.

## Autres éléments de la rue

Les appareils de cuisine et réfrigérateurs ont une taille crédible près du
personnage actuel. Les longues étagères latérales se lisent comme plusieurs
modules alignés au mur, et non comme un meuble haut à redimensionner selon
sa longueur à l’écran. La table et ses deux sièges méritent une nouvelle
comparaison après remplacement du lit ; pas de réduction globale justifiée
par le seul contrôle présent. Les encadrements de porte restent lisibles et
compatibles avec la silhouette humaine. Aucun accès n’a été changé.

## Pièces de contrôle

- `furniture-inventory.json` : tailles des dix modèles du camp, références
  humaines et trois intérieurs de rue examinés.
- Aperçu courant : `artifacts/art-quality-audit-01/furniture-audit/`.
- Priorité de la prochaine correction de décor : les deux lits des logements.

Cet audit termine la vérification demandée. Les remplacements de mobilier
restent une étape distincte, comme prévu dans le plan de campagne.
