# Intégration jouable des compétences de l’agent

## État courant — intégration générale

Le catalogue compte **94 compétences : 68 actives et 26 passives**. Les 68
actives disposent désormais d’un chemin d’exécution et les passifs sont
raccordés à leurs effets. Les effets mécaniques et les rendus déjà validés ont
été implémentés et les contrôles ciblés ci-dessous sont réussis.
L’ancien état « sept actives raccordées, 61 restantes » est donc dépassé.
Une validation antérieure de texte, de chiffres ou de dessin conserve sa portée :
ce raccordement ne constitue aucune nouvelle décision d’équilibrage.

### Validation

Contrôle du 16 octobre 2026 :

- **945 tests moteur**, **224 tests ciblés d’interface** et **39 tests du laboratoire**
  réussis. Ces groupes sont annoncés séparément, sans additionner leurs éventuels recouvrements.
- Les quatre tests de migration d’une reprise agent v14 passent : cache historique
  exact accepté ; modification des règles, de l’état, du format ou absence de cache refusée.
- `cargo fmt --all --check` et `git diff --check` réussis.
- Binaire debug reconstruit sans avertissement. Vérification du binaire en mode release
  réussie ; aucun exécutable release n’a été construit pour ce contrôle.
- Cinq captures natives à 1 280 × 800 et cases de 64 px vérifient la fenêtre et les
  variantes, les mines, Gel, les transformations, les drones et Furtivité. La session
  est isolée ; aucune sauvegarde de campagne n’est lue, écrite ou supprimée.
  L’hologramme allié est désormais classé selon sa relation réelle dans les capteurs.

**La suite globale n’est pas entièrement verte.** Son dernier passage complet avait
944 tests moteur réussis et, côté client, 856 réussites, 37 échecs et 15 tests ignorés.
Depuis ce passage, les trois attentes périmées du laboratoire ont été adaptées et ses
39 tests passent : ancien raccourci fixe et ancien feu générique remplacés par les
chemins actuels. Les 34 autres échecs ne sont pas corrigés ici. Trois attentes périmées
sont démontrées dans HEAD ; les 31 autres restent non attribuées à une référence antérieure.
Aucune empreinte attendue n’a été remplacée pour masquer un échec. Le passage complet
n’a pas été répété après les derniers correctifs ciblés.
Les noms et les résultats sont conservés dans
`docs/reviews/refonte-agent-2026-10-06/skill-playable-review-01/validation.json`, bloc
`general_integration`.

La suppression des captures et journaux temporaires de ce contrôle a été tentée
avec la permission du joueur, mais le contrôle automatique l’a refusée :
`blocked by policy`, sans autre motif fourni. Aucun fichier n’a été supprimé et
aucun contournement n’a été tenté. Les assets utilisés et le binaire sont conservés.

### Correctifs après essais du joueur — 16 octobre 2026

**Blizzard — FR05-HITS02.** Le nombre fixe d’impacts diluait les coups parmi
les cases vides : au rang 10, trois cases seulement étaient tirées parmi 81.
Le joueur a remplacé cette règle par un jet indépendant de **50 % par case et
par tour**, sans plafond global de victimes. Le jet concerne toutes les cases
admissibles, occupées ou non. Les obstacles et cases protégées restent exclus,
y compris si le terrain change après le lancement. Les dégâts d’un impact
s’appliquent avant le jet de Gel ; un échec de Gel n’annule pas les dégâts.
Coût, avertissement, durée, portée, rayon et dégressivité restent inchangés.
La fiche indique « Chance d’impact par case : 50 % ». L’ancien paramètre de quota
reste disponible pour les prototypes historiques et est masqué dans la fiche jouable.
Accord : `docs/reviews/refonte-agent-2026-10-06/blizzard-impact-acceptance-02.json`.

**Réanimation — OC07.** La marque persistante, l’apparition du relevé et le crâne
allié réutilisent les six images déjà acceptées de l’aperçu. Le nouvel identifiant
du relevé reçoit le glyphe et l’orientation de la créature d’origine avant le
nettoyage de son ancienne présentation. Le pourcentage de PV approuvé est réappliqué
après l’initialisation du corps, qui le remplaçait sinon par les PV complets.
Des tests vérifient la marque sur le vivant, le relevé après sa mort, son attaque,
sa présence plusieurs tours, son expiration et la conservation du dessin.

La sélection automatique pouvait ensuite reprendre cet allié et permettre à F de
le tuer de nouveau. Elle ignore désormais les compagnons et alliés pour ce choix
automatique. Une cible alliée choisie explicitement conserve le comportement
existant ; les cibles neutres, Tab et l’inspection ne changent pas.

**Marqueurs.** Les icônes compactes d’affaiblissement ne sont plus recouvertes
par un badge générique de durée. Les autres badges utilisent une rangée séparée.
La capture native confirme trois icônes, la marque de Réanimation et son allié,
sans ajouter de nouveau dessin.

**Contagion — AF06.** Elle reste le passif explicitement approuvé : transmission
à la mort d’un porteur affaibli. Elle ne se place pas dans la barre des actives.
Les tests vérifient le glisser-déposer des 68 actives, le refus des 26 passives
et l’apprentissage de Contagion par son icône.

Les contrôles de ce correctif passent : **949 tests moteur, 231 tests ciblés
client et 53 tests du menu contextuel**. Le test du vrai laboratoire au rang 10 touche cinq des sept cibles au
premier tour d’impacts, dont deux sans Gel. Les résultats de contexte, compilation
et capture sont consignés dans le bloc `regression_repairs` du bilan JSON.
Le bilan de la suite globale historique ci-dessus reste distinct ; cette suite
complète n’a pas été rejouée pour ce correctif.

La suppression des six nouveaux journaux et capture de ce correctif (435 413 octets)
a été refusée par la vérification automatique : `blocked by policy`, sans autre motif.
Ils restent en place ; aucun nouvel essai ni contournement n’a été effectué.

### Laboratoire courant

Lancer le binaire avec **`--laboratoire-competences`** ouvre une session isolée,
sans charger ni sauvegarder la campagne. Les 94 compétences y sont apprises au
rang 1 et 1 000 points de compétences permettent d’essayer leurs progressions. Ce laboratoire
est le point d’entrée courant pour tester l’ensemble de l’intégration. Les anciens
lanceurs CMD cités plus bas documentent les essais historiques et ne constituent
pas des chemins d’accès courants garantis.

### Périmètre raccordé

Les techniques d’armes, leurs passifs et buffs passent par les armes réellement
actives, leurs ressources, jets et défenses. Les sorts, zones persistantes,
contrôles, soins et transformations utilisent leurs conditions acceptées. Les
actions propres aux formes apparaissent dans les menus et la barre temporaire ;
l’attaque ordinaire sélectionne leur action par défaut, puis la barre humaine
retrouve ses affectations au retour.

Les cinq drones et leurs utilitaires sont raccordés, ainsi que les mines,
grenades, variantes élémentaires et résidus persistants. L’Éclaireur et la Mule
restent chacun limités à un exemplaire. Les réglages provisoires de capacité
commune et de provocation du Gardien restent ceux approuvés pour ce premier essai,
détaillés dans la section D58 / D59 / D66.

La barre, le bouton Utiliser et le clic droit partagent le précontrôle et la
commande du moteur. Les refus de cible ou de ressources conservent leur absence
de dépense ; une action effectivement commencée peut être interrompue par les
dangers ou les réactions du trajet. Les règles AGENT-AIM01 / AGENT-AIM02,
AGENT-MIXED01 et AGENT-CADENCE01 restent applicables.

Les états persistants de ces compétences utilisent maintenant la reprise
**RLWS v15**. La lecture des reprises **v12 à v14** reste prise en charge par le
chemin de compatibilité. L’état de l’agent, les effets différés et les objets
persistants suivent les sauvegardes et les changements de zone ; l’état des
zones inactives participe également à l’empreinte complète.

Une ancienne suspension de l’agent avec cache moteur v14 peut recevoir les nouvelles
définitions intégrées de statuts seulement après vérification exacte de ses anciennes
règles et de son état complet. Les définitions modifiées par le contenu ou les mods
ne sont pas retirées pour fabriquer une correspondance. Ce chemin ne rejoue pas
une sauvegarde invalide, ne consomme ni hasard ni ressource et ne modifie pas le fichier
d’entrée. La politique habituelle de refus des autres incompatibilités est conservée.

### Précisions approuvées dans les derniers échanges

- **HL01 Barrage** : le joueur a choisi un carré de sélection **5 × 5**, centré
  sur le curseur. Les priorités de cibles et le fonctionnement déjà acceptés
  restent conservés.
- **SF00 Furtivité** : la détection part du quart du rayon de vision habituel de
  l’observateur, puis applique SF02, avec un minimum d’une case. Le joueur a
  approuvé ce réglage provisoire pour l’intégration. Si l’énergie ne suffit pas
  au coût du prochain déplacement, la furtivité s’arrête et le pas normal a lieu.
  Le bonus de l’attaque directe reste celui validé : +5 % par rang effectif,
  donc +50 % au rang 10, avec progression au-delà par les rangs effectifs.

Les choix HL01 et SF00 ci-dessus ont été approuvés explicitement par le joueur
pendant la session d’intégration du 16 octobre 2026. Ils complètent les décisions
archivées sans modifier les autres parties de ces compétences.

## Socle et lots historiques

Les sections suivantes conservent les règles et références utiles des premiers
lots. Leurs mentions de raccordements futurs ou d’anciens essais décrivent leur
étape d’origine ; l’état courant et le laboratoire indiqués ci-dessus priment.

### Ciblage au survol et clic de confirmation — AGENT-AIM01 / AGENT-AIM02

Accord du joueur : toutes les compétences ciblées partagent une prévisualisation
qui suit immédiatement la souris, sans clic de déplacement. Le clavier déplace
le même curseur et conserve la priorité tant que la souris reste immobile.
Le passage sur une commande du HUD garde la dernière case visée. Les cases
non visibles ne peuvent pas être sélectionnées par ce suivi.

Déplacer l'aperçu ne dépense ni tour ni ressource. La portée, les obstacles et
les conditions de cible restent ceux de la compétence. AGENT-AIM02 ajoute,
à la demande du joueur, le clic gauche sur la case visée pour confirmer,
comme l'action Attaquer ou son bouton. Un clic sur une cible invalide, une case
cachée ou hors de la carte ne lance pas l'attaque précédente. Maintenir le bouton
ne répète pas l'action. Échap conserve son annulation existante.
Le ciblage commun transmet aussi ce suivi aux attaques d'arme avec visée.

### Boule de feu — FE05 / AGENT-PLAY05

FE05 utilise le coût, le rang, le niveau actuel, la classe, la Volonté et
Canalisation du profil partagé avec sa fiche. Une visée visible à portée peut
être libre. Le projectile s'arrête au premier personnage ou obstacle rencontré ;
contre un obstacle, l'explosion part de la dernière case libre avant le choc.
Les coins fermés arrêtent également le trajet. L'aperçu calcule ce même centre
réel. Le disque, sa croissance et sa portée restent ceux de ZONE-GROWTH02.
Les murs, les cases protégées et les cellules non visibles sont exclus.
SELF-DMG01 protège le joueur ; les autres acteurs, dont ses alliés et drones,
peuvent être touchés. Aucun nouveau coût d'arme ou de munition n'est ajouté.

Les dégâts directs conservent la dégressivité AOE-FALLOFF01, un jet critique
par victime et un seul arrondi après les protections. La distance du disque
reste fractionnaire jusqu'à cet arrondi. Le coût est payé une fois ; la visée,
l'annulation et les refus ne changent ni les ressources, ni le tour, ni le hasard.
Le sort peut être relancé à l'action suivante selon AGENT-CADENCE01.

Le jet de brûlure est séparé pour chaque survivant. La puissance est celle du
lancement, sans critique ni dégressivité de l'explosion. Brûlure conserve son
identifiant partagé `core:burning` pour les autres systèmes de statut. Elle
applique son premier tic dès la pose : ce tic est inclus dans ses trois tics,
sans second tic automatique pendant le même tour. Elle ne se cumule pas : la
dernière application réussie remplace la puissance et redémarre la durée, même
si elle est plus faible. Les résistances actuelles de la victime
s'appliquent à chaque tic. Les profils d'importance des futurs boss restent
à raccorder lorsque ces profils existent dans le bestiaire.

`StatusInstance` peut désormais conserver cette puissance et son tour de
pose. Cette étape avait introduit la reprise RLWS v14 ; l’intégration générale
utilise désormais v15 et conserve la lecture v12 à v14, y compris avec des
statuts non vides. Le format JSON accepte
l'absence de cette nouvelle donnée. Les acteurs, leurs statuts et cette
puissance voyagent et se sauvegardent ensemble. Le Debug historique des
statuts sans puissance est conservé pour les empreintes de reprise.

Le rendu reprend FE05-A07 : projectile de 0,35 seconde, explosion de
0,55 seconde et découpe sur les obstacles. L'animation instantanée s'achève
sans attendre le prochain tour. Les flammes de Brûlure utilisent l'atlas et
les deux plans déjà validés, attachés au corps de la cible et à son recul.
Le recul et l'apparition des flammes attendent l'arrivée du projectile.
Les dégâts de feu ne créent pas de sang.

Ce lot avait vérifié les chemins de la barre (clic et touche), Utiliser et
clic droit. Son option d’essai historique était `--essai-boule-de-feu` ; le point
d’entrée courant est le laboratoire général indiqué plus haut. Aucun nouveau
lanceur CMD n’est créé. La page et le WebP courants sont remplacés ; les captures
brutes restent recensées dans le manifeste de nettoyage existant.

Références : FE05-03 et ses révisions numériques du catalogue ; FE05-A07 ;
INTERACT-STATUS02 ; INTERACT-POWER01 ; AOE-FALLOFF01 ; SELF-DMG01.

### Déflagration occulte — A02 / AGENT-PLAY04

Le centre peut être une case libre visible à portée, avec ligne de vue. Le
carré commence à 3 × 3 cases et suit ZONE-GROWTH02 au rang effectif actuel,
avec la portée déjà validée. Les obstacles et les coins fermés arrêtent la
zone. Les victimes correspondent aux cases visibles de la prévisualisation ;
les cases protégées n'y figurent pas. Le lanceur conserve SELF-DMG01 : il ne
subit pas ses propres dégâts. L'accord individuel A02 reste applicable aux
alliés, neutres et drones présents dans la zone.

Le coût et les dégâts emploient le même profil courant/prochain rang que la
fiche : rang, niveau actuel, Volonté, classe et Canalisation déjà raccordée.
Chaque victime reçoit son propre jet de critique. La dégressivité carrée
AOE-FALLOFF01 conserve un facteur rationnel jusqu'au dernier arrondi après
les protections : plein au centre, moitié sur la couronne extérieure.
Une cloison retire ses cases et leur ombre, sans renforcer le reste.

Prévisualiser, annuler ou refuser une cible ne consomme rien. Lancer sur une
case libre est permis ; sans victime, le coût est payé et le hasard n'est pas
consommé. Un nouvel usage est disponible à l'action suivante, conformément à
AGENT-CADENCE01. Les ressources, événements et commandes de sauvegarde existants
sont réutilisés ; aucun champ persistant n'est ajouté.

Le rendu reprend l'atlas A02 accepté et le violet franc A04. La première image
passe derrière les acteurs ; les suivantes passent devant. L'explosion s'achève
en 0,55 seconde, sans rester jusqu'au tour suivant. Le dessin peut déborder sur
le sol libre voisin, mais coupe sur les obstacles ; seuls les effets logiques
de la prévisualisation font des dégâts. L'occulte n'ajoute aucun sang.

`Voir_deflagration_occulte.cmd` désignait l’essai isolé de ce lot. Le même aperçu
HTML et le même WebP étaient remplacés pour ce lot. Les captures temporaires
restent recensées pour nettoyage, sans dupliquer une nouvelle galerie.

La permission de suppression a été renouvelée explicitement le 16 octobre.
La nouvelle tentative a encore été rejetée avant exécution par le contrôle
automatique (« blocked by policy »). Aucune suppression n'est enregistrée.

### Frappe de rupture — B01

Une seule frappe avec l'arme de mêlée réellement active, choisie dans la paire
à une main si nécessaire. Le coefficient et le coût suivent les rangs déjà
validés ; les défenses, jets, effets, consommation et récupération de l'arme
restent ceux du moteur. Prévisualiser ou refuser une cible ne dépense ni tour,
ni énergie, ni hasard. La confirmation exécute l'attaque et ferme l'aperçu.
Le trait de coupe et l'impact B01-A01 sont partagés avec le laboratoire validé.

### Traqueur, Artilleur et Gardien — D58 / D59 / D66

L'invocation choisit une case libre voisine, diagonales comprises. Les trois
modèles créent de vrais acteurs persistants, alliés et commandables par les
ordres existants. Ils ne frappent pas dès leur naissance. Le Traqueur poursuit
et attaque au contact ; l'Artilleur utilise son tir et la ligne de vue ; le
Gardien provoque ou attaque au contact. Les drones n'utilisent pas les munitions
du joueur et ne paient pas d'entretien énergétique par tour.

Leur niveau suit celui du joueur, selon ATTR03/PETS-EQ : attributs propres,
PV, batterie et bonus de rang/Intelligence déjà acceptés. Le vrai gain de niveau
restaure PV et batterie ; un simple changement de rang ou de modificateur
redimensionne les jauges sans soin gratuit. L’Éclaireur et la Mule sont désormais
raccordés avec leurs fonctions propres de vision et de chargement ; chacun est
limité à un exemplaire.

La capacité commune provisoire DRONE-ARMY01-NUM01 commence à quatre et gagne
une place tous les cinq rangs appris dans la branche Drones. Les bonus d'objet
ne comptent pas. Plusieurs Traqueurs, Artilleurs et Gardiens partagent ces places.
Le compteur du HUD reprend cette capacité plutôt que l'ancien plafond fixe.

D66-TAUNT01-NUM01, validé pour le premier essai : carré 5 × 5 limité par les
obstacles, deux phases ennemies et un intervalle de quatre tours entre deux
provocations. Provoquer remplace l'attaque du Gardien pour sa phase. Les candidats
simultanés sont comparés avant tout déplacement : distance de grille, puis ID
stable en cas d'égalité. Une cible conserve son premier Gardien et sa durée.
La disparition de la source supprime l'effet sans transfert automatique.
Les attaques et déplacements ennemis restent soumis aux obstacles ordinaires.

Le statut source/durée et le compteur interne utilisent les champs existants
et survivent à la restauration ; aucun nouveau champ de sauvegarde n'est ajouté.
Le marqueur D66-TAUNT-I01 sans fond suit chaque cible, avec le petit recul négatif
commun. L'impulsion D66-TAUNT-A01 passe derrière les personnages. Les corps
D58-BODY01, D59-BODY03 et D66-SIZE01 sont les exports natifs déjà approuvés.
La fiche du Gardien décrit la provocation à la place de l'ancienne protection.

`Voir_frappe_rupture.cmd` et `Voir_drones_agent.cmd` étaient les lanceurs des
premiers essais isolés, sans charger ou écrire la campagne. La page partagée `apercu-animations.html`
est réutilisée ; aucun nouveau dossier de galerie n'est nécessaire.

Le nettoyage des fichiers temporaires reste bloqué par le contrôle automatique
d'approbation (« blocked by policy »). Le manifeste
`agent-skill-bar-cleanup.json` conserve les chemins et tailles ; aucune suppression
n'est annoncée comme réalisée. Les PDB et futurs exports temporaires sont ignorés
par Git ; l'historique et les commits ne sont pas modifiés.


## Lot AGENT-PLAY01 · Trait arcanique (A01)

Historique : après la fin des aperçus d’animations, ce premier lot a commencé
l’intégration jouable par A01, en réutilisant les règles et dessins acceptés.
L’intégration générale décrite en tête de document lui succède ; l’équilibrage
reste ajustable.

Ce premier lot raccorde **A01 Trait arcanique** au moteur, au ciblage, au
menu rapide et au bouton Utiliser de la nouvelle fenêtre de compétences.
Une cible hostile visible, sans obstacle et dans la portée est nécessaire.
Le sort ne requiert pas d’arme à distance et ne fait pas de jet de précision
d’arme. Les refus et les aperçus ne dépensent ni énergie, ni tour, ni hasard.

Les dégâts utilisent le rang actuel, le niveau actuel du personnage, la
Volonté et le modificateur de classe. Le critique conserve 5 % et ×2.
La résistance occulte est appliquée avant l’arrondi final. Les coûts
appliquent le modificateur de classe et Canalisation (A03), puis l’arrondi
supérieur. Les valeurs de la fiche utilisent le même profil que l’exécution.
Les passifs et les bonus dédiés d’équipement ont ensuite été raccordés dans
l’intégration générale, selon leurs périmètres acceptés.

Le projectile reprend le dessin A01 et le violet franc déjà acceptés.
Le recul de la cible accompagne l’impact ; les dégâts occultes ne produisent
pas de sang. Le temps de présentation ne modifie pas la simulation par tours.

Sous AGENT-CADENCE01, le sort est réutilisable dès l’action suivante. Le tour
du lancement suffit ; aucune action d’attente supplémentaire n’est nécessaire.
Les anciens compteurs de recharge du sort restaurés ne bloquent plus ce lancement.
Les rangs utilisent les champs existants des sauvegardes.
Le lancement est enregistré sous la commande existante `UseTechniqueAt`,
avec l’identifiant réservé `agent:a01`. Aucun changement du format de
sauvegarde n’est nécessaire pour ce lot.

### Essai isolé

`Voir_trait_arcanique.cmd` était le lanceur historique d’un Arcaniste de
laboratoire avec A01 appris et un mannequin hostile, sans modifier la campagne.
Le laboratoire général est désormais le point d’entrée courant.
Utiliser ouvre la visée ; sélectionner la cible puis confirmer avec l’action
Attaquer. Le clic droit propose également les compétences actives apprises.
Chaque lancement fait avancer le tour ; le suivant est disponible tant que
le joueur conserve l’énergie nécessaire.

En campagne, apprendre A01 dans **Énergie occulte** donne accès au même chemin
d’exécution. Les autres compétences apprises apparaissent dans le menu rapide,
selon leur type et leurs conditions d’utilisation. La période où leurs effets
restaient indisponibles est révolue ; le contrôle commun reste en cours.

### Barre de compétences · AGENT-BAR01

Accord du joueur : six emplacements visibles pendant le jeu et dans l’arbre.
Les compétences actives disponibles se déposent par glisser-déposer depuis
leur icône. Déplacer une icône entre deux emplacements les échange ; la sortir
de la barre ou faire un clic droit sur son emplacement retire le raccourci.
Un clic simple sur une icône de l’arbre conserve l’apprentissage existant.
Les raccourcis sont enregistrés avec la suspension, sans modifier la simulation.

Par défaut : **1–6** pour les compétences, **7–9** pour choisir l’arme.
Les anciennes touches d’arme par défaut sont migrées, les touches personnalisées
restent conservées. Préparer un raccourci ne dispense jamais des conditions
d’utilisation propres à la compétence.

AGENT-BAR02 : un bouton de changement d’arme est placé à gauche des six
emplacements, uniquement en jeu. Chaque clic sélectionne le prochain des
trois emplacements occupés, dans l’ordre puis en boucle. À la demande du joueur,
le bouton montre une flèche circulaire sans numéro. Le changement reprend la sélection
existante sans tour ni ressource et annule une visée déjà ouverte ; un bouton
inactif est conservé lorsqu’il n’existe pas d’autre arme équipée.

### Armes mixtes · AGENT-MIXED01

Le joueur a demandé de pouvoir utiliser les compétences de mêlée et de tir
avec une arme à une main de chaque type, puis a choisi explicitement l’attaque
ordinaire automatique selon la distance : mêlée au contact, tir plus loin.

Les deux armes à une main constituent la configuration active. Les compétences
d’arme choisissent le membre de cette paire dont le type convient, même si
l’autre main était sélectionnée. L’arme à deux mains ne participe pas à cette
paire ; lorsqu’elle est sélectionnée, les deux armes à une main restent en réserve.

Le contact correspond aux cases voisines, diagonales comprises. Chaque action
ordinaire utilise une seule arme. La portée, la ligne de vue, les munitions,
les coûts et les contrôles de cible sont ceux de l’arme réellement choisie.
Une pénurie de munitions n’entraîne ni tir gratuit, ni déplacement automatique
au contact. Le clavier, la collision avec un ennemi, le menu clic droit et
le ciblage partagent cette sélection.

Le choix se traduit par l’emplacement explicite de la commande enregistrée.
Les anciens canaux séparés conservent leur fonctionnement et aucun champ de
sauvegarde n’est ajouté pour cette sélection. `Voir_armes_mixtes.cmd` désignait
l’essai historique avec deux distances de cible et les trois emplacements équipés.
Le nouveau catalogue raccordé emploie également cette même sélection de main.

### Compétences temporaires d’équipement · AGENT-GEAR01

Instruction du joueur : une compétence accordée par un objet équipé peut aussi
être placée dans la barre. Elle disparaît dès que sa dernière source équipée
est retirée ; une compétence apprise définitivement reste disponible.

La disponibilité est calculée depuis les vrais emplacements d’équipement,
sans inscrire le don dans les rangs appris et sans dépenser de point.
Le sac et l’emplacement de transmogrification ne constituent pas des sources.
La fenêtre, le menu rapide, la barre et l’exécution consultent cette même règle.
Une autre source équipée ou un apprentissage permanent conserve l’accès.

Le moteur permet de déclarer les dons par modèle dans
`GameRules::equipment_agent_skills`. Cette préparation n’ajoute aucun objet
ni affixe aléatoire au catalogue : les modèles et leurs valeurs restent à revoir
avant leur création. Un don fournit son propre rang ; les sources ne sont pas
additionnées pour créer un nouveau bonus d’équilibrage.

### Suite

Terminer le contrôle commun et corriger ses éventuels écarts avant le test
manuel complet du joueur. La révision conceptuelle évoquée pour Vitalité améliorée
reste une décision de conception séparée ; elle ne remplace pas silencieusement
le passif existant. L’adaptation et l’extension du bestiaire viennent après les
compétences, conformément à la priorité du joueur.

### Références de décisions

- `skill-implementation-sequence-decisions.json`
- `skill-rank-balancing-delegation-decisions.json`
- `current-level-skill-power-numeric-acceptance-decisions.json`
- `direct-spell-damage-bonus-acceptance-decisions.json`
- `critical-hit-rules-acceptance-decisions.json`
- `immediate-persistent-timing-review-validation-01.json`
- `barrage-enemy-targeting-acceptance-decisions.json`
- `stealth-roleplay-cost-and-detection-revision-decisions.json`
- `stealth-step-cost-progression-acceptance-decisions.json`
- `stealth-sneak-attack-progression-acceptance-decisions.json`
- `stealth-no-energy-movement-continuation-decisions.json`
- `review-individual-description-A01-02.json`
- Revue d’animation 32 : palette occulte violette acceptée.

Les fichiers de décision sont conservés dans
`artifacts/refonte-agent-2026-10-06/`.
