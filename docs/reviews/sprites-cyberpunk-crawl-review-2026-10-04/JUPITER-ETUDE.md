# Direction des icônes au sol — 4 octobre 2026

Références observées : les captures publiées par ChaosForge sur les pages
officielles de [Jupiter Hell Classic](https://store.steampowered.com/app/3126530/Jupiter_Hell_Classic/)
et [Jupiter Hell](https://store.steampowered.com/app/811320/Jupiter_Hell/).

Dans Classic, les objets au sol gardent des silhouettes figuratives compactes,
des contours sombres et quelques zones de couleur/shading. L'armure ressemble
à une protection rigide ; les armes ont des profils allongés. Dans la capture
Callisto Rift L3 de Jupiter Hell, plusieurs armes au sol apparaissent comme
des silhouettes très lumineuses, jaunes, sur le décor sombre.

Ces observations orientent la proposition : profils anguleux, gris métalliques,
ambre pour l'arme à distance, kit médical rouge, cartouches en laiton,
grenade militaire et modules avec de grandes zones cyan. Les lumières occupent
quelques pixels entiers ; il n'y a pas de texture fine à deviner sur le plateau.

La proposition contient 16 icônes de catégorie couvrant les 150 objets.
Les 61 armes à distance partagent le fusil ; les 40 armes de mêlée partagent
la lame ; les 32 pièces d'armure partagent la même icône de protection. Les cinq
charges explosives partagent la grenade. Les autres ressources ont leur
pictogramme distinct. Le modèle précis reste dans l'inventaire et la cible.

Les icônes sont originales, dessinées dans ground-icons-jupiter-v1.js,
directement sur la grille finale de 32 pixels. Elles ne sont pas des assets
extraits de ces jeux. build-jupiter-icons.cjs exporte les PNG et les SVG.
render-jupiter-icons-review.py rend la planche à partir des grilles sources.

L'aperçu actif proposé est apercu-icones-jupiter.html. Les propositions
précédentes sont conservées pour comparaison. L'intégration dans le moteur
attend le retour visuel de l'utilisateur sur cette direction.
