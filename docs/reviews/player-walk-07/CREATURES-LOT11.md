# WALK07-M61 et M62 — onzième lot validé, sans nouvelle intégration

Le joueur répond ensuite « ok on continue :) ». Les deux sources sélectionnées,
28 poses natives et calages exacts sont figés dans
`visual-acceptance-creatures-lot11.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot11/`.
Les59 cycles acceptés totalisent826 poses natives. La demande suivante autorise
la préparation de tout le reste avant une revue globale, sans acceptation
anticipée ni nouvelle intégration au jeu (`bulk-preparation.json`).

L'accord « ok je valide :) » fige les quatre candidats M57–M60 du lot10.
Il ne valide pas ces deux bipèdes, dessinés lors de la préparation précédente
et présentés maintenant. Les57 cycles déjà acceptés,798 poses natives, restent
identiques. M12, M23 et M44 restent différés.

| Candidat | Identité existante | Anatomie et matières conservées |
|---|---|---|
| M61 | MON-162-0 | Deux jambes fines, deux bras, deux ailes rouges, tête pâle et tenue sombre avec étoffe rouge |
| M62 | MON-156-0 | Deux jambes massives, deux longs bras, deux cornes ivoire, corps facetté bleu/gris et étoffe brune ; aucune aile |

Les deux cycles conservent une marche compacte avec alternance des appuis,
peu de mouvement vertical et un véritable repos. Les ailes de M61 restent
distinctes de ses bras ; les pans de tissu ne deviennent pas une queue.
Les noms de revue ne renomment pas le contenu du jeu.

## Sources et prompts

Aucune nouvelle génération ni retouche dans cette étape. Les sources R01,
produites précédemment avec l'outil image_gen intégré, sont réutilisées :

- `assets/source-art/player-walk-review-07/MON-162-0-cycle-lot10-r01.png`.
- `assets/source-art/player-walk-review-07/MON-156-0-cycle-lot10-r01.png`.

Le suffixe lot10 garde leur provenance. Les empreintes et sorties initiales
restent dans `generation-creatures-lot10-r01.json`. Leurs prompts complets et
références sont dans `creature-jobs-lot11.json` et
`prompts/MON-162-0-lot10-r01.txt`, `prompts/MON-156-0-lot10-r01.txt`.
Le transfert depuis `creature-prepared-bipeds-lot11.json` garde ces sources,
prompts et identifiants M61/M62 exacts. La tentative de nettoyage R02 de M62
n'est pas sélectionnée ; les franges résiduelles ne sont pas annoncées comme
entièrement corrigées.

Le conditionnement extrait chaque figure entière et applique une seule
réduction uniforme au cycle. Il ne greffe aucun membre et ne déforme aucune
bande de pixels. Les deux rangées sources sont réassociées gauche/droite
pour chacun des deux candidats, sans miroir du corps ni changement des phases.

## Taille, placement et cadence

La grille reste64×64px. Les silhouettes conservent leur hauteur native maximale
de75px et91px ; les hauteurs des14 poses sont72–75px et88–91px. Le canevas128
est une marge de revue. Le repère d'appui reste[64,112] et le décalage vertical
est nul pour ces deux figures plus grandes que leur case. Aucun recentrage
indépendant des phases ni changement d'échelle pendant le cycle.

« Dessin actuel » montre la référence exacte ; « Nouvel essai » montre le candidat.
Le détail×2 reprend les pixels de la scène.400ms reste le défaut réglable,
avec choix120/200/400ms, pause, pas manuel et sols sombre/clair/texturé.
Trois phases accompagnent une case, les trois suivantes la suivante, puis
un vrai repos. Les520ms de pause de démonstration ne créent pas de délai en jeu.
Les mouvements haut/bas, tours, positions logiques et collisions restent inchangés.

## Contrôles et portée

Les planches sources et natives sont examinées pour la direction, les membres,
les proportions et les matières. Les dimensions et différences de pixels ne
prouvent pas à elles seules la qualité d'une démarche ou l'anatomie.

Preuves : `creature-packing-lot11.json`, `creature-validation-lot11.json`,
`creature-preview-pack-lot11.json`, `renderer-checks.json`, `source-manifest.json`
et `bastion-transfer-checks-r07.json`. La comparaison native est dans
`artifacts/player-walk-review-07/creature-lot11-comparison-native.png`.
Le lecteur est vérifié avec Node et DOM/canvas simulé ; aucune exécution réelle
de ces candidats en navigateur ou dans le jeu n'est revendiquée.

Contrôles terminés le24 octobre2026 :28 poses nouvelles, six poses de marche
distinctes par direction, sources sans contact opaque avec le bord extérieur,
références natives copiées exactement et zoom contenant la silhouette et sa
case. Les798 poses acceptées et leurs sources passent leurs empreintes et
comparaisons exactes. Le lecteur couvre2 984 couples de sélection,300 fixtures
de rendu et601 échantillons de cadence ; les six fragments restent sous1Mo.
Le fragment courant mesure387 184octets. Le transfert joueur vérifie16 380
poses pour dix tenues ; les cinq atlas WALK06 actifs restent inchangés.

Ce lot a reçu son accord visuel. Aucune nouvelle intégration
au jeu, changement de statistiques, d'identité ou de règles. Les variantes
supplémentaires et le reste du bestiaire gardent leur propre revue.
