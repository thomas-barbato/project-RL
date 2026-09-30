# Première faune de surface

24 septembre 2026 — prototype jouable, générations 101–103.

**Noms validés le 26 septembre :** Grignoteur de gravats devient **Grignoteur**,
Fouisseur vibrant devient **Fouisseur pâle**, Herbivore à carapace devient
**Dos-rond**. Mordeur des friches et Brise-os sont conservés. La
[revue des noms](NOMS_DU_BESTIAIRE.md) est appliquée à la présentation sans changer
les identifiants, comportements, populations ou versions de génération. Les
anciens noms dans le bilan ci-dessous décrivent les lots historiques.

**Conception suivante :** le [catalogue du bestiaire](BESTIAIRE_ET_RENCONTRES.md)
et les [butins des rencontres](BUTINS_DES_RENCONTRES.md) préparent la suite avant
toute nouvelle intégration. Leurs noms proposés et profils de butin ne changent
pas les comportements des cinq espèces jouables décrites ici.

Cette tranche éprouve les familles du [bestiaire proposé](PROPOSITION_BESTIAIRE_SURFACE.md),
pas une liste définitive d'espèces. Cinq représentants sur neuf sont jouables.
La surface reste majoritairement humanoïde ; les profondeurs ne changent pas.
La réputation reste reportée. Aucun nouveau butin, objet marchand ou objectif
de campagne n'est ajouté.

## Cinq comportements

- **Mordeur des friches**, niveau de référence 2, glyphe `b` : petits groupes
  de deux ou trois. Chaque individu cherche une position de contact libre
  autour d'une cible réellement perçue ; la meute ne partage pas une position
  cachée du joueur. Les murs, occupations, territoires et zones protégées
  restent contraignants.
- **Fouisseur vibrant**, niveau de référence 2, glyphe `f` : solitaire,
  quasiment aveugle. Suit les bruits proches, dont les leurres, avec propagation
  et atténuation par les obstacles. Garde brièvement la dernière case entendue,
  pas la position réelle d'une cible qui se déplace silencieusement. Le son ne
  permet pas de mordre à travers un mur. Il ne creuse pas les décors.
- **Herbivore à carapace**, niveau de référence 1, glyphe `g` : petit carapacé
  herbivore neutre, seul ou par deux. Se déplace lentement près de son point
  d'apparition, sans poursuivre le joueur ni attaquer. S'il survit à un coup,
  se replie : immobilité et +3 d'armure pendant quatre occasions d'action,
  durée rafraîchie par un nouveau coup. Un petit bouclier et un état textuel
  signalent ce repli. Aucune récompense de défaite n'est attachée à cette espèce.
- **Grignoteur de gravats**, ajouté en génération 102, niveau de référence 1,
  glyphe `n` : charognard neutre, seul ou par deux. Évite le joueur et ses
  compagnons réellement perçus à courte portée. Cherche une issue, y compris
  par un détour latéral, sans se rapprocher d'une menace. Ne mord au contact
  que si cette recherche établit qu'il est acculé. Un manque de budget de
  recherche ou un déplacement empêché ne l'autorise pas à attaquer. Dès qu'une
  issue s'ouvre, il fuit de nouveau ; perdre la menace de vue le calme. Pas
  d'hostilité permanente, de poursuite cachée ni de récompense de défaite.
- **Brise-os**, ajouté en génération 103, niveau de référence 4, glyphe `k` :
  charognard solitaire rare des étendues sauvages, absent des friches initiales
  et des territoires habités. Au contact, annonce une morsure en marquant une
  case. Son action suivante frappe cette même case, pas la nouvelle position
  du joueur. Quitter la case peut donc éviter le coup même en restant à portée.
  Après avoir mordu, même dans le vide, reste immobile deux occasions d'action
  avant de pouvoir poursuivre ou préparer une autre attaque. Le déplacement
  forcé annule le coup préparé ; les nouveaux obstacles et la protection des
  abris restent respectés. Valeurs d'essai : 6 dégâts perforants, 8 XP, pas
  de butin spécifique. Une action longue du joueur peut laisser s'écouler
  plusieurs occasions ennemies : l'annonce ne met pas le monde en pause.

« Herbivore à carapace » remplace le nom provisoire « Brouteur de mousse ».
« Herbivore » indique le régime alimentaire, pas un tempérament obligatoirement
passif. Son identifiant stable reste `core:moss_grazer` indépendamment du nom.
Les valeurs de combat sont des essais, pas un équilibrage définitif.

## Tirage borné, cohérent et persistant

Les deux biomes de surface déclarent un profil `fauna` dans
`content/core/regional_worlds/simulation.json5`. Un profil contient :

- Un nombre de tentatives de groupes, ici 1 à 2, une plage de niveaux autorisés
  de 1 à 3 dans les territoires habités, de 1 à 4 dans les étendues sauvages
  depuis la génération 103, et un budget de danger de 6.
- Des familles pondérées : charognards 40, fouisseurs 30, carapacés 30.
- Des espèces avec identifiant, niveau fixe, habitats, poids, taille de groupe,
  distance minimale aux passages et profil d'acteur ordinaire.

La génération 102 ajoute le Grignoteur aux charognards avec le même poids
d'espèce que le Mordeur (100 chacun). Le poids de leur famille, le nombre de
groupes et le budget ne sont pas augmentés. Il occupe les graviers, les
broussailles et les sols de ruines. Les autres profils restent inchangés.

La génération 103 ajoute le Brise-os uniquement aux charognards des étendues
sauvages, avec un poids de 25 contre 100 pour chacune des deux autres espèces.
Il reste seul, à au moins 20 cases des passages, sur graviers, broussailles ou
sols de ruines. Son coût de 4 dans un budget total de 6 empêche d'en générer
deux sur la même carte. Le nombre de groupes et les poids des familles ne
changent pas. La zone initiale utilise toujours le profil des territoires
habités, qui ne contient pas cette espèce.

Le tirage filtre d'abord les niveaux, habitats disponibles et coûts possibles,
puis choisit une famille et une espèce. Ajouter des espèces à une famille ne
multiplie donc pas son poids. Le coût d'un groupe est son effectif multiplié
par le niveau de référence de l'espèce. Ce niveau ne multiplie pas directement
ses statistiques et ne suit pas automatiquement le niveau du joueur.

Les groupes sont rapprochés dans un habitat admissible ; si aucune petite
zone ne peut les accueillir, le tirage est abandonné. Les mordeurs apparaissent
dans graviers, broussailles et sols de ruines ; les fouisseurs dans prairie,
broussailles et sol humide ; les carapacés dans les mêmes terrains végétalisés
ou humides. Aucun animal n'apparaît dans un abri protégé, sur une case réservée
ou trop près d'un passage. Toutes les familles ne sont pas garanties par carte.

Ce même mécanisme peuple les friches initiales et les régions de surface.
Il intervient après les sites, contacts, butins et populations antérieurs,
avec son propre flux aléatoire déterministe. Les animaux restent des acteurs
physiques persistants : revisiter la zone ne retire pas leurs blessures et
ne relance pas le tirage. La mémoire sonore et le repli expirent aussi hors
écran, sans transmettre au joueur les événements d'une autre carte.

## Présentation et compatibilité

Les cinq espèces sont identifiables par un glyphe, une description dans la
légende et leur nom/niveau lors de l'inspection. CAPTEURS distingue les hostiles
du neutre. Les marqueurs n'exigent pas F2 et respectent la visibilité actuelle.
Les animaux n'ont pas de système électronique ; le soigneur humanoïde ne
les considère pas comme ses patients.

Les nouvelles parties utilisent la génération 103. Une reprise 102 retire le
Brise-os et rétablit le plafond de niveau 3 des étendues sauvages, sans modifier
les autres poids, profils ou états sauvegardés. Une reprise 101 retire aussi
le Grignoteur de son catalogue, retrouve exactement ses poids et
son ordre de tirage précédents, et garde ses trois espèces. Le changement
de nom de l'herbivore est une présentation sans effet sur la simulation.
Les générations 100 et
antérieures retirent ces métadonnées avant calcul d'empreinte et génération :
elles gardent leurs anciennes populations et leur rejeu déterministe. Les
nouvelles variantes de comportement et d'état sont ajoutées à la fin des enums
sérialisées, sans modifier les anciens indices ni la structure des acteurs.

## Vérification

Tests de simulation : perception sonore, leurres et obstacles, expiration de
la mémoire, protection temporaire et dégâts, neutralité, placement de meute,
zones protégées, fuite, défense acculée, issue rouverte, morsure annoncée,
esquive par déplacement, récupération immobile et progression hors écran.
Tests de génération sur 64 graines :
répétabilité, variété des compositions, habitats, niveaux, budget et cases
réservées, maintien du poids d'une famille après ajout d'une espèce. Tests client :
reprise par instantané et rejeu, y compris générations 100, 101 et 102.

Les diagnostics isolés `--ui-cold-surface-fauna`,
`--ui-cold-surface-fauna-shell`, `--ui-cold-surface-forager`,
`--ui-cold-surface-forager-cornered`, `--ui-cold-surface-bone-breaker`,
`--ui-cold-surface-bone-breaker-recovery` et `--ui-cold-legend` capturent le rendu natif
sans modifier la sauvegarde ni les réglages du joueur. Ils ne remplacent pas
une séance d'équilibrage jouée dans les friches générées.

Validation du lot 102 : les 555 tests moteur passent, ainsi que compilation
toutes cibles, formatage et contrôle du diff. Le parcours client complet a
réussi 249 tests et ignoré un diagnostic manuel ; deux échecs initiaux ont été
revérifiés séparément. La scène visuelle plaçait le joueur à couvert : sa
géométrie a été corrigée, sans désactiver la furtivité, puis le test est passé.
Le test de chargement avec délai fixe de cinq secondes a dépassé ce délai sous
charge parallèle, puis a réussi isolément sans changement de son code. Les
captures natives de fuite, d'état acculé, de l'herbivore renommé et de la légende
ont été inspectées. Aucun remplacement de la partie ouverte ni de son
exécutable n'a été effectué.

Validation du lot 103 : 560 tests moteur et 254 tests client réussis ; un
diagnostic manuel reste ignoré. Après les derniers ajustements de présentation,
les trois tests client ciblés du Brise-os et les 560 tests moteur ont été
relancés avec succès. Les captures natives de préparation, de récupération et
de légende ont été inspectées. L'exécutable normal a été recompilé, sans lancer
de partie ni modifier les sauvegardes du joueur. Une séance d'équilibrage en
conditions réelles reste à faire.
