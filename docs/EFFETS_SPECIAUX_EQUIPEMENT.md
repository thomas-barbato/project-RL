# Effets spéciaux d'équipement — catalogue de conception

24 septembre 2026. Les recommandations de l'analyse sont retenues.
Ce catalogue ne signifie pas que les soixante idées sont livrées en jeu.
Le contrat du service et les décisions ouvertes restent dans
[Amélioration d'équipement](AMELIORATION_EQUIPEMENT.md).

## Mise à jour du 25 septembre 2026

- Descriptions courtes et naturelles : voir `TEXTES_EFFETS_A_REFORMULER.md`.
- Perforation rémanente devient **Perforation** ; Fracture accumulée devient **Marquage**.
- **Ricochet** remplace Alternance dans le laboratoire : un seul rebond vers
  une autre cible proche, sur les armes de tir.
- **Catalyse** devient un cône depuis le personnage, orienté vers la cible :
  il brûle les occupants et fait exploser chaque cible déjà brûlante avant
  l'attaque. Les nouvelles brûlures ne provoquent pas de chaîne immédiate ;
  les anciennes sont renouvelées, pas consommées.
- Écho différé reste en attente d'une décision de suppression.
- Ces changements concernent les prototypes du laboratoire, pas les objets
  de campagne ni un service PNJ. Les anciennes primitives moteur sont conservées
  pour la compatibilité. Les sections historiques 12 et 14 ne décrivent plus
  les profils Catalyse et Ricochet jouables.

## 1. Direction retenue

- Préparer un grand catalogue avant la distribution des propriétés ; ne pas
  inventer les mécaniques ou leurs animations au moment du tirage.
- Commencer par douze effets représentatifs, sans réduire le catalogue futur
  à ces douze entrées ni intégrer en bloc soixante effets non éprouvés.
- Mélanger plaisir immédiat, positionnement et choix de construction du
  personnage. Tous les bonus n'ont pas besoin d'être des systèmes complexes.
- Mutualiser les mécanismes sans conserver des doublons uniquement recolorés.
- Garder les dégâts intrinsèques et l'identité de l'objet. Un effet secondaire
  est distinct et ne doit pas rendre la vitesse ou la forme de l'arme inutiles.
- Le hasard de l'amélioration et celui d'une activation en combat sont séparés.
  Charges et conditions lisibles sont préférées pour les effets qui déplacent,
  interrompent, consomment un état ou changent fortement la situation.
- Ne pas garantir le fun sur la seule fiche : vérifier les usages réels en jeu.

## 2. Règle commune : impact par défaut, porteur sur mention explicite

| Nature | Origine ou bénéficiaire |
|---|---|
| Zone offensive issue d'une touche, mêlée **ou** distance | Case de l'impact effectivement réussi |
| Effet annoncé « autour du porteur » | Position du porteur, quelle que soit l'arme |
| Soin/protection du porteur | Porteur ; ce n'est pas une zone offensive |
| Soutien à un allié | Allié admissible désigné par la fiche |
| Marque ou charge attachée à une cible | Cible identifiée ; la fiche précise si elle suit ses déplacements |
| Ligne, cône, retour ou écho | Géométrie déclarée ; Catalyse est une exception explicite depuis le porteur vers la cible |

Exemples : une explosion d'impact sur une épée et sur une arme de tir part
dans les deux cas de la cible touchée. La Décharge circulaire historiquement
demandée autour du joueur reste une exception **explicitement** centrée sur
le porteur. Il n'existe plus de règle « mêlée = porteur / distance = cible ».

Le point d'impact est mémorisé lors de la touche. Un ennemi tué disparaît,
mais l'origine de la zone reste disponible. Un repoussement ne déplace pas
rétroactivement l'explosion ; une charge qui suit sa cible doit le préciser.
Rater une attaque ne crée pas un impact réussi. Une attaque visant une case
vide peut néanmoins toucher un occupant de son cône : seule une vraie touche
est alors admissible, jamais le simple point visé.

L'origine n'accorde ni immunité au porteur ni permission de toucher les neutres.
Portée, obstacles, porteur, alliés, neutres et zones protégées constituent des
règles distinctes. Les informations affichées restent limitées à la perception,
sans utiliser la visibilité du joueur comme filtre artificiel des dégâts.

## 3. Premier ensemble retenu : douze fiches fonctionnelles

Les déclenchements sont des orientations de conception. Taux, puissances,
délais, limites de charges et compatibilités exactes ne sont pas encore chiffrés.
Aucune valeur des tests moteur ne vaut approbation de paramètres de jeu.

| Effet | Fonction et origine | Déclenchement à détailler | Présentation et garde-fou principal |
|---|---|---|---|
| Décharge circulaire | Dégâts électriques expressément autour du **porteur** ; exception à l'impact par défaut | Frappe admissible ; chance bornée ou charge à comparer en essai | Anneau/arcs courts sur les cases affectées ; pas de seconde décharge produite par la première |
| Conduction | Décharge depuis l'**impact**, propagée plus facilement dans l'eau | Frappe admissible et terrain compatible | Branches au sol conformes à la propagation ; aucune traversée de mur implicite, exposition des acteurs déclarée |
| Brandon | Brûlure portée par la cible touchée | Touche ou dégâts positifs à arrêter dans la fiche chiffrée | Marque de brûlure et durée ; cumul et rafraîchissement bornés |
| Suintement caustique | Zone persistante sur la case d'**impact** | Frappe admissible | Flaque reconnaissable ; expiration et danger pour les occupants explicités |
| Percussion | Repoussement de la cible depuis l'axe de l'impact | Condition/charge prévisible, pas un déplacement surprise incontrôlable | Direction et destination légale indiquées ; masse, résistance, murs et zones protégées respectés |
| Vol de vie (précision utilisateur : ancien « soin sur impact ») | Rend au **porteur** une fraction des dégâts réellement infligés | Dégâts directs positifs sur cible admissible | Soin réel plafonné par attaque et aux PV manquants ; pas de guérison gratuite sur décor ou cible artificielle hors banc d'essai |
| Égide d'impact | Protection temporaire du **porteur** préparée par sa frappe | Frappe admissible, réserve et fréquence bornées | Indicateur de protection disponible ; ne s'empile pas sans limite |
| Perforation | Ligne secondaire prolongeant l'attaque **au-delà de l'impact** | Frappe admissible | Trajet lisible, obstacles vérifiés ; pas de seconde résolution gratuite sur tout le trajet initial |
| Marquage | Marque sur une même cible, détonation autour d'elle au seuil annoncé | Frappes répétées et compteur visible | Segments de charge et explosion ; état attaché à la bonne cible, seuil/cumul bornés |
| Ricochet | Rebond depuis l'impact vers une autre cible proche | Touche avec une arme de tir | Un rebond ; obstacles respectés ; pas de retour vers le porteur ou les victimes principales |
| Catalyse | Cône depuis le porteur ; brûlure et explosion sur chaque cible déjà enflammée | Touche orientant le cône vers la cible | État capturé avant l'attaque ; pas de réaction en chaîne avec les nouvelles brûlures |
| Écho différé | Répétition affaiblie sur l'ancienne case d'**impact**, pas une poursuite de la cible | Frappe admissible et échéance de simulation | Marque persistante avant résolution ; délai lisible, dégâts sur les occupants réellement présents |

### Combinaisons à éprouver

- Brandon + Catalyse : préparer une cible pour déclencher une explosion.
- Conduction + eau : choisir un champ de bataille favorable.
- Marquage / Ricochet : concentration sur une cible ou dégâts répartis sur deux.
- Égide d'impact + futur Pas de dégagement : frapper, se protéger, se replacer.
- Suintement caustique + future Dette de mouvement : rester ou bouger devient coûteux.
- Futur Ancrage explosif + Percussion : déplacer une cible piégée vers un groupe.

Les combinaisons ne sont pas des boucles automatiques de déclenchements.
L'ordre des impacts, états, déplacements, dégâts secondaires et décès sera
explicite. Une animation ne décide jamais de cet ordre.

## 4. Réserve du catalogue : conserver, différencier ou retravailler

Les entrées ci-dessous restent proposées, pas individuellement promises pour
la première livraison. Les douze fiches précédentes complètent cette réserve
de quarante-huit idées. Le classement exprime une priorité, pas un niveau d'objet.

| Famille | Effets en réserve | Recommandation retenue |
|---|---|---|
| Foudre | Arc vagabond, Foudre différée, Lien ionique, Condensateur de riposte | Différencier l'arc du ricochet ; annoncer le délai ; rendre le lien lisible ; empêcher la charge gratuite sur menace insignifiante |
| Feu et chimie | Gerbe incendiaire, Brume irritante, Évaporation brutale, Cendre piégée | Valoriser l'alignement et les passages ; rendre la baisse de précision perceptible ; vapeur réellement liée à la vision ; pièges utiles dans les rencontres |
| Contrôle | Emprise de givre, Prison cristalline, Aimantation, Rupture de geste, Entrave filamenteuse | Conditions prévisibles ; préciser si un dégât périodique brise la prison ; attraction compatible avec la matière ; lien et interruptions bornés |
| Soutien | Réserve de survie, Suture de victoire, Transfusion, Purge offensive, Récupération énergétique | Réserve distincte d'un soin retardé opaque ; éviter l'attente après élimination ; soutien compatible avec les alliés ; ni purge universelle ni énergie infinie |
| Protection | Carapace de riposte, Contre-onde, Voile de secours, Conversion thermique, Écran d'interception | Riposte bornée ; poussée maîtrisée ; mémoire des ennemis conservée ; conversion spécialisée ; interception de trajectoire avant distribution |
| Mobilité | Pas de dégagement, Élan de chasse, Charge d'inertie, Retraite fumante, Crochet latéral, Sillage électrifié | Recul facultatif sans fenêtre répétitive ; pas de charge par allers-retours gratuits ; fumée simulée ; crochet distinct de Percussion ; poursuite non trivialement exploitable |
| Projectiles | Retour spectral, Projectile satellite, Éclatement radial, Ancrage explosif | Différencier trajectoires et résistances ; retour avec vrai intérêt de placement ; charge visible ; éclatement autour de l'impact ; charge mobile explicitement attachée |
| Combinaisons | Moisson d'états, Dette de mouvement, Dette d'agression | Consommation d'état maîtrisable ; événements de mouvement/attaque précis ; IA capable d'exploiter les choix lorsqu'ils doivent être tactiques |
| Perception | Balise sonore, Brouillage focal, Marque révélatrice, Silence d'impact, Leurre d'impact, Verrou de phase | Balise à retravailler pour ne pas devenir une pénalité involontaire ; spécialisations filtrées ; pas de vision à travers les murs ; leurre compris par l'IA ; verrou différé sans adversaires concernés |
| Anomalies | Faille d'attraction, Cisaillement spatial, Miroir d'agression, Déphasage réactif, Couronne orbitale | Déplacements cohérents ; géométrie affichée ; miroir limité à des profils pris en charge ; projectile admissible clairement défini ; orbite testée près des obstacles |

Une grande liste ne justifie pas soixante mécaniques indépendantes. Les formes
partagent propagation, marques, échéances et présentation, tout en produisant
des décisions différentes. Un effet étrange profond n'est pas automatiquement
plus puissant qu'un effet simple de surface.

## 5. Garde-fous communs

- Un effet temporaire ne cumule pas sa puissance avec lui-même. Brûlure et
  corrosion renouvellent leur durée ; les durées ne s'additionnent pas. Les
  restrictions de renouvellement des contrôles existants sont conservées.
- Les effets distincts peuvent tous coexister, chacun avec son échéance :
  aucune limite artificielle de deux. L'identité est celle de l'effet, non
  celle de l'arme qui l'applique. Un changement de source ne crée pas un doublon.
- Pas de relance générale des propriétés par leurs propres dégâts secondaires.
  Les synergies autorisées sont identifiées, bornées et testées.
- Une action multicible n'accorde pas gratuitement une activation par ennemi.
  Les exceptions éventuelles exigent un budget et une fiche explicites.
- Équilibrer dans le temps de simulation : une arme rapide ne doit pas gagner
  automatiquement tous les comparatifs avec du soin ou des marques par touche.
- Éviter soin, énergie, protection et contrôle illimités ; conserver l'intérêt
  des dégâts de base, consommables, états adverses et compétences apprises.
- Ne pas transformer une amélioration en malus involontaire : déplacement,
  bruit, fumée et consommation d'état exigent une lecture et une maîtrise réelles.
- Compatibilités de tirage déclarées : réserve d'énergie, attaque concernée,
  équipement et contraintes de cible. Une spécialisation n'est pas un bonus
  universel ; elle ne doit pas être techniquement inutilisable sur son objet.
- Charges, délais et résultat du tirage appartiennent à l'exemplaire. Déposer,
  vendre, racheter, équiper ou sauvegarder ne doit pas les réinitialiser à profit.
- Même simulation dans les deux rendus ; formes, symboles et texte complètent
  la couleur. Marques durables distinctes des animations brèves, options de
  réduction d'animations sans perte d'information, aucune cible cachée révélée.
- Prévisualisations non mutantes et cohérentes avec l'exécution ; préciser les
  zones conditionnelles plutôt que promettre une activation aléatoire certaine.

## 6. Première brique technique et limites actuelles

`WeaponEffectKind::RadialDamage` permet une zone secondaire déclarative.
Exemple technique, **non ajouté aux objets de campagne**, valeurs de laboratoire :

```json5
{
  type: "radial_damage",
  trigger: "on_hit",
  origin: "impact", // valeur par défaut ; "bearer" doit être explicite
  radius: 1,
  damage: { amount: 7, damage_type: "electrical" },
  affects_source: false, // choix obligatoire, distinct du centre
}
```

Le champ `radius` est le budget de propagation existant, pas forcément une
distance géométrique lorsque les coûts de sol/eau diffèrent. Les coûts sont
strictement positifs ; murs et portes fermées bloquent cette propagation.

- Déclenchements autorisés : `on_hit` ou `on_damage` seulement, explicites.
- Au plus une zone par définition d'effet et attaque normale. Priorité à la
  case visée si elle a produit l'événement requis, sinon première case
  admissible dans l'ordre stable des coordonnées ; aucune sélection aléatoire.
- Position d'impact capturée avant suppression ou déplacement de la cible.
  L'origine porteur utilise sa position à la résolution de la zone.
- Une attaque de réaction n'active pas cette brique. Les zones utilisent les
  dégâts et événements existants sans réexécuter une attaque d'arme.
- `affects_source` est obligatoire. Pour les autres occupants, cette brique
  conserve la sémantique spatiale des dégâts de zone existants ; elle **ne
  fournit pas encore de filtre hostile/allié/neutre**. Ce n'est pas une décision
  de tir ami pour le futur catalogue. Les protections existantes du monde
  continuent de bloquer les dégâts. Le ciblage final doit précéder la distribution.
- Présentation via `PropagationResolved` et les clients existants, sans nouveau
  profil électrique distinct par effet ni prévisualisation spécifique de la zone secondaire.
- Aucun effet attribué par le PNJ, charge, taux ou nouveau champ d'inventaire.
  Les douze fiches ne sont pas douze effets implémentés. Les définitions des
  armes existantes et le format de sauvegarde ne sont pas modifiés.

Validation : mêlée/distance, exception porteur, raté, coup mortel, absorption
totale, multicible, case vide, absence de relance sur réaction, exposition du
porteur, zone protégée et reprise déterministe. Avant ouverture aux objets,
ajouter les essais de ciblage social, sauvegarde des bonus par exemplaire,
prévisualisation et lisibilité des nouveaux profils visuels.

## 7. Banc d'essai accessible au joueur

Le menu principal propose désormais un [Laboratoire de test](LABORATOIRE_DE_TEST.md)
isolé des parties normales. Il contient 54 exemplaires mêlée/tir : témoin sans
effet, onde d'impact, décharge circulaire, conduction, brandon et suintement
caustique, vol de vie, Perforation, Égide d'impact, Percussion, Catalyse,
Marquage, Ricochet et Écho différé ; chacun avec et sans bonus statistiques
aléatoires. Les bonus chiffrés se cumulent depuis tous les objets équipés ;
les effets spéciaux ne viennent que de l'arme qui effectue l'attaque.
Ricochet n'existe qu'en fusil ; les autres profils existent en lame et fusil. Il permet de
réinitialiser cibles, terrain et équipement. Ce sont des profils des primitives
existantes, avec les douze fiches exécutables en essai, pas leur attribution par le PNJ.

La présentation distingue désormais onde d'impact, décharge du porteur,
conduction, brûlure et acide par leurs formes, mouvements et palettes. Les
zones électriques tracent des éclairs reliés et non des cadres ; les effets ne remplacent pas le glyphe du
personnage, et les états temporaires/expositions au sol ont leurs indicateurs.
Un seul total de dégâts est affiché par destinataire et résolution, sans
modifier les paquets simulés ni ajouter/enlever de victime.

## 8. Vol de vie — première brique, limitée aux essais

L'utilisateur a explicitement précisé qu'il veut du **vol de vie**, pas un
soin fixe. `life_steal` exige `trigger: "on_damage"`, un `percent` de 1 à 100,
un `maximum_per_attack` positif et un `required_target_tag` explicite.
Aucun personnage n'est admissible par défaut. Le laboratoire utilise son
propre marqueur sur ses mannequins ; cela
ne définit pas encore le filtre social ni la liste des ennemis de campagne.

Le soin est proportionnel au total des dégâts directs admissibles de l'attaque,
arrondi à l'entier inférieur une seule fois, plafonné par attaque puis aux PV
manquants. Les dégâts excédant les PV restants de la cible ne sont pas comptés.
Il est résolu une fois par définition d'effet et attaque normale.
L'admission est capturée avant suppression d'une cible tuée. Une réaction,
un raté, une absorption totale, un terrain vide ou les dégâts secondaires
n'accordent rien. Le porteur mort pendant la résolution n'est pas ressuscité.
Les PV rendus utilisent `IntegrityRestored`, également présenté pour les
consommables : croix ascendantes sur le bénéficiaire et valeur réelle.

Le profil de laboratoire utilise 50 %, au maximum 3 PV par attaque, sans
hasard d'activation ni délai.
Ce n'est pas un équilibrage de campagne : la fréquence dans le temps, les
cibles admissibles et les bonus par exemplaire restent à compléter. Aucune
définition d'arme de campagne ni structure de sauvegarde n'est changée.

## 9. Perforation — première brique, limitée aux essais

`piercing_line` exige une `length` et des dégâts strictement positifs, avec
un déclenchement explicite `on_hit` ou `on_damage`. Le laboratoire utilise
`on_hit`, 3 cases et 3 dégâts perforants. La base de l'arme reste inchangée.

- Un trait par définition d'effet et attaque normale, jamais par victime.
- L'impact visé admissible est prioritaire ; à défaut, première vraie touche
  admissible dans l'ordre stable des coordonnées. Un tir vide sans touche
  ne produit rien. `on_damage` exige des dégâts réellement positifs.
- Départ après l'impact, dans la direction porteur initial → impact capturé.
  Pas d'accrochage aux huit directions ; l'arrondi de grille est symétrique.
- La cible initiale et toutes les victimes de l'attaque principale sont exclues
  des dégâts du trait, même après un déplacement. Un coup mortel conserve
  sa position d'impact, comme les effets radiaux.
- Murs, portes fermées, deux obstacles fermant un coin et cellules protégées
  arrêtent le trait. Les portes ouvertes et l'eau ne sont pas des barrières.
- Dégâts secondaires via les primitives existantes, sans nouvelle attaque
  ni nouveau déclenchement des propriétés d'arme. Les réactions ne lancent
  pas cet effet. Les protections et résistances des destinataires s'appliquent.
- La simulation n'est pas filtrée par la visibilité. La présentation ne reçoit
  que les cellules perçues puis vérifie à nouveau leur visibilité au rendu.
- Le profil reprend le ciblage spatial des premières zones : aucun nouveau
  filtre social n'est déduit. Alliés/neutres, fréquence et équilibrage restent
  à arrêter avant distribution dans la campagne, ainsi que la prévisualisation
  de cette portée secondaire et la persistance des bonus par exemplaire.

Présentation : trait ivoire orienté, pointe mobile, traînée courte, glyphe du
personnage préservé et variante stable en animations réduites. Le profil se
choisit à partir de la définition d'effet exacte de `PropagationResolved`,
pas de l'arme actuellement sélectionnée au moment du rendu.

## 10. Égide d'impact — protection de laboratoire

`apply_bearer_status` applique un état une fois au porteur survivant après une
attaque normale admissible (`on_hit` ou `on_damage` explicite), jamais une fois
par victime. Les réactions et dégâts secondaires ne créent pas une nouvelle
application. Le profil de laboratoire choisit `on_damage` ; les coups mortels
comptent, les ratés et impacts sans dégâts directs ne comptent pas.

L'état `lab:impact_aegis` porte `damage_guard: 3` et dure trois fins de tour,
acquisition comprise. Le modificateur exige une valeur positive, une durée
finie et `keep_existing` : pas de cumul de puissance ou de renouvellement.
Le prochain impact positif sur les PV consomme l'état après armure et
résistances, pour bloquer `min(dégâts, protection)` ; le reliquat est perdu.
Un impact mixte reste indivisible. Les dégâts périodiques passent également
par cette résolution, pas les dommages à la durabilité des composants.

Des états différents continuent à coexister sans plafond arbitraire. Si un
contenu déclare plusieurs protections différentes, seule la plus forte se
consomme sur un impact (égalité départagée par identifiant croissant) ; leurs
valeurs ne s'additionnent pas. Les autres états restent intacts. La consommation
émet `DamageGuardAbsorbed` puis `StatusRemoved(Consumed)`, pas une expiration
naturelle ni sa transition. Les composantes de `DamageImpactApplied` décrivent
toujours les dégâts après armure/résistances, avant protection et limitation
aux PV restants ; son total décrit bien la perte réelle de PV.

Présentation : contour bleu en forme de bouclier autour du porteur, petite
icône tant qu'il reste actif, puis contour fragmenté et montant bloqué à sa
consommation. Les animations restent locales aux cellules visibles, derrière
le personnage, et fixes en mode réduit. Aucun effet offensif n'est suggéré.

Ce profil ne change aucune arme de campagne ni structure sérialisée : l'état
utilise les instances de statut existantes. Il reste à équilibrer sa fréquence
d'acquisition et ses valeurs avant distribution dans le jeu, puis à porter les
effets spéciaux par exemplaire. Une protection consommée peut être regagnée à
la frappe admissible suivante ; aucun délai interne n'est ajouté en silence.

## 11. Percussion — poussée et recharge de laboratoire

`percussion` déclare une `force` positive et un `recovery_status` connu. Son
déclenchement est fixé à une touche directe d'attaque normale, même entièrement
absorbée, sur une cible survivante. Le statut de récupération doit être inerte,
fini (au moins deux fins de tour), `keep_existing`, sans famille, modificateur,
hook ni transition. Ces contraintes sont vérifiées au chargement et à la
construction des règles moteur.

Le profil d'essai utilise une force de 4 et `lab:percussion_recovery` pour
trois fins de tour, acquisition comprise : deux tours restent après l'attaque.
Cette recharge **commune au porteur** est une limite de prototype explicite,
pas l'implémentation des futures charges par exemplaire. Elle interdit le
contournement par changement d'arme, n'est pas rafraîchie par les frappes et
reste dans les statuts sérialisés existants. La cible visée survivante est
prioritaire, sinon le premier impact admissible dans l'ordre des coordonnées.
Une seule cible par définition et attaque. Une cible déjà déplacée par une
autre résolution n'est pas repoussée une seconde fois depuis son ancien impact.
Un raté, une case vide, une réaction ou une cible tuée n'active rien. Chaque
tentative valable consomme la disponibilité, même face à un blocage, une
résistance, un corps incompatible ou un ancrage fixe.

La poussée d'une case réutilise la résistance au déplacement existante : masse
du corps, masse transportée et ancrage. Elle est distincte des dégâts et de la
Puissance de l'arme ; le profil de base n'est pas transformé en attaque de mêlée
pour permettre un tir. Sa continuation de grille suit la pente de la frappe,
symétriquement dans toutes les directions. Les cases infranchissables, occupées
ou protégées arrêtent le mouvement ; deux coins bloqués empêchent le passage
diagonal. Aucun acteur n'est poussé en chaîne et aucune collision n'ajoute de
dégât. Les hooks de mouvement existants s'exécutent une fois si la cible bouge.

Les anciennes techniques de mêlée gardent leur calcul de force et leur
géométrie historiques pour préserver les reprises. La résolution de résistance,
de destination et l'événement `ForcedMovementResolved` sont partagés ;
`WeaponImpulseResolved` ajoute seulement le trajet pour la présentation.
Chevrons ambre et arrêt transversal se distinguent des ondes et projectiles ;
seules les cases visibles sont dessinées, avec un repère fixe en mode réduit.
Le montant de dégâts reste unique et suit la cible déplacée si elle est visible.

Aucun objet de campagne ni format de sauvegarde n'est modifié. Avant
distribution : paramètres d'équilibrage, filtrage social et compatibilités,
prévisualisation de destination et propriétés persistantes par exemplaire.

## 12. Historique : ancienne Catalyse — remplacée par le cône

`catalysis` déclare un `required_status` connu, un `radius`, des `damage`
strictement positifs et un choix obligatoire `affects_source`. Le déclenchement
est une touche directe normale (`on_hit`), même absorbée ; l'origine est
toujours l'impact. Le chargement n'accepte ni origine porteur ni autre trigger.
Le profil d'essai consomme `core:burning`, inflige 6 dégâts thermiques avec un
budget de propagation de 1 (sol et eau peu profonde à coût 1 ; murs, portes
fermées et eau profonde bloqués),
sans atténuation, et épargne explicitement le porteur.

- Le statut doit exister avant les dégâts et applications d'états de cette
  touche ; Brandon ajouté par la même arme ne crée pas une combinaison gratuite
  dès le premier coup. Le brûleur peut être une autre source.
- Après l'attaque primaire, l'impact visé admissible est prioritaire, sinon
  le premier impact brûlant dans l'ordre stable des coordonnées. Une seule
  Catalyse par attaque, même avec plusieurs propriétés Catalyse ou victimes.
- Le statut requis est retiré sans transition d'expiration naturelle. Les autres
  effets ne sont ni purgés ni renforcés. Si le statut a déjà été consommé sur un
  survivant, il ne peut pas servir une deuxième fois.
- Une touche mortelle conserve le carburant et la case capturés : l'explosion
  reste possible, mais aucun faux retrait de statut n'est émis sur l'acteur
  disparu. `StatusCatalyzed` relate la conversion dans les deux cas.
- Un déplacement antérieur dans la même résolution ne déplace pas le centre
  de l'explosion. Sur un survivant, la consommation suit bien son identité.
- Raté, case vide sans vraie touche, réaction, absence de statut requis ou
  porteur mort ne déclenchent rien. Aucune nouvelle attaque/propriété d'arme
  n'est exécutée par les dégâts de la zone. Les défenses et protections
  spatiales existantes continuent de s'appliquer.
- Comme les premières zones de laboratoire, le ciblage secondaire est spatial,
  sans nouveau filtre social implicite. Il reste à définir avant distribution.

Présentation : condition visible sous l'arme active sur la cible perçue,
disparition de la marque consommée, ignition puis flammes pleines et braises,
total unique par victime et variante stable pour les animations réduites.
Les cases cachées ne sont jamais dessinées. Aucune nouvelle donnée de
sauvegarde : la condition repose sur le statut déjà sérialisé, pas sur un
compteur client. Les versions antérieures et les armes de campagne restent
inchangées. La sélection explicite de l'arme Catalyse maîtrise la consommation ;
compatibilités de tirage, prévisualisation de zone et équilibre sont encore
nécessaires avant son attribution à des objets normaux.

## 13. Marquage (anciennement Fracture accumulée) — marque de préparation du laboratoire

`accumulated_fracture` déclare `mark_status`, `threshold`, `radius`, `damage`
et un `affects_source` obligatoire. Le déclenchement est une touche normale
(`on_hit`) et l'origine reste l'impact réel. La marque doit être un statut inerte,
fini (au moins deux fins de tour), à charges bornées exactement au seuil,
rafraîchissant sa durée. Aucun hook, modificateur, famille, blocage de famille
ou transition d'expiration n'est admis. Charge n'est pas cumul de dégâts.
Chargement et initialisation moteur vérifient ces contraintes.

Le laboratoire retient provisoirement trois charges, cinq fins de tour (celle
de l'application comprise), 6 dégâts cinétiques, rayon de propagation 1 et
porteur exclu. Chaque nouvelle charge renouvelle la durée ; les durées ne
s'additionnent pas. La marque est commune sur une cible entre les exemplaires
Fracture et les sources : le dernier attaquant admissible déclenche la zone.
C'est une limite de prototype, pas le futur stockage des propriétés par objet.

- Une charge au plus par attaque normale, même multicible ou avec plusieurs
  définitions Fracture. Priorité à l'impact visé admissible, sinon premier
  impact admissible dans l'ordre stable des coordonnées.
- Raté, réaction, case vide sans touche, source morte et terrain protégé ne
  chargent pas. Une touche totalement absorbée compte, car le trigger est une
  touche et non des dégâts positifs.
- Les charges avant la touche sont capturées avant les dégâts et applications
  d'états. Un changement de ce même compteur pendant la résolution ne donne
  pas une deuxième charge gratuite par Fracture.
- La marque suit l'identité du survivant. Un déplacement antérieur ne change
  pas rétroactivement l'origine de la détonation : l'impact reste capturé.
  Le coup mortel au seuil peut exploser ; un coup mortel avant le seuil ne
  laisse ni marque, ni champ au sol, ni explosion.
- Au seuil, la marque seule est consommée avant la zone. Expiration naturelle
  sans nouvelle frappe : disparition sans détonation. Les autres états restent
  indépendants. Les dégâts secondaires ne déclenchent ni nouvelle charge ni
  nouvelle propriété d'arme, y compris sur un voisin déjà marqué.
- Murs, portée, armure et zones protégées restent gérés par la propagation et
  les dégâts existants. Comme les autres zones de laboratoire, les destinataires
  sont spatiaux : le filtrage hostile/allié/neutre reste à arrêter avant campagne.

Présentation : petits segments au-dessus de la cible, compteur exact et durée
dans les informations perçues, éclatement en fragments anguleux au seuil,
un seul total de dégâts par victime. Aucune information cachée, animation
réduite stable, aucune transformation des dégâts intrinsèques de l'objet.
La marque utilise les instances de statut sérialisées existantes ; aucun
changement de format de sauvegarde ni de règles de campagne. Reprise binaire
du moteur testée à deux charges. Les deux dernières fiches sont décrites ci-dessous.

## 14. Historique : Alternance remplacée ; Écho différé en réexamen

`alternation` déclare `range`, `memory_turns` et `damage`. Une touche normale
mémorise une cible par porteur, pour cinq fins de tour dans le laboratoire.
Si la précédente cible est différente et encore vivante, un retour lui inflige
3 dégâts cinétiques depuis le nouvel impact, dans une portée de 4 cases
(Chebyshev). Le lien suit sa position actuelle, respecte murs, portes fermées,
coins fermés et zones protégées, et ne blesse pas les occupants du trajet.
Un coup mortel sur la nouvelle cible peut provoquer le retour, mais ne conserve
pas de mémoire de cette cible morte. La visibilité ne filtre pas les dégâts ;
seules les cellules actuellement visibles sont montrées, sans révéler de cible
cachée ni annoncer un lien possible vers elle.

`delayed_echo` déclare `delay_turns` (1 à 8) et `damage`. Le laboratoire utilise
un délai de 2 : après la frappe, deux actions supplémentaires consommant un tour
s'écoulent avant la résolution, pendant la phase d'environnement après les PNJ.
La case capturée est immobile. À l'échéance, son occupant reçoit 2 dégâts
cinétiques, sauf le porteur ; case vide, protégée ou devenue mur = aucun dégât.
Un coup mortel peut laisser un écho. Le changement d'arme ou la mort de la source
n'annule pas une frappe déjà programmée. La fin de partie, comme les autres
effets d'environnement, arrête la simulation.

Une case n'accepte qu'un écho en attente, toutes sources confondues : nouvelle
touche sans cumul, renouvellement, déplacement du délai ni changement de source.
Une attaque normale active au plus une instance de chaque mécanisme, même si
plusieurs définitions ou victimes sont présentes. La case visée réellement
touchée est prioritaire, sinon première touche dans l'ordre stable des coordonnées.
Réactions, ratés et tirs vides ne les déclenchent pas. Les dégâts secondaires
n'activent pas les propriétés de l'arme, le vol de vie ou une nouvelle mémoire.

Les mémoires et échéances sont sérialisées dans l'instantané moteur ; l'empreinte
de compilation protège la reprise de l'ancien schéma via le rejeu existant.
Le `Debug` omet l'état vide pour conserver les empreintes historiques. Aucun
changement de génération, de butin, d'objet de campagne ou de service PNJ.
Dans ce prototype, la mémoire est partagée par porteur entre les armes Alternance ;
la propriété spéciale par exemplaire reste à concevoir. Hors porteur et zones
protégées, l'Écho conserve la règle spatiale actuelle sans filtre social inédit.

Présentation : rubans entrelacés mobiles pour Alternance, signe de lien sur la
cible mémorisée ; crochets persistants et compteur pour l'Écho, fermeture brève
à l'échéance. Rendu limité à la perception, compatible avec les animations réduites.
Les textes actuels sont rassemblés dans `TEXTES_EFFETS_A_REFORMULER.md`.
