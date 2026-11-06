# CYBERPUNK11 — clarification et reprise des décors

## Demande du joueur — 5 novembre 2026

Le joueur juge les décors sales et abîmés, mais insuffisamment cyberpunk.
Il rappelle la direction grimdark et précise qu'elle doit être futuriste,
sale et endommagée. Il demande ensuite de revoir **tous les décors**, en
excluant explicitement les égouts, jugés satisfaisants en l'état.

Cette précision est consignée dans `docs/DIRECTION_ARTISTIQUE_TILES.md`,
section « Orientation globale […] — grimdark et glauque ». Elle ne vaut
pas acceptation des nouveaux pixels ou autorisation d'intégration d'un essai.

## Direction de travail

Rendre la technologie lisible dans les formes et les usages : systèmes
d'accès et de surveillance, réseaux, équipements techniques et infrastructures
transformant réellement l'habitat. Leur abandon s'exprime par des pannes,
protections cassées, réparations de fortune, connexions arrachées et traces
de violence. Les lumières fonctionnelles restent ponctuelles. Les surfaces
mates, contours, projection et proportions suivent les références acceptées.

Les bâtiments gardent des usages, silhouettes et degrés de modernisation
variés. Conserver les maçonneries européennes lorsque cela convient au lieu.
Les détails ne doivent pas encombrer tous les toits ni masquer les passages.
La végétation garde sa fonction et ses variétés ; sa revue ne lui impose pas
d'accessoires électroniques. La saleté, les ruines, le gore et la verdure
demandés pour la ville restent dans le périmètre.

## Première référence pré-validée — B01 / 10-1 r04

Le même immeuble éventré reçoit une couche technique : relais réseau,
refroidissement, chemin de câbles endommagé et raccordé à la façade, terminal
d'accès, caméra, affichage de service et volets métalliques. Le toit reprend
les axes parallèles et les faces supérieures préparés par l'étude r03.

La géométrie PLAN09, B01 `[12,91,20,96]`, l'approche `(13,97)`, les quatre
baies, le seuil à gauche, le corps, la caisse, le poteau et le personnage
de revue conservent leurs emplacements. L'édition de l'aspect de la façade
fait désormais partie de la demande élargie, contrairement à la correction
de perspective seule initialement envisagée. Les sources r01, r02 et r03
restent intactes. Aucun dessin d'un autre bâtiment n'est régénéré.

Source et prompt : `assets/source-art/north-city-01/production10/manifest.json`.
Statut : **dessin fermé pré-validé le 5 novembre 2026**, sans intégration à la campagne.
Accord : « oui c'est bien comme ca :) ». Portée et empreintes figées dans
`PRODUCTION10-B01-ACCEPTANCE-r04.json`. B01 r04 devient une référence de direction
pour les prochains décors.
Le bâtiment fermé ne constitue pas une validation de son intérieur ou de
ses collisions. La validation définitive reste à faire en jeu ; les futurs
dessins conservent leur propre revue avant intégration.

## Reprise générale — étapes restant à réaliser

**Ordre précisé ensuite par le joueur : première map de ville, puis nouvelle
ville, en conservant leurs tracés.** La reprise commence dans
`docs/reviews/city-cyberpunk12/REVUE.md`. B01 r04 reste la référence acceptée ;
les autres familles seront revues après les deux villes. Les égouts restent exclus.

| Repère | Famille | État |
| --- | --- | --- |
| CP11-B01 | Immeuble témoin du quartier nord | Dessin fermé r04 pré-validé ; inspection en jeu à faire |
| CP11-NORD | Autres bâtiments, commissariat, école, pompiers, caserne et maisons | À produire selon la direction B01 r04 ; tracé pré-validé conservé |
| CP11-VILLE | Bâtiments et décors des cartes de ville existantes | À revoir par éléments identifiés ; ressources actuelles conservées |
| CP11-CAMP | Architecture et équipements du camp | À revoir dans le même cadre |
| CP11-INT | Intérieurs, murs, coins, sols et mobilier hors égouts | À revoir avec raccords et circulation |
| CP11-RUE | Voirie, accès, enseignes, mobilier et autres décors extérieurs | À revoir selon leur fonction, avec surfaces et passages lisibles |
| CP11-EGOUTS | Égouts, canaux, équipements et décors propres aux égouts | **Exclus de la reprise, conservés en l'état** |

Ce tableau suit la demande ; il ne prétend pas que les familles futures
soient déjà dessinées ou validées. Garder `EDIT-SCOPE01` à chaque retouche :
un changement demandé à un objet ne régénère pas son voisinage.
