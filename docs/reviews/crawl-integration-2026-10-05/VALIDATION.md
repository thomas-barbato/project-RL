# Validation de l'intégration native — 5 octobre 2026

## Résultat actif

113 modèles d'armes existants sont associés explicitement à 18 familles visibles.
Les armes réellement équipées sont dessinées pour chaque main ou pour le groupe
deux mains actif. Les 17 tenues et les 23 références existantes de bestiaire/NPC
sont intégrées. Le zombie est une image en réserve, sans ajout de contenu jouable.
Le fer de hache est à l'extérieur : source droite, couche retournée à gauche.

Le corps demeure une référence native de 30 px dans une case de 32 px. La
composition de 48 px permet le débordement de l'arme sans changer les collisions.
Le torse choisit la tenue complète. Le casque, les jambes et les couleurs de cette
tenue ne sont pas redessinés à chaque arme. Les autres slots gardent leurs effets
et leurs dessins distincts dans l'inventaire.

## Tests ciblés

| Commande / filtre du binaire | Résultat final |
| --- | --- |
| `game_art::` | 12 réussis |
| `weapon_hands_app::` | 5 réussis |
| `one_hand_firearms::` | 6 réussis |
| `combat_visuals_app::` | 3 réussis |
| `cyberpunk_capture::` | 3 réussis |

**29 tests ciblés réussis.** Les logs `tests-*.log` conservent les commandes
résultantes, noms des tests et durées. Les contrôles couvrent notamment :

- Toutes les armes du contenu core chargé disposent d'une couche portée ; tous
  les objets core chargés ont leur entrée d'illustration.
- Les 113 couches choisissent une famille non vide et une prise explicite.
- Tous les corps gardent 30 px ; les pixels fixes hors membres et occlusion
  d'arme restent identiques entre configurations.
- Le retournement échange uniquement la couche portée, et le fer de hache reste
  vers l'extérieur. Les armes distinctes de gauche/droite gardent leur famille.
- Toutes les palettes gardent l'alpha et les pixels protégés des 17 tenues.
- Les casques, gants et bottes ont réellement des dessins distincts, en plus
  d'indices distincts. Les 162 cases d'objet sont non vides.
- Le groupe actif se commute sans déséquiper ni dépenser un tour ; équipement,
  groupes, palettes et anciennes sauvegardes concernées restent compatibles.
- Les effets de combat/cadavres et leur persistance restent fonctionnels ; les
  captures et retours visuels ne changent pas la simulation.
- Les armes des NPC suivent leur équipement réel sur les six bases humaines
  compatibles, sans équiper artificiellement les autres créatures.

## Suite générale : résultat distinct

La commande `cargo test --locked --bin project-rl -- --test-threads=4` a terminé
avec **603 réussites, 28 échecs et 15 ignorés**, en 1081,36 s. Ce passage comportait
646 tests ; deux contrôles visuels supplémentaires ont ensuite été ajoutés et
réussissent dans la validation ciblée ci-dessus. La suite entière n'a pas été
relancée après ces ajouts.

Les échecs concernent les empreintes historiques de génération/loot, le commerce,
les cartes, certains parcours et des créations de partie refusées lorsqu'une
suspension existe. La liste exacte est dans `full-suite-summary.json`. Il n'y a
pas eu de passage complet avant modification permettant d'attribuer chaque
échec avec certitude ; ces échecs ne sont donc pas présentés comme tous
préexistants. Les fichiers de simulation, loot, instances et `content/core`
comparés au début de cette intégration sont inchangés. Les corrections de ces
échecs n'ont pas été mêlées à ce changement de rendu.

## Contrôles des assets et préservation

`build_crawl_assets.py --check` reproduit les atlases et le catalogue Rust octet
pour octet. Les anciens scripts de packaging délèguent au constructeur actif.
`asset-validation.json` vérifie tailles natives, alpha binaire et préservation
exacte des IDs, indices d'illustration, indices de couche et prises du catalogue
initial. Les 18 familles sont toutes utilisées.

`preservation-validation.json` compare 656 fichiers présents au début du travail :
aucun n'est supprimé ; le contenu core, la simulation, le loot et les données
d'instance comparés ne changent pas. Les modifications de rendu et de packaging
sont listées. Un changement extérieur des noms attendus dans
`src/armor_models_tests.rs` a été observé pendant le travail et conservé ; ce
fichier n'a pas été restauré ou réécrit par cette intégration.

`cargo fmt --check` et `git diff --check` réussissent. Les avertissements de
conversion LF/CRLF sont distincts d'une erreur de whitespace.

## Captures et caches

`galerie-integree.html` rassemble les captures réelles du renderer : armes
numérotées 01–18, tenues 01–17, sept palettes, bestiaire/NPC, configurations de
mains, monde, inventaire équipé et survol. Les planches montrent la taille native
et un zoom entier nearest. Les casques/gants/bottes de l'inventaire ont été
revérifiés après restauration de leurs dessins distincts.

Les cinq diagnostics `validated-crawl-*` comparent les octets du snapshot avant
et après rendu : état identique dans chaque scène. Ils comparent également les
créations de textures après un premier rendu et après les frames répétées :
**aucune nouvelle texture après cette initialisation**, y compris les planches
de variantes de couleur. Les caches plafonnent à 192 personnages et 256 objets.
Les données brutes sont dans `render-validation.json` et les
`captures/*/art-validation.json`.

Les résultats finaux de compilation et de mesure sont enregistrés dans les logs
et dans la section ci-dessous. La mesure debug porte sur dix scènes de 120 frames,
avec première frame séparée et empreinte de simulation vérifiée. Elle décrit
l'état final ; aucune amélioration de vitesse n'est affirmée sans comparaison
avant/après correspondante.

## Écran de mort et warning signalés ensuite

Cause reproduite : le cadre terminal du plateau était tracé après le bilan de
fin. Les panneaux et raccourcis de jeu avaient également priorité sur ses
entrées. Le bilan est désormais une branche dédiée du rendu ; les panneaux de
jeu ne peuvent plus le recouvrir ni recevoir les raccourcis derrière lui. Les
panneaux temporaires se ferment à la fin de partie. L'historique et les menus
restent accessibles au-dessus du bilan.

Cinq tests supplémentaires utilisent une mort réelle par attaque dans une petite
scène déterministe. Ils vérifient la fermeture des anciens panneaux, les
raccourcis bloqués, souris/clavier, historique, pause/options, redémarrage,
changement de profil et retour à l'accueil. Trois régressions existantes passent
aussi : bilan et historique, impossibilité de sauvegarder une partie terminée,
mémoire des murs hors vision. Cela porte la validation ciblée à **37 tests
réussis** au total. La suite générale précédente reste le passage distinct décrit
plus haut, sans relance complète après cette correction.

`wall_joins` n'est utilisé que par un contrôle de mémoire du terrain. Il est
maintenant compilé uniquement pour les tests. Le build debug final ne produit
plus le warning `wall_joins` (et ne produit aucun warning).

Captures finales dédiées : `captures/validated-death/cold-start.png` et
`captures/validated-death-history-small/cold-start.png`. La galerie ajoute le
bilan de mort. Les nouvelles armes, tenues et images de monstres restent actives.

## Compilation et mesure finales

Le build debug final réussit sans warning (`build-debug.log`). Le build release
final réussit en 1 min 43 s (`build-release.log`). Il conserve quatre autres
avertissements d'import/méthodes inutilisés ; `wall_joins` n'en fait plus partie.

Mesure debug isolée : `performance-isolee/timings.json`, dix scènes × 120 frames,
aucune compilation/test parallèle pendant la mesure. Médiane CPU du monde :
9,79 ms ; retour au monde : 8,38 ms ; inventaire : 11,57 ms. Première frame du
monde : 397,24 ms. L'empreinte de simulation demeure identique pour chaque scène.
Cette mesure précède le correctif de bilan, dont la nouvelle branche n'est pas
empruntée dans ces scènes actives. Les données brutes, p95 et coûts de présentation
sont conservés ; aucune accélération avant/après n'est revendiquée.
