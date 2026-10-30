# VERT18-C05 — reprise des gabarits

Le joueur signale que les personnages changent de taille entre marche et repos,
puis demande « compare les toutes :) » et « oui donc il faut corriger tout ca ».
C04 reste non validé ; les références acceptées C01, C02 et C03 sont conservées.

Le repos précédent reprenait le corps de l'ancien rendu normal, tandis que la
marche montrait une nouvelle figure complète. Les proportions différaient même
lorsque la hauteur totale était proche. Le conditionnement utilisait aussi une
échelle médiane pour les seize figures d'une planche, sans corriger les écarts
d'échelle entre ses rangées. La Traqueuse lourde montrait les écarts les plus
forts : environ 60 px de face, puis 52 à 57 px de dos selon la phase.

C05 reprend les neuf silhouettes déjà présentées. Il emploie une vraie pose de
repos frontale issue du même dessin entier que la marche, avec deux pieds posés.
Chaque rangée est calibrée par rapport à sa propre pose neutre ; le redimensionnement
reste uniforme sur X et Y et se fait dans les ressources, avant le moteur. Aucun
zoom, étirement de membre ou déplacement de bande ne simule une correction pendant
la marche. Les petites variations de hauteur liées à la flexion et aux appuis restent
possibles. Les pieds gardent leur repère dans la case native64.

Le dos de la Traqueuse lourde est redessiné en figures entières : sa source avait
aussi des jambes raccourcies et une tête disproportionnée. Le reste des dessins
entiers vient des sources existantes. Les armes sont composées indépendamment sur
les nouvelles paumes ; leurs dessins communs restent conservés. Les six dessins
frontaux de casque et les têtes modulaires de C04 sont réutilisés.

Le comparatif C05 distingue les anciens dessins et les captures corrigées du moteur.
Il contient neuf comparaisons avant/après, cinq ports d'armes pour les quatre armures
lourdes, les sept choix de tête avec ou sans les deux armes, l'Arcaniste de base armé
et les séquences dans le camp. La vue native montre les pixels sans redimensionnement.

La nouvelle pose de repos et les variantes montrées sont validées le 29 octobre
2026. C05 demeure un lot visuellement accepté, en attente d'intégration normale.
Les dessins exacts et leurs dépendances sont figés dans `acceptance-C05.json`.
Les autres niveaux d'armure, prises à deux mains, palettes et autres PNJ restent
hors de ce lot. L'accord DUAL17-C02 reste distinct et toujours en attente.

- Ressources : `assets/source-art/vertical-walk-review-18/candidate-C05/manifest.json`.
- Nouvelle source de dos : `assets/source-art/vertical-walk-review-18/traqueuse-lourde-back-size05-r01-source.png`.
- Génération : outil imagegen intégré ; prompt exact conservé dans `prompts-C05.json`.
- Audit : `SIZE-AUDIT-C05.json` ; conditionnement reproductible par `tools/art/pack_vertical_walk_size18.py`.
- Captures : `--ui-cold-vertical-size artifacts/vertical-walk-review-18/current/size-C05/engine`.
- Publication : `tools/art/publish_vertical_walk_size18.py`.

## Vérifications effectuées

La compilation Debug verrouillée et les deux tests ciblés de sélection/cache
passent. Les captures du moteur couvrent 396 retours frontaux, 4 056 contacts
arme/main incluant le repos, et 672 contacts échantillonnés avec les casques.
Les 624 comparaisons de régions de corps entre les choix de tête sont identiques.
Les 30 fichiers de référence acceptés conservent leur empreinte.

Les 38 animations utilisent WebP sans perte ; les 1 196 images décodées ont été
comparées aux images de présentation. L'aperçu est ouvert et vérifié dans le
navigateur : les comparaisons 380 × 290 et les quatre armures 380 × 315 gardent
exactement leurs dimensions natives. Les captures de preuve sont dans le dossier
courant C05. Le jeu normal, les sauvegardes et les préférences restent inchangés.
Ces contrôles techniques ne remplacent pas la validation visuelle du nouveau lot.

## Accord du joueur

« pour l'instant ca m'a l'air bien alors je valide :) »

Cet accord valide les neuf corps corrigés, leur vrai repos frontal, les cinq
ports d'armes lourdes présentés et les 56 combinaisons de têtes/armes du lot.
Les 57 fichiers de ressources référencés et les 40 fichiers de présentation
conservent leur empreinte SHA-256. La validation visuelle et l'activation dans
le rendu normal restent deux états distincts. Les variantes absentes et DUAL17
ne reçoivent aucun accord implicite de cette réponse.

## Rangement après validation

Les 1 688 captures PNG brutes et atlas de diagnostic reproductibles sous les
seuls dossiers `headgear-C04/engine/` et `size-C05/engine/` sont déplacés dans la
Corbeille Windows. Les 57 fichiers de ressources, les 38 animations comparatives,
les deux captures de preuve et tous les contrôles JSON sont conservés ; leurs
empreintes ont été vérifiées après l'opération. La Corbeille n'est pas vidée et
aucun espace disque libéré n'est annoncé. Pour republier ou réexécuter l'audit
pixel, régénérer d'abord les captures avec la commande de diagnostic ci-dessus.
Détail : `cleanup-approved-C05-raw.json`.
