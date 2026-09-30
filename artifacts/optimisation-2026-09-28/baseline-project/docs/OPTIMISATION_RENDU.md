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
| Monde | 10,47 ms | 7,63 ms | 27 % |
| Pause | 12,99 ms | 9,35 ms | 28 % |
| Inventaire | 17,29 ms | 11,94 ms | 31 % |
| Compétences | 15,96 ms | 10,52 ms | 34 % |
| Aide | 22,96 ms | 9,36 ms | 59 % |
| Accueil | 11,70 ms | 1,13 ms | 90 % |
| Feu au sol | 14,01 ms | 9,89 ms | 29 % |

Première image de l'inventaire : 73,62 → 44,66 ms ; des compétences :
39,79 → 24,99 ms. La première image de l'aide reste autour de 30–33 ms.
Les ouvertures à froid restent donc une piste de travail, malgré les gains.
Ces mesures sont un échantillon local, pas une garantie sur toutes les machines,
sur une longue partie ou sur une compilation `release`.

Les rapports initiaux sont dans
`artifacts/optimisation-2026-09-28/before/timings.json` et
`artifacts/optimisation-2026-09-28/after-final/timings.json`.
Le dossier `after` documente un essai de préchargement massif des polices rejeté
car il dégradait les ouvertures. Ses résultats ne décrivent pas le code retenu.
Les PNG de ces trois premières séries ne sont pas exploitables (lecture après
échange du framebuffer) ; les contrôles `check-*` et les séries avec capture
corrigée sont les preuves visuelles à utiliser.

## Reproduire

```powershell
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-performance artifacts\nouvelle-mesure
```

Ce diagnostic travaille sur une partie déterministe isolée, sans charger ni
modifier la sauvegarde ou les réglages de l'utilisateur. Il vérifie aussi que
le rendu ne change pas l'empreinte de la simulation. Il exporte `timings.json`
et une capture par scène ; les captures sont réalisées après les mesures.
Ne pas lancer compilation, tests lourds ou plusieurs diagnostics simultanément
pendant la comparaison chronométrée.

Contrôles visuels complémentaires : `--ui-cold-main-960`,
`--ui-cold-inventory-wide`, `--ui-cold-skills-largeui`,
`--ui-cold-lab-fx-ground-fire` et `--ui-cold-lab-fx-ground-fire-reduced`.

## Périmètre restant

Cette passe ne profile pas les gros tours d'IA, les voyages, la génération de
zones ni l'écriture des points de récupération. Il faudra les mesurer sur une
partie longue et dense avant de choisir d'autres optimisations. Les caches de
rendu n'altèrent pas leurs règles ni leur comportement.
