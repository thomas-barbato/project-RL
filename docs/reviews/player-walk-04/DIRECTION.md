# WALK04-r01 — archive de la revue statique

**Rendu actif corrigé : WALK02-FIX02, voir `INTEGRATION.md`.**
Le faible mouvement des tibias décrit ci-dessous a été rejeté et remplacé.

Intégration autorisée explicitement le 23 octobre 2026. Voir `INTEGRATION.md`
pour la couverture réellement activée et les vérifications dans le jeu.
Les 400 ms restent à évaluer par le joueur après essai.

## Choix conservés

- Une commande = une case de 64 × 64 ; 400 ms par défaut, choix 120 / 200 / 400 ms.
- Gauche et droite : deux vrais dessins de trois quarts, tête ET torse tournés.
- Haut et bas : vue de face. Diagonales : orientation selon la composante horizontale.
- Les déplacements logiques, tours, collisions et règles ne changent pas.

Le catalogue WALK03 déformait les images de face. Il a été rejeté pour cette
raison et ne constitue pas la direction demandée. WALK04 le remplace dans
`catalogue.html`. Son découpage des bras n'est pas réutilisé.

## Bras et armes

Les sources peintes conservent chaque bras entier, de l'épaule aux doigts,
ainsi que l'arme. Les figures sont extraites par appartenance de silhouette,
sans découpe rectangulaire à travers les coudes, poignets ou hampes.

Pendant un pas équipé, le haut du corps et l'objet porté restent une couche
rigide, déplacée seulement d'un pixel entier. Les pieds et le bas des tibias
bougent sous cette couche. Le conditionnement vérifie image par image que
les pixels du haut du corps sont inchangés à cette translation près.
Il ne faut pas confondre ce contrôle avec une validation artistique de la
géométrie peinte : hampe droite, fer cohérent, axe poignée/lame et deux prises
sur la même hampe restent à examiner dans les vues natives et leurs zooms.

L'ancien aperçu armé WALK02-r02 utilise désormais la même conservation des
bras et de l'arme ; le remplacement des jambes par celles d'un corps sans arme,
avec une découpe horizontale à travers le bas du manche, a été retiré.

## Couverture et limites de la revue

Les ID du catalogue actuel restent stables : 25 tenues de classe, 129 modèles
de campagne et leurs 471 variantes, 14 PNJ/drones/formes et 149 entrées historiques.
Le fichier `coverage.json` donne la couverture effectivement conditionnée,
les sources encore en préparation et les éventuelles erreurs. Une entrée en
préparation affiche sa source fixe explicitement ; elle n'utilise pas une
déformation de face comme substitut à une orientation dessinée.

Six configurations représentatives par tenue actuelle : sans arme, pistolet
101, marteau 97, fusil 2, hache à deux mains 66 et deux pistolets 101. Les
anciennes tenues ont leurs six configurations historiques correspondantes.
Cette revue ne couvre pas encore les 113 modèles d'armes individuellement,
ni toutes les combinaisons des six casques interchangeables. Elle ne permet
donc pas de valider ces combinaisons pour une intégration générale.

Les variantes de palette conservent la géométrie du même dessin de base.
Les couleurs partagées inchangées sont exclues du transfert pour préserver
les détails et objets portés. La vue de face est la composition actuelle exacte.
Les grands corps conservent leur dimension et leur point de contact actuels.
Les anciennes sources de 64 px peuvent recevoir un canevas transparent de
128 px afin de ne pas couper une tête ou une arme tournée. Ce cadre plus grand
ne change ni leur taille à l'écran, ni leur point de contact sur la grille.

## Sources, génération et vérification

Génération : outil intégré `image_gen`, sans CLI ni API de remplacement.
Sources peintes : `assets/source-art/player-walk-review-04/`.
Prompts exacts et références : `jobs.json`, `other-jobs.json`, `prompts/`.
Les références de classe incluent les sources canoniques acceptées à pleine
résolution ; les références de bestiaire utilisent les visuels réellement
sélectionnés par le rendu. Les sorties originales sont conservées.

Conditionnement reproductible : `tools/art/build_player_walk_review_04.py`.
Le compléter par `tools/art/register_player_walk_actors_04.py` pour recaler les
acteurs à configuration unique sur la hauteur visible de leur source actuelle ;
la transformation conserve les proportions de l'objet entier, sans cisaillement.
Revue native : `artifacts/player-walk-review-04/*-native.png` et catalogue.
Contrôles techniques : `packaging.json`, `coverage.json`, puis `verification.json`.
Scène de revue : sols clair, sombre et camp, autres acteurs et objet au sol.
Les pages conservent un iframe d'origine opaque et une CSP restrictive.
Le premier HTML attaché reste inchangé octet pour octet.

L'accord d'intégration a été donné après la préparation de cette revue.
Il ne permet pas de remplacer un modèle d'arme absent par une arme du prototype.
