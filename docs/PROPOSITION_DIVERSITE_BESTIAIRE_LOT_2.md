# Diversité du bestiaire — proposition de deuxième lot

27 septembre 2026. **Les huit noms sont validés et raccordés en générations 114–116.**

Après Guetteur et Spectre, le joueur valide explicitement les six noms restants :
Cracheur des mares, Chauve-souris des ruines, Riveuse, Écailleux des cavernes,
Traînard et Gueule du vide. Cette validation ne transforme pas les profils
encore à développer en rencontres déjà jouables.

La reprise autorisée après validation des noms porte d'abord sur ces deux profils.
Voir les règles et vérifications dans [Guetteur et Spectre](GUETTEUR_ET_SPECTRE.md).
Le point de départ ci-dessous décrit l'état antérieur à cette intégration :
le total passe de neuf à onze identités de bestiaire, sans compter les profils
génériques ni les cibles du laboratoire.

La v115 intègre ensuite le [Cracheur des mares](CRACHEUR_DES_MARES.md) :
**trois rencontres du lot intégrées, cinq encore à développer**, soit douze
identités de bestiaire jouables au total. Les cinq noms restants sont validés.
La génération 116 termine ces cinq intégrations : voir
[le rapport du lot](INTEGRATION_BESTIAIRE_LOT_2.md). Le total atteint 17 identités,
hors profils génériques et mannequins.

## Revue des noms — nouvelle proposition

La première liste a été jugée trop proche de noms d'objets ou de formes.
La réponse suivante (« Feuilleté », « Entrelacé », « Dédoublé », « Annulaire »)
n'a pas été validée : l'accord portait sur la reprise du travail, pas sur ces
appellations. Elles ne deviennent donc pas les noms du lot.

Les cinq remplacements ci-dessous retracent la dernière proposition, désormais
validée avec Cracheur des mares, Chauve-souris des ruines et Riveuse.

Correction validée par le joueur : **Guetteur** et **Spectre**, sans qualificatif.
Les six autres noms ont ensuite été validés ensemble.

| Proposition précédente | Nouvelle proposition | Origine du nom courant |
|---|---|---|
| Rosace de pierre | **Écailleux des cavernes** | Les lamelles minérales évoquent des écailles qui se referment. |
| Relais vivant | **Guetteur** | Repère une présence et transmet l'alerte près des relais. |
| Ruban dédoublé | **Traînard** | Son rythme lent peut tromper avant une action brusquement rapide. |
| Écho fendu | **Spectre** | Une silhouette étrange, divisée en deux contours joints. |
| Anneau brisé | **Gueule du vide** | Son ouverture centrale évoque une gueule béante. |

Ces appellations ne modifient ni le corps décrit, ni les comportements.
« Écailleux » n'en fait pas un reptile ; « Spectre » n'ajoute ni mort-vivant,
ni intangibilité, ni traversée des murs. « Gueule » est une image de la forme,
pas une promesse de morsure ou de dévoration : son centre reste sûr pendant
l'expansion annoncée. Le Guetteur reste un organisme, pas un habitant humanoïde
ou une machine par simple emploi d'un nom de rôle.

Les silhouettes décrites dans le catalogue restent la référence. Cette revue
ne fixe pas de nouveau peuple, de taxonomie ou d'histoire de campagne.

## Point de départ vérifié

Les populations et rencontres du monde régional déclarent 14 comportements
distincts : 12 profils de combat et deux comportements d'animaux neutres.
Il ne s'agit ni de 14 espèces, ni du nombre d'adversaires sur une carte.

- Cinq profils génériques : Traqueur, Tirailleur, Sentinelle historique,
  Artilleur et Soigneur de terrain. Les deux derniers ont une identité
  humanoïde explicite ; ne pas reclasser rétroactivement les trois autres.
- Sept identités hostiles du bestiaire : Mordeur des friches, Fouisseur pâle,
  Brise-os, Ver cuirassé, Anémone des caves, Hurleur des failles, Sentinelle machine.
- Deux animaux neutres : Grignoteur et Dos-rond. Le premier peut se défendre
  acculé ; le second se protège. Ils ne sont pas des ennemis ordinaires.

Cela représente **neuf identités de bestiaire intégrées**, dont huit biologiques
ou minérales et une machine. La Sentinelle machine et le profil de garde
historique restent distincts malgré leur nom commun. Les profils ayant des
statistiques ou équipements différents ne sont pas comptés comme autant d'espèces.

Références de contrôle : `simulation.json5`, `AiBehavior`, légende et
identification des acteurs dans le client. Les cibles du laboratoire, dangers
inertes, invocations alliées et PNJ de service ne sont pas comptés comme ennemis.
Le catalogue de conception propose 36 identités et six rôles humanoïdes ; il
n'est pas un inventaire de 42 types déjà jouables. Les biomes `core:network`
et `core:corrupted` n'ont actuellement ni faune ni population propre déclarée.

## Direction du prochain ensemble

Étendre la variété des décisions, des habitats et des rythmes, plutôt que
la seule quantité de PV ou la densité. Les huit rencontres ci-dessous reprennent
des fiches du catalogue existant, avec des noms courants validés selon la
charte validée. Elles ne valident pas leurs anciennes appellations de famille.

Les profondeurs sont des cibles de placement proposées, indépendantes du
niveau du joueur, dans le monde de prototype actuel. Les bandes de niveau
113 feront référence ; les anciennes valeurs numériques du catalogue ne
seront pas reprises automatiquement. Les objectifs de touches sont indicatifs,
pour du matériel ordinaire adapté ; ils ne plafonnent pas le bénéfice d'un bon objet.

| Nom retenu | Correspondance actuelle du catalogue | Milieu proposé | Rôle et réponse du joueur |
|---|---|---|---|
| **Cracheur des mares** | Ostrèle des noues | Surface humide, fossés et mares | Fragile, 1–3 touches ; jet irritant annoncé sur une case. Se déplacer ou rompre la ligne de tir. Pas de flaque acide ajoutée implicitement. |
| **Chauve-souris des ruines** | Véline des beffrois | Ruines de surface autour d'un nid | Fragile, 1–3 touches ; piqué annoncé puis retrait obligatoire. Les obstacles coupent l'approche ; le vol ne traverse pas les murs. Nom validé ; conserver la silhouette ailée du catalogue. |
| **Riveuse** | Riveuse M-4 | Maintenance/production, couches 1–3 | Machine de chantier, non hostile par défaut, danger local de ligne de travail ; 3–5 touches si combat. Contourner l'axe ou interrompre une tâche par une interaction accessible. Ne devient pas un chasseur poursuivant toute la carte. |
| **Écailleux des cavernes** | Lithère feuilletée | Cavités sèches/recherche, couches 2–4 | Mobile, protection orientée vers une menace réellement perçue ; viser 3–5 touches depuis un angle non protégé. Contourner plutôt que frapper une grande réserve de PV. |
| **Guetteur** | Orve maillée | Réseaux, couches 4–7 | Fragile, 1–3 touches ; transmet une alerte locale entre voisins compatibles. Interrompre la chaîne ou éviter sa perception. Ni invocation infinie ni connaissance de la position cachée du joueur. Organisme anormal, pas robot piratable. |
| **Traînard** | Orve à contretemps | Réseaux, couches 4–7 | Intermédiaire, 3–5 touches ; deux rubans superposés, rythme lent puis action rapide annoncée. État visible persistant ; pas de surprise fondée sur la vitesse d'une animation. |
| **Spectre** | Réman bifide | Corruption, couches 5–7 | Fragile, 1–3 touches ; une attaque et un écho sur une seconde case annoncée. Chaque impact est distinct et borné ; l'écho n'est pas un second ennemi, ni une seconde récompense. |
| **Gueule du vide** | Aphre annulaire | Corruption, couches 5–7 | Intermédiaire, 3–5 touches ; expansion annulaire annoncée, centre sûr. Se placer dans une zone sûre puis profiter de la récupération. Portée bornée, pas un anneau couvrant toute la carte. |

Ce lot propose quatre fragiles, trois intermédiaires et une machine de
travail conditionnellement dangereuse. Il comprend **sept organismes/créatures
et une machine**, sans transformer les habitants en robots. Les attaques,
hostilités précises et coefficients devront être définis dans les fiches
d'intégration, avec leur contre-jeu avant le raccordement aux régions.

## Butins : provenance définie, objets utiles avant les tables

Pas de second tirage universel d'arme à la mort et pas de nouvelle matière sans
usage pour donner artificiellement l'impression que chaque créature rapporte.

- Cracheur et Chauve-souris : BIO-C. Aucun équipement fabriqué ; tant qu'un
  reste organique utile n'est pas défini, aucun objet biologique ajouté.
- Écailleux : MIN-C. Pas d'armure fabriquée sortant de son corps ; ressource
  minérale uniquement après définition d'un usage réel.
- Riveuse : MEC-C. Composants et outil réellement installés, récupération
  selon leur état ; aucun équipement humain ni outil intact recréé après destruction.
- Guetteur et Spectre : ANO-C ; Traînard et Gueule du vide : ANO-R. Aucun résidu
  tant que sa matérialité et son usage ne sont pas définis. ANO-R ne garantit
  pas un objet rare. Le même individu ne peut multiplier ses récompenses.

Les caches de sites gardent leurs tables séparées et leur provenance propre.
Cette proposition ne crée aucun nouveau nid contenant du butin, objet marchand,
recette, ingrédient d'amélioration, prix ou taux de drop.

## Ordre d'intégration — noms validés

1. Cracheur + Guetteur + Spectre : profils fragiles avec tir fixe,
   perception/alerte bornée et deux impacts annoncés. Réutiliser les primitives
   d'attaque et de perception ; développer séparément les règles qui manquent.
2. Écailleux + Gueule du vide + Traînard : protection directionnelle, couronne avec centre
   sûr et rythme persistant. Ces différences nécessitent du travail moteur,
   pas seulement des définitions avec d'autres couleurs.
3. Chauve-souris + Riveuse : approche/retrait aérien et tâche de chantier
   interruptible. Ne pas simuler le vol par un passe-muraille ni une tâche par
   une IA de chasseur renommée. Réutiliser la récupération électronique pour
   les pièces réellement présentes de la machine.

Les huit fiches forment un ensemble conçu avant l'intégration. Les trois
passes sont un ordre technique, pas un ajout au hasard d'une nouvelle créature
à chaque tour de conversation. Toute restriction qui empêcherait une identité
promise doit être signalée, pas masquée par un nom prétendument définitif.

Chaque passe devra vérifier les habitats, les groupes, les arrivées sûres,
les synergies, les dégâts et annonces, la persistance, les anciennes sauvegardes,
les noms/glyphes/CAPTEURS et la distinction entre rendu terminal et futur rendu
illustré. Ajouter de la variété sans augmenter automatiquement les effectifs.

## Suite de l'intégration

Les noms du lot sont tous validés. Poursuivre les étapes techniques décrites
ci-dessus, sans confondre validation du nom et intégration du comportement.
Le catalogue reste extensible au-delà de ses 36 fiches actuelles.
