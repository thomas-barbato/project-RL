# FIREARM02-r01 — fusils légèrement agrandis

Suivi : la marche latérale a ensuite été corrigée par **LATERAL01**. Les
anciennes comparaisons partageaient un chemin d'image fixe et ne validaient
pas la variation des phases. Voir `LATERAL01-VALIDATION.md` pour les contrôles
de mouvement qui les complètent et remplacent cette conclusion.

## Comparatif initial, avant validation

Le joueur précise que son problème de visibilité concerne les fusils d'assaut,
à pompe et similaires, puis précise « un tout petit peu trop petit ».
L'ajustement 02 agrandit légèrement ces fusils par rapport au candidat 01.
Il conserve les points de prise, le placement, les angles, les figures complètes
et l'éclairage Discret. Les armes lourdes gardent leur comparaison séparée.

Dimensions opaques du dessin source, avant l'échelle propre au porteur :

- W002 : 41 × 17 → 44 × 19 px.
- W007 : 42 × 21 → 44 × 23 px.
- W035 : 42 × 14 → 44 × 15 px.
- W057 : 34 × 12 → 36 × 13 px.

Les modifications finales représentent quelques pixels selon la classe et
l'orientation. Les deux prises et leur rectangle d'alpha restent conservés ;
les prolongements sont préparés avant affichage, puis le compositeur habituel
applique l'angle, l'échelle et les occultations. Le contraste du métal et
l'ombre de séparation restent ceux du précédent candidat.

## Contrôles

- Construction Debug et `cargo check --locked --release --bin project-rl` : réussis.
- Diagnostic natif `--ui-cold-rifle-size .../rifles-r01/current` : réussi.
- 224 contrôles de source, soit 32 modèles de fusils × sept couleurs :
  prises conservées, gabarit supérieur ou égal au précédent essai et source
  normale identique après sortie de la portée de revue.
- 203 comparaisons des autres modèles/couleurs : identiques au rendu source
  normal. Aucun nouveau réglage n'est appliqué aux armes lourdes.
- 896 planches natives : quatre fusils sur quatre classes en armure lourde ;
  fusil d'assaut et fusil à pompe également en armures légère et très lourde
  sur chaque classe. Quatre directions, six phases de marche et vrai repos.
- Même position, même instant de lumière et aucun changement du décor hors
  de la région du joueur dans chaque paire. Matériau LIGHT01 Discret conservé.
- 32 retours au repos après une phase de marche : identiques.
- 504 panneaux de référence communs à FIREARM01 et FIREARM02 : pixels identiques
  entre le précédent candidat à droite dans FIREARM01 et la référence à gauche
  dans FIREARM02, hors bande de légende.
- Navigateur : Arcaniste léger avec fusil à pompe de dos, marche ralentie puis
  retour au repos ; Métamorphe avec Neral E12 de face. Les sélections, le cycle
  et la navigation passent sans erreur ni avertissement console. Capture :
  `rifles-r01/current/browser-rifle-size.png`.
- Formatage Rust, Python, JavaScript intégré, JSON et `git diff --check` : réussis.
- L'empreinte de l'exécutable Release reste celle de LIGHT01 intégré.

Les détails sont dans `rifles-r01/current/checks.json` sous l'aperçu courant.
Les légendes occupent uniquement la bande vide au-dessus des captures. Le
candidat est limité au diagnostic Debug, en attente du choix visuel ; le jeu
normal et son exécutable Release ne sont pas remplacés par cette revue.
Aucune campagne sauvegardée n'est chargée. La suite complète n'a pas été relancée.

## Intégration normale après accord

Le 1er novembre 2026, le joueur répond « ok bon je valide ». L'accord est figé
dans `FIREARM02-ACCEPTANCE.json` et le réglage 02 est intégré au rendu normal
Debug et Release des fusils du joueur, ainsi qu'à leur représentation portée
dans les portraits. Les armes lourdes de FIREARM01 ne sont pas activées.

La préparation validée devient un module de rendu normal, appliqué aux mêmes
sources après coloration et avant composition. Les prises proviennent des
liaisons déjà utilisées par chaque arme. Le compositeur conserve les angles,
les mains, les protections de tête incluses dans la tenue et l'ordre de profondeur.
Les PNJ qui réutilisent un montage de joueur gardent leur source d'origine.
Les clés de cache séparent ce montage, le réglage validé et les comparatifs
Debug ; une préparation verticale différée conserve son profil explicite.
Les capacités des caches existants restent bornées. Aucun dessin sous
`artifacts/` n'est utilisé par le programme.

Contrôles après intégration :

- `cargo check --locked --bin project-rl` et construction Debug : réussis.
- 427 sources normales comparées au profil explicite validé : identiques,
  dont 224 variantes de fusils et 203 autres modèles/couleurs inchangés.
- 896 comparaisons entre le vrai dessin normal et le candidat 02 : identiques.
- Les 896 planches recapturées sont également identiques aux planches montrées
  avant l'accord, hors de la bande vide des légendes. L'intégration conserve
  donc les pixels acceptés, et pas seulement les mêmes paramètres.
- 32 retours au repos après la marche : identiques.
- 18 comparaisons de PNJ, PNJ armés de fusils, monstre et objets au sol :
  identiques avec le chemin de source d'origine.
- Huit portraits (quatre classes × fusil d'assaut/fusil à pompe) : le nouveau
  rendu diffère du précédent, correspond au profil 02 et reste identique après
  utilisation du profil d'origine ; aucun mélange des caches n'est constaté.
- Construction Release par le vrai `Jouer.cmd` : réussie. Le test depuis un
  autre dossier transmet `--verify-assets`, passe sur les données empaquetées
  existantes et retourne l'erreur attendue pour des données absentes. Le
  manifeste et les quatre fichiers JSON utilisateur restent identiques.

Preuves : `rifles-r01/integration/checks.json`, `accepted-parity.json` et
`launcher/launcher-tests.json` dans l'aperçu courant. Une planche d'intégration
est conservée ; les 895 sorties redondantes réutilisent les captures courantes.
Les contrôles de scripts, de formatage et de différences sont ciblés. La suite
complète n'a pas été relancée et aucune campagne sauvegardée n'a été chargée.
