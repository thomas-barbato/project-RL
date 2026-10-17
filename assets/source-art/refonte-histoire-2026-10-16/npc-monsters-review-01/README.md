# Sources proposées — NPC-MONSTERS-01

Première série de dix-huit dessins. **Les apparences 01–07 et 09–18 sont validées ;
seul le 08 reste en revue. Aucun n'est encore raccordé au rendu du jeu.**
Les numéros, prompts, rôles provisoires et empreintes sont conservés dans
`docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01/`.

Le sous-dossier `archive/` contient le premier essai de proportions de NPC-01
et le premier cadrage de MON-05. Le responsable courant est NPC-01-r03,
officier en uniforme court, casqué et armé, validé le 17 octobre.
La source r02 est explicitement refusée car elle ressemble trop au joueur ;
elle reste conservée, ainsi que r01. Les sources précédentes gardent leurs pixels.

Les exports et la planche se régénèrent avec
`python tools/art/show_npc_monsters_review.py`, dans `artifacts/`.
Les PNG natifs 32/64 acceptés de NPC-01 sont figés dans
`accepted/NPC-01-r03/`. Le script les réutilise sans les reconditionner,
après vérification des empreintes de la source et des deux PNG.
L'étude armée du responsable ne constitue pas encore une séparation en
couches de corps, bras, arme et doigts prête à intégrer.

Le joueur abandonne ensuite le mode de jeu 32 px. Seuls les exports 64 px
sont désormais entretenus. Le PNG 32 px accepté précédemment reste une
archive figée de l'accord, sans nouvelle fabrication ni choix de rendu.
MON-06 à MON-08 proposent les rôles de distance, soutien et contrôle ;
leurs trois sources, prompts et dimensions sont conservés pour revue.

La revue 03 propose MON-06 à MON-08 en r02, moins détaillés. Leurs r01
restent conservés. CULT-01 à CULT-03 sont des humains ordinaires du culte,
destinés aux premières rencontres avant les possédés et les hybrides.
CULT-02 r02 détache le pistolet du corps pour améliorer sa lecture.
Ces six dessins sont conditionnés avec une palette de 16 couleurs ; leur
validation ultérieure est précisée ci-dessous. Leurs armes tenues ne sont pas encore séparées
en couches pour le jeu.

Versions courantes après les corrections du joueur : CULT-01 r03,
CULT-02 r04 et CULT-03 r03. Les trois portent un uniforme commun sombre,
une capuche et un masque ivoire opaque. Les visages et essais de capuche
noire antérieurs sont remplacés, mais leurs sources restent conservées.
CULT-03 propose un lanceur de sorts humain avec focaliseur violet ; les
sorts et leurs effets ne sont pas encore définis.

Les six apparences 10–15 sont ensuite validées et leurs PNG natifs 64 px
figés dans `accepted/`, avec sources et empreintes dans la deuxième
acceptation visuelle de la revue. Le script ne les reconditionne plus.

ELITE-01, ELITE-02 et BOSS-01 r01 proposent les numéros 16–18 : deux humains
du culte et une première étude du Maréchal des Chaînes. Le boss emploie
une case de référence 64 px et un canevas de débordement de 256 px ; sa
taille visuelle ne modifie pas les collisions. Ces trois apparences sont
ensuite validées par « je les aimes bien oui :) » et figées dans `accepted/`.
Leurs sources, empreintes et PNG exacts sont consignés dans
`visual-acceptance-03.json` de la revue. Les noms et les règles restent à examiner.

NPC-02, NPC-03 et NPC-04 r02 sont les révisions proposées dans la revue 07 :
tablier et mécanisme d'arme, mallette médicale, puis clé et avant-bras prothétique.
Les grandes sources r01 non validées sont conservées. Le joueur valide ensuite
les trois apparences r02 par « je valide :) ». Leurs PNG 64 px exacts sont figés
dans `accepted/NPC-02-r02/`, `accepted/NPC-03-r02/` et `accepted/NPC-04-r02/`,
avec sources et empreintes dans `visual-acceptance-04.json` de la revue.
Cet accord ne définit aucun service ou effet nouveau.
Leurs prompts et références sont dans `services-07.json` de la revue ;
`artifacts/npc-monsters-review-01/services-02-04.png` est l'aperçu reproductible.
Leurs accessoires sont encore inclus dans la pose statique, sans couches
séparées de bras, d'objet et de doigts. Aucune ressource du jeu n'est remplacée.

MON-01, MON-02 et MON-03 r02 sont les révisions proposées dans la revue 08 :
ouvrier possédé à grande griffe, quadrupède greffé et élite au bras-presse.
Les sources r01 appréciées restent conservées. Les r02 simplifient les marques
d'usure, greffes et pièces secondaires, avec les mêmes identifiants et gabarits.
Le joueur valide ensuite les trois apparences r02 par « je valide :) » ;
leurs PNG natifs exacts sont figés dans `accepted/MON-01-r02/`,
`accepted/MON-02-r02/` et `accepted/MON-03-r02/`, avec sources et empreintes
dans `visual-acceptance-05.json` de la revue.
Prompts, références et empreintes de fabrication sont dans
`possessed-hybrids-08.json` ; la planche reproductible est
`artifacts/npc-monsters-review-01/possedes-hybrides-05-07.png`.
Les machines sont intégrées à l'anatomie ; aucune arme équipée, capacité,
collision, animation ou apparition n'est ajoutée. Les autres apparences
déjà acceptées restent inchangées.

MON-04 r02 et MON-05 r03 sont les propositions de la revue 09. Le premier
conserve sa tête fendue et ses griffes osseuses. Pour le second, le joueur
refuse la r02, trop osseuse, et demande davantage de chair répugnante couleur
peau. La r03 remplace les plaques d'os par des membres charnus, des plis et
des boursouflures, en conservant six appuis et trois pointes de glace.
Le 08 r02 reste inchangé, sans acceptation déduite de cette correction.
La r02 refusée et l'étape `archive/MON-05-r02-supports-before.png` restent
conservées. Le joueur valide ensuite le 09 r03 par « oui je valide » : son PNG
exact est figé dans `accepted/MON-05-r03/`, avec sa source et ses empreintes
dans `visual-acceptance-06.json`. Le 08 r02 reste à valider.
Le 09 est une créature du plan glacé, pas un boss de secteur ; sa catégorie
de rencontre reste à définir. Voir `encounter-direction-10.json` de la revue.
Prompts, références et empreintes : `depths-ice-09.json`, puis
`MON-05-flesh-revision-03.json` dans le dossier de revue. La planche courante
est `artifacts/npc-monsters-review-01/profondeurs-glace-08-09.png`.

La revue des grands boss `sector-bosses-11.json` ajoute la source BOSS-02-r01
du candidat 19, Mère Suture, et le grand export 18 r02 issu de la même source
BOSS-01-r01 acceptée. Les deux sont validés à 288 et 192 px de haut dans
`visual-acceptance-07.json`, puis figés dans `accepted/BOSS-02-r01/` et
`accepted/BOSS-01-r02/`. Ils se consultent dans
`artifacts/npc-monsters-review-01/boss-secteurs-18-19.png` ou `boss-secteurs.html`.
La planche utilise une tenue native du joueur acceptée comme repère ; aucun
nouveau service, pouvoir, placement, animation ou collision n'est défini.

Le joueur trouve ensuite le 19 peut-être trop haut. Le 19 r02 propose un
export direct de 224 px de haut depuis la même source, sans modifier le
dessin ou le PNG de 288 px accepté. Le 18 reste inchangé à 192 px. Cette
révision de taille reste à valider ; voir `BOSS-02-scale-revision-02.json`.
La direction de diversité des futurs démons, notamment insectoïdes et
lovecraftiens, est consignée dans `demonic-diversity-12.json` de la revue.

Les sources `BOSS-03-r01.png` et `BOSS-04-r01.png` ont d'abord servi aux
candidats 20–21 de `sector-bosses-15.json`. Le joueur apprécie leurs dessins,
mais corrige leur rôle : ce sont des propositions d'élites, avec un gabarit
réduit, et les chefs de secteur doivent évoquer des dirigeants. Les anciens
noms de fichier documentent cette première attribution ; ils ne définissent
plus un rôle de boss. Les dessins ne changent pas. L'accord sur les sources
est dans `visual-acceptance-08.json`, les nouveaux exports et leurs gabarits
à examiner dans `elite-monsters-16.json` (ELITE-03 et ELITE-04, numéros 20–21).
Les noms des chefs Le Chœur et Gueule-Fournaise ne sont pas réaffectés aux élites.

`contact-shadows-14/` contient les ombres de contact séparées, générées à partir
des appuis avec la forme à bandes du rendu existant. Elles se composent sous
les corps et ne modifient aucun dessin. Le manifeste `contact-shadows-14.json`
de la revue décrit les variantes courantes ; les deux anciens calques de grand
gabarit BOSS-03 et BOSS-04 restent traçables dans la proposition historique 15.
Aucun nouvel acteur ou effet n'est raccordé au jeu à cette étape.

La série `TROUPES-17` ajoute douze petits démons ordinaires (28–39), avec
une variante de palette par dessin et des planches uniquement à taille native.
Les sources sont `TROUP-07-r01.png` à `TROUP-18-r01.png` ; leurs dimensions
initiales vont de 20 à 52 px de haut dans des canevas 64 × 64. Le 28 est ensuite
réexporté à 65 px de haut, depuis la même source, sur un canevas 128 × 128.
Les ombres sont
séparées dans `troupes-shadows-17/`. Les variantes B conservent la géométrie
et l'alpha de A ; les cartes de couleur sont dans `petites-troupes-17.json`.
Ces nouveaux dessins, palettes et gabarits restent des propositions.

Les essais `TROUP-01` à `TROUP-06` sont antérieurs à la précision « je parlais
de démons ». Les cultistes, l'hybride humain et le drone ne font pas partie
du lot démoniaque demandé. Le joueur apprécie cependant `TROUP-04-r01.png`
et propose de l'utiliser comme magicien possédé : cette source reste intacte,
avec direction dans `magicien-possede-25-direction.json` de la revue.
Le rôle ne définit aucun sort ou effet. Aucun ancien dessin accepté n'est retouché.

La galerie `DEMONS-18` prépare vingt briefs, dont les dix sources
`DEMON-01-r01.png` à `DEMON-10-r01.png` sont produites (numéros 40–49),
avec des gabarits variés et une seconde
palette sur la même géométrie. Les ombres sont dans `demons-shadows-18/`.
Le 28 réutilise sa source `TROUP-07-r01.png`, sans retouche : la révision r02
le porte à la hauteur native du joueur, 65 px, avec un canevas de débordement.
Les anciens exports r01 restent conservés. Les choix individuels seront faits
plus tard par le joueur ; aucun nouveau candidat n'est intégré ou accepté
automatiquement. Voir `demons-varied-18.json` de la revue.

Le recadrage suivant prend Diablo II et Grim Dawn comme références directes.
La revue 18 reste un essai antérieur : seules les sources DEMON-01 à DEMON-10
sont produites, les DEMON-11 à DEMON-20 restent des briefs non générés.
La nouvelle revue `DEMONS-19` produit quinze sources originales
`ARPG-01-r01.png` à `ARPG-15-r01.png` (60–74), et leurs ombres séparées dans
`demons-shadows-19/`. Les proportions sont adultes et les matières organiques
plus sombres ; gabarits, palettes et noms restent des propositions à sélectionner.
Les références officielles, prompts et empreintes sont dans `demons-arpg-19.json`.

Le joueur apprécie ces designs, puis précise que leur dessin doit conserver
le style du joueur et des armes actuels. Les quinze r01 restent donc intactes
comme références de design. `DEMONS-20` produit les vingt sources
`ARPG-01-r02.png` à `ARPG-20-r02.png` (60–79) : quinze reprises graphiques
des designs précédents et cinq nouveaux candidats directement dans le bon
style. Les contours sombres, aplats et ombres simples suivent la source
acceptée `BD-C01-comic-master.png`, utilisée uniquement pour son trait.
Les ombres sont séparées dans `demons-shadows-20/`. Les palettes, tailles et
apparences finales attendent la sélection individuelle ; aucun nouveau
personnage n'est raccordé au jeu. Prompts exacts et empreintes :
`demons-style-20.json` dans le dossier de revue.

Le joueur demande ensuite d'ajouter les vingt candidats à la liste et de
poursuivre avant une sélection ultérieure. `DEMONS-21` ajoute douze sources
`ARPG-21-r01.png` à `ARPG-32-r01.png` (80–91), dans le même trait que le joueur
et ses armes, avec des tailles variées. Leurs ombres séparées sont dans
`demons-shadows-21/`. Les vingt r02 précédentes et leurs palettes restent
intactes. Les prompts exacts, références et empreintes figurent dans
`demons-21.json` ; la liste est `monster-candidate-list.json`, dans le dossier
de revue. L'inscription des candidats ne vaut pas validation individuelle
ni intégration dans les ressources d'exécution.

`UNIVERS-22` ajoute les sources originales `RLU-01-r01.png` à
`RLU-12-r01.png` (92–103). Elles sont imaginées directement depuis les motifs
de la campagne : carcans brisés, conversions, surveillance détournée, forges,
rituels et prison glacée. La référence graphique reste le joueur et ses armes.
Les grandes sources, variantes précédentes et ressources du jeu sont conservées.
Les nouvelles ombres sont dans `demons-shadows-22/`. Les prompts exacts,
empreintes et intentions sont dans `universe-monsters-22.json` dans la revue.
Les nouvelles origines suggérées et les apparences restent des propositions.

Sur demande explicite du joueur, les sources `RLU-03-r01.png`, `RLU-04-r01.png`
et `RLU-06-r01.png` (94, 95 et 97), leurs variantes, ombres et copies de
génération sont supprimées. Il ne reste pas d'archive graphique de ces trois
modèles dans la banque. Le manifeste conserve les numéros et la trace du refus
dans `removed-monsters-23.json`. Les neuf autres sources du lot 22 restent intactes.

`UNIVERS-23` ajoute dix sources : `RLU-13` à `RLU-18`, `RLU-20` à `RLU-22`
et `RLU-24`, toutes en r01. Les 110 / `RLU-19` et 114 / `RLU-23` restent des
briefs sans image après blocage du générateur ; ils ne sont pas des candidats
dessinés ni des refus du joueur. Les ombres disponibles sont dans
`demons-shadows-23/`. Prompts exacts, références et empreintes :
`universe-monsters-23.json` dans la revue.


`CULTES-24` ajoute seize sources originales en r01 : `CULT-04` à `CULT-07`,
`ZOM-01` à `ZOM-04`, `ESP-01` à `ESP-04`, `SQUE-01`, `SQUE-02`, `BRA-01`
et `BRA-02` (116–131). Les ombres sont dans `demons-shadows-24/` ; les prompts
et empreintes sont conservés dans `cultists-zombies-spirits-24.json`. Ces
nouvelles apparences restent à sélectionner. Le catalogue global reprend les
exports natifs antérieurs sans repeindre ni redimensionner les corps, et
restaure la visibilité des anciens cultistes. Aucun atlas du jeu n'est modifié.


`NECROS-25` ajoute les sources courantes `NEC-C01-r02` à `NEC-C03-r02`,
`NEC-C04-r01`, `NEC-D01-r01` à `NEC-D03-r01` et `NEC-D04-r02` (132–139).
Les r01 des C01–C03 et D04 restent des essais précédents hors catalogue courant.
Les prompts, corrections et empreintes sont dans `necromancers-25.json` ;
les ombres de contact sont dans `demons-shadows-25/`. Ces nouvelles apparences
restent proposées. Le masque et le livre du D04 ont été repris sur demande ;
la blouse et la seringue du C01 ont été corrigées sans modifier ses règles.


Le 137 passe en `NEC-D02-r02.png` : un reliquaire sordide remplace le livre,
sur demande du joueur. La r01 reste conservée hors catalogue courant. Source,
prompt et exports sont enregistrés dans `NEC-D02-artifact-26.json`. Les sept
autres nécromanciens sont confirmés pour leur inscription comme candidats.


`DEMONS-27` ajoute vingt-quatre sources originales en r01 : `DEM-V01–04`
(vers/larves), `DEM-A01–04` (ailés), `DEM-M01–04` (minotaures ailés),
`DEM-P01–04` (peste), `DEM-F01–04` (froid), `DEM-H01–04` (humanoïdes),
numéros 140–163. Les grandes sources viennent de l'outil intégré `image_gen`.
Prompts, références et empreintes : `demon-bestiary-27.json` dans la revue.
Ombres séparées : `demons-shadows-27/`. Ces apparences sont proposées ; aucune
ressource de runtime ou validation antérieure n'est remplacée par cet ajout.
