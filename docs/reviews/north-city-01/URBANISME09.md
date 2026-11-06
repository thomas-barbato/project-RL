# CITY-NORTH01 — plan de ville 09

Le joueur précise que la première carte fixe **la qualité du dessin**, pas le
contenu ni l'aménagement de la nouvelle ville. Il demande une grande carte
urbaine originale, avec bâtiments, maisons, verdure et les équipements déjà
listés. Le plan 09 traduit cette clarification ; le témoin graphique 08 sert
uniquement à examiner la finition et l'échelle.

## Tracé pré-validé — 5 novembre 2026

Le joueur répond : « le schéma du tracé de la ville me plait bien ». Le plan
09 est retenu pour la suite. Ses rues, emprises, espaces verts, accès et
identifiants sont conservés. Cet accord porte sur le schéma ; les dessins
natifs et la validation définitive en jeu restent ouverts. Empreinte du
plan et périmètre : `PLAN09-ACCEPTANCE-r01.json`.

## Grande ville et quartiers

La proposition couvre **112 × 112 cases**, soit **7168 × 7168 pixels** à
l'échelle native de 64 px/case. Elle comporte 100 bâtiments identifiés,
22 intérieurs envisagés, six emprises végétales et 103 arbres proposés.
Ce sont des données d'aménagement, pas 100 dessins déjà terminés.

Les 34 dernières parcelles complètent des façades mitoyennes en préservant
les 66 premières. Les quartiers associent maisons étroites, petits immeubles,
commerces et ateliers, avec des cours et des passages. La hiérarchie des rues
distingue l'avenue principale, des rues locales et les cheminements piétons.
Les alignements se décalent ; le réseau ne reprend ni la première scène de
ville, ni une répétition de son carrefour et de ses quatre bâtiments.

- Faubourg résidentiel au sud : maisons, immeubles et jardins de fond de cour.
- Quartier ancien à l'ouest : îlots serrés, passages et immeuble éventré.
- Secteur central : commerces, place de marché et jardin public.
- École : plusieurs ailes, cour et arbres ; aucun toit unique ne couvre toute la cour.
- Caserne de pompiers : corps de bâtiment, garages, cour de manœuvre et voie de sortie.
- Commissariat : ailes, cour de service, pavillon d'entrée et étage.
- Est : ateliers, livraisons, friches, îlots incendiés ou effondrés.
- Nord-est : poste de contrôle et accès à la caserne militaire, point de sortie.

Les scènes de gore ont des repères distincts : secours et évacuation interrompus,
barricade militaire, victimes civiles et petites créatures mortes. Les sept
repères du plan désignent des groupes de décors à dessiner, pas une limite de
sept corps pour toute la ville.

## Références d'aménagement

Sources consultées : [plan-guide de Strasbourg](https://www.visitstrasbourg.fr/wp-content/uploads/2020/09/OTSR_Plan-guide_2021.pdf)
et [plans et parcellaire de Paris](https://www.paris.fr/pages/plans-donnees-et-documents-d-urbanisme-3708).
Ils servent à réfléchir à la hiérarchie des voies, aux îlots, aux cours et aux
espaces publics. Le tracé du plan 09 est fictif et original ; aucun quartier
réel n'est recopié. Les noms de travail ne deviennent pas des lieux réels.

## Raccords et sous-sols

Les deux raccords viennent du nord de la première carte, `(18,1)` et `(18,2)`.
L'arrivée proposée dans la nouvelle ville reste `(41,108)` / `(41,109)`.
La sortie vers `core:military_barracks` reste `(99,4)`, au nord-est. Plusieurs
voies relient les quartiers ; le commissariat est un détour facultatif.

Le commissariat conserve ses identifiants et ses deux ancrages d'escalier
`(31,23)` / `(34,23)`. L'étage et les deux séries de trois variantes inférieures
restent les fichiers existants. Le tirage à la création de partie et sa
persistance ne changent pas. Les pièces et raccords visuels du commissariat
devront être adaptés au plan retenu avant d'en proposer l'intégration.

## Fichiers éditables et portée

`assets/maps/north-city-01/city-plan09.json` contient chaque bâtiment, emprise,
porte proposée, passage, rue, espace vert et repère de décor séparément.
B01–B10 gardent les repères des lieux principaux ; H01–H90 et les autres
familles ont leurs identifiants durables. Une retouche porte sur les objets
demandés, pas sur un nouveau tirage de toute la ville.

`prepare_north_city_plan09.py` est un amorçage qui refuse d'écraser un plan
existant. `infill_north_city_plan09.py` refuse de répéter son ajout de façades.
`render_north_city_plan09.py` relit le JSON sans le modifier et produit le SVG
interactif et le PNG de revue. Le SVG est un **schéma d'aménagement**, distinct
des futurs dessins natifs. Les bâtiments peuvent être sélectionnés dans
l'aperçu courant pour désigner précisément une retouche.

Le plan 09 est une proposition séparée. Les dix JSON existants du quartier,
des sous-sols et de leur revue restent identiques, ainsi que les règles,
sauvegardes, textures validées et fichiers de la première ville. Aucun
raccord normal ni changement de campagne n'est effectué à cette étape.

Le contrôle de circulation utilise une grille de planification : emprises
et troncs bloqués, rues et accès réservés. Il confirme la liaison des deux
arrivées, de toutes les entrées proposées et de la sortie. Ce contrôle ne
valide pas le futur rendu des murs, les portes, les intérieurs ou les
collisions du jeu. Voir `VALIDATION-PLAN09.json`.

Pré-validation de l’aménagement : **acceptée**. Pré-validation des dessins
et validation définitive en jeu : **en attente**, distinctes de cet accord.
