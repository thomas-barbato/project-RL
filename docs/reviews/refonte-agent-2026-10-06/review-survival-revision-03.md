# Descriptions et chiffres des compétences de Survie

6 octobre 2026. Les six noms et leurs rôles sont retenus dans le
[catalogue](PROPOSITION_COMPETENCES_PRINCIPALES_AGENT.md). Les textes roleplay
et les nouveaux chiffres ci-dessous constituent le prochain lot à valider.
Ils servent de base de travail ; l'équilibrage sera repris progressivement
dans la salle de test, selon le souhait du joueur.

B03 est ensuite renommée **Vitalité améliorée** par le joueur, qui valide
sa fiche individuelle : texte roleplay, passif permanent, augmentation du
maximum de vie sans soin, +30 % au rang 11 et +32 % au rang 12. L'exemple
utilise 100 PV avant le passif, puis 130 et 132 PV maximum. Les autres
paramètres non montrés et le rendu visuel restent à revoir. La fiche
validée est conservée sous le même identifiant B03 dans
[sa revue individuelle](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-B03-02.json).

B02 Égide occulte est ensuite validée : texte roleplay et fonctionnel,
réduction des dégâts directs de 50 % au rang 11 et de 51,2 % au rang 12,
durée de quatre tours, coût de 13 énergie et recharge de huit tours.
L'exclusion des dégâts persistants est retenue. Les conditions
supplémentaires non montrées et le rendu restent à revoir. La fiche
validée est conservée dans
[sa revue individuelle](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-B02-02.json).

## Référence et exemple actuel

« Par défaut » désigne la compétence au rang effectif 1, sans les bonus
des autres compétences, équipements, implants ou buffs. Les calculs
utilisent un maximum de vie de référence de 100 PV.

La colonne actuelle illustre un personnage avec dix rangs appris et deux
rangs d'équipement, soit un rang effectif de 12. Il possède 100 PV maximum
avant Vitalité améliorée, puis 132 PV avec ce passif au rang 12. Ces PV
avant le passif comprennent déjà ses éventuels niveaux, attributs et
bonus fixes. Il n'a pas d'autre bonus aux coûts, soins ou protections.
L'exemple ne décrit pas une sauvegarde du joueur.

Les deux colonnes distinguent le pourcentage de l'effet de la quantité
qu'il produit sur ce profil. Les gains de PV et absorptions sont arrondis
au supérieur ; le maximum de vie est arrondi à l'inférieur. Les valeurs
affichées en pourcentage sont arrondies au dixième, en conservant les
fractions exactes dans le calcul. Le rang renforcé sera indiqué en cyan.

## Conditions communes proposées

Les passifs appris fonctionnent sans emplacement à équiper. Les actifs
prennent une action ordinaire, soit un tour de référence, sauf la
réparation qui demande une action par tour maintenu. Les menus ne font
pas avancer le temps. Un refus avant exécution ne dépense ni énergie,
ni tour, ni recharge. Les soins ne dépassent jamais les PV maximum.

Pour les protections décrites ici, l'ordre proposé est : armure et
résistances, Égide occulte, absorption de Barrière réactive, puis Dernier
sursaut si le coup restant devait tuer. Les effets persistants suivent
leurs propres règles et ne deviennent pas des coups directs.

### B03 Vitalité améliorée

> Votre corps renforcé supporte davantage de blessures avant de céder.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Bonus de PV maximum | +10 % | +32 % |
| PV maximum avec 100 PV avant le passif | 110 | 132 |
| Coût et recharge | Aucun | Aucun |
| Fonctionnement | Passif permanent | Passif permanent |

**Progression conservée :** bonus = 10 + 2 × (n − 1) pour cent.
Les +10 %, +28 % et +32 % aux rangs 1, 10 et 12 sont déjà retenus.
Les gains continuent au-delà du rang 10. Les bonus de maximum de vie en
pourcentage sont proposés comme additifs entre eux, puis appliqués au
maximum avant ces pourcentages, avec un arrondi final unique.

Améliorer le passif ou changer d'équipement ne soigne pas gratuitement.
Les PV actuels sont seulement plafonnés lorsque le maximum diminue.
Le Bastion conserve son premier rang offert au départ.

**Intention visuelle retenue :** indication du maximum sur la jauge de vie.
Le rendu demeure à présenter.

### B02 Égide occulte

> Un sceau protecteur entoure votre corps et amortit les coups reçus.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Réduction des dégâts directs | 25 % | 51,2 % |
| Durée | 4 tours | 4 tours |
| Coût d'énergie | 10 | 13 |
| Recharge | 8 tours | 8 tours |

**Progression conservée :** réduction = 25 + 50 × (n − 1) / (n + 9)
pour cent ; énergie = 10 + floor((n − 1) / 3). Les réductions de
25 %, 48,7 % et 51,2 % aux rangs 1, 10 et 12 sont déjà retenues.
La protection reste strictement inférieure à 75 %, même au-delà de 10.
Le texte présenté, les valeurs aux rangs 11 et 12, la durée, le coût,
la recharge et l'exclusion des dégâts persistants sont validés dans la
fiche individuelle. Les formules et conditions non montrées restent à revoir.

La protection s'applique après l'armure et les résistances. Elle couvre
les dégâts directs, avec leurs différents types, pendant sa durée.
Elle ne réduit pas les dégâts persistants d'un poison ou d'une zone déjà
active. Deux Égides ne se cumulent pas ; relancer une Égide active est
refusé sans coût. Les paramètres sont fixés au lancement pour cet effet.

**Intention visuelle retenue :** coque runique réagissant aux impacts.
Le rendu demeure à présenter.

### SU03 Barrière réactive

> Après une accalmie, vos protections se reforment pour encaisser le prochain coup.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Absorption du prochain coup direct | 5 % des PV maximum | 13,9 % des PV maximum |
| Quantité absorbable sur le profil | 5 PV | 19 PV |
| Accalmie nécessaire | 4 tours | 4 tours |
| Coût d'énergie | Aucun | Aucun |

**Progression proposée :** absorption = 5 + 25 × (n − 1) / (n + 19)
pour cent des PV maximum. Cette part augmente au-delà de 10, en restant
strictement inférieure à 30 %. L'attribut Vitalité et le passif Vitalité améliorée renforcent la
quantité absorbable en augmentant le maximum de vie.

La barrière est prête après quatre tours sans recevoir de dégâts,
directs ou persistants. Un coup direct qui infligerait des dégâts après
les protections précédentes utilise la barrière, puis la consomme
entièrement même si ce coup est plus faible que sa capacité. Un coup
entièrement absorbé compte comme reçu pour l'accalmie. Les dégâts
persistants interrompent l'accalmie mais ne consomment pas une barrière
déjà prête. Une attaque sans dégâts après les protections précédentes
ne consomme pas la barrière.

Un changement de rang ou de maximum de vie actualise sa capacité, sans
réinitialiser le délai d'accalmie ni recréer une barrière consommée.
Cette barrière est un effet du personnage ; elle ne protège pas les drones.

**Intention visuelle retenue :** plaques lumineuses apparaissant à l'impact
puis se brisant. Le rendu demeure à présenter.

### SU04 Réparation canalisée

> Vous immobilisez votre corps tandis que vos systèmes réparent ses blessures.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Soin par tour maintenu | 4 % des PV maximum | 12,4 % des PV maximum |
| PV rendus par tour sur le profil | 4 PV | 17 PV |
| Durée maximum | 3 tours | 3 tours |
| Soin total maximum sur le profil | 12 PV | 51 PV |
| Énergie par tour maintenu | 4 | 6 |
| Énergie pour trois tours | 12 | 18 |
| Recharge au lancement | 8 tours | 8 tours |

**Progression proposée :** soin par tour = 4 + 16 × (n − 1) / (n + 9)
pour cent des PV maximum ; énergie par tour = 4 + floor((n − 1) / 4).
Le soin reste strictement inférieur à 20 % par tour, et continue de
progresser au-delà de 10. Les PV maximum issus de Vitalité et d'Endurance
augmentent la quantité rendue.

Le lancement coûte le premier tour et produit le premier soin. Chaque
maintien accepté paie ensuite un tour et son énergie, jusqu'à trois
soins. Le joueur peut arrêter entre ces tours ; se déplacer ou effectuer
une autre action met fin à la canalisation. Toute perte de PV l'interrompt.
Un coup entièrement absorbé sans perte de PV ne l'interrompt pas.

Les ennemis continuent d'agir. La recharge commence au lancement et ne
bloque pas les maintiens de la canalisation en cours. Un lancement à
vie pleine est refusé ; atteindre le maximum termine la canalisation,
sans facturer un prochain maintien. Une interruption ne rembourse pas
les tours ou l'énergie déjà dépensés. Cette réparation ne nettoie pas
les altérations et ne peut pas relever un personnage mort.

**Intention visuelle retenue :** fils lumineux refermant la silhouette.
Le rendu demeure à présenter.

### SU05 Purgation

> Vous chassez les poisons et les influences hostiles qui entravent votre corps.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| Altérations compatibles retirées | Jusqu'à 1 | Jusqu'à 3 |
| Coût d'énergie | 10 | 8 |
| Recharge | 6 tours | 6 tours |
| Soin direct | Aucun | Aucun |

**Progression proposée :** maximum d'altérations = 1 + floor((n − 1) / 5) ;
énergie = ceil(10 × (1 − 0,4 × (n − 1) / (n + 9))). L'amélioration
permet de retirer davantage d'altérations, puis continue au-delà de 10.
Le coût diminue progressivement et ne devient jamais gratuit.

Le joueur choisit les altérations temporaires compatibles à retirer,
dans la limite du maximum. Le lancement est refusé sans coût s'il n'en
existe aucune. Poison, brûlure, ralentissement et aveuglement font partie
des compatibilités proposées ; chaque nouvel effet devra déclarer la
sienne. Les malus d'implant ou d'équipement ne sont pas retirés.

Purgation ne rend pas de PV et ne donne pas d'immunité temporaire.
Une zone dangereuse encore présente peut réappliquer son effet selon
ses règles. Une compétence active ne devient pas utilisable pendant un
silence qui bloque toutes les compétences actives.

**Intention visuelle retenue :** balayage blanc expulsant les fragments
d'altération. Le rendu demeure à présenter.

### SU06 Dernier sursaut

> Vos systèmes d'urgence vous maintiennent en vie lorsque votre corps devrait céder.

| Effet | Par défaut | Actuel dans l'exemple |
| --- | --- | --- |
| PV conservés sur un coup normalement fatal | 1 PV | 1 PV |
| Délai avant un nouveau déclenchement | 100 tours | 64 tours |
| Coût d'énergie | Aucun | Aucun |
| Déclenchement | Automatique si disponible | Automatique si disponible |

**Progression proposée :** délai = ceil(30 + 700 / (n + 9)) tours.
Il vaut 100 tours au rang 1, 67 au rang 10 et 64 au rang 12 ; il
diminue au-delà de 10 en restant strictement supérieur à 30 tours.

Le passif est disponible au début de la partie. Après les protections,
un coup direct normalement fatal laisse le personnage à un PV puis
consomme cette disponibilité. Il ne soigne pas, n'offre pas d'invulnérabilité
et ne bloque pas les attaques suivantes. Les dégâts persistants ne
déclenchent pas ce secours et peuvent tuer le personnage après son activation.

Le délai s'écoule avec les tours réels, même hors combat. Les menus
ne le réduisent pas. Un changement d'équipement ne le remet pas à zéro ;
la durée engagée est celle du rang au moment du déclenchement.

**Intention visuelle retenue :** pulsation et rupture d'un sceau autour
du personnage. Le rendu demeure à présenter.

## Progression comparée

| Effet | Rang 1 | Rang 10 | Rang 12 |
| --- | --- | --- | --- |
| B03 bonus de PV maximum | +10 % | +28 % | +32 % |
| B02 réduction directe | 25 % | 48,7 % | 51,2 % |
| SU03 absorption en part des PV maximum | 5 % | 12,8 % | 13,9 % |
| SU04 soin par tour en part des PV maximum | 4 % | 11,6 % | 12,4 % |
| SU05 altérations retirées au maximum | 1 | 2 | 3 |
| SU05 coût d'énergie | 10 | 9 | 8 |
| SU06 délai de secours | 100 tours | 67 tours | 64 tours |

Ces compétences conservent les points de progression déjà retenus.
Les paliers, les coefficients définitifs, les nouveaux textes et les
rendus gardent leur validation distincte. Le
[menu du personnage de test](REVUE_PERSONNAGE_TEST_AGENT.md) sera traité
après toutes les compétences et animations, selon l'échéance demandée.
