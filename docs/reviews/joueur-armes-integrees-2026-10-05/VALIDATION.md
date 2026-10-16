# Variantes armées complètes — 5 octobre 2026

## Résultat intégré localement

Les 14 tenues de torse possèdent chacune trois dessins complets, soit **42 variantes armées** : une épée sur le côté, un fusil tenu à deux mains devant le corps et un pistolet levé. Les trois classes de base casquées ont aussi leurs neuf variantes armées. Les armes, bras et mains appartiennent directement à chaque dessin ; le client affiche une seule figure, sans assemblage d'arme ou calque de doigts.

Un torse équipé choisit la tenue entière, indépendamment de la classe. Les autres pièces gardent leurs slots et leurs effets. Les 17 figures sans arme restent disponibles pour les personnages désarmés. Les nouveaux noms validés des 14 tenues sont appliqués aux textes français et aux tableaux d'équipement ; la liste est dans [NOMS.md](NOMS.md).

La pose pistolet est produite et vérifiée dans le client. Le catalogue actuel contient 61 armes de tir à deux mains et 40 armes de mêlée, sans pistolet jouable ; cette étape n'ajoute pas d'objet au butin. Les variantes avec deux armes de mêlée ou deux pistolets restent prévues pour plus tard.

## Production et assets

Mode : **imagegen intégré**, puis cadrage, nettoyage de l'alpha et rangement en atlas avec Pillow. Le script ne dessine pas les armes ou les mains et ne les extrait pas en calques séparés.

Sources finales sauvegardées dans ce dossier :

- [tenues-side.png](tenues-side.png) : 14 tenues avec épée.
- [tenues-two-handed.png](tenues-two-handed.png) : 14 tenues avec fusil.
- [tenues-raised.png](tenues-raised.png) : 14 tenues avec pistolet.
- [classes-armees.png](classes-armees.png) : neuf variantes des trois classes.
- [PROMPTS.md](PROMPTS.md) : les quatre prompts exacts.

Le rendu charge seulement `assets/sprites/cyberpunk/player-poses-{unarmed,side,two-handed,raised}-{32,128}.png`, soit huit banques. `player-poses.json` conserve les correspondances et empreintes des sources. Les figures désarmées proviennent des sources de `artifacts/joueur-tenues-completes-2026-10-04`. Les anciens atlas et repères de prise restent des références archivées et ne sont plus chargés.

Reproduction : `python tools/art/build_player_outfits.py`. Contrôle : la même commande avec `--check`.

## Validation terminée

- **7 tests graphiques** passent : sélection de la tenue et de la catégorie, figures présentes aux deux résolutions, contours non coupés, cases inutilisées transparentes, variantes armées distinctes et ligne des bottes commune.
- **2 tests `player_art`** passent : changement de l'arme active et déplacements clavier/souris sans modification de la partie.
- **1 test des armures renforcées** passe : nouveaux noms, même protection, même malus d'esquive, même emplacement et mêmes effets.
- `cargo check --locked --all-targets`, `cargo build --locked` et `cargo build --locked --release` réussissent.
- `rustfmt --edition 2021 --check` sur les trois modules Rust modifiés et `git diff --check` passent.
- `build_player_outfits.py --check` et `build_cyberpunk_assets.py --check` confirment la reproductibilité des assets et des correspondances.
- Quatre captures natives en **1280 × 800**, sans erreur de sortie : [les 42 variantes](player-outfits-review/cold-start.png), [personnage et combats](combat-review/cold-start.png), [HUD en jeu](terminal-hud-live/cold-start.png), [inventaire équipé](terminal-inventory-live-equipped/cold-start.png). Les images ont été inspectées.

La suite complète du client et les 806 tests du moteur n'ont pas été relancés. Cette validation est ciblée sur le rendu, les noms et le choix des variantes. Les avertissements existants de compilation concernent `wall_joins`, un import d'arsenal et des méthodes du client en release.

## Préservation

`verify_preservation.py` passe ; les résultats sont dans [preservation.json](preservation.json).

- La fonction de fil de fer est identique à la version approuvée.
- Les pixels RGB du centre de l'inventaire, slots compris, et des jauges restent identiques à la capture du 4 octobre. Les noms du torse ont changé dans la liste et le panneau de détails.
- Les sources de l'inventaire et de la sélection des visuels ainsi que les PNG d'objets, d'icônes au sol et de PNJ sont identiques par empreinte.
- Les trois fichiers de traduction ne changent que 12 noms affichés ; les noms Cerbère et Chrysalide étaient déjà validés. Les autres textes sont identiques aux sauvegardes de début de travail.
- Aucun identifiant, profil d'équipement, règle de combat ou format de sauvegarde n'est changé.

Les deux anciens chemins `compat-baseline/content` et `baseline-project/content` sont des jonctions existantes vers le contenu principal. Leur structure est conservée ; les mêmes traductions y apparaissent par ce lien.

Travail local, sans commit ni publication.

## Diagnostic de rendu

Diagnostic debug `--ui-cold-performance` terminé après les compilations : 120 images sur chacune des dix scènes.

- `world` : première image CPU 356.36 ms ; médiane CPU 8.29 ms ; p95 CPU 12.73 ms.
- `inventory` : première image CPU 29.02 ms ; médiane CPU 11.26 ms ; p95 CPU 13.95 ms.

Données complètes : [performance/timings.json](performance/timings.json). Cette mesure du client debug ne constitue pas un benchmark contrôlé ni une mesure de FPS de la version release.
