# VERT18-C12 — armes à deux mains des douze tenues intermédiaires

Proposition du 30 octobre 2026, puis **validée visuellement** par « oui on continue ».
L'état d'intégration au jeu normal est consigné dans `integration-C12.json`.
Le joueur demande de vérifier les armes à deux mains, absentes de C10.
Les huit tenues de base/lourdes déjà validées C06 restent inchangées.

## Ce qui est présenté

L'aperçu `grips-C12/apercu.html` propose les 74 modèles partagés à deux mains
sur les douze tenues C10 : armes à feu, haches à prises écartées et lames à
prises serrées. Sept couleurs d'arme et sept états de tête sont sélectionnables.
Chaque famille a six phases de face, six de dos et un repos frontal du même
cycle. Les détails de prise sont explicitement agrandis ; les figures restent
en native64. Les références du moteur et du vrai camp sont accessibles.

Imagegen intégré (`builtin_imagegen`) a dessiné 36 feuilles de figures entières.
Une feuille a été reprise en r02 pour séparer correctement les corps, sans
couper de pixel structurel. Prompts et provenance : `prompts-C12.json`,
`generation-C12/`. Sources, figures et compositions :
`assets/source-art/vertical-walk-review-18/grips-C12/`.

## Prises et cohérence

Les ouvertures de prise sont examinées sur les doigts réellement dessinés,
et consignées dans `conditioned/grip-openings-front.json`. Les deux attaches
suivent ensemble le mouvement, en gardant leur écart et leur orientation.
Le suivi déplace les attaches et leurs masques ; il ne déforme aucun membre.
Corps et mains sont des masques complémentaires du même dessin entier.

Les gabarits suivent C10, avec une échelle uniforme fixe par rangée de source,
sans zoom par phase. Les têtes canoniques C10 sont partagées entre familles.
Les armes, leurs palettes, leurs échelles par classe et leurs poignées/appuis
restent ceux de C06. De dos, la figure entière masque l'arme portée devant elle.

## Vérifications

`ANALYSE-C12.json` et `native-checks-C12.json` consignent :

- 888 configurations tenue/modèle, 12 432 contacts vérifiés sans échec,
  échelle et orientation stables dans chaque cycle ; 518 alphas de palettes
  contrôlés, sans changement des dessins d'armes ni des règles de jeu.
- 1 104 configurations dessinées dans le moteur : 14 352 poses, 4 416 retours
  frontaux et 72 planches. Les six casques sont vérifiés sur trois armes par tenue.
- Douze groupes dans le vrai camp : 1 872 observations, 144 retours frontaux,
  72 captures natives ; mêmes matériaux, lumière et gain en marche/au repos.
- Audit des 36 gabarits ; compositeur comparé au calcul scalaire C08 sur 42 cas
  (écart prémultiplié maximal 1/255, alpha 1/255) ; lectures/décodages hors dessin,
  textures sur le thread graphique, cache du diagnostic borné à 32 Mio.

Captures retenues : `grips-C12/engine-r02/` sous l'aperçu courant. L'aperçu prépare
les poses avant affichage et borne son cache d'images à 32 Mio. Aucune sauvegarde
chargée ni préférence écrite. Les banques antérieures et le jeu normal C09
sont conservés par leurs empreintes. L'accord C12 est enregistré dans
`acceptance-C12.json` ; les contrôles techniques complètent la revue humaine
des prises. Les autres configurations à une main restent hors de cet accord.
