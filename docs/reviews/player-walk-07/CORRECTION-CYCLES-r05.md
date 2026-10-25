# WALK07-r05 — conserver le cycle accepté de chaque personnage

Le24 octobre2026, le joueur rejette également r04 : « ben pas trop non ... »
puis « pareil pour la metamorphe s'il te plais ». Les deux candidats r04 sont
rejetés. L'accord WALK06 reste conservé ; ces propositions ne sont pas intégrées.

## Cause de l'écart

Les personnages redessinés s'étaient éloignés du mouvement demandé : silhouettes
trop de profil, pas plus amples, variations entre les moitiés du cycle. La reprise
de plusieurs planches puis leur assemblage ne conservait pas les dessins acceptés.
Les contrôles de durée, de taille et de composition ne validaient pas la démarche.

Le contenu exact du canvas02 de la pièce jointe a été lu sans exécuter ses scripts.
Ses images de Bastion sont identiques aux références enregistrées, pixel pour
pixel (`exact-canvas02-comparison.json`). La mauvaise référence de fichier n'était
donc pas la cause ; la fabrication des nouvelles déclinaisons était en cause.

La découpe générique des bras atteignait aussi les cuisses.738 pixels des28 poses
de ces essais avaient été affectés aux bras avant correction. Un bras déplacé
pouvait alors emporter ces pixels. Ils sont désormais conservés dans le corps.

## Reprise locale

Les bases sont les poses entières de chaque personnage dans la banque WALK06
déjà acceptée : Traqueuse tenue1 (CLASS-10) et Métamorphe tenue5 (CLASS-24).
Leur tête, leurs jambes, leurs appuis, leurs bottes et leur position dans la case
restent ceux des mêmes images d'origine. Aucun bas de corps d'un autre personnage
n'est repris ; aucun membre n'est déformé ou greffé.

Seule la bande native y20..34 est restaurée pour retirer le fusil peint, retrouver
le vêtement derrière lui et disposer de mains vides. Les pixels hors de cette
zone restent exactement ceux de la pose propre au personnage. Le résultat généré
sert uniquement à cette retouche locale ; ses autres pixels ne sont pas utilisés.

La propriété des bras et des doigts s'arrête strictement avant y35 (y67 sur le
canevas128). Tous les pixels des jambes appartiennent au corps fixe. Les prises
et les mains sont mesurées à nouveau après cette limite. Les nouvelles armes
restent des couches réelles du jeu, rigides et liées à leurs prises.

## Revue

- WALK07-C11-r05 : Traqueuse tenue1.
- WALK07-C25-r05 : Métamorphe tenue5.

À gauche de chaque aperçu : la pose originale acceptée avec le fusil ; à droite :
la même pose avec l'équipement sélectionné. La comparaison est synchronisée,
en cases64 et400ms réglables. Les six phases et les poses de repos sont conservées.
Les huit autres tenues de ces familles restent à reprendre après examen de cette
adaptation locale. Les nouveaux bras et montages attendent l'avis du joueur.

## Sources et preuves

Sources : `assets/source-art/player-walk-review-07/LOCAL-10-r05-left.png`,
`LOCAL-10-r05-right.png`, `LOCAL-24-r05-left.png`, `LOCAL-24-r05-right.png`.
Retouche avec l'outil image_gen intégré, sans CLI de génération. Les cibles,
zones de restauration et prompts exacts sont dans `local-retouch-jobs.json`,
`local-retouch-generation.json`, `prompts/LOCAL-*-r05-*.txt` et `references/`.

`frozen-gait-checks-r05.json` vérifie les3276 poses des deux essais et leurs117
configurations chacune : tête et bas du corps identiques aux sources acceptées,
aucun pixel protégé dans les sources des bras ou doigts. Les cinq atlas WALK06
actifs gardent leurs empreintes. Les exports Rust des recettes et des compositions
ont réussi. Les contrôles du navigateur simulé restent dans `renderer-checks.json` ;
ils ne sont pas une exécution dans un navigateur ou une partie.
