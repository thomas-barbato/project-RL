# ITEM-LOOT01-DROPSTUDY02 — butin de Diablo II classique

Étude des tables locales, complétée par le guide officiel et la reconstruction du moteur D2MOO. Aucun nouveau taux Project RL appliqué.

## Exemples calculés

Solo `/players 1`, table nommée fixée, armes et armures hors anneaux/amulettes. Les valeurs de boss sont des moyennes par mort, pas des probabilités par tirage.

| Source | Aucun objet | Au moins une arme/armure | Armes/armures moyennes | Tous objets moyens |
| --- | ---: | ---: | ---: | ---: |
| Monstre ordinaire, début acte I | 62.5000 % | 10.00 % | 0.100 | 0.375 |
| Lanceur de sorts, table acte I | 62.5000 % | 9.38 % | 0.094 | 0.375 |
| Tireur, table acte I | 62.5000 % | 11.88 % | 0.119 | 0.375 |
| Spectre, table acte I | 72.1154 % | 0.00 % | 0.000 | 0.279 |
| Champion, début acte I | 0.0000 % | 37.00 % | 0.370 | 3.000 |
| Monstre nommé, début acte I | 0.0000 % | 93.55 % | 0.935 | 5.000 |
| Monstre ordinaire, fin acte V Enfer | 62.5000 % | 10.00 % | 0.100 | 0.375 |
| Andariel, hors récompense de quête | 0.0147 % | 90.13 % | 1.947 | 4.918 |
| Mephisto, hors récompense de quête | 0.0008 % | 99.93 % | 4.363 | 5.454 |
| Diablo, hors récompense de quête | 0.0008 % | 99.93 % | 4.363 | 5.454 |
| Baal, hors récompense de quête | 0.0008 % | 99.93 % | 4.363 | 5.454 |
| Andariel, table de quête | 0.4590 % | 98.72 % | 3.233 | 3.743 |
| Mephisto, table de quête | 0.0021 % | 99.99 % | 5.025 | 5.315 |

## Lecture et limites

- Les monstres ordinaires de la table H2H de départ ont 100 tickets sans objet sur 160 ; les armes/armures occupent 16 tickets, soit 10 %.
- Le champion a un tirage principal avec 37 tickets d’armes/armures sur 100, puis des potions. Le monstre nommé utilise 58 tickets sur 62 pour son tirage principal, soit environ 93,55 %.
- Les boss ont plusieurs tirages avec un plafond de six objets ; tous ces objets ne sont pas des équipements.
- La qualité est un second calcul. Le facteur Magic=1024 des champions et monstres nommés garantit le test magique s’il est atteint après les qualités supérieures. Les facteurs ne sont pas des pourcentages de rareté.
- Les tables H2H de départ et de fin d’Enfer gardent ici 10 % d’armes/armures ; leurs catalogues de modèles sont différents.
- Le Magic Find intervient sur la qualité, le nombre virtuel de joueurs sur NoDrop. Le niveau et la source déterminent les modèles accessibles.
- Les chiffres ne sont pas un calculateur de chances pour un objet nommé, ni une règle universelle pour toutes les familles de monstres.

## Adaptation à Project RL

Conserver les trois qualités standard, modifié et rare. Étudier séparément la fréquence des équipements, le palier des bases et la qualité. Le craft reste reporté. Les taux propres à Project RL nécessitent ensuite une calibration sur les rencontres d’une expédition ; aucune transposition brute ni conversion automatique des uniques/ensembles en rares n’est validée.

Les distributions exactes, hypothèses, empreintes, contrôles et références sont dans `diablo2-drop-study-02.json`.
