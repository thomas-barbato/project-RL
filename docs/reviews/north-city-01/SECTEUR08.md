# CITY-NORTH01 — témoin de finition 08, candidat r03

Précision ultérieure du joueur : la nouvelle carte doit être une grande ville
originale. Le secteur 08 sert de témoin local de finition et d'échelle ;
son petit aménagement n'est pas le modèle à répéter. Le plan complet et
distinct est décrit dans [URBANISME09.md](URBANISME09.md).

Le joueur refuse MAP07 : qualité insuffisante face à la première ville,
perspectives, décorations et décors eux-mêmes. Cette reprise du 5 novembre
2026 porte sur une composition de quartier complète. Elle remplace MAP07
comme proposition graphique courante, sans présumer de validation visuelle.

## Reprise proposée

Quatre bâtiments distincts encadrent une rue en décroché et une petite cour :
habitation endommagée, logement ouvert, épicerie et atelier. Les couvertures
sont compactes, au-dessus de façades courtes, avec des angles de maçonnerie et
des raccords de bordures. Les fenêtres, accès, conduites et objets de rue
participent au lieu ; ils ne sont plus répartis comme des timbres répétitifs.
Les trottoirs restent continus, sans quadrillage décoratif. Le civil, le
militaire et la petite créature morts appartiennent à ce candidat graphique.

La scène validée `SCENE-U01-07-r01.png` fixe le dessin et la projection. Le
comparatif est rendu avec le vrai `assets/maps/prison-street-expanded-01/street.png`
à son échelle native, et le même acteur 64 × 64 dans les deux panneaux. La
source 08 de 1536 × 1024 est conditionnée uniformément à 75 %, soit 1152 × 768,
pour conserver des ouvertures et objets à l'échelle du personnage. L'acteur
garde son fichier, ses pixels et sa taille : silhouette visible de 58 px.
Aucune bande n'est déformée pour fabriquer un bâtiment ou une perspective.

Le premier intérieur avait du mobilier trop petit. La correction r03 reprend
la référence `house-a-body-native.png` : lit entier, appareils de cuisine,
table, rangements et coin d'eau. Elle est appliquée uniquement à **S08-B02**,
par une couche recadrée sur son rectangle source `[992,24,432,440]`. Le rendu
RGBA est identique au candidat r02 partout hors de ce rectangle. Les autres
bâtiments, rues, objets et corps sont ainsi conservés exactement.

## Sources et retouches

Outil utilisé : **image_gen.imagegen intégré**, éditions avec références.
Les fichiers bruts sont conservés sans altération dans :

- `assets/source-art/north-city-01/sector08/SECTEUR08-01-source.png` : première composition ;
- `assets/source-art/north-city-01/sector08/SECTEUR08-01-r02-source.png` : sols extérieurs continus ;
- `assets/source-art/north-city-01/sector08/SECTEUR08-B02-interieur-r03-source.png` : source du correctif B02 uniquement.

Les briefs exacts sont `SECTEUR08-PROMPT-r01.txt`, `SECTEUR08-PROMPT-r02.txt`
et `SECTEUR08-INTERIEUR-PROMPT-r03.txt`. Le manifeste voisin des images porte
leurs empreintes et références. `scene.json` conserve les rectangles de revue
et identifiants S08-B01 à B04, C01 et D01 à D03. Ces rectangles servent aux
retouches graphiques ; ils ne prétendent pas être des collisions validées.
Une correction locale peut ajouter une couche ciblée, sans régénérer ni
remplacer les autres éléments. La source de base reste disponible.

## Périmètre et inspection

**Cette étape est une proposition visuelle avant reprise des emprises et
collisions du secteur dans la carte jouable.** Elle ne modifie aucun des dix
JSON du quartier et du commissariat, aucune règle, sauvegarde, variante ou
zone de campagne. Le plan 112 × 112 et les accès `(18,1)` / `(18,2)` restent
les données de travail. La composition 08 n'est pas encore positionnée sur ce
plan et n'est pas une preuve de navigation, de portes ou d'occultation.

La pré-validation du joueur sur la qualité graphique reste en attente, puis
la validation définitive doit se faire en jeu, comme demandé. Les essais
jouables précédents conservent leurs données ; leur dessin 07 est refusé et
ne devient pas une référence acceptée.

La page courante `artifacts/north-city-01/apercu.html` montre la vue d'ensemble,
le comparatif natif et le personnage dans le logement, avec des repères
facultatifs de retouche. `Apercu_visuel_quartier_nord.cmd` ouvre le même
candidat dans une fenêtre native. I choisit le repère rue / intérieur ;
le personnage sert à examiner l'échelle.

Les captures sont produites par `examples/north_city_sector_review.rs`.
Les PNG sont lus et décodés sur le worker commun, puis les textures préparées
avant l'affichage sur le thread graphique. Les limites communes 64 Mio de
sources et 128 Mio de textures restent appliquées. Aucun accès lourd à une
image ne se trouve dans le dessin. Les captures ne sont pas une dépendance
du programme. Le bilan `VALIDATION-SECTEUR08.json` distingue ces contrôles
techniques de la validation graphique et du futur test de déplacement.
