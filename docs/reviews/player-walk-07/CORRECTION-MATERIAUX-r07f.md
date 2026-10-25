# WALK07-C11-r07f — deux jambes métalliques

**Accepté visuellement le24 octobre2026** : « ok on peut peut passer à la suite ».
Les sources exactes sont figées dans `visual-acceptance-r07f.json` ; la suite
est préparée dans [TRAQUEUSE-FAMILLE-r08.md](TRAQUEUSE-FAMILLE-r08.md).
Cet accord ne demande pas l'intégration ni l'acceptation des autres tenues.

Le 24 octobre2026, le joueur demande : « bon, le probleme n'arrivant pas à être
corrigé, je pense que tu devrais faire les 2 jambes en metal. » Cette demande
remplace explicitement la recherche d'une seule jambe métallique pour cet
essai de la Traqueuse. Elle autorise la préparation du nouveau dessin ; la
revue avant intégration reste conservée.

## Changement présenté

Traqueuse, tenue1, deux orientations latérales, repos et six poses de marche :
les **deux mollets portent les mêmes protections métalliques**, sous le genou
jusqu'à la cheville, d'après la protection existante. Les cuisses gardent leur
pantalon bleu et les bottes leur dessin. Les poses entières, la direction de
la tête, les bras et le mouvement actuel restent la cible de retouche.

Le candidat utilise les armes réelles dans les117 configurations de l'aperçu.
Les400ms par case restent réglables. Aucun bonus, statistique, nouvel objet ou
changement de règle n'est associé au métal dessiné. Les déplacements verticaux,
les autres tenues, la Métamorphe et les monstres ne sont pas repris dans ce lot.

## Sources et consignes

Retouche par **image_gen intégré**, sans CLI de génération. Deux sources
complètes sont conservées dans `assets/source-art/player-walk-review-07/` :

- `BASTION-TRANSFER-10-r07f-left.png`.
- `BASTION-TRANSFER-10-r07f-right.png`.

Les prompts exacts sont `prompts/CLASS-10-bilateral-metal-r07f-left.txt` et
`prompts/CLASS-10-bilateral-metal-r07f-right.txt`. Les entrées figurent dans
`bilateral-metal-jobs-r07f.json`, et les fichiers sélectionnés avec leurs
empreintes dans `bilateral-metal-sources-r07f.json`. Le conditionnement extrait,
réduit et aligne des figures entières, sans transformation d'une jambe isolée.

## Vérification et état de la revue

Les14 poses ont été examinées sur la planche de figures entières, puis avec
le fusil à taille native. Le contrôle de matériaux exige désormais du métal
dans les deux zones de mollet inspectées, au repos comme en marche. L'ancienne
exigence « un seul mollet métallique » est conservée uniquement pour les
révisions historiques ; elle est remplacée ici par la demande du joueur.

Les exports Rust de recettes et de pixels réussissent pour les117 équipements.
La comparaison des corps source, la protection des jambes contre la découpe
des bras et les empreintes inchangées des cinq atlas WALK06 passent dans
`bastion-transfer-checks-r07.json`. Les contrôles du rendu JavaScript et des
phases case par case sont conservés dans `renderer-checks.json` et
`raster-audit.json` ; ils utilisent un DOM/canvas simulé, sans exécution réelle
de ce candidat en navigateur ou dans le jeu.

Le candidat a ensuite reçu l'accord visuel ci-dessus. Aucune nouvelle ressource
n'est intégrée au jeu ; les déclinaisons r08 sont présentées séparément.
