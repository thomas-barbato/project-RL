# ITEM-LOOT01-AFFIXSTUDY03 — bonus et ordre de conception

Date : 23 octobre 2026. Étude et proposition de méthode, sans modification des règles du jeu ni validation de nouvelles valeurs.

## Contexte donné par le joueur

- Les bonus actuels semblent trop élevés pour le niveau des objets.
- Les cartes et les monstres actuellement placés représentent moins de 2 % du jeu envisagé : ce contenu doit être traité comme le tout début, sans convertir ce pourcentage en niveau ou en durée.
- Le joueur envisage de construire les cartes, puis de placer les monstres avant d'équilibrer l'itemisation. L'ordre de travail proposé ci-dessous reste une recommandation.
- Le système de fabrication et la qualité artisanale restent reportés. Le périmètre envisagé conserve standard, modifié et rare (`scope-decision-02.json`).

## Constats vérifiés dans Project RL

`src/loot/equipment.rs`, `EquipmentLootCatalog::draw_monster_drop`, choisit parmi tous les modèles de forme humanoïde, sans paramètre de niveau, de zone ou de profondeur. Seul le modèle immédiatement précédent est écarté quand plusieurs modèles existent. Les bornes de profondeur employées par l'autre chemin `draw` ne sont pas consultées ici. Un modèle de haut palier et ses bonus peuvent donc apparaître dès les premiers ennemis éligibles.

`roll_properties` fixe le palier des bonus à celui du modèle (ou à la moitié supérieure de celui-ci pour les modèles à bonus réduits). Avec les comptes pondérés, le plafond est de six propriétés, même au premier palier si le catalogue du modèle le permet. Une propriété numérique distincte ne peut pas être sélectionnée deux fois et un effet spécial compte dans ce total.

`src/item/affix.rs` définit au premier palier :

| Famille de bonus | Plage actuelle |
| --- | ---: |
| Puissance, Coordination, Résilience, Perception, Traitement | +1 à +2 |
| Précision | +5 à +10 |
| Pénétration d'armure | +1 |
| PV maximum, énergie maximale | +5 à +15 |
| Dissipation | +1 par tour |

Ces paliers sont explicitement des paliers de puissance prototypes, pas des niveaux de personnage ou des conditions d'équipement. Ils ne suffisent donc pas à garantir une progression adaptée au début du jeu.

`src/game/equipment_bonuses.rs` additionne les bonus des emplacements actifs. Les caractéristiques augmentées participent ensuite aux statistiques dérivées ; les bonus directs de PV et d'énergie peuvent s'y ajouter. Avec les règles physiques par défaut (`src/stats/physical.rs`), +2 Résilience apporte déjà +10 PV avant les autres multiplicateurs. Un même objet portant aussi +15 PV cumulerait donc +25 PV avant ces multiplicateurs. C'est une combinaison possible, pas une récompense moyenne ni une mesure de sa fréquence.

Les nombres bruts de Diablo II ne doivent pas être copiés : +2 Force et +2 Puissance n'ont pas nécessairement le même poids, et la valeur d'une ligne dépend du cumul sur l'équipement complet.

## Référence Diablo II consultée

Tables extraites en lecture seule de `D:\JEUX\Diablo 2 old\Diablo II`, avec priorité `patch_d2.mpq`, puis `d2exp.mpq`, puis `d2data.mpq`. Le lecteur d'archives installé est utilisé comme bibliothèque ; aucune donnée ProjectD2 ni sauvegarde n'est utilisée. Les quatorze tables demandées ont été trouvées dans `patch_d2.mpq`. Aucun jeu lancé. Les tailles et dates des trois archives sont restées inchangées pendant l'extraction ; les empreintes SHA-256 des tables ont été vérifiées.

Reproduction : lancer `tools/art/study_diablo2_crafting_readonly.ps1` dans Windows PowerShell 32 bits. Les tables et leurs empreintes sont dans `artifacts/itemisation-loot-01/diablo2-reference/inspection.json`. Le nom historique du lecteur n'implique aucune remise en chantier du craft.

Exemples contrôlés dans les tables locales ; numéros de ligne physique, en-tête compris :

| Table et ligne | Affixe / supports indiqués par cette ligne | Valeur | Niveau d'affixe | Niveau requis |
| --- | --- | ---: | ---: | ---: |
| MagicSuffix.txt:315 | of the Jackal, notamment torse et ceinture | +1 à +5 vie | 1 | 1 |
| MagicSuffix.txt:316 | of the Fox, mêmes supports | +6 à +10 vie | 7 | 5 |
| MagicSuffix.txt:317 | of the Wolf, mêmes supports | +11 à +20 vie | 15 | 11 |
| MagicSuffix.txt:374 | of Strength, notamment amulette et ceinture | +1 à +2 Force | 1 | 1 |
| MagicSuffix.txt:734 | of Strength, gants | +1 Force | 1 | 1 |
| MagicPrefix.txt:665 | Lizard's, notamment armures hors gants/bottes, anneaux, amulettes | +1 à +2 mana | 1 | 1 |
| MagicPrefix.txt:306 | Lizard's, mêmes supports | +3 à +5 mana | 3 | 2 |

Les supports et niveaux varient selon les lignes : ces exemples ne décrivent pas tous les objets. Le niveau d'affixe n'est ni le niveau du personnage ni, à lui seul, une règle d'accès complète. Aucun calcul de probabilité des affixes de Diablo II n'est revendiqué ici.

Les colonnes `level`, `maxlevel`, `levelreq`, `frequency`, `group`, `rare`, `itype*` et `etype*` distinguent progression, niveau requis, pondération, familles, qualité autorisée et supports admis/exclus. Plusieurs lignes portent parfois le même nom : ne pas sélectionner la première occurrence sans contrôler sa version, sa fréquence et ses supports.

Le guide officiel décrit les objets magiques avec au maximum un préfixe et un suffixe ; les rares peuvent en porter deux à six, avec au maximum trois préfixes et trois suffixes et sans doublon de groupe. Un affixe peut produire plusieurs effets affichés : le nombre d'affixes ne correspond pas toujours au nombre de lignes de statistiques.

Sources : [objets magiques et rares](https://classic.battle.net/diablo2exp/items/magic.shtml), [suffixes](https://classic.battle.net/diablo2exp/items/magic/suf.shtml), [préfixes](https://classic.battle.net/diablo2exp/items/magic/pre.shtml). Les formules simplifiées ou anciennes de ces pages ne sont pas utilisées pour calculer l'éligibilité exacte.

## Ordre de travail recommandé, à discuter

1. Définir le parcours global : grandes zones, ordre d'accès, rôles, durée approximative et jalons de progression. Un schéma suffit avant de dessiner toutes les cartes.
2. Construire une première zone représentative du début du jeu et y prévoir les rencontres : familles de monstres, groupes, densité, élites, récompenses et ressources de soin.
3. Équilibrer un petit catalogue d'équipements de début de partie sur cette zone jouable, avec les trois qualités envisagées. Mesurer le temps pour remplacer un objet, les combats entre deux améliorations et le cumul de l'équipement complet.
4. Étendre cartes, rencontres et équipement par étapes, en réservant de la marge pour les zones suivantes. Finaliser la courbe globale lorsque le parcours est suffisamment représentatif.

Fixer les taux et toutes les plages de bonus sur les cartes actuelles produirait une courbe globale sans données sur la suite. À l'inverse, les rencontres et les récompenses peuvent révéler des besoins dans la géométrie des cartes : leur conception gagne à avancer ensemble.

L'étude Diablo II peut donc continuer comme référence structurelle, avec des valeurs de travail clairement provisoires. Aucun taux, plafond de propriétés, changement de qualité, nerf chiffré ou blocage des objets de haut palier n'est intégré par ce document.
