# Audit des animations réellement utilisées en jeu — 24 octobre 2026

L'intégration est incomplète : la présence des dessins validés ne signifie pas
que toutes les configurations jouables les utilisent. Le joueur à la hachette
et les variantes colorées de monstres reviennent au rendu statique existant.

## Scène observée et limites

Observation de l'exécutable Release courant, dans une copie isolée du profil
de la dernière partie. Le joueur place la scène devant trois hostiles, aux
Abords de la prison, profondeur1, X18/Y43. Le panneau affiche « Hachette » et
la classe de la sauvegarde est Bastion. Les réglages d'origine indiquent
`reduced_motion: false`, avec des cases de64px.

Les entrées automatisées ont été interrompues plusieurs fois par le message
« user input was detected in this window ». Les captures observées ne forment
donc pas une vidéo permettant de mesurer une démarche ou une cadence. Les
conclusions de raccordement ci-dessous sont établies par les conditions exactes
du lecteur. Les identifiants numériques des trois monstres de cette scène
n'ont pas été instrumentés ; leur couverture individuelle n'est pas affirmée.

## Constats

### 1. La configuration actuelle du joueur est exclue de la marche dessinée

`src/game_art_walk_approved.rs:20` exige simultanément : le modèle d'arme2,
aucune configuration de deux mains indépendantes, les trois palettes d'origine,
une des cinq classes et une des25 tenues sans casque supplémentaire.

La hachette utilise le modèle67, donc la condition échoue. Le dessin courant
est alors affiché pendant la translation, sans les poses de jambes du cycle.
Le fusil d'assaut2 est le seul modèle d'arme raccordé parmi113 modèles tenus.

Les cinq équipements initiaux échouent à cette condition : hachette67 pour
le Bastion, couteau de camp70 pour l'Arcaniste et la Métamorphe, pistolet101
pour le Technomancien, fusil de patrouille38 pour la Traqueuse. Le dernier est
un fusil, mais son identifiant diffère du fusil2 accepté par le lecteur.

### 2. La plupart des couples modèle/couleur de monstres sont exclus

`src/game_art_walk_creatures.rs:78` autorise uniquement `variant == 0`.
Les129 modèles de base sont présents. L'union du catalogue et des deux banques
de palettes compte471 couples modèle/couleur :129 animables et342 exclus.
Il s'agit d'une couverture de catalogue, pas d'une fréquence mesurée en partie.
La génération choisit réellement parmi ces variantes (`src/monster_art.rs`).
Ainsi, deux couleurs d'une même espèce peuvent utiliser des rendus différents.

Le drone canonnier dispose aussi d'un cycle raccordé. Les autres drones,
formes de transformation, PNJ et anciennes banques ne sont pas raccordés à
ces cycles. La banque WALK04 reste volontairement désactivée : elle contient
les essais rejetés et ne constitue pas une solution à réactiver globalement.

### 3. Le mélange des deux rendus explique le manque de cohérence

La position est interpolée même lorsqu'aucun dessin de marche ne correspond
à l'acteur. Les cas exclus peuvent donc glisser avec leur silhouette statique,
tandis que les cas pris en charge changent de jambes et d'orientation.
Changer d'arme, de casque ou de palette peut supprimer la marche du joueur.
Les déplacements verticaux gardent la face existante, conformément à l'accord.

### 4. Les400ms peuvent être raccourcies lors d'un maintien de touche

Le contrôleur réduit la durée visuelle au délai entre deux déplacements
successifs (`src/combat_visuals.rs:189`). La répétition clavier est réglée à85ms
après un délai initial (`src/controls.rs:686`). Un pas isolé demandé à400ms et
un déplacement continu peuvent donc présenter des cadences différentes ;
trois phases sur85ms laissent environ28ms par phase. L'impact visuel exact
reste à mesurer avec une capture temporelle contrôlée.

## Pourquoi les contrôles précédents n'ont pas suffi

Les tests vérifient que les cas explicitement raccordés utilisent leurs pixels
autorisés. Ils vérifient aussi que les équipements et palettes exclus reviennent
au rendu précédent : ils peuvent donc réussir avec une couverture incomplète.

Le diagnostic de déplacement réel équipe explicitement le fusil d'assaut2
(`src/walk_capture.rs:59`). Sa matrice montre d'autres armes, mais sur une seule
phase imposée (`src/walk_capture.rs:274`), sans exiger une marche animée pour
chacune. Les130 cycles installés ne prouvaient donc pas une couverture complète
des situations jouées. Mon compte rendu d'intégration était trop large.

## Suite recommandée

1. Recenser les configurations visuellement validées et les raccorder au
   lecteur du joueur, en commençant par les armes initiales ; ne pas remplacer
   une arme réelle par un fusil peint pour satisfaire la condition actuelle.
2. Préparer les déclinaisons de couleur de chaque cycle de monstre, en
   conservant leurs matériaux et leurs poses validées.
3. Vérifier les raccords de silhouette, de taille et d'ancrage entre repos,
   marche et face existante, puis observer la marche sur plusieurs pas.
4. Comparer un appui isolé et une touche maintenue pour décider du traitement
   de la cadence sans changer les tours ou le déplacement logique.
5. Contrôler les configurations de départ des cinq classes et plusieurs
   variantes effectivement générées avant de déclarer la couverture complète.

Aucune correction de rendu n'est appliquée pendant cet audit. Les395 fichiers
contrôlés du code, des banques de marche et du profil d'origine conservent
leurs empreintes. Les données chiffrées sont dans `coverage.json` ; les limites
d'observation et le contrôle de conservation sont dans `observation.json`.
