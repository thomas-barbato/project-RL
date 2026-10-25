# TECHNOMANCIEN-MP01 — armures du prêtre-machine sombre

22 octobre 2026 — cinq propositions, en attente de revue native.

Le joueur demande de passer au Technomancien, de conserver son identité de
prêtre-machine sombre distincte du Bastion et de présenter une progression
de protection lisible en 64 px. Le Bastion cuivre reste validé et inchangé.

## Direction conservée

Un corps presque entièrement robotisé, un visage mécanique à une optique
ambre, une capuche ajustée et un manteau fendu sobre. Le graphite et le
tissu anthracite dominent ; les grands plans gris cendré et les membres
métalliques séparent la silhouette du sol sombre. Un panneau intérieur
aubergine désaturé constitue une masse calme, sans variantes de couleur.

La progression travaille le col, le châssis vertical, les protections des
avant-bras et les pans rigides du manteau. Les versions lourdes conservent
une silhouette de prêtre-machine, avec des épaules tombantes et un manteau
long. Elles ne reprennent pas la cuirasse cuivre et les gros blocs
d'épaules du Bastion. Aucun symbole, drone, effet ou équipement fonctionnel
nouveau n'est ajouté par cette proposition.

## Paliers proposés

**01 · Très légère.** Tunique fendue courte, capuche souple, châssis et membres
mécaniques largement apparents. Protection légère des mains et pieds.

**02 · Légère.** Manteau textile de longueur intermédiaire, manches fines,
protection verticale du sternum et bas des jambes mécaniques dégagé.

**03 · Intermédiaire.** Manteau long, protection de torse resserrée,
petit mantelet tombant, avant-bras et jambes davantage protégés. Ce dessin
entier sert de référence commune de visage, palette et posture.

**04 · Lourde.** Col et mantelet épaissis, torse blindé vertical, grands pans
protecteurs sur abdomen et hanches, manchettes et protections de jambes renforcées.

**05 · Très lourde.** Mantelet et capuche structurés autour de la tête compacte,
châssis protégé en profondeur, lourds pans de manteau blindés sur les cuisses,
gantelets et appuis mécaniques plus massifs.

La numérotation 01–05 appartient à cette nouvelle gamme. Les anciens numéros
06–10 de la planche commune restent historiques et ne changent pas. Les
nouveaux IDs sont `TE-MP01-P01-r01`, `TE-MP01-P02-r02`, `TE-MP01-P03-r02`,
`TE-MP01-P04-r02` et `TE-MP01-P05-r02`. Il ne s'agit pas de cinq nouveaux
objets ou bonus : la correspondance au catalogue reste à préparer.

## Fabrication et preuve de lecture

Les éditions et générations utilisent le tool intégré `image_gen`.
Les prompts exacts, références et chemins de génération sont dans
`prompts.json`. Les deux anciennes sources tête/corps guident uniquement
l'identité du premier témoin. Chaque nouvelle tenue est dessinée entière,
tête et corps ensemble, pour éviter les raccords de nuque artificiels.

Le premier témoin intermédiaire r01 avait une capuche trop ample en réduction.
Une correction ciblée r02 resserre la capuche autour du même visage et garde
la tenue. Cette source complète corrigée guide ensuite les quatre autres
paliers. La première comparaison native montre ensuite trop peu d'écart
entre les coupes et protections des paliers 02, 03 et 04. Les paliers 02,
04 et 05 sont donc repris avec une référence limitée à la tête : manteau
léger plus court, vrai mantelet et pans rigides épaissis pour les lourds.
Les dessins supplantés et les prompts de correction restent historiques.

Les sources et exports sont dans
`assets/source-art/class-readability-01/technomancien-machine-priest-01/`.
Le conditionnement conserve alpha et proportions, avec réduction Lanczos
et pieds à y=61. Chaque dessin occupe une toile transparente de 64 × 64,
avec une silhouette solide de 58 pixels de haut. Il s'agit d'images générées
et conditionnées, sans prétention de retouche manuelle pixel par pixel.

Les largeurs hors tout sont 27, 26, 30, 36 et 35 px ; elles dépendent aussi
de la position des mains. Le dernier palier protège davantage le bas du corps
avec ses longs panneaux, sans exiger une largeur maximale supérieure à celle
du palier 04. L'emprise solide totale croît de 721 à 1 359 pixels entre
les deux extrêmes. Cette mesure décrit le dessin, pas une statistique de jeu
ni une preuve suffisante de lisibilité : celle-ci reste soumise à la revue.

La planche montre les cinq paliers sur fond sombre, clair et sur le même
sol de rue. Le zoom est un agrandissement entier ×3 avec Nearest. Une
comparaison séparée place les cinq Bastion acceptés et les cinq Technomanciens
à la même échelle, pour juger directement leur différence d'identité.
La rue est un montage sur une capture existante, sans nouvelle simulation
de l'éclairage des acteurs.

Les composants tête et corps sont dérivés du dessin entier, avec masques
locaux explicites et recomposition exacte. Ils servent à la préparation des
couches. Les points de prise, les bras avec armes et les masques des casques
équipables ne sont pas encore des rigs de production validés. L'ancre de sol
est [32, 61]. La tête montrée appartient au torse par défaut ; un casque
équipé la remplacera selon la règle déjà acceptée.

## État de validation

Le choix de direction ne valide pas automatiquement les cinq dessins.
Cette gamme attend la revue du joueur en taille native. Les images et
manifestes d'acceptation antérieurs recensés dans `prior-images.json`
doivent conserver leurs empreintes, notamment les cinq Bastion figés.
Les contrôles sont dans `checks.json`, et ceux du navigateur dans
`ui-checks.json`. L'intégration et les variantes de couleur restent différées.
