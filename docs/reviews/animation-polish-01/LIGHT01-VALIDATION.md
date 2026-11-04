# LIGHT01-r01 — exposition du joueur

## Présentation initiale, avant le choix du joueur

Le joueur trouve la lumière du personnage trop forte, puis autorise la préparation
d’une réduction. Deux candidats numérotés sont présentés avant choix visuel :
01 « Discret » et 02 « Mat ». Le rendu normal reste celui déjà intégré.

L’essai est compilé uniquement en Debug et actif uniquement sous une portée
explicite du prototype. Les deux matériaux de comparaison sont distincts du
matériau normal. Les dessins, textures d’acteur et palettes ne sont pas modifiés.
Les candidats réduisent le relèvement global des couleurs, le gain des cartes
éclairées et le renforcement des contours. Ils ne créent pas de nouveau modèle
de reflets localisés ou de matériaux physiques.

Les captures emploient le rendu normal des figures complètes et le vrai décor
du camp, avec sa correction et son irradiance. Chaque candidat occupe exactement
la même position que la référence, au même instant d’éclairage. Les sources de
texture, matériaux et cycles utiles sont préparés avant leur capture. Les six
phases de marche sont conservées et la septième vue montre le vrai repos frontal.

## Contrôles exécutés

- `cargo check --locked --bin project-rl` et `cargo build --locked --bin project-rl` : réussis.
- `cargo check --locked --release --bin project-rl` : réussi. Les contrôles et
  matériaux de cet essai sont absents du rendu Release.
- `--ui-cold-player-exposure artifacts/animation-polish-01/player-light-r01/current` :
  exécution graphique réussie. Quatre classes, cinq tenues de base/armure, quatre
  directions et sept phases produisent 560 planches natives.
- 1 120 comparaisons de candidats avec le rendu actuel : luminance réduite et
  absence de modification du décor hors de la région occupée par le joueur.
- 60 comparaisons de pixels au retour au repos après un état de marche : identiques.
- Huit comparaisons de pixels sur PNJ, monstre, objet au sol et portrait joueur :
  identiques entre les trois réglages.
- Les audits de texture vérifient le matériau d’acteur et le gain correspondant
  pour chaque configuration de marche et de repos dans le camp.
- L’aperçu annote seulement la bande vide de 32 px au-dessus des figures ; les
  pixels du camp et du personnage sont vérifiés inchangés pendant la publication.
  Le navigateur recadre les panneaux et zoome par un facteur entier, sans filtre
  de luminosité. Le cycle complet est décodé avant lecture.
- Python, JavaScript intégré, JSON, formatage Rust et `git diff --check` : réussis.
- Dans le navigateur : sélection de la Traqueuse très lourde de dos, lecture de
  la variante 02, puis comparaison des trois expositions du Bastion lourd au
  repos. Aucun avertissement ni erreur console. Capture de revue :
  `browser-light-compare.png` dans le dossier des captures LIGHT01.

Journaux de contrôle : `player-light-r01/current/checks.json` dans l’aperçu.
La luminance enregistrée porte sur la région de comparaison, qui contient aussi
du sol ; elle ne représente pas un pourcentage de luminosité finale du personnage.
La suite complète n’a pas été relancée.

Les PNJ, monstres, icônes et portraits conservent le rendu existant. C03 et HELM06
gardent leur état d’intégration. Aucun programme Release, état de campagne ni
réglage utilisateur n’est remplacé par ce prototype.

## Intégration du choix 01, 1er novembre 2026

Après le comparatif, le joueur répond « Je pense que discret est mieux ».
L'accord est consigné dans `LIGHT01-ACCEPTANCE.json` ; le réglage 01 est désormais
actif dans le rendu normal Debug et Release du joueur humanoïde dans le monde.
L'ancienne exposition et le candidat 02 restent accessibles uniquement dans
le diagnostic Debug. Les portraits et les autres acteurs gardent leur rendu.

Le matériau commun reçoit les coefficients du joueur pendant toute la portée
de son dessin, au repos et pendant la marche. Cette portée est restaurée à la
sortie ; elle ne se propage pas aux autres figures. Un seul matériau est conservé
en cache et préparé sur le thread graphique au démarrage, avant affichage.
Aucune texture, figure ou phase d'animation n'est redessinée.

Contrôles après intégration :

- `cargo check --locked --bin project-rl` et construction Debug : réussis.
- 560 comparaisons de pixels entre le rendu normal sans forçage de diagnostic
  et le candidat 01 : identiques sur quatre classes, cinq tenues/armures,
  quatre orientations et sept phases.
- Les 560 planches des trois profils restent identiques au précédent aperçu
  accepté, hors de la bande de légende. Le passage des coefficients en paramètres
  du matériau n'altère pas les pixels de référence.
- 1 120 comparaisons confirment la réduction d'exposition et le décor inchangé ;
  les 60 retours au repos et les huit contrôles PNJ/monstre/objet/portrait passent.
- `--ui-cold-walk-lighting ... --expect-fixed` : 576 routes en lumière et sans
  lumière, aucune incohérence de matériau ou de retour au repos.
- Construction Release par le vrai `Jouer.cmd` : réussie. Le test du lanceur
  depuis un autre dossier vérifie la transmission de `--verify-assets` sur les
  données empaquetées existantes (code 0) et l'erreur attendue pour des données
  absentes (code 1). Les quatre JSON utilisateur et le manifeste sont inchangés.
  Ce mode exige un manifeste d'installation ; il ne vérifie pas directement
  les ressources libres du dépôt de développement.
- Formatage Rust ciblé, scripts de publication, JSON et `git diff --check` : réussis.

Preuves : `player-light-r01/integration/checks.json`, `preview-parity.json`,
`walk-lighting/checks.json` et `launcher/launcher-tests.json` sous l'aperçu.
La planche `light-3-3-front-6.png` est conservée comme capture d'intégration ;
les autres planches identiques réutilisent l'aperçu courant. Aucune campagne
sauvegardée n'a été chargée et la suite complète n'a pas été relancée.
