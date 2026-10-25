# WALK07 — revue groupée des cycles restants

Suite de cette préparation : le joueur valide69 cycles et demande les corrections
M100/M104, le nettoyage et l'intégration. Les130 cycles de créatures, en comptant
les59 accords antérieurs et les deux corrections autorisées, sont désormais
installés. Voir `integration/INTEGRATION.md` pour l'état courant et ses limites.
Les sections suivantes conservent l'état de préparation présenté avant cet accord.

État au24 octobre2026 : **71 cycles de base proposés,994 poses**, sans
acceptation visuelle et sans intégration au jeu. Cela comprend68 identités de
campagne restantes et les trois cas auparavant différés M12, M23 et M44.
Les59 cycles déjà acceptés et leurs826 poses figées restent inchangés.

## Retours du joueur

La préparation groupée est autorisée par « en faite tu pourrais surement toutes
les faires , puis je vérifierais comme je le fais actuellement , et je te dirais
si y'en a à modifier ou non :) ». Cet accord permet la préparation, pas
l'acceptation automatique des dessins ni leur intégration.

Le joueur signale ensuite que plusieurs planches répètent le même sens sur
leurs deux lignes, puis « tu recommences à faire des planches qui vont dans la
meme direction ». Le défaut concerne effectivement plusieurs sources à14 poses.
Des sources intermédiaires à8 poses d'un seul sens ont également été affichées
trop tôt. Elles ne constituent jamais une comparaison gauche/droite complète.

## Sélection et présentation

- `bulk-draft-selection.json` désigne les71 sources actuellement proposées,
  avec leurs empreintes et l'association des moitiés gauche/droite.
- `bulk-review-groups.json` indique les groupes présentés. Ils gardent tous les
  pixels RGBA natifs, sans compression avec perte.
- Le lecteur affiche simultanément un corps entier qui marche à gauche et un
  autre qui marche à droite. Chacun utilise ses sept dessins propres : repos,
  puis six phases. Les positions bouclent après deux cases ; trois phases sont
  utilisées par case.400ms reste le défaut réglable à120/200/400ms.
- Les planches intermédiaires à un seul sens ne sont plus présentées isolément.
  Les tentatives rejetées restent des sources de travail, jamais des références
  acceptées. Les dessins actuels sont des candidats soumis au joueur.

Les remplacements reprennent des figures dessinées entières. Une source à un
seul sens peut fournir sa moitié complète après contrôle. Les deux résolutions
de source sont alors calibrées une fois par série directionnelle à partir du
repos, avec le même facteur pour toutes ses phases. Le conditionnement ne
retourne aucun corps, ne greffe aucun membre et ne déplace aucune bande de
jambes. Les manifestes `whole-figure-assembly-*.json` conservent les entrées,
indices, facteurs et empreintes correspondants.

Les asymétries ont leur propre contrôle : bras de pierre, bras-canon, tubes
dorsaux, manches, nodules, panneaux de costume et objets tenus. Pour les cas
répétant une inversion d'objet, une pose entière correctement orientée et
équipée a précédé le nouveau cycle de ce sens. Les sources et consignes de
toutes les tentatives restent versionnées sous `assets/source-art/` et
`prompts/`, avec journaux de l'outil de génération intégré.

## Contrôles et limites

Les planches natives de contrôle examinent repos et phases2/5 pour les deux
sens sur les71 cycles. Les cycles repris font aussi l'objet d'une inspection
de leurs planches complètes. Le résultat est une sélection pour revue, pas un
nouvel accord artistique.

Les vérifications automatiques portent sur14 figures complètes, six dessins
de marche distincts par sens, marges, calage, hauteur native, rendu RGBA sans
perte et empreintes acceptées. Le lecteur est exécuté en Node avec DOM/canvas
simulés ; ses deux vues sont vérifiées contre les commandes des poses
directionnelles correspondantes. Cela ne constitue pas une exécution du
navigateur ou du jeu et ne prouve pas à soi seul la qualité d'une démarche.

Les monstres de moins de64px conservent un unique décalage vertical centré pour
leur cycle et leur ombre. Les plus grands conservent leur hauteur et leur
ancrage. Aucun changement des déplacements haut/bas, tours ou règles de jeu.

Le périmètre couvre les129 silhouettes de base de campagne, en comptant les
58 déjà acceptées. Le drone accepté forme le59e cycle hors de cette campagne.
Les342 variantes de couleur sont répertoriées séparément dans
`bulk-catalogue-scope.json` ; la présente sélection ne vaut pas revue de chacune
de leurs palettes. Les anciennes banques, PNJ et armures hors de ce périmètre
ne sont pas déclarés adaptés par ce document.
