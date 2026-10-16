# SKILLS-IC17 — reprise sombre de la banque compacte

Le 11 octobre 2026, après les quatre essais de SKILLS-IC16, le joueur indique :
« ca me plait bien mieux, ca veut dire aussi qu'il faut refaire les autres :) ».
Cette demande autorise la reprise des trente autres icônes compactes déjà
dessinées dans la même direction. Les nouveaux candidats restent à examiner.

Les numéros, identifiants, noms, types actif/passif et textes du catalogue sont
conservés. Les quatre essais de référence restent dans SKILLS-IC16. Les grandes
illustrations de fiche ne sont pas remplacées par ces compactes.

La génération utilise le mode intégré `image_gen.imagegen`, un appel par dessin.
Chaque appel reprend son ancienne icône comme référence de sujet et l'un des
quatre essais sombres comme référence de style. Les prompts exacts et les sources
sont conservés dans `sources.json`, `prompts.txt` et `prompts/`.

`index.html` présente les trente nouveaux dessins à 64, 32 et 24 pixels réels,
avec quatre filtres. Cliquer sur un dessin ouvre la comparaison avec la version
précédente. Les planches PNG par groupe et la planche d'ensemble reprennent les
mêmes exports sans zoom des petites vues.

`protected-baseline.json` couvre les ressources actives, les dessins acceptés,
les anciens candidats et les quatre essais de référence. Le conditionnement
vérifie leurs empreintes avant et après la préparation. Aucun nouvel accord
individuel ni remplacement dans le jeu n'est déduit de l'accord sur la direction.

Conditionnement : `tools/art/prepare_agent_skill_dark_catalogue_review.cjs`.
Les vérifications de dimensions, d'identités et d'empreintes sont enregistrées
dans `validation.json`. La revue visuelle à taille native reste distincte de ces
contrôles techniques.

Pendant la revue, le joueur demande un vrai bâillon pour Silence. Le candidat
31 devient AF03-C03 : bande de tissu serrée sur la bouche, nœud et attaches
visibles. AF03-C02 reste conservé ; la comparaison de Silence montre cette
révision et sa provenance complète figure dans l'historique de `sources.json`.

Les trente candidats et les quatre planches par groupe ont été observés aux
tailles natives. Les 330 images du navigateur sont chargées ; les vues affichées
gardent leurs dimensions réelles, sans débordement horizontal. Les quatre filtres
et une comparaison par groupe ont été vérifiés. Les captures et mesures sont
conservées dans `browser-layout-validation.json`, `browser-proof-native.png`
et `browser-proof-silence.png`. Le contrôle de syntaxe du conditionnement et
`git diff --check` passent. Les 364 fichiers protégés restent identiques.

Le joueur valide ensuite la série (« je valide :) »), y compris AF03-C03.
Cet accord exact est enregistré dans `visual-acceptance-01.json`. Les trente
compactes sont intégrées par le conditionnement habituel ; les données de la
préparation ci-dessus restent l'historique de revue. Les contrôles après copie
et les captures réelles figurent dans `../skill-window-native-04/`.
