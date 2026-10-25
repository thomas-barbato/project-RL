# TRAQUEUSE-SNIPER-W01 — cinq protections de tireuse / ninja

22 octobre 2026. Après la validation des cinq tenues sans robe et des têtes
corrigées du Technomancien, le joueur demande de poursuivre la suite.
La troisième classe est la **Traqueuse**, femme, avec la direction tireuse
d’élite / ninja demandée précédemment. Les cinq nouveaux dessins attendent
leur propre revue en 64 × 64. Aucun dessin n’est intégré au jeu.

## Identité proposée

Une silhouette humaine athlétique, une combinaison bleu ardoise et des
protections gris bleuté asymétriques. Les grands plans clairs du métal,
la peau visible et les visières pâles séparent les tenues du sol sombre.
La posture, les tissus et les proportions la distinguent du Bastion robuste
et du châssis entièrement robotique du Technomancien.

L’optique, l’avant-bras cybernétique à droite de l’image et le bas de jambe
cybernétique à gauche conservent le principe d’augmentations localisées de
la base féminine historique. Les armures peuvent les couvrir progressivement.
Ces choix visuels ne créent aucun nouveau bonus. Les compétences existantes
« Marquage balistique » et « Maîtrise des attaques à distance » guident la
direction tireuse ; leurs identifiants et effets ne sont pas modifiés.

La robe et les longs pans cérémoniels du Technomancien ne sont pas réutilisés.
Le casque ne doit pas grossir pour afficher davantage de détails. Les membres,
le pantalon, les gants et les bottes évoluent avec la tenue, ainsi que sa tête
par défaut. Les armes restent communes aux classes et les prises seront
préparées après les dessins.

## Candidats à examiner

1. `TR-SN01-P01-r01` : très légère, textile ajusté et tête découverte.
2. `TR-SN01-P02-r01` : légère, premiers renforts et cagoule/masque souples.
3. `TR-SN01-P03-r01` : intermédiaire, protections asymétriques et demi-casque.
4. `TR-SN01-P04-r01` : lourde, couverture des membres accrue et casque fermé.
5. `TR-SN01-P05-r02` : très lourde, exocombinaison renforcée et casque intégral.

Ces descriptions fixent les intentions. Les mesures et limites réellement
constatées après génération figurent dans `candidates.json` et `checks.json`.
La progression de masse et la lisibilité restent à juger par le joueur.
La première réduction du palier 05 était trop proche du 04. Sa reprise r02
utilise seulement une référence de casque pour épaissir librement le torse,
les hanches et les jambes. L’ancienne source reste conservée. Les emprises
extérieures ne s’élargissent que peu ; la couverture et les grandes masses
de plaques distinguent davantage les niveaux, sans prétendre que chaque
palier gagne une largeur fixe.

## Fabrication et conservation

Les dessins complets utilisent l’outil intégré `image_gen`. Les prompts
exacts et rôles des références sont conservés dans `prompts.json`.
La base féminine historique guide seulement l’identité et les augmentations ;
elle ne valide pas l’ancienne armure. Le Technomancien accepté sert seulement
de référence de finition, pas de morphologie. Le témoin intermédiaire complet
sert ensuite de référence commune aux quatre autres paliers.

Les sources et exports sont conservés dans
`assets/source-art/class-readability-01/traqueuse-sniper-wave-01/`.
Le script prépare le cadrage isotrope Lanczos avec alpha, les masques de tête
explicites et les planches. Il ne dessine et ne recolore aucun candidat.
Chaque sprite occupe 64 × 64, avec une silhouette de 56 px et les pieds
à y = 61. Le Bastion et le Technomancien gardent leurs 58 px acceptés.
Les zooms entiers utilisent Nearest et sont identifiés comme tels.

La planche principale montre les cinq tenues sur fond sombre, clair et dans
un montage sur une capture existante de rue. Ce montage ne prouve pas le rendu
sous l’éclairage moteur. Une autre planche compare les trois classes à même
grille. Les couches tête/corps se recomposent exactement mais restent des
couches d’étude : les masques de remplacement de casque et les prises d’armes
de production ne sont pas encore validés.

Les sources et acceptations précédentes, y compris les deux gammes validées,
restent protégées par leurs empreintes. Les variantes de couleur sont différées.
L’accord sur cette direction ne vaut pas acceptation des nouveaux pixels,
des casques équipables, des statistiques, des animations ou de l’intégration.
