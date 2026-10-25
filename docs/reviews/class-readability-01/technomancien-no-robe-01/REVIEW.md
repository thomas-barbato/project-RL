# TECHNOMANCIEN-NOROBE01 — progression mécanique sans robe

22 octobre 2026. Génération par l’outil intégré `image_gen`, sans intégration.
Les dessins complets et exports natifs sont dans
`assets/source-art/class-readability-01/technomancien-no-robe-01/`.
Le jeu et ses règles ne sont pas modifiés.

Le joueur demande ensuite des têtes plus lisibles et une évolution de leurs
protections. La revue courante des compositions est
`../technomancien-no-robe-heads-02/REVIEW.md`. Les corps de cette première
gamme sont conservés exactement hors région de tête ; cette gamme r01 reste
une référence de fabrication, sans acceptation déduite.

## Demandes et portée

Le joueur apprécie les formes de la gamme `TECHNOMANCIEN-MP01`, mais veut
reprendre la robe. Il choisit d’abord le rouge et des bordures dorées et précise
que les futures couleurs alternatives doivent changer seulement le rouge.
Pendant cette préparation, il demande : « bon essaye de faire une version sans
la robe. Car ca ressemble trop au mechanicus de warhammer là :) », puis
« continue à faire sans robe ».

La nouvelle direction retire robe, capuche, manteau, tabard et pans pendants.
Le palier intermédiaire `TE-NR01-P03-r01` sert de témoin pour poursuivre les cinq
paliers. L’identité reste presque entièrement robotique : petit crâne mécanique
découvert, visage sombre, mâchoire bronze et optique ambrée unique. Les coques
acier, le châssis sombre et les articulations remplacent l’habit cérémoniel.
Les sources de robe rouge et or restent archivées comme essais interrompus.

La poursuite autorise la préparation des dessins. Les cinq nouveaux rendus
doivent encore être examinés à taille native ; elle ne vaut pas acceptation
de tous leurs pixels. Les futures variantes restent différées. La règle de
changement du rouge est conservée comme intention et n’est pas étendue
automatiquement aux nouvelles surfaces métalliques sans robe.

## Candidats

1. `TE-NR01-P01-r01` : châssis très léger, mécanique apparente et quelques gardes.
2. `TE-NR01-P02-r01` : premiers renforts du torse et des membres, joints dégagés.
3. `TE-NR01-P03-r01` : témoin intermédiaire, protections articulées et corps découvert.
4. `TE-NR01-P04-r01` : coques courbes épaissies, membres davantage protégés.
5. `TE-NR01-P05-r01` : carapace mécanique renforcée, couverture la plus importante.

Les volumes réellement obtenus sont documentés après génération dans
`candidates.json` et `checks.json`. Les descriptions ci-dessus fixent les
intentions de fabrication ; la revue native décide de leur efficacité visuelle.

## Fabrication et contrôle

Les prompts exacts, rôles des références et origines des fichiers sont conservés
dans `prompts.json`. Les modifications sémantiques sont faites par `image_gen`.
Le script Python prépare seulement le cadrage, la réduction isotrope Lanczos,
les couches de tête explicites et les planches. Il ne peint ni ne recolore les
dessins. Les zooms entiers utilisent Nearest.

Chaque composition occupe 64 × 64, avec une silhouette solide de 58 px et les
semelles à y = 61. Les comparaisons montrent les cinq nouveaux paliers sur fond
sombre, clair et sur une capture existante de rue ; celle-ci est un montage.
Le Bastion cuivre accepté est repris sans modification à même échelle.

Les couches de tête permettent une recomposition exacte de l’étude ; elles
ne sont pas des casques équipables ni des rigs de mains validés. Une source
entière nouvellement générée n’est pas une retouche à pixels mécaniques
identiques de l’ancienne robe. Les sources antérieures restent intactes.

L’intégration reste différée jusqu’à la préparation et la revue des armures,
têtes par défaut, casques et bonnes prises d’armes de toutes les classes.
