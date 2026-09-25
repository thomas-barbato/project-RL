# Marchands et paris d'équipement

25 septembre 2026 — génération 107. Prix et stocks provisoires.

## Offre ordinaire

Les marchands de la ville de surface et des cinq villes souterraines proposent
les trois familles intégrées : couteaux, fusils et vestes. À la profondeur D,
ils vendent les modèles P(D+1), plafonnés à P6, et ceux du palier précédent.
La surface propose donc trois modèles ; chaque ville souterraine en propose six.
Les consommables existants et leurs tarifs sont conservés.

Chaque achat ordinaire crée un exemplaire **blanc**, sans bonus aléatoire. Les
stocks sont finis : deux exemplaires du palier local, un du palier précédent.
L'étal affiche jusqu'à sept lignes selon la hauteur de fenêtre, avec compteur
et rappel du défilement au clavier/à la molette. Les noms réservent la place des prix.
Changer d'onglet, reparler au marchand, changer de zone ou reprendre une partie
ne renouvelle ni le stock ni les propriétés.

## Vente et rachat

Les marchands acceptent les dix-huit bases, même celles absentes de leur étal.
Les objets équipés ou portant un propriétaire restent invendables, comme avant.
La caisse du marchand reste une limite réelle ; aucun crédit gratuit n'est créé.
Une vente ajoute un exemplaire à la liste des objets revendus, avec exactement
les mêmes bonus chiffrés et le même effet spécial, conservés après reprise.
Le rachat ne retouche pas la base ni les propriétés.

## Paris

Trois modèles sont proposés par ville : un couteau, un fusil et une veste du
palier local, avec deux achats possibles par modèle. Le joueur connaît le nom
de base et le prix, **jamais les préfixes, suffixes, valeurs ou effets avant achat**.
Le résultat est créé seulement pendant un achat valide, sur le modèle annoncé.

Le générateur commun tire une à trois propriétés distinctes, au maximum un
effet spécial. Les affixes conservent leurs poids respectifs ; les vestes ne
reçoivent aucun effet offensif. Les nouveaux paris ne génèrent plus les anciens
bonus anonymes d'armure et de réduction de poids.

La profondeur détermine le modèle et donc le palier des bonus. Le niveau du
personnage et la profondeur influencent aussi la qualité des valeurs via le
profil `gamble_scaling` existant : on conserve le meilleur de un à quatre tirages
bornés pour chaque valeur. Cela ne change ni le palier de base, ni les bornes
des bonus, ni la rareté des propriétés, ni les paramètres d'un effet spécial.
Le réglage actuel, plafonné au rang 12, atteint au maximum trois tirages de valeur.

Une vue ne consomme aucun hasard. Fonds insuffisants, inventaire plein ou stock
épuisé ne modifient ni la caisse, ni le stock, ni le prochain résultat du pari.

## Tarifs de travail

| Palier | Couteau | Fusil | Veste |
|---|---:|---:|---:|
| P1 | 50 | 85 | 70 |
| P2 | 85 | 130 | 105 |
| P3 | 130 | 200 | 160 |
| P4 | 190 | 300 | 240 |
| P5 | 280 | 440 | 350 |
| P6 | 410 | 640 | 510 |

Prix en crédits. Vente au marchand : un tiers du prix de base, arrondi à l'entier
inférieur. Rachat d'un exemplaire revendu : prix de base. Pari : deux fois le prix
de base. Il n'existe pas encore d'estimation commerciale distincte selon les
affixes ; ces tarifs ne constituent pas un équilibrage final de l'économie.

L'adaptateur `src/equipment_commerce.rs` porte cette première sélection et ces
prix pour les trois familles connues ; les données de bonus restent dans
`equipment_loot/*.json5`. Les offres historiques du contenu ne sont pas réécrites.
Une autre famille ou un catalogue marchand moddé réclame son raccordement explicite.

## Persistance et validation

`WorldState::register_equipment_merchant` active explicitement ces règles. La
voie `register_merchant` reste inchangée pour les anciennes parties et les anciens
tests. Les profils des paris et les prix de reprise sont persistants ; leurs
références et propriétés sont validées au chargement.

Le commerce décrit ici est actif depuis la génération 107. Une partie 106 ou antérieure
conserve ses anciennes offres et ses anciens paris. Le cache moteur `RLWS` v6
conserve les profils commerciaux et les armes portées ; les caches v1 à v5 sont reconstruits via le
journal vérifié. Une empreinte v106 capturée avant modification contrôle que
la présence du nouveau système ne change pas l'ancien état.

Contrôles : stocks par couche, achats blancs, paris d'armes et armures, identité
du modèle, raretés communes, progression bornée, absence de révélation/RNG dans
les vues, refus atomiques, vente/reprise d'un objet hors stock, reprise des
propriétés et du prochain tirage, clavier et souris.

Diagnostics visuels sans partie normale : `--ui-cold-equipment-merchant`,
`--ui-cold-equipment-gamble`, `--ui-cold-deep-merchant` et `--ui-cold-deep-gamble`, suivis d'un dossier neuf.
Le [premier butin des ennemis](EQUIPEMENT_DES_ENNEMIS.md) est raccordé en génération 108.
Le service d'amélioration reste hors de ce jalon.
