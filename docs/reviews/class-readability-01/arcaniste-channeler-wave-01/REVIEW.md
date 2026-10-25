# ARCANISTE-CANALISATION-W01 — cinq protections de canalisation

22 octobre 2026. Après les validations du Bastion, du Technomancien et de la
Traqueuse, le joueur confirme la poursuite. L’Arcaniste est la quatrième classe.
Ces cinq nouveaux dessins attendent leur revue en 64 × 64 ; aucune intégration
au jeu ni modification de règles n’est réalisée.

## Identité proposée

Un homme élancé, visage humain pâle, cheveux blancs quand la tête est ouverte,
manteau prune, protections de céramique claire et quelques masses cuivrées
sur les avant-bras de canalisation. Les compétences de départ existantes
`FE01` — Braise et `A03` — Canalisation guident cette direction. Les implants
et leur dessin n’ajoutent aucun effet, bonus, appareil actif ou compétence.

Les nouvelles têtes restent compactes, proches des épaules et assorties aux
matières du torse. La gamme répond au retour sur les anciennes têtes trop
grosses et les raccords de nuque trop longs. Les nouveaux casques n’utilisent
pas une tête agrandie pour montrer davantage de détails.

La progression associe vêtement et céramique protectrice, du léger au lourd.
Bras, gants, pantalon, jambes et bottes évoluent avec le torse. La tête par
défaut est fournie par le torse ; un casque indépendant devra la remplacer
selon la règle déjà acceptée. Les coiffes et casques présentés ici ne créent
pas de nouveaux objets de casque.

## Candidats à examiner

1. `AR-CA01-P01-r01` : tunique très légère, bras dégagés et tête nue.
2. `AR-CA01-P02-r01` : manteau léger à manches et bandeau protecteur.
3. `AR-CA01-P03-r01` : manteau long, céramique de torse et coiffe renforcée.
4. `AR-CA01-P04-r02` : protections lourdes et casque ouvert assorti.
5. `AR-CA01-P05-r01` : manteau blindé, céramique épaissie et casque intégral.

Ces descriptions fixent les intentions de fabrication. La géométrie, la
couverture et les différences réellement lisibles sont à juger sur la planche
native ; les mesures sont conservées dans `candidates.json` et `checks.json`.
Le premier casque du 04 restait trop proche de la coiffe prune du 03. Sa
reprise r02 introduit une calotte, un front et des joues de céramique claire,
en laissant le visage humain ouvert. L’ancienne source reste archivée.

## Fabrication et contrôle

Les dessins complets sont générés avec l’outil intégré `image_gen`.
Les prompts exacts, rôles des références et empreintes se trouvent dans
`prompts.json`. La base historique guide seulement l’identité humaine et les
implants cuivre/céramique ; ses anciens équipements ne sont pas réapprouvés.
La Traqueuse acceptée guide la finition des grandes valeurs, pas la morphologie
ou la tenue. Le témoin intermédiaire est dessiné entier ; son visage recadré
guide ensuite les quatre autres tenues pour permettre des corps différents.

Les sources et exports sont conservés dans
`assets/source-art/class-readability-01/arcaniste-channeler-wave-01/`.
Le script prépare le cadrage isotrope Lanczos avec alpha, des masques de tête
explicites et les planches. Il ne peint ni ne recolore les dessins.
Chaque sprite occupe 64 × 64 et garde une silhouette élancée de 60 px, pieds
à y = 61, comme la base de cette classe. Les trois classes acceptées conservent
leurs proportions exactes. Les zooms entiers sont en Nearest.

La planche montre des fonds sombre et clair et un montage sur une capture
existante de rue. Le rendu sous l’éclairage moteur reste à examiner.
Une comparaison réunit les quatre classes dans la même grille. Les couches
tête/corps se recomposent exactement ; elles restent des couches d’étude,
pas des masques de remplacement de casque ou des prises d’armes validés.

Les sources et acceptations antérieures restent protégées par leurs empreintes.
L’accord sur la direction ne vaut pas acceptation des cinq nouveaux rendus.
Variantes, casques équipables indépendants, prises d’armes, animations et
intégration restent différés jusqu’à la préparation et la revue de l’ensemble.
