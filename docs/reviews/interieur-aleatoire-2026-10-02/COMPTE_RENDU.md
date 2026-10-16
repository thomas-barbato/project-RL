# Intérieur mobile et décors urbains — 2 octobre 2026

La version de jeu installée utilise la génération 139.

## Résultat

- Une nouvelle graine est choisie au début de chaque création de personnage.
- L’intérieur avec les services et le départ du recyclage forment un groupe sur un bord aléatoire, avec une position variable le long de ce bord.
- Les coordonnées physiques des consoles, portes, personnages, matériaux, quêtes et services suivent le placement. Les accès régionaux restent sur leurs bords et les retours utilisent leurs positions effectives.
- L’extérieur conserve les règles de quartiers : avenues, rues secondaires, ruelles et cours. Certains îlots deviennent des parcs, boutiques, supérettes, cafés ou immeubles résidentiels.
- Douze glyphes supplémentaires montrent arbres de parc, bancs, jardinières, éclairages, poubelles, comptoirs, étagères, présentoirs, distributeurs, tables, enseignes et repères de transport.
- Le prologue garde son chemin principal vers l’intérieur. Le mobilier est exclu des ruelles étroites et des accès réservés.
- Les nouveaux magasins sont des décors. Les marchands et services déjà fonctionnels restent présents dans l’intérieur.
- Les révisions 136, 137 et 138 conservent leur propre génération. La révision 139 ajoute le nouvel habillage, sans modifier le format des sauvegardes.

## Contrôles réellement exécutés

- `cargo check --locked --all-targets` : réussi.
- `cargo build --locked --bin project-rl` : réussi.
- `cargo build --locked --release --bin project-rl` : réussi. Deux avertissements de compilation existants subsistent dans les diagnostics.
- Formatage Cargo et `git diff --check` : réussis.
- Dernier lot de **16 tests ciblés : 16 réussis** (`tests-cartes-final.log`).
- 48 graines : quatre bords, positions variables, conservation des terrains et protections de l’intérieur, liens des consoles et accès valides, présence des décors et commerces.
- Quatre bords : trajet réel du prologue, services et contacts déplacés, sauvegarde et reprise.
- Reprise des générations 136, 137, 138 et 139 avec conservation de la carte et du décor.
- Douze régions voisines : conservation des sites, rencontres, butins, installations et passages après ajout des parcs et du mobilier.
- Essai avec création normale, sortie vers le souterrain, récupération de butin et retour ; enquête et mise à jour du panneau à sa nouvelle position.
- Captures natives : palette des douze glyphes, plans sur quatre bords, écran de jeu avec coordonnées à droite.

La suite complète initiale avait **517 réussites, 40 échecs et 14 tests ignorés**. Une copie des sources antérieures au déplacement (`compat-baseline`) reproduisait 21 de ces échecs. Les coordonnées fixes des fixtures ont été adaptées, et les événements d’installation des nouveaux départs sont purgés avant le premier checkpoint. Le dernier lot ciblé ne constitue pas une nouvelle exécution de toute la suite : les anciens tests d’empreintes historiques et un scénario de commerce restent en échec, comme dans la comparaison de départ. Les logs sont conservés pour distinguer ces limites des tests de carte réussis.

## Aperçus

- `glyphes/cold-start.png` : les symboles du moteur terminal en taille agrandie.
- `ville-nord/cold-start.png`, `ville-est/cold-start.png`, `ville-sud/cold-start.png`, `ville-ouest/cold-start.png` : plans de diagnostic de la génération 139.
- `jeu/cold-start.png` : rendu du vrai HUD et du prologue, coordonnées X 82 / Y 45 à droite.

Les plans complets sont hors jeu. La partie normale conserve visibilité, exploration et mémoire. Le compteur `new_buildings` des JSON de diagnostic ne compte que les quatre catégories techniques historiques ; il exclut les nouveaux commerces, immeubles résidentiels et parcs.

## Essayer

Lancer `Essayer_expedition.cmd`, puis choisir **Nouvel essai d’expédition**. Une reprise conserve l’ancienne carte. Les sauvegardes normales et celles de l’essai restent séparées.

Les combats, leur équilibre et le rythme des longues expéditions n’ont pas été redéfinis par ce changement. La validation des passages et des sauvegardes ne suffit pas à valider l’équilibrage.

## Copie de comparaison conservée

Le contrôle automatique a refusé le retrait des jonctions du dossier `compat-baseline`, sans motif plus précis. Ce dossier de diagnostic a donc été conservé. Ses liens de ressources pointent vers le projet principal ; son README précise de ne pas les modifier depuis cette copie.
