# Bestiaire, drones et transformations — intégration du 6 octobre 2026

Le joueur a autorisé l'intégration de toute la sélection courante aux fichiers
du jeu, puis demandé une ombre sous tous les sprites. L'autorisation et les
empreintes des références acceptées sont figées dans
`integration-authorization.json`. Le script de packaging refuse une source ou
une tile différente de ces références tant qu'une nouvelle revue n'est pas enregistrée.

## Ressources intégrées

- **65 modèles et 130 PNG natifs exacts** : vivant et cadavre/épave pour chaque numéro.
- 32 nouveaux ennemis, dont quatre boss natifs de **64 × 64** (quatre cases en surface).
- 23 références d'acteurs existants, cinq drones et cinq transformations.
- Versions courantes du 09, des machines 21–24 et 42–43, des 57, 59 et 66.
- Le **60 est retiré**, avec une correspondance historique vers l'unique gardien 66.
- **Ombres de contact séparées** sous les vivants et les corps, y compris le drone volant.
- **Machines sans sang** ; créatures de chair et hybrides selon les dessins validés.

Les fichiers sont dans `assets/sprites/bestiary/` : copies natives dans `tiles/`,
quatre atlas de vivant/cadavre en 32 et 64 px, catalogue Rust et manifeste
avec noms, numéros, IDs graphiques, IDs de contenu existants et provenance.
Les noms nouveaux gardent leur statut provisoire. Les pixels validés n'ont pas
été repeints ni recolorés ; l'ombre est ajoutée au rendu.

## Rendu et persistance

`src/game_art_bestiary.rs` charge les quatre atlas une fois, en nearest.
Le rendu actif des **23 acteurs déjà présents** utilise leurs cadavres dessinés,
au lieu de faire pivoter leur sprite vivant. Les objets restent dessinés
après les corps dans `TerminalView`. Les machines existantes ne produisent
plus de sang sur les dégâts ou à la mort.

Les indices d'acteurs enregistrés dans les sauvegardes sont conservés,
ainsi que la structure du résidu de combat. Les corps gardent la persistance
déjà en place entre sauvegardes et changements de zone. Un ancien cadavre
mécanique enregistré avec du sang est affiché avec la nouvelle épave sèche.

Les modèles supplémentaires sont disponibles par
`game_art::Visual::Bestiary { number, corpse }`. Ils n'ajoutent **aucune espèce
active, statistique, apparition, collision, capacité de drone ni transformation
jouable**. Ces règles restent à définir. Les boss s'étendent visuellement sur
64 px en conservant le point de contact de la case qui les porte.

## Vérification

- Packaging reproductible : `python tools/art/build_bestiary_assets.py --check`.
- Banque d'armures, armes et acteurs antérieure inchangée :
  `python tools/art/build_crawl_assets.py --check`.
- `cargo check --locked`, compilation debug/release, formatage et contrôle du diff.
- **26 tests ciblés réussis** : chargement, IDs, corps existants, machines sèches,
  ombres, boss, dégâts/mort, sauvegarde et état de simulation.
- **817 tests moteur réussis**.
- Suite client : **620 réussis, 28 échecs, 15 ignorés**.
  Les 28 tests en échec ont également échoué sur la copie du code antérieur,
  avec le même contenu et les mêmes mods. Les journaux et la comparaison sont
  conservés ; les tests de cette intégration passent. Aucun correctif sur
  les autres systèmes n'a été inclus.

`baseline-project/` est une copie de contrôle : sources prises avant cette
intégration, contenu, mods et images d'origine. Son exécutable de comparaison
a un nom distinct pour éviter le verrou Windows du test courant. La comparaison
porte sur les tests et leurs échecs, pas sur l'identité binaire de compilation.

Les quatre vues suivantes viennent **du moteur actif** : taille native, zoom
entier, ombres et corps. Chaque scène vérifie que le rendu conserve les octets
de la simulation et ne fait pas croître les caches d'armes/objets existants.

- [01–16](C:/Users/User/Desktop/project-RL/artifacts/bestiaire-integration-2026-10-06/captures/bank-1/cold-start.png)
- [17–32, machines et boss](C:/Users/User/Desktop/project-RL/artifacts/bestiaire-integration-2026-10-06/captures/bank-2/cold-start.png)
- [33–55, corps et loots](C:/Users/User/Desktop/project-RL/artifacts/bestiaire-integration-2026-10-06/captures/bank-3/cold-start.png)
- [56–66, drones et transformations](C:/Users/User/Desktop/project-RL/artifacts/bestiaire-integration-2026-10-06/captures/bank-4/cold-start.png)

Les images validées et les autres modifications du workspace sont conservées.
`validation.json`, `baseline-comparison.json` et les journaux détaillent la portée
des contrôles. Aucune modification des fichiers de gameplay dans `content/`
ou des règles moteur n'a été nécessaire.
