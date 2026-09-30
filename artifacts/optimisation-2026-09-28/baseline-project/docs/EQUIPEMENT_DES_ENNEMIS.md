# Armes portées et butin des ennemis

25 septembre 2026 — génération 108, premier raccordement jouable.

Depuis la génération 109, les mêmes humanoïdes équipés sont aussi présents dans
les friches de départ ; voir [Densité de surface](POPULATION_DE_SURFACE.md).

## Ce qui fonctionne

Les **Artilleurs** et **Soigneurs de terrain** des rencontres régionales de surface
portent maintenant respectivement un fusil et un couteau. Les tags explicites
`core:humanoid_rifle_carrier` et `core:humanoid_knife_carrier` identifient ces
porteurs dans le contenu. Ni le glyphe, ni le comportement d'IA, ni le biome
ne suffisent à classer une créature comme humanoïde.

L'arme est générée à la création de la rencontre, sur un flux séparé dépendant
de la région et de la position initiale. Changer l'ordre des acteurs ou regarder
un ennemi ne change pas ses propriétés. Les autres possessions ne sont pas
encore modélisées par un inventaire général de PNJ.

Premiers réglages :

- une arme par porteur, sans armure ni monnaie supplémentaire ;
- famille imposée par le rôle, source `humanoid_equipped` obligatoire ;
- palier au plus local, soit P1 en surface, sans arme P6 exceptionnelle sur
  un ennemi débutant ; les caches conservent leurs trouvailles exceptionnelles ;
- 65 % de blancs et 35 % d'exemplaires magiques, avec les poids d'affixes communs ;
- pas de bonus d'énergie ni de dissipation sur ces deux profils sans jauges
  correspondantes. Les autres bonus et effets restent ceux du générateur.

Ces chiffres sont provisoires. Le danger influence actuellement le rôle et sa
famille d'arme, pas encore une formule supplémentaire de niveau de rencontre.

## Une arme réelle, pas un deuxième tirage à la mort

Le porteur utilise le profil de cette arme : dégâts, portée, précision et
pénétration. Les bonus de caractéristiques et de PV sont pris en compte sans
réécrire les caractéristiques intrinsèques. Braise et Décharge passent par la
même résolution d'effets que pour le joueur. L'Artilleur conserve son annonce
avant le tir et le Soigneur ses soins limités.

À sa mort, même par un effet persistant, le porteur dépose exactement son arme
avec ses propriétés. Le transfert ne consomme aucun hasard et ne peut pas
produire deux copies en traitant plusieurs dégâts létaux. Une arme naturelle
ne devient jamais un objet. Les cas de disparition sans mort restent distincts.

Une case peut recevoir ce butin même si un objet s'y trouve déjà : rien n'est
écrasé ni téléporté derrière un mur. Les objets sont ramassés un par un avec la
commande ordinaire ; l'indication contextuelle signale leur nombre. L'événement
de butin n'est annoncé que si sa case est visible. Les bonus sont conservés au
ramassage, à la sauvegarde et à la vente/reprise chez un marchand.

## Limites assumées

Les anciens profils de prototype dont la nature n'est pas fixée, les robots,
la faune, les renforts renouvelables et les PNJ de service ne reçoivent pas ce
butin. Les soins du Soigneur restent des charges internes, pas des consommables
récupérables. Les profils propres aux machines et aux animaux restent à créer.

Les munitions des PNJ ne sont pas encore simulées : ils gardent le fonctionnement
antérieur de leurs attaques. Les armes nécessitant une réserve d'énergie sont
refusées par cette première API, plutôt qu'utilisées gratuitement. Le service
d'amélioration et l'inventaire complet des PNJ restent des chantiers séparés.

## Compatibilité et preuves

Les armes portées ont été introduites en génération 108. Les générations 107 et antérieures
conservent leurs anciens profils et l'absence de ces possessions. Seuls les deux
nouveaux tags sont retirés de leur contenu compatible ; les autres tags restent.
Une empreinte v107 capturée avant modification vérifie la reprise inchangée.
Le cache `RLWS` v6 enregistre les armes portées ; les caches v1 à v5 sont repris
par le journal vérifié, sans suppression de sauvegarde.

Tests : effet réellement déclenché par le porteur, bonus passifs, mort directe
ou environnementale, case occupée, transfert unique, ramassage, revente/reprise,
reprise d'un porteur blessé sans soin gratuit, refus d'un profil incohérent,
tirages reproductibles et sans contamination des créatures non déclarées.

Diagnostics isolés : `--ui-cold-enemy-loot <dossier>` effectue une vraie attaque
létale puis montre le butin ; `--ui-cold-enemy-loot-inventory <dossier>` ramasse
la même arme et ouvre sa fiche. Ils utilisent le laboratoire jetable, pas la
sauvegarde de campagne.
