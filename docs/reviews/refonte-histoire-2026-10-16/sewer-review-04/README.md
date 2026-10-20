# EG-COMP-04 / EG-POP-04 — Composition et densité

Le retour du joueur sur EG-HAB-03 demande davantage de monstres pour le mélange
roguelike/hack'n slash, et des décors plus occupés avec une composition cohérente.
Ce retour ne valide pas le lot EG-HAB-03. La citation et les limites de la demande
sont conservées dans `request.json`.

## Décors proposés

Les sources acceptées EG01 et EG-DEC-02 restent intactes. Le plan des salles,
leurs raccords, leurs matériaux, le réseau de conduites murales, les anciens
équipements, les positions des anciens membres et les corps décoratifs restent
conservés. La révision ajoute des ensembles de mobilier selon la fonction des
salles : refuge dans une alcôve, réserves et ateliers contre les murs, équipements
et stockage sur les berges. Les nouveaux objets solides restent hors des
passerelles, galeries et trajets protégés. Le sol restant est intégralement relié.

Trois dessins isolés complètent le kit : EG-C01 rayonnage de service, EG-C02
palette de réserve, EG-C03 dépôt d'écoulement. Les sources et exports natifs sont
dans `assets/source-art/sewer-review-04/kit/`. Les prompts exacts de l'outil
imagegen intégré sont dans `prompts.json`. Les dessins EG-H01 à EG-H05 restent
des candidats et sont réemployés dans une disposition différente.

Les axes, les faces courtes et la grille native64 sont conservés. Les meubles
debout ne sont pas tournés pour simuler une autre orientation. Les silhouettes
entières, les bases de collision, les corps existants et les cases de capture
sont contrôlés séparément. Les dépôts sont plats, sans effet toxique mécanique ;
les grilles de drainage sont franchissables. Les conduites principales restent
sur les murs, avec les coins EG-MUR-02 acceptés.

## Peuplement proposé

Les graines 206014, 206027 et 206053 passent respectivement de 29, 30 et 30
monstres à 85, 104 et 78, en 11, 14 et 9 groupes. Les familles et profils
existants sont réutilisés. Les anciennes positions sont retenues ; les groupes
sont étoffés et les salles suffisamment grandes reçoivent parfois un second
foyer séparé. Le départ garde un rayon libre de six cases, et les nouveaux
ennemis sont éloignés d'au moins huit pas de trajet de l'arrivée.

Les deux scientifiques déjà proposés gardent leur présence. Les règles chiffrées
EG-NEC-02, les conditions de réanimation et son animation ne sont pas validées ni
activées par cette proposition. Les statistiques, dégâts, récompenses et règles
d'XP restent celles des profils existants. Les captures n'avancent pas l'IA et ne
constituent pas un essai d'équilibrage.

## Preuves

La commande de revue est `--ui-cold-sewer-composition-review` suivie du dossier
de sortie. Elle utilise uniquement les scènes de cette revue. L'export
`seed-N-scenery.png` montre le décor ; `seed-N-map.png` ajoute tous les monstres
pour l'évaluation du placement. Cette révélation complète est réservée à l'image
de revue et ne modifie pas le champ de vision ou la simulation. Les vues locales
restent des captures natives du moteur avec la vision habituelle.

La revue inclut les trois cartes, les comparaisons avec EG-HAB-03, les vues
natives des fonctions de salle et des rencontres, et les nouveaux dessins.
L'audit porte sur 100 générations déterministes. Le rendu du socle EG01 reste
identique pixel pour pixel pour le fond et les deux cartes d'éclairage.

Le 20 octobre 2026, le joueur valide les deux lots pour les tester en jeu.
`acceptance.json` conserve ses deux réponses et les empreintes des ressources.

## Intégration jouable — génération 162

Les nouvelles parties proposent la trappe en X 2, Y 14 dans les Abords de la
prison. L'interaction habituelle, au clavier ou au clic, entre dans les égouts.
L'escalier d'arrivée sert au retour dans la même rue. Le donjon est un lieu
latéral du même secteur : la règle historique des couches plus profondes reste
inchangée. Les anciennes générations conservent leur monde sans ajout de passage.

Ce premier essai choisit l'un des trois agencements acceptés selon la graine de
partie. Le choix ne consomme pas le générateur aléatoire de combat. La génération
procédurale complète à partir du kit reste une étape suivante ; cette intégration
ne prétend pas produire de nouveaux plans arbitraires au lancement.

Les plans et les images utiles sont conditionnés dans
`assets/maps/prison-sewer-01/`. Les sources de revue sont conservées. Le jeu ne
lit pas `artifacts/` pour fabriquer le lieu. Les obstacles d'eau et de mobilier
sont reconstruits pour les zones actives ou inactives après lecture d'instantané.
Les acteurs, objets et états de combat restent ceux de la zone persistante.

`Essayer_egouts.cmd` démarre une partie d'essai séparée près de la trappe, avec
l'équipement initial ordinaire du Bastion. `--variante-egouts=2` ou `=3` permet
de choisir les autres plans pour ce lanceur. Les sauvegardes de cet essai sont
dirigées vers `artifacts/sewer-playtest-04/`, à l'écart de la partie habituelle.
Une nouvelle partie normale permet de rejoindre cette même entrée depuis le camp.

Mise à jour après l'essai : le joueur demande aussi les monstres en ville.
La génération 163 active les quinze membres U01-POP-01 dans les nouvelles rues.
Le lanceur commence désormais au portail sud de la rue (X 7, Y 62), avec ces
groupes présents, puis le joueur rejoint la trappe en X 2, Y 14. Les fixtures
historiques de vérification de l'intégration 162 restent près de la trappe avec
leur rue originale. Les anciennes sauvegardes conservent leur génération.

Les chiffres et conditions de réanimation EG-NEC-02 restent hors de cet accord.
L'XP, les dégâts et les autres pouvoirs n'ont pas été rééquilibrés.

## Vérification de l'intégration

Les 23 tests ciblés passent : sept tests des égouts et seize tests du camp et de
la rue, dont les essais de peuplement existants. Un test GPU manuel historique
reste ignoré dans le filtre du camp. La compilation de développement, le contrôle
release, le formatage et le contrôle des espaces du diff passent.

La commande `--ui-cold-sewer-integration` contrôle dans le moteur les entrées,
retours et reprises d'instantané des trois variantes. Elle produit 18 captures :
les trajets réels et des scènes distinctes de contrôle du rendu livré. Ces
dernières utilisent les cartes et acteurs intégrés sans simuler un parcours de
combat. Le test de combat séparé vérifie un ennemi réellement vaincu et la
conservation de l'état et du butin après sauvegarde. Ces contrôles ne concluent
pas à un équilibrage de la difficulté.

`integration.json` conserve les résultats et empreintes des ressources livrées.
Les illustrations sources acceptées restent inchangées.
