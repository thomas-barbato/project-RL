# ARCANISTE-SHORT-NECK04 — nuque raccourcie

22 octobre 2026. Le joueur juge la nuque de l’Arcaniste trop haute et étrange.
La correction rapproche la tête des épaules sur les cinq tenues, sans
redessiner les visages ou les coiffes de la revue précédente.

## Correction proposée

Les candidats `AR-NC04-P01-r01` à `AR-NC04-P05-r01` conservent les numéros
01–05 et leur progression du très léger au très lourd. La couche de tête
existante est abaissée de deux pixels natifs, sans redimensionnement ni
rééchantillonnage. Les essais à un et trois pixels ont servi au contrôle
du raccord ; la proposition présentée utilise deux pixels pour les cinq.

Les mêmes pixels de tête et de visage sont conservés par translation entière.
Les coiffes gardent leurs occlusions : front, tempes ou bords des joues
peuvent rester couverts. Le bas du corps garde son échelle et ses appuis.
Les silhouettes solides font désormais 58, 58, 59, 58 et 58 px de haut dans
la case de 64 × 64 ; elles ne sont pas agrandies pour compenser la nuque.

Les masques d’appartenance de l’ancienne et de la nouvelle tête délimitent
le raccord corrigé. Ils retirent l’ancien placement et masquent le cou sous
la tête abaissée, en conservant les épaules latérales. Tous les pixels hors
de ces empreintes et toutes les lignes à partir de y = 13 restent identiques.
Le changement local au col est nécessaire au raccourcissement de la nuque.

## Fabrication et portée

Il s’agit d’un réglage de composition des couches existantes, sans nouveau
dessin généré. `recipe.json` conserve les références, empreintes et déplacements.
Les grandes sources sont également composées à partir des têtes existantes,
avec un déplacement entier correspondant au cadrage d’origine. Les couches
tête/corps et les masques d’attache sont conservés séparément. Le natif est
la référence exacte de placement ; les anciennes images restent intactes.

Les planches présentent les cinq tenues sur trois fonds, les raccords
avant/après au zoom entier ×4 et les quatre classes dans une même grille.
La rue est un montage, pas une exécution sous l’éclairage du moteur.

Les quinze compositions acceptées des autres classes restent figées.
La correction attend une revue en 64 px. Casques équipables indépendants,
prises d’armes, variantes, animations et intégration restent différés.
Aucune règle, identité de contenu, compétence ou texte joueur ne change.

Le navigateur intégré charge les quatre planches de la section à leur taille
naturelle, sans redimensionnement CSS. La capture de contrôle reste dans
l’aperçu. Cette vérification technique ne vaut pas validation par le joueur.
