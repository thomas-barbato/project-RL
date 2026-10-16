# Textures de surface — premier lot à relire

1er octobre 2026. Proposition visuelle, sans intégration dans le jeu.

La maquette `reference-validee.png` est la référence artistique approuvée dans
la conversation. Ce lot concerne uniquement la surface, la couche la plus haute.
Les couches suivantes feront l'objet de propositions distinctes, conformément
à `docs/DESCENTE_VERS_LE_NUMERIQUE.md` : évolution des matériaux, des silhouettes
et des formes, au-delà d'un changement de palette.

## Contenu

- `atlas-source.png` : nouvelle image générée avec l'outil intégré imagegen.
- `PROMPT.txt` : consigne exacte utilisée, avec la maquette comme référence.
- `review-data.json` : échantillons de couleur lus dans l'atlas pour le rendu
  de contrôle à 32 × 32 ; le fichier source n'est pas retouché.
- `preview.html` : scène de contrôle autonome et sélection des seize tuiles,
  avec affichage à 32 pixels par case et agrandissements entiers ×2 / ×3.
- `preview-32.jpg` : capture de la scène affichée à 32 pixels par case.

Ordre de l'atlas, ligne par ligne : métal A, métal B, béton A, béton B ;
caillebotis, gravier, terre, herbe sèche ; eau peu profonde, eau profonde,
mur droit, angle de mur ; porte fermée, porte ouverte, caisse, terminal allumé.

## Portée du contrôle

L'atlas généré mesure 1 254 × 1 254 pixels. Ses seize régions sont lues par
quarts, puis affichées sur une grille de 32 × 32 avec un échantillonnage au
centre des pixels et sans interpolation. Il ne s'agit donc pas encore d'un
atlas natif de seize sprites de 32 × 32. L'approbation demandée concerne leur
apparence à la taille cible.

Les personnages restent les repères textuels du mode Terminal. Le châssis du
joueur n'est pas redéfini. Les murs sont tournés dans la scène uniquement pour
examiner un assemblage ; leurs raccords et leur éclairage devront être repris
avant un jeu complet de connexions. Les transitions entre terrains demandent
encore des tuiles dédiées.

Le générateur a conservé une légère transparence dans les terrains (alpha
échantillonné entre 246 et 253), alors qu'ils étaient demandés opaques. Les objets
disposent de zones entièrement transparentes. Une finition technique devra
précéder leur usage comme assets définitifs.

Aucun fichier du moteur, du rendu du jeu, des réglages ou des sauvegardes n'a
été modifié par ce lot. La validation visuelle de l'utilisateur reste attendue
avant l'intégration.

## Vérification de la vue de contrôle

La scène et les seize sélecteurs ont été examinés dans le navigateur. Le passage
de 32 à 64 pixels par case et la sélection de la porte ouverte puis du terminal
fonctionnent. À 736 et 320 pixels de largeur, les cases restent réellement à
32 pixels ; la fenêtre de la scène se resserre sans déborder horizontalement.
Aucune erreur ou alerte de console n'a été relevée. Ces contrôles concernent
l'aperçu autonome ; ils ne constituent pas une validation du mode texturé en jeu.
