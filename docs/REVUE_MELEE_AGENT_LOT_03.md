# Descriptions et chiffres des compétences de mêlée

6 octobre 2026. Les six noms, leurs rôles et leurs intentions visuelles
sont retenus dans le [catalogue](PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md).
Le joueur accepte le lot présenté et autorise la suite : les six textes
roleplay, les tableaux affichés aux rangs 1 et 12 et les conditions
exposées dans la réponse sont retenus comme base de travail. L'équilibrage
sera ajusté progressivement, plus tard. Les formules et conditions
supplémentaires présentes seulement dans ce document restent proposées.
Les bonus déjà acceptés de B01 aux rangs 1, 10 et 12 sont conservés. Ces fiches suivent le
[format demandé](REVUE_DESCRIPTIFS_COMPETENCES_AGENT.md).

ME02 porte désormais le nom de Fauchage. Le joueur valide sa fiche en
prose sans tableau, y compris les progressions présentées et les valeurs
d'exemple. Les conditions détaillées non présentées et le rendu visuel
restent à revoir ; les coefficients d'attributs restent eux-mêmes proposés.

ME03 Percée est ensuite validée dans le format « Effet actuel / Prochain
rang », avec neuf rangs appris et deux d'équipement, puis dix rangs appris
et deux d'équipement. Les plages de dégâts de cet exemple utilisent une
arme fictive à 18–22 dégâts. Les formules supplémentaires non présentées
et le rendu visuel restent à revoir.

ME04 Repousser est ensuite validée avec son texte, les valeurs actuelles
et du prochain rang présentées, ainsi que la collision directe : +25 %
des dégâts réels du coup initial sur la cible repoussée, et 25 % sur le
monstre percuté. Les cas supplémentaires non présentés restent à revoir.

ME05 Charge est ensuite validée avec son texte, ses plages de dégâts
actuelles et du prochain rang, sa cible visible à 2–5 cases, son trajet
libre, son coût et sa recharge présentés. Les conditions supplémentaires
non montrées et le rendu visuel restent à revoir.

ME06 Tourbillon reçoit une demande explicite de saignement léger. La
nouvelle fiche est ensuite validée : saignement sur chaque ennemi touché, à 10 % des
dégâts réels du coup par tour pendant trois tours, sans cumul et avec
renouvellement de durée. Les valeurs affichées du coup, 22–27 au rang 11
et 23–28 au rang 12, sont retenues pour le profil d'exemple. Le bloc
« Prochain rang » présente uniquement les valeurs qui changent. Les
formules et les cas supplémentaires non montrés restent à revoir, ainsi
que le rendu visuel.

## Référence et exemple actuel

Pour cette première revue, « par défaut » désigne le rang effectif 1,
avant les bonus d'attributs, d'équipement, d'implants et de buffs. Cette
référence est celle du premier lot accepté.

La colonne « Actuel » illustre un profil de calcul : dix rangs appris,
deux rangs d'équipement, donc un rang effectif de 12, avec Force 10 et
Agilité 10. Elle ne décrit pas une sauvegarde du joueur. L'affichage
en jeu utilisera les valeurs réelles du personnage.

Pour comparer les dégâts, les deux colonnes utilisent une attaque de
mêlée fictive de 20 dégâts et une portée d'une case. La référence emploie
Force 5 et Agilité 5, donc aucun bonus de ces attributs. Dans l'exemple
actuel, les contributions proposées de S01 et S02 donnent +10 % par
Force et +2,5 % par Agilité, soit un multiplicateur de 1,1275. Ces
coefficients restent eux-mêmes à valider. Aucun autre modificateur
de dégâts ou de coût n'est présent dans le profil.

Les dégâts chiffrés sont calculés avant les défenses de la cible et
arrondis à l'entier inférieur. Ils ne garantissent ni une touche ni
des dégâts nets sur un monstre. Les portées indiquent des cases.
Tous les paramètres propres à la compétence utilisent son rang effectif.
Le niveau 12 sera distingué en cyan lorsqu'il est augmenté par équipement,
selon l'accord précédent ; le rendu exact sera présenté séparément.

## Arme requise et conditions proposées

**Exigence confirmée :** ces actifs nécessitent une arme de mêlée équipée
et active. Les dégâts gardent les types de l'arme employée.
Chaque cible fait l'objet de la résolution habituelle de précision
et de défense ; plusieurs cibles ne garantissent pas plusieurs touches.

Chaque utilisation prend une action ordinaire, soit un tour de référence,
et paie l'énergie une seule fois. Un refus avant exécution ne dépense ni
énergie, ni tour, ni recharge. Une attaque exécutée qui manque paie ses
coûts. Les déplacements et aperçus respectent les obstacles et les règles
de passage ; ils ne révèlent pas de cibles ou de terrain cachés.

### B01 Frappe de rupture

> Vos implants libèrent toute leur puissance dans un coup brutal.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Bonus de dégâts de mêlée | +40 % | +150 % |
| Dégâts avec l'attaque de comparaison | 28 | 56 |
| Coût d'énergie | 6 | 9 |
| Recharge | 3 tours | 3 tours |
| Cible et portée | 1 cible, portée de l'attaque | 1 cible, portée de l'attaque |

**Progression proposée :** +10 points de pourcentage de dégâts par rang
supplémentaire ; énergie = 6 + floor((n − 1) / 3). Les bonus de +40 %,
+130 % et +150 % aux rangs 1, 10 et 12 sont déjà retenus. Le coût,
la recharge et le texte présentés sont retenus comme base de travail.
Les conditions supplémentaires restent proposées.

Un seul coup est porté. Cette compétence n'ajoute pas de recul, de frappe
secondaire ou de réduction d'armure. Sa portée reste celle de l'attaque employée.

**Intention visuelle retenue :** impact lourd et anguleux suivant l'arme.
Le rendu demeure à présenter.

### ME02 Fauchage

> Vous dessinez un large arc avec votre arme pour faucher les ennemis devant vous.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par cible, par rapport à une attaque de mêlée | 80 % | 135 % |
| Dégâts par cible avec l'attaque de comparaison | 16 | 30 |
| Zone | 3 cases adjacentes devant vous | 3 cases adjacentes devant vous |
| Coût d'énergie | 8 | 10 |
| Recharge | 3 tours | 3 tours |

**Progression retenue :** dégâts en pourcentage = 80 + 5 × (n − 1) ;
énergie = 8 + floor((n − 1) / 4). La zone ne grandit pas avec le rang.
Les dégâts continuent au-delà du rang 10.

Le joueur choisit une direction. Le secteur contient la case adjacente
dans cette direction et ses deux voisines dans l'anneau de huit cases.
Chaque ennemi légalement atteignable dans ce secteur peut subir un coup.
Les alliés ne sont pas visés ; les murs et les passages bloqués limitent
les cibles selon les règles de mêlée. Au maximum trois ennemis sont frappés.

**Intention visuelle retenue :** croissant large suivant l'arme devant
le personnage. Le rendu demeure à présenter.

### ME03 Percée

> Vous visez une faille dans l'armure et y concentrez votre frappe.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par rapport à une attaque de mêlée | 100 % | 188 % |
| Dégâts avec l'attaque de comparaison, avant armure | 20 | 42 |
| Part d'armure ignorée | 20 % | 48,7 % |
| Coût d'énergie | 8 | 11 |
| Recharge | 4 tours | 4 tours |
| Cible et portée | 1 cible, portée de l'attaque | 1 cible, portée de l'attaque |

**Progression proposée :** dégâts en pourcentage = 100 + 8 × (n − 1) ;
armure ignorée = 20 + 60 × (n − 1) / (n + 11) pour cent ;
énergie = 8 + floor((n − 1) / 3). L'armure ignorée reste strictement
inférieure à 80 %, même avec beaucoup de bonus d'équipement. Son affichage
est arrondi au dixième ; son calcul conserve la précision complète.

La part d'armure ignorée vaut uniquement pour ce coup. La compétence ne
pose pas d'affaiblissement durable et ne retire pas les résistances
élémentaires de la cible. Elle garde la résolution ordinaire de précision.

**Intention visuelle retenue :** impact étroit et éclats de plaques.
Le rendu demeure à présenter.

### ME04 Repousser

> Vous frappez pour chasser votre adversaire de votre chemin.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par rapport à une attaque de mêlée | 90 % | 156 % |
| Dégâts avec l'attaque de comparaison | 18 | 35 |
| Recul maximum après une touche | 1 case | 3 cases |
| Coût d'énergie | 7 | 9 |
| Recharge | 3 tours | 3 tours |
| Cible et portée | 1 cible, portée de l'attaque | 1 cible, portée de l'attaque |

**Progression proposée :** dégâts en pourcentage = 90 + 6 × (n − 1) ;
recul = min(3, 1 + floor((n − 1) / 5)) ;
énergie = 7 + floor((n − 1) / 4). Les dégâts continuent au-delà du
plafond de trois cases de recul.

Un impact réussi repousse la cible dans la direction qui l'éloigne du
personnage, uniquement sur des cases franchissables et libres. Le recul
s'arrête au premier obstacle ou occupant. Une cible qui ne peut pas
reculer reçoit tout de même le coup.

**Collision retenue :** lorsque la cible repoussée heurte un mur ou un autre
monstre, elle reçoit un bonus de dégâts de 25 % des dégâts réellement
infligés par le coup initial. Le monstre percuté reçoit lui aussi 25 %
de cette même base, avant le bonus de collision. Les réponses du joueur
confirment cette base commune et l'application directe aux deux victimes,
sans nouvelle réduction par leur armure ou leurs résistances.

Cet ajout remplace l'ancienne proposition sans dégâts de collision.
L'arrondi et les autres cas de collision restent à revoir. L'étourdissement n'est
pas ajouté par la proposition actuelle. Les éventuelles immunités au
déplacement de certains monstres seront définies séparément.

**Intention visuelle retenue :** flash carré d'impact et trace de recul.
Le rendu demeure à présenter.

### ME05 Charge

> Vous foncez sur votre cible, arme en avant, et frappez au contact.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par rapport à une attaque de mêlée | 110 % | 198 % |
| Dégâts avec l'attaque de comparaison | 22 | 44 |
| Distance maximum de la cible au départ | 3 cases | 5 cases |
| Coût d'énergie | 10 | 13 |
| Recharge | 5 tours | 5 tours |

**Progression proposée :** dégâts en pourcentage = 110 + 8 × (n − 1) ;
distance de cible = min(6, 3 + floor((n − 1) / 4)) ;
énergie = 10 + floor((n − 1) / 3). La portée plafonne à six cases,
mais les dégâts continuent d'augmenter.

La cible doit être à au moins deux cases, visible et accessible par une
ligne de charge franchissable. Le personnage s'arrête sur une case libre
adjacente à la cible, puis porte un seul coup. Il ne traverse ni mur ni
occupant et ne frappe pas gratuitement les ennemis rencontrés en chemin.
Le déplacement et l'attaque constituent ensemble une action ordinaire.
Si le coup manque, le déplacement déjà effectué demeure et les coûts
sont payés. Une cible déjà au contact n'autorise pas une charge sur place.

**Intention visuelle retenue :** traînée basse et front lumineux sur l'arme.
Le rendu demeure à présenter.

### ME06 Tourbillon

> Vous pivotez au milieu des ennemis et frappez tout autour de vous.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Dégâts par cible, par rapport à une attaque de mêlée | 60 % | 115 % |
| Dégâts par cible avec l'attaque de comparaison | 12 | 25 |
| Zone | 8 cases adjacentes autour de vous | 8 cases adjacentes autour de vous |
| Coût d'énergie | 12 | 15 |
| Recharge | 6 tours | 6 tours |

**Progression proposée :** dégâts en pourcentage = 60 + 5 × (n − 1) ;
énergie = 12 + floor((n − 1) / 3). La zone ne grandit pas avec le rang.
Les dégâts continuent au-delà du rang 10.

Chaque ennemi légalement atteignable dans l'anneau adjacent peut subir
un coup, au maximum huit ennemis. Les alliés ne sont pas visés ; les
obstacles et les règles de passage limitent les attaques diagonales.
Le personnage conserve sa case. La technique est une frappe circulaire
ponctuelle, sans déplacement.

**Saignement présenté et validé :** chaque ennemi
touché reçoit un saignement pendant trois tours. Chaque tour, ce saignement
inflige 10 % des dégâts réellement subis lors du coup initial. Le pourcentage
et la durée restent fixes ; les dégâts suivent ceux du coup, donc l'arme,
les attributs, les bonus applicables et le rang effectif de Tourbillon.
Une nouvelle application de Tourbillon renouvelle la durée sans cumuler
les saignements de cette compétence. Les arrondis, l'ordre des ticks et les
interactions avec d'autres sources de saignement restent à préciser pour revue.

La fiche actuelle et le prochain rang sont conservés dans
[la fiche individuelle ME06, révision 4](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-ME06-04.json).
Le joueur demande que le bloc « Prochain rang » affiche seulement les
valeurs qui changent : le saignement et la zone inchangés y sont retirés,
tout en restant dans les effets actuels. Cette correction ne change aucun chiffre.
Le joueur valide ensuite la fiche corrigée et les chiffres présentés.
Cette validation ne porte pas sur les détails non montrés ni sur une animation.

**Intention visuelle retenue :** cercle complet de coups autour du corps.
Le rendu demeure à présenter.

## Progression comparée

Les valeurs sont des pourcentages de l'attaque de mêlée, sauf B01 où le
tableau distingue le bonus de dégâts de son multiplicateur total.
Les chiffres présentés aux rangs 1 et 12 sont retenus provisoirement.
Les autres valeurs et les formules détaillées restent proposées pour revue.

| Compétence | Rang 1 | Rang 10 | Rang 12 |
| --- | --- | --- | --- |
| B01 bonus de dégâts | +40 % | +130 % | +150 % |
| ME02 dégâts par cible | 80 % | 125 % | 135 % |
| ME03 dégâts | 100 % | 172 % | 188 % |
| ME03 armure ignorée | 20 % | 45,7 % | 48,7 % |
| ME04 dégâts | 90 % | 144 % | 156 % |
| ME04 recul maximum | 1 case | 2 cases | 3 cases |
| ME05 dégâts | 110 % | 182 % | 198 % |
| ME05 distance de cible | 3 cases | 5 cases | 5 cases |
| ME06 dégâts par cible | 60 % | 105 % | 115 % |

Le placement dans les paliers et leurs seuils n'est pas fixé par ces
fiches. Les six compétences utilisent les points de niveau déjà retenus.
Le texte, les nouveaux barèmes, les conditions d'emploi et les rendus
gardent des validations distinctes sous les mêmes identifiants.
