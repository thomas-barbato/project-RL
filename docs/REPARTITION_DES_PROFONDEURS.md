# Rencontres et milieux — répartition 117

27 septembre 2026. Première application de la
[descente vers le numérique](DESCENTE_VERS_LE_NUMERIQUE.md).

## Constat et périmètre

Les espèces avaient déjà des habitats distincts, mais les poids des milieux
étaient constants dans toute leur plage de profondeur. Aux couches 5 et 6,
la sécurité mécanique conservait environ 43 % des tirages. À la couche 7,
les réseaux représentaient encore 62,5 %, contre 37,5 % pour le milieu altéré.
La descente changeait les milieux possibles, mais exprimait peu leur montée
ou leur recul progressif.

La génération 117 ajuste donc d'abord le choix du milieu, auquel restent liés
ses rencontres et son décor existants. Aucun ennemi n'est déplacé dans un habitat
incompatible. Les familles et espèces continuent d'être tirées ensuite, avec
leurs règles et leurs budgets propres. Il ne s'agit pas d'un nouveau bestiaire.

## Répartition d'essai intégrée

Pourcentages théoriques du tirage des milieux, hors villes imposées. Une province
regroupe plusieurs cartes voisines : ces valeurs ne sont pas des quotas de
créatures ou de régions garantis dans chaque partie.

| Couche | Maintenance | Production | Recherche | Sécurité | Réseaux | Milieu altéré |
|---|---:|---:|---:|---:|---:|---:|
| 1 | 65 % | 35 % | — | — | — | — |
| 2 | 30 % | 45 % | 25 % | — | — | — |
| 3 | — | 25 % | 45 % | 30 % | — | — |
| 4 | — | — | 25 % | 40 % | 35 % | — |
| 5 | — | — | — | 20 % | 55 % | 25 % |
| 6 | — | — | — | 10 % | 40 % | 50 % |
| 7 | — | — | — | — | 25 % | 75 % |

La surface conserve exactement ses tirages. Les organismes liés aux installations
et les machines de travail gardent les premières profondeurs ; les milieux de
recherche font transition vers les rencontres de réseau, puis les anomalies
deviennent majoritaires. Les réseaux subsistent tout au fond pour éviter un
remplacement uniforme. Ces valeurs sont une première calibration, pas une
promesse d'équilibrage d'une campagne entière.

## Ce qui ne change pas

- Les 17 identités intégrées, leurs noms, comportements et natures. Le Guetteur
  ne devient pas un robot ; le Spectre ne produit pas une deuxième récompense.
- Les bornes de profondeur et les terrains autorisés de chaque espèce.
- Les PV, dégâts, armures et l'ajustement de puissance par couche.
- Les budgets et effectifs par milieu, ainsi que la provenance du butin.
  Changer la fréquence des milieux peut néanmoins changer la composition des
  rencontres réellement croisées : elle reste à observer en partie.
- Les villes, leurs services, les passages et les graines propres aux régions.
- Les graphismes et annonces des attaques : aucun nouvel effet ou mécanisme
  de synchronisation n'est ajouté dans cette tranche.

## Données et compatibilité

Le champ optionnel `depth_weights` des biomes donne un poids positif pour chaque
profondeur, dans l'ordre de `minimum_depth` à `maximum_depth` inclus. Une liste
non vide doit couvrir cette plage bornée entièrement ; zéro et les listes
incomplètes sont refusés au chargement. En son absence, le champ historique
`weight` conserve son comportement. Les mods sans ce champ ne changent pas.

Les versions de génération 116 et antérieures retirent uniquement cette nouvelle
métadonnée avant calcul d'empreinte et génération. Les poids historiques restent
dans le contenu. Une reprise ne redistribue donc ni les lieux déjà visités, ni
les futures destinations d'une ancienne partie. La nouvelle répartition demande
une nouvelle partie en version 117.

## Vérification et suite

Vérifications ciblées réussies :

- 65 536 tirages de provinces sur 32 graines, de la surface à la couche 7 :
  proportions à moins de deux points des poids théoriques, cartes voisines
  cohérentes, graines locales inchangées et surface identique.
- Dix empreintes historiques capturées avant la modification, dont les versions
  15, 98 et 109 à 116, retrouvées exactement.
- Reprises 116 et 117 par snapshot et par rejeu de commande : même état et
  mêmes futures destinations.
- 28 destinations profondes réellement générées, couvrant les six milieux :
  entrée et retour immédiat sans perte de PV, identiques après reprise du snapshot.
- Suite globale `cargo test --locked --quiet` : 748 tests moteur et 391 tests
  client réussis, un test manuel ignoré, aucun échec. Suite client en 384 s.
- `cargo check --locked --all-targets`, `cargo fmt --all -- --check`,
  `git diff --check` et compilation native du jeu réussis.

Le dernier essai ne prouve pas la survie après un combat profond. Aucun contrôle
visuel en partie n'est revendiqué pour cette modification de répartition.

La suite visuelle est documentée séparément : [terrain local des Guetteurs et
transmissions visibles](GUETTEURS_DECOR_ET_SIGNAUX.md). Elle s'appuie sur le
comportement déjà intégré, sans redessiner les villes ni prétendre matérialiser
toute la direction artistique des profondeurs.
