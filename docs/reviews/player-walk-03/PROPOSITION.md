# WALK03-r01 — catalogue de déplacement avant validation

Statut : proposition rejetée pour ses déformations de face, remplacée par les
dessins de trois quarts de WALK04. Aucune animation de ce catalogue n’est intégrée
au rendu actif du jeu. Un accord sur la couverture, le temps ou les directions
ne constitue pas un accord sur les nouveaux profils ou sur chaque morphologie.

## Choix demandés

- Une commande correspond toujours à une case sur la grille de 64 × 64.
- 400 ms par défaut, avec choix de 120 / 200 / 400 ms dans la revue.
- Orientation gauche / droite lorsque la direction possède une composante
  horizontale ; face pour le haut et le bas.
- Présenter les personnages, les armures et les monstres avant intégration.
- Conserver les silhouettes, les couleurs, les équipements et les règles de jeu.

## Couverture de ce premier passage

Le catalogue contient 659 entrées numérotées : les 25 tenues actuelles des cinq
classes ; les 129 modèles de campagne et leurs 471 variantes ; les quatre PNJ du
camp, les quatre drones et les six transformations ; 149 entrées historiques
(65 modèles de bestiaire, 27 anciens acteurs et 19 anciennes tenues avec leurs
trois palettes figées). Ces entrées historiques comprennent des remplacements
ou des ressources qui ne sont pas des rencontres actives.

Les classes utilisent les couches corps / arme / doigts du compositeur actuel,
avec ses 113 modèles d’armes, trois associations à deux armes et six casques.
Les casques peuvent être combinés avec les armes dans l’aperçu ; leur masque de
tête vient de la ressource actuelle. Les configurations historiques restent
leurs 16 anciennes compositions de revue, sans les présenter comme de nouvelles
compositions du moteur actuel.

`coverage.json` fournit les ID, les sources, les empreintes des pixels exportés
et les dimensions. Les remplacements de lisibilité et les finitions du rendu
actuel priment sur les anciens fichiers des manifestes. Les numéros retirés
restent retirés et les boss de campagne différés 18 / 19 ne sont pas inventés.
Les corps sans mouvement (cadavres, objets et décors) ne sont pas animés.

## Nature du candidat

Ce passage est un **essai technique d’animation 2D des sources actuelles**.
L’orientation latérale est une déformation des pixels existants ; elle ne
reconstitue pas les surfaces cachées et n’équivaut pas aux trois vues peintes
du Bastion de WALK02. Les profils peints restent une étape artistique à revoir.
Ce catalogue ne doit donc pas être accepté ou intégré comme un jeu complet de
sprites de profil déjà dessiné.

Dix profils de mouvement séparent les bipèdes, les armures lourdes, les robes,
les quadrupèdes, les pattes multiples, les serpents / vers, les flottants, les
ailes, les châssis sur pattes et les véhicules à roues / chenilles. Leur choix
est fondé sur les planches de sources ; le sélecteur permet d’évaluer une autre
solution sans modifier le jeu. Il ne définit aucun pouvoir ou déplacement de
gameplay nouveau. Le Gardien reste bipède ; l’Artilleur reste sur ses chenilles.

Pour les classes, l’orientation gauche échange ensemble les parties portées,
les manches et les doigts autour du noyau du corps. L’armure centrale conserve
sa couleur et son orientation. Le raccord est encore un candidat à examiner,
particulièrement aux coudes et aux limites des manches. Pour les animaux à
silhouette de profil, le corps peut être retourné ensemble ; les humanoïdes
monoblocs conservent leur source et une orientation latérale limitée.

La banque historique garde son échelle actuelle d’affichage sur des cases de
64 px. Ses anciennes sources ne sont pas présentées comme de nouveaux dessins
en 32 px. Aucun modèle de grande taille n’est réduit pour entrer dans une case.

## Revue visuelle proposée

Le site compare le dessin fixe actuel et l’essai de mouvement à la même
échelle. Le zoom entier examine les pixels. Les sols clair, sombre et le montage
sur le vrai sol du camp permettent de juger la lisibilité. Ce dernier ajoute
plusieurs acteurs et un objet au sol ; il s’agit d’un montage de revue, pas
d’une capture d’un déplacement intégré dans le jeu.

Les flèches font avancer d’une case. Une nouvelle flèche remplace la présentation
en cours ; aucune file de tours n’est simulée. La démonstration automatique,
la pause et l’avance image par image servent uniquement à la revue.

La page d’origine `index.html` et le prototype peint `armes.html` restent
inchangés. Le nouveau catalogue utilise un iframe avec `sandbox="allow-scripts"`
et une CSP restrictive. Les images sont limitées aux ressources recensées ;
les noms issus des manifestes sont injectés comme texte, jamais exécutés.

## Reproduction et contrôles

1. Définir `RL_MOVEMENT_REVIEW_OUTPUT` vers
   `artifacts/player-walk-review-03/compositor`, puis exécuter
   `cargo test --locked movement_catalogue_source_diagnostic -- --ignored --nocapture`.
   Ce diagnostic ignoré par défaut exporte les couches sans créer de texture GPU
   ni ajouter un chemin de rendu au jeu.
2. Exécuter `python tools/art/build_player_walk_review_03.py --output <dist-du-site>`.
3. Exécuter `node tools/art/verify_player_walk_review_03.js` et les vérifications
   de syntaxe JavaScript. L’export vérifie les cellules non vides, les dimensions,
   les ID uniques et la couverture des variantes.
4. Vérifier dans le navigateur les armes et casques, la robe, les directions
   verticales, les différentes morphologies et les trois durées.

Ces contrôles ne valent ni validation artistique ni test de gameplay. Une
validation visuelle explicite reste nécessaire avant toute intégration. La
présentation finale doit rappeler la limite des profils obtenus par déformation.
