# WALK07-ME-FAMILY-r09b — cinq tenues de la Métamorphe

**Accord visuel du 24 octobre 2026 : « oui je valide ».** Les cinq tenues
ci-dessous sont acceptées dans le périmètre présenté. Les dix sources exactes
et les cinq atlas natifs sont figés par `visual-acceptance-metamorphe-r09.json`
et `assets/source-art/player-walk-review-07/accepted/metamorphe-r09b/`.
Cette validation ne constitue pas une nouvelle demande d'intégration et
ne valide pas le lot de créatures ou les formes de transformation.

Le 24 octobre2026, après la présentation des quatre autres tenues de la
Traqueuse, le joueur répond : « ok on peut continuer ». Leur accord visuel
est conservé dans `visual-acceptance-traqueuse-r08.json`, avec les sources
exactes et les14 poses natives de chaque tenue. L'accord de la tenue1 reste
dans `visual-acceptance-r07f.json`. Aucune intégration nouvelle n'est déduite
de ces accords visuels.

## Suite présentée

- **WALK07-C21-r09** : Métamorphe tenue1, tunique olive sans manches, cuir,
  cheveux découverts, sans col de fourrure ni bandeau.
- **WALK07-C22-r09b** : tenue2, bandeau, col clair, protections légères,
  mollets en tissu et cuir avec genouillères et petits renforts de cheville.
- **WALK07-C23-r09b** : tenue3, col plus large, plastron et protège-tibias ivoire.
- **WALK07-C24-r09b** : tenue4, protections d'épaules et de cuisses, manches
  couvertes et petits renforts aux tempes.
- **WALK07-C25-r07** : tenue5 déjà présentée, conservée comme essai précédent.

Les quatre premières tenues constituent56 nouveaux dessins entiers. La tenue5
et ses sources r07 sont conservées. Les cinq tenues ont reçu leur propre
accord visuel, distinct de celui de la Traqueuse.
La référence animée à gauche reste le Bastion original du canvas02. Le sélecteur
permet de parcourir les cinq tenues, les113 modèles d'armes réels et les quatre
autres configurations de mains, soit117 configurations par tenue.

Deux orientations latérales proches de la face, six phases et un vrai repos
sont conservés. Les400ms par case restent réglables à120/200/400ms, avec trois
phases sur une case et les trois suivantes au pas suivant. Aucun dos, nouveau
déplacement vertical ou changement de règle n'est ajouté. Les520ms de pause
de démonstration restent propres à la revue.

## Identités, matières et sources

Le mouvement utilise les figures entières de la Métamorphe tenue5 r07, issues
du gabarit du Bastion. L'apparence vient du dessin canonique de chaque tenue.
Les matières propres à la classe restent tissu olive, cuir brun, fourrure et
plaques ivoire ; les jambes métalliques de la Traqueuse ne sont pas appliquées
à la Métamorphe. Les pieds gardent le cuir sur les tenues1–4. Les premières
sorties avaient repris des embouts ivoire du modèle lourd, ainsi que ses
protège-tibias dans la vue droite de la tenue2 ; une retouche de matière les
remplace par les protections légères correspondantes, sans changer les poses.

Outil **image_gen intégré**, sans CLI de génération. Les sources sélectionnées
sont dans `assets/source-art/player-walk-review-07/` :

- `BASTION-TRANSFER-20-r09-{left,right}.png`.
- `BASTION-TRANSFER-{21,22,23}-r09b-{left,right}.png`.
- `BASTION-TRANSFER-24-r07-{left,right}.png`, sources précédentes inchangées.

Les entrées et prompts exacts figurent dans `metamorphe-family-jobs-r09.json`,
`metamorphe-material-jobs-r09b.json`, `prompts/CLASS-{20,21,22,23}-family-r09-*.txt`
et `prompts/CLASS-{21,22,23}-material-r09b-*.txt`. Les empreintes sélectionnées
sont conservées dans `metamorphe-family-sources-r09.json` et le manifeste commun.

Le conditionnement extrait, réduit uniformément et aligne chaque figure
entière. Une seule échelle est utilisée par direction : aucun ajustement de
taille pose par pose, aucune tête ou jambe greffée, aucun cisaillement.
Les écarts de hauteur native restent au plus2px au cours d'un cycle. Les bras
et doigts sont séparés pour les prises des armes réelles, en protégeant les
pixels du bas du corps et les contacts entre gants, tunique et cuisses.

## Contrôles et limites

Les exports Rust de recettes et de pixels réussissent sur les quatre nouvelles
tenues, chacune dans117 configurations. Les9828 comparaisons du lot précédent
sont étendues à16380 poses des deux familles, soit8190 pour la Métamorphe et
8190 pour la Traqueuse. Les corps source restent invariants selon l'arme ;
les masques de jambes restent exclus des bras et doigts. Les poses entières
acceptées et leurs fichiers de la Traqueuse sont comparés aux références
figées. Les cinq atlas WALK06 actifs conservent leurs empreintes.

Les planches de poses entières et les compositions avec pistolet, fusil et
hache sont examinées à taille native dans `artifacts/player-walk-review-07/`.
Les contrôles JavaScript et les fixtures du compositeur réel sont conservés
dans `renderer-checks.json`, `raster-audit.json` et
`bastion-transfer-checks-r07.json`. Ils utilisent un DOM/canvas simulé ; aucune
lecture réelle de ces candidats en navigateur ou dans le jeu n'est revendiquée.
Ces mesures ne remplacent pas l'examen de la démarche et de l'équipement.

Les nouvelles tenues sont acceptées visuellement et restent hors du jeu.
Le bestiaire, les autres classes, les palettes et
les casques équipés supplémentaires conservent leur état précédent.
