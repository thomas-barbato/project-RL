# Vérifications U01 r05

19 octobre 2026 · candidat `MAP-U01-EXP-01-r05`, non activé en partie normale.

La fabrication conserve les sources complètes de 17 éléments individuels.
Le catalogue vérifie leurs marges de fabrication, la transparence des exports,
leurs dimensions et empreintes. Les images sont conditionnées sans déformation.
Les originaux et prompts de l'outil intégré imagegen sont conservés.

Le constructeur vérifie les 692 cases libres reliées, les portes et leurs
intérieurs, l'absence de sol derrière la prison, les quatre coins de l'enceinte,
l'absence de débris couvrant une lentille et les empreintes inchangées de la
carte active. Deux appareils sont supprimés ; six autres ont une puissance
nulle et une lentille sombre. Les 29 sources actives comprennent les intérieurs
et la voiture. La benne ne chevauche plus l'impact de la cour.

Vérifications exécutées avec succès :

- `cargo fmt --all -- --check`.
- `cargo build --locked`.
- `cargo test --locked --bin project-rl prison_street -- --test-threads=1` :
  huit tests réussis, après le dernier déplacement de la benne.
- `cargo check --locked --release`.
- `git -c core.safecrlf=false diff --check`.
- Mode isolé `--ui-cold-street-expansion` : 21 vues natives, trois animations
  de 24 images, rendu sans mutation de simulation, toits ouverts/fermés et
  retour réel vers le même camp. Rapport exact : `render-r05.json`.

Inspection visuelle : scène d'ensemble continue, départ et limite sud,
accessoires de cour, lentille de porte sud-est, coins et portail nord.
L'ancien montage de quatre coins sans séparateurs est supprimé. Le joueur
confirme que son refus concernait les coupures de ce montage. Les images de
la page sont soit des vues continues, soit des captures séparées et légendées.

La vue complète sans brouillard est une composition numérique des ressources
et lumières de la scène. Les gros plans annoncés comme vues du moteur sont
des captures GPU avec sa visibilité normale. Cette vérification technique ne
remplace pas la validation artistique explicite de la r05.
