# DEFENSE-U01-10 — proposition de défense du portail

Demande du 20 octobre 2026 : dessiner deux gardes à la porte d’arrivée,
avec des sacs de sable devant eux, davantage de cadavres civils, ennemis et
militaires, ainsi que des flammes à certains endroits. Le joueur précise
ensuite vouloir voir le résultat sur la carte avant toute installation.

## Dessins et composition proposés

- **GARD-U01-01 r03** et **GARD-U01-02 r03** : deux sentinelles en acier clair,
  casque fermé et Gatling sombre tenue à deux mains. Les masses claires du
  métal détachent les gardes du béton. Leurs pieds sont à (468,3930) et
  (586,3930) px, devant les piliers et dans les poches des barricades. Les
  poses fixes ne définissent pas encore un
  système d’équipement échangeable ni une animation de tir.
- **SACS-U01-02 r03** : barricade complète en L, exportée entière dans une
  toile de 80 × 72 px et réfléchie horizontalement pour l’autre côté. Les
  retours rejoignent les montants ; une ouverture de 64 px demeure au centre.
  Les ressources sont posées à (416,3864) et (560,3864) px, avec une ombre de
  contact distincte. Les gardes sont dessinés après ces protections pour
  conserver la bonne superposition des corps et des armes.
- **MORT-U01-10-01 à 12** : quatre corps de cultistes, quatre militaires et
  quatre civils. Réutilisation des dessins déjà acceptés ; transformations
  entières par rotations à angle droit, sans redessiner ou couper les corps.
  Les placements sont liés aux approches, replis, sorties et dégâts. Six
  décors de sang existants complètent ces placements.
- **FEU-U01-10-01 à 04** : quatre foyers dans les débris de la ruine du portail,
  le moteur d’une camionnette détruite, une façade effondrée et les gravats de
  la cour. Animation peinte existante, décalages de phase, petites braises et
  vrais éclairages chauds dans le composite du moteur.

Les sources générées avec l’outil imagegen intégré, les prompts exacts, les
exports natifs et les placements sont dans
`assets/source-art/first-sector-review-01/u01-defense-r10/`.
La r01 (fusil et arme d’appui, postes avancés) a été refusée après la première
capture native : « ils doivent avoir l’air de garder la porte », avec demande
d’armures métalliques et de Gatling. Ses sources restent archivées ; la r02
remplace uniquement cette proposition, sans activer de personnage en jeu.
La r02 est ensuite jugée peu convaincante. Le joueur demande de voir la
nouvelle composition en L et précise « tu peux la faire en metal clair ».
Cette demande autorise la préparation de **r03**, pas son activation normale.
Le recadrage r02 et son manifeste sont conservés dans `rejected-r02/`.
Le joueur signale aussi un manque général d’armures métalliques : à reprendre
dans un futur lot d’équipements avec sa propre revue, sans étendre ce lot à
toutes les armures ni inventer de statistiques.
Après le signalement d’une stature trop petite, le corps opaque a été comparé
à la tenue FIN-01 du joueur : les premières versions mesuraient tous 60 px
de haut à l’alpha ≥ 128. Le petit casque, la posture et le volume masqué par
les protections réduisaient néanmoins la présence des gardes. La r03 est
conditionnée à nouveau depuis ses grands originaux, avec une silhouette de
62 px dans une toile 64 × 64 px et une marge d’un pixel en haut et en bas.
Les pieds restent exactement aux mêmes coordonnées ; aucun sprite de jeu
déjà réduit n’est agrandi. La planche `references/stature-native.png` aligne
le joueur et les deux gardes sur le même sol, à un pixel pour un pixel.
Cette correction reste soumise à la revue visuelle du joueur.
Les barricades r03 occupent 80 × 72 px ; les petites lignes
de sacs r01/r02 restent archivées et ne sont plus placées. Les objets sont conditionnés entiers, avec alpha
et marges, sans étirement. La projection demeure celle de la carte acceptée.

Le corps CIV-U01-04-20 et ses deux décors de sang sont déplacés ensemble de
(+24,-104) px dans la proposition, en avant du poste. Ce dégagement évite une
superposition avec les nouveaux sacs ; les douze corps supplémentaires du
lot restent présents. Ces placements et les empreintes des sources sont dans
`proposal.json`. Aucun de ces déplacements n’est activé sur la carte normale.

## Portée de la revue

Ce lot est **une proposition visuelle, non intégrée**. Aucun acteur réel,
nouvelle faction, quête, arme ou règle de combat n’est ajouté. Les gardes
figurants respirent légèrement pour juger leur présence ; ils ne tirent pas.
Les incendies n’infligent pas de dégâts dans ce prototype. La validation
artistique ne doit pas être interprétée comme une validation de statistiques
ou de nouvelles règles de combat.

Les foyers utilisent des masques de réception extérieurs propres : une épave
ou un morceau de toiture peut recevoir leur lumière même lorsque sa case
est solide dans la simulation. Ces masques sont explicités dans
`lighting-receivers.json` et ne modifient ni collisions ni vision ni intérieur
visitable. Les lumières ordinaires gardent leur calcul séparé ; seule la
contribution des quatre feux est ajoutée à leur irradiance. Les halos ne sont
pas peints sur les sprites de flamme ou le fond de carte.

L’aperçu corrige aussi un défaut de fabrication découvert pendant ce lot :
le générateur partagé utilisait encore 48 × 36 cases pour calculer certaines
irradiances de la rue étendue, qui en mesure 26 × 66. Le helper se base
désormais sur l’image réellement reçue ; les cartes d’irradiance de cette
proposition font 208 × 528 échantillons (un pour 8 px), contre 384 × 288 dans
la ressource normale héritée. La ressource normale n’est pas écrasée dans ce
lot de prévisualisation ; le recalage est montré avec la proposition pour
validation. Garder ce contrôle de dimensions lors de l’intégration finale.

Le rendu est activé uniquement par le mode de diagnostic
`--ui-cold-street-defense-10`, exclu de la compilation release. Le lancement
normal et les sauvegardes du joueur conservent leur carte et leurs acteurs.
Le manifeste de la proposition réutilise le fond normal et possède ses propres
décors et cartes d’éclairage. Les empreintes de toutes les ressources normales
sont vérifiées avant et après fabrication.

Le contrôle de passage utilise une copie de la carte : les quatre cases
proposées pour gardes et sacs deviennent solides sur cette copie seulement.
La voie centrale reste praticable, avec un trajet A* depuis la porte vers la
rue. Les corps restent de simples décors au sol. Le prototype ne consomme pas
de tours ni de ressources lors du rendu et ne modifie aucune sauvegarde.

La page courante `artifacts/street-defects-review-01/index.html` présente les
scènes avant/après à leur taille native, deux boucles animées et une carte
entière à parcourir. Cette dernière retire le masque de vision uniquement
pour examiner la répartition ; les scènes conservent le cercle de six cases.
Les captures retenues et leurs empreintes sont conservées ici dans
`references/` et `checks.json`. L’accord du joueur reste à recueillir.
