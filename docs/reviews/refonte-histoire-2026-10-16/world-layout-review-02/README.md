# Préparation du premier secteur — PM02

17 octobre 2026 · **MONDE-PM02-01**.

Le joueur a validé la base MONDE-01 à MONDE-04 et demandé la création progressive
des décors au fil des cartes. L’accord est conservé dans
[design-acceptance-01.json](../world-generation-study-01/design-acceptance-01.json).

Il confirme ensuite cette proposition, avec une condition explicite : voir
chaque lieu fait à la main avant de le valider. L’accord complémentaire
[MONDE-PM02-ACCEPT02](design-acceptance-02.json) distingue l’organisation acceptée
des plans et dessins encore à examiner. La première présentation est
[PLACE-CAMP-01](../handcrafted-places-review-01/README.md).

Le [cahier de génération](../../../GENERATION_CAMPAGNE.md) précise sept familles
fonctionnelles de cartes/modules et un raccordement possible du premier secteur.
Les boîtes du schéma ne fixent pas un nombre de cartes. Le détour rejoint le
parcours avant le lieu obligatoire ; les emplacements de ce lieu et du futur
chef restent incontournables dans ce graphe.

Le [plan des décors](../../../DECORS_CAMPAGNE.md) définit cinq lots de besoins :
sols/limites, ouvertures, mobilier courant, repères et refuge. Ils seront
préparés avec les modules utiles puis montrés sur carte, à taille native,
avant intégration des dessins et effets concernés.

`first-sector-contract.json` décrit cette préparation de manière structurée.
`validation.json` vérifie la connexité du graphe, l’ordre des lieux obligatoires,
la possibilité d’ignorer le détour et la cohérence documentaire.

Ces contrôles ne prouvent pas la praticabilité d’un terrain généré : aucune
carte de jeu ni nouvelle illustration n’est produite par cette étape.
Dimensions, emprises des décors, objectifs détaillés, pouvoirs, boss et
renouvellement des rencontres restent à définir ou à vérifier au prototype.
Les problèmes critiques de reprise de PM00 précèdent l’activation en campagne.
