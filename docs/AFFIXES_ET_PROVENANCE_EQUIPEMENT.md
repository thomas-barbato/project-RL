# Affixes et provenance des équipements

25 septembre 2026 — conception retenue dans son principe ; nouvelles bases et
distribution ci-dessous restent principalement **hors campagne**. Le
[raccordement jouable](GENERATION_EQUIPEMENT.md) distribue dix-huit bases P1 à P6 dans les
régions habitées de surface et les caches souterraines des nouvelles parties. Les valeurs chiffrées sont des essais proposés,
pas un équilibrage validé ni un changement des plafonds du personnage.

Premier raccordement réalisé : les dix affixes chiffrés ont maintenant une
identité persistante et des noms composés dans le laboratoire (tirages P1).
Les effets spéciaux sont des profils persistants par exemplaire. Braise et
Décharge sont éligibles au générateur commun ; les autres restent au laboratoire.

## 1. Décisions retenues

- La base possède son identité, ses dégâts et son comportement propres.
- Les préfixes et suffixes désignent des propriétés prédéfinies. Les valeurs
  peuvent varier ; « de vitalité » ne change pas de signification au gré du tirage.
- Tous les objets équipés contribuent aux bonus chiffrés, y compris les armes
  secondaires. Changer l’arme active ne fait pas varier les jauges.
- Les effets spéciaux d’une arme ne se déclenchent qu’avec cette arme.
- Les objets rangés ne contribuent pas. Augmenter une réserve ne la remplit pas.
- Le monde mélange objets humains, technologies étranges, organiques et vivants.
- Le type d’ennemi fixe les objets admissibles : aucun équipement humain sur
  un robot. Le hasard intervient ensuite à l’intérieur de ce groupe.

Les [144 bases](CATALOGUE_BASES_EQUIPEMENT.md) et les 22 familles d’affixes
sont décrites dans [le catalogue structuré](catalogues/equipements.json).
Dix-huit bases sont des définitions chargées : six couteaux, six fusils et six vestes.

## 2. Ordre des tirages et persistance

Profil de source → familles compatibles → palier de base → modèle → qualité
→ groupes d’affixes compatibles → valeurs.

Le tirage de possession a lieu à la création de l’individu, celui d’une cache
à la création du lieu. La mort transfère ce qui reste réellement présent :
pas de nouvelle arme sans rapport avec celle utilisée, ni de recharge des
consommables épuisés. Ouvrir la fiche ou revenir sur une carte ne relance rien.

Un groupe vide donne **aucun équipement**, pas un tirage dans le catalogue entier.
Les restes exploitables et le contenu du lieu ont leurs propres règles ;
une créature organique sans arme ne reçoit pas de matériel fabriqué par défaut.
Le budget, la fréquence et le nombre d’objets ne sont pas fixés par ce catalogue.

Choisir d’abord une famille évite qu’ajouter beaucoup de modèles dans une famille
augmente silencieusement sa fréquence. L’outil d’aperçu choisit les familles
compatibles uniformément ; les poids finaux par source restent à régler.

## 3. Composition proposée des objets

- Blanc : aucun affixe ; nom et profil de base.
- Pour le premier essai nommé : 1 à 3 affixes différents et au maximum un effet
  spécial par arme. C’est une limite de prototype, pas la rareté finale.
- Un même groupe ne se cumule pas sur un exemplaire, même avec deux paliers.
- Le palier d’un effet spécial indique son accès, pas une augmentation automatique
  de son rayon, de ses cibles, de sa durée ou de ses dégâts.
- Les affixes offensifs ne sont pas distribués sur les armures sans déclencheur
  défensif dédié. Cela n’interdit pas de futurs effets propres aux protections.
- Ricochet est réservé aux familles à projectiles ; pas aux lames ou à un cône
  de flammes simplement parce qu’il s’agit d’une attaque à distance.
- Écho différé reste exclu des nouveaux tirages en attendant la décision sur sa
  suppression. Alternance reste remplacé par Ricochet.

Le stockage conserve l’identifiant de la base et, pour les nouveaux bonus chiffrés
du laboratoire, ceux des affixes, leurs paliers et leurs valeurs, ainsi que
l'identifiant du profil d'effet spécial éventuel.
Le nom affiché est une présentation, pas la donnée
à partir de laquelle recalculer les bonus. Dépôt, revente et reprise conservent
ces informations sans les tirer à nouveau.

## 4. Noms courts, détails complets

On affiche au plus un préfixe et un suffixe dans le nom. Toutes les propriétés
restent chiffrées et mises en évidence dans la fiche, sans répéter leur affixe
ni afficher de tiret de liaison. Si plusieurs affixes se disputent
une place, une priorité déclarée puis l’identifiant stable les départagent ;
le rendu ne tire jamais au hasard. L’effet spécial est prioritaire parmi les suffixes.

En français, un « préfixe » mécanique peut être placé après le nom :
**Épée de garde précise de vitalité**. Les accords masculin/féminin et
singulier/pluriel sont définis explicitement.

Exemple à trois affixes : **Épée de garde précise de siphon** ; sa fiche peut
indiquer Précision +12, PV maximum +15 et la description du vol de vie.
Le bonus de PV ne disparaît pas parce qu’il n’entre pas dans le titre.

## 5. Affixes chiffrés

### Rareté propre à chaque affixe

Chaque affixe possède un `weight` indépendant de la qualité de l'objet. Le tirage
est pondéré parmi les propriétés compatibles restantes, sans doublon de groupe.
Les poids provisoires des effets sont : Braise 20, Percussion 16, Acide 12,
Perforation 10, Décharge/Conduction/Ricochet 8, Couronne/Égide 6,
Siphon/Marquage/Fournaise 4. Les poids chiffrés vont de 35 à 100 ; voir
[les valeurs actuellement distribuées](GENERATION_EQUIPEMENT.md#tirage-commun-et-rareté).
Ce sont des tickets relatifs, pas des pourcentages. La rareté n'ajoute aucun dégât.

Les bonus ci-dessous utilisent des propriétés prises en charge par le moteur.
Le laboratoire génère des affixes nommés P1 dans l’inventaire ; le premier
générateur de campagne utilise désormais P1 à P6, pondérés par la profondeur.
Aucun allègement n’est tiré.

| Identité | Nom masculin / féminin | Propriété | P1 | P2 | P3 | P4 | P5 | P6 |
|---|---|---|---|---|---|---|---|---|
| affix:power | puissant / puissante | Puissance | +1 à +2 | +1 à +3 | +2 à +4 | +3 à +5 | +4 à +7 | +5 à +9 |
| affix:coordination | adroit / adroite | Coordination | +1 à +2 | +1 à +3 | +2 à +4 | +3 à +5 | +4 à +7 | +5 à +9 |
| affix:resilience | robuste / robuste | Résilience | +1 à +2 | +1 à +3 | +2 à +4 | +3 à +5 | +4 à +7 | +5 à +9 |
| affix:perception | de vigilance | Perception | +1 à +2 | +1 à +3 | +2 à +4 | +3 à +5 | +4 à +7 | +5 à +9 |
| affix:processing | lucide / lucide | Traitement | +1 à +2 | +1 à +3 | +2 à +4 | +3 à +5 | +4 à +7 | +5 à +9 |
| affix:accuracy | précis / précise | Précision | +5 à +10 | +7 à +14 | +10 à +18 | +14 à +23 | +18 à +28 | +23 à +35 |
| affix:armor_penetration | pénétrant / pénétrante | Pénétration d'armure | +1 à +1 | +1 à +2 | +2 à +3 | +3 à +4 | +4 à +5 | +5 à +7 |
| affix:vitality | de vitalité | PV maximum | +5 à +15 | +10 à +20 | +15 à +30 | +20 à +40 | +30 à +55 | +40 à +75 |
| affix:energy_reserve | de réserve | Énergie maximale | +5 à +15 | +10 à +20 | +15 à +30 | +20 à +40 | +30 à +55 | +40 à +75 |
| affix:heat_dissipation | de dissipation | Dissipation par tour | +1 à +1 | +1 à +2 | +2 à +3 | +3 à +4 | +4 à +5 | +5 à +7 |

La précision est un bonus au score, pas des points de pourcentage ajoutés
directement à la probabilité finale. Le moteur conserve ses limites de touche.
Les nombres élevés devront être revus avec l’élargissement prévu des statistiques
et les cumuls entre emplacements ; aucun tirage de P6 n’est injecté maintenant.

## 6. Affixes d’effets spéciaux

Les mécaniques ci-dessous existent dans le laboratoire. Leur profil prédéfini
est désormais référencé par chaque exemplaire, séparément de sa base et de ses
bonus chiffrés. Le laboratoire les attribue explicitement pour couvrir les essais ;
leur distribution aléatoire en campagne reste à intégrer.

| Suffixe | Effet existant | Accès proposé |
|---|---|---|
| de décharge | Onde d'impact | P1+ |
| de couronne | Décharge circulaire | P2+ |
| de conduction | Conduction | P2+ |
| de braise | Brandon | P1+ |
| d'acide | Suintement caustique | P2+ |
| de siphon | Vol de vie | P2+ |
| de perforation | Perforation | P2+ |
| d'égide | Égide d'impact | P2+ |
| de percussion | Percussion | P1+ |
| de marquage | Marquage | P3+ |
| de ricochet | Ricochet | P2+ |
| de fournaise | Catalyse | P3+ |

L’origine reste celle de l’effet : impact par défaut, autour du porteur sur
mention explicite ; Catalyse part du porteur vers la cible. Les valeurs et
limites actuelles sont dans [le laboratoire](LABORATOIRE_DE_TEST.md).
Un objet vivant ne gagne ni soin autonome, ni faim, ni attaque supplémentaire
simplement par sa nature. Ces idées exigeraient des propriétés dédiées.

## 7. Profils de profondeur proposés

Poids sur 10 000, applicables **après** le filtre de provenance. Ce sont des
paramètres de simulation du catalogue, pas les taux des parties actuelles.

| Profondeur de référence | P1 | P2 | P3 | P4 | P5 | P6 |
|---|---|---|---|---|---|---|
| 0 | 7000 | 2200 | 600 | 160 | 35 | 5 |
| 1 | 3000 | 4000 | 2000 | 750 | 200 | 50 |
| 2 | 1200 | 2600 | 3800 | 1700 | 550 | 150 |
| 3 | 500 | 1300 | 2300 | 3600 | 1700 | 600 |
| 4 | 200 | 600 | 1400 | 2600 | 3400 | 1800 |
| 5 et plus | 100 | 250 | 650 | 1700 | 3300 | 4000 |

Tous les paliers restent possibles dans une famille autorisée. Une trouvaille
exceptionnelle ne contourne jamais la nature de l’ennemi. L’outil conserve le
dernier profil au-delà de la profondeur 5 ; cela ne limite pas les futures couches.
Le danger propre de la rencontre et les règles particulières des paris devront
être ajoutés comme contextes distincts, sans adaptation universelle au joueur.

## 8. Vérifier sans modifier le jeu

```text
node --test tools/equipment_catalogue.test.mjs
node tools/equipment_catalogue.mjs robot 3 42
node tools/equipment_catalogue.mjs humanoid_equipped 2 42
node tools/equipment_catalogue.mjs organic_creature 5 42
```

Les aperçus sont reproductibles et en lecture seule. Le générateur de démonstration
a son propre RNG ; il ne remplace pas le RNG du moteur. Les tests couvrent les
provenances, l’absence de repli universel, les fourchettes, les groupes exclusifs,
les compatibilités, les accords français et la conservation des propriétés dans
la fiche. Ils ne prouvent ni l’équilibrage en combat ni une intégration jouable.

## 9. Intégration et suite

1. Réalisé pour les bonus chiffrés : identités et valeurs persistantes, conservation au dépôt/ramassage et à la vente/reprise ; anciens caches binaires repris via le journal vérifié.
2. Réalisé : noms composés, accords et détail des affixes ; effets spéciaux persistants par exemplaire, utilisés uniquement par l'arme qui attaque. Le laboratoire utilise ces propriétés sans changer les dégâts de base.
3. Ensemble réalisé : dix-huit bases, trois familles complètes P1 à P6, générateur commun pondéré et caches de surface/souterraines en génération 106. Reste à intégrer d'autres familles utiles et à équilibrer les valeurs.
4. Premier raccordement en génération 108 : [armes réellement portées et récupérables](EQUIPEMENT_DES_ENNEMIS.md) sur Artilleurs et Soigneurs. Inventaires complets, armures et autres profils restent à intégrer.
5. Marchands et paris raccordés en génération 107 : [règles et limites](COMMERCE_EQUIPEMENT.md). Avant l'achat d'un pari, seul le modèle et son prix sont révélés : aucun préfixe, suffixe, bonus ou effet spécial. Équilibrage et service d'amélioration restent séparés.

Ce premier raccordement ne crée ni artisanat, ni nouvel emplacement de main,
ni nouvelle monnaie, ni effet offensif sur les armures. L'extension des possessions
des ennemis et le service d'amélioration restent à intégrer.
