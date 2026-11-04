# LATERAL02-r03 — foulée resserrée de la Traqueuse

Le 2 novembre 2026, retour sur r02 : « c'est mieux mais les jambes s'écartent
trop ». Ce retour demande un ajustement de la foulée ; il ne valide pas
l’intégration de r02. Le périmètre reste la Traqueuse en armure lourde
(`traqueuse-armor3`, corps 5, armure 101, tenue 13), sans arme. Le Bastion
reste inchangé, conformément à la précision précédente du joueur.

## Proposition 03

Édition par l’outil intégré `image_gen` de la planche r02 : petits pas plus
près du bassin, avec conservation des deux appuis alternés. Chaque pose
est une figure entière ; aucun membre n’est comprimé, greffé ou déplacé
séparément par le conditionnement. Une seule échelle commune est utilisée.
Les deux poses natives de repos de r02 sont réutilisées avec des pixels
strictement identiques. Le nouveau dessin porte donc sur douze poses de marche.

Sur les huit poses étendues (phases 0, 2, 3, 5 dans les deux directions),
l’enveloppe opaque du bas du corps passe de 30–35 px à 18–22 px de largeur,
soit une réduction médiane de 38,5 %. Cette mesure de pixels, limitée aux
lignes 60 à 82 de la toile, n’est pas une mesure anatomique ni un accord visuel.
Les proportions, détails et enchaînements restent à juger dans l’aperçu.

Sources et conditionnement :
`assets/source-art/animation-polish-01/lateral-identity-02/r03/`.
Consigne exacte : `LATERAL02-PROMPT-r03.md`.

## Vérifications

- Compilation Debug verrouillée et format Rust ciblé réussis.
- Capture native isolée `--ui-cold-lateral-compact` : 14 planches, six phases
  distinctes par direction, camp accepté et éclairage LIGHT01 Discret.
- Comparaison native 02/03 ; anciennes références face/dos et colonne 02
  conservées. Repos 02/03 identiques. Ressources préparées avant le dessin.
- Syntaxes Python et JavaScript vérifiées. Navigateur : gauche/droite,
  marche continue, pause, phase sélectionnée et mode avec repos disponibles.
  Le panneau courant affiche le nouveau candidat 03.

État : `awaiting_visual_review`, aucun nouveau dessin intégré au jeu normal.
Les anciennes captures et sources restent conservées pour cette revue.
Les règles de jeu, préférences, sauvegardes, armes, autres classes et animations
verticales ne sont pas modifiées. L’accord visuel sur cette révision reste
distinct de son éventuelle intégration.

Aperçu : `animation-polish-01/apercu.html?revision=LATERAL02-r03#identite-laterale`.
