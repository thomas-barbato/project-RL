# WEAPON-SETS-01 — deux ensembles d’armes

18 octobre 2026. Visuel accepté : « ok je valide ce visuel :) ».
Les notes ci-dessous décrivent la proposition présentée avant cet accord.
Le fonctionnement intégré et sa conversion des sauvegardes sont documentés
dans `integration.md` ; les accords distincts figurent dans `decision.json`.

Révision acceptée ensuite, WEAPON-SETS-01-GHOST-02 : la seconde main d’une
arme à deux mains affiche son image exacte à 30 % d’opacité, en remplacement
du symbole et du texte « Deux mains ». Les notes historiques ci-dessous
décrivent le rendu précédent ; l’aperçu courant montre la révision intégrée.

## Demande déjà acceptée

Remplacer les trois emplacements actuels par deux ensembles libres, chacun
composé d’une main droite et d’une main gauche. Un seul ensemble est visible
et actif. La permutation de l’inventaire et celle près de la barre de
compétences changent d’ensemble. Une arme à deux mains occupe les deux mains.

## Rendu présenté

Deux cases sous la silhouette : main droite à gauche de l’écran, main gauche
à droite. Le bouton de permutation existant est repris entre ces cases.
Un repère I ou II indique l’ensemble visible, également près de la barre de
compétences. L’arme à deux mains apparaît une seule fois ; l’autre case porte
un symbole de liaison et « Deux mains ». Aucun nouveau dessin d’arme ou de
personnage n’est fabriqué.

Les captures montrent une paire pistolet/lame et un lance-flammes à deux mains,
puis la répartition inverse, dans les thèmes métal et terminal. Ces exemples
ne réservent aucun des deux ensembles à une catégorie d’arme.

Les intitulés « Main droite », « Main gauche », « Deux mains » sont repris de
l’interface actuelle. Les repères I / II et leur disposition sont à valider.
Aucune nouvelle valeur de combat, coût ou consommation de tour n’est proposée.

## Limite explicite du prototype

Le mode debug `--ui-cold-weapon-sets` utilise le véritable inventaire et la
véritable barre, avec un rendu conditionnel réservé aux captures. Sa partie
isolée conserve encore les trois emplacements du moteur actuel : les captures
ne prouvent donc pas le fonctionnement ni la sauvegarde de quatre emplacements.
Le lancement normal garde l’équipement actuel jusqu’à validation et intégration.

L’aperçu courant est `artifacts/weapon-sets-review-01/index.html`. Les captures
temporaires n’entrent dans aucune dépendance du jeu. Le code de capture reste
exclu des compilations release.

## Points d’intégration repérés

- Versionner les quatre emplacements et leur restauration ; conserver la
  compatibilité des anciennes générations et reprises à trois emplacements.
- Réserver virtuellement la seconde main pour une arme à deux mains : ne pas
  dupliquer une instance d’objet dans l’inventaire ou dans la sauvegarde.
- Remplacer le cycle des trois armes par une permutation des deux ensembles,
  synchronisée entre inventaire, barre, apparence et sélection de compétences.
- Conserver la sélection automatique mêlée/tir dans la paire active. Ne pas
  chercher l’arme requise dans l’ensemble de réserve.
- Examiner les bonus et ressources : certaines boucles actuelles parcourent
  aussi l’équipement de réserve. Cette revue visuelle ne valide aucune nouvelle
  politique de bonus.
- Adapter les info-bulles, raccourcis et actions d’équipement après validation,
  en distinguant textes proposés et règles déjà acceptées.

## Validation attendue

La demande fonctionnelle et le rendu WEAPON-SETS-01 sont acceptés. Le joueur
valide ensuite les bonus et compétences accordées par le seul ensemble actif.
Les optimisations précédentes restent indépendantes de cette revue.
