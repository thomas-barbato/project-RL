# C06-HELM01-r01 — casques adaptés au porteur

Demande du 30 octobre 2026 : « aussi il faudrait faire en sorte que la taille
des casques correspondent à la corpulence du personnage :D ».

La proposition 04 est acceptée le 31 octobre 2026 : « oui ca me convient,
ensuite on va faire des modifications sur les casques ». L'accord porte sur les
proportions et le placement montrés ci-dessous. Voir `HELM01-ACCEPTANCE.json`.
Les prochaines modifications de dessin restent à préciser et à présenter.

## Proposition 04

Chaque personnage reçoit un facteur constant pour ses six modèles de casques.
Le Bastion passe à 1,15 ; la Traqueuse à 0,86 ; l'Arcaniste à 0,96 ; le
Métamorphe à 1,06, par rapport à la taille actuelle. Ces valeurs constituent
un calibrage visuel accepté pour les quatre gabarits courants, pas une formule
universelle fondée sur la taille d'une boîte opaque.

Le facteur reste identique entre les niveaux d'armure d'un même porteur et entre
les phases. Une épaulière plus large ne fait pas gonfler la tête. La projection
latérale existante est conservée. Le casque garde ses matières, sa silhouette,
ses ouvertures et son identité d'équipement.

La mise à l'échelle se fait autour du bas de la coque, posé sur la nuque.
Les ouvertures, le masque de possession de la tête et la coque suivent le même
placement. Les anciennes franges de tête sont dégagées sous le nouveau casque ;
le visage visible vient toujours de la figure peinte. Le cou, le torse et les
armes ne sont pas redimensionnés avec le casque.

## Revue

L'aperçu compare côte à côte les quatre personnages : tête de la tenue témoin,
taille actuelle du casque, puis proposition 04. Il couvre les six casques
115–120, les tenues de base et lourdes, face/dos/gauche/droite, les six phases
et le retour frontal. Le camp fournit une comparaison native du Bastion et de
la Traqueuse sous son éclairage accepté.

Les armes des trois lignes utilisent la réduction modérée 01 acceptée et le
même recalage de figure C01. La comparaison isole donc le casque. Les sources
graphiques acceptées restent conservées. Les silhouettes anciennes, monstres,
PNJ et cas non couverts gardent leur rendu normal.

La taille ajustée est un prototype isolé, désactivé dans le rendu normal.
L'accord antérieur C01/C02-A/C03/C04 demeure conservé, ainsi que l'accord C06
sur cette proposition 04. Le lance-grenades C05-GL01-r01 attend son propre accord.

## Reproduction

```powershell
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-helmet-fit artifacts/animation-polish-01/helmets-r01/current
python tools/art/build_animation_polish_review.py
```
