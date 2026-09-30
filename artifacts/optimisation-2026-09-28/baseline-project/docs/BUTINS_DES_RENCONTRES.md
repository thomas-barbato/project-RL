# Butins des rencontres — direction retenue et réglages à définir

24 septembre 2026. À lire avec le [catalogue du bestiaire](BESTIAIRE_ET_RENCONTRES.md).

## 1. Décisions, questions et périmètre

Demandes confirmées : préparer le catalogue des ennemis avant leur intégration,
employer des noms destinés à rester et concevoir leurs butins en même temps.
Après comparaison des références, l'utilisateur a accepté le modèle proposé
le 24 septembre 2026. **Les principes retenus sont :**

- une provenance cohérente, avec des tables partagées plutôt qu'une petite
  liste exclusive à chaque espèce ;
- des possessions réelles, des restes exploitables et des trouvailles de sites
  traités séparément ;
- 0 à 1 objet ou lot comme point de départ pour les trouvailles supplémentaires
  et les ressources, pas comme plafond de tout l'équipement porté ;
- une puissance principalement liée à la profondeur et au danger de la
  rencontre, sans alignement automatique sur le niveau du joueur ;
- de rares belles trouvailles possibles près de la surface et plusieurs voies
  d'acquisition pour les besoins ordinaires.

**Précision confirmée le 25 septembre :** le mélange d'équipements humains,
étranges, organiques et vivants concerne le monde, pas une table de butin
universelle. Le type d'ennemi filtre les objets admissibles avant tout tirage :
un robot ne donne pas d'équipement humain. Une source sans objet compatible
ne se rabat jamais sur l'ensemble du catalogue. Les profils peuvent être
partagés entre ennemis compatibles, sans devenir une liste exclusive par espèce.
La nature organique d'un objet ne suffit pas à l'attribuer à un animal.
Voir le [catalogue des bases](CATALOGUE_BASES_EQUIPEMENT.md) et les
[règles d'affixes et de provenance](AFFIXES_ET_PROVENANCE_EQUIPEMENT.md).

Cet accord ne fixe ni les objets précis, ni leurs taux, prix, niveaux ou affixes.
Les pourcentages pédagogiques et les profils chiffrés ci-dessous restent des
exemples à éprouver. Il ne valide pas individuellement tous les noms du bestiaire
et ne déclenche aucune intégration automatique.

Un accord ultérieur avait retenu les noms des quatre familles de surface :
Vaurins, Talvènes, Ostrèles et Vélines. La
[revue des noms du 26 septembre](NOMS_DU_BESTIAIRE.md) rouvre ces appellations ;
elles restent ici des repères de provenance, sans changement des familles ou
des règles de butin. Les déclinaisons par espèce et les objets de la section 11
restent à finaliser ; aucun objet n'est ajouté au jeu par ces accords.

**Direction ultérieure retenue :** l'artisanat est reporté. Le
[service d'amélioration d'équipement](AMELIORATION_EQUIPEMENT.md) agit sur les
bonus d'un objet existant, jamais sur sa base. Il pourra tirer des statistiques
ou des effets spéciaux dans un catalogue prédéfini avec leurs visuels. Les
ressources nécessaires et les modalités précises restent à définir.

Direction antérieure conservée : la profondeur favorise les objets puissants,
mais une trouvaille exceptionnelle près de la surface reste possible. Les
objets nécessaires à la progression ne dépendent pas d'un tirage rare. Voir
le [système de butin déjà implémenté](BUTIN.md).

Cette note consigne le modèle retenu et propose des profils et critères de validation.
Elle ne crée aucun objet jouable, prix, affixe, cadavre, inventaire de PNJ,
recette ou nouvelle table chargée. Les marchands et les paris restent inchangés.

## 2. Ce que font les références

Sources consultées le 24 septembre 2026. Les observations suivantes décrivent
des principes, pas une reproduction exhaustive de leurs règles ou exceptions.

### Caves of Qud

Qud associe des inventaires aux créatures et peut varier leur contenu par
des tables. Exemple documenté : le snapjaw scavenger possède une morsure
naturelle et un tirage de la table « Snapjaw Minion 0 » ; sa description
mentionne des armes de bronze ou de fer et des protections de faible palier.
La fiche distingue cet inventaire de la probabilité de laisser un cadavre.
Ce n'est donc pas simplement une entrée universelle « tuer = objet quelconque ».
Source : [fiche du snapjaw scavenger, wiki officiel](https://wiki.cavesofqud.com/wiki/Snapjaw_scavenger).

Les paliers servent de repères de progression pour créatures, objets et zones ;
le niveau du personnage n'est pas l'unique notion pertinente. Cette observation
ne signifie pas que tous les inventaires ou tous les marchands suivent une
même formule. Source : [terminologie des paliers, wiki officiel](https://wiki.cavesofqud.com/wiki/Terminology#Tier).

Les fiches consultées indiquent des données issues d'anciennes versions du jeu.
Nous n'en reprenons ni les pourcentages ni une garantie universelle selon laquelle
chaque objet naturel, porté ou équipé tomberait toujours. Le wiki officiel est
une documentation communautaire, distincte d'une déclaration des développeurs.

### Cogmind

Le lien entre adversaire et pièces récupérables est central : les robots sont
constitués des pièces qu'ils utilisent. Le développeur présente explicitement
ce choix comme une manière de permettre au joueur de chercher une pièce sur
un type de robot connu, plutôt que de dépendre seulement d'un butin universel.
Source : [Grid Sage Games — Robots](https://www.gridsagegames.com/blog/2014/08/robots/).

Le manuel officiel Beta 17.1 décrit un contrôle de survie de chaque pièce,
influencé notamment par son intégrité et les dégâts subis. La matière récupérée
possède aussi sa propre fourchette. L'inventaire transporté, distinct des pièces
attachées, est déposé au sol ; les robots fabriqués ont une restriction sur
les pièces récupérables. Il n'y a donc pas un plafond général de 0 à 1 objet
pour chaque robot. Source : [manuel officiel, section Salvage](https://www.gridsagegames.com/cogmind/manual.txt).

### Ce que nous pouvons en retenir

L'aléatoire et la cohérence ne sont pas opposés : on peut varier ce qu'une
rencontre possède, puis conserver cette réalité pendant le combat. Une source
identifiable permet aussi au joueur de chercher quelque chose délibérément.
Ce constat est notre interprétation de conception ; il ne nous oblige ni à
copier la simulation de Cogmind, ni à peupler le jeu de robots.

## 3. Trois modèles possibles

| Modèle | Intérêt | Risque pour ce projet |
|---|---|---|
| Tout adversaire tire dans une grande table universelle | Beaucoup de surprises, réglage simple, récompense directe du combat | Armes sorties d'animaux sans raison ; peu de choix éclairé sur la recherche d'équipement ; incitation à tout tuer |
| Chaque espèce a sa petite liste exclusive | Sources faciles à reconnaître et identité forte | Répétition, collection de matières sans usage, obligation de chercher une espèce pour chaque besoin |
| **Provenance cohérente + tables partagées + trouvailles de sites** | Variété, monde crédible et plusieurs voies d'acquisition | Demande de bien distinguer inventaire, reste exploitable et récompense du lieu |

**Modèle retenu : le troisième.** L'espèce ne doit pas
enfermer tout son butin dans une liste exclusive, mais la provenance limite
ce qui est plausible. La qualité et la quantité se règlent séparément.

## 4. Trois sources séparées

### A. Possessions existantes

Un humanoïde peut transporter une arme, une protection, de la monnaie, des
fournitures et des objets récupérés. Une machine possède certaines pièces ;
un transporteur peut avoir une cargaison. Les biens sont choisis à la création
de la rencontre, selon le contexte, puis persistent.

- L'arme employée doit correspondre à l'objet récupérable, pas à un second
  tirage d'arme sans rapport effectué après la mort.
- Si un personnage a utilisé deux fournitures sur trois, seule celle qui
  reste peut être récupérée. Aucun nouveau tirage ne remplit sa trousse.
- Un objet donné, volé, consommé ou détruit n'apparaît pas en double au décès.
- Une arme naturelle ou une coque biologique n'est pas automatiquement un
  objet équipable par le joueur.
- La règle simple recommandée est de récupérer les possessions ordinaires
  encore présentes. Les pièces intégrées peuvent relever d'une récupération
  distincte. Une simulation générale de détérioration par les dégâts est
  une extension possible, pas un prérequis adopté ici.

Un plafond universel de 0 à 1 objet ne convient pas à cet inventaire : il
ferait disparaître arbitrairement le reste. Le volume se contrôle en amont,
dans la composition des possessions, puis par leur valeur et leur utilité.

### B. Restes exploitables

Une créature peut laisser une ressource plausible : matière organique,
plaque minérale, pièce mécanique ou résidu anormal matérialisé. Les profils
sont partagés par plusieurs espèces. Il n'est pas nécessaire de définir un
objet « dent de X » différent pour chaque animal.

Le point de départ retenu est **0 à 1 lot exploitable** pour une source commune,
avec une chance explicite de n'obtenir aucun lot. Un corps encore présent
et un lot exploitable sont deux choses différentes : « aucun objet » ne signifie
pas que la créature n'avait pas de corps ou de dents.

Une matière ne sera ajoutée que si un usage ou une valeur d'échange raisonnable
existe. Sinon, son profil peut rester sans objet. La découpe, la cuisine et
l'artisanat ne sont pas ajoutés implicitement par cette proposition.

Un animal calme ne doit pas devenir la meilleure source de monnaie ou
d'équipement rare. Mues abandonnées, sites et commerce peuvent fournir des
alternatives aux prélèvements mortels, si les ressources concernées sont retenues.

### C. Trouvailles du lieu

Un camp, une cache, un nid collecteur ou une carcasse ancienne peut contenir
des objets de tables larges : équipement, consommables, biens échangeables,
éventuellement une trouvaille rare. Cela permet la surprise sans faire porter
une armure à chaque animal.

Le lieu et son contenu existent indépendamment de la mort de son occupant.
Contourner un gardien, l'éloigner ou trouver un autre accès peut permettre la
même découverte. Tuer ne fait pas apparaître automatiquement un coffre ou une
cache supplémentaire. La récompense n'est pas dupliquée entre gardien et site.

## 5. Profils partagés associés au bestiaire

Codes de conception uniquement, pas des identifiants déjà acceptés par le
chargeur. « C » désigne une source commune ; « R » une source pouvant soutenir
un meilleur budget, **pas une rareté d'objet garantie**.

| Profil | Sources compatibles | Résultat possible | Garde-fou |
|---|---|---|---|
| BIO-C | Animaux ordinaires | 0–1 lot organique utile | Pas d'arme, d'armure fabriquée ou de monnaie créée par défaut |
| BIO-R | Grands organismes ou organes particuliers | 0–1 lot, budget potentiel supérieur | Un lot peut rester ordinaire ; pas de trophée obligatoire par espèce |
| MIN-C / MIN-R | Organismes minéraux | 0–1 lot minéral exploitable | Le corps entier ne devient pas automatiquement une armure |
| MEC-C / MEC-R | Machines | Pièces existantes selon récupération ; éventuellement un lot de débris distinct | Une pièce ne devient pas à la fois un objet intact et sa matière de recyclage |
| ANO-C / ANO-R | Êtres anormaux | 0–1 résidu seulement si sa matérialité et son usage sont définis | Pas de monnaie ou d'objet magique universel par simple étrangeté |
| POS | Individus capables de porter des possessions | 0–1 trouvaille diverse attribuée à la génération, en plus de l'équipement réel | Tables communes ; pas de second tirage à la mort |
| SITE | Caches, camps, nids ou dépôts conçus | Nombre de lots propre au lieu, par exemple 1–3 pour un essai | Accessible selon les règles du lieu, pas nécessairement après combat |

BIO, MIN, MEC et ANO décrivent des provenances, pas quatre nouvelles monnaies.
POS est déjà inclus dans l'inventaire persistant ; il ne s'additionne pas une
seconde fois au moment du dépôt. SITE est un budget séparé placé dans le monde.
Les profils ne promettent pas de ressource si aucun objet admissible n'existe.

## 6. Quantité, fréquence et qualité

Il faut régler séparément :

1. **La fréquence** : quelle chance de produire un lot ?
2. **Le nombre de lots** : combien de tirages distincts ?
3. **La quantité par lot** : une arme ou une pile de plusieurs unités ?
4. **Le niveau et la qualité** : quels objets et éventuels affixes sont admissibles ?

Exemple pédagogique, non approuvé : pour un profil 0–1, donner un poids de 75
à « rien » et de 25 à « un lot » produit 25 % de chances d'un lot, pas 50 %.
Sur 100 rencontres de ce profil, l'espérance est de 25 lots ; ce n'est pas une
garantie de résultat. Avec 80 % ordinaire, 18 % inhabituel et 2 % exceptionnel
conditionnellement à l'obtention d'un lot, la chance exceptionnelle par rencontre
est 25 % × 2 % = 0,5 %. Ces chiffres illustrent les paramètres, pas l'équilibrage.

Un uniforme entre 0 et 1 donnerait, lui, 50 % de chances de chaque résultat.
Le choix d'une fourchette ne suffit donc pas à définir la fréquence souhaitée.
Ne pas multiplier silencieusement une chance de drop par un second tirage
0–1 et réduire ainsi une probabilité annoncée deux fois.

### Niveau des objets

- La profondeur et le danger prévu du lieu déterminent une distribution
  préférentielle de puissance ; le niveau de référence de la rencontre et
  sa richesse peuvent la modifier de façon bornée.
- Pour un matériel porté, on tire avant la rencontre. L'ennemi et son équipement
  restent cohérents : il ne devient pas rétroactivement mieux équipé parce
  que le joueur a gagné un niveau ou l'a attiré dans une zone plus profonde.
- Une plage comme « objets de niveau 2 à 4 » peut être une bande habituelle,
  pas une interdiction universelle. Des exceptions rares et explicites préservent
  les belles trouvailles proches de la surface déjà souhaitées.
- Distinguer puissance d'un objet, rareté de rencontre, présence d'affixes,
  prix et condition d'utilisation. Un objet rare n'est pas forcément plus fort.
- Ne pas aligner automatiquement tous les objets sur le niveau actuel du joueur.
  Les règles propres aux paris du marchand restent un sujet distinct.
- Les objets de progression indispensable gardent un placement ou une voie
  d'obtention garantie, hors de ces chances de butin ordinaire.

## 7. Variété sans dépendance à une espèce unique

Un besoin courant doit avoir plusieurs sources : objets transportés,
commerce, sites et exploration. Les tables de possessions peuvent largement
se recouvrir entre humanoïdes. Les profils de matières se recouvrent entre
espèces d'une provenance compatible.

Une source thématique peut favoriser un type d'objet sans en avoir le monopole.
Un médecin transporte plus souvent des fournitures, mais n'est pas la seule
manière d'en obtenir ; un atelier favorise les outils sans interdire d'en
trouver dans un sac de voyage. Un animal organique reste une source différente
d'un transporteur mécanique : partager les tables ne signifie pas ignorer
leur nature.

Pas de règle générale « vaincre un ennemi fort = équipement meilleur que celui
du joueur ». Une rencontre peut valoir pour son accès, son information ou le
choix qu'elle impose. Les services ordinaires et les objets de base des
marchands assurent une partie de l'approvisionnement sans quête obligatoire.
Le stock normal sans affixes, la revente des objets cédés et les paris ne sont
pas redéfinis dans cette note.

## 8. Contrat de persistance et prévention des doublons

Le futur système devra distinguer possession existante et récompense générée.
Les possessions et cargaisons sont fixées à la création ; les restes exploitables
sont tirés une seule fois avec un état durable, au moment choisi par le futur
contrat de récupération. Ouvrir un menu, changer de carte ou suspendre ne
relance jamais les dés.

- Chaque objet existant est transféré, pas recréé à partir de son nom.
- Les consommations et pertes réduisent réellement ce qui reste disponible.
- Les décès hors écran suivent les mêmes règles, sans révéler leur contenu
  au joueur hors perception.
- Les doubles, segments de colonie, invocations et productions gratuites ne
  multiplient pas le budget de butin. Une colonie peut partager une enveloppe
  fixée à sa création ; aucune division ne recrée une enveloppe complète.
- Une possession réellement confiée à une créature ne doit pas être supprimée
  par une règle anti-farm : prévenir la création gratuite, pas effacer les biens
  existants. Une invocation ne génère pas d'équipement revendable gratuitement.
- Les anciennes parties conservent leurs tirages, empreintes et possessions.
  Une migration ou une nouvelle version de génération sera nécessaire pour
  du contenu nouveau ; le document n'autorise aucune réécriture de sauvegarde.

## 9. Écart avec le moteur actuel

Lecture du code local au 24 septembre 2026 :

- `src/loot/mod.rs` fournit des tirages pondérés de définitions existantes,
  filtrés par profondeur, type de carte et source, avec quantité par pile.
  Le nombre de tirages est fourni par l'appelant ; zéro tirage est possible,
  mais une entrée avec quantité nulle est refusée. Un profil 0–1 doit donc
  décider le nombre de tirages ou être une extension explicitement validée,
  pas une ligne `quantity: [0, 1]` glissée dans le format actuel.
- Ce module ne génère pas encore les niveaux d'objets ni leurs affixes.
  Une plage de profondeur n'est pas un niveau d'objet.
- Le traitement de mort dans `src/game/game_state.rs` peut créer une épave
  à partir de composants récupérables. Cela ne constitue pas déjà un système
  complet d'inventaire équipé et pillable pour tous les PNJ.
- Les cinq animaux de surface ne reçoivent aucun drop nouveau par cette note.
  Les profils BIO/POS/etc. ne sont pas des champs de contenu implémentés.

Avant intégration : définir des objets réellement utiles dans le modèle retenu,
puis concevoir les possessions des PNJ, les transferts à la mort et les profils
de restes. Ne pas rendre les ennemis porteurs de faux objets uniquement pour
imiter visuellement un système qui n'existe pas.

## 10. Validation future et arbitrages ouverts

Le modèle général et les quatre noms de famille de surface sont acceptés.
Restent à finaliser avec l'utilisateur : les noms des espèces et des familles
profondes, les objets précis et l'utilité des matières, ainsi que la place
d'une éventuelle dégradation des objets. Les valeurs de fréquence et de puissance
seront éprouvées après définition des objets. Une nouvelle confirmation du
modèle général n'est pas nécessaire pour poursuivre cette conception.

Tests futurs :

- probabilités et quantités mesurées sur beaucoup de graines, incluant les
  cas sans objet et les rares exceptions de profondeur ;
- absence de doublons et conservation exacte des possessions, fournitures
  utilisées, quantités, modifications et propriétaires ;
- stabilité après reprise, retour de zone, mort environnementale ou hors écran ;
- distinction entre fréquence par individu et par rencontre ; pas de pluie
  d'objets due au nombre de membres d'une meute ou d'une colonie ;
- accès à plusieurs sources pour les besoins ordinaires ; pas de quête dont
  l'unique solution attend un drop rare ;
- économie des ventes : objets par heure de jeu, valeur des trajets de retour,
  nombre d'améliorations réellement intéressantes et encombrement ;
- intérêt d'éviter un combat ou d'explorer un site, sans que tuer tous les
  animaux neutres soit systématiquement le meilleur choix économique.

Le plafond 0–1 est le point de départ retenu pour les trouvailles et restes.
Il ne remplace ni la conception des objets, ni la cohérence des possessions,
ni les essais d'économie à l'échelle d'une expédition.

## 11. Premières ressources de surface — proposition à valider

Cette liste précise quatre objets candidats, pas quatre objets déjà acceptés
ou implémentés. Elle concerne les restes animaux ; l'équipement des humanoïdes
et les trouvailles des sites devront avoir leur propre catalogue. La faune
n'est pas chargée de fournir tous les besoins du joueur.

**Statut actualisé :** les pistes de fabrication ci-dessous sont conservées
comme historique, mais l'artisanat est reporté. Elles ne valent ni recettes
adoptées ni ingrédients validés pour le nouveau service du PNJ.

| Objet proposé | Provenance plausible | Usage minimal proposé | Ancienne piste d'artisanat — reportée |
|---|---|---|---|
| **Peau souple** | Plusieurs Vaurins et Talvènes, si une partie exploitable subsiste ; stocks d'artisans et commerce | Bien échangeable commun, regroupé sans une variante par espèce | Travail du cuir pour certaines protections ou pièces de portage |
| **Plaque chitineuse** | Les trois Ostrèles ; également mues abandonnées, caches de récupération et commerce | Bien échangeable ; une plaque brute n'est pas une armure déjà équipée | Renforcement ou fabrication de certaines protections |
| **Membrane intacte** | Les différentes Vélines ; stocks d'artisans, caches et commerce | Bien échangeable distinct d'une peau ; aucune promesse de vol pour le joueur | Confection d'éléments souples d'équipement ou de protection |
| **Poche irritante** | Ostrèle des noues, dont la poche est déjà décrite ; approvisionnement d'artisans ou commerce adapté | Candidat à un composant spécialisé ; à laisser hors du premier lot si aucun usage intéressant n'est retenu | Préparation d'un consommable irritant ; ni lancer utilisable, ni nuage, ni recette ajoutés implicitement |

Les références aux espèces utilisent leurs déclinaisons proposées. Valider
une famille ne valide pas automatiquement la distribution précise de cette table.

### Quantité et choix du résultat

Une récupération de restes fournit au plus **un lot** dans ce premier cadre,
et peut ne rien fournir. Le lot n'est pas la somme de plusieurs jets indépendants
« peau + membrane + plaque + poche ». Une Ostrèle des noues pourra, si le profil
est retenu, donner une plaque **ou** une poche, pas les deux par défaut.
L'équipement porté et les biens du site ne sont pas compris dans cette limite.

Les taux, masse, taille de pile, prix et état exploitable restent à définir.
Les niveaux du bestiaire ne créent pas une série « Peau +1, Peau +2, Peau +3 ».
Un grand prédateur peut avoir de meilleures chances de laisser un reste utile
si l'équilibrage le justifie ; cela ne garantit ni objet rare ni bénéfice.

### Utilité avant multiplication des ressources

Le moteur possède déjà des objets de type matériau et du commerce, mais ces
quatre propositions n'ont ni définition chargée ni prix. Leur revente reste
donc elle aussi à intégrer et vérifier, pas une fonctionnalité livrée ici.
Un futur stock de rachat devra rester compatible avec la revente des objets
cédés au marchand ; aucune matière vendue ne disparaît au profit d'une règle
commerciale différente introduite silencieusement.

Trois biens uniquement revendables peuvent déjà faire double emploi. Si la
distinction peau/plaque/membrane n'apporte aucun choix utile après les essais,
réduire la liste plutôt que fabriquer artificiellement une recette pour chacun.
La Poche irritante est conditionnelle à un usage retenu : son nom ne suffit pas
à justifier un quatrième emplacement de butin sans intérêt.

Les usages d'atelier de la table sont des possibilités à discuter. Aucun
système complet d'artisanat, métier, quête de collecte ou contrat répétable
n'est adopté par cette liste. Aucun prélèvement ne devient un ingrédient
obligatoire pour utiliser une compétence déjà apprise. Ces matières ne
soignent pas automatiquement le corps du joueur et n'ajoutent pas une faim
ou une obligation de chasser pour se nourrir.

### Ne pas récompenser systématiquement l'abattage des neutres

Les animaux calmes ne reçoivent pas un bonus de rareté destiné à rentabiliser
leur mise à mort. Toute ressource retenue doit aussi disposer d'au moins une
voie d'acquisition sans tuer son producteur : commerce ou stock d'un lieu,
et mue lorsque c'est pertinent. Les sites de collecte restent finis et
persistants ; aucun passage de zone ne renouvelle gratuitement leur contenu.

Cela ne garantit pas, à lui seul, un bon équilibre économique : mesurer le
gain de vente, le temps de collecte, la charge et les alternatives d'exploration.
Si la chasse aux petites créatures paisibles devient la meilleure source de
revenu, revoir le taux, la valeur ou la disponibilité des autres sources.

### Décision : artisanat reporté, service de bonus retenu

L'utilisateur a préféré reporter l'artisanat et proposer un PNJ capable
d'ajouter ou de modifier les bonus d'un équipement existant contre des
ressources. La base de l'objet reste intacte. Les bonus peuvent comprendre
des effets spéciaux prédéfinis avec leurs visuels ; une intervention unique
par objet a été envisagée. Voir le [contrat de conception](AMELIORATION_EQUIPEMENT.md).

La crainte d'ingrédients introuvables à cause de l'aléatoire reste un point
à résoudre pour ce service. Les propositions de sources garanties et de
substitutions ne sont pas transformées silencieusement en règles approuvées.

Suite : définir les opérations et effets utiles, puis retenir ou réduire les
ressources candidates, leurs usages, leur approvisionnement et leur valeur.
Les noms d'espèces, leur intégration et les tables de butin ne changent pas.
