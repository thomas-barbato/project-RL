# HELM06 — retour aux tenues complètes

Décision du joueur du 31 octobre 2026 : abandonner les casques indépendants
et conserver le visuel des armures. Règle courante : `HELM06-ARMOR-ONLY.json`.

Le rendu du joueur ignore le seul emplacement de tête pour sélectionner sa
tenue. Les images originales des tenues restent intactes, avec leurs têtes,
capuches ou protections incluses. Cette sélection vaut pour le repos frontal,
les quatre directions de marche, les armes simples ou doubles, le portrait
« Visée » et le schéma du mode Terminal. Les autres emplacements, objets,
icônes actuelles, bonus et sauvegardes ne sont pas modifiés. Les PNJ et monstres
gardent leur rendu.

Les modules expérimentaux `helmet_fit_review.rs` et `helmet_worn_review.rs`
ne sont plus compilés. Les anciennes commandes de revue `--ui-cold-helmet-*`
annoncent la clôture du chantier, sans lancer les essais. La page courante
retire les comparatifs et commandes de casques ; elle conserve les autres
reprises d’animation et d’armes avec leurs accords séparés. Les dessins sources,
scripts et décisions historiques sont conservés, sans devenir des références
à intégrer.

## Vérifications exécutées

- `cargo test --locked --bin project-rl armor_only -- --nocapture` : 4 réussites.
  Vérification de l’emplacement visuel seul, des clés des cinq profils et cinq
  degrés d’armure pour les six modèles de casque ; comparaison des pixels
  frontaux, du portrait et des 13 phases verticales ainsi que des deux cycles
  latéraux de six phases sur les quatre personnages courants et cinq tenues.
  La configuration à deux armes actives est également comparée.
- `cargo test --locked --bin project-rl animation_polish_review:: -- --nocapture` :
  6 réussites. Les reprises distinctes des armes, leurs prises et les caches
  du rendu normal conservent leurs contrôles.
- `cargo build --locked --bin project-rl` : réussi, sans avertissement.
- `cargo check --locked --release --bin project-rl` : réussi.
- Après « tu peux integrer ceci que je test en live . :) »,
  `cargo build --locked --release --bin project-rl` réussit (2 min 25 s).
  L’exécutable normal est lancé sans argument de revue ; son menu principal
  est vérifié visuellement puis laissé ouvert pour le test du joueur.
  Le premier lancement via le contrôleur d’application utilisait un chemin
  de projet en minuscules et ne trouvait pas le paquet core. Ce processus
  échoué est fermé ; le lancement vérifié utilise le dossier exact du projet
  comme répertoire courant et comme `PROJECT_RL_DATA_DIR`, limité à ce processus.
  Aucun réglage système ni lanceur n’est modifié. `Jouer.cmd` reste le lanceur
  habituel depuis le dossier du projet.
- `project-rl.exe --ui-cold-animation-polish artifacts/animation-polish-01/current` :
  exécution graphique réussie, 20 configurations sans casque indépendant,
  140 captures natives, six vues du camp, la comparaison des objets au sol
  et les sept images de synthèse renouvelées.
- L’aperçu des cinq tenues dans les quatre directions est extrait à l’identique
  des anciennes lignes témoins sans casque du moteur : 140 planches, quatre
  personnages par planche. Cette galerie dérivée est distincte des captures
  graphiques nouvelles ci-dessus ; ses figures n’ont pas été redessinées.
- Python, JavaScript intégré, JSON et formatage Rust vérifiés. `git diff --check`
  réussi. Les sorties de revue ne constituent aucune dépendance du jeu.
- Aperçu vérifié dans le navigateur : sélection d’armure, profil, repos frontal,
  zoom entier, absence des sections de casques et absence d’erreur console.
  Capture : `artifacts/animation-polish-01/armor-only/current/browser-armor-only.png`.

Ces vérifications sont ciblées ; la suite complète n’a pas été relancée.
L’activation de HELM06 dans le rendu normal n’active pas les autres propositions
de la revue ANIM-POLISH01.
