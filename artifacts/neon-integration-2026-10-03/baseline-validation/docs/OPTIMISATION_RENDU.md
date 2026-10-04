# Optimisation du rendu — 28 septembre 2026

Cette passe traite les ouvertures d'interfaces, les calculs répétés du rendu et
le rythme des flammes. Elle ne modifie ni le temps de simulation, ni les dégâts,
ni la génération, ni les formats des sauvegardes.

## Corrections

- Les cases visibles fournissent leurs données d'affichage une fois par image.
  La carte, les badges, le capteur et l'inspection réutilisent ce résultat.
  Le résultat est reconstruit à chaque image : aucun état périmé ne survit à une
  commande, à un déplacement ou à un changement de perception.
- Les mesures de texte sont mémorisées par texte, police, taille et échelle.
  Le cache est borné à 8 192 entrées de 1 024 octets maximum par chaîne et est
  invalidé lors d'un changement de densité d'écran. Les caractères manquants
  d'une chaîne sont préparés ensemble avant le dessin : Macroquad ne recharge
  plus son atlas séparément pour chacun de ces caractères.
- L'accueil, ses options et la création de personnage ont un fond opaque :
  le monde et son HUD ne sont plus calculés derrière eux. Les interfaces
  transparentes continuent à montrer et à animer le monde.
- Les pixels adjacents de même couleur des effets élémentaires sont regroupés
  en bandes. Les silhouettes, palettes et zones atténuées sous les acteurs
  sont conservées ; un test reconstruit chaque pixel à partir de ces bandes.
- Une flamme utilise une phase de mouvement indépendante de la courte durée
  de son flash. Une flamme persistante n'est plus recouverte par une deuxième
  flamme transitoire. Les jets directionnels et les explosions distinctes
  conservent leur retour visuel.
- Les animations transitoires terminées sont retirées lors de la mise à jour,
  même sans nouvelle attaque. La pose fixe de 350 ms des animations réduites
  reste disponible jusqu'à son échéance.

## Mesures locales

Comparaison du même exécutable de développement, sans optimisation Rust, en
1280 × 800, sur les mêmes scènes et dans le même ordre. Chaque scène compte
120 images ; les dix premières sont exclues des statistiques du régime établi.
Le temps CPU inclut le traitement d'une entrée vide et la préparation du rendu.
Il ne représente ni la latence clavier complète, ni un nombre garanti de FPS.
Le rapport JSON conserve séparément le temps jusqu'au retour de `next_frame`.

| Scène | CPU médian avant | CPU médian après | Réduction |
|---|---:|---:|---:|
| Monde | 10,94 ms | 7,94 ms | 27 % |
| Pause | 12,74 ms | 8,44 ms | 34 % |
| Inventaire | 16,46 ms | 11,16 ms | 32 % |
| Compétences | 15,30 ms | 9,85 ms | 36 % |
| Aide | 22,24 ms | 8,64 ms | 61 % |
| Accueil | 11,23 ms | 1,01 ms | 91 % |
| Feu au sol | 13,65 ms | 9,40 ms | 31 % |

Première image de l'inventaire : 65,39 → 42,16 ms ; des compétences :
42,18 → 24,75 ms ; de l'aide : 27,96 → 24,15 ms.
Les ouvertures à froid restent donc une piste de travail, malgré les gains.
Ces mesures sont un échantillon local, pas une garantie sur toutes les machines,
sur une longue partie ou sur une compilation `release`.

Les rapports retenus sont dans
`artifacts/optimisation-2026-09-28/baseline-timing-only/timings.json` et
`artifacts/optimisation-2026-09-28/final-timing-only/timings.json`.
Ils ne réalisent aucune capture d'écran : une lecture du framebuffer entre
deux scènes peut perturber la première image de la suivante. Les séries
exploratoires `before`, `after`, `after-final` et `verified` ne sont donc pas
utilisées pour cette comparaison. Les captures à inspecter sont dans les
dossiers `check-*` et `baseline-*-960` / `baseline-*-1920`.

## Reproduire

```powershell
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-performance artifacts\nouvelle-mesure
```

Ce diagnostic travaille sur une partie déterministe isolée, sans charger ni
modifier la sauvegarde ou les réglages de l'utilisateur. Il vérifie aussi que
le rendu ne change pas l'empreinte de la simulation. Il exporte `timings.json`.
Les captures utilisent les commandes de contrôle visuel séparées ci-dessous.
Ne pas lancer compilation, tests lourds ou plusieurs diagnostics simultanément
pendant la comparaison chronométrée.

Contrôles visuels complémentaires : `--ui-cold-main-960`,
`--ui-cold-inventory-wide`, `--ui-cold-skills-largeui`,
`--ui-cold-lab-fx-ground-fire` et `--ui-cold-lab-fx-ground-fire-reduced`.

## Validation

- `cargo check --locked --all-targets`, formatage et vérification du diff : OK.
- Suite complète : 777 tests moteur réussis ; 436 tests client réussis,
  14 ignorés et 2 échecs.
- Les six nouveaux tests du rendu passent : réutilisation des cases visibles,
  absence de double flamme, cadence transitoire/persistante, conservation des
  pixels et de leur opacité, cache de texte borné et invalidation de densité.
- Les deux échecs sont reproduits sur la copie d'avant optimisation :
  `combat_balance_surface_roles_remain_distinct_with_starting_white_weapons`
  et `combat_balance_white_equipment_matrix_matches_real_impacts`.
  Les deux scénarios attendent une attaque acceptée avec zéro munition
  (`InsufficientMatter { required: 1, available: 0 }`). Ces essais
  d'équilibrage préexistants n'ont pas été modifiés dans cette passe.
- Captures inspectées : accueil 960 × 540, inventaire 1920 × 1080,
  compétences 1280 × 800 à 150 %, feu animé et animations réduites.
- Accueil et inventaire également comparés aux captures de la version
  d'avant optimisation ; disposition, textes et dessins conservés.
- Exécutable de développement reconstruit ; les 15 tests du lecteur d'effets
  repassent après le calcul préalable des échéances d'animation.

## Périmètre restant

Cette première passe ne profile pas les gros tours d'IA, les voyages, la
génération de zones ni l'écriture des points de récupération. La
[deuxième passe](OPTIMISATION_ACTIONS.md) mesure ces traitements sur un parcours
déterministe et corrige notamment les calculs de perception des tirs de réaction.
Une campagne longue et dense reste à mesurer. Les caches de rendu n'altèrent
pas les règles ni le comportement de la simulation.
