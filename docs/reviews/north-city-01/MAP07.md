# CITY-NORTH01 — MAP07-r01 et CONTOURS07-r02

Reprise du 5 novembre 2026, après la demande de garder les bordures noires
de la première ville et de poursuivre la nouvelle carte. Candidat séparé de
la campagne normale ; pré-validation visuelle et validation du joueur en jeu
toujours en attente.

## Dessin témoin 07-1 r02

Le garage natif de la première ville fixe le poids du trait : silhouettes
noires franches, traits structurels plus fins, ombres de matière conservées.
La première tentative 07 donnait des contours trop fins à 288 px de large ;
elle n'est pas retenue. Le candidat r02 reprend seulement le traitement du
trait de la maison témoin, sans demander une nouvelle perspective ou un autre
agencement. Les proportions doivent encore être jugées par le joueur.

L'image source complète est conservée sans retouche algorithmique sous
`assets/source-art/north-city-01/style07/VOLUME07-01-r02-source.png`.
La comparaison utilise un cadrage de ses marges transparentes, une réduction
uniforme à 288 px de large et une hauteur native arrondie à 282 px. Le
personnage reste le même fichier 64 × 64. Le garage, le sol, la rue et la
position des acteurs sont conservés. Outil intégré `image_gen.imagegen` ;
brief exact : `CONTOURS07-r02-PROMPT.txt`, empreintes dans le manifeste voisin
de la source. Aucun filtre global ou épaississement automatique des pixels.

## Poursuite de la carte parcourable

Les neuf masses mitoyennes F01–F09 et les bâtiments fermés reprennent leurs
emprises réelles. Les volumes courants sont élevés de 96 px ; les façades rejoignent
le sol, sous leur propre toiture. Les contours sont composés aux angles et
aux décrochements. Les intérieurs visitables s'ouvrent à l'entrée et restent
consultables en coupe avec R. Les personnages et les décors au sol sont
occultés par les couvertures placées devant eux.

Les sols extérieurs, pierres de couronnement, briques et équipements de toit
réutilisent les ressources de la première ville. Les matières de couverture
sont séparées des appareils entiers : aucun morceau de corniche ou d'appareil
ne doit se répéter dans un aplat de toit. Les fenêtres reprennent le candidat
07. Les murs intérieurs conservent leurs profils communs pour les coins,
jonctions et portes, avec un couronnement repris de la référence native.

B01 reçoit ensuite un dessin complet propre : **ILOT07-B01-r02**, avec quatre
sections de zinc/ardoise, des conduites, deux cheminées, un ventilateur, une
rupture localisée de couverture et des fenêtres différenciées. Son image est
réduite uniformément à 1280 × 842 px, depuis ses seules marges transparentes.
Le toit couvre les 20 × 11 cases existantes ; sa façade et son couronnement
occupent 138 px supplémentaires, avec le même pied de bâtiment qu'avant.
L'occultation suit cette hauteur propre à B01. Le reste du quartier conserve
ses volumes de préparation. La première tentative d'îlot r01 était trop peu
profonde et n'est pas affichée dans le prototype. Les sources sont conservées
sans déformation par bandes ; leurs briefs et empreintes sont enregistrés.

Les dix fichiers JSON gardent exactement leurs empreintes antérieures à cette
reprise. Aucun bâtiment, objet, corps, porte, pièce ou passage n'est déplacé.
Le modèle lit désormais les emprises F01–F09 déjà présentes dans les données,
pour la présentation uniquement. Les trois variantes de chacun des deux
sous-sols restent choisies à la création de partie, puis sauvegardées.
Preuve : `MAP07-DONNEES-CONSERVEES.json`.

## Inspection et vérifications

`Apercu_quartier_nord.cmd` lance l'exploration isolée ;
`Apercu_logement_nord.cmd` commence devant B01. La page courante
`artifacts/north-city-01/apercu.html` présente le quartier repris, le comparatif
de contours natif, le logement parcouru, dix lieux et six plans de sous-sols.
`map07-ilot.png` montre B01 fermé à 64 px/case ; `decor04-logement.png` montre
ses pièces ouvertes. Les noms historiques des deux médias DECOR04 sont
réutilisés pour conserver un seul aperçu courant.

36 tests de l'exemple passent, dont les 15 vérifications propres au quartier,
aux raccords et aux volumes. Un contrôle du dessin entier vérifie ses dimensions,
son emprise et son pied de façade. Le parcours B01 emploie les commandes,
portes et collisions du moteur ; son animation est vérifiée sans perte sur
les cinquante images natives. Les tests ne constituent pas l'inspection
manuelle du joueur. Les preuves courantes figurent dans `VALIDATION-MAP07.json`.

La préparation et le décodage restent sur le worker commun ; les textures
sont créées sur le thread graphique avant l'affichage. Les caches communs
restent bornés à 64 Mio de sources décodées et 128 Mio de textures, plus une
palette dérivée fixe de 12 Mio maximum. Les sauvegardes ne dépendent pas de
ces caches. Les ressources du prototype sont dans `assets/`, sans dépendance
d'exécution aux captures sous `artifacts/`.

Il reste à affiner les façades par fonction, les rencontres et le butin ;
les petits cadavres de monstres ne sont pas encore dessinés. La sortie de
caserne est le repère futur, sans nouvelle zone militaire raccordée.
L'éclairage de revue et l'animation du personnage ne constituent pas leur
présentation finale. Aucun accord visuel ou intégration normale n'est présumé.
