# Premier essai de surface bâtie

2 octobre 2026 · génération 137.

La direction demandée est un monde largement construit, composé de secteurs,
de salles, de passages et d'installations, avec une densité d'exploration proche
de Cogmind. Les grandes friches naturelles ne constituent plus la cible à long
terme. Cette première étape enrichit les cartes jouables existantes ; elle ne
remplace pas encore tout le monde par un complexe continu.

## Ce qui change

La surface de départ conserve ses dimensions de 192 × 128, sa ville, son
prologue, ses personnages, ses ennemis et ses accès. Des rues en boucle
desservent des bâtiments répartis sur l'extérieur, y compris au sud et à
l'ouest. Les implantations alternent ateliers, entrepôts, locaux techniques et
bâtiments abandonnés, de dimensions et d'agencements différents. Certaines
grandes pièces sont cloisonnées. Chaque bâtiment possède deux entrées, dont
une peut être une porte ordinaire à ouvrir.

Les consoles inactives, conteneurs, serveurs et piliers utilisent les éléments
existants. Ce sont de vrais obstacles pour le mouvement et la vue, avec des
allées pour circuler. Ils ne deviennent pas automatiquement des machines
utilisables ou destructibles.

Les régions ordinaires de surface reçoivent elles aussi des bâtiments. Les
propositions sont validées après la pose des rencontres, caches, portes,
terminaux et installations : celles qui isoleraient une zone sont refusées.
Les villes régionales déjà dessinées conservent leur propre plan. Les
souterrains restent inchangés à cette étape.

Le nombre d'ennemis et les quantités de butin ne sont pas augmentés. La densité
du terrain constitue une première étape ; le rythme des découvertes et des
rencontres devra être travaillé avec ces nouveaux trajets. Les lignes de vue
et les itinéraires ayant changé, cette version ne constitue pas une validation
de l'équilibrage.

## Essayer

Lancer `Essayer_expedition.cmd`, puis choisir **Nouvel essai d'expédition** pour
obtenir une nouvelle carte. Les parties normales nouvellement créées utilisent
aussi cette génération.

Les sauvegardes des générations précédentes conservent leurs cartes. Reprendre
un ancien essai ne densifie donc pas sa surface. L'essai et la partie normale
gardent leurs emplacements de sauvegarde distincts.

Les captures et le compte rendu se trouvent dans
`artifacts/surface-dense-2026-10-02`. Le plan complet est un diagnostic hors jeu :
la partie conserve la perception et la mémoire normales du personnage.

## Référence de conception

Le développeur de Cogmind décrit une génération de salles et de circulations,
puis leur peuplement avec machines, réserves et rencontres. Cette séparation
entre structure et contenu sert de référence, sans reprendre ses cartes, ses
assets ou son univers :
[Map Composition](https://www.gridsagegames.com/blog/2015/05/map-composition/) et
[Tunneling Algorithm](https://www.gridsagegames.com/blog/2014/06/mapgen-tunneling-algorithm/).

La suite devrait rapprocher les bâtiments en secteurs reliés, donner des
fonctions et des interactions aux lieux, puis répartir les découvertes et
rencontres selon ces fonctions. Ajouter seulement des objets à une carte
ouverte ne suffira pas à obtenir ce monde.
