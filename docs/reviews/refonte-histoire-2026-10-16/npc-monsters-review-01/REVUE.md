# NPC-MONSTERS-01 — première série de la nouvelle campagne

16 octobre 2026 ; mise à jour le 17 octobre. **Galerie initiale : apparences
01–07 et 09–18 validées ; seul le 08 reste en revue. Aucun raccord au jeu effectué.**

La revue suivante `SECTOR-BOSSES-11` présente séparément les 18 r02 et 19 r01,
validés visuellement dans l'accord 07 à 192 et 288 px de haut. Le joueur
trouve ensuite le 19 peut-être trop haut : un gabarit r02 de 224 px est
proposé, sans changer son dessin. Les anciennes références acceptées restent
intactes. La diversité insectoïde et lovecraftienne est retenue pour la suite.

Le joueur précise ensuite que cette diversité concerne les monstres ordinaires
et les élites. Les boss de secteur doivent évoquer des chefs. Les dessins 20–21
sont appréciés mais réaffectés à des propositions d'élites plus petites dans
`ELITE-MONSTERS-16` ; leurs nouveaux gabarits et leurs ombres restent à examiner.

L'accord explicite « ok je valide :) » porte sur le responsable des missions
01 — r03 présenté juste avant. La source et les deux exports natifs sont figés
dans [visual-acceptance-01.json](visual-acceptance-01.json), avec empreintes.
Les fichiers acceptés sont conservés dans `assets/source-art/`, indépendamment
des aperçus temporaires. Aucun acteur de l'ancien prototype n'est réaffecté :
le nouveau responsable n'a pas encore d'identité de contenu dans la campagne
jouable. L'accord visuel ne crée ni nom propre, dialogue, règles ou animation.

L'accord suivant « oui c'est parfait on continue :) » confirme les cultistes
masqués 13 r03, 14 r04 et 15 r03 et les hybrides simplifiés 10–12 r02 de la
présentation précédente. Les six sources et PNG 64 px sont figés dans
[visual-acceptance-02.json](visual-acceptance-02.json), indépendamment des
aperçus. Le pipeline réutilise les pixels acceptés, sans reconditionnement.
Les huit autres candidats précédents gardent leur statut. Les identités
de contenu de cette nouvelle campagne ne sont pas encore raccordées aux
acteurs du prototype ; aucune règle ou rencontre n'est créée par cet accord.

Le joueur confirme ensuite les trois apparences 16–18 r01 par
« je les aimes bien oui :) ». La troisième acceptation visuelle,
[visual-acceptance-03.json](visual-acceptance-03.json), conserve les sources,
empreintes et PNG natifs exacts du garde, du prédicateur et du Maréchal.
Les noms restent provisoires et les règles ne sont pas validées par cet accord.
Les huit candidats 02–09 gardent leur statut antérieur, sans nouvelle validation.

Le joueur valide ensuite les PNJ de services 02–04 r02 par « je valide :) ».
Leurs grandes sources et PNG natifs exacts sont consignés dans
[visual-acceptance-04.json](visual-acceptance-04.json). Seules ces trois
apparences courantes sont acceptées ; les r01 précédentes restent non validées.
Les candidats 05–09 gardent leur statut antérieur. Aucun nom, règle de service,
dialogue, animation ou raccord à un acteur jouable n'est déduit de cet accord.

Le joueur valide ensuite les révisions 05–07 r02 par « je valide :) ».
[visual-acceptance-05.json](visual-acceptance-05.json) fige les grandes sources,
les PNG natifs exacts et leurs empreintes. Cet accord visuel porte sur
l'ouvrier possédé, le coureur greffé et le Porte-presse présentés dans la
planche de la revue 08 ; il n'étend pas l'acceptation aux anciennes r01.
Les 08–09 restent en revue. Les noms et les règles ne sont pas validés par
cet accord et aucun nouvel acteur n'est raccordé au jeu.

Le joueur valide ensuite la correction charnelle du 09 r03 par « oui je valide ».
L'accord [visual-acceptance-06.json](visual-acceptance-06.json) fige sa source
et son PNG natif exact. Il ne valide pas le 08 r02, qui reste inchangé en revue.
Le joueur précise que le 09 ne sera pas le boss des plans gelés : c'est une
créature du plan glacé, sans rôle de boss de secteur. Sa catégorie de rencontre
reste à définir. Plusieurs autres boss sont possibles, avec un seul grand boss
par secteur ; cette direction est consignée dans `encounter-direction-10.json`.

Le joueur demande de créer maintenant les PNJ de missions et de services et
les monstres de la nouvelle campagne. Sa direction confirmée est une descente
vers des créatures de plus en plus démoniaques et répugnantes, avec des
hybrides de chair et de machines dans les secteurs intermédiaires. Cette
direction remplace le peuplement de l'ancien prototype comme base de création.
Le plan démoniaque final demeure l'enfer de glace de l'histoire courante ;
les autres secteurs ne reçoivent pas automatiquement son esthétique gelée.

## Taille courante — décision du 17 octobre

Le joueur annule la version du jeu en 32 px. L'aperçu courant ne fabrique
et n'affiche désormais que les exports natifs 64 px et leurs zooms entiers.
Les anciens fichiers 32 px et leurs validations restent des archives ;
l'export 64 px accepté de NPC-01 demeure strictement identique.

## Candidats

### Progression corrigée — cultistes humains d'abord

Corrections suivantes du joueur : capuches, uniforme commun glauque et aucun
visage visible ; puis masques physiques à la place de l'ouverture noire.
Les candidats courants sont **13 r03, 14 r04 et 15 r03** : anthracite et
bordeaux, masque ivoire à fentes noires, peau et yeux entièrement cachés.
Ils restent des humains ordinaires. La magie existe dans cet univers et
le joueur autorise des cultistes lanceurs de sorts : le 15 devient une
proposition d'occultiste, avec focaliseur violet tenu et geste de préparation.
Aucun sort précis, projectile, dégât, durée, recharge ou effet de combat
n'est défini ou intégré par ce dessin. Les ouvertures noires et visages
des essais précédents sont conservés comme versions remplacées.
Trace : `hooded-cultists-04.json`, puis `masked-cultists-05.json`.

Le joueur demande que les premières menaces soient des humains ordinaires
du culte, moins dangereux dans la progression que les possédés et les
hybrides. Les 13–15 proposent trois rôles de cette **même famille humaine**,
sans mutation ni greffe. Les numéros sont des références de revue stables,
pas l'ordre des apparitions. Le 05 possédé et les hybrides 06, 07, 10–12
ne constituent plus le premier contact ennemi ; leurs emplacements précis
restent à définir. Les démons des profondeurs conservent leur direction.

Les 10–12 reçoivent une proposition r02 simplifiée : moins de marques
d'usure, traits internes et subdivisions, avec un canon, deux cuves ou
deux bras-câbles comme signe dominant. Palette de conditionnement limitée
à 16 couleurs pour ces six nouveaux dessins. Les identités et le nombre
de membres sont conservés ; les sources r01 restent disponibles pour la
comparaison. Ces révisions visuelles ont depuis été validées dans l'accord 02.

| Numéro | Identifiant stable | Intitulé provisoire | Intention visuelle |
| --- | --- | --- | --- |
| 01 | NPC-01 | Responsable des missions | Révision r03 : officier trapu en veste courte olive, baudrier ivoire, casque à visière saillante, visage inférieur visible et pistolet au holster. |
| 02 | NPC-02 | Armurier | Apparence r02 validée : silhouette trapue, chemise turquoise, grand tablier sombre et mécanisme d'arme tenu à deux mains. |
| 03 | NPC-03 | Secouriste | Apparence r02 validée : manteau olive et ivoire simplifié, grande mallette médicale portée à la main. |
| 04 | NPC-04 | Technicien | Apparence r02 validée : salopette orange, veste olive, protections auditives et avant-bras prothétique tenant une clé. |
| 05 | MON-01 | Ouvrier possédé | Apparence r02 validée : veste de travail jaune, pantalon bleu-vert et grande griffe ; moins de marques et respirateur retiré. |
| 06 | MON-02 | Coureur greffé | Apparence r02 validée : quadrupède bas, thorax converti et pince mécanique simplifiés. |
| 07 | MON-03 | Porte-presse | Apparence r02 validée : élite massive, bras-presse hydraulique et noyau thoracique encadré d'os ; greffes simplifiées. |
| 08 | MON-04 | Fendeur | Révision r02 proposée : démon organique, tête fendue et grandes griffes osseuses ; dents et détails internes simplifiés. |
| 09 | MON-05 | Amas du gel | Apparence r03 validée : créature du plan glacé à six appuis, chair plissée et trois pointes de glace ; pas un boss de secteur. |
| 10 | MON-06 | Tireur greffé | Possédé au bras-canon anatomique, vestige de tenue industrielle ; proposition de menace à distance. |
| 11 | MON-07 | Porte-cuves | Conversion médicale, deux cuves hautes et quatre appuis ; proposition de soutien. |
| 12 | MON-08 | Entraveur | Enrouleur industriel envahi de chair, deux bras-câbles et deux jambes ; proposition de contrôle. |
| 13 | CULT-01 | Cultiste à matraque | Humain masqué, uniforme anthracite/bordeaux et capuche ; matraque courte, contact. |
| 14 | CULT-02 | Tireuse du culte | Même uniforme, capuche et masque ; pistolet séparé du corps, distance. |
| 15 | CULT-03 | Occultiste du culte | Même uniforme, capuche et masque ; focaliseur violet tenu, lanceur de sorts proposé. |
| 16 | ELITE-01 | Garde du culte | Élite humaine : uniforme et masque du culte, bouclier industriel rectangulaire et masse courte. |
| 17 | ELITE-02 | Prédicateur du culte | Élite humaine : mantelet large, tenue longue et focaliseur violet ; silhouette de lanceur de sorts. |
| 18 | BOSS-01 | Maréchal des Chaînes | Dessin conservé, gabarit r02 de 192 px validé dans l'accord 07. |
| 19 | BOSS-02 | Mère Suture | Apparence et ancien gabarit r01 de 288 px validés ; nouveau gabarit r02 de 224 px proposé à la demande du joueur, dessin conservé. |
| 20 | ELITE-03 | Masse des relais | Dessin apprécié, rôle d'élite ; export réduit proposé de 98 × 72 px, avec ombre séparée. |
| 21 | ELITE-04 | Insectoïde de forge | Dessin apprécié, rôle d'élite ; export réduit proposé de 109 × 64 px, avec ombre séparée. |

### Suite 06 — deux élites et premier boss

Les 16–17 restent humains, à la même hauteur de corps que les cultistes.
Leur distinction vient de l'équipement, de la silhouette et de la pose,
avec l'uniforme anthracite/bordeaux et le masque ivoire déjà acceptés.
Le 18 reprend le démon de guerre au carcan ouvert de l'histoire actuelle.
Son corps et ses cornes occupent 96 px de haut, contre 60 pour les humains,
sur une référence de case 64 px. Le canevas 256 px et son ancrage servent
au débordement visuel ; ils ne définissent aucune collision nouvelle.

Les trois apparences sont validées dans l'accord 03. Le bouclier du 16 ne
définit pas un taux de blocage ; le rôle magique du 17 n'approuve aucun
sort ou bonus ; le 18 ne crée aucun effet de chaîne, renfort ou règle
d'arène. Les noms restent provisoires. Prompts et références sont dans
`elites-boss-06.json`. La planche `elites-boss-16-18.png` montre les trois
à taille native et au zoom ×2, puis sur sol de référence. Il s'agit de
montages de revue, sans intégration active.

### Suite 07 — PNJ de services, révisions 02–04 r02

La suite reprend les trois identités de service existantes dans cette revue,
avec des dessins simplifiés et un accessoire principal par métier. Le 02
porte un mécanisme d'arme devant son tablier sombre ; le 03 porte une mallette
médicale ivoire et brique ; le 04 tient une clé avec sa prothèse claire.
Les petites attaches, poches et pièces d'armure répétées sont réduites.
Leurs proportions humaines, coiffures et identités sont conservées ; les
trois propositions r01 non validées restent disponibles comme sources.

Ces trois apparences r02 sont **validées dans l'accord 04** et leurs PNG exacts
figés dans `accepted/` du dossier de sources de cette revue.
Elles ont été exportées directement depuis leurs grandes sources en
64 px, corps de 60 px de haut, canevas 128 px et pieds sur le même ancrage.
Leur palette de conditionnement est limitée à 20 couleurs. Les numéros restent
02–04 : il ne s'agit pas de trois PNJ supplémentaires.

`services-07.json` conserve les prompts exacts, cibles de révision, références
de style, sources et empreintes. `services-02-04.png` montre les trois à taille
native, au zoom ×3 et sur sol de référence. Le responsable 01 déjà validé et
les autres apparences acceptées ne sont pas retouchés. Les couches de corps,
bras, doigts et accessoires restent à fabriquer séparément après revue.
Ces dessins ne définissent aucun tarif, stock, soin, entretien, réparation,
compétence ou nouveau service. Le technicien reste une proposition de métier,
sans ajout imposé d'un atelier dans chaque refuge.

### Suite 08 — possédé et premiers hybrides, révisions 05–07 r02

La suite reprend les trois concepts précédemment appréciés : ouvrier possédé,
coureur greffé et Porte-presse. Leurs identifiants et numéros restent inchangés.
Les sources r01 restent disponibles ; les apparences r02 sont ensuite validées
dans l'accord 05, avec leurs PNG natifs figés dans `accepted/` des sources.

Le 05 conserve sa veste jaune, son pantalon bleu-vert et sa grande griffe,
avec le respirateur retiré et moins de petites marques. Le 06 conserve quatre
appuis, sa posture basse et son membre mécanique à pince, avec des formes
de métal et des greffes simplifiées. Le 07 conserve deux jambes, deux bras,
le bras-presse et le moteur thoracique encadré d'os, avec des pièces plus larges
et moins de détails d'usure. Les machines des 06–07 sont intégrées à l'anatomie,
pas des armes équipées et tenues.

Les gabarits maximaux précédents sont conservés : 60 px de haut pour le 05,
58 pour le 06 et 76 pour le 07, sur une case de référence 64 px. Les images
sont directement exportées depuis les nouvelles grandes sources, en palette
de 16 couleurs et alpha binaire, avec le même ancrage au sol. Les canevas et
les tailles visuelles ne changent aucune collision.

La planche `possedes-hybrides-05-07.png` montre les trois à taille native,
au zoom ×2 et sur sol de référence. `possessed-hybrids-08.json` conserve
les prompts exacts, cibles de révision, référence de style, sources et
empreintes. Ces révisions sont désormais validées visuellement ; les treize
apparences acceptées auparavant restent strictement identiques. Aucun raccord
au rendu du jeu n'est effectué par cette revue.

Les premières rencontres restent les cultistes humains ordinaires. Aucun
changement de vitesse, dégâts, portée, compétence, statut, placement ou
animation n'est défini par cette revue. Les noms restent provisoires.

### Suite 09 — profondeurs et plan glacé, 08 r02 et 09 r03

Les deux derniers concepts de la première série sont repris avec des masses
plus simples. Le 08 conserve sa tête verticale fendue, son crochet crânien,
ses deux bras et deux jambes et ses longues griffes osseuses. Les petites dents
et subdivisions internes sont réduites. Il reste entièrement organique, sans
habillage glacé ajouté. Cette proposition r02 n'est pas encore validée.

Le 09 conserve six appuis distincts, trois de chaque côté, une bouche ventrale
verticale et trois grandes pointes de glace sombre. Une correction de pose
écarte les appuis qui étaient trop masqués. Le joueur refuse ensuite la r02 :
« trop de parties osseuses et pas assez de chaire répugnante (avec couleur
chaire ) ». Ce refus ne valide pas le 08 par déduction.

La nouvelle proposition 09 r03 retire les plaques osseuses des membres, le
cadre de la bouche et les racines d'os des pointes de glace. Une chair couleur
peau maladive, avec plis lourds, boursouflures et creux rouges, domine désormais
le corps et les six membres. Les petites dents et les ongles sombres restent.
Le joueur valide ensuite cette apparence r03 dans l'accord 06. Le PNG natif
exact est conservé dans `accepted/MON-05-r03/` des sources. La r02 refusée et
les sources r01 sont conservées sans modifier leurs pixels.

Les limites de gabarit précédentes sont conservées : 68 px de haut pour le 08,
124 px de haut et de large au maximum pour le 09, sur une case de référence
64 px. Le 09 utilise un canevas de débordement de 256 px, sans définition
de nouvelles collisions. Les palettes de conditionnement sont de 16 et
20 couleurs. Le 09 reste une étude de créature du plan glacé, sans attribution
au Premier Libre ou à un autre personnage nommé de l'histoire.

`depths-ice-09.json` conserve les prompts et étapes initiales ;
`MON-05-flesh-revision-03.json` conserve la correction demandée et son prompt.
`profondeurs-glace-08-09.png` présente les deux à taille native et au zoom ×2,
puis sur sol de référence avec le responsable 01 accepté comme repère humain.
Les seize apparences déjà acceptées et le 08 r02 restent identiques pendant
la correction du 09. Aucun effet de froid, statistique, compétence, animation,
rencontre ou raccord au jeu n'est défini par cette revue.

### Suite 11 — grands boss de secteur, 18 r02 et 19 r01

Le joueur demande des boss de secteur bien plus grands que 64 px, avec des
tailles et silhouettes différentes. Le premier duo propose un Maréchal compact
et massif de 192 px de haut et Mère Suture haute et désarticulée de 288 px.
Ces apparences et dimensions sont ensuite validées par « on valide :) » dans
`visual-acceptance-07.json`, avec sources et PNG exacts figés dans `accepted/`.
Elles ne forment pas une grille commune pour les futurs boss. La règle d'un
seul grand boss par secteur reste confirmée.

Le 18 r02 réutilise exactement la grande source image_gen acceptée de r01,
avec un nouvel export direct de 123 × 192 px. Son PNG natif accepté de 96 px
reste intact et constitue toujours la référence de l'accord 03. Il ne s'agit
ni d'un redessin ni de l'agrandissement de ce petit PNG. Le nouveau gabarit
de 192 px est validé dans l'accord 07 et son PNG exact conservé séparément.

Le 19 r01 propose une silhouette clinique haute, deux bras organiques, deux
jambes de chair et quatre bras chirurgicaux intégrés à un bâti dorsal. Une
tenue verte déchirée et une chair boursouflée le distinguent du Maréchal.
Le dessin r01 occupe 186 × 288 px dans son canevas. Aucun outil chirurgical ne
définit de compétence ou d'effet de combat. Le nom reste provisoire, comme
les autres noms de l'histoire.

`sector-bosses-11.json` conserve la demande, les sources, le prompt exact du
19, les empreintes et les exports. `tools/art/show_sector_bosses_review.py`
fabrique `boss-secteurs-18-19.png` et `boss-secteurs.html` dans le dossier
d'aperçu courant. La planche place les deux à côté d'une tenue du joueur
acceptée dans ACTOR-RES01, à l'échelle native, sans zoom des personnages.
Le joueur de référence mesure 65 px dans ce PNG. Les canevas des boss sont
de 384 et 512 px ; ils ne créent aucune nouvelle taille logique ou collision.

Le joueur trouve ensuite la démone peut-être trop haute. Le 19 r02 est un
nouvel export direct de la même grande source, proposé à 144 × 224 px.
Son apparence reste celle de l'accord 07 ; ce nouveau gabarit est à examiner.
Le PNG de 288 px accepté reste intact. `BOSS-02-scale-revision-02.json`
conserve la demande et les dimensions ; le 18 reste à 192 px, sans retouche.

La demande suivante de diversité est consignée dans `demonic-diversity-12.json` :
prévoir aussi des démons insectoïdes et lovecraftiens, avec des anatomies et
silhouettes variées. Cette direction ne valide aucune nouvelle espèce ni
capacité et ne modifie aucun dessin accepté.

La galerie initiale et ses dix-huit exports restent identiques, avec dix-sept
apparences déjà validées et le 08 encore en revue. Les grands exports acceptés
de l'accord 07 restent suivis séparément. Aucun raccord au jeu, nouvelle
capacité, animation ou rencontre n'est ajouté.

Les numéros 10–12 complètent le répertoire par des rôles visuels proposés.
Leur nom, leurs capacités et leurs chiffres ne sont pas validés. Aucun tir,
soin, renfort, effet de contrôle ou changement de placement n'est intégré.
Les prompts et références sont conservés dans `monster-roles-02.json`.
Les révisions simplifiées et les cultistes sont documentés dans
`readability-cultists-03.json`, puis `CULT-02-pose-02.json` pour la correction
de la pose de tir. `comparaison-lisibilite.png` montre les 10–12 avant/après
à même échelle ; `cultistes-13-15.png` présente les humains et leurs montages
sur sol. Le 15 reçoit ensuite le focaliseur de la revue masquée 05 : ses
sorts, effets et bonus éventuels restent à présenter séparément.
La planche `monstres-10-12.png` les montre en 64 px natifs, zoom ×3 et
montage sur sol de référence. Les neuf dessins précédents sont préservés.

Le 09 est une créature du plan glacé, pas un boss de secteur ni une définition
du Premier Libre ou d'un autre chef nommé. Sa catégorie de rencontre reste à définir.
Les intitulés, rôles précis, statistiques, services, rencontres et capacités
restent à examiner séparément. Aucun de ces candidats n'est accepté par le
simple fait de partager une direction ou un type de créature demandé.

### Suites 14–16 — ombres et réaffectation des candidats 20–21

Le joueur demande une ombre sous les monstres. `contact-shadows-14.json`
conserve les calques séparés des dix-huit variantes courantes de revue :
les quatorze monstres de la galerie initiale, les deux grands boss courants
et les deux nouvelles élites. La forme à trois bandes, les couleurs et les
opacités reprennent `src/game_art.rs::draw_contact_shadow` à l'échelle 64 px ;
la largeur suit les appuis visibles. Aucun PNG de corps antérieur n'est modifié.
Les ombres sont composées sous les créatures dans les aperçus, sans raccord
au rendu du jeu. Leur présence est demandée ; le rendu concret reste en revue.

La première proposition `SECTOR-BOSSES-15` attribuait les dessins 20 et 21
aux chefs du troisième et du quatrième acte. Le joueur corrige cette lecture :
il apprécie les deux dessins, mais les voit comme des élites de taille adaptée.
Les boss doivent avoir l'allure de dirigeants. L'ancienne attribution de rôle
est donc remplacée par `ELITE-MONSTERS-16` ; elle n'est pas validée comme boss.
Les numéros 20 et 21 sont conservés pour la revue, avec les identifiants
graphiques ELITE-03 et ELITE-04 ; aucune identité jouable n'était raccordée.

Le 20 r02 est une masse organique à trois visages fusionnés, avec relais et
appendices charnus. Le 21 r02 est un insectoïde de forge bas et large, avec
six pattes articulées. Leurs dessins sources image_gen restent strictement
identiques. L'appréciation des apparences est enregistrée séparément dans
`visual-acceptance-08.json`, sans validation de leurs anciens grands gabarits.
Les nouveaux exports proviennent directement des grandes sources ; les limites
proposées sont de 72 × 104 px et 64 × 120 px (hauteur × largeur). Ni la taille
ni la nouvelle palette conditionnée ne sont présentées comme déjà validées.
Les noms « Le Chœur » et « Gueule-Fournaise » restent réservés aux chefs à
redessiner ; les élites portent uniquement des libellés descriptifs provisoires.

`elite-monsters-16.json` conserve les sources, empreintes, cibles de gabarit
et liens vers les prompts exacts de `sector-bosses-15.json`.
`tools/art/show_elite_monsters_review.py` produit `elites-20-21.png`,
`elites-20-21.html` et `ombres-monstres.png` dans le dossier d'aperçu courant.
La planche présente les deux élites à taille native, au zoom entier et à côté
du joueur accepté. Les ombres restent séparées. Aucun nom de jeu, capacité,
collision, animation ou rencontre n'est défini par cette réaffectation.
Le 08 et le gabarit de 224 px de Mère Suture restent à valider séparément.

### Suite 17 — petites troupes démoniaques et variantes de palette

Le joueur demande davantage de petites troupes, puis précise qu'il parle
des démons. Les essais 22–24, 26 et 27 sont donc écartés de cette sélection.
Le dessin 25 est apprécié séparément et conservé comme proposition de magicien
possédé ; ses sorts restent à définir. Ces sources ne sont pas supprimées.

La sélection contient les douze démons 28–39, avec deux palettes par
dessin et des corps initialement de 20 à 52 px de haut. Le 28 est ensuite
révisé à 65 px de haut, comme détaillé en suite 18. Les canevas sont de
64 × 64, sauf celui du 28 r02 de 128 × 128. Les planches sont strictement natives,
sans agrandissement ; le joueur accepté sert de repère dans chaque case.
Les ombres de contact restent sur des calques distincts. Les géométries,
masques alpha et contours sombres sont identiques entre les palettes A et B.
Ces variantes cosmétiques ne créent aucune différence de statistiques.

Voir [TROUPES-17.md](TROUPES-17.md) pour les numéros et les tailles réelles,
`petites-troupes-17.json` pour les prompts image_gen, références et empreintes,
et `troupes-17-validation.json` pour les contrôles. Aucun nouvel acteur,
comportement, placement, animation ou effet de combat n'est intégré.

### Suite 18 — tailles variées et galerie avant sélection

Le joueur corrige l'interprétation précédente : tous les démons ne doivent
pas être plus petits que le joueur. Le 28 est réexporté directement de la
même grande source à 67 × 65 px, soit la hauteur du joueur de référence.
Les anciens PNG r01 et le dessin source restent intacts ; la r02 utilise
un canevas de débordement de 128 px sur la même base de 64 px par case.

`demons-varied-18.json` prépare vingt nouveaux dessins 40–59, avec des formes
charnues, des bêtes, des insectoïdes et des organismes des profondeurs. Les
gabarits mêlent petits corps, stature du joueur et créatures plus imposantes.
Une grande taille ne donne pas automatiquement un rôle de chef ou d'élite.
Les variantes de palette conservent les pixels de silhouette et les ombres
restent séparées. Les planches sont natives, sans zoom des acteurs.

Le joueur souhaite une galerie abondante avant de choisir. Les nouveaux
candidats et ceux du lot 17 restent donc en attente de sélection individuelle ;
aucun accord de groupe ni raccord au jeu n'est déduit de cette demande.
Les accords antérieurs et le magicien possédé 25 restent conservés.
Prompts et sources : `demons-varied-18.json`. Fabrication :
`tools/art/show_demon_gallery_review.py`.

### Suite 19 — références Diablo II et Grim Dawn

Le joueur demande explicitement de s'inspirer plutôt de Diablo II, puis de
Grim Dawn. La série 18 est mise de côté : dix sources 40–49 sont conservées,
les dix briefs 50–59 restent non générés. Cette interruption n'est ni un refus
individuel de chaque dessin ni une validation des précédents.

`DEMONS-19` prépare vingt nouveaux candidats 60–79, avec des familles de démons
cornus, bêtes infernales, insectoïdes, formes serpentines et organismes grotesques.
Les références officielles observées sont les sprites Fallen et Goatman de
Blizzard, et les illustrations Chthonian Devourer et Thorned Horror de Crate.
L'interprétation de travail privilégie des proportions adultes, des articulations
prédatrices et des matières plus organiques, sans reprendre leurs personnages.
Les sources sont de nouvelles générations image_gen, sans utiliser les dessins
caricaturaux précédents comme référence d'image. Aucun art de ces jeux n'est
copié dans les ressources du projet.

La taille reste variée et le 28 reste révisé à 65 px de haut. Deux palettes
sont proposées pour chaque nouvelle source, avec géométrie identique et ombres
séparées. Les planches ne redimensionnent pas les acteurs. La sélection et
les refus individuels sont remis à plus tard, comme demandé ; aucun nouveau
dessin, chiffre de combat, sort, rencontre ou acteur jouable n'est intégré.
Prompts et liens : `demons-arpg-19.json`.

### Suite 20 — designs conservés, style du joueur et des armes

Le joueur apprécie les designs de la série 19 mais relève que leur rendu ne
correspond pas au jeu actuel. Son instruction distingue explicitement le
design à garder du style de dessin à reprendre. Les quinze sources r01
produites (60–74) sont conservées intactes et utilisées comme cibles de design.
La source acceptée du joueur armé `BD-C01-comic-master.png` sert uniquement de
référence graphique : contours sombres, aplats, deux ou trois valeurs par
matière, une ombre principale et peu de petits détails. Son équipement et
sa silhouette humaine ne sont pas repris par les démons.

`DEMONS-20` contient vingt sources r02, numéros 60–79 : quinze reprises des
designs précédents et cinq nouveaux modèles dessinés directement dans ce
style. La stature reste variée et les palettes A/B conservent une géométrie
identique. Les ombres sont séparées ; le joueur est montré à la même échelle
sur les cinq planches natives. Le 28 reste à sa hauteur corrigée de 65 px.

L'appréciation des designs ne vaut pas validation de chaque nouvelle
apparence, palette ou gabarit. La sélection individuelle reste différée,
comme demandé. Aucun acteur, chiffre, compétence ou effet n'est intégré.
Voir [DEMONS-20.md](DEMONS-20.md), `demons-style-20.json` pour les prompts
exacts et `demons-style-20-validation.json` pour les contrôles.

### Suite 21 — ajout à la liste et poursuite des candidats

Le joueur demande d'ajouter les vingt candidats 60–79 à la liste, puis de
continuer la génération. Il précise de nouveau que le regroupement et les
validations ou refus individuels auront lieu plus tard. Les fichiers du lot
20 restent intacts ; cet ajout n'est pas enregistré comme une acceptation.

`DEMONS-21` ajoute douze candidats 80–91, avec des gabarits variés, des démons
de chair, des bêtes, de la vermine et des formes insectoïdes. Le rendu reprend
la même source acceptée du joueur armé, uniquement pour le trait et les aplats.
Chaque modèle dispose de deux palettes de géométrie identique et d'une ombre
séparée. Les trois planches sont natives, sans zoom, avec le joueur à côté.
Le concept initial du 89 n'a pas produit d'image exploitable : il est remplacé
avant présentation par l'Arpenteur des failles ; les tentatives et le brief
final sont conservés dans le manifeste.

Voir [CANDIDATS-MONSTRES.md](CANDIDATS-MONSTRES.md) et
`monster-candidate-list.json` pour la liste, `demons-21.json` pour les sources
et prompts, `demons-21-validation.json` pour les contrôles. Les revues plus
anciennes restent séparées en attente du regroupement général. Aucun nouvel
acteur, chiffre, pouvoir, placement ou effet n'est intégré au jeu.

### Suite 22 — série imaginée depuis l'univers

Le joueur demande de conserver les douze derniers candidats 80–91 avec les
autres, puis de créer une série issue directement de notre univers. L'inscription
des précédents reste distincte de leur validation individuelle.

`UNIVERS-22` propose douze modèles 92–103 depuis les motifs de la campagne
actuelle : asservissement brisé, cliniques de conversion, corps relais,
production détournée, rituels et prison du Gel Noir. Les origines et associations
aux secteurs sont des pistes à examiner, sans canonisation. Aucun jeu extérieur
n'est utilisé comme référence de design dans ces nouveaux prompts. Le trait
reprend le joueur et ses armes, avec les mêmes contraintes de lisibilité.

Voir [MONSTRES-UNIVERS-22.md](MONSTRES-UNIVERS-22.md), le manifeste
`universe-monsters-22.json`, la [liste des candidats](CANDIDATS-MONSTRES.md)
et `universe-22-validation.json`. Les dessins et palettes précédents restent
intacts. Les trois planches sont natives avec ombres séparées ; aucun nouvel
acteur, effet mécanique, texte joueur ou rencontre n'est raccordé au jeu.

### Suite 23 — trois refus et poursuite des propositions

Le joueur demande de supprimer directement les trois modèles montrés :
94 — Greffé de triage, 95 — Brancard rampant et 97 — Œil décroché. Les dessins
et leurs variantes, ombres et copies de génération sont supprimés ; les anciennes
planches sont remplacées par des planches ne contenant que les neuf conservés.
La trace de décision reste dans `removed-monsters-23.json`, sans archive graphique.
Les neuf autres restent candidats, sans acceptation individuelle implicite.

`UNIVERS-23` poursuit la création avec dix dessins disponibles parmi les briefs
104–115 : corps asymétriques, cocons, entrelacs, membranes, formes non humanoïdes
et faune du Gel Noir. Les 110 et 114 n'ont pas produit d'image après deux
tentatives bloquées par le générateur ; ils restent hors de la liste et ne
sont pas marqués comme refusés par le joueur. Les numéros sont conservés.

Voir [MONSTRES-UNIVERS-23.md](MONSTRES-UNIVERS-23.md), le manifeste
`universe-monsters-23.json` et `universe-23-validation.json`. La liste courante
écarte les trois refus et ne contient que les candidats avec dessins disponibles.
Aucun nouvel acteur, effet ou comportement n'est intégré.

## Fabrication et contrôle

Les dessins sont créés avec le tool **image_gen** intégré, en utilisant les
sources BD frontales déjà acceptées comme références de style. Chaque sujet
possède sa propre génération. Les prompts et les chemins sont dans
`review.json`. Le 01 a reçu une correction de proportions ; le 09 une correction
de cadrage. Les essais précédents sont conservés en archive, sans validation.

Les sources sélectionnées sont conservées dans
`assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/`.
Le script `tools/art/show_npc_monsters_review.py` conditionne les propositions
dans `artifacts/npc-monsters-review-01/`, sans écrire les atlas du jeu.
Il supprime les marges transparentes, exporte directement chaque résolution
depuis le grand dessin, applique un alpha solide binaire et une palette de
24 couleurs maximum sans dithering, ou 16 pour la revue de lisibilité 03.
Il ne peint ni ne corrige l'anatomie.

La première fabrication comprenait 18 exports de revue en 32 et 64 px.
Depuis l'abandon du 32 px, la galerie courante contient dix-huit exports 64 px,
issus directement des grands dessins. Les humains mesurent environ
60 px de haut en 64 ; les gabarits du coureur, de l'élite et du boss restent
différents. Les canevas et points de contact sont explicités dans `exports.json`.
Ces gabarits sont des propositions graphiques et ne changent aucune collision.

La planche PNG montre chaque candidat à l'échelle du fichier et au zoom ×2
nearest. L'aperçu HTML autonome propose le 64 px seul, le zoom entier ×2,
les filtres PNJ/monstres et l'ouverture du dessin source. La planche a été
inspectée. L'automatisation du navigateur refuse l'ouverture d'un fichier
local par politique de protocole ; aucune autre méthode d'accès n'a été tentée.
Le JavaScript peut être contrôlé syntaxiquement sans ouverture de navigateur.

## Armes et suite de fabrication

Les premières propositions 02–04 r01 sont des **bases mains libres** ; la revue
07 propose ensuite les r02 avec accessoires de métier. Le responsable 01 dispose depuis
le 17 octobre d'une **étude de pose armée**. Le joueur refuse explicitement
r02, trop proche du joueur. Les sources r01 et r02 sont conservées comme
essais antérieurs, sans acceptation. La nouvelle proposition r03 conserve
l'homme casqué et armé, mais supprime le plastron, les grosses épaulières et
le manteau long. Veste courte olive, baudrier ivoire, casque à visière saillante,
moustache visible, main à la ceinture et pistolet au holster donnent une
silhouette de commandement pénitentiaire plutôt qu'une tenue de combattant.
Le joueur valide ensuite explicitement cette apparence r03, le 17 octobre.
Le montage courant `NPC-01-revue.png` montre la taille native 64 px, des zooms entiers,
le r02 refusé et trois sols ; il ne s'agit pas d'une capture en jeu.
Les huit autres sources sont inchangées. Les cinq monstres ont reçu un avis
positif ; cet avis ne définit pas leurs règles ou une intégration au jeu.

Les nouvelles couches de bras, doigts, prises et armes ne sont pas encore
conditionnées séparément. Après sélection des silhouettes, celles-ci devront
reprendre les règles acceptées : corps et équipement stables, arme active
dans la bonne main, deux prises pour un tir à deux mains et débordement
distinct de la taille du porteur. Une silhouette acceptée n'approuve pas
automatiquement ces futures couches.

Les pinces des hybrides sont des parties intégrées à leur anatomie proposée,
pas des armes équipées. Les corps morts, épaves, animations, sons et effets
ne sont pas définis par cette première série. Leur revue restera distincte.

Références : `AGENTS.md`, `docs/DIRECTION_ARTISTIQUE_TILES.md`,
`docs/HISTOIRE_CAMPAGNE.md`. Le bestiaire antérieur sert uniquement à identifier
les contenus existants ; ses peuples, noms et habitats ne deviennent pas
le contenu de cette nouvelle campagne.


## Suite 24 — catalogue global et nouvelles familles

La liste précédente suivait seulement les lots récents. À la demande du joueur,
le catalogue réunit maintenant 102 modèles avec nom, code et image native, dont
les anciens cultistes 13–17 et 22–24. Les seize propositions 116–131 ajoutent
cultistes possédés, zombies, esprits torturés, squelettes et créatures enflammées.
Voir `CANDIDATS-MONSTRES.md`, `CULTISTES-MORTS-VIVANTS-24.md` et
`catalogue-layout-24.json`. La vue défilante conserve 920 px de largeur ; aucun
acteur n'est agrandi ou réduit lors de la composition. Les treize apparences
déjà validées dans ce regroupement gardent leur statut. Les autres validations,
les gabarits révisés, les noms et les règles restent distincts. Les refus
94/95/97 restent supprimés ; aucun raccord au runtime n'est effectué.


## Suite 25 — nécromanciens

Le lot 116–131 est explicitement conservé comme candidats. Huit nécromanciens
132–139 rejoignent la revue, quatre scientifiques du culte et quatre démoniaques.
Les 132–134 sont repris avec une lecture clinique plus claire ; le 132 r01 est
refusé pour sa seringue et sa tenue. Le 139 r02 corrige le visage et le livre
à la demande du joueur. Voir `NECROMANCIENS-25.md`, `necromancers-25.json` et
`catalogue-layout-25.json`. Le catalogue courant contient 110 modèles.
Les révisions antérieures sont séparées, les nouvelles restent à sélectionner.


## Suite 26 — artefact du 137

Le joueur confirme les sept autres nécromanciens pour la liste et demande un
artefact sordide à la place du livre du 137. La r02 présente un reliquaire de
chair desséchée sous métal noirci et crochets d'os, au même gabarit 44 × 72 px.
Source r01 conservée ; r02 à examiner dans `NEC-D02-ARTEFACT-26.md`. Le catalogue
garde 110 modèles et les nouvelles mécaniques restent hors périmètre.


## Suite 27 — grand bestiaire démoniaque

À la demande du joueur, vingt-quatre candidats 140–163 ajoutent des vers et
larves, des démons ailés musclés, des minotaures ailés, des démons de peste,
du froid et des formes humanoïdes. Quatre modèles par famille, deux palettes
par dessin, gabarits variés et ailes pouvant dépasser 64 px. Voir
`BESTIAIRE-DEMONIAQUE-27.md`, `demon-bestiary-27.json` et
`catalogue-layout-27.json`. La grande planche du lot et les scènes de contrôle
conservent les corps à taille native, avec ombres séparées. Le catalogue réunit
134 modèles ; les nouvelles apparences et règles restent à examiner séparément.


## Suite 28 — conservation du lot démoniaque

Le joueur demande explicitement d’ajouter les 140–163 au catalogue. Leur
conservation est enregistrée dans `catalogue-retention-28.json`, sans modifier
les treize apparences déjà validées ni déduire une sélection finale ou des
règles. Le catalogue reste à 134 modèles graphiques ; les palettes ne sont
pas comptées comme des modèles supplémentaires.
