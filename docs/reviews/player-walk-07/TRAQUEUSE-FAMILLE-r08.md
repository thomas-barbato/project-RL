# WALK07-TR-FAMILY-r08 — déclinaison sur les cinq tenues

**Lot accepté visuellement le24 octobre2026** par « ok on peut continuer ».
L'accord et les empreintes sont dans `visual-acceptance-traqueuse-r08.json` ;
les poses natives sont figées dans
`assets/source-art/player-walk-review-07/accepted/traqueuse-r08/`.
La suite est [la famille Métamorphe r09b](METAMORPHE-FAMILLE-r09.md).
Les paragraphes ci-dessous décrivent la présentation avant cet accord ;
aucune intégration au jeu n'est autorisée par cette seule validation visuelle.

Le 24 octobre2026, après la présentation de la Traqueuse à deux jambes
métalliques, le joueur répond : « ok on peut peut passer à la suite ».
La tenue1 et son cycle latéral r07f sont donc acceptés visuellement ; les
sources exactes et leurs empreintes sont figées dans
`visual-acceptance-r07f.json`. Cet accord n'est pas une autorisation de nouvelle
intégration et ne valide pas les quatre nouvelles déclinaisons.

## Candidats à examiner

- **WALK07-C12-r08** : Traqueuse, tenue2, capuche et masque, petites protections.
- **WALK07-C13-r08** : tenue3, casque ouvert, bandeau de visière et plaques.
- **WALK07-C14-r08** : tenue4, casque entourant les yeux, protections de cuisses.
- **WALK07-C15-r08** : tenue5, armure complète et casque fermé à visière cyan.

Chaque tenue conserve deux vues de trois quarts proches de la face, six poses
de marche et un vrai repos, soit56 nouveaux dessins entiers. Les deux mollets
restent métalliques dans chaque pose, selon le choix fait pour la Traqueuse.
Les identités d'équipement, statistiques et règles de jeu sont conservées.
Le mouvement de référence est désormais celui de la tenue1 acceptée, elle-même
issue du Bastion du canvas02. Les identités de tenues proviennent des dessins
canoniques existants ; le stade de casque et la couverture des plaques restent
propres à chaque tenue.

Les cinq tenues sont disponibles dans le même aperçu : la tenue2 apparaît
d'abord ; la tenue1 validée reste à gauche pour comparer les poses. Le sélecteur
permet de choisir les autres tenues et les117 configurations, comprenant les
113 modèles d'armes réels et quatre configurations de mains. Les400ms par case
restent réglables à120/200/400ms. Les trois phases par case, le pas suivant et
le vrai repos à l'arrivée sont conservés. Les520ms de pause de démonstration
ne deviennent pas une règle du jeu.

## Sources et méthode

Outil **image_gen intégré**, sans CLI de génération. Chaque direction d'une
tenue est dessinée comme une rangée de sept figures complètes, à partir de
`BASTION-TRANSFER-10-r07f-left.png` ou de sa version droite, avec le dessin
canonique de sa propre tenue comme référence d'apparence.

Les huit sources sélectionnées sont
`assets/source-art/player-walk-review-07/BASTION-TRANSFER-{11,12,13,14}-r08-{left,right}.png`.
Les entrées et prompts exacts sont dans `traqueuse-family-jobs-r08.json` et
`prompts/CLASS-{11,12,13,14}-family-r08-{left,right}.txt` ; les empreintes sont
dans `traqueuse-family-sources-r08.json`. La référence native acceptée est
`assets/source-art/player-walk-review-07/accepted/traqueuse-r07f/whole-body-native64.png`.

Le conditionnement extrait et réduit uniformément chaque figure entière,
puis l'aligne par le buste et les pieds. Aucune tête ou jambe n'est greffée ni
déformée pour construire le mouvement. Les bras et doigts sont séparés pour
poser les armes réelles, avec protection explicite des pixels des jambes.
L'examen natif vérifie les contacts gant/cuisse et les armures plus larges ;
les limites de découpe ne constituent pas une règle universelle de morphologie.

## Contrôles et limites

Les exports Rust de recettes et de pixels passent sur les quatre nouvelles
tenues, chacune dans117 configurations. Les contrôles couvrent9828 corps et
poses composées des six tenues actuellement en revue ou acceptées (cinq
Traqueuses et le pilote Métamorphe), dont8190 pour la famille Traqueuse.
Ils vérifient le corps invariant selon l'arme, les masques de jambes, le métal
sur les deux mollets et la hauteur native stable. Les sources de la tenue1
acceptée et les cinq atlas WALK06 actifs conservent leurs empreintes.

Les planches entières et les compositions natives avec pistolet, fusil et
hache sont examinées dans `artifacts/player-walk-review-07/`. Les bilans
courants sont `bastion-transfer-checks-r07.json`, `raster-audit.json` et
`renderer-checks.json`. Les contrôles JavaScript utilisent un DOM/canvas simulé ;
aucune exécution réelle de ces candidats en navigateur ou dans le jeu n'est
revendiquée. Les mesures techniques ne remplacent pas la validation visuelle.

Les tenues2–5 ont ensuite reçu l'accord visuel conservé en tête de ce document.
La Métamorphe et le bestiaire conservent leur état précédent pour ce lot. Les déplacements
verticaux, les casques équipés supplémentaires et les palettes ne sont pas
étendus par cet aperçu. Aucune nouvelle animation n'est intégrée au jeu.
