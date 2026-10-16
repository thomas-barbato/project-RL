# Propositions de mêlée futuriste — 5 octobre 2026

Statut : **aperçus à valider**, sans intégration dans le jeu.

## Épée commune portée

Une lame courte en métal, avec une poignée technique, un petit logement de batterie, une rainure cyan et un tranchant lumineux fin. Le dessin reste une épée solide et garde la pose actuelle, levée sur le côté.

[comparatif-epee-portee.png](comparatif-epee-portee.png) montre le dessin actuel et la proposition sur quatre tenues représentatives : Cerbère, Azurite, Fantôme et technicien. Le comparatif présente un agrandissement et une réduction à 32 pixels. Les quatorze tenues et les trois classes ne sont pas encore remplacées par cette proposition.

La première proposition décalait la lame par rapport au milieu de la garde. La version corrigée reprend cette jonction et l'axe de la poignée sur les quatre figures. [garde-corrigee.png](garde-corrigee.png) montre les personnages, leur réduction en 32 pixels et un gros plan sur la garde. Le premier comparatif reste archivé dans `comparatif-epee-portee-v1.png`.

## Icône commune au sol

[comparatif-icone-sol.png](comparatif-icone-sol.png) compare l'icône actuelle et la nouvelle épée sur trois matières réellement utilisées par le jeu, agrandies trois fois et à leur taille de 32 pixels. La version initiale était trop fine et a été corrigée pour renforcer la silhouette.

## Six armes à moderniser

[comparatif-six-armes.png](comparatif-six-armes.png) compare les illustrations actuelles et les propositions. Les noms sont des noms de travail. L'aspect des batteries, canaux, contacts ou logements mécaniques constitue une proposition visuelle et n'ajoute pas d'effet de combat.

| Objet existant | Identifiant conservé | Nom de travail |
|---|---|---|
| Épée longue | core:melee_epee_longue | Lame d’induction |
| Couteau de camp | core:melee_couteau_de_camp | Couteau à impulsion |
| Hache de guerre | core:melee_hache_de_guerre | Hache de brèche |
| Marteau de guerre | core:melee_marteau_de_guerre | Marteau à percussion |
| Lance | core:melee_lance | Lance conductrice |
| Gourdin ferré | core:melee_gourdin_ferre | Matraque à décharge |

Les illustrations individuelles concernent les objets de l'inventaire et des panneaux de détails. Le joueur continue de partager une seule épée portée pour la catégorie mêlée, conformément à la décision précédente.

## Production et vérification

Mode de production : **imagegen intégré**. Les prompts exacts sont dans [PROMPTS.md](PROMPTS.md). Sources enregistrées ici :

- six-armes-futuristes-source.png : les six illustrations proposées.
- personnages-epee-futuriste-source.png : quatre exemples avec la nouvelle épée.
- personnages-epee-futuriste-v2-source.png : correction de l'alignement lame, garde et poignée sur les mêmes quatre exemples ; source du comparatif final.
- icone-melee-futuriste-source.png : premier essai de l'icône au sol.
- icone-melee-futuriste-v2-source.png : icône corrigée pour sa lisibilité.
- icone-melee-futuriste-32.png : version proposée en taille réelle.

`build_previews.py` ne dessine aucune arme : il cadre les sources, nettoie l'alpha, redimensionne et dispose les comparatifs. Les textures de sol viennent de la banque existante.

Ces images sont des **aperçus préparés hors du client**, pas des captures d'une version modifiée du jeu. Les comparatifs ont été inspectés à grande taille et avec les vignettes de 32 pixels. `preservation.json` confirme que les 215 fichiers d'assets actifs, de rendu, de contenu et de traductions contrôlés sont identiques à ceux du début de cette préparation. Aucun code actif, atlas embarqué, nom du contenu ou profil d'arme n'a été modifié. Aucun commit ni publication.

Après validation, seuls les dessins et noms retenus seront intégrés ; les profils de combat restent une décision distincte.
