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

SU03 est ensuite renommée **Barrière d’épine** à la demande du joueur,
avec un buff de renvoi à la place de l'absorption. Le saignement est
explicitement exclu du renvoi. Le joueur distingue ensuite les deux
compétences : SU03 renvoie les dégâts directs de mêlée ; BM05, le glyphe,
renvoie les dégâts magiques directs. Le poison et tous les dégâts indirects
sont exclus des deux effets. Le joueur valide ensuite la fiche individuelle
de SU03 : texte, buff actif de cinq tours, renvoi de 30 % au rang 11 et de
31 % au rang 12, coût de 15 énergie et recharge de dix tours. Le calcul
sur les dégâts réellement subis, leur application directe à l'attaquant
sans nouvelle réduction et l'absence de chaîne de renvois sont retenus.
Les autres conditions non montrées et le rendu restent à revoir. Les deux
buffs sont regroupés dans **Buffs → Protection et renvoi** conformément
à AR-BUFFS-01. Aucun de ces effets n'est encore intégré au jeu.

SU04 est ensuite renommée **Régénération** à la demande du joueur, qui
valide sa fiche : soin de 16 PV par tour au rang 11 et de 17 PV au rang 12
avec 130 PV maximum conservés dans les deux aperçus, canalisation de trois
tours maximum, coût de 6 énergie par tour et recharge de huit tours.
La canalisation occupe l'action de chaque tour maintenu ; une autre action
ou une perte de PV l'interrompt. L'arrêt volontaire entre deux tours et
l'arrêt à vie pleine sont retenus. Les ennemis continuent d'agir.
Les autres conditions non montrées et le rendu restent à revoir ;
[la fiche validée](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SU04-02.json)
conserve l'identifiant SU04.

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
résistances, Égide occulte, puis Dernier sursaut si le coup restant devait
tuer. Le renvoi proposé de SU03 se calcule sur les dégâts réellement
subis après les protections. Les effets persistants suivent leurs
propres règles et ne deviennent pas des coups directs.

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

### SU03 Barrière d’épine

> Votre protection se hérisse d’épines occultes qui punissent ceux qui vous frappent au corps à corps.

**Fiche individuelle validée :** actif, buff temporaire. Le profil conserve neuf
rangs appris et deux d'équipement, soit un rang effectif de 11. Le prochain
achat mène au rang effectif 12, en conservant l'équipement et les attributs.

Renvoie à chaque attaquant une portion des dégâts directs de mêlée
qu'il vous inflige.

**Effet actuel**

- Dégâts de mêlée renvoyés : **30 %**.
- Durée : **5 tours**.

**Prochain rang — 12 · 1 point de compétence**

- Dégâts de mêlée renvoyés : **31 %**.

**Coût : 15 énergie · Recharge : 10 tours.**

Le renvoi est calculé sur les dégâts réellement subis après vos protections.
Vous subissez le coup normalement. Les dégâts renvoyés
s'appliquent directement à l'attaquant, sans nouvelle réduction par ses
défenses. Le poison, le saignement et tous les dégâts indirects sont exclus.
Un renvoi ne déclenche pas d'autre renvoi.

**Calculs techniques proposés, hors texte joueur :** pourcentage de
renvoi = 10 + 40 × (n − 1) / (n + 9) ; énergie = 12 + floor((n − 1) / 3).
Durée de cinq tours et recharge de dix tours fixes. Le pourcentage continue
au-delà du rang 10 en restant inférieur à 50 %. Les arrondis des dégâts,
les composantes mixtes d'une attaque, les cas mortels, les sources sans attaquant
et les règles de cumul ou relance restent à préciser pour revue.

La fiche validée est conservée dans
[sa revue individuelle](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SU03-04.json).
Le joueur conserve BM05 Sceau de vindicte, appelé « glyphe » dans la
discussion, en réservant son renvoi aux dégâts magiques directs. Son
barème et son descriptif complet restent à revoir individuellement.

**Rendu visuel :** l'ancienne intention d'absorption est retirée de cette
proposition. La nouvelle intention de renvoi et son rendu restent à présenter.

### SU04 Régénération

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
progresser au-delà de 10. Les PV maximum issus de Vitalité et de Vitalité améliorée
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

### SU05 Purge

> Vous chassez les poisons et les effets néfastes de votre corps.

Le nom, la phrase roleplay exacte et la priorité automatique au plus récent
sont demandés par le joueur. Il précise ensuite que le nombre maximum
d'effets purgés d'un coup doit augmenter avec le rang. Cette décision
remplace l'interprétation précédente d'un retrait unique fixe.
Le barème exact et les autres paramètres restent soumis à revue.

Retire automatiquement des effets néfastes.

**Effet actuel — rang 10, huit rangs appris et deux d'équipement**

- Effets néfastes retirés : **jusqu'à 2**.

**Prochain rang — 11 · 1 point de compétence**

- Effets néfastes retirés : **jusqu'à 3**.
- Coût : **8 énergie**.

**Coût : 9 énergie · Recharge : 6 tours.**

Peut retirer le poison, la brûlure, le ralentissement et l'aveuglement.
Les malus d'équipement et d'implant sont exclus. Ne rend pas de vie et
ne protège pas contre une nouvelle application de ces effets.

La [fiche révisée](../docs/reviews/refonte-agent-2026-10-06/review-individual-description-SU05-04.json)
conserve l'identifiant SU05. Les coûts et la recharge restent proposés.

**Calculs techniques provisoires, hors texte joueur :** maximum d'effets =
1 + floor((n − 1) / 5) ; énergie =
ceil(10 × (1 − 0,4 × (n − 1) / (n + 9))). Le maximum proposé augmente
par paliers, y compris au-delà de dix avec les bonus d'équipement.
L'exemple au rang 10 montre un palier : le maximum passe de deux à trois
et le coût de neuf à huit au rang 11. Les rangs 11 et 12 conservent tous
deux un maximum de trois et un coût de huit ; tous les rangs ne changent
donc pas les valeurs. Ces seuils et le coût restent proposés.

Les compatibilités citées, l'absence de soin ou d'immunité et le refus
sans coût en l'absence d'effet compatible restent proposés. Les égalités
de date d'application et le rafraîchissement d'un effet existant restent
à préciser. Une zone dangereuse peut réappliquer son effet selon ses
propres règles ; l'interaction avec le silence reste à revoir.

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
| SU03 part des dégâts directs de mêlée renvoyée, nouvelle proposition | 10 % | 28,9 % | 31 % |
| SU04 soin par tour en part des PV maximum | 4 % | 11,6 % | 12,4 % |
| SU05 maximum d'effets retirés par utilisation, proposition révisée | 1 | 2 | 3 |
| SU05 coût d'énergie | 10 | 9 | 8 |
| SU06 délai de secours | 100 tours | 67 tours | 64 tours |

Ces compétences conservent les points de progression déjà retenus.
Les paliers, les coefficients définitifs, les nouveaux textes et les
rendus gardent leur validation distincte. Le
[menu du personnage de test](REVUE_PERSONNAGE_TEST_AGENT.md) sera traité
après toutes les compétences et animations, selon l'échéance demandée.
