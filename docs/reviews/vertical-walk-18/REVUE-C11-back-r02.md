# VERT18-C11-BACK-R02 — face conservée, dos repris

Le joueur valide C11-B pour la descente et la face, et demande encore une
correction des bras et des armes de dos. L'accord **partiel** est consigné dans
`decision-C11.json` ; il ne valide pas l'ancien dos ni une intégration normale.

La reprise conserve exactement les sept images de face/repos de chaque
configuration : **1 176 cellules RGBA identiques**, vérifiées après réouverture
des PNG. Le nouvel arrière a ensuite été validé, puis intégré avec C12.

Le dos reprend les figures **entières** de port vers l'avant préparées dans
la famille C12 à armes à feu, pour les mêmes tenues et les mêmes têtes C10.
Épaules, coudes et avant-bras sont vus de derrière, avec les mains devant le
corps. Les armes à une main restent deux objets indépendants ; aucune greffe
de bras ni arme peinte dans le corps. Figures figées sous
`assets/source-art/vertical-walk-review-18/knife-contact-C11/back-r02/rear-actors/`.

Chaque arme partagée est réfléchie selon sa main anatomique pour la vue arrière.
La lame garde ses 20° vers l'extérieur/bas et le pistolet ses 10° vers l'extérieur.
Le pistolet rejoint la vraie main portée vers l'avant, plutôt que l'ancien
repère de manchette. Le corps et les avant-bras passent devant les armes de dos ;
certaines poignées peuvent donc être cachées.

Deux études spécifiques de dos Imagegen intégré (`prompts-C11-back-r02.json`)
ont été écartées et ne sont consommées par aucun composite. Les poses retenues
proviennent des sources complètes C12 : `prompts-C12.json`, `generation-C12/`.

`ANALYSE-C11-back-r02.json` et `native-checks-C11-back-r02.json` consignent
192 configurations natives, 2 496 poses, 768 retours de face, 72 planches,
et quatre groupes dans le vrai camp (624 observations, 48 retours frontaux).
La face validée, les ressources normales C09 et les banques antérieures sont
préservées. Aucune sauvegarde chargée et aucune préférence écrite.

Le comparatif courant reste `knife-contact-C11/apercu.html` : A est l'ancien
dos de B, B sa reprise r02 ; les deux faces sont identiques. Le détail suit
la face ou le dos. Les armes à deux mains sont dans `grips-C12/apercu.html`.

La nouvelle vue arrière suit AGENTS.md : « Présenter les nouveaux candidats
numérotés à taille native avant leur intégration ». **Pas d'intégration normale
de cette reprise avant accord visuel.**

Le 30 octobre 2026, le joueur répond « ca me va pour le moment :), il faut aussi
faire les armes à deux mains :) ». Le dos B r02 est accepté pour les deux
configurations présentées sur les douze tenues. L'accord et les empreintes sont
dans `acceptance-C11-back-r02.json`. Les deux configurations sont intégrées au rendu normal en Debug et Release,
avec les prises C12 validées ensuite. Contrôles : `integration-C12.json`.
