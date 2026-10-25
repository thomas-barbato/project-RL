# ARCANISTE-RED-ROBE-W05 — progression vers l’ancienne robe rouge

**Validé le 22 octobre 2026 par « je valide :) ».** Les cinq compositions
natives et leurs cinq têtes par défaut sont acceptées. L’accord
`ARCANISTE-RED-ROBE-W05-ACCEPT01`, les empreintes et les copies conservées
se trouvent dans `acceptance.json`. Cette validation ne déclenche pas
l’intégration au jeu.

22 octobre 2026. Après « bon je valide pour l'instant », le joueur précise
qu’il préfère l’ancien style de l’Arcaniste. Trois références existantes
sont présentées ; il choisit explicitement « 01 — Ancien personnage en jeu :
robe rouge, capuche et masque », puis précise que cette version pourrait
être l’une des dernières armures. La validation provisoire précédente est
donc dépassée pour cette direction. Aucun manifeste d’acceptation de la
gamme prune n’avait été créé avant ce retour.

## Progression proposée

Le rouge sombre, le tissu usé, les sous-vêtements charbon, le cuir et le
masque ivoire reprennent l’identité du sprite historique `FIN-16`. La
protection augmente par les couches, les manches, les poignets, les jambes
et les bottes. La robe reste dominante, avec une silhouette distincte des
plaques de mêlée du Bastion et du châssis robotique du Technomancien.

1. `AR-RR05-P01-r01` — tunique légère sans manches, bras dégagés, tête nue,
   petites protections de poignet et bottes légères.
2. `AR-RR05-P02-r01` — tenue à manches et capuche légère, visage partiellement
   ombré, renforts de cuir aux poignets et aux jambes.
3. `AR-RR05-P03-r01` — robe longue doublée, manches plus couvrantes, capuche
   profonde et bas du visage enveloppé de tissu sombre.
4. `AR-RR05-P04-r01` — ancienne silhouette exacte : robe rouge ample,
   capuche et masque ivoire fermé à fente verticale.
5. `AR-RR05-P05-r01` — robe protectrice à panneaux superposés, épaules et
   manches épaissies, avant-bras renforcés, gants et bottes plus protecteurs ;
   capuche rouge et masque ivoire conservés.

Ces descriptions sont les intentions de progression. Le volume et les
différences réellement visibles se jugent sur la planche en 64 × 64. Elles
ne créent aucun objet, bonus d’armure, rang ou règle de jeu.

## Dessins, références et contrôle

Les 01, 02, 03 et 05 sont de nouveaux dessins complets, fabriqués avec l’outil
intégré `image_gen`. Les prompts exacts, rôles des références, origines et
empreintes se trouvent dans `prompts.json`. Le grand concept historique
guide le vêtement ; son fond, sa lueur et ses effets ne sont pas repris.
Le vrai sprite historique guide les masses et la pose. Le même visage humain
de référence guide les parties exposées des 01–02 ; les tenues suivantes le
masquent progressivement. Il ne s’agit pas d’un collage de visage découvert
par-dessus les coiffes. La tête et le corps sont dessinés ensemble avec un
raccord de nuque court.

Le 04 réutilise l’image de jeu historique sans redessiner ses pixels. Son
image de 128 × 128 contient une silhouette de 40 × 60 px dans des marges
transparentes. Le recadrage `(32,34,96,98)` fournit la case de 64 × 64 et
aligne les pieds à y = 61, deux pixels plus haut que le cadre logique centré
du jeu. Il n’y a aucun rééchantillonnage de cette référence.

Les autres sources sont réduites de façon isotrope avec alpha, pieds au même
appui. Les couches natives de tête, corps et leurs masques sont conservées
pour l’étude. La recomposition par masque binaire conserve tous les octets ;
le composite alpha conserve également chaque pixel visible. Le PNG historique
stocke des couleurs sous quelques pixels totalement transparents, sans
conséquence sur le rendu. Les zooms entiers sont en Nearest.

Les fonds sombre, clair et le montage sur une capture de rue servent à la
revue native. Une comparaison réunit les quatre classes ; une autre place
l’ancienne robe à côté du nouveau palier très lourd. La rue n’est pas une
preuve de contrôle sous l’éclairage du moteur.

Les anciennes gammes sont conservées, ainsi que les quinze compositions
acceptées des trois autres classes. Cette nouvelle gamme est désormais
validée. Les têtes restent fournies par les torses ; casques équipables
indépendants, variantes, prises d’armes, animations et intégration sont
différés. Aucune ressource active du jeu n’est modifiée.

Les cinq images de la section sont chargées à leur taille naturelle dans
le navigateur intégré. La capture de contrôle est conservée dans l’aperçu.
Le premier dessin est également vérifié sur fond sombre et clair pour
confirmer la transparence réelle, indépendamment des couleurs stockées
sous son alpha nul. Ces contrôles ne constituent pas une validation visuelle
de la nouvelle gamme par le joueur.
