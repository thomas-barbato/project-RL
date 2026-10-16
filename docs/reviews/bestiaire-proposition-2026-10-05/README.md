# Bestiaire — proposition du 5 octobre 2026

Cette galerie est une proposition visuelle soumise au joueur avant intégration.
Les noms des nouveaux ennemis sont provisoires. Leurs statistiques, comportements,
récompenses et apparitions restent à définir après la sélection des dessins.

## Numérotation

- **01–28** : nouveaux ennemis, tiles natives de 32 × 32 pixels.
- **29–32** : quatre propositions de boss, tiles de 64 × 64 pixels, comparées au personnage de 32 pixels.
- **33–55** : les 23 références vivantes actuellement utilisées, copiées sans modification, avec un nouveau cadavre pour chacune.

Chaque référence présente le vivant et le corps couché dans sa flaque de sang.
Pour les machines, le liquide rouge représente une fuite de fluide.
Le numéro est stable et permet de demander une correction du vivant, du cadavre ou des deux.

## Examiner les dessins

Ouvrir `galerie-bestiaire.html`, ou la copie servie sur
<http://127.0.0.1:8769/galerie-bestiaire.html> tant que le serveur local est actif.
La galerie propose une vue native, des zooms entiers, une recherche, des familles
et plusieurs fonds de contrôle. Une vue native reste présente sous chaque paire agrandie.

Planches PNG directement consultables :

| Numéros | Famille | Planche |
| --- | --- | --- |
| 01–04 | Monstrueux | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/01-04.png) |
| 05–08 | Lovecraftien | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/05-08.png) |
| 09–12 | Humains et possédés | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/09-12.png) |
| 13–16 | Humains robotisés | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/13-16.png) |
| 17–20 | Démons et mages | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/17-20.png) |
| 21–24 | Robots et androïdes | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/21-24.png) |
| 25–28 | Robots-démons | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/25-28.png) |
| 29–32 | Boss | [Voir](C:/Users/User/Desktop/project-RL/assets/source-art/bestiaire-proposition-2026-10-05/review/29-32.png) |
| 33–55 | Bestiaire existant et cadavres | [Voir](C:/Users/User/Desktop/project-RL/artifacts/bestiaire-proposition-2026-10-05/review/33-55.png) |

Les scènes dans `review/scene-*.png` sont des compositions de revue avec le sol,
le personnage et les objets du jeu. Ce sont des compositions de contrôle,
pas des captures du moteur après intégration. Les objets sont composés par-dessus
les corps pour illustrer l'ordre de dessin demandé.

## Sources et fabrication

- `sources/` conserve les sorties complètes de l'outil imagegen, ainsi que les révisions.
- `references/` contient les références vivantes existantes agrandies sans lissage.
- `prompts.json` conserve les instructions de génération et de correction.
- `tiles/` contient les PNG natifs transparents de revue.
- `catalog.json` décrit les candidats ; `manifest.json` décrit les fichiers effectivement livrés.
- `validation.json` et `audit.json` consignent les vérifications.

Les dessins proviennent d'imagegen. `package.py` découpe les sujets complets,
élimine les petits fragments de découpe, place le sujet dans sa tile,
redimensionne au plus proche et limite la palette à 24 couleurs sans dithering.
Le script ne dessine ni anatomie ni équipement. Les planches et scènes sont
assemblées dans `review/`, hors des ressources actives.

Pour reconstruire les fichiers de revue : exécuter `package.py`, puis
`build_gallery.py`, puis `audit.py`, avec Python et Pillow. `prepare.py` est
le script d'initialisation et ne doit pas être relancé sur une proposition déjà préparée.

## Étape d'intégration après avis

Les flaques sont incluses dans les composites de cadavres présentés ici.
Leur séparation en couche réutilisable, les points d'attache d'équipements et
les règles des boss restent à traiter pendant l'intégration des candidats retenus.
La taille dessinée de 64 pixels ne fixe pas encore la collision d'un boss.

## Vérifications effectuées

Les 110 fichiers natifs ont été contrôlés : dimensions, présence d'un sujet,
alpha binaire, palette de 24 couleurs au maximum et marges des nouveaux dessins.
Les 23 vivants existants sont identiques, pixel par pixel, aux cellules de l'atlas actif.
Les empreintes des 765 fichiers actifs examinés sont restées identiques.
Les planches et les compositions ont été inspectées visuellement.
Le contrôle de la galerie dans le navigateur intégré a été bloqué par la politique
de sécurité du navigateur après un échec de chargement. Le serveur local a été
relancé et sa présence sur le port 8769 vérifiée. Les contrôles des commandes
interactives de la galerie restent donc non confirmés dans ce navigateur.

Le comportement final demandé reste : cadavres persistants au sol, avec les
éventuels loots visibles au-dessus. Cette proposition n'ajoute pas les nouveaux
ennemis aux rencontres et ne modifie aucune règle du jeu.
