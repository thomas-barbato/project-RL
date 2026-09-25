# Amélioration d'équipement — bonus et effets prédéfinis

25 septembre 2026 — recommandations retenues ; bonus chiffrés et effets spéciaux
persistants par exemplaire testables au laboratoire et premier
[générateur de butin de campagne](GENERATION_EQUIPEMENT.md) raccordé.
Le service d'amélioration auprès d'un PNJ reste à intégrer.

## 1. Décisions confirmées

- Pas de système d'artisanat pour le moment. La fabrication de nouveaux objets
  et les recettes de transformation de matières sont reportées.
- Un PNJ pourra proposer un service sur un équipement existant : le joueur
  place l'objet et les types de ressources requis dans une fenêtre dédiée.
- L'intervention ajoute ou modifie aléatoirement des **bonus**, sans changer
  la base de l'objet. Une épée conserve son identité et ses dégâts de base.
- Les bonus peuvent être des statistiques supplémentaires **ou des effets
  spéciaux**, notamment une attaque de foudre autour du porteur et du vol de
  vie lorsqu'il inflige des dégâts à une cible (précision utilisateur confirmée).
- Les effets appartiennent à un **catalogue conçu à l'avance**. Le hasard
  sélectionne dans cette liste ; il n'invente ni mécanique ni animation.
- Les bonus chiffrés concernent les caractéristiques, le maniement, les PV et
  les autres jauges. Ils peuvent appartenir à des armes, armures ou autres
  équipements compatibles ; la réduction de poids n'est pas le bonus recherché.
- Tous les objets réellement équipés contribuent aux statistiques, y compris
  les armes secondaires. Une arme rangée dans l'inventaire ne contribue pas.
  Changer d'arme active ne fait donc pas varier les jauges.
- Les effets spéciaux d'une arme ne se déclenchent qu'avec cette arme : ils ne
  se transmettent pas aux autres armes équipées. Les propriétés de base restent
  propres à chaque modèle.
- Les bonus seront désignés par des préfixes et suffixes aux significations
  stables. Le [catalogue de conception](AFFIXES_ET_PROVENANCE_EQUIPEMENT.md)
  sépare bases, affixes et paliers ; il n'est pas encore intégré aux tirages du jeu.
- Les objets mélangent fabrications humaines, technologies étranges et
  équipements organiques ou vivants. Le type d'ennemi fixe les sources compatibles :
  un robot ne fournit pas d'équipement humain, même par un tirage exceptionnel.
- Chaque effet possède un déclenchement, une action, des limites, une
  description et une présentation visuelle adaptée aux modes de rendu.
- Les recommandations de l'analyse sont retenues : grand catalogue préparé
  avant distribution, premier ensemble de douze effets, distinction entre
  surprise agréable et perte de contrôle, combinaisons bornées et identité
  des armes préservée. Voir le [catalogue de conception](EFFETS_SPECIAUX_EQUIPEMENT.md).
- Une zone offensive est centrée **sur le point d'impact, en mêlée comme à
  distance**, sauf si sa fiche indique expressément « autour du porteur ».
  Le type d'arme ne décide jamais implicitement de cette origine. Le soin,
  les protections et les effets de soutien conservent leur bénéficiaire propre.

L'utilisateur a envisagé une intervention unique par objet. Cette limitation
reste la piste de travail, pas encore un contrat complet : son application,
les cas d'échec et les règles de conservation du résultat sont à préciser.

Ces décisions ne valident pas les taux, coûts, nombres de bonus, listes de
statistiques ou paramètres de combat. Les modalités suggérées précédemment
(choix d'un bonus à remplacer, protection contre un résultat moins favorable)
ne sont pas automatiquement adoptées.

## 2. Base de l'objet et bonus : deux informations distinctes

L'objet garde sa définition de base. Le résultat de l'intervention appartient
à l'exemplaire concerné, pas à toutes les épées ou armures du même modèle.
Le service ne remplace pas l'objet par une autre base et ne réécrit pas ses
dégâts intrinsèques sous prétexte de lui ajouter une propriété.

Un bonus de dégâts éventuel s'ajouterait dans le calcul, sans changer la base.
Par exemple, « base 10, bonus +2 » ne devient pas une nouvelle base de 12.
C'est une illustration de séparation, pas un bonus de dégâts déjà approuvé.
Les valeurs finales et les effets additionnels doivent être lisibles comme
des conséquences des bonus, pas comme un changement caché de l'objet d'origine.

Un effet de foudre est une action secondaire distincte de la frappe normale.
Un soin est également un effet distinct ; sa présence ne transforme pas les
dégâts de base en une autre statistique. Retirer l'équipement désactive ses
bonus, sans retirer les compétences que le personnage a apprises.

## 3. Contrat d'une entrée du catalogue

Chaque effet devra être décrit avant d'être proposé au tirage. Les valeurs
peuvent être fixes ou tirées dans des bornes prévues, jamais improvisées.

| Élément à définir | Contenu attendu |
|---|---|
| Identité | Identifiant stable, nom joueur et description compréhensible |
| Compatibilité | Équipements et attaques autorisés ; incompatibilités entre bonus |
| Déclenchement | Événement exact : attaque lancée, touche, dégâts réellement infligés, critique ou élimination ; ne pas les confondre |
| Fréquence | Activation certaine ou chance annoncée ; éventuel délai entre activations exprimé dans le temps de simulation |
| Action | Dégâts, soin, état ou autre effet prédéfini ; ordre de résolution |
| Cibles et zone | Origine, portée, forme, obstacles et traitement du porteur, des alliés et des neutres |
| Limites | Puissance, cumul, nombre de déclenchements, interactions et absence de boucles incontrôlées |
| Tirage de l'amélioration | Poids, paramètres variables autorisés et plafond de puissance ; règles de remplacement |
| Présentation | Rendu terminal, intention du futur rendu dessiné, texte et mode d'animations réduites |
| Persistance | Bonus attribué, paramètres tirés et éventuel délai restant conservés après reprise |

Deux opérations aléatoires doivent rester distinctes :

1. **Chez le PNJ :** sélection du bonus et, si prévu, de ses paramètres.
2. **En combat :** test d'activation uniquement pour les effets qui en prévoient un.

Ouvrir une fiche, changer de rendu ou rejouer une animation ne relance aucun
de ces tirages. L'animation montre une résolution du moteur ; elle ne décide
ni les cibles ni les dégâts. Un catalogue prédéfini peut être enrichi plus
tard, mais chaque ajout doit être conçu, borné et vérifié avant activation.

## 4. Origine des zones et deux exemples initiaux

L'origine spatiale et le bénéficiaire sont deux informations différentes.
Une explosion d'impact part de la case de la cible effectivement touchée,
pas du tireur ni du combattant de mêlée. La position est capturée lors de
la touche : une élimination ou un repoussement ultérieur ne la déplace pas.
Un effet qui suit une cible, tel l'Ancrage explosif, doit le dire explicitement.

La décharge initialement demandée **autour du joueur** reste une exception
explicite autour du porteur ; ce n'est pas la règle par défaut des autres
effets de zone. Une version autour de l'impact doit être décrite comme telle.
Ni le soin sur impact ni un bouclier du porteur ne sont transférés à l'ennemi.

Les deux idées suivantes viennent de la demande utilisateur. Les noms sont
des intitulés de conception ; les détails non décidés sont laissés ouverts.
Elles ne constituent pas encore un catalogue complet prêt à charger.

| Intention | Mécanique demandée | Direction visuelle proposée | Paramètres encore ouverts |
|---|---|---|---|
| **Décharge circulaire** | Une attaque de foudre expressément autour du porteur, en complément de sa frappe ; exception `bearer`, indépendante du type d'arme | Arcs courts autour du porteur et impacts sur les cases réellement touchées ; glyphes et motifs électriques en terminal, animation correspondante dans le futur rendu dessiné | Événement et chance d'activation, rayon, dégâts, alliés/neutres et exposition du porteur, délai |
| **Vol de vie** (ancien « soin sur impact ») | Le porteur récupère une fraction des dégâts réellement infligés | Signal de restauration sur le porteur et valeur des PV réellement rendus | Pourcentage, cibles admissibles, plafonds et délai |

L'utilisateur a précisé explicitement qu'il veut du **vol de vie** : le soin
dépend donc des dégâts réellement infligés, sans retirer des PV supplémentaires
à la cible. De même, « foudre autour du joueur » n'implique
pas une chaîne illimitée, des murs ignorés ou une électrification persistante.

La présentation doit respecter les informations autorisées : pas d'arc qui
dessine la position d'un ennemi caché, pas de terrain inconnu révélé par un
flash. Les deux modes rendent les mêmes événements accessibles au joueur.
La couleur seule ne porte pas la signification de l'effet, et réduire les
animations ne supprime ni les règles ni les informations utiles.

## 5. Garde-fous retenus pour les effets déclenchés

Les principes sont retenus ; leurs valeurs restent à équilibrer :

- Un effet secondaire ne redéclenche pas automatiquement toute la chaîne des
  effets de l'équipement. Une décharge ne doit pas créer d'autres décharges
  sans limite, ni multiplier un soin par chaque rebond caché.
- Préciser le traitement des attaques multiples et de zone : un tirage par
  action ou par cible n'a ni la même fréquence ni la même puissance.
- Pour le soin, préciser les cibles valides afin de ne pas offrir une guérison
  illimitée en frappant un mur, un objet inerte ou une invocation gratuite.
- Borner les interactions entre plusieurs pièces équipées et garder les
  délais dans l'état simulé, pas dans la durée de leurs animations.
- Ne pas déduire le budget de puissance du seul niveau actuel du joueur :
  la règle reste à choisir, notamment pour éviter d'inciter à repousser
  indéfiniment une intervention unique jusqu'au dernier niveau.
- Réserver les activations aléatoires aux bénéfices qui ne sabotent pas une
  décision. Un déplacement, une consommation d'état ou un effet majeur
  privilégie une condition lisible, une charge affichée ou une activation
  maîtrisable ; pas de confirmation modale à chaque frappe.
- Filtrer les compatibilités du tirage. Un bonus spécialisé peut être utile
  plus tard, mais ne doit pas être mécaniquement impossible sur son équipement.
- Évaluer la fréquence par action et dans le temps de simulation, pas seulement
  par touche : vitesse, cônes et attaques multiples ne doivent pas multiplier
  gratuitement le soin, les charges ou les explosions.
- Conserver les compétences apprises indépendantes des propriétés d'objet.

## 6. Fenêtre du PNJ et ressources

La fenêtre prévue distingue équipement déposé, ressources requises et
intervention proposée. Toutes ses actions doivent être utilisables au clavier
et à la souris, selon les commandes configurées du jeu.

Avant confirmation, le joueur doit connaître ce qui peut changer et ce qui
reste intact. Le résultat exact demeure inconnu lorsque l'opération est
aléatoire. Aucun aperçu gratuit d'un résultat renouvelable à volonté ne doit
permettre de contourner le coût du tirage.

Restent à décider : consommation des ressources, éventuel prix en monnaie,
quantités, choix du bonus modifié, risque de résultat moins utile, nombre de
bonus spéciaux et traitement d'une annulation ou d'un refus. Le service ne
change pas silencieusement les règles des marchands ordinaires ou des paris.

L'utilisateur a soulevé un risque précis : des ressources purement aléatoires
peuvent rendre une opération impossible. Le passage de recettes à un service
de PNJ ne supprime pas ce risque. Il faudra définir l'approvisionnement et les
éventuelles substitutions avant de fixer les coûts. Un stock marchand lui-même
entièrement aléatoire n'est pas une garantie d'accès.

Les quatre [ressources animales candidates](BUTINS_DES_RENCONTRES.md#11-premières-ressources-de-surface--proposition-à-valider)
ne sont pas automatiquement les ingrédients de ce service. Leurs anciens
usages de fabrication sont reportés. Définir d'abord les opérations utiles,
puis retenir les ressources qui leur donnent un intérêt réel.

## 7. État local et prochaines étapes

Le code conserve déjà des bonus simples par exemplaire :
`src/entity/inventory.rs` définit `MagicItemModifiers` avec les anciens bonus
d'armure et réduction de masse, désormais complétés par les caractéristiques,
précision, pénétration, PV maximum, énergie maximale et dissipation. Le
laboratoire tire 1 à 3 de ces nouveaux bonus sans allègement ; ces nombres
sont des paramètres d'essai, pas l'équilibrage du futur service. Les profils visuels déclaratifs existants, par exemple
`content/core/visuals/radial_damage.json5`, décrivent des séquences terminal.
Ces éléments ne constituent pas le service demandé ni un catalogue générique
de bonus par exemplaire.

La première brique ajoutée est un effet d'arme déclaratif `radial_damage` :
origine `impact` par défaut, `bearer` explicite, déclenchement `on_hit` ou
`on_damage`, exposition du porteur déclarée obligatoirement. Elle résout une
zone secondaire par définition d'effet et attaque normale. La cible visée
effectivement touchée est prioritaire ; à défaut, une case réellement touchée
est choisie dans l'ordre stable des coordonnées. Les réactions ne déclenchent
pas cette brique et les dégâts secondaires ne relancent pas les effets d'arme.
La propagation et sa présentation passent par les événements existants.

Aucun profil de cette brique n'est distribué dans la campagne. Le
[laboratoire de test](LABORATOIRE_DE_TEST.md), accessible depuis le menu principal,
permet seulement d'en essayer des variantes temporaires. La brique
n'ajoute ni les douze effets complets, ni une chance d'activation, ni des charges,
ni une interface PNJ, ni des bonus spéciaux persistants par objet. Les valeurs
des tests ne sont pas des paramètres d'équilibrage. Le catalogue précise les
limites de ciblage de cette première brique avant toute distribution.

Ordre de travail retenu : fiches des douze effets et origines explicites,
modèle des propriétés par exemplaire et sauvegarde, déclenchements/charges
bornés, intégration et essais des effets, puis service et approvisionnement.
Le tirage, les coûts et la marque d'intervention devront se résoudre ensemble,
sans perte ni double application.
Si la limite d'une intervention est retenue, vendre, déposer, racheter ou
suspendre ne devra pas la réinitialiser.

Le laboratoire dispose aussi d'un premier vol de vie (50 % des dégâts directs
admissibles, arrondi inférieur, au plus 3 PV par attaque et jamais au-delà des
PV manquants). Ses mannequins sont autorisés uniquement par un marqueur de test.
Ce prototype ne distribue aucun bonus en campagne et ne fixe ni les
cibles finales, ni la fréquence d'équilibrage. Voir le catalogue
et le laboratoire pour les garde-fous et les variantes mêlée/tir.

Validation future : conservation exacte de la base, compatibilités, tirages
bornés, absence de récursion, cibles réellement touchées, soin plafonné aux PV
manquants, persistance, contrôle clavier/souris et rendu dans les limites de
perception. Les anciennes parties devront conserver leurs propriétés ; aucune
modification de sauvegarde ou intégration rétroactive n'est autorisée ici.

Le laboratoire comprend maintenant Marquage : marque inerte par
cible, compteur visible, détonation au troisième coup et expiration sans
explosion après abandon. Comme les autres profils de ce banc d'essai, elle
ne constitue pas encore une propriété spéciale par exemplaire ni un service
PNJ ; les dégâts de base restent inchangés. Au 25 septembre, Ricochet remplace
Alternance et Catalyse devient un cône depuis le porteur qui fait exploser
chaque cible déjà brûlante. Écho différé reste en réexamen. Le laboratoire compte
54 armes (Ricochet uniquement en fusil), témoin et Onde d'impact compris,
plus deux armures témoins avec/sans bonus aléatoires.
Les descriptions sont courtes et naturelles, les limites importantes séparées.
Distribution en campagne,
propriétés spéciales par exemplaire et service PNJ restent à réaliser.
