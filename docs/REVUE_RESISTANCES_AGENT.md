# Résistances de la refonte — proposition complète

9 octobre 2026. Lot accepté **RESIST-LOT04**, parties **R01 à R05**. Le joueur a
accepté S06-RESIST01 et demandé de préparer les résistances pour les voir
ensuite. Cette délégation autorise la proposition ; les nouveaux textes,
chiffres et effets restent soumis à sa validation avant intégration.

## Présentation soumise au joueur

La **résistance occulte est validée**. Voici l’ensemble que je propose pour les autres protections.

**R01 · Résistances aux dégâts**

*Vos protections atténuent les dégâts. Vos faiblesses les amplifient.*

Cinq résistances aux dégâts proposées : **feu, froid, électricité, poison et dégâts occultes**.
Les radiations sont retirées de cette proposition : leur type existe dans le
moteur, mais aucun contenu de jeu trouvé ne l’utilise.

- Base hors bonus : **0 % pour chaque type**.
- Maximum effectif : **75 %**.
- Faiblesse maximale : **−50 %**.
- Les bonus d’équipement, d’implants, de passifs et de buffs s’additionnent ; les malus sont retirés avant ces limites.

Exemples avec **100 dégâts** :

- Résistance de **40 %** : **60 dégâts subis**.
- Résistance de **75 %** : **25 dégâts subis**.
- Résistance de **−25 %** : **125 dégâts subis**.

Le surplus protège contre les malus : **90 % de bonus − 25 % de malus = 65 % de résistance effective**. Le joueur et les monstres suivent ces mêmes limites.

**R02 · Armure**

*Votre blindage absorbe une partie des impacts.*

L’armure retire des points de dégâts à la partie physique d’un impact : coups, balles et souffle physique des explosions. Elle s’applique **une seule fois par impact** et peut absorber entièrement cette partie.

Exemple : **20 dégâts physiques + 20 dégâts de froid**, contre **5 armure et 50 % de résistance au froid** → **15 physiques + 10 froid = 25 dégâts subis**.

**R03 · Dégâts persistants**

Brûlure utilise la **résistance au feu** et Empoisonné la **résistance au poison** pour leurs dégâts à chaque tour.

Le **Gel utilise la résistance au froid** : elle réduit les dégâts de froid
et la chance d’application du Gel, sans changer sa durée. Cette direction
est explicitement choisie par le joueur le 9 octobre ; son calcul chiffré
est proposé plus bas. Aucun nouveau dégât périodique n’est ajouté au Gel.

Un dégât périodique déjà calculé à partir des **dégâts réellement subis**, comme le saignement de Tourbillon, ne repasse pas une deuxième fois dans les défenses.

Les dégâts restent **entiers**, arrondis à l’entier inférieur à la fin du calcul. La résistance utilisée est celle de la cible au moment où les dégâts arrivent.

**R04 · Contrôles — règles conservées**

La **résistance occulte** validée reste une valeur distincte : **25 avec Volonté 5, puis +2 par point**. Le plafond de 75 % concerne les résistances aux dégâts.

- **Élites :** chance de contrôle multipliée par **0,75**.
- **Boss :** chance de Gel et de Brûlure multipliée par **0,5** ; immunité aux autres contrôles.
- **Poison :** aucun malus lié à l’importance du monstre.

Les chances hors Gel gardent leurs règles. Pour le Gel, la résistance au
froid sera prise en compte selon le choix du joueur ; le calcul proposé
n’ajoute pas de deuxième jet de résistance. Les valeurs de chaque fiche
concernée devront être recalculées et soumises pour la partie modifiée.

**R05 · Exceptions et affichage**

Les dégâts de collision de **Repousser** et les dégâts renvoyés par **Barrière d’épine** conservent leur application directe, sans nouvelle réduction. Vos propres sorts continuent à ne pas vous blesser.

Les résistances actuelles seront affichées dans la fiche du personnage et dans l’analyse détaillée des cibles. Les bonus et malus temporaires figureront dans les barres de buffs et de debuffs prévues.

**Les nouvelles règles de ce lot restent à valider.**

## Périmètre de la proposition

Les pourcentages de résistance aux dégâts sont séparés de la valeur de
résistance occulte utilisée contre les effets. Aucun nouveau gain de
résistance élémentaire par niveau ou attribut n’est créé. Les six
attributs conservent leurs coefficients acceptés.

Les limites −50 % et +75 % concernent la couche des résistances aux dégâts,
pas l’armure, l’esquive, les chances de contrôle ni les autres protections
spécifiques. Les réductions supplémentaires déjà approuvées conservent
les conditions de leur propre fiche, notamment l’exclusion des dégâts
persistants par Égide occulte. Leur cumul détaillé sera vérifié lors de
l’intégration et reste à revoir lorsqu’il n’a pas déjà été présenté.

Les monstres peuvent avoir leurs propres résistances et faiblesses dans
ces limites ; aucun profil de monstre ni aucune nouvelle immunité de statut
liée à sa nature n’est défini dans ce lot. Leurs statistiques et compétences
seront revues séparément. Le poison conserve l’absence de malus lié à
l’importance, tout en passant par sa résistance ordinaire aux dégâts.

Une explosion est classée selon ses composantes de dégâts : sa partie
physique est absorbée par l’armure ; un éventuel feu passe par la résistance
au feu. Aucune compétence explosive déjà acceptée ne reçoit une nouvelle
composante ni de nouveaux dégâts par ce document.

Le surplus de résistance est conservé avant les malus : avec 90 points de
bonus et un malus de 25 points, on obtient 65 %. Sans malus, les mêmes 90
points donnent 75 % de résistance effective. Aucun affixe permettant de
modifier ce plafond n’est créé. Les valeurs et probabilités des futurs
bonus de loot restent à définir.

La résistance disponible au moment de chaque impact ou tick ordinaire est
utilisée pour cette résolution. Les montants dérivés de dégâts réellement
subis évitent une seconde application des défenses ; Tourbillon sert ici
d’exemple. Cela ne définit pas les horaires des ticks ni les interactions
entre plusieurs sources de saignement. Aucun minimum général d’un point de
dégât n’est ajouté ; un résultat arrondi à zéro ou entièrement absorbé
reste possible. La variance des armes demeure à revoir séparément.

Les immunités de contrôle des boss et les malus d’importance retenus sont
conservés. Le multiplicateur applicable n’est pris qu’une fois. Le lot ne
transforme pas les chances de Brûlure en contrôle d’élite, ne plafonne pas
les chances de zone déjà acceptées à 95 % et n’ajoute aucun jet défensif
supplémentaire aux compétences. Les règles des fiches hors Gel sont
conservées. Le lien entre Gel et résistance au froid remplace la proposition
d’exclusion de toute chance d’application ; les calculs et les valeurs
affectés restent à soumettre. La durée du Gel est conservée.

Repousser et Barrière d’épine gardent leurs exceptions directes déjà
approuvées. Les défenses du destinataire et les autres cas non montrés de
Sceau de vindicte restent à revoir, sans les valider par analogie. La
protection du joueur contre ses propres dégâts ne vaut pas permission de
modifier les règles de dégâts aux autres familiers.

L’analyse détaillée de cible conserve l’accès par l’Éclaireur déjà demandé.
Les valeurs de buff et debuff restent visibles sans lui. Les intitulés des
résistances, leur emplacement, leurs couleurs et le rendu natif seront
revus avant intégration ; ce document ne crée aucun écran ni asset.

## Cas encore à revoir

Les paramètres de loot, les résistances individuelles des monstres et des
familiers, les effets de pénétration, les protections spéciales, les
composantes physiques multiples, le timing des statuts, les transitions,
les transformations et le rendu natif restent à détailler. Ces limites
empêchent d’interpréter la proposition comme une validation de tous les
cas non présentés.

Manifeste accepté actuel : [RESIST-LOT04](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-04.json).
Présentation acceptée : [RESIST-LOT03](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-03.json).
Révision précédente : [RESIST-LOT02](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-02.json).
Brouillon historique : [RESIST-LOT01](../docs/reviews/refonte-agent-2026-10-06/review-resistance-system-01.json).
Accord précédent : [S06-RESIST-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/occult-resistance-acceptance-and-full-resistance-review-request-decisions.json).


## R01-RAD01 et R03-ELEMENT01 Correction et choix du joueur — 9 octobre 2026

La présence technique de Radiation ne constitue pas un contenu jouable.
La famille est retirée du lot proposé ; aucune déclaration, sauvegarde ni
ligne d’interface du moteur n’est supprimée dans cette phase de conception.
L’électricité reste couverte ; la branche Foudre reste supprimée.

Le joueur rattache les effets à leur élément : Brûlure au feu, Empoisonné
au poison, Gel au froid. Pour le Gel, il choisit explicitement « Dégâts de
froid et chance de Gel », avec durée inchangée. Aucun nouveau malus de
chance de Brûlure ou de Poison n’est déduit de ce choix. Les règles liées
aux boss, élites et à l’exposition aux zones restent conservées.

## R03-GEL02 Calcul proposé après clarification

Tu as raison pour les **radiations** : ce type existe dans le moteur, mais je n’ai trouvé aucun contenu de jeu qui inflige ces dégâts. Je le retire donc de la proposition.

Je corrige les associations :

- **Brûlure → résistance au feu**, pour ses dégâts à chaque tour.
- **Empoisonné → résistance au poison**, pour ses dégâts à chaque tour.
- **Gel → résistance au froid**, pour les dégâts du sort et, selon ton choix, la **chance d’être gelé**. Sa durée reste inchangée.

La résistance occulte conserve son rôle contre les malédictions et les contrôles occultes.

Pour le Gel, je propose ce calcul simple : **chance de Gel × (100 − résistance au froid) ÷ 100**.

Exemple avec **80 % de chance avant résistance** et **40 % de résistance au froid** :

- Chance de Gel : **48 %**.
- Sur un boss, avec le malus déjà validé : **24 %**.
- Si le Gel s’applique, sa durée reste celle de la compétence.

**Les associations sont notées ; ce calcul chiffré reste à valider.**

Dans la proposition, la résistance au froid serait la défense élémentaire
du Gel ; elle ne s’ajouterait pas comme deuxième défense à une ancienne
résistance occulte de cible. Les contributions du rang et des attributs,
le cumul d’exposition, les plafonds et les valeurs de chaque fiche doivent
être conciliés avant intégration. Les nouvelles valeurs de tooltips
concernés ne reçoivent aucune validation par cet exemple fictif. Les
formules exactes de plafonnement et les cas de résistance négative restent
à présenter. Aucune modification du moteur ni nouveau rendu.

Décision : [R03-GEL01](../docs/reviews/refonte-agent-2026-10-06/radiation-placeholder-withdrawal-and-elemental-status-resistance-decisions.json).

## R03-GEL02-ACCEPT01 Accord sur le calcul montré — 9 octobre 2026

La formule linéaire du Gel et les exemples quarante-huit pour cent sur la
cible ordinaire et vingt-quatre sur le boss sont acceptés. Les anciens
passages les présentant comme en attente restent un historique de revue.
Le champ de cet accord n’inclut pas les règles communes encore proposées.
Les limites générales, les nouveaux rangs calculés, les exemples de zone et
le plafonnement de probabilité en cas de résistance négative restent à valider.

Décision : [R03-GEL02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/freeze-elemental-chance-formula-acceptance-decisions.json).

## RESIST-LOT03 Proposition commune restante — 9 octobre 2026

Le calcul du **Gel est validé**. Pour terminer ce bloc, voici les règles communes que je propose.

**R01 · Résistances aux dégâts**

*Vos protections atténuent les dégâts. Vos faiblesses les amplifient.*

- Types : **feu, froid, électricité, poison et dégâts occultes**.
- Base hors bonus : **0 %**.
- Limites : **−50 % à +75 %**, pour le joueur et les monstres.
- Bonus additionnés, puis malus soustraits, avant d’appliquer les limites.

**100 dégâts avec 40 % de résistance → 60 dégâts subis.**
**100 dégâts avec −25 % de résistance → 125 dégâts subis.**

Le surplus reste utile contre les malus : **90 % de bonus − 25 % de malus → 65 % effectifs**.

**R02 · Armure**

*Votre blindage absorbe une partie des impacts.*

- **1 point d’armure retire 1 dégât physique**.
- Application **une seule fois par impact**, sur sa partie physique.
- Une partie physique entièrement absorbée inflige **0 dégât**.

**40 dégâts physiques contre 12 armure → 28 dégâts subis.**

**R03 · Effets persistants et arrondis**

- La résistance actuelle est utilisée à chaque nouveau coup ou tour de dégâts persistants.
- Un saignement calculé sur les dégâts réellement subis ne passe pas une deuxième fois dans l’armure.
- Dégâts ordinaires arrondis **à l’entier inférieur après les protections**.
- Une faiblesse au froid augmente aussi la chance de Gel, limitée à **100 % avant le malus d’élite ou de boss**.

**R05 · Affichage**

Les résistances effectives apparaissent dans la fiche du personnage et dans l’analyse détaillée accessible avec l’Éclaireur. Les modifications temporaires restent consultables dans les barres de buffs et de debuffs.

Exemple de valeur affichée : **Résistance au froid : 40 %**.

**Ces règles communes restent à valider.**

Le plafonnement proposé à cent pour cent concerne le Gel après la
modification par une faiblesse au froid et avant le malus d’importance.
Il ne remplace pas la formule de résistance occulte déjà acceptée, ne
crée pas de nouveau malus de Brûlure ou de Poison et ne fixe pas encore
l’ordre complet des jets d’exposition en zone.

Le texte de statistiques et la direction d’affichage ne valent pas accord
sur un rendu natif. L’accès à l’analyse détaillée par l’Éclaireur et les
valeurs de statut visibles sans lui sont conservés. Les exceptions de
Repousser et de Barrière d’épine restent directes ; les cas non montrés de
Sceau de vindicte ne sont pas validés par ce regroupement. Les profils des
monstres et familiers et les bonus précis des objets restent à définir.

## RESIST-LOT03-ACCEPT01 Accord explicite — 9 octobre 2026

Le joueur valide la dernière présentation commune de R01, R02, R03 et R05.
Les passages plus anciens indiquant ces règles comme proposées sont
conservés comme historique. Les textes RP, cinq familles sans Radiation,
base zéro, limites −50 % à +75 %, addition avant limites, surplus contre
les malus, armure fixe une fois par impact avec plancher zéro, résistance
courante, absence de seconde armure sur saignement dérivé de dégâts réels,
arrondi ordinaire final et Gel plafonné à cent pour cent avant importance
sont acceptés. Les exemples et la direction d’affichage sont acceptés.

Les cas encore ouverts et le rendu natif ne sont pas approuvés par cet
accord. Aucun moteur, contenu jouable ou asset n’est modifié.

Décision : [RESIST-LOT03-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/common-resistance-system-acceptance-decisions.json).
