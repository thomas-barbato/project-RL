# ARMOR-STYLE-STUDIES02 — visages dégagés, cuivre simplifié

21 octobre 2026 — deux reprises en attente de revue visuelle.

Le joueur apprécie les deux nouvelles directions, et trouve le 01 adapté au
64 px. Il souhaite retirer la plaque sur le visage, dont l'inspiration paraît
trop identifiable, simplifier le Briseur et remplacer son orange par du cuivre.
Le retour exact est conservé dans `../armor-style-studies-01/feedback-01.json`.
Cet accord sur la direction ne valide pas les dessins modifiés ni leur intégration.

## Modifications

**01 · Vétéran industriel — BA-STYLE01-r02.** Visage humain avec deux yeux
naturels, sans demi-masque métallique ni œil lumineux. L'édition source
prévoit un petit implant près de l'oreille, qui ne constitue pas un détail
garanti lisible à 64 px. La tenue native est conservée pixel pour pixel.

L'édition générative du dessin entier est conditionnée en 64 px, puis seule
la zone de tête `[27, 3, 37, 12[` est reprise sur le 01 précédent. Le masque
d'édition explicite, le cadrage intermédiaire et le composite natif sont
conservés avec les sources. Les épaules, le col, le torse et les membres
restent exactement ceux de r01. Cette opération ne définit pas encore les
masques d'occlusion des futurs casques équipables.

**02 · Briseur cuivre — BA-STYLE02-r02.** Le cuivre rouge-brun remplace
l'orange sur les grandes plaques de torse, avant-bras et jambes. Les petits
marquages du casque et des épaules, les bandes des poignets et les accents
de ceinture, cuisses et bottes sont retirés. Le visage est humain et dégagé ;
la coiffe grise reste ajustée. Les volumes de cuivre ont de grandes zones
calmes, avec peu de plans de lumière. Les pixels chauds de peau des mains
et du visage restent distincts des surfaces métalliques.

## Réduction et contrôles

Le Briseur r01 était réduit directement de sa grande source avec Nearest.
La comparaison de la même source avec Lanczos montre des accents minuscules
plus durs et dispersés avec ce premier conditionnement. Nearest ne crée pas
de nouveaux éléments : il échantillonne des détails présents dans la source,
qui peuvent devenir trompeurs à la taille cible. Des bandes et marques
existaient aussi réellement dans le dessin et ont donc été retirées du r02.

Les deux nouveaux dessins sont conditionnés avec Lanczos en conservant
l'alpha et les proportions. Le zoom de contrôle reste un agrandissement
entier avec Nearest. La planche montre le 01 avant/après et le 02 avant/après,
en 64 × 64 sur fond sombre, clair et sur le même sol de rue. La comparaison
contrôlée du filtrage de l'ancienne source est séparée dans la galerie.

Les sources et PNG natifs sont dans
`assets/source-art/class-readability-01/armor-style-studies-02/`.
Les prompts exacts sont dans `prompts.json` ; les éditions ont été réalisées
avec le tool intégré `image_gen`. Le conditionnement Python ne redessine
ni les visages ni les armures.

Les deux exports sont RGBA, transparents, de 64 × 64, avec pieds à y=61.
Tous les pixels du 01 hors masque de tête sont identiques au r01 ; les 636
images et manifestes d'acceptation antérieurs recensés gardent leur empreinte.
Les contrôles sont enregistrés dans `checks.json` et la vérification du
navigateur dans `ui-checks.json`.

Il s'agit toujours d'études de style entières, sans rig équipable ni prises
d'armes validées. La règle torse/tête/casque reste inchangée. L'intégration
moteur, les casques indépendants, les prises et les variantes de couleur
restent différés. La rue est un montage sur une capture existante, sans
nouvelle simulation de l'éclairage des acteurs.
