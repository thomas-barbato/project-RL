# CIVILS-U01-08 — civils morts et traces de violence

Le 19 octobre 2026, le joueur répond « Valider et intégrer le lot », puis
confirme son accord. Les dessins et leurs placements sont intégrés au jeu
normal. L'accord exact, les empreintes et les placements actifs sont conservés
dans `acceptance.json`. La demande porte sur davantage de cadavres civils dans
la rue et les maisons, différentes morts, du sang et du gore.

## Lot proposé

Cinq dessins de civils adultes : abattu (CIV-U01-04), lacérée (05), écrasé (06),
démembré (07) et carbonisé (08). Chaque corps occupe une toile de 80 × 48 px
avec marges transparentes ; la silhouette est proportionnée au joueur de 64 px.
Deux décors de sang séparés complètent le lot : flaque (SANG-U01-04, 64 × 48 px)
et traînée (05, 96 × 32 px). Les dessins sont entiers et isolés, sans découpe
d'une illustration de carte ni morceau de sol emporté avec la silhouette.

La variante CIV-U01-09 n'a pas produit d'image : le service de génération a
bloqué sa sortie. Elle est absente de la planche, des placements et du jeu.
Son identifiant n'est pas réattribué. Aucun essai de contournement n'est fait.

Les cinq corps sont répartis en **18 placements**, dont cinq dans les trois
intérieurs visitables. Dix-neuf décors de sang sont associés aux scènes :
fuite vers la prison, véhicule incendié, façades détruites, locaux et logements.
Les orientations carrées varient sans étirer les silhouettes. La position
exacte, la couche et le dessin de chaque placement figurent dans `proposal.json`.

## Limites et revue

Il s'agit de décors statiques au sol, sous les acteurs et les toits. Aucun
ennemi, combat, butin, quête ou collision n'est ajouté. Les passages et les
lumières restent ceux du jeu ; le sol validé PAV-U01-06 est conservé. Le
masquage des intérieurs par les toits et leur révélation par les portes sont
ceux du moteur, sans simulation dans la page HTML.

Le mode debug initial a servi à comparer la rue normale et la proposition à
positions et horloge identiques. Ce chargement spécial a été retiré après
validation. Les 37 nouveaux décors sont désormais chargés par le chemin normal
de la rue depuis `assets/maps/prison-street-expanded-01/decor/civilians-08/`.
Les 17 PNG natifs nécessaires sont des copies exactes des exports acceptés.
Le jeu ne dépend d'aucun dossier temporaire pour ce lot.

Les décors intérieurs portent une référence optionnelle à leur toit
(`interior_roof`). Ils ne sont dessinés que lorsque ce toit est masqué selon
la règle déjà validée de porte/présence du joueur. Cela évite qu'une légère
transparence de l'image du toit laisse passer un corps ou du sang. Les décors
actuels ne portent pas ce champ et conservent leur comportement.

## Sources et reproduction

Sources complètes et exports natifs :
`assets/source-art/first-sector-review-01/u01-civilians-r08/`.
Les images ont été produites avec le **mode intégré d'imagegen**. `prompts.json`
et `generation-results.json` conservent les prompts exacts, les sorties et le
statut de chaque dessin. Les copies sous cette revue conservent ce dossier.

Pour vérifier l'intégration :

1. `cargo build --locked`.
2. `target/debug/project-rl.exe --ui-cold-street-civilians-integration artifacts/street-defects-review-01/engine09`.
3. `python tools/reviews/check_street_civilians_integration.py`.
4. `python tools/reviews/show_street_civilians_review.py` remet à jour la page validée.

Le générateur `build_street_civilians_review.py` reste l'outil de conditionnement
des dessins sources. Les captures de proposition déjà approuvées restent dans
engine08 ; elles ne doivent pas être remplacées par les contrôles normaux.
`previous-street.json` conserve le manifeste exact avant intégration pour
vérifier que seul le lot demandé a été ajouté. `integration-checks.json`
consigne la comparaison entre la revue et le rendu normal, ainsi que les
contrôles de données et de circulation.

L'aperçu courant est réutilisé :
`artifacts/street-defects-review-01/index.html`.
La planche montre les sept nouveaux dessins à taille native, avec le corps
civil déjà accepté comme comparaison. La carte entière et les vues jouables
sont des captures du moteur avec ses vraies lumières et sa vision de six cases.

## Contrôle final et audit demandé

L'intégration normale reproduit exactement les dix vues acceptées et la carte
entière : zéro pixel différent dans les zones de jeu comparées. Les douze
tests ciblés `cargo test --locked --bin project-rl prison_street` passent ; la
compilation debug et `cargo check --locked --release` réussissent. Ces résultats
portent sur ce lot et ne prétendent pas valider l'ensemble des tests du projet.

L'audit de qualité demandé ensuite est dans
`../art-quality-audit-01/AUDIT.md`. Les règles observées sur les corps acceptés
sont ajoutées à `docs/DIRECTION_ARTISTIQUE_TILES.md` et rappelées dans
`docs/STYLE_VISUEL_REFERENCE.md`. Les personnages et équipements restent
inchangés pendant cette comparaison.

Les onze captures normales engine09, redondantes avec les références engine08
conservées, ont été supprimées après comparaison : 29 906 851 octets libérés.
Les deux vues natives utiles, les contrôles et les empreintes sont conservés ;
`cleanup.json` détaille ce rangement. Les étapes ci-dessus régénèrent les
captures de contrôle si nécessaire. Les sources et `artifacts/map-editor/`
ne sont pas touchés.
