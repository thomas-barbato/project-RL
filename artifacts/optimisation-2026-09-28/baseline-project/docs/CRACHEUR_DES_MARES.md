# Cracheur des mares — intégration v115

27 septembre 2026. Nom validé avec les cinq autres noms restants du deuxième lot.
Correspondance de conception : Ostrèle des noues. Arthropode à coque allongée
et poche ventrale ; ni machine ni humanoïde.

## Rencontre

- Surface sauvage uniquement, sur boue ou eau peu profonde ; apparition à
  au moins 16 cases des passages selon la distance de génération existante.
- Solitaire, niveau de référence 2. Le niveau effectif suit la bande de
  surface (1–3), jamais le niveau du joueur.
- Base de 6 PV, aucune armure innée, aucune arme ou pièce électronique portée.
- Jet chimique de base 3 dégâts, portée 4 cases, perception 5, territoire 6.
- Vise une case, y tire à l'occasion d'action suivante, puis récupère sur
  place pendant deux occasions d'action. L'attaque ne suit pas le joueur.

Changer de case, casser la ligne de vue ou déplacer le Cracheur avant le tir
évite l'impact. Murs et cases protégées sont respectés. Les jets ne laissent
ni flaque, ni poison, ni corrosion, ni autre effet persistant. Le soigneur
humanoïde ne le prend pas pour un allié à soigner.

## Tirage et butin

L'espèce rejoint la famille existante `core:fauna_carapaces`, avec un poids
interne de 60 face au Dos-rond à 100, uniquement lorsqu'un habitat convient.
Le poids de la famille, le nombre de tirages et le budget de danger ne changent
pas : il s'agit de varier les rencontres, pas d'ajouter un groupe systématique.
Les quartiers habités, villes et autres couches ne reçoivent pas cette espèce.

Profil de butin BIO-C : pas d'équipement humain, de nouvelle ressource sans
usage ou d'objet garanti. L'expérience persistante de base vaut 5, ajustée
par les règles de couche existantes.

## Présentation et compatibilité

Nom **Cracheur des mares**, glyphe terminal `o`, case visée marquée sans
dépendre de F2. Inspection, ciblage, légende et CAPTEURS identifient l'animal.
Texte d'annonce : « Le Cracheur des mares prépare son jet. Quittez la case
marquée ! » Le visuel utilise des gouttes et éclaboussures transitoires,
sans représenter une flaque durable qui n'existe pas dans la simulation.

Visée et récupération survivent à une suspension. Une attaque engagée peut
se terminer dans une zone quittée sans viser le joueur distant ni afficher
les événements de cette zone. La génération 115 ajoute l'espèce ; les versions
114 et antérieures retirent seulement cette entrée de la surface sauvage,
en conservant les poids, l'ordre et les autres espèces historiques.

Diagnostics isolés : `--ui-cold-marsh-spitter`, `--ui-cold-marsh-spitter-impact`
et `--ui-cold-marsh-spitter-recovery`, avec un dossier de capture neuf.

## Vérifications réalisées pour cette tranche

Tests du jet fixé, de l'esquive, des murs et protections, du déplacement
interruptif, de la récupération, de l'exclusion des soins humanoïdes et de
la continuation hors écran. Profil issu du vrai tirage de faune, résistance
face au couteau de camp, noms/aperçu, instantanés et rejeu v114/v115.

Résultats : **737 tests moteur, quatre tests client ciblés et 76 tests de
compatibilité de générations réussis**. La suite client complète n'a pas été
relancée pour cette tranche. Génération contrôlée sur 24 cartes, absence sur
terrain sec, absence dans les autres biomes et distance aux passages vérifiées.
Sur huit tirages de niveau, le profil réel tombe en trois coups réussis maximum
avec le couteau de camp ; les ratés, déplacements et temps de récupération
ne sont pas comptés comme des coups réussis.

Compilation tous targets, formatage et contrôle des différences validés.
Captures natives inspectées : `target/ui-marsh-spitter-20260927/cold-start.png`
et `target/ui-marsh-spitter-impact-20260927/cold-start.png` : annonce sur une
case, nom lisible, jet transitoire et récupération après impact.
