# EG-HAB-03 — Habitats de fortune dans les égouts

Le joueur a validé le lot visuel EG-DEC-02 le 20 octobre 2026, puis demandé
des habitats de fortune et des barils pour enrichir davantage les lieux.
L'accord exact et les empreintes des ressources acceptées sont conservés dans
`../sewer-review-02/acceptance.json`.

Ce lot propose cinq dessins isolés : EG-H01 bâche rapiécée, EG-H02 abri de tôle,
EG-H03 couchage, EG-H04 affaires récupérées, EG-H05 groupe de barils.
Les identifiants désignent des candidats de revue et ne sont pas de nouveaux
noms d'objets présentés au joueur dans la campagne.

Les sources et les images natives sont sous
`assets/source-art/sewer-review-03/kit/`. `prompts.json` conserve les prompts
exacts utilisés avec l'outil imagegen intégré ; `source-inputs.json` conserve
les sorties et les dimensions de conditionnement. Le fond alpha et les formes
entières sont préservés, sans étirement ni découpe de fragments de scène.

Les dessins conservent la grille de 64 pixels, la vue de haut à axes parallèles,
les faces avant courtes et les matières mates sales. Les deux abris occupent
deux cases sur deux, avec l'ouverture tournée vers le bas. Le matelas a une
silhouette de 66 pixels au plus dans son grand axe ; le joueur reste à son
échelle native. Affaires et barils occupent chacun une petite case.

Trois regroupements différents sont placés contre les limites de salles sèches.
Les emprises de collision sont distinctes des cadres alpha et contrôlées avec
les silhouettes des anciens décors. Aucun objet nouveau ne recouvre les
canaux, passerelles, galeries, trajets de rats ni chemins protégés entre les
salles. Les acteurs et la topologie acceptés gardent leurs positions. Le réseau
de murs, ses coins et ses conduites proviennent du lot EG-DEC-02 inchangé.

La revue montre trois graines avec trois refuges et huit nouveaux objets par
carte. L'audit vérifie 100 générations déterministes et leur connectivité.
Les captures natives du moteur sont produites avec
`--ui-cold-sewer-habitat-review artifacts/sewer-review-03/engine`.

Les nouveaux habitats attendent une validation visuelle. Ils ne sont pas activés
dans la campagne. Aucune personne, quête, ressource de survie ou interaction de
refuge n'est ajoutée. Les chiffres et conditions de réanimation EG-NEC-02
attendent toujours une validation distincte ; aucune capacité n'est activée par
la validation des décors.
