# CITY-NORTH01 — B01 / 10-1 r04, référence futuriste grimdark pré-validée

Le tracé PLAN09 est pré-validé le 5 novembre 2026. La nouvelle demande porte
sur une identité cyberpunk, futuriste, sale et abîmée pour les décors.
Les égouts sont exclus de la reprise générale. Voir `CYBERPUNK11.md`.
Le joueur pré-valide ensuite ce dessin : « oui c'est bien comme ca :) ».
Portée et empreintes : `PRODUCTION10-B01-ACCEPTANCE-r04.json`.

## Implantation et échelle

B01 reste l'immeuble éventré, rectangle `[12,91,20,96]`, soit 9 × 6 cases.
Son approche est `(13,97)`, au sud près de l'extrémité gauche de la façade.
Rues, emprises, autres bâtiments, espaces verts et identifiants du plan
restent identiques. Les données de la carte antérieure et des six variantes
de sous-sols de police restent conservées.

Le bâtiment est réduit uniformément à 576 px de large ; sa hauteur visible
est de 474,65 px. Le pied de façade reste à `Y=6208 px`. Le recadrage retire
seulement les marges transparentes. Aucune déformation locale n'est appliquée.
Le personnage conserve son fichier original 64 × 64, visible sur 58 px.
Les quatre baies et la porte restent aux mêmes emplacements dans la façade.

La chaussée et la bordure suivent le segment R02 du plan. Sols, petit poteau,
caisse, sang et corps civil réutilisent les ressources de la première ville,
aux positions de la précédente revue. Ce sont des placements de candidat.

## Révisions conservées

- **r01** : image signalée par le joueur pour sa perspective incorrecte.
- **r02** : essai de traitement des matières conservant les défauts de projection ; non accepté.
- **r03** : étude de correction des axes du toit et des faces supérieures de cheminée et d'évent ; non acceptée, avant élargissement de la demande.
- **r04** : pré-validé par le joueur, avec équipements réseau/refroidissement, câblage technique raccordé, terminal, caméra, écran et volets de sécurité abîmés. Le traitement de la façade est repris dans le cadre de la nouvelle demande futuriste.

Les sources brutes restent intactes dans
`assets/source-art/north-city-01/production10/`. Outil : **image_gen.imagegen
intégré**, édition avec références. Les prompts exacts r01 à r04, les
empreintes, le cadrage et l'échelle figurent dans le manifeste de ce dossier.
Le garage et la scène validés de la première ville guident le dessin et
la caméra ; leur aménagement n'est pas recopié.

## Revue et limites

`Apercu_batiment_nord.cmd` lance la revue native isolée. La page courante
montre le candidat, le comparatif avec le garage accepté et un avant/après
avec l'image r01 signalée par le joueur. Chaque scène reste à 64 px par case,
avec le même acteur. Le repère d'emprise est disponible séparément.

`examples/north_city_building_review.rs` vérifie emprise et approche contre
PLAN09. Lectures, décodages et préparation du sol passent par le worker
commun avant affichage ; les textures sont créées sur le thread graphique.
Les plafonds du cache commun restent ceux du projet.

**Dessin fermé pré-validé le 5 novembre 2026.** L'intérieur, les occultations et les collisions
restent à réaliser et à tester en jeu. La campagne normale et les égouts
ne sont pas modifiés. La reprise des autres familles de décors reste à faire.
