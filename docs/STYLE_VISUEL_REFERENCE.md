# Style visuel de référence

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
