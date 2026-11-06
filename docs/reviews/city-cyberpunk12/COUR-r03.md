# Cour centrale · retrait de la machine r03

Le joueur trouve l’emplacement de la machine au milieu de la ville peu
crédible. La variante r03 dégage cet espace. Le béton d’origine est recomposé
à partir des couches de la carte acceptée, puis rétabli sous la seule empreinte
du dessin retiré. La comparaison se fait avec le personnage à 64 px par case.

La retouche prévalidée le 6 novembre 2026 est intégrée dans les deux
versions de la rue. Elle libère `(11,39)`, `(11,40)`, `(12,39)` et `(12,40)`.
Les cases de la benne sont indépendantes. La libération est enregistrée à
l’arrivée dans la rue ou après vérification d’une sauvegarde antérieure,
sans consommer de tour. [Accord et empreintes](ACCORD-COUR-r03.json)

La couche de retrait et les cases concernées sont enregistrées dans le
[manifeste](../../../assets/source-art/city-cyberpunk12/cour-r03/manifest.json).

Les tests vérifient la première entrée, la reprise d’une partie où la
machine bloquait encore la cour, et une nouvelle sauvegarde après libération.
La vérification couvre l’instantané et le rejeu des commandes. La benne reste
bloquante dans le scénario de contrôle.
