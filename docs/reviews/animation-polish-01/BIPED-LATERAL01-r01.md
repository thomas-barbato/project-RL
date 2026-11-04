# BIPED-LATERAL01-r01 — morphologies et prise du fusil

**Lot refusé le 2 novembre 2026.** Le joueur signale des jambes qui pédalent
ou montent trop et un mouvement artificiel du corps, pour le démon ailé et
les autres candidats. Les vérifications techniques ci-dessous ne valent pas
acceptation du rendu. R02 reprend seulement MON-144-0 avant extension.

Le 2 novembre 2026, le joueur juge CLASS-LATERAL02-r02 « bien mieux » et demande
la même reprise pour les monstres bipèdes, avec contrôle du port des armes.
Le retour favorable et les pixels présentés des trois classes sont conservés
dans `CLASS-LATERAL02-DECISION-r02.json`. Il ne valide pas les dessins encore
absents des autres armures, monstres ou configurations d'équipement.

## Premier lot à comparer

01 MON-005-0 : possédé voûté, deux jambes chaussées, bras longs. 02 MON-014-0 :
cultiste avec tabard court et bottes distinctes. 03 MON-156-0 : démon massif
cornu, appuis courts sous sa masse. 04 MON-144-0 : démon ailé marchant au sol,
deux bras et deux ailes conservés. Les sources frontales viennent du rendu
actuel VERT18-C06 ; les anciens cycles latéraux du comparatif proviennent du
chemin actif WALK07/WALK11. Aucun nouveau nom ni comportement de monstre
n'est introduit dans le contenu du jeu.

Chaque proposition compte huit figures entières par direction : contact,
charge, passage du pied puis reprise, suivis de l'appui opposé. Les bras,
la tête et les jambes sont dessinés ensemble, sans greffe, déformation de
bandes ou imitation anatomique entre familles. La cadence de revue reprend
650 ms par pas, 1 300 ms par cycle, avec contact et charge plus longs.
L'aller-retour sur 40 px et la pause de 650 ms sont des outils de comparaison,
pas une modification de la distance, des tours ou de la durée des commandes.

05 Bastion au fusil examine séparément le port de l'arme : tenue lourde,
modèle partagé 2, palette d'origine, deux directions. Les bras sont redessinés
en position de port. Le fusil conserve son dessin FIREARM02 et une échelle
fixe sur les seize phases. Ses prises sont placées sur les gants de chaque
figure ; les doigts d'origine sont restaurés au premier plan. Seule leur
propriété de couche change : aucun bras ou poignet n'est déformé ou déplacé.
Le contrôle des attaches est technique ; il ne remplace pas le jugement
visuel des doigts, coudes, occultations et proportions. Cette nouvelle pose
de port reste à examiner, y compris sa taille d'arme, avant déclinaison.

## Couverture et préparation

`BIPED-LATERAL01-COVERAGE.json` conserve les 87 sources déclarées à deux
supports et vérifie leur présence dans le catalogue actuel. Ce nombre est
un inventaire à examiner, pas 87 bipèdes confirmés ou corrigés. Les cas dont
le rig ou le descriptif indique quatre supports/un corps rampant sont marqués
comme contradictoires. Le premier lot confirme seulement les quatre
morphologies montrées. Oiseaux, autres créatures ailées, corps rampants,
formes flottantes et supports multiples exigent leur propre vérification.

Les trois classes r02 et la Traqueuse sont conservées. Les autres classes,
armures, prises à une main, deux mains, paires d'armes et palettes ne sont pas
couvertes par l'essai du Bastion. Les statistiques, équipements, identités,
sauvegardes et marches verticales restent conservés.

Les sources sont sous `assets/source-art/animation-polish-01/biped-lateral-01/r01/`
et `weapon-fit-01/r01/`. Les prompts complets, la correction de transparence
du démon massif et le mode intégré image_gen sont consignés dans
`BIPED-LATERAL01-PROMPTS-r01.md`.

Le mode Debug `--ui-cold-biped-lateral OUTPUT` prépare les images et textures
avant ses captures. Il compose chaque acteur à taille native sur le décor du
camp accepté, avec l'exposition normale respective du monstre ou du joueur.
Le lecteur précharge les 66 captures de la sélection, conserve au plus deux
sélections et propose pause, poses, côtés, cadences et zoom entier.
Les captures et preuves du lecteur sont reproductibles sous `artifacts/` ;
le jeu normal ne dépend d'aucune de ces captures.

État : premier lot en revue native ; aucune intégration au rendu normal.
La règle de `AGENTS.md` et la procédure de revue latérale imposent la
présentation des nouvelles déclinaisons avant intégration. Les autres modèles
et ports d'armes restent suivis dans la couverture, sans être déclarés validés.
