# Ensemble des armures des cinq classes — CLASS-ARMORS-ALL01

21 octobre 2026 — **25 compositions en 64 × 64, à comparer ensemble**.

Le joueur demande : « fait toutes les armures de toutes les classes puis ensuite
tu me montreras l'ensemble en taille réelle pour voir lesquelles seront à modifier
/ garder :) ». Cette cadence remplace la validation des vagues une par une.
La production complète n’autorise pas l’intégration ni l’acceptation automatique
des nouveaux dessins. Les variantes de couleur restent reportées.

## Numéros de revue

| Classe | Très légère | Légère | Intermédiaire | Lourde | Très lourde |
| --- | --- | --- | --- | --- | --- |
| Bastion | 01 | 02 | 03 | 04 | 05 |
| Technomancien | 06 | 07 | 08 | 09 | 10 |
| Traqueuse | 11 | 12 | 13 | 14 | 15 |
| Arcaniste | 16 | 17 | 18 | 19 | 20 |
| Métamorphe | 21 | 22 | 23 | 24 | 25 |

Les cinq Bastion 01–05 et le témoin du Technomancien 08 sont des références
déjà acceptées, conservées exactement. Les **19 autres compositions attendent
la revue commune**. La réutilisation d’une coiffe acceptée n’étend pas cet accord
à une autre composition de corps. Les numéros de poids sont des positions de
revue ; ils ne créent ni objets, noms de butin, statistiques ou restrictions.

## Identités et portée

- **Bastion** : corps robustes et blindages de mêlée validés, avec les cinq
  têtes assorties acceptées. Aucun dessin n’est refait.
- **Technomancien** : manteaux techniques et membres robotiques cuivrés ; la
  capuche souple et le col asymétrique acceptés sont réutilisés sur les cinq
  degrés de protection. Le modèle 08 reste strictement le témoin accepté.
- **Traqueuse** : femme compacte et athlétique, silhouettes ajustées de
  reconnaissance, vêtements courts et protections inclinées peu saillantes.
- **Arcaniste** : homme élancé, volumes verticaux de tabard et de robe technique,
  céramique et cuivre, avec des protections de canalisation de plus en plus couvrantes.
- **Métamorphe** : femme humaine au repos, musclée et ramassée, biotextiles et
  carapaces portées aux masses courbes ; aucune transformation du corps n’est ajoutée.

Pour les trois nouvelles gammes, une protection de tête propre à la classe est
commune à ses cinq tenues et fait partie de l’apparence fournie par le torse.
Les visages et augmentations de base sont conservés sous son masque explicite
d’occlusion. Les armes gardent des modèles communs aux classes et ne sont pas
dessinées pour cette revue. Les mains restent vides ; leurs prises sont un autre lot.

Le torse fournit l’ensemble de la tenue : tête par défaut, haut du corps,
bras, mains, pantalon ou robe, jambes et bottes. Un casque équipé remplace
uniquement la tête par défaut. Les objets de casque indépendants restent à
préparer et à revoir ; ces coiffes ne créent pas d’objets dans l’inventaire.

## Fabrication et contrôle

L’outil intégré **image_gen** fournit quinze modules de corps sans tête et trois
protections de tête. Les prompts et références exacts sont dans `prompts.json`.
Les sources sont copiées dans
`assets/source-art/class-readability-01/class-armors-all-01/`.

Le script `tools/art/prepare_all_class_armors_01.py` réalise uniquement le
cadrage, la réduction isotrope Lanczos, la composition par couches et les planches.

Les cinq premiers corps de la Traqueuse se fondaient trop dans le sol sombre
en 64 × 64. Leurs révisions r02 éclaircissent les grandes surfaces de protection
et simplifient les valeurs. Les sources r01 et leurs prompts restent tracés dans
`superseded_jobs` ; il ne s’agit pas de variantes colorées proposées en supplément.

Chaque case et chaque export de corps ou de tête mesure 64 × 64. La ligne
d’appui est Y=61. Les joints tête/corps sont explicites par classe et les deux
calques reconstituent les pixels RGBA exacts du personnage.

Les vingt-cinq compositions sont numérotées sur trois planches d’ensemble,
sur fonds sombre, clair et sol de rue. Chaque classe dispose aussi d’une
comparaison native sur trois fonds et d’un zoom entier ×3. Les images de revue
ne sont pas redimensionnées par le navigateur. Le sol de rue est un montage,
pas une preuve de rendu ou d’éclairage dans le moteur.

`candidates.json` conserve les sources, calques, identités, empreintes,
références acceptées et contrôles. Les références précédentes sont vérifiées
avant et après la préparation. `ui-checks.json` conserve le contrôle du navigateur.

## Décision et suite

Le joueur peut désigner les numéros 01–25 à conserver ou à modifier. Un accord
sur un modèle ne valide pas les autres modèles, couleurs, poses ou animations.
L’intégration reste différée jusqu’aux armures, têtes par défaut, casques et
positions des mains avec les armes préparés et revus.

Aucun fichier Rust, règle de jeu, catalogue d’objets, sauvegarde ou ressource
active n’est modifié. Le jeu ne dépend d’aucun export sous `artifacts/`.
