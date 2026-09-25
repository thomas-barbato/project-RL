# Laboratoire de test

Carte temporaire accessible directement depuis **Menu principal → Laboratoire de test**,
au clavier ou à la souris, sans création de personnage. Ce n'est ni une zone de
campagne, ni une quête, ni une nouvelle partie sauvegardable.

## Utilisation

- Ouvrir l'inventaire avec la commande habituelle et équiper une variante dans
  un emplacement d'arme. Les raccourcis restent ceux des options du joueur.
- Attaquer les cibles immobiles `X` : seules, regroupées, dans l'eau ou séparées
  par un mur. Elles ne répliquent pas et ne donnent pas de butin.
- Pour Percussion : les `X` sont déplaçables ; au sud-est, `L` est trop lourd
  pour la force d'essai et `A` est fixé au sol. Tous restent sans riposte.
  Le mannequin devant le mur permet de vérifier une poussée bloquée.
- Au sud-est, `B` a 4 points d'armure et `E` peut esquiver : ils permettent
  d'essayer la pénétration et la précision. Ils ne ripostent pas.
- Au sud, le `!` est un **dispositif d'essai** immobile : 4 dégâts électriques
  quand le personnage est à une case, aucun tir à plus grande distance.
  Il sert à tester les protections et peut tuer le personnage ; réinitialiser
  les essais restaure tout. Les douze mannequins `X` restent inoffensifs.
- **Tab** parcourt les cibles visibles ; **F** attaque directement la cible
  sélectionnée, sauf pour Catalyse : **F ouvre la prévisualisation du cône**,
  puis F ou un clic confirme. Souris et flèches orientent l'aperçu ; Échap
  ou clic droit annule sans consommer de tour. Ces commandes restent réattribuables.
  La frappe principale doit toucher une cible à portée de l'arme : une case
  vide ou trop éloignée affiche un aperçu invalide et ne permet pas d'attaquer.
  L'aperçu inclut les explosions des cibles déjà brûlantes et visibles.
- Un **clic gauche sur une cible visible** la sélectionne ; un nouveau clic
  sur cette même cible la désélectionne. Cliquer sur une autre cible remplace
  la sélection. Ces clics ne lancent aucune attaque et ne consomment aucun tour.
- Les mannequins ne peuvent pas esquiver, à l'exception de `E`. Les armes de
  tir n'y consomment pas de munitions ; ce ne sont pas des valeurs d'équilibrage.
- **Échap → Réinitialiser le laboratoire** restaure les cibles, le personnage,
  le sol et l'ensemble des variantes, avec un nouveau tirage de bonus. La commande habituelle de recommencement
  réinitialise également le laboratoire, pas une partie de campagne.
- **Échap → Retour au menu principal** quitte les essais.
- Le personnage commence avec 12 PV manquants (sans descendre sous 1 PV),
  pour essayer le soin. Réinitialiser le laboratoire rétablit cette blessure
  de test ; aucune blessure n'est infligée au personnage de campagne.

## Variantes présentes

54 armes : treize profils en lame et fusil, plus Ricochet en fusil uniquement,
chacun **sans bonus statistique** et **avec 1 à 3 bonus aléatoires distincts**.
Deux armures témoins, avec et sans bonus, complètent ces essais (56 objets).
Les bonus appartiennent à chaque exemplaire, sont visibles dans sa fiche
et ne modifient ni sa définition ni ses dégâts de base. Aucun allègement
n'est tiré pour ces variantes.
Les armes sources sans masse déclarée reçoivent une base d'essai de 2 kg,
identique pour les deux exemplaires ; leurs définitions de campagne sont inchangées.

Les bonus chiffrés sont désormais des **affixes nommés par exemplaire** :
identifiant stable, palier, valeur et accord grammatical sont conservés.
Le nom affiche au plus un adjectif et un suffixe ; la fiche met les bonus en
évidence, en caractères plus grands et gras, sans répéter les noms d'affixes
ni afficher de tiret de liaison. Exemple : « Lame précise de siphon » peut
aussi porter un bonus de vitalité, affiché simplement « PV maximum +15 ».
Les suffixes et descriptions d'effets viennent désormais d'un profil prédéfini
référencé par chaque exemplaire. Les définitions des modèles du laboratoire ne
portent plus ces effets : l'arme réellement utilisée fournit son propre profil,
y compris pour la prévisualisation du cône et les animations. Les deux variantes
d'un essai portent le même effet, avec ou sans bonus chiffrés. Le dépôt, le
ramassage, la revente et la reprise conservent ces propriétés sans nouveau tirage.
Les effets sont attribués explicitement dans ce laboratoire, pas encore tirés
dans les butins de campagne.

Tirage provisoire au **palier P1** du catalogue : Puissance, Coordination,
Résilience, Perception ou Traitement (+1 à +2) ; précision (+5 à +10 au score,
pas un pourcentage) ; pénétration (+1) ; PV maximum ou énergie maximale
(tout entier entre +5 et +15) ; dissipation thermique (+1 par tour).
Ces valeurs remplacent les anciennes fourchettes anonymes du laboratoire,
sans modifier les objets existants de campagne. Les six paliers ont des bornes
validées, mais seul P1 est généré dans ces essais ; aucun plafond du joueur ne change.
Aucun bonus de régénération d'énergie ni de
bande passante n'est encore ajouté. Les plafonds intrinsèques seront revus
séparément : les bonus d'équipement ne sont pas écrêtés au plafond de création.

**Équipé n'est pas synonyme d'actif pour attaquer :** toutes les armes équipées
et les armures portées cumulent leurs bonus chiffrés. Un objet rangé n'apporte
rien. Changer d'arme active ne modifie ni les statistiques ni les jauges.
En revanche, les effets spéciaux appartiennent uniquement à l'arme utilisée :
le vol de vie d'une lame secondaire ne s'applique pas aux tirs du fusil.

Augmenter les PV ou l'énergie maximum ne remplit pas la réserve. Retirer
l'équipement réduit le maximum et rabat la réserve actuelle seulement si elle
le dépasse. La dissipation ne change que les prochains refroidissements,
sans effacer immédiatement de chaleur. Le dépôt et la récupération conservent
les nouveaux bonus par exemplaire. Aucun de ces tirages n'est distribué en campagne.
Les noms et valeurs sont aussi conservés par les instantanés moteur et par le
circuit de vente/rachat (vérifié dans les tests, sans ajouter de marchand au laboratoire).
Le laboratoire reste temporaire, sans sauvegarde de campagne. Le nouveau cache
binaire moteur porte l'enveloppe `RLWS` version 2 ; un cache antérieur est rejeté
avant décodage et la suspension normale utilise son journal vérifié. Les bonus
historiques sans affixes gardent leur représentation et ne sont pas renommés.

| Profil | Effet exécuté |
| --- | --- |
| Témoin | Aucun effet spécial |
| Onde d'impact | Zone électrique autour de la cible touchée |
| Décharge circulaire | Exception explicite : zone autour du porteur |
| Conduction | Zone d'impact favorisée par l'eau, arrêtée par les murs |
| Brandon | Brûlure temporaire de la cible |
| Suintement caustique | Flaque chimique temporaire sur la case touchée |
| Vol de vie | Rend 50 % des dégâts directs admissibles au porteur, au plus 3 PV par attaque |
| Perforation | Après une touche, trait de 3 cases derrière l'impact, 3 dégâts perforants par occupant admissible |
| Égide d'impact | Des dégâts directs accordent au porteur une protection de 3 dégâts pour le prochain impact sur ses PV |
| Percussion | Une touche sur un survivant tente une poussée d'une case ; deux tours de recharge après l'action |
| Catalyse | Cône depuis le porteur : dégâts et brûlure ; chaque cible déjà brûlante provoque une explosion locale |
| Marquage | Trois touches sur une même cible consomment la marque et déclenchent 6 dégâts cinétiques autour de l'impact, rayon d'une case |
| Ricochet | Le projectile rebondit une fois vers une autre cible proche et accessible |
| Écho différé | La case touchée est frappée pour 2 dégâts cinétiques après les 2 prochaines actions consommant un tour |

### Essayer les nouveaux comportements

- Ricochet : équiper un fusil Ricochet, frapper un mannequin proche d'un autre.
  Une courte traînée de projectile relie l'impact au second mannequin.
  Un seul rebond, sans mémoire de la frappe précédente ; 3 dégâts perforants,
  portée de 4 cases dans ce prototype. Murs, coins fermés et protections bloquent
  le trajet. Les victimes du tir principal et le porteur ne sont pas retouchés.
- Écho différé : frapper puis attendre deux tours. La case marquée est frappée,
  même si son ancien occupant s'est déplacé. Cet effet reste présent en attendant
  la décision de suppression demandée à l'auteur.
- Les descriptions ont été raccourcies. Les limites de soin et de bouclier
  sont affichées séparément. Paramètres provisoires, aucune distribution en campagne.

Pour **Marquage**, frapper trois fois le même mannequin avec une lame
ou un fusil Marquage. Les petits segments au-dessus de la cible et le compteur
**MARQUAGE : 1/3**, puis **2/3**, annoncent l'explosion au prochain coup réussi.
La marque suit le personnage, pas une case ; le panneau de cible indique aussi
sa durée. Elle expire sans explosion après cinq fins de tour sans nouvelle
charge, celle de l'application comprise : quatre tours restent après une frappe
ordinaire. Une nouvelle charge renouvelle cette durée, sans addition de durées.

Ce sont des **charges de préparation**, pas des dégâts périodiques qui se
cumulent. Le troisième coup supprime la marque et fait éclater des fragments
anguleux ivoire/ocre, distincts du feu et de la foudre. Aucun feu ni champ au sol
n'est laissé. La cible touchée et les occupants accessibles dans un rayon d'une
case prennent les dégâts secondaires, sauf le porteur ; murs et protections
spatiales restent respectés. Les valeurs sont des paramètres d'essai.

Une attaque ajoute au plus une charge à une cible, même si elle touche plusieurs
personnages ou possède plusieurs propriétés Marquage. Un raté ne charge pas ;
une touche absorbée compte. Le coup mortel final peut déclencher l'explosion,
mais tuer avant le seuil n'en crée pas. Les dégâts secondaires ne chargent pas
d'autres marques. Changer de cible, d'emplacement ou d'exemplaire ne remet pas
le compteur à zéro : le prototype utilise une marque commune par cible. Les
autres statuts sont conservés. Aucun effet ni nouvel objet n'est distribué en
campagne par ce test.

Pour Catalyse, frapper le groupe de mannequins au nord-est. Le cône part du
personnage vers la cible : 7 cases de portée, demi-largeur maximale 2, 2 dégâts
thermiques et application de brûlure dans sa zone. Refrapper le groupe allumé
produit une explosion de 6 dégâts thermiques (rayon 1) sur **chacune** des cibles
qui brûlaient avant l'attaque. Brandon permet aussi de préparer une seule cible.

Les brûlures créées pendant l'attaque n'alimentent pas aussitôt des explosions.
Les brûlures préexistantes sont renouvelées sans cumul, pas consommées.
Toutes les explosions déjà prévues ont lieu, y compris si le premier impact
ou une explosion voisine tue une cible préparée. Les explosions peuvent donc
se recouper ; leurs dégâts sont regroupés en un seul total affiché par victime.
Le porteur est épargné ; murs et zones protégées sont respectés. Aucun filtre
allié/neutre supplémentaire n'est introduit pour la campagne par ce prototype.

Des langues de feu orientées suivent le cône, et les explosions locales restent
distinctes. Les brûlures gardent leur icône et leur animation tant qu'elles durent.
Ni le cône ni ses explosions ne créent de feu persistant au sol. Les dégâts
intrinsèques de l'arme ne changent pas.

Percussion conserve les dégâts de base, sans dégâts de collision. Sa force
d'essai est 4 ; les mannequins `X` pèsent 10 kg, le `L` 80 kg, le `A` est ancré.
Ce sont des paramètres de banc d'essai, pas les masses d'un bestiaire définitif.
Murs, portes fermées, occupants, coins fermés et zones protégées arrêtent le
déplacement. La direction prolonge la frappe, en mêlée comme à distance.
Une seule cible est poussée par attaque ; un raté ou une cible tuée ne déclenche
pas la tentative. En revanche une cible résistante ou une destination bloquée
consomme la disponibilité. Les frappes ordinaires restent possibles en recharge.

L'état **PERCUSSION PRÊTE / RECHARGE** est visible sous l'arme active. Dans ce
prototype, les deux tours de recharge sont partagés entre les armes Percussion
du porteur : changer d'emplacement ou d'exemplaire ne réinitialise rien.
Les menus et clics de sélection ne font pas passer le temps. Le compte ne se
renouvelle pas lorsque le joueur frappe pendant la recharge. Des chevrons ambre
matérialisent l'impulsion ; une barre transversale distingue une poussée arrêtée.
La cible sélectionnée reste la même, et les dégâts affichés suivent sa position.

L'Égide se consomme sur le prochain impact infligeant des dégâts après armure
et résistances, même si elle l'absorbe entièrement. Le reliquat de protection
est perdu. Sans impact, elle expire après trois fins de tour, **celle de son
acquisition comprise** : deux tours restent après une attaque ordinaire.
Refrapper pendant son activité ne renouvelle ni sa durée ni sa puissance.
Un impact mixte n'utilise qu'une protection ; un raté ou une absorption totale
par les défenses ordinaires ne la consomme pas. Les dégâts périodiques peuvent
la consommer. La durabilité des composants n'est pas protégée par cette brique.
Un petit bouclier bleu marque son activité ; son contour se fracture à la
consommation avec le montant réellement bloqué. Pour essayer le fusil, tirer
sur le dispositif depuis deux cases, puis s'approcher d'une case.
Les chiffres sont des paramètres de test, pas un équilibrage de campagne.

La perforation conserve la direction de la frappe, y compris oblique. Elle ne
reparcourt pas le trajet entre le porteur et la cible et ne retouche aucune
victime de l'attaque principale, même repoussée dans le trait. Un coup mortel
conserve son point d'impact. Murs, portes fermées, coins fermés et terrain
protégé arrêtent le trait ; l'eau et les portes ouvertes le laissent passer.
Une attaque de zone ne produit qu'un trait et les dégâts secondaires ne
déclenchent ni un nouveau trait ni le vol de vie. La valeur 3 n'est pas un
équilibrage de campagne. Sur la rangée de départ, frapper le premier mannequin
permet d'atteindre les deux mannequins derrière lui ; le premier n'est pas
frappé une seconde fois.

Le trait ivoire possède une pointe mobile et une courte traînée, distinctes des
arcs électriques. Il reste derrière les personnages et uniquement sur les
cases perçues. En animations réduites, il devient un bref tracé stable.

Le vol de vie est proportionnel aux dégâts réellement infligés et n'ajoute
aucun dégât à la cible. Le total admissible de l'attaque est multiplié par
50 %, arrondi à l'entier inférieur, puis plafonné à 3 PV et aux PV manquants.
Par exemple, 4 dégâts rendent 2 PV ; 3 dégâts rendent 1 PV. Les dégâts qui
dépasseraient les PV restants de la victime ne sont pas comptés.
Un raté, une absorption totale ou des PV déjà pleins ne produisent pas de soin.
Une attaque multicible partage un seul plafond ; réactions et dégâts secondaires
ne le déclenchent pas. Un coup mortel admissible peut soigner le porteur s'il
survit à la résolution. Le texte indique uniquement les PV réellement rendus.
Les petites croix ascendantes encadrent le porteur, sans dessiner un faux
transfert depuis la victime. Elles sont fixes en mode d'animations réduites.

Les mannequins portent explicitement le marqueur `lab:healing_target` pour
ce seul essai. Il n'existe aucune admission implicite de tous les personnages,
du décor ou des invocations. Les cibles et paramètres de campagne restent à
définir avant distribution ; 50 % et le plafond de 3 PV sont des valeurs de test, sans équilibrage
de fréquence ou de vitesse d'arme.

Les profils offensifs d'impact suivent la même règle en mêlée et à distance.
Le porteur est exclu des dégâts des trois profils électriques d'essai.
Les trois profils électriques montrent maintenant des éclairs ramifiés et
irréguliers : bleus à l'impact et autour du porteur, cyan pour la conduction.
Leurs connexions suivent les cases réellement propagées, pas un cadre de zone.
La brûlure montre des flammes pleines ; l'acide des gouttes lourdes, des
éclaboussures arrondies et, pour le suintement, une flaque avec des bulles.
Tous emploient le même langage terminal, des palettes de trois couleurs et
une extinction progressive. Les glyphes des personnages restent visibles.
L'option de réduction d'animations remplace les nouvelles ondes par un bref
repère stable, sans modifier les dégâts.

Le rendu terminal anime l'intérieur des cases sur une grille de 16 × 16 pixels :
éclairs cassés et fourchus, flammes à cœur chaud, langues irrégulières et braises
montantes, puis gouttes qui tombent, s'écrasent et éclaboussent. Seul le profil
caustique dessine l'étalement d'une flaque ; la corrosion ne crée pas de faux
danger au sol. Les mouvements sont bornés à leurs cases visibles, derrière les
personnages et leurs marqueurs. Ils utilisent le temps de présentation, jamais
le hasard de la partie, et restent figés avec l'option de réduction d'animations.
Les branches électriques sont un habillage local des cases déjà perçues, pas
une prédiction de cibles supplémentaires ou un changement de propagation.

## Durées, coexistence et lecture des dégâts

- L'animation des champs au sol suit leur **durée réelle en tours** : flammes,
  flaques bouillonnantes et arcs électriques restent visibles et animés même
  si le joueur attend sans agir. Le dernier tour consommé les retire aussitôt.
  La présence vient de l'état courant du monde, jamais de la file des flashes ;
  plusieurs champs distincts sur une case restent rendus ensemble.
- Les états élémentaires attachés à un personnage (brûlure, corrosion) suivent
  ce personnage jusqu'à leur disparition, sans créer de faux effet au sol.
  Les explosions sans champ persistant, dont Catalyse, restent ponctuelles.
  En animations réduites, le motif reste fixe mais visible pendant toute la
  durée de l'effet. Aucune animation ne révèle une case hors de la perception.
- Un même effet ne se cumule pas : l'identité est celle de l'effet, pas celle
  de l'arme ou de l'attaquant. Brûlure/corrosion renouvellent leur durée, sans
  charges supplémentaires ; une flaque du même type sur la même case reste
  une seule flaque, dont les durées ne s'additionnent pas.
- Tous les effets différents peuvent coexister : **aucune limite de deux**.
  Leurs dégâts et leurs échéances restent indépendants. Les contrôles qui
  refusent déjà le renouvellement conservent cette règle.
- Chaque état temporaire a une petite icône au-dessus du personnage. Une
  exposition au sol possède son propre indicateur ; il disparaît lorsqu'on
  quitte la flaque, contrairement à un état attaché à la cible. Les noms et
  durées apparaissent aussi dans le panneau de cible, dans l'espace disponible.
- Le nombre flottant est le total des dégâts réellement reçus par une cible
  pendant la résolution. Coup direct et zone secondaire ne donnent donc plus
  deux nombres sur le même personnage. Le journal conserve les paquets détaillés.
- Les nouvelles parties (génération 104) et le laboratoire appliquent la
  corrosion non cumulable. Les parties historiques conservent leurs règles de
  génération pour garder un rejeu exact ; elles ne sont pas converties en silence.

## Isolation

Le client conserve l'état du menu principal en mémoire pendant les essais.
Il restaure cet état en quittant le laboratoire. Les options de commandes et
d'affichage restent communes. Les commandes de sauvegarde et les points de
récupération sont bloqués dans le laboratoire ; quitter sa fenêtre ne remplace
ni ne supprime les sauvegardes normales. Les règles et objets `lab:*` ne sont
jamais enregistrés dans le contenu de campagne. Les mémoires et échéances des
échos sont incluses dans les instantanés moteur. Un ancien instantané binaire
est ignoré lorsque l'empreinte de compilation diffère : le rejeu vérifié existant
reste la voie de reprise. Les empreintes des états sans échos restent inchangées.

## Périmètre et suite

C'est un banc d'essai des primitives déjà exécutables, pas l'intégration de
tous les effets envisagés. Les douze fiches prioritaires disposent maintenant
de profils exécutables, dont Ricochet et Écho différé (maintien à décider). Le témoin et l'Onde
d'impact portent le total à quatorze profils, sans être deux nouvelles fiches.
Les futures propriétés spéciales par
exemplaire, leurs délais/charges et le service d'amélioration PNJ restent à
développer. Le laboratoire recevra leurs variantes lorsqu'elles fonctionneront
réellement. Pour l'instant, les effets sont ceux des définitions d'armes de test ;
seul le bonus statistique est une propriété d'exemplaire.

Implémentation : `src/effects_lab.rs`, entrée et garde-fous dans `src/ascii_app.rs`.
Diagnostics de rendu : `--ui-cold-effects-lab`, `--ui-cold-effects-lab-inventory`,
`--ui-cold-effects-lab-impact`, `--ui-cold-effects-lab-pause`, suivis d'un dossier
de capture neuf. Ces commandes sont réservées aux compilations de développement ;
le bouton du menu est également disponible en version release.

Diagnostics supplémentaires : `--ui-cold-lab-fx-impact`, `bearer`, `conduction`,
`caustic`, `burning`, `healing`, `piercing`, `aegis`, `percussion`, `catalysis`, `statuses` (chaque suffixe après `--ui-cold-lab-fx-`).
Pour Ricochet : `ricochet`, `ricochet-late`, `ricochet-reduced`
et `ricochet-inventory`. Pour l'Écho : `echo-mark`, `echo-last`, `echo-burst`,
`echo-expired`, `echo-empty`, `echo-empty-reduced`, `echo-reduced`, `echo-inventory`.
Les textes exacts à soumettre à l'auteur sont dans `TEXTES_EFFETS_A_REFORMULER.md`.
Pour Fracture : `--ui-cold-lab-fx-fracture` montre la détonation ; les suffixes
`-one`, `-two`, `-expired`, `-ranged`, `-reduced` et `-inventory` montrent les
charges, leur expiration, le tir, le rendu fixe et la fiche.
Les diagnostics `lab-fx-*` acceptent également `-960` (960 × 540) et `-1080`
(1920 × 1080, interface à 125 %) en dernier suffixe, sans modifier les réglages
du joueur, par exemple `--ui-cold-lab-fx-fracture-two-960`.
Pour les champs persistants : `--ui-cold-lab-fx-ground-fire`, `ground-acid`,
`ground-electric`, `ground-mixed` montrent de vrais champs sur des cases vides,
après expiration de tous les flashes. Les suffixes `-last`, `-expired` et
`-reduced` vérifient le dernier tour, la disparition et le motif fixe.
Ces configurations de diagnostic n'ajoutent aucune arme au laboratoire jouable.
Pour Catalyse : `catalysis-ready` montre une cible préparée avec Brandon,
`catalysis-empty` le cône sans cible déjà brûlante, `catalysis-multi` les explosions
multiples, `catalysis-inventory` la fiche,
`catalysis-early`, `catalysis-late`, `catalysis-embers` et `catalysis-reduced` les phases visuelles.
`catalysis-preview`, `catalysis-preview-burning` et `catalysis-preview-empty`
vérifient l'ouverture par F, les explosions prévues et une confirmation invalide.
`bonuses-inventory` et `bonuses-armor-inventory` montrent de vrais exemplaires
équipés avec bonus ; le tirage de diagnostic est fixe, celui des essais jouables varie.
`bonuses-three-inventory` sélectionne une arme portant trois affixes chiffrés ;
les suffixes `-960` et `-1080` permettent de vérifier les colonnes et la fiche.
La liste est regroupée en **Équipement / Sac** : arme **EN MAIN**, autres armes
et protection **ÉQUIPÉ** avec leur emplacement. La sélection reste attachée
au même exemplaire après équipement ou tri. `Tab` change de catégorie et `T`
change le tri, selon les commandes configurées ; clic et molette restent disponibles.
`inventory-bag` vérifie la fin du sac et son titre de section après défilement.
`inventory-held` simule un appui maintenu avec le délai et la cadence réels,
puis son relâchement ; la sélection doit rester sur le treizième objet sans
modification de la partie. La capture contrôle le suivi visuel de cette sélection.
`inventory-dense` montre une arme avec trois bonus et une description d'effet longue,
notamment en `-960`, pour vérifier les espacements et les retours à la ligne.
La fiche ne répète plus les emplacements d'arme sous les statistiques.
`--ui-cold-lab-fx-frost` (aussi `-late`, `-reduced`) présente des cristaux facettés.
C'est un **aperçu graphique seulement**, sans arme de glace, type de dégâts froid
ou statut de gel. Le diagnostic vérifie que l'animation ne modifie pas le monde.
Pour Percussion, les suffixes `-ready`, `-blocked`, `-heavy`, `-fixed`, `-oblique`,
`-inventory` et `-reduced` vérifient respectivement disponibilité, mur, masse,
ancrage, tir oblique, fiche et animations réduites.
Pour l'Égide, `aegis-status` montre le marqueur après l'animation et
`aegis-block` montre la consommation par le dispositif d'essai ;
`aegis-inventory` présente la fiche et `aegis-reduced` l'animation réduite.
Ajouter `-early` ou `-late` à un profil animé pour contrôler plusieurs phases.
`--ui-cold-lab-fx-healing-inventory` ouvre directement la fiche de la lame
de vol de vie ; dégâts de base, pourcentage et plafond restent séparés.
Le même suffixe `-inventory` fonctionne pour la perforation. Le diagnostic
`--ui-cold-lab-fx-piercing-oblique` vérifie une frappe de fusil non cardinale.
Le suffixe `-reduced` contrôle la version stable. Le rendu pixel animé est isolé
dans `src/terminal_fx.rs` et `src/terminal_elements.rs` ; ses instants et connexions viennent de
`src/visual_effects.rs`, sans dépendance aux règles de combat.

Ciblage à la souris : `--ui-cold-lab-target`, `--ui-cold-lab-target-cleared`,
`--ui-cold-lab-target-960` et `--ui-cold-lab-target-1080` vérifient les clics via
la caméra réelle et la transformation des coordonnées. Chaque diagnostic
sélectionne, désélectionne et change de cible sans tour ni attaque, y compris
si l'attaque est réattribuée au clic gauche. La capture finale montre soit la
cible sélectionnée, soit l'absence de cible. La variante 1080 utilise une
échelle d'interface de 125 % ; aucun réglage du joueur n'est chargé ou modifié.
