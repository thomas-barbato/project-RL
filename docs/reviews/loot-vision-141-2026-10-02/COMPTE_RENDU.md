# Butin et vision — génération 141

2 octobre 2026. Modifications locales, sans commit ni publication.

## Butin intégré

À chaque mort hostile, le moteur choisit un modèle dans le catalogue d'équipement
humanoïde, sans restriction de famille portée ni de profondeur. Toutes les armes
et protections déclarées dans ce catalogue sont admissibles. Les modèles ont la
même probabilité ; le modèle de la récompense précédente est exclu lorsqu'il existe
une alternative. Le nombre de bonus et les propriétés sont tirés ensuite :
0/1/2/3/4/5/6 bonus aux taux existants 40/20/16/11/7/4/2 %.

Les armes utilisées par les ennemis gardent leurs profils de combat. Les robots et
animaux hostiles peuvent donner de l'équipement ; les civils, alliés, compagnons et
le joueur n'en donnent pas. Les morts directes ou environnementales passent par le
même point de dépôt. Il y a un seul exemplaire par mort et aucune relance au ramassage.

Le dernier modèle est un état persistant du joueur, conservé pendant les changements
de zone et dans RLWS v11. Le hasard suit les flux de zone existants. Une commande
refusée conserve cet état et le hasard. Les caches, boutiques et paris gardent leurs règles.

**Pour essayer : nouvelle partie ou Nouvel essai d'expédition via
`Essayer_expedition.cmd`.** La sauvegarde actuelle est en génération 140 et conserve
ses règles précédentes. Sa copie se restaure correctement par le journal vérifié
après refus du cache RLWS v10. L'original n'a pas été écrit, consommé ou déplacé.

## Vision intégrée

Le terrain visible reçoit un masque circulaire adouci au bord, calculé en pixels.
Le contour se situe environ 0,7 case à l'intérieur du rayon des centres de cases,
afin de ne pas conserver leurs coins en escalier. Les murs gardent leurs occlusions.
Le champ de perception du moteur, la grille et les informations connues restent les
mêmes. Les objets, acteurs, marqueurs et effets sont dessinés au-dessus du masque.
La limite de portée des capteurs est également dessinée en arcs continus, interrompus
par la visibilité réelle. Les règles historiques non euclidiennes restent distinctes.

## Direction artistique à choisir

`directions-artistiques-v2.png` est une maquette générée avec imagegen, pas une capture
du jeu ni une promesse d'assets déjà disponibles. Les prompts exacts sont dans `PROMPTS.md`.

- A : une carte composée de caractères, avec murs continus, objets composés sur plusieurs
  cases, contrastes sobres et couleurs pour les éléments utiles. Moins de production
  d'images, mais les objets restent abstraits.
- B : des sprites en pixel art sombre, vus verticalement de dessus, avec silhouettes
  reconnaissables et surfaces usées. C'est la direction qui répond le mieux au besoin
  de détails, mais elle exige un catalogue cohérent de vrais éléments graphiques.

Pour rendre les zones reconnaissables, il faut organiser les bâtiments par fonction
et leurs quartiers : logements, commerces, clinique, stockage, serveurs ou armurerie.
Leur identité doit venir de leur implantation, de leurs sols, de leurs objets et de
leur éclairage, avec une palette commune. Les propositions ne créent pas encore de
services marchands ou de nouveaux quartiers dans le générateur. Le prochain travail
graphique sera un petit ensemble pilote à valider avant l'intégration au jeu.

Les effets lumineux actuels restent la base : des décors calmes et sombres permettront
au feu, aux arcs électriques et aux impacts de ressortir sans recouvrir les acteurs.

## Vérification

- Compilation debug et release réussie ; vérification de tous les targets réussie.
- Formatage et `git diff --check` réussis.
- 26 tests ciblés réussis : récompenses à la mort, ancien transfert d'arme,
  reprise, ramassage/commerce, champ de vision, cercle, effets, ville et expédition.
- Une restauration de la copie de la sauvegarde utilisateur 140 réussie.
- Trois tests supplémentaires réussis : maintien de plusieurs statuts, feu suivant
  son porteur sans fausse traînée et disparition des marqueurs à l'expiration ou hors
  de la visibilité. Total : 29 tests ciblés et une restauration vérifiée.
- 600 morts réelles d'ennemis portant le même fusil : 127 modèles, 18 familles,
  niveaux 1 à 6 et qualités de 0 à 6 bonus, sans répétition immédiate.
- Captures natives inspectées : cercle dégagé, inventaire de dix récompenses,
  laboratoire, feu persistant et électricité persistante.
- La suite complète n'a pas été relancée ; les anciens échecs de tests d'empreinte
  documentés dans les retours précédents ne sont pas déclarés corrigés.

Le diagnostic détaillé des effets dépassait la pile du binaire debug Windows.
`build.rs` réserve maintenant 8 MiB pour ce binaire, avec le commit initial inchangé
de 4 KiB ; le binaire release conserve 1 MiB. Les captures d'effets passent.
Le réglage concerne la réserve virtuelle, selon la
[documentation Microsoft de /STACK](https://learn.microsoft.com/en-us/cpp/build/reference/stack-stack-allocations?view=msvc-170).

## Captures

- `vision-finale/cold-start.png` : carte de test dégagée, cercle lissé et capteurs.
- `butin/cold-start.png` : inventaire obtenu par dix morts réelles et ramassages.
- `effets-feu-final/cold-start.png` : feu persistant.
- `effets-electricite/cold-start.png` : champ électrique persistant.
- `directions-artistiques-v2.png` : comparaison artistique à valider.
