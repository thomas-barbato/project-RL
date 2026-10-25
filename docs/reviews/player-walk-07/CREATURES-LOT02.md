# WALK07-M08 à M14 — deuxième lot anatomique, six candidats présentés

**Accord visuel du24 octobre2026 : « je valide :) ».** Les six cycles
M08, M09, M10, M11, M13 et M14 sont acceptés. Les six sources exactes et les
84 poses natives sont figées dans `visual-acceptance-creatures-lot02.json`
et `assets/source-art/player-walk-review-07/accepted/creatures-lot02/`.
La larve M12 reste différée. La demande suivante ajuste le placement vertical
des petits monstres, en conservant ces dessins et cycles exacts.

Le24 octobre2026, « je valide :) » accepte le premier lot M01 à M07.
Son accord et ses98 poses natives exactes sont conservés dans
`visual-acceptance-creatures-lot01.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot01/`.
Le présent lot reste distinct et proposé avant intégration.

| Candidat | Identité existante | Anatomie conservée |
|---|---|---|
| M08 | MON-031-0 | Six pattes, carapace bleu-vert et segments ambrés |
| M09 | MON-071-0 | Huit pattes, abdomen rouge et charbon |
| M10 | MON-083-0 | Huit pattes de marche, deux pinces et une queue arquée |
| M11 | MON-064-0 | Quatre appuis, chair pâle et cavité rouge du dos |
| M12, différé | MON-029-0 | Larve encore à reprendre, absente de l'aperçu |
| M13 | MON-125-0 | Esprit flottant, longues bandes de tissu, sans pieds |
| M14 | MON-145-0 | Deux jambes, deux bras, deux ailes et une queue |

Les libellés ci-dessus décrivent les morphologies pour la revue ; ils
ne renomment pas les créatures du jeu. Les variantes de matière et les
autres références du bestiaire restent à traiter séparément. Les catégories
historiques `quad` ou `crawl` ne servent pas de preuve anatomique.

## Méthode et portée

Chaque pose est dessinée entière. L'apparence vient du dessin actuel exact,
conservé dans `references/<identité>-native-current.png` ; les agrandissements
`*-identity-detail.png` servent à la fabrication. Le mouvement reprend la
cadence compacte et la cohérence des figures du premier lot accepté. Les
références anatomiques compatibles guident les appuis ; elles n'imposent
ni espèce, ni armure, ni couleur, ni forme de corps à la nouvelle créature.

Les pinces et la queue du scorpion ne deviennent pas des pattes. Le corps
du quadrupède reste bas ; la larve à reprendre devra se contracter contre le sol ; l'esprit
conserve des pans de tissu sans pieds. Les ailes du démon restent portées
pendant une marche au sol. Aucun vol, nouveau comportement ou effet n'est ajouté.

L'aperçu courant compare le dessin actuel et la proposition à l'échelle
native, avec un détail×2, trois sols, les commandes de pause et de pas manuel.
Le trajet avance de case64 en case64, trois phases puis les trois suivantes.
Les400ms restent réglables à120/200/400ms. Le repos à l'arrivée est une pose
dédiée. Les520ms de pause de démonstration ne sont pas un délai de jeu.
Les déplacements verticaux restent conservés. Le même aperçu de créatures
est réutilisé ; le premier lot accepté demeure figé dans ses ressources sources.

## Sources, prompts et corrections

Mode de fabrication : **outil image_gen intégré**, sans CLI ni API externe.
Les références, prompts exacts et destinations sont dans
`creature-jobs-lot02.json`, `prompts/<identité>-lot02-r01.txt` et
`generation-creatures-lot02-r01.json`. Les corrections et leurs prompts
complets restent dans `creature-corrections-lot02-r02.json`,
`generation-creatures-lot02-r02.json` et `creature-larva-lot02-r03.json`.

Les six dessins sélectionnés vont dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot02-r*.png`.
La sélection effective est `MON-{031,071,083,064,125}-0-cycle-lot02-r01.png`
et `MON-145-0-cycle-lot02-r02.png`, dans ce dossier de sources.
Le manifeste de génération conserve les chemins originaux et empreintes ;
`creature-jobs-lot02.json` indique la sélection effective, y compris les
dimensions et l'ordre constatés des atlas. Les sources rejetées ne servent
pas de références acceptées.

Une première larve recevait une seconde ouverture et des changements de sens.
La reprise r02 conserve encore ce défaut ; l'outil rejette la génération r03
à l'étape de modération de sortie, sans produire de dessin utilisable. M12
est donc différé et n'entre pas dans la présentation ou sa future validation.
Le détail reste dans `creature-deferred-lot02.json`. Une première planche du démon bleu ne
conservait pas assez l'envergure et contenait32 figures. Sa reprise contient
14 figures entières, en deux rangées de sept ; les deux moitiés directionnelles
sont remises dans l'ordre gauche/droite au conditionnement.

## Contrôles et limites

Le conditionnement extrait les figures entières, applique une seule échelle
uniforme par créature et les aligne au point natif de référence, sans
cisaillement, transformation de membres ni ajustement de taille par image.
Le canevas128 conserve les marges des débordements ; la case logique et
l'unité d'échelle du jeu restent64px.
Les pixels des références sont copiés sans redimensionnement. Les poses
acceptées du premier lot sont comparées exactement à leurs atlas figés.

`creature-packing-lot02.json` et `creature-validation-lot02.json` conservent
les dimensions, les sources sélectionnées, le cadrage et les contrôles des
84 poses proposées. `renderer-checks.json` vérifie le lecteur, ses sélections,
les six phases, les deux orientations et le repos. Ces contrôles utilisent
Node avec DOM/canvas simulé ; aucune exécution réelle de ces candidats en
navigateur ou dans le jeu n'est revendiquée. Les planches natives sont examinées
dans `artifacts/player-walk-review-07/`. Ces mesures ne prouvent pas à elles
seules la qualité d'une démarche ou l'identité anatomique d'une patte.

L'accord artistique de ce lot est enregistré. Aucun fichier du rendu actif,
aucune règle, statistique ou collision n'est modifié par cette proposition.

Résultats du24 octobre :84 poses proposées et98 poses acceptées du lot01
vérifiées ; tous les pixels figés restent exacts. Les hauteurs natives du lot02
sont33–35,54–57,47–48,29–32,64–66 et88–89px. Les deux directions des six cycles sont
examinés sur les planches complètes. Les2938 sélections des données de revue
se chargent ; les601 instants du contrôleur correspondent à la cadence
WALK05-STEP03. Les six aperçus courants restent sous1Mo chacun. Les300 fixtures
du compositeur d'armes conservent l'écart visible maximal antérieur de2/255.
Ces résultats techniques ne constituent pas l'accord artistique du joueur.

Le placement suivant est documenté dans `PLACEMENT-VERTICAL-01.md`.
