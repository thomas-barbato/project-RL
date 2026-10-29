# Style visuel de référence

## Règle commune de cohérence visuelle — VIS-COH01-RULE01

Consignée le **29 octobre 2026**, à la demande du joueur. Cette règle rassemble
les décisions visuelles déjà acceptées pour les cartes, décors, personnages,
monstres et équipements actuels et futurs. Elle sert aussi lors des retouches.

**Chaque nouvelle carte, silhouette ou pose doit rester cohérente avec les
références validées et avec les autres éléments visibles dans le jeu.**
La qualité se juge sur leur composition à taille native, dans le moteur.
Dungeon Crawl Stone Soup reste la référence de lisibilité et de qualité ;
les images acceptées du projet fixent sa projection et son traitement exacts.

### Projection, échelle et dessin communs

- Conserver la projection `STYLE-RL-2026-10-18-r01` : axes du sol horizontaux
  et verticaux parallèles, cases carrées, murs courts et faces avant visibles.
  Comparer directement au camp et à la rue acceptés ; ne pas inventer un
  autre angle de caméra pour une nouvelle carte.
- Garder **64 × 64 px par case**, les proportions relatives des acteurs,
  des ouvertures et des équipements. Une toile plus grande permet les marges
  ou un décor sur plusieurs cases ; elle ne change pas l'échelle du monde.
- Accorder les contours, les masses de couleur, les ombres et la densité
  de détails des décors à ceux des personnages. Privilégier les silhouettes
  lisibles, les matières mates et l'usure située et irrégulière. Les surfaces
  calmes et les détails utiles doivent rester nets à taille de jeu.
- Une ambiance ou un matériau peut varier selon le lieu en conservant cette
  cohérence. La simplification ne se fait ni par flou ni par agrandissement
  d'un ancien petit dessin.

### Cartes et raccords

Les sols, murs, coins, portes, seuils et meubles suivent les mêmes repères
de projection et d'échelle. Dessiner les éléments entiers, avec leurs pieds
et leurs contours, puis vérifier leurs raccords et leur superposition.
Le décor doit conserver des passages lisibles et praticables pour le joueur
et les monstres. Sa boîte d'image ne définit pas une collision.

À l'extérieur, conserver les sols sans quadrillage décoratif ni damier
suggérant de fausses cases. Les matériaux intérieurs restent ceux de leurs
lots validés ; l'accord sur les plaques des salles et le caillebotis usé du
camp ne transforme pas tous les lieux en copies de ce camp.

### Personnages, équipement et animation

Un même personnage conserve son identité, son gabarit, ses vêtements,
ses protections, ses matières et ses accessoires asymétriques entre la face,
la marche et les différentes orientations. Les changements de pose ou de
recouvrement des volumes ne justifient pas une autre tenue ou une variation
arbitraire de taille ou de niveau de détail.

Conserver les figures entières et cohérentes : tête, torse, membres et appuis
suivent l'orientation montrée. Les nouvelles phases ne se fabriquent pas par
greffe de jambes, cisaillement ou déplacement de bandes. Chaque morphologie
de monstre conserve ses supports et ses appendices propres.

Les bras, gants et repères de prise proviennent du même cycle et de la même
projection que le corps. Composer séparément le corps, les équipements portés,
les armes communes et les mains, avec des attaches stables et un ordre de
profondeur cohérent. Le casque suit la tête réellement dessinée. L'armure du
joueur s'adapte à sa classe ; le dessin d'une arme reste partagé, avec sa
prise et son placement adaptés. Les règles acceptées de marche et de retour
de face restent celles de WALK12, ARM14 et DUAL15.

### Éclairage et contrôle avant intégration

Les acteurs et les décors restent cohérents dans les lumières réelles du
lieu. À position et éclairage identiques, un passage entre repos, respiration
et marche doit conserver une exposition et des matières cohérentes ; un
changement de chemin de rendu ne doit pas éclaircir ou assombrir soudainement
le personnage. L'ambiance peut évoluer avec le lieu ou le déplacement réel.

Présenter les candidats numérotés à taille native avant intégration, sauf
accord explicite déjà reçu pour ces candidats. Pour une carte, montrer le
décor avec les acteurs et objets existants. Pour un personnage, comparer face,
marche gauche/droite, équipement et retour au repos dans les mêmes conditions.
Vérifier les raccords, les prises, les occultations et l'exposition dans le
moteur, puis conserver les sources et la preuve de validation.

Cette règle ne modifie pas le statut des anciennes propositions : les accords
restent limités à leurs dessins et à leur périmètre. Les identités de contenu,
les règles de jeu et les sauvegardes restent conservées. Les dessins sources
appartiennent à `assets/`, les décisions à `docs/reviews/` et les captures
reproductibles à `artifacts/`, selon [la règle de rangement](GESTION_FICHIERS_GENERES.md).

Références complémentaires : [direction artistique](DIRECTION_ARTISTIQUE_TILES.md),
[marche WALK12](reviews/player-walk-12/REGLE-MARCHE-12.md),
[bras ARM14](reviews/walk-arm-projection-14/acceptance.json),
[ambidextrie DUAL15](reviews/dual-walk-15/acceptance.json) et
[exposition des poses](reviews/visual-polish-01/WALK-LIGHT01.md).

### Sols continus — accord extérieur du 28 octobre 2026

Le joueur valide SOL-CONT01-r01, version 02, sans quadrillage décoratif à
l'extérieur : béton continu pour les cheminements et asphalte sans grandes
fausses dalles. Les bordures physiques, objets et marquages restent conservés.
La matière du camp est acceptée avec une demande de réparer les pieds de décor
et les fragments de damier restants. La suggestion de caillebotis 03 est ensuite
refusée ; elle ne fait pas partie du style retenu. Le béton corrigé 04 est
ensuite écarté : des restes de dallage subsistent aux extrémités. Le joueur
demande une direction plus futuriste pour l'intérieur, puis valide 05, pont
technique, maintenant intégré. Les cinq salles gardent leurs grandes plaques
métalliques bleu-gris mates. Le caillebotis des couloirs 06 est ensuite validé
et intégré ; le joueur demande de l'user davantage par endroits. La finition
07 est ensuite validée et intégrée à l'identique : patine irrégulière sur les
barres, plus présente près des murs. Les salles 05, les ouvertures sombres de
la maille 06 et l'éclairage sont conservés. Les accords et contrôles sont dans
`reviews/floor-future-01/`. Accord extérieur et contrôles :
`reviews/floor-continuous-01/`.

### Égouts — réseau d'eau continu demandé le 28 octobre 2026

Le joueur demande de relier toutes les portions d'eau et de conserver quelques
passages praticables pour lui et les monstres. La construction doit produire
un réseau unique par jonctions cardinales, continu sous les grilles ; le sol
praticable doit rester relié depuis l'escalier. Le contrôle porte sur chaque
plan utilisé par le tirage aléatoire. La nouvelle eau EG-EAU01 reçoit un retour
positif, puis les nouveaux tracés EG-CAN02 sont validés explicitement. La
génération 167 les utilise avec un contrôle des cases praticables et des
emplacements du joueur et des monstres. Les sauvegardes antérieures gardent
leurs anciens tracés, avec le traitement d'eau accepté.
Règle, candidats et vérifications : `reviews/sewer-water-01/CANAUX02.md`.

### Finition commune POLISH01-02 — accord du 27 octobre 2026

Le joueur valide la version 02 avec les éclats d'impact et demande son adaptation
à toutes les cartes. Le rendu commun renforce la présence des silhouettes, les
volumes et les lumières existantes, avec des surfaces plus calmes. Les décors
peints conservent leur éclairage calculé ; les sols générés très sombres et les
acteurs déjà en pleine exposition reçoivent une adaptation préservant leurs
détails et teintes. Le dessin, la projection et les cases de 64 px sont conservés.

La Frappe de rupture reçoit le bref accent lumineux et les six éclats montrés,
désactivés en mouvement réduit. La lumière ne traverse ni les murs ni les zones
non perçues. Les sources et les règles de jeu restent conservées. Accord, pixels
acceptés et contrôles : `reviews/visual-polish-01/`.

### Finition des acteurs — ART-FINISH-01, accord du 19 octobre 2026

Le lot pilote complet est validé : treize dessins, six configurations de mains
et sept échantillons de palettes, comparés dans deux scènes éclairées. La preuve
et les références natives sont dans
`reviews/refonte-histoire-2026-10-16/art-finish-review-01/`.
Les matières mates, l'usure située et les volumes plus nuancés du lot prolongent
la finition CIVILS-U01-08. Conserver la projection, l'échelle, les silhouettes
lisibles et les couches d'équipement. L'accord concerne les ressources et les
configurations montrées ; les autres familles restent à harmoniser par lots.

**STYLE-RL-2026-10-18-r01 — validé le 18 octobre 2026.**

Le joueur demande de conserver ce style exact, son angle et ses réglages.
Cette référence accompagne les règles de `DIRECTION_ARTISTIQUE_TILES.md`.
L’accord sur les pixels et les données est figé dans
[visual-acceptance-r07.json](reviews/refonte-histoire-2026-10-16/first-sector-review-01/visual-acceptance-r07.json).

## Images à comparer directement

- Projection et volumes : camp accepté **PLACE-CAMP-01 r09b**, source
  `assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/r05/camp-reference-user-r05.png`.
- Dessin du décor : **SCENE-U01-07-r01**, source intacte
  `assets/source-art/first-sector-review-01/u01-style-r07/SCENE-U01-07-r01.png`.
- Résultat réel éclairé, avec le personnage à taille native :
  `assets/source-art/first-sector-review-01/u01-style-r07/references/street-native-lighting.png`
  et `references/home-native-lighting.png` dans le même dossier.
- Réglages : `assets/source-art/first-sector-review-01/u01-style-r07/lighting.json`.

Les empreintes des fichiers exacts figurent dans l’accord. Ces références
durables ne dépendent pas des captures temporaires sous `artifacts/`.

## Projection et échelle

La vue est une projection dessinée en 2D depuis le dessus, avec la faible
hauteur apparente des murs et les faces avant visibles du camp. Le plan du
sol garde des axes horizontaux et verticaux parallèles et des cases carrées.
Les murs, ouvertures et meubles suivent cette même projection. Aucun angle
numérique de caméra 3D n’est validé : les images du camp et de la rue sont
la référence exacte à comparer, plutôt qu’un degré supposé.

Les anciennes vues en élévation frontale, vues aériennes trop verticales,
volumes à points de fuite et sols inclinés en losange sont écartés. Une
nouvelle composition ne doit pas entraîner une autre caméra.

Une case mesure **64 × 64 px**, avec un rapport identique entre les deux axes.
Les silhouettes, équipements et meubles sont contrôlés ensemble à taille
native. Les marges transparentes des sprites ne fixent pas leur taille physique.
La voiture occupe au plus deux cases sur une, soit 128 × 64 px, flammes comprises.
Les futurs gros décors peuvent occuper plusieurs cases.

## Traitement du dessin

Les surfaces sont stylisées et regroupées en masses simples, avec des
contours foncés affirmés et une ombre lisible par matériau. Le détail se
concentre sur les signes utiles : ouverture, volume, état du bâtiment,
équipement ou fonction. Les sols et toitures conservent des plages calmes.

Le grain réaliste, les rayures minuscules et les éclats répétés ont été
réduits pour accorder les décors aux personnages, monstres et armes existants.
La netteté est conservée ; cette simplification ne se fait pas par flou,
agrandissement des petits dessins ou perte de résolution.

Les bâtiments gardent des silhouettes, matériaux, usages et dégâts variés.
Le style commun ne doit pas produire des alignements de copies. Le mobilier
est regroupé selon les lieux et laisse les passages compréhensibles.

### Finition usée de la rue — accord du 19 octobre 2026

Le joueur valide **STYLE-MAP-U01-04** après avoir demandé la carte entière et
neuf vues à taille native. La rue étendue reçoit la finition de surface montrée :
teintes légèrement ternies, salissures mates, peinture usée et éclats irréguliers.
Le traitement s'applique au décor avant les acteurs et l'éclairage, ainsi qu'aux
toits et à la voiture. Il conserve les contours, la projection, les passages et
les sources de lumière existants. Les personnages ne sont pas retravaillés.

L'accord concerne cette carte et ce rendu précis. Les quatre éléments visibles
près de la prison (GRIM-U01-01 et CORP-U01-01/02/03) reprennent leurs dessins et
placements montrés, comme décors au sol. Les sources et empreintes acceptées sont
dans `reviews/refonte-histoire-2026-10-16/street-defects-review-04/acceptance.json`.
Les ressources normales sont rangées dans
`assets/maps/prison-street-expanded-01/decor/`.

Références durables du rendu accepté, dans
`reviews/refonte-histoire-2026-10-16/street-defects-review-04/references/` :
`map-native-lighting.png` pour la carte entière, `prison-native.png` et
`interior-native.png` pour les vues natives avec personnages et éclairage.

Le petit patch de sol et l'entrepôt redessiné de la revue locale r03 ne sont pas
activés. La station médicale GRIM-U01-02 reste écartée. L'ambiance GRIM-L02 ne fait
pas partie de cet accord ; les vraies lumières et leur ambiance restent celles
de la rue montrée. Le camp et les anciennes petites cartes gardent leur rendu.

### Dallage irrégulier — accord du 19 octobre 2026

Le joueur valide et demande l'intégration de **PAV-U01-06**, révision des
joints irréguliers : supprimer les traits intérieurs parasites, atténuer les
séparations trop uniformes et faire disparaître l'herbe répétée sur chaque
dalle. Les joints du nouveau dallage de l'extension varient entre plus nets,
interrompus et presque effacés. Leur répartition est fixe par emplacement ;
elle ne change pas pendant le jeu. De petites touffes séparées apparaissent
par groupes rares et irréguliers près des bordures et murs.

La finition sombre validée reste appliquée par le moteur. Les dalles gardent
leur taille native de 64 px ; la composition du départ sud et les sols
intérieurs sont conservés. Cet accord porte sur ce lot et ses placements,
sans changer les règles de collision ou les éclairages. Les sources et
contrôles sont dans `reviews/refonte-histoire-2026-10-16/street-paving-review-06/`.

### Raccords et entrepôt U01 — accord du 20 octobre 2026

**U01-REPAIRS-09** est validé : entrepôt en L moins saturé, contours moins
uniformément noirs, asphalte débarrassé des bordures échantillonnées par
erreur, haies complètes et annexe entière au raccord sud. Les trois petits
objets près de l’entrepôt sont retirés. Les dalles qu’ils masquaient reprennent
les joints fragmentés ou presque effacés PAV-U01-06B/C, selon la même rotation
fixe que le reste du quartier. Conserver les pièces entières et composer le
sol sous les objets, sans découper les voisins. La caméra, l’échelle et les
vrais éclairages restent ceux de la rue validée. L’accord et les preuves
normales sont dans `reviews/refonte-histoire-2026-10-16/street-repairs-review-09/`.

### Corps civils et sang — accord du 19 octobre 2026

**CIVILS-U01-08** est validé et intégré : cinq dessins de corps, dix-huit
placements dont cinq en intérieur, et dix-neuf décors de sang. Le lot conserve
l'échelle native, la projection, la finition de surface et les vrais éclairages
de la rue. Le dessin complet et isolé des objets, la variation sobre des
matières/vêtements, les blessures lisibles et les placements liés aux lieux
servent de référence de finition. Les règles détaillées sont en tête de
`DIRECTION_ARTISTIQUE_TILES.md`.

Les pixels, emprises et orientations acceptés sont conservés dans
`reviews/refonte-histoire-2026-10-16/street-civilians-review-08/acceptance.json`.
Les corps intérieurs restent masqués tant que leur toit est présent. Le joueur
demande de comparer cette qualité aux personnages, armures, armes et monstres ;
aucune refonte de ces autres dessins n'est validée par cet accord.

## Éclairage de cette rue

**LUM-U01-07-r01** donne une pénombre froide lisible, avec des points chauds
localisés près des bâtiments et du portail. Le boîtier est dessiné ; l’émission
de sa lentille, son halo sur les surfaces et l’éclairage des acteurs sont
calculés dans le rendu. Les nappes lumineuses peintes ne constituent pas le
système d’éclairage.

Le système du camp est réutilisé : atténuation des sources, occlusion statique
par la grille des murs, illumination du premier mur atteint et composite GPU
du monde complet, avec l’interface hors de l’éclairage. Les corps ne projettent
pas d’ombres mobiles dans ce système.

Valeurs de cette version : lumière ambiante RGB **[0.56, 0.61, 0.72]**, quatorze
sources et émission des lentilles **[1.0, 0.82, 0.46]**. Les positions, rayons,
puissances, masques et données d’irradiance exacts sont dans les fichiers
validés ; ces paramètres graphiques ne sont pas des chiffres de combat.

Cette ambiance nocturne est celle de la rue actuelle. Les lieux suivants
peuvent avoir leur propre ambiance validée, en gardant la même projection,
la même lisibilité et la cohérence du trait.

## Toits — accord du 19 octobre 2026

Les deux toits **ROOF-U01-01A/B r01** et leur masquage sont acceptés dans
`reviews/refonte-histoire-2026-10-16/street-roofs-review-01/acceptance-r01.json`.
La porte ouverte dévoile l'intérieur ; le toit reste également masqué tant
que le joueur est dedans, même porte fermée. Dehors et porte fermée, le toit
est visible. Les toits sont des couches séparées, sans lumière intérieure
traversant leur face supérieure. Les lampes et la pénombre de la rue restent
celles de LUM-U01-07. Les prochains bâtiments visitables suivent ce principe.

Le correctif **DOOR-U01-FIX-01**, demandé le 19 octobre, dégage le seuil de
la maison et place son luminaire sur le mur à droite. Sa source réelle
suit ce déplacement ; l’ambiance, les treize autres lumières et les sources
graphiques approuvées restent identiques. La porte et le toit gardent leurs
règles validées. Voir `doorway-fix-r01.json` dans la revue des toits.

## Étendue des prochaines cartes

Le joueur précise que les prochaines cartes seront **plus grandes** que
cette première rue. On augmente leur surface jouable, en conservant les
cases de 64 px et les proportions des acteurs et des décors. Leurs dimensions
exactes et leurs plans seront présentés dans leurs revues ; aucun nouveau
nombre de cases n’est fixé par cette demande.

## Vérification des prochains lots

Comparer chaque nouveau décor à ces sources, puis le montrer avec les
sprites existants à taille native. Vérifier caméra, proportions, entrées,
raccords, collisions et passages avant intégration. Conserver les sources,
identifiants et validations. Une variante de disposition ou de secteur ne
valide pas automatiquement de nouveaux textes, effets ou règles de jeu.
