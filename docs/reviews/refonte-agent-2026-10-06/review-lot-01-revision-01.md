# Revue des textes et effets de la refonte

6 octobre 2026. Premier lot proposé : réveil, briefing, attributs, énergie,
progression et compétences de départ du Bastion. Tous les nouveaux textes
et tous les effets chiffrés de ce lot attendent l'accord du joueur. Aucun
n'est intégré au contenu jouable.

Les noms des cinq classes, leurs silhouettes et leurs orientations sont
déjà acceptés dans le [cadrage de la refonte](REFONTE_AGENT_CRYOGENISE.md).
Les chiffres ci-dessous sont des valeurs de départ à tester, pas un
équilibrage validé en partie.

## Comment valider

Chaque proposition possède un identifiant stable. Une validation peut
porter sur un texte, sur son effet mécanique ou sur les deux. Par exemple :
« B02 : texte accepté, protection à 20 % au lieu de 25 % ».

Les citations constituent les textes proposés au joueur. Les paramètres,
conditions et cas particuliers restent séparés. Valider ce lot n'autorise
pas les futurs sorts, les autres classes, les quêtes ou de nouveaux rendus
visuels. Les nouveaux effets visuels seront montrés en situation avant
leur intégration.

| Ensemble | Texte | Effet mécanique | Nouveau rendu visuel |
| --- | --- | --- | --- |
| N01 et N02 | À valider | Aucun effet de combat | Non proposé dans ce lot |
| S01 à S06 | À valider | À valider | Non proposé dans ce lot |
| E01 à E03 | À valider | À valider | Non proposé dans ce lot |
| P01 | À valider | À valider | Non proposé dans ce lot |
| B00 à B03 | À valider | À valider | À présenter séparément |

## Réveil et mission

### N01 Réveil

Affichage proposé : introduction après le choix du personnage.

> La cuve s'ouvre. Vous reprenez votre souffle.
>
> Vous êtes dans une base de l'organisation qui vient de vous réveiller.
> Votre équipement vous attend. On vous demande de rejoindre le briefing.

L'identité de ceux qui ont placé l'agent en cryogénie et son passé
personnel restent à définir.

### N02 Briefing

Locuteur proposé : le responsable de mission, sans nom propre fixé.

> Nous avons des renseignements sur une offensive qui se prépare sous terre.
> Il faut l'arrêter avant qu'elle atteigne la surface.
>
> Commencez par les installations sous la base. Éliminez les premières
> menaces et cherchez d'où elles viennent. Nous avons besoin de comprendre
> ce qui se passe avant d'aller plus loin.

Le briefing annonce l'enquête et le nettoyage initial. Il ne révèle ni
l'organisation adverse, ni ses expériences sur les démons, ni leur échec.

## Attributs

Les six noms sont retenus. Leurs formulations et contributions chiffrées
ci-dessous attendent une validation. Les valeurs de départ restent fixes
par classe ; la progression permet ensuite de répartir des points libres.

### S01 Force

> Renforce vos attaques de mêlée.

**Effet proposé :** les dégâts de mêlée varient de 10 % par point de Force
autour de 5. Force 4 donne −10 %, Force 5 ne modifie pas les dégâts,
Force 7 donne +20 %.

Le bonus s'applique aux attaques de mêlée ordinaires et aux techniques
qui les renforcent, avant les défenses de la cible. Il n'augmente pas
automatiquement la portée, le nombre de coups ou les dégâts des sorts.

### S02 Agilité

> Améliore votre précision avec les armes et votre esquive.

**Effet proposé :** chaque point autour de 5 ajoute ou retire 3 points de
précision avec les armes et 3 points d'esquive. Agilité 8 donne +9 à
chacune ; Agilité 4 donne −3.

Les plafonds de probabilité restent appliqués. Cette contribution
n'accélère pas à elle seule toutes les actions du personnage.

### S03 Vitalité

> Augmente vos points de vie et vous aide à tenir sous les coups.

**Effets proposés :**

- PV maximum : 30 + 4 × (Vitalité − 5) + 2 × (niveau − 1), avant équipement
  et implants. Vitalité 7 donne 38 PV au niveau 1.
- Résistance aux interruptions physiques : +5 points de chance par point
  de Vitalité au-dessus de 5 ; contribution négative sous 5. Les autres
  facteurs du coup reçu restent pris en compte.

Gagner des PV maximum ne rend pas les PV manquants. L'armure et la
résistance aux pouvoirs occultes restent des protections distinctes.

### S04 Perception

> Vous repérez les dangers de plus loin et visez mieux à distance.

**Effets proposés :**

- Vision de base : 8 cases à Perception 5, avec une case supplémentaire
  tous les deux points au-dessus de 5. Sous 5, la baisse utilise le même
  palier : Perception 4 donne 7 cases ; Perception 7 donne 9 cases.
- Précision à distance : +2 points par point au-dessus de 5, et −2 par
  point en dessous. Cette contribution s'ajoute à celle de l'Agilité.

Les obstacles et les conditions du lieu continuent à limiter la vision.
La portée de l'arme ne change pas. L'écoute conserve la propagation réelle
des sons ; cette proposition ne remplace pas cette propagation par un
cercle et ne révèle pas d'ennemi caché.

### S05 Intelligence

> Renforce vos drones et vos affaiblissements techno-occultistes.

**Effets proposés :**

- PV maximum et dégâts des drones : variation de 10 % par point autour
  de 5. Intelligence 8 donne +30 % ; Intelligence 4 donne −10 %.
- Durée des affaiblissements techno-occultistes : un tour supplémentaire
  tous les trois points au-dessus de 5. Sous 5, leur durée de base reste
  conservée. Intelligence 8 donne un tour supplémentaire.

Le nombre de drones contrôlés dépendra des capacités apprises. Il ne reçoit
pas d'augmentation implicite dans ce lot. Les affaiblissements conservent
leur efficacité sur les machines, les êtres organiques et les démons.

### S06 Volonté

> Renforce vos sorts, vos transformations et votre résistance à l'occulte.

**Effets proposés :**

- Dégâts des sorts : variation de 10 % par point autour de 5.
- Durée des transformations : un tour supplémentaire tous les deux points
  au-dessus de 5 ; durée de base conservée sous 5.
- Résistance aux altérations occultes : +3 points de chance par point
  au-dessus de 5, et −3 par point sous 5, avant les limites de probabilité.

Les altérations physiques, les dégâts reçus et la résistance aux
interruptions physiques ne reçoivent pas tous ce bonus. Les transformations
et sorts précis feront l'objet de leurs propres fiches.

## Énergie et récupération

Les compétences technologiques et magiques partagent une réserve. Le nom
« Énergie » et les barèmes suivants sont proposés pour cette réserve.

### E01 Énergie

> Sert à utiliser vos pouvoirs magiques et technologiques.

**Capacité proposée :** 30 + 3 × (le plus élevé d'Intelligence et de Volonté
− 5) + 2 × (niveau − 1), avant équipement et implants. Intelligence et
Volonté ne s'additionnent pas dans ce calcul. Un personnage avec
Intelligence 4 et Volonté 6 commence donc à 33, avant son implant.

**Régénération proposée :** 3 points par tour hors combat, 1 point par
tour en combat. Consulter un menu ou attendre en temps réel ne rend rien.

Pour cette proposition, le combat dure tant qu'un ennemi hostile est
actuellement visible ou qu'une attaque du joueur ou une attaque reçue
remonte à moins de trois tours. La disparition d'un ennemi du champ de
vision ne met donc pas immédiatement fin au combat. Un ennemi caché et
inconnu ne doit pas modifier la récupération et révéler sa présence.

Les attaques ordinaires des armes gardent leurs propres coûts, notamment
leurs munitions. Une augmentation de réserve maximale ne la remplit pas.

### E02 Récupérer

> Prenez un moment pour récupérer de l'énergie.

**Effet proposé :** consomme un tour et rend 6 points, en plus de la
régénération normale. La récupération totale vaut donc 7 points en combat
ou 9 hors combat, dans la limite du maximum. Les ennemis peuvent agir
pendant ce tour.

Si la réserve est déjà pleine, la commande est refusée sans faire avancer
le temps. Elle ne rend pas de PV.

### E03 Recharge énergétique

> Rend 12 points d'énergie.

**Effet proposé :** un consommable rend jusqu'à 12 points, dans la limite
du maximum, puis sa quantité diminue d'un. Son utilisation consomme un
tour et la régénération normale de ce tour s'applique aussi.

Une réserve pleine entraîne un refus sans consommation ni tour. Si moins
de 12 points manquent, seule cette quantité est rendue et le consommable
est tout de même utilisé. Le modèle visuel du consommable reste à choisir.

## Progression

### P01 Gagner un niveau

> Vous avez gagné un niveau. Choisissez les capacités que vous voulez
> développer.

**Récompenses proposées :** 1 point d'attribut libre et 2 points de
compétence par niveau gagné. Les nouvelles parties commencent avec les
deux actifs et le passif de leur classe, sans point libre supplémentaire.

**Apprentissage proposé :** apprendre un actif ou un passif coûte 1 point
de compétence. Les améliorations feront l'objet de fiches distinctes ;
ce lot ne fixe pas encore leurs effets ou leur coût. Un achat nécessite
le niveau et les prérequis annoncés par sa fiche, sans exclusivité de classe.

**Limites proposées :** attributs compris entre 1 et 10 ; aucun gain
automatique de PV ou d'énergie actuelle au passage de niveau. Augmenter
un maximum ne remplit pas la réserve. Apprendre un passif le rend actif
en permanence ; il ne crée pas de nouvelle commande.

## Bastion

### B00 Implant initial du Bastion

> Vous encaissez mieux les coups, mais disposez de moins d'énergie.

**Effets proposés :** +10 % de PV maximum, +1 point d'armure et −10 %
d'énergie maximale. Les pourcentages s'appliquent aux maxima issus des
attributs et du niveau. Les bonus d'équipement s'ajoutent ensuite ; les
effets de ces équipements feront l'objet de leurs propres fiches.
Les arrondis de réserves vont vers l'entier inférieur.

Au niveau 1, avec Vitalité 7, Intelligence 4 et Volonté 6, l'implant donne
41 PV maximum au lieu de 38 et 29 points d'énergie maximum au lieu de 33.
L'armure du matériel de départ reste à fixer séparément.

Un implant trouvé remplace cette paire de traits. Il n'efface ni les
compétences ni les points d'attribut dépensés. Changer de maximum conserve
les quantités actuelles, plafonnées si nécessaire ; les échanges ne
permettent pas de se soigner ou de recharger gratuitement.

### B01 Frappe de rupture

> Portez un coup de mêlée plus puissant.

**Effet proposé :** une seule attaque de mêlée, avec +40 % de dégâts
avant les défenses de la cible. Elle garde la portée et les types de
dégâts de l'attaque employée. Le bonus se combine avec la Force.

**Paramètres proposés :** 6 points d'énergie ; recharge de 3 tours ;
une action ordinaire. Aucun déplacement forcé, coup supplémentaire ou
affaiblissement n'est ajouté à cette première version.

Il faut disposer d'une attaque de mêlée légale et d'une cible à portée.
Une tentative impossible ne dépense ni énergie, ni tour, ni recharge.
Une attaque légale qui manque consomme ses coûts. Le traitement sous
transformation sera fixé dans la revue des formes.

### B02 Égide occulte

> Vous subissez moins de dégâts pendant quelques tours.

**Effet proposé :** réduction de 25 % des dégâts directs reçus, après
l'armure et les résistances. Elle s'applique à tous les types de dégâts
directs. Le poison déjà actif et les autres dégâts persistants n'en
bénéficient pas dans cette version.

**Paramètres proposés :** 10 points d'énergie ; durée de 4 tours ;
recharge de 8 tours ; une action ordinaire. La protection commence au
lancement et couvre les réponses ennemies pendant sa durée.

Deux Égides ne se cumulent pas. Relancer une Égide encore active est
refusé sans coût. Un changement d'équipement ne retire pas une protection
déjà lancée.

### B03 Endurance de combat

> Les coups vous interrompent moins souvent.

**Effet proposé :** +10 points de chance de résister à une interruption
physique pendant la préparation d'une technique. La chance finale reste
bornée entre 5 % et 95 %. Ce bonus s'ajoute à la Vitalité et aux facteurs
du coup reçu.

**Paramètres proposés :** passif permanent, sans coût d'énergie et sans
recharge. Il ne réduit pas les dégâts et ne donne pas d'immunité aux
interruptions. Il ne neutralise pas une interruption due à une altération
occulte, à une disparition de cible ou à un prérequis devenu impossible.

## Conventions de résolution proposées

Ces règles font partie de la validation des effets de ce lot.

- Les durées et recharges suivent le temps du jeu. Un tour de référence
  correspond au coût d'une action ordinaire ; une action plus longue
  fait écouler davantage de temps. Les menus ne comptent jamais.
- Les contributions d'attributs et d'implants aux dégâts s'appliquent avant
  les défenses ; les contributions de sources distinctes se multiplient.
  Les dégâts sont arrondis à l'entier inférieur à la fin de leur calcul.
  L'armure peut absorber totalement une attaque trop faible.
- Les coûts modifiés sont arrondis à l'entier supérieur. Les capacités et
  gains de réserves sont arrondis à l'entier inférieur et plafonnés.
- Un refus avant l'exécution ne consomme ni temps, ni ressource, ni
  recharge, ni tirage aléatoire. Un échec d'une action légalement engagée
  conserve ses coûts annoncés.
- Les effets ne confirment pas une cible cachée et ne dévoilent aucune
  case à travers un obstacle par leur aperçu ou leur message de refus.

## Suite de la revue

Les descriptions et les capacités de l'Opérateur, du Traqueur, de
l'Arcaniste et du Métamorphe seront présentées avec la même séparation
entre texte, paramètres et limites. Les quatre familles de magie,
les drones, les cinq transformations, les implants trouvés, les portails,
les objectifs de quêtes et leurs messages auront ensuite leurs propres
fiches. Leurs nouveaux effets visuels devront être montrés en situation.
