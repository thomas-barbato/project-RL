# Définition du contenu visuel de Project RL

Version 1 — 5 octobre 2026.

### Référence de finition CIVILS-U01-08 — accord du 19 octobre 2026

Le joueur valide les cinq dessins de corps civils, leurs dix-huit placements
(dont cinq en intérieur) et les dix-neuf décors de sang. L'accord et les pixels
exacts sont dans `reviews/refonte-histoire-2026-10-16/street-civilians-review-08/`.
Il juge cette finition supérieure à celle des personnages, armures, armes et
monstres actuels et demande un audit avant toute refonte de ces catégories.

Règles à conserver pour les prochains lots, avec comparaison à taille native :

- La case reste de 64 px et la projection demeure celle du camp accepté.
  Définir l'emprise de l'objet avant fabrication. Les corps de ce lot utilisent
  une toile de 80 × 48 px avec marges ; leur silhouette garde l'échelle humaine.
- Dessiner chaque objet entier sur un vrai fond transparent. Aucune coupe
  d'illustration de carte, aucun sol ou voisin collé au contour. Conserver les
  sources et conditionner le dessin complet sans étirement.
- Travailler une finition de jeu lisible : volumes nets, ombres par masses,
  matières mates et usées, plis et ruptures placés selon l'objet. Le détail doit
  rester utile à la taille de jeu ; un grand original ne suffit pas à le prouver.
- Conserver les contours sombres lisibles sans transformer tous les petits
  volumes en bordures épaisses uniformes. Préserver les transitions de bord
  des exports natifs ; ne pas obtenir le rendu 64 px en agrandissant un ancien
  petit sprite.
- Varier sobrement les vêtements et leurs teintes pour distinguer les corps.
  Le sang utilise des rouges sombres, bordeaux et bruns, avec quelques accents
  plus frais. Garder le sol et les silhouettes lisibles dans l'éclairage réel.
- Fabriquer les flaques et traînées comme décors séparés. Les blessures propres
  au corps peuvent faire partie de son dessin. Une mutilation dessinée et voulue
  ne justifie pas une silhouette coupée accidentellement lors de l'export.
- Composer des scènes liées aux lieux et aux dégâts, avec poses et orientations
  variées, sans quadrillage de copies ni placements systématiques. Préserver les
  seuils et passages ; la boîte d'image ne définit pas une collision.
- Placer les corps et le sang sous les acteurs. Un décor intérieur rattaché à
  son toit ne se dessine que lorsque la pièce est dévoilée. Les sources de
  lumière et l'usure commune restent celles du moteur.
- Revoir les ressources numérotées et leur composition avec les acteurs à taille
  réelle, sur la carte éclairée. L'accord sur cette finition ne valide pas
  d'avance de nouvelles silhouettes, animations, morts de combat ou règles.

Ces règles décrivent le lot effectivement montré et accepté ; elles ne fixent
ni angle numérique de caméra ni nouvelle palette chiffrée. L'audit demandé doit
séparer défaut de source, changement d'échelle, filtrage et choix de style.

Le conditionnement accepté réduit l'original isolé avec Lanczos, conserve les
transitions d'alpha utiles et place l'objet entier dans sa toile native. Le
filtre de dessin du moteur reste `Nearest`. Cela n'impose ni palette réduite
ni contours crénelés : ne pas confondre ce filtre avec une obligation de pixel
art grossier. Les orientations de placement du lot sont des rotations à angle
droit sans rééchantillonnage.

L'audit ART-QUALITY-01 est conservé dans
`reviews/refonte-histoire-2026-10-16/art-quality-audit-01/AUDIT.md`, avec une
planche du rendu actuel à échelle constante. Il confirme des écarts de finition
et certains chemins historiques encore agrandis. Ses recommandations de lot
pilote ne constituent pas une autorisation de refaire les acteurs existants.

Le joueur demande ensuite d'appliquer ces recommandations et de montrer le
résultat ensemble avant décision. **ART-FINISH-01** prépare donc un lot pilote
complet de treize candidats, avec prises à une/deux mains, palettes et scènes
éclairées. La revue est dans
`reviews/refonte-histoire-2026-10-16/art-finish-review-01/`. Le 19 octobre,
le joueur répond **« Valider le lot complet »** : les treize dessins, les six
configurations de mains, les sept palettes et les deux comparaisons en carte
sont acceptés. `acceptance.json` conserve les empreintes exactes. Les exports
embarqués vont dans `assets/sprites/art-finish-01/` et servent au plateau
comme à l'inspection. Le PNJ du camp et les corps civils restent les références.

Conserver pour la suite les matières mates, l'usure située, les volumes par
masses et les contours plus nuancés montrés dans ce lot, en vérifiant toujours
le résultat à taille native avec l'éclairage réel. Bastion et Cerbère gardent
la même tête et les mêmes jambes entre leurs trois configurations ; l'arme et
les doigts restent des couches séparées. L'accord porte sur les configurations
et palettes effectivement présentées. Les autres poses et coloris gardent leur
dessin existant en attendant leurs propres lots ; il ne s'étend pas aux autres
monstres ni à une nouvelle mort ou animation de combat.

À la demande suivante du joueur, **ART-FINISH-02** poursuit cette finition
sur les quatre autres tenues de classe et quatre armures : FIN-14 à FIN-21,
avec vingt-quatre configurations de mains et quatre comparaisons en carte.
Le 19 octobre, le joueur répond **« Valider le lot complet »** pour ce lot.
L'accord et les empreintes exactes des 56 exports sont conservés dans
`reviews/refonte-histoire-2026-10-16/art-finish-review-02/acceptance.json`.
Ces ressources vont dans `assets/sprites/art-finish-02/` pour le plateau et
l'inspection. Les vingt-quatre configurations acceptées conservent les couches
corps/arme/doigts et les points d'attache montrés ; tête et bas du corps restent
identiques entre les prises. Les autres armes, prises et peintures attendent
leurs propres lots. Les contrôles du rendu normal sont conservés dans cette
revue ; ses planches et références acceptées ne sont pas régénérées.

Le menu de création utilise aussi les cinq dessins de classe acceptés,
FIN-01 et FIN-14 à FIN-17, dans ses vignettes et son grand portrait. La fiche
conserve les armes de départ réelles ; leur prise ne force plus l'ancien
assemblage dans ces portraits de sélection. Les captures des cinq classes
sont conservées dans le complément `creation-menu-checks.json` du lot 02.

**CLASS-START-03**, accepté le 20 octobre, fait de ces cinq modèles l'apparence
de base sans objet d'armure équipé. Les tenues de départ ne sont plus accordées
dans l'inventaire. Une armure réelle remplace ce modèle ; une fois retirée,
le modèle de classe revient pour le plateau et l'inspection. Cette règle
réutilise FIN-01 et FIN-14 à FIN-17 sans nouveau dessin ni changement d'échelle.
L'accord et la compatibilité des anciens objets sont conservés dans
`reviews/refonte-histoire-2026-10-16/class-start-03/`.

**ART-FINISH-03** poursuit la finition sur quatre équipements existants :
Tablier de cuir, Tenue de membranes, Combinaison Peau-Seconde et Tenue du
technicien, sous les codes FIN-22 à FIN-25. Le joueur répond ensuite
**« Valider le lot complet »** pour ces quatre dessins, leurs douze configurations
et les deux comparaisons éclairées. L'accord et les empreintes des 28 exports
sont conservés dans `reviews/refonte-histoire-2026-10-16/art-finish-review-03/`.
Les images embarquées vont dans `assets/sprites/art-finish-03/` pour le plateau
et l'inspection, avec les mêmes couches et points d'attache. Les planches et
références acceptées restent figées. Cet accord porte sur les prises et
palettes présentées ; il ne valide pas les autres ressources par extension.

**ART-FINISH-04** poursuit avec la Robe de la Seconde Ombre et la Tenue du
nomade, codes FIN-26 et FIN-27. Le 19 octobre, le joueur répond
**« Valider le lot complet »** pour les deux dessins, leurs six configurations
et les deux comparaisons éclairées. L'accord et les empreintes des 14 exports
sont conservés dans `reviews/refonte-histoire-2026-10-16/art-finish-review-04/`.
Les images embarquées vont dans `assets/sprites/art-finish-04/` pour le plateau
et l'inspection, avec les identités, couleurs, échelles et points d'attache
présentés. Les planches et références acceptées restent figées ; les autres
prises et palettes conservent leurs ressources précédentes.

Suite aux quatre lots intégrés, **ART-FINISH-05** prépare quatre ennemis existants
sous les codes FIN-28 à FIN-31 : Cultiste 014 A, Cultiste 016 A, Possédé 006 A
et Zombie 120 A. Le 19 octobre, le joueur répond **« Valider le lot complet »**
pour ces quatre dessins, avec trois fonds et deux scènes éclairées. L'accord,
les empreintes exactes et les références figées sont dans
`reviews/refonte-histoire-2026-10-16/art-finish-review-05/`. Les quatre images
embarquées vont dans `assets/sprites/art-finish-05/` pour le plateau et
l'inspection. Leurs identités, règles et répartition sont conservées. Les
autres variantes de couleur gardent leurs dessins précédents ; les boss
de secteur restent un chantier distinct.

### Orientation globale demandée le 19 octobre 2026 — grimdark et glauque

Le 19 octobre, après FIN-28 à FIN-31, le joueur demande « je te laisse tous les
finires » puis « et les valider ». **ART-FINISH-COMPLETE-06** conserve cette
délégation : Codex peut terminer, contrôler et accepter visuellement les
ressources restantes du chantier dans le style déjà accepté, sans demander
un nouvel accord pour chaque lot. Cela n'autorise ni nouvelle règle de jeu,
ni nouveaux noms, ni boss de secteur. Les contrôles natifs, prises, matériaux,
références et preuves d'intégration restent requis. Voir
`reviews/refonte-histoire-2026-10-16/art-finish-completion-06/delegation.json`.

Le joueur confirme cette intention pour l'ensemble du jeu. La première ville
doit montrer un monde déjà brutal, dévasté par l'assaut et l'invasion : défenses
militaires brisées, morts, sang, quarantaine abandonnée et détournement inquiétant
des machines. La corruption organique et les horreurs du culte prennent davantage
de place dans les secteurs suivants. Cette progression garde la diversité des
démons et la distinction entre humains, possédés et hybrides.

La direction repose sur les formes, les usages et les traces de violence autant
que sur la pénombre. Garder des matériaux froids et salis, quelques lumières
réelles localisées et une circulation lisible. Les couleurs fonctionnelles des
portes et la lisibilité des acteurs restent utiles. Éviter de recouvrir les
passages par des accessoires ou d'utiliser du noir pour cacher les défauts.

L'angle, les contours et l'échelle de STYLE-RL-2026-10-18-r01 restent la référence.
Les éléments sont fabriqués entiers et séparés, avec leur emprise propre. Cette
orientation générale ne valide pas les pixels d'un lot. La revue FIXES-U01-03
succède au premier essai FIXES-U01-02.

Précision du joueur : travailler **les décors dans leur ensemble en premier**,
leur dessin étant jugé trop propre, parfait et reconnaissable comme généré.
Conserver les personnages actuels comme comparaison ; leur révision éventuelle
dépendra du résultat des décors. L'essai STYLE-U01-01/02 propose du béton terne,
de la peinture écaillée et une usure plus irrégulière. Il reste candidat ; il ne
remplace pas la référence approuvée de projection ni les ressources normales.

Le dessin des deux victimes de GRIM-U01-01 est apprécié, mais son ancien placement
est jugé arbitraire. Sa nouvelle disposition près de la défense de la prison
reste à revoir. GRIM-U01-02, la station médicale de contrainte, est **refusée pour
cette rue** et retirée de l'aperçu courant. Le joueur demande des corps de civils,
cultistes et militaires au sol. CORP-U01-01/02/03 répondent à cette demande sous
forme de candidats séparés, entiers, présentés à taille native avant activation.
L'éclairage GRIM-L02 du premier essai n'est pas considéré comme validé par ces
retours ; la nouvelle comparaison garde l'éclairage actuel identique avant/après.

Le joueur demande ensuite de voir **la carte entière avec cette direction avant
de pouvoir valider**. STYLE-MAP-U01-04 étend une finition de surface commune aux
décors existants, avec vue d'ensemble et neuf vues natives dans le moteur. Le
petit patch de sol de l'essai local est retiré de ce mode pour éviter une rupture
de matière. Cette demande ne valide pas encore le style ; les personnages et
la géométrie de production sont conservés. Voir la revue `street-defects-review-04`.

Après consultation de la carte complète, le joueur confirme : **« oui je valide »**.
STYLE-MAP-U01-04 est donc accepté et activé sur la rue étendue, avec les quatre
décors au sol visibles près de la prison. L'accord exact et les empreintes sont
conservés dans `street-defects-review-04/acceptance.json`. Les paragraphes ci-dessus
conservent les étapes de proposition et leurs limites ; les autres variantes
locales et l'ancienne proposition d'ambiance ne sont pas validées par extension.

Référence de travail pour la création, la modification, le conditionnement et
la revue des personnages, équipements et monstres. Lire ce document avant tout
nouvel ajout dans ces catégories. Une conformité technique ne remplace pas
l'acceptation artistique du joueur.

## 1. Statut des décisions

### Référence exacte de style — 18 octobre 2026

Le joueur valide **SCENE-U01-07-r01 / LUM-U01-07-r01** : textures simplifiées,
contours plus affirmés, projection conservée depuis le camp, pénombre et
éclairage des luminaires calculé par le moteur. Il demande de noter ce style
exact, notamment l’angle. Lire [STYLE_VISUEL_REFERENCE.md](STYLE_VISUEL_REFERENCE.md)
et comparer ses images sources avant les prochains décors. Le même document
conserve les réglages validés, l’échelle de 64 px et la demande de cartes
suivantes plus grandes, sans inventer leurs dimensions.

### Vue de dessus des décors — correction du 18 octobre 2026

Précision suivante : la rue SCENE-U01-04/05 conserve un angle refusé par le
joueur malgré la première correction. La caméra doit être comparée directement
au camp accepté : axes du sol parallèles et orthogonaux, cases carrées,
hauteurs apparentes de murs et de mobilier cohérentes avec ce camp. Le terme
« vue de dessus » seul ne suffit pas ; éviter toute dérive en vue aérienne
ou en volumes obliques à points de fuite. Voir
`docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01/U01-PROJECTION-06.md`.
SCENE-U01-06-r01, redessiné depuis le camp, est ensuite accepté dans
`visual-acceptance-r06.json` de cette revue. Le joueur insiste pour rester
au maximum proche de la salle de départ lors des dessins suivants.

Le joueur refuse les bâtiments présentés en élévation frontale. Les nouveaux
décors doivent respecter la vue de dessus sur grille orthogonale du camp
accepté : toitures ou intérieurs vus en coupe dominants, murs de faible hauteur
apparente. La largeur des ouvertures et l'emprise au sol servent de repères
d'échelle ; la hauteur d'une porte sur une élévation ne convient pas à cette vue.
Les véhicules respectent aussi la projection du monde. Cette correction ne
modifie pas les dessins d'acteurs acceptés. Les nouveaux modèles sont soumis
dans `docs/reviews/refonte-histoire-2026-10-16/first-sector-review-01/U01-REVIEW-03.md`.
La demande de correction ne valide pas leurs pixels avant présentation.

### Production progressive des décors — 17 octobre 2026

Le joueur demande de nouveaux décors tout au long du développement des cartes.
La production accompagne donc les plans et modules, à commencer par les sols,
murs, coins, seuils et portes du premier secteur. Chaque lot est montré à
taille native sur la grille de 64 px, avec les acteurs et objets acceptés à
leur échelle réelle, avant intégration des nouveaux dessins ou effets concernés.
Le plan est conservé dans `docs/DECORS_CAMPAGNE.md`. Cette décision de méthode
ne valide pas un dessin, une nouvelle collision, interaction ou zone d’effet.

### Décision courante du 17 octobre 2026 — rendu 64 px uniquement

Le joueur annule explicitement la version du jeu en 32 px. Le seul rendu
proposé dans les options et la seule taille à fabriquer pour les nouvelles
ressources sont désormais **64 px par case**. Les cases logiques, collisions,
proportions relatives, points d'attache, couches et règles de jeu restent
inchangés. Les anciennes préférences de taille sont migrées vers 64 sans
réinitialiser les autres réglages. Les sources, preuves d'accord et ressources
historiques 32 px sont conservées ; certaines restent des données internes
ou des icônes, ce qui ne rétablit pas un mode de jeu 32 px.
Cette décision remplace les obligations de double export ci-dessous, dont
les paragraphes datés conservent la trace historique.
Voir `docs/reviews/refonte-histoire-2026-10-16/native64-only-01/decision.json`.

### Décisions confirmées par le joueur

Le 17 octobre, le joueur juge les derniers monstres potentiellement trop
détaillés et difficiles à lire. La revue suivante privilégie des masses
simples, un signe principal par modèle et une forte réduction des textures
et traits internes, à contrôler en 64 px natifs. Elle ne remplace aucun dessin
accepté sans revue. Le joueur précise aussi que les premières rencontres
doivent être des cultistes humains ordinaires, moins dangereux dans la
progression que les possédés et hybrides. Ceux-ci sont réservés à la suite
de la descente. Aucune valeur de combat n'est déduite de cette hiérarchie.
Trace : `docs/reviews/refonte-histoire-2026-10-16/npc-monsters-review-01/readability-cultists-03.json`.

Le joueur précise ensuite la tenue des cultistes : capuches, uniforme commun
glauque, aucun visage visible et **masques physiques**, qui remplacent
l'essai de simple ouverture noire. Ce sont des humains ; certains peuvent
lancer des sorts puisque la magie existe dans l'univers. Cette permission
ne définit pas leurs compétences, chiffres ou effets. Les candidats concrets
restent alors à valider. Trace : `masked-cultists-05.json` dans la même revue.

Le joueur valide ensuite la présentation courante : les cultistes masqués
13–15 et les versions simplifiées 10–12 sont figés dans
`npc-monsters-review-01/visual-acceptance-02.json` de la refonte d'histoire.
Cette acceptation porte sur les apparences, sans définir les compétences
des monstres, leurs statistiques ou leurs rencontres.

Après leur présentation à taille native, le joueur confirme les deux élites
humaines du culte et le premier boss, candidats 16–18 r01, par
« je les aimes bien oui :) ». Leurs apparences et PNG exacts sont figés dans
`npc-monsters-review-01/visual-acceptance-03.json`. Les noms et capacités restent
à examiner séparément ; cet accord ne raccorde aucun nouvel acteur au jeu.

Le joueur valide ensuite par « je valide :) » les apparences r02 des PNJ de
services 02–04 : armurier, secouriste et technicien. Leurs dessins et PNG 64 px
exacts sont figés dans `npc-monsters-review-01/visual-acceptance-04.json`.
Les anciens essais r01 restent non validés ; les noms, règles de service,
dialogues et futures couches séparées ne sont pas approuvés par cet accord.

Le joueur valide aussi par « je valide :) » les apparences r02 des candidats
05–07 : ouvrier possédé, coureur greffé et Porte-presse. Leurs grandes sources
et PNG natifs exacts sont figés dans `npc-monsters-review-01/visual-acceptance-05.json`.
Les anciennes r01 restent distinctes de cet accord ; les noms, compétences,
statistiques, collisions, animations et rencontres ne sont pas définis par
cette acceptation visuelle.

Pour la créature 09 du plan glacé, le joueur refuse ensuite la r02 :
trop d'os et pas assez de chair répugnante couleur chair. La r03 proposée
privilégie une peau charnue, plissée et boursouflée, avec des teintes chair
naturelles et beaucoup moins d'os visibles. Cette correction est limitée
au 09 ; elle ne change pas le démon 08 ni les apparences acceptées.
Le joueur valide ensuite la r03 par « oui je valide » ; sa source et son PNG
exact sont figés dans `npc-monsters-review-01/visual-acceptance-06.json`.
Il exclut le rôle de boss de secteur pour le 09. Plusieurs autres boss sont
possibles, avec un seul grand boss par secteur ; la catégorie de rencontre
du 09 reste à définir. Voir `encounter-direction-10.json` dans la même revue.
Le 08 reste inchangé et non validé par cet accord.

Le joueur précise ensuite que les **boss de secteur dépassent largement une
case de 64 px**, avec des tailles et silhouettes différentes pour chacun.
Cette direction ne fixe pas une taille unique. La revue `sector-bosses-11.json`
présente d'abord le Maréchal à 192 px de haut et Mère Suture à 288 px ; ces
apparences et dimensions sont validées dans `visual-acceptance-07.json`.
Le joueur trouve ensuite Mère Suture peut-être trop haute : un nouveau
gabarit r02 de 224 px est proposé, sans changer son dessin ni effacer le
PNG de 288 px accepté. Cette révision reste à valider ; voir
`BOSS-02-scale-revision-02.json`. Le Maréchal réutilise sa grande source
acceptée avec un nouvel export direct ; son ancien PNG accepté reste intact.
Les canevas de 384 et 512 px servent aux débordements visuels et ne définissent
aucune collision. Les noms et capacités des boss restent en revue séparée.

Le joueur demande pour la suite une **diversité anatomique des démons**,
avec notamment des insectoïdes et des formes lovecraftiennes. Les futures
silhouettes doivent varier au-delà des humanoïdes corrompus, en conservant
une lecture claire à taille native. Cette direction ne fixe ni espèce,
nom, dessin, compétence ou rencontre précis et ne remplace aucun dessin
accepté. Trace : `npc-monsters-review-01/demonic-diversity-12.json`.

Le joueur précise ensuite que cette diversité insectoïde et lovecraftienne
concerne les monstres ordinaires et les élites. Les **boss de secteur doivent
évoquer des dirigeants, des chefs**, avec une identité et une autorité lisibles.
Une grande taille ou une anatomie inhabituelle ne suffit pas à définir ce rôle.
Les dessins 20 et 21 sont appréciés, mais réaffectés à des propositions d'élites
de taille réduite. Leur ancienne attribution aux chefs « Le Chœur » et
« Gueule-Fournaise » est retirée ; ces personnages restent à dessiner.
Leurs nouveaux gabarits restent à examiner dans `elite-monsters-16.json`.
Cette précision ne remet pas en cause les dessins acceptés des 18 et 19.

Le joueur rappelle qu'une **ombre de contact doit accompagner chaque monstre**.
Elle est préparée sur un calque séparé sous le corps, sans retoucher les PNG
des créatures. La revue `contact-shadows-14.json` montre les variantes courantes
avec les bandes sombres déjà employées par le rendu du jeu, adaptées aux appuis.
La direction est confirmée ; ces compositions restent des aperçus de revue.

Le joueur demande ensuite davantage de **petites troupes démoniaques ordinaires**,
de type « chair à canon ». Il précise qu'il parle des démons : les nouveaux
essais de cultistes, d'hybrides humains et de machine ne constituent pas ce lot.
La revue `TROUPES-17` propose douze démons 28–39, avec de petits gabarits variés
sur des cases de 64 px. Leurs planches sont présentées **uniquement à taille
native, sans zoom**, conformément à cette demande. Les variantes de couleur
sont autorisées dans leur principe : elles conservent exactement la géométrie
et l'alpha du même dessin, sans définir de différence de puissance ou de règles.
Les candidats, palettes et tailles concrets restent à examiner avant intégration.
Le dessin 25 est apprécié séparément comme proposition de magicien possédé ;
ce rôle ne définit pas ses sorts. Voir `petites-troupes-17.json` et
`magicien-possede-25-direction.json` dans la revue de la nouvelle campagne.

Le joueur précise que **varier les tailles ne signifie pas rendre tous les
démons plus petits que le joueur**. Il demande explicitement que le 28 ait
la stature du joueur. Son dessin est conservé et réexporté à 65 px de haut,
sans agrandissement du petit PNG précédent. Les corps peuvent déborder d'une
case de référence de 64 px ; les canevas transparents ne définissent aucune
collision. La galerie `DEMONS-18` ajoute des gabarits petits, de stature humaine
et plus grands, avec des anatomies différentes. Le joueur souhaite accumuler
de nombreux candidats avant de choisir : **les validations et refus individuels
viendront plus tard**. Aucune approbation globale des dessins, palettes ou tailles
n'est déduite de cet encouragement à poursuivre. Les accords plus anciens
restent inchangés. Voir `demons-varied-18.json` dans la même revue.

Le joueur recadre ensuite les inspirations : **Diablo II et Grim Dawn**.
La revue `DEMONS-19` prend leurs bestiaires comme références de familles,
d'anatomies et de matières, avec une interprétation plus sombre et moins
caricaturale pour les nouveaux candidats. Les silhouettes et créatures restent
originales. Les pages officielles et les illustrations effectivement observées
sont consignées dans `demons-arpg-19.json`. Les dix dessins déjà fabriqués de
la revue 18 sont conservés à part ; ses dix autres briefs ne sont pas générés.
Le 28 garde sa correction de stature. Les nouveaux dessins, palettes, gabarits
et noms attendent toujours la sélection individuelle annoncée par le joueur.

Le joueur apprécie les **designs** de cette série mais demande de conserver
le **style graphique actuel du joueur et des armes**. L'inspiration Diablo II
et Grim Dawn concerne donc les anatomies et les familles, sans remplacer le
trait du jeu par un rendu réaliste. `DEMONS-20` reprend les quinze sources
60–74 en contours sombres, aplats et ombres simples, en gardant leurs designs ;
les cinq derniers candidats 75–79 sont dessinés directement dans ce style.
Les sources précédentes restent intactes comme références de design. Les
vingt versions r02, leurs palettes et leurs tailles restent à sélectionner
individuellement. Voir `demons-style-20.json` et `DEMONS-20.md` dans la revue.

Le joueur demande ensuite de **les ajouter à la liste et de continuer**,
avec regroupement et validations ou refus individuels plus tard. L'index
`monster-candidate-list.json` et `CANDIDATS-MONSTRES.md` conservent les lots
récents, sans convertir leur inscription en acceptation. `DEMONS-21` ajoute
douze propositions 80–91 dans le même style ; les lots antérieurs restent
conservés en attente du regroupement général.

Le joueur demande ensuite de conserver les douze derniers et d'imaginer une
série **directement depuis notre univers**. `UNIVERS-22` propose les 92–103
depuis les motifs de l'histoire courante, en gardant le trait du joueur et
des armes. Les nouvelles espèces, origines et associations aux secteurs sont
des propositions, sans canonisation ni intégration. Les validations individuelles
restent différées ; voir `MONSTRES-UNIVERS-22.md` dans la même revue.

Le joueur refuse ensuite explicitement les **94, 95 et 97**, montrés en images,
et demande leur suppression directe. Leurs ressources graphiques sont supprimées,
les neuf autres restent candidats. La trace textuelle est conservée dans
`removed-monsters-23.json`. Ce refus porte sur ces modèles précis et ne crée
pas d'interdiction générale des hybrides. `UNIVERS-23` poursuit avec dix dessins
disponibles parmi les 104–115 ; les 110 et 114 restent des briefs sans image.
Les validations individuelles des autres modèles restent différées.

- Dungeon Crawl Stone Soup est la référence de **taille et de qualité**.
- Étudier sa méthode depuis les sources locales et l'adapter aux personnages,
  armes et monstres de Project RL.
- Rester dans un univers futuriste et industriel. Les armes peuvent être
  brutales, lourdes et imparfaites ; ne pas uniformiser en SF propre et lisse.
- Varier librement les couleurs, même peu réalistes, en excluant le rose depuis
  la demande du 9 octobre 2026. Les variantes cosmétiques d'un modèle restent
  le même objet, avec le même nom et les mêmes statistiques.
- Éviter les tenues monochromes et les deuxièmes couleurs invisibles à petite taille.
- Changer d'arme ne change pas le casque, le torse, les jambes ou la taille du porteur.
- L'armure de torse sélectionne une tenue complète, plutôt qu'une icône de
  plastron collée sur un ancien corps encore visible en dessous.
- Le personnage de base porte un casque. Les autres casques ont des formes
  réellement différentes, pas uniquement une visière sur une même base.
- Une arme rejoint sa main ; une arme de tir à deux mains possède deux prises.
- Les slots sont main gauche, main droite et deux mains. Tous peuvent être
  équipés ; un seul groupe est actif. Les armes visibles suivent ce groupe.
- Une arme peut dépasser du corps sans agrandir ses collisions.
- Un dessin tenu commun par catégorie est accepté : une pose n'a pas besoin
  d'être adaptée à chaque modèle statistique d'une famille.
- Montrer les nouveaux candidats **numérotés avant intégration**.
- Pour toutes les compétences, une cible qui subit des dégâts ou reçoit un
  effet négatif fait un petit mouvement d'aller-retour visuel. Cette réaction
  concerne aussi bien les monstres que le joueur et garde la cible sur sa case.
- Les dégâts renvoyés par Barrière d’épine produisent le sang physique accepté
  sur leur victime vivante, monstre ou joueur. Les machines gardent leur règle
  sans sang. Cette précision est validée sous `ANIM-S35-ACCEPT01`.

### Conventions de fabrication

Les objectifs de palette, conventions de fichiers et coordonnées d'exemple
ci-dessous sont des traductions de cette direction en règles de travail. Ils ne
sont pas présentés comme des paramètres approuvés individuellement par le joueur.
Les dessins de `artifacts/crawl-methode-2026-10-05/` restent des propositions.
Un dessin précédemment refusé ne devient pas une référence de qualité.

### Intégration autorisée le 5 octobre 2026

Le joueur a demandé explicitement d'adapter les armes visibles à toutes les armes
du jeu, puis d'intégrer les créations présentées. La banque active utilise les
17 tenues et les 23 références existantes du bestiaire/NPC, avec des corps fixes
et des couches séparées pour les bras, armes et doigts. Les 113 modèles d'armes
sont associés à 18 dessins de famille, sans modification des IDs ni des règles.
La hache a ensuite été corrigée à sa demande : fer vers l'extérieur.

Les sources, prompts, captures et contrôles de cette intégration sont conservés
dans `artifacts/crawl-integration-2026-10-05/`. L'image de zombie est disponible
en réserve dans l'atlas ; elle n'ajoute pas une espèce active au jeu.

Ce document ne crée ni espèce, classe, objet, statistique, comportement ou règle
de gameplay. Une image de zombie proposée reste une proposition. Conserver les
noms validés ; soumettre les noms manquants à la revue.

### Respiration validée et intégrée le 5 octobre 2026

Le joueur a validé le prototype de respiration présenté dans la galerie des
prises. Le rendu du plateau utilise deux poses distantes d'un pixel natif,
maintenues 1,5 seconde chacune : cycle de trois secondes, sans interpolation.
Le haut du corps, les manches, les armes et les doigts se déplacent ensemble.
Le bas du corps à partir de la ligne native 23 et l'ombre restent fixes.
La jonction à la taille est recouverte par le bas du corps fixe.

Cette respiration concerne le joueur et les six bases humanoïdes compatibles
de NPC. Ne pas l'appliquer automatiquement au bestiaire non humanoïde. Les
portraits, objets au sol et fiches d'inspection restent fixes. Les mouvements
réduits et les instants de déplacement par un effet de combat désactivent la
respiration. Elle ne consomme aucun tour et ne modifie pas les collisions.

Le 16 octobre, le joueur demande son extension à **toutes les armures**.
Les 19 tenues frontales et leurs trois palettes reprennent ce même cycle,
avec une amplitude équivalente entre résolutions : un pixel en 32 px, deux
pixels en 64 px. Le haut du corps et les couches portées suivent le mouvement ;
les pieds et le bas de la tenue restent fixes. Les parties d'arme dépassant
sous la taille continuent à bouger avec les mains. Le rendu statique accepté
reste la première pose exacte. Les portraits, mouvements réduits et réactions
de combat conservent leur désactivation existante. Cette extension ne crée
aucune règle de déplacement ni animation de marche.

### Glissement entre cases accepté le 16 octobre 2026

Le joueur valide `ACTOR-GLIDE-01` : les déplacements ordinaires d'une case
glissent pendant 80 ms, avec un départ immédiat et une arrivée adoucie.
Le dessin complet se déplace ensemble : corps, tenue, armes, ombre et effets
attachés, y compris les marqueurs au-dessus de la tête. Cette présentation
commune concerne le joueur, les monstres, les drones et les transformations.
La caméra suit le même glissement et la projection du pointeur reste alignée.

Une nouvelle commande n'attend jamais la fin du glissement. Un pas rapproché
repart de la position affichée ; il ne crée pas une file d'animations retardées.
Les positions logiques, collisions, tours, portées et perceptions s'appliquent
immédiatement comme avant. Les déplacements de compétence conservent leur
présentation propre. Aucun mouvement de marche, rebond ou particule n'est ajouté.
Le réglage « mouvements réduits » conserve le déplacement instantané.
La respiration reprend au repos, après le déplacement et les réactions de combat.
La validation et ses contrôles sont dans
`docs/reviews/refonte-agent-2026-10-06/actor-glide-01/`.
Après avoir vu l’aperçu intégré, le joueur confirme ce rendu par « Je valide :) ».
Cette validation visuelle finale est conservée dans `visual-acceptance-02.json`
du même dossier ; elle complète l’accord initial sur la proposition.

Les deux poses partagent le cache borné à 192 textures, sans création d'image
par frame après son échauffement. Les captures et tests sont conservés dans
`artifacts/terminal-console-proposition-2026-10-05/`. La proposition de HUD
terminal du même dossier reste à soumettre à l'avis du joueur.

### Banque du bestiaire autorisée le 6 octobre 2026

Le joueur a autorisé l'intégration de toute la sélection actuelle aux fichiers
du jeu : 65 références et leurs cadavres/épaves, dont 23 acteurs existants,
32 nouveaux ennemis (quatre boss de 64 × 64), cinq drones et cinq transformations.
La dernière version du Pillard remplace la proposition médiévale refusée.
Le 60 est retiré et fusionné dans le 66, sans bouclier. Les machines disposent
d'épaves sans sang ; les êtres de chair et hybrides gardent leurs dessins de sang.
Les PNG validés demeurent intacts. Chaque vivant et chaque corps reçoit une
ombre de contact séparée, conformément à la demande du joueur.

La banque `assets/sprites/bestiary/` conserve les numéros, sources et empreintes.
Les 23 indices d'acteurs existants et la structure des sauvegardes restent
inchangés. Le rendu actif utilise leurs cadavres dessinés sous les loots.
Les modèles supplémentaires sont des ressources visuelles disponibles ; cette
intégration ne définit ni statistiques, apparitions, talents, transformations
jouables ou nouvelles collisions. Les noms proposés restent provisoires.
L'autorisation et les contrôles sont dans `artifacts/bestiaire-integration-2026-10-06/`.

## 2. Références observées dans Crawl

Racine étudiée :
`C:/Users/User/Desktop/crawl-master/crawl-master/crawl-ref/source/`.

| Source | Méthode observée | Adaptation attendue |
| --- | --- | --- |
| `enum.h:188-189` | `TILE_X = 32`, `TILE_Y = 32` | Tiles natives de 32 × 32 pixels |
| `rltiles/dc-player.txt` | Catalogue de parties du personnage et d'équipement | Sources organisées par couche et famille |
| `tilepick-p.cc`, `tilep_fill_order_and_flags` | Ordre arrière → avant, visibilité adaptée à la forme et à l'équipement | Ordre déclaré et exceptions explicites |
| `tiledoll.cc`, `fill_doll_equipment` | Parties choisies depuis l'équipement | Configuration graphique issue des slots actifs |
| `tiledoll.cc`, `pack_doll_buf` | Composition des parties et offsets | Corps stable avec éléments superposés |
| `tilemcache.cc`, `get_weapon_offset` et `info` | Corps de monstre et équipement avec offsets par silhouette | Points d'attache propres aux NPC/espèces équipés |
| `rltiles/tool/tile.cc`, `add_rim` | Bordure autour des pixels occupés par voisins orthogonaux | Contour sombre contrôlé, examiné à taille native |
| `rltiles/tool/tile_list_processor.cc`, directives `%rim` | Contour activé par les définitions | Ne pas épaissir automatiquement tous les détails |

Les sources définissent une structure de composition. Elles ne contiennent pas
une limite universelle de douze couleurs ni un filtre automatique de lisibilité.
L'économie de pixels, les silhouettes et les palettes restent des choix de dessin.

Les tiles de Crawl servent à l'étude et aux comparaisons. Les dessins du projet
sont adaptés à son univers. Toute éventuelle réutilisation directe doit identifier
sa provenance et sa licence exacte : `rltiles/license.txt` mentionne le domaine
public pour une **partie** des ressources, pas une garantie globale par fichier.

## 3. Taille native, échelle et cadrage

### Grille de référence

- Rendu courant : **64 × 64 pixels par case** ; version 32 px abandonnée.
- Vue native : un pixel du dessin pour un pixel de l'écran.
- Agrandissements de revue : facteurs entiers ×2, ×3 ou ×4, filtrage nearest.
- Le zoom d'une fenêtre ou du navigateur ne constitue pas une mesure native.
- Une capture indique clairement son échelle ; ne pas juger seulement un grand portrait.

### Clarification BD du 10 octobre 2026

Le joueur reprend la première planche **BD-STYLE01** comme référence de qualité
et préfère ses vues **« BD zoom ×2 »**. Ces vues agrandissent par nearest les
dessins déjà réduits pour une case logique de 32 px : une case est alors affichée
à **64 px**, sans dessin supplémentaire ni changement des proportions.
Le canevas de composition passe de 64 à 128 px à l'écran pour conserver les
débordements ; ce canevas couvre deux cases et ne définit pas la taille du corps.

Le joueur autorise cet affichage du jeu en 64 px **si nécessaire pour conserver
ce rendu**. Cette décision assouplit la contrainte de taille d'affichage, sans
changer les cases logiques, collisions, poses, points d'attache, proportions
relatives des équipements ou ordre des couches. Seul le style de dessin change.
Les sources BD-C01 à BD-C05 de la première étude et leurs réductions originales
servent de référence ; les montages ultérieurs signalés comme défectueux ne la
remplacent pas. Les nouveaux modèles restent à présenter avant intégration.

Trace : `docs/reviews/refonte-agent-2026-10-06/comic-style-reference-03.json`.

### Révision confirmée le 11 octobre 2026 : dessins adaptés aux résolutions

Le joueur suspend temporairement la création des icônes de compétences pour
reprendre les personnages. Il demande deux tailles d'affichage, **32 et 64 px**,
avec une véritable image adaptée à chacune : la version 64 px ne doit plus
être un agrandissement de l'image 32 px. Cette décision remplace la convention
d'agrandissement BD ci-dessus. Les cases logiques, poses frontales, points
d'attache communs, débordements d'armes, palettes et choix de tenue par le
torse ou la transmogrification conservent leurs règles validées.

Les PNJ existants, notamment de quête et de service, doivent également être
contrôlés pour leur taille relative et la résolution réellement utilisée.
La revue **ACTOR-RES01** mesure les six références humaines actives à 30 px
de haut dans une case native de 32 px. Leurs portraits 128 px sont exactement
des agrandissements ×4 ; ils ne fournissent pas de modèle indépendant 64 px.
Les tenues de la banque frontale occupent 29 à 31 px de haut en 32 px.

Les grandes compositions d'armures sont déjà disponibles. La première
comparaison présente un export direct en 64 px, les vues actuelles 32 px et
leurs agrandissements, pour les 28 tenues du catalogue, avec leurs palettes
et seize configurations. Cet export est un candidat de résolution, pas un
redessin déclaré ni une nouvelle acceptation des dessins encore en attente.
La galerie montre les PNJ actuels à côté des nouvelles tenues, à taille réelle.
L'intégration et la restriction du réglage aux deux tailles restent à effectuer
après examen des candidats concrets.

Revue : `artifacts/refonte-agent-2026-10-06/actor-resolution-review-01/`.

Le joueur choisit ensuite « 64 px direct » pour l'intégration. L'accord
**ACTOR-RES01-ACCEPT01** fige les 576 exports des douze tenues déjà acceptées,
leurs trois palettes et seize configurations. Les autres dessins du catalogue
gardent leur statut ; aucune nouvelle apparence de PNJ n'est validée par ce choix.
La banque de rendu utilise un canevas de 128 px pour une case de 64 px, avec
les détails des exports directs et les mêmes proportions. Les dessins 32 px
restent identiques. La résolution sélectionnée entre dans la clé du cache borné.
Les portraits utilisent également la source détaillée. Les réglages proposés
sont désormais 32 et 64 px ; les anciens réglages intermédiaires chargés sont
rapprochés de ces deux tailles sans réinitialiser les autres préférences.

### Revue des armes et armures en 64 px — 16 octobre 2026

Le joueur signale encore un rendu de type 32 px agrandi pour les armes et les
armures, à l'exception de la tenue de mage. Il demande de remplacer les anciens
rendus après préparation de vrais visuels adaptés au 64 px.

L'audit du chemin actif distingue deux problèmes : `game_art_palette.rs`
échantillonne systématiquement les icônes en 32 px ; `game_art_worn.rs` ne
sélectionne la banque détaillée que pour les douze tenues de l'accord
ACTOR-RES01-ACCEPT01. Les autres torses passent par le corps historique 32 px.

La revue **EQUIPMENT-NATIVE64-01** réutilise les sources détaillées conservées,
avec export direct, sans étape 32 px et sans prétendre à un nouveau dessin.
Elle présente 110 armes et 34 modèles d'armure, soit 176 exports d'icônes avec
les trois couleurs des seize tenues associées à un objet torse. Les équipements
déjà supprimés ou rejetés ne sont pas réintroduits. Les douze tenues détaillées
acceptées restent des références inchangées ; sept autres tenues portées sont
des candidats dont le remplacement reste à valider. Trois références acceptées
n'ont actuellement pas d'objet torse associé et sont identifiées comme telles.

Les identifiants, sources et empreintes sont conservés dans
`docs/reviews/refonte-agent-2026-10-06/equipment-native64-01/review.json`.
Le joueur autorise ensuite l'intégration des candidats présentés. Les 176
exports d'icônes sont raccordés au rendu commun du sol, de l'inventaire et des
fiches. Une destination dépassant 32 px sélectionne la source 64 px, y compris
un objet de 56 px dans une case de 64 px. Les petits emplacements utilisent
une réduction des mêmes dessins. La résolution fait partie de la clé du cache.

Les sept tenues supplémentaires disposent des poses et doigts séparés pour
tous les modèles d'armes et les paires à une main. Leurs 336 compositions de
référence sont conditionnées dans `assets/sprites/equipment-native64/`, avec
les icônes et les masques de matériaux. Les 576 compositions des douze tenues
initialement acceptées restent identiques. Les anciens rendus de PNJ et des
contenus historiques restent distincts ; la banque 32 px n'est plus utilisée
pour agrandir les équipements actuels dans les affichages supérieurs à 32 px.

Le joueur refuse ensuite la base rose/cyan de la Carapace Chrysalide et choisit
finalement de reprendre sa variante déjà montrée **olive sombre et os**.
Cette variante approuvée devient aussi sa base sur les icônes, les portraits
et toutes les poses. Les autres variantes gardent leurs pixels. Le choix
graphite précédemment envisagé est abandonné ; aucun nouveau dessin n'est créé.
La décision est conservée dans `palette-defaults-01.json` auprès de la revue.

La page unique `artifacts/apercu-equipement64.html` montre désormais les banques
actives et les captures du jeu. Elle est produite par
`tools/art/show_equipment_native64.py`. Les propositions antérieures restent
reproductibles par `prepare_equipment64_review.py` ; les sources et décisions
sont conservées. Les captures utiles sont embarquées dans cette seule page
d'aperçu ; le résultat réel du nettoyage, y compris un éventuel refus du
contrôle automatique, est indiqué dans `integration-validation-01.json`.

La référence de butin encore présente pour la Tenue d'aviateur, déjà rejetée,
est retirée des nouvelles parties à partir de la génération 149. Le filtrage
du catalogue est réversible pour reconstruire les parties antérieures,
notamment 148 ; les identifiants historiques restent lisibles.

### Prises des armes lourdes et de mêlée — revue du 16 octobre 2026

Le joueur demande surtout des **armes lourdes à distance** plus imposantes
lorsqu'elles sont portées. Les autres armes de tir à deux mains lui conviennent
et conservent leur échelle. Il demande également une prise horizontale pour les
haches et contondantes à deux mains, avec un fer de hache correctement aligné,
sans inclinaison parasite.

La revue **HELD-WEAPON-64-01**, dans
`docs/reviews/refonte-agent-2026-10-06/held-weapon-review-01/review.json`,
présente cinq études numérotées et un fusil témoin inchangé, sur trois tenues.
L'exemple isolé `examples/held_weapon_review.rs` réutilise les dessins sources,
les manches et les doigts séparés, avec des points d'attache identiques entre
armures. Le manche de la hache est tourné exactement de 90 degrés depuis son
axe vertical source, sans reprendre l'angle oblique de la précédente pose.
L'étude de contondante n'attribue pas une prise à deux mains aux marteaux à une
main existants et ne modifie aucune règle.

Le joueur valide ces propositions sous **HELD-WEAPON-64-01-ACCEPT01**.
Le gabarit est raccordé aux 27 modèles actifs de mitrailleuse, lance-flammes,
lance-grenades et lance-roquettes, ainsi qu'aux sept haches à deux mains.
Les fusils ordinaires et armes à une main restent inchangés. La prise
contondante acceptée reste une référence pour les modèles à deux mains ;
elle ne convertit pas les marteaux et masses à une main du catalogue.

Les nouvelles couches d'armes sont conditionnées dans
`assets/sprites/held-native64/`. Les poses utilisent les manches et doigts
des 19 tenues et de leurs trois couleurs, avec les mêmes prises par modèle.
La composition rejoint le cache existant du plateau et des portraits, sans
nouveau calcul à chaque image. Les douze compositions concrètes de la revue
restent vérifiées pixel par pixel et les anciens presets non concernés
conservent leurs pixels. Aucun identifiant ni comportement d'objet ne change.

La page unique `artifacts/apercu-equipement64.html` montre le rendu intégré dans
l'onglet « Prises intégrées », ajouté par `tools/art/show_held_weapon_review.py`.
`tools/art/build_held_weapon_assets.py --check` vérifie les sources, les
correspondances de modèles et la conservation des anciennes banques.

### Humains — proportions

Les sources humaines étudiées occupent environ trente pixels de haut dans une
tile de trente-deux. Utiliser cette échelle comme repère, avec une anatomie
stable entre tenues et configurations d'armes.

- Repère initial : pieds sur `y = 31`, corps approximativement de `y = 2` à `31`.
- Mesurer crâne, épaules, bassin, genoux et pieds ; une hauteur de boîte identique
  ne prouve pas que les personnages ont les mêmes proportions.
- L'armure peut épaissir ou élargir le contour. Une robe peut élargir le bas.
- Un casque peut dépasser légèrement selon sa forme ; ne pas réduire tout le corps.
- Une grande arme ne rétrécit jamais son porteur.
- Tête, mains et lame peuvent être stylisées pour mieux se lire.
- Un bras trop long n'est pas une solution à un mauvais placement d'arme.

### Monstres et animaux

Leur taille relative découle du bestiaire. Un rat reste petit ; un prédateur
massif est plus large ; une chauve-souris se lit par ses ailes ; une machine par
ses supports. Ne pas ramener toutes les silhouettes à la même hauteur ni gonfler
un petit animal parce que la découpe supprime ses marges transparentes.

### Débordements

Déclarer séparément la case logique, le point de contact et les marges visuelles.
Une arme ou un appendice peut dépasser. Un canevas de composition élargi reste
un support de rendu, pas une nouvelle taille anatomique ou une collision élargie.
Les objets au sol sont cadrés dans leur case ; éviter les débordements arbitraires.

## 4. Grammaire du dessin

### Priorités de lecture

1. Identifier la catégorie ou l'espèce à 64 pixels natifs.
2. Lire les grandes masses du corps, du vêtement et de l'arme.
3. Séparer les matériaux et les éléments tactiquement utiles.
4. Reconnaître le modèle par quelques signes propres.
5. Ajouter seulement les détails qui restent utiles à cette taille.

Si un détail gêne une priorité précédente, le retirer ou l'agrandir. La qualité
ne se mesure pas au nombre de rivets, lignes ou rayures.

### Silhouette

- Contour intentionnel et continu, avec espaces négatifs utiles.
- Tête distincte des épaules ; bras distincts du torse ; pieds distincts du bas.
- Hache reconnaissable par sa tête ; fusil par canon/corps/crosse ; matraque
  par son volume contondant, sans garde d'épée ajoutée par habitude.
- Espèces différentes par leur morphologie, pas seulement par leur teinte.
- Préserver les trous et parties disjointes intentionnels des organismes anormaux.

### Contours et lumière

- Généralement un contour sombre d'environ un pixel natif.
- Ne pas avaler un canon, fermer une cavité ou supprimer une patte avec la bordure.
- Séparations internes aux articulations, plis ou changements de matériau.
- Pas de quadrillage de panneaux décoratifs.
- Éclairage simple, de préférence haut gauche, stable entre les assets.
- Deux ou trois valeurs par matériau ; ombres en grandes zones cohérentes.
- Ombre portée, lueur, fumée et particules sur des couches dédiées.
- Une bordure automatique améliore le contraste ; elle ne corrige pas une forme confuse.

### Détails

Un œil, une ouverture de casque, un projecteur ou une poche ventrale peut être
très petit s'il porte l'identité. Les dizaines de points de rouille et reflets
indépendants sont du bruit. Éviter textures photographiques, micro-rayures,
dithering dense, dégradés continus et petits panneaux sans fonction lisible.
Un pixel isolé doit être intentionnel ; un reste de découpe ne l'est pas.

### Point de vue

Humains presque de face, avec une posture cohérente. Animaux de trois quarts ou
de profil si leur anatomie se lit mieux, comme dans les références étudiées.
Le niveau de simplification et l'éclairage restent communs. Éviter le mélange
de portraits peints, animaux réalistes réduits et humanoïdes miniatures.

## 5. Couleurs et matériaux

### Deux grandes zones colorées

Une tenue possède une couleur principale et une secondaire réellement visible.
La seconde occupe un panneau, des manches, un pantalon, un tablier ou une large
partie de robe. Deux pixels de visière ne rendent pas toute une armure bicolore.
Contours, acier, peau et cuir sombre complètent ces masses sans les fragmenter.

### Palette bornée

Objectif initial : environ **12 à 24 couleurs opaques utiles** pour un humain
équipé ou un monstre, moins pour un petit objet. Il s'agit d'un objectif de
fabrication, pas d'une constante de Crawl. Un dépassement doit améliorer la
lecture et se justifier par les matériaux. Plusieurs centaines de teintes
proches pour quelques centaines de pixels signalent une simplification nécessaire.

Le conditionnement peut utiliser une palette indexée fermée sans dithering.
Le dessin doit déjà posséder de bonnes masses. Comparer la vue native avant/après :
yeux, contours, poignées et extrémités ne doivent pas disparaître à l'encodage.
Réduire une palette ne répare pas une mauvaise silhouette.

### Variantes cosmétiques

- Même ID, modèle, nom et statistiques.
- Palette indépendante de qualité et de bonus.
- Variante attachée à l'instance, conservée par équipement, abandon et sauvegarde.
- Même géométrie et mêmes affectations de matériaux pour toutes les variantes.
- Palette de l'arme indépendante de celle de l'armure.
- Points d'attache indépendants des couleurs.
- Présenter les palettes sur la même tenue et les mêmes postures.

### Recoloration

Déclarer des matériaux ou indices : principal, secondaire, acier, contour, peau,
lumière. Leur affectation appartient à la source. Ne pas compter sur des
rectangles approximatifs ou un seuil général de couleur qui change de sens
selon la pose.

Un filtre « orange → bleu » peut toucher la peau, laisser une texture parasite
ou recolorer un fusil avec le torse. Les vêtements et armes ont des couches et
matériaux séparés ; les neutres et la peau restent protégés.

### Contraste

Tester sur sol clair, sombre et chargé. Deux teintes différentes de même valeur
peuvent fusionner. Le contraste clair/sombre compte autant que la teinte.
Duos possibles : rouge/ivoire, violet/turquoise, jaune/violet, cuivre/bleu,
bleu/ivoire, anthracite/turquoise. Ne pas imposer le même accent partout.
Les couleurs sont expressives ; le réalisme n'est pas une limite imposée.

## 6. Personnages, tenues et couches

### Référence fixe

Une référence par tenue pour casque, torse, jambes et bottes. Ces parties ne
sont pas redessinées indépendamment pour chaque arme. Les membres peuvent
bouger ; un avant-bras peut cacher le torse sans changer son dessin.
Les changements autorisés de pose et masques d'occlusion sont explicites.

### Tenue complète choisie par le torse

Le torse choisit une identité cohérente de haut, bas, casque et chaussures.
La structure en couches ne réintroduit pas une icône de plastron collée sur un
ancien personnage encore visible en dessous.

Le joueur confirme le 10 octobre 2026 que **seule l'armure de torse définit le
visuel complet du personnage sans armes**, casque, gants et bottes compris.
Les objets de tête, mains et pieds conservent leurs slots, leurs effets et
leurs illustrations individuelles dans l'inventaire. Les équiper ne remplace
aucune partie de cette tenue complète sur le personnage. Changer d'arme ou de
posture ne change pas non plus son casque.

### Ordre conseillé, arrière vers avant

1. Contact/ombre au sol, si la scène en emploie.
2. Cape, sac et appendices arrière.
3. Corps et parties fixes de la tenue.
4. Membres correspondant à la posture, manches et gants.
5. Parties d'arme derrière une main ou un avant-bras.
6. Arme au premier plan.
7. Doigts ou tranche de gant qui referment la prise au-dessus de la poignée.
8. Effets temporaires indépendants.

Déclarer les exceptions par famille. Un casque ne doit pas être recoloré avec
le torse ; un fusil ne doit pas disparaître dans la couleur de l'armure.
Une grande illustration de l'objet n'est jamais une couche de vêtement portée.

### Configurations à représenter

Sans arme ; mêlée à une main ; tir à une main ; tir à deux mains ; mêlée à deux
mains ; deux armes de mêlée ; deux armes de tir à une main ; une arme de chaque
catégorie. Prévoir le sens inverse des mains pour les combinaisons mixtes.

L'activation d'une arme à une main montre les deux mains actives équipées.
L'activation du slot deux mains montre seulement sa configuration. L'autre
groupe reste équipé mais ne se montre pas dans les mains. Les métadonnées de
prise déterminent la posture ; la portée d'attaque ne détermine pas le nombre de mains.

### Points d'attache

Déclarer en coordonnées natives : pied/sol, épaules, coudes, poignets, prises
gauche/droite, pivot de l'arme, prises principale/secondaire d'une arme à deux
mains et ordre de superposition. Les points correspondent aux pixels de la
silhouette concernée ; une valeur universelle copiée partout ne suffit pas.
Un outil de contrôle peut afficher ces repères temporairement.

### Direction acceptée le 10 octobre 2026 : vue frontale et poses communes

Le joueur choisit une version « plate », strictement de face, sans orientation
privilégiée vers la gauche ou la droite. Toutes les tenues visibles de la
refonte sont construites sur un même gabarit anatomique. Pour une configuration
d'équipement donnée, épaules, coudes, poignets et prises sont communs à toutes
les tenues. Les manches, gants et plaques sont dessinés autour de ces repères.
Les coordonnées communes doivent correspondre au dessin réel, et non seulement
être déclarées dans les métadonnées.

La configuration active choisit la pose : mains indépendantes, arme à deux
mains ou arme lourde horizontale. Les deux mains indépendantes doivent couvrir
les armes simples, deux armes de mêlée, deux pistolets ou pistolets-mitrailleurs,
les associations mêlée/tir dans les deux sens et le bouclier. Une classe
n'impose pas de pose. Une arme conserve son placement et son échelle lorsqu'on
change de tenue ; la banque ne contient pas de correction d'arme par tenue.

Cette direction autorise la préparation des candidats. Leur rendu concret
reste soumis à la revue avant intégration. Un premier essai compare le Bastion
et la robe rouge dans les mêmes configurations. Le redessin des armes jugées
inadaptées, la variété des robes et l'identité visuelle du Métamorphe feront
l'objet de propositions ultérieures distinctes.

Retourner une arme autour de sa prise. Ne pas retourner le casque et la tenue
uniquement pour échanger les mains. Une hache à gauche présente son fer vers
la gauche ; à droite, vers la droite. Les bras restent à l'échelle anatomique.

## 7. Armes

### Icône et arme tenue

Une famille possède une représentation au sol/inventaire et une tenue.
Orientation et simplification peuvent différer, mais type, matériaux et signes
distinctifs se correspondent. Une vignette détaillée n'est pas automatiquement
un sprite tenu. Le même dessin de famille peut représenter plusieurs modèles
statistiques ; la diversité de modèles ne multiplie pas les postures humaines.

### Critères par forme

| Type | À lire immédiatement | Erreurs à éviter |
| --- | --- | --- |
| Couteau/poignard | Lame courte et poignée distinctes | Lame noyée dans une grande garde |
| Épée à une main | Longueur de lame, petit garde, axe continu | Lame excentrée, garde disproportionnée |
| Épée à deux mains | Grande lame et deux prises | Porteur rétréci, main manquante |
| Hache | Fer large relié au manche droit | Manche et tête tordus, mauvais côté du fer |
| Matraque | Volume contondant | Garde d'épée ajoutée |
| Arme d'hast | Hampe, tête, prise | Hampe cassée ou invisible |
| Pistolet | Canon, petit corps et poignée en L | Canon hors de l'axe du bras, poignée flottante |
| Pistolet mitrailleur | Forme compacte différente du pistolet | Silhouette de fusil réduite illisible |
| Fusil/arme lourde | Canon, corps et crosse/nozzle, double prise | Arme fondue dans le torse, deux mains mal placées |

### Anatomie de la prise

L'axe du pistolet suit le bras et la main. Avec le bras descendant du
Technomancien, l'orientation diagonale vers le bas et l'extérieur a été
validée le 9 octobre 2026. Conserver son gant noir et son bras d'origine.
Un pistolet dressé exige une prise et un avant-bras relevés compatibles.
Pour une arme à deux mains, suivre visuellement chaque bras jusqu'à sa prise.
Corriger le pivot plutôt que d'allonger ou retourner la main.

Pour les armes de mêlée à deux mains, le joueur demande le 10 octobre 2026
des prises opposées : main gauche orientée vers le haut, main droite vers
le bas. Adapter les poignets à ces orientations en conservant les points
d'attache communs. Cette correction ne s'applique pas automatiquement aux
poses de fusil ou d'arme lourde.

La partie de l'arme réellement tenue doit être masquée par les doigts du
gant. La demande du joueur de faire disparaître la « crosse » à cet endroit
concerne cette occlusion locale ; les parties libres de l'arme restent visibles.
Utiliser le contour du gant source, sans rectangle noir ajouté.

### Caractère industriel

Une grosse dent, une tête asymétrique, une lame épaisse ou un bloc de culasse
exprime la brutalité à 32 pixels. Garder quelques signes de fabrication et des
masses claires ; éviter micro-vis, rayures et rouille dispersée.
La diversité des palettes ne rend pas chaque centimètre de l'arme multicolore.

## 8. Monstres et NPC

### Contenu de référence

Lire `docs/BESTIAIRE_ET_RENCONTRES.md`, `docs/NOMS_DU_BESTIAIRE.md`, les documents
spécifiques et le contenu actif. Les propositions de peuples ou espèces ne
deviennent pas canon par la production d'une image.

### Fiche d'identité obligatoire

- Nom et ID existants ; statut intégré ou proposé.
- Morphologie, taille relative, nombre de supports/pattes/ailes nécessaires.
- Signe distinctif reconnaissable à taille native.
- Matières, palette et point de contact au sol.
- Capacité à porter ou non de l'équipement ; points d'attache compatibles.
- Fragments séparés et cavités intentionnels si l'organisme est inhabituel.

Ne pas transformer systématiquement animaux, êtres minéraux et organismes
étranges en robots. La diversité de matière appartient au bestiaire.

### Équipement des NPC

Une base stable par espèce/rôle et des armes aux points d'attache propres à la
silhouette. Déclarer les compatibilités ; ne pas coller un fusil à un organisme
sans bras. L'équipement graphique ne change ni espèce, faction ou statistiques.
La couleur ne sert pas seule à distinguer un danger.

### Petites et grandes formes

Petits animaux : contour, museau, posture et appendice principal. Machines :
nombre de supports et outil/projecteur. Anomalies : conserver fragments et
centres vides prévus. Un contrôle de composants alpha doit accepter ces formes
disjointes ; un fond noir peint ne remplace pas un trou transparent.

## 9. Effets et interaction

Attaque de mêlée, recul sur dégât, sang, éclaboussures et corps au sol restent
des systèmes séparés, dans le périmètre des demandes de gameplay déjà autorisées.
Cette définition ne les implémente pas automatiquement.

- Même pixelisation que les acteurs ; effets sur couches dédiées.
- Bump de présentation sans changement d'occupation logique de case.
- Impacts qui ne masquent pas durablement le porteur et son équipement.
- Sang au sol/mur distinct des sprites et halos.
- Les machines entièrement mécaniques (drones, robots, androïdes) ne saignent
  pas : leurs versions détruites sont des épaves sèches, sans flaque rouge ni
  liquide ajouté pour remplacer le sang. Les êtres organiques et les hybrides
  dotés de chair peuvent conserver du sang selon leur matière.
- Corps mort clairement couché avec point de contact cohérent.
- Flash temporaire distinct de la palette cosmétique sauvegardée.
- Durée et priorité compatibles avec la lecture tactique.

## 10. Sources, conditionnement et métadonnées

### Sources

Conserver dessin, références, prompts et corrections. Une planche générée est
une source de travail : ne pas supposer ses cases exactes. Contrôler les silhouettes
qui franchissent les limites et les morceaux de cellule voisine.
La source de la miniature et du tooltip doit être la même référence validée ;
ne pas générer un autre visage ou casque pour l'agrandissement.

Une illustration haute résolution réduite n'est pas validée parce qu'elle fait
32 pixels. Vérifier le résultat natif et simplifier/retoucher si nécessaire.
Les mots « pixel art » dans un prompt ne garantissent ni grille ni lisibilité.

### Transparence et atlas

- Corps, membres et armes sur alpha transparent ; alpha binaire pour les solides.
- Semi-transparence réservée aux effets intentionnels de leur couche dédiée.
- Pas de fond peint, damier, halo, bruit ni morceau voisin dans les cellules.
- Dimensions, nombre de cellules, colonnes, lignes et marges déclarés.
- Filtrage nearest sans prélèvement dans la cellule adjacente.
- Scripts de proposition qui écrivent dans `artifacts/`, pas dans les assets actifs.
- Toute découpe ou réduction de palette se vérifie à taille native après traitement.

### Contrat de travail conseillé

Exemple à adapter à l'architecture réelle ; il ne crée pas un format de sauvegarde :

```json
{
  "id": "core:identifiant_existant",
  "status": "proposal",
  "tile_size": 32,
  "source": "sources/tenue.png",
  "body": "tiles/tenue-body-32.png",
  "parts": ["rear", "fixed", "arms", "weapon", "grip"],
  "foot_anchor": [16, 31],
  "anchors_by_pose": "anchors/tenue.json",
  "material_map": "materials/tenue.json",
  "palette_variants": "palettes/tenue.json",
  "draw_order": ["rear", "fixed", "arms", "weapon", "grip"],
  "visual_overflow": {"left": 0, "top": 0, "right": 0, "bottom": 0},
  "review_number": 1,
  "review_capture": "review/tenue-32.png"
}
```

Les coordonnées d'exemple ne remplacent pas la mesure des sprites. Les clés de
cache incluent modèle, palette, posture, groupe actif et orientation quand ces
valeurs modifient l'image. Ne pas recomposer ni créer une texture identique à
chaque frame ou à chaque survol.

### Nommage et traçabilité

Nommer famille/rôle, variante et résolution. Éviter « final-final-v2 ».
Conserver les numéros entre révisions pour permettre une correction précise.
Enregistrer le candidat accepté, son ID, sa source et les réserves éventuelles.

## 11. Revue visuelle

### Présentation

Planche numérotée avec noms, taille native et zoom entier. Présenter : version
actuelle ou essai précédent correctement légendé ; nouveau candidat ; configurations
d'armes de la même tenue ; palettes de même géométrie ; sols clair, sombre et
texturé ; scène native avec plusieurs acteurs et objets.

Le catalogue sur fond uni sert à examiner les pixels. Une capture en situation
est également nécessaire. Des illustrations agrandies seules ne suffisent pas.

### Douze questions de contrôle

1. La catégorie ou l'espèce se reconnaît-elle à taille native ?
2. Tête, torse, membres et arme se séparent-ils ?
3. La seconde couleur occupe-t-elle une véritable zone visible ?
4. Chaque couleur appartient-elle à un matériau, sans teinte parasite ?
5. Casque, torse et jambes conservent-ils leur référence entre les poses ?
6. Chaque poignée rejoint-elle la bonne main ?
7. La double prise est-elle anatomiquement plausible ?
8. Le porteur conserve-t-il sa taille et ses proportions ?
9. Les détails agrandis restent-ils utiles dans la miniature ?
10. La silhouette se détache-t-elle sur les sols du jeu ?
11. Les variantes conservent-elles géométrie, ID et sens de l'objet ?
12. Les fragments séparés, appendices et cavités du monstre sont-ils intentionnels ?

### Défauts bloquants

Arme flottante ; main retournée ; axe lame/poignée cassé ; taille humaine variable
selon l'arme ; casque redessiné par posture ; armure qui recolore ou absorbe l'arme ;
texture parasite ; fond opaque ; morceau voisin ; échelle animale incohérente ;
sprite illisible à taille native ; contenu nouveau non validé ; remplacement
actif avant la revue demandée. Une compilation ne compense pas ces défauts.

## 12. Contrôles et intégration

### Avant revue

Vérifier dimensions, alpha, cellules non vides, couverture des ID, palettes,
orientations et points d'attache. Comparer le corps fixe entre configurations
hors **masque explicite d'occlusion**. Une arme peut cacher un panneau ; une
tête différente ou une botte raccourcie n'est pas une occlusion.

Comparer repères anatomiques, pas seulement la hauteur opaque. Afficher les
points de prise pour contrôler les raccords. Juger les composants alpha selon
la morphologie : une lame déconnectée est suspecte, un Spectre peut être disjoint.

Pour les armes portées, isoler la silhouette principale **avant** de calculer
son cadrage : une barre ou un point détaché dans une source ne doit pas réduire
l'arme entière. Les couches de doigts et de manches viennent d'une source sans
arme ; un filtre de couleur ne suffit pas à séparer un gant du métal d'une
ancienne poignée. Déclarer les prises par famille : poignée arrière et appui
avant pour un fusil, deux prises sur le manche pour une arme de mêlée lourde.
Raccorder chaque manche au coude du corps fixe, sans agrandir son épaisseur.

Le couteau, la hache et la grande épée à un seul tranchant présentent leur côté
coupant vers l'extérieur du personnage. Une inversion de main inverse ensemble
l'arme, les manches et les doigts ; elle ne retourne jamais l'armure. Les icônes
au sol reprennent la même silhouette nettoyée que l'arme portée.

La revue des prises couvre toutes les bases humanoïdes compatibles, chaque
famille, les deux côtés des armes à une main et les associations d'ambidextrie.
Vérifier à la fois le contact réel doigt/poignée, les raccords aux coudes et
l'absence de nouveaux fragments isolés dans la composition finale.

### Après acceptation

Intégrer les seuls candidats acceptés et leur composition. Préserver les autres
modifications du workspace. Un changement visuel ne modifie ni IDs, statistiques
ou collisions. Une approbation antérieure explicite des candidats concernés
suffit ; ne pas demander une confirmation identique à nouveau.

Vérifier dans le rendu actif : joueur, changement d'arme/groupe, mains inversées,
tooltip, inventaire, objets au sol, NPC équipés, monstres et plusieurs sols.
Tester la persistance si une donnée d'instance ou de sauvegarde est touchée.
Exécuter les contrôles Rust adaptés aux modifications du pipeline de rendu.
Mesurer cache et performances si leur fonctionnement change.

### Compte rendu

Distinguer proposition, aperçu fonctionnel, intégration active, compilation,
tests ciblés, capture en jeu et suite complète. Signaler les défauts réellement
présents. Une planche produite n'est pas une validation de gameplay ; une compilation
réussie n'est pas une acceptation artistique.

## 13. Procédure d'ajout futur

1. Lire cette définition et l'entrée de contenu concernée.
2. Vérifier nom, ID et statut du contenu.
3. Choisir silhouette, échelle, matériaux et deux masses colorées.
4. Définir couches, prises et points d'attache.
5. Produire une référence fixe native.
6. Ajouter membres et armes sans redessiner les parties fixes.
7. Décliner les couleurs sur cette même géométrie.
8. Conditionner ; contrôler alpha, palette et découpe.
9. Construire la revue numérotée et une scène native sur sols du jeu.
10. Corriger les problèmes de lecture et de prise.
11. Recueillir l'avis du joueur sur les candidats concrets.
12. Intégrer les candidats acceptés, vérifier rendu actif et persistance concernée.

## 14. Modèle de brief

```text
Contenu : ID existant, nom, statut.
Usage : joueur / NPC / monstre / objet au sol / arme tenue.
Référence : DCSS pour taille, économie de pixels et qualité.
Grille finale : 64 × 64 ; revue 1:1 et zoom entier nearest.
Silhouette : forme dominante, proportions, parties à préserver.
Échelle : taille relative, pied/contact, marges visuelles.
Palette : zones principale/secondaire, rampes, neutres et peau protégés.
Matière : quelques signes lisibles, détails à omettre.
Composition : couches, ordre, pivot, prises gauche/droite ou double prise.
Invariants : mêmes casque, torse, jambes et anatomie entre configurations.
Variantes : couleurs sur même géométrie, stats et ID inchangés.
Fond : transparent ; aucun halo, décor, texte ou ombre dans le corps.
Contraintes : pas de bruit, texture fine, icône collée ou arme flottante.
Sorties : source, tile native, métadonnées, revue numérotée et scène native.
Livraison : aperçu avant avis, sans intégration automatique.
```

Pour un outil de génération, fournir des références natives agrandies sans
lissage et insister sur les grandes masses. Conserver prompts et corrections,
mais valider le résultat réel, pas la formulation du prompt.

## 15. Règle de référence

L'identité se lit d'abord à 32 pixels. Le corps garde sa référence, les armes
rejoignent des prises définies, les couleurs appartiennent à des matériaux
déclarés, et les monstres conservent leur morphologie. Montrer chaque nouveau
candidat numéroté à taille native avant son intégration.

## 16. Évolution du 9 octobre 2026 : armes des cinq classes et gris neutre

Le joueur demande l'adaptation de toutes les armes aux cinq silhouettes de
la refonte. Le catalogue actuel conserve ses 113 modèles et 18 familles de
dessins. Les prises doivent couvrir chaque famille, les deux mains, les
armes à deux mains et l'ambidextrie. Les prochaines armes réutilisent ces
points d'attache lorsqu'ils conviennent ; une nouvelle géométrie reste à
contrôler et à présenter. La taille du corps et les collisions ne changent pas.

Le pistolet du Technomancien est demandé en gris clair. Les armes et armures
de variante rose doivent passer à un gris neutre, sans supprimer les modèles
ni leurs statistiques ou bonus. Le violet existant reste distinct du rose.
Ces directions sont acceptées ; les palettes concrètes et nouvelles prises
sont présentées dans la revue `WEAPON-SIL01`, sans intégration automatique.

La banque proposée contient 375 configurations sur cinq corps fixes natifs
32 × 32, dans un canevas 64 × 64 réservé aux débordements. Le Traqueur utilise
une couche sans arme préparée depuis sa source disponible : cette couche et
ses nouvelles prises sont à examiner avec la pose au fusil déjà acceptée.
La robe rouge vif, les gants et l'orientation du pistolet précédemment validés
restent acquis. Les avant-bras à deux mains suivent les matériaux de la tenue ;
la palette de l'arme demeure indépendante.

Revue et galerie :
[WEAPON-SIL01, révision 3](../docs/reviews/refonte-agent-2026-10-06/weapon-silhouette-review-03/review-weapon-silhouettes-WEAPON-SIL01-03.json),
[galerie des prises](../artifacts/refonte-agent-2026-10-06/weapon-silhouette-review-03/weapon-silhouette-gallery-02.html).

### Acceptation et intégration des couches

« Oui je valide » accepte explicitement l'ensemble de la galerie, les deux
rendus gris et la base sans arme du Traqueur. La banque est intégrée dans
`assets/sprites/cyberpunk/agent-rigs/` : cinq corps fixes, prises propres aux
silhouettes, armes, manches, doigts et cartes de matériaux séparés. Les 375
compositions produites par le moteur sont identiques aux aperçus acceptés.
Les palettes d'arme et de tenue restent indépendantes. Les familles de
pistolet et de fusil de précision conservent leurs matériaux par défaut
quel que soit le porteur, également dans les illustrations d'objet.

Le rendu conserve les indices de tenue et d'arme existants, les autres
silhouettes, la respiration discrète et le cache borné. La variante `Pink`
devient `Gray` au même indice sérialisé ; les anciennes valeurs JSON `Pink`
sont acceptées en lecture. Les objets, statistiques, bonus et règles ne
changent pas. Cette étape intègre les ressources et leur rendu ; les règles
des cinq nouvelles classes et le reste de la refonte restent des travaux
distincts. Les futures formes nécessitent toujours leur revue propre.

Accord : [WEAPON-SIL01-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/weapon-silhouette-visual-acceptance-01.json).
Banque active : [manifest](../assets/sprites/cyberpunk/agent-rigs/manifest.json).
Contrôles : [validation de l'intégration](../docs/reviews/weapon-integration-2026-10-09/validation.json).
Capture native et zoom entier : [rendu actif](../artifacts/weapon-integration-2026-10-09/crawl-agents-final/cold-start.png).

## 17. Accord du 10 octobre 2026 : extension du gabarit frontal aux armures

Le joueur accepte explicitement `BD-FLAT01`, révision 2 : « ok je valide,
on peut faire pour toutes les armures ». La prise de mêlée à deux mains reste
opposée, main gauche vers le haut et main droite vers le bas. Les doigts
cachent localement le manche ou la poignée tenue. Les armes gardent les mêmes
points d'attache sur toutes les tenues et une palette indépendante.

Le catalogue de revue `ARMOR-FLAT01` présente les 17 tenues existantes
(`AR-01` à `AR-17`), six propositions (`AR-N01` à `AR-N06`) et les 18 pièces
d'inventaire séparées (`AR-P01` à `AR-P18`). Les nouvelles propositions
comprennent deux tenues de mage, deux tenues de combat et deux alternatives
pour la tenue de départ du Métamorphe. Le joueur demande plusieurs propositions,
sans imposer leurs formes ou couleurs. Ces dessins restent à examiner.

Chaque tenue complète possède trois palettes proposées, sur la même géométrie
et les mêmes poses. À la demande explicite du joueur, les casques, gants et
bottes séparés conservent une seule illustration, puisqu'ils ne changent pas
la silhouette affichée sur le plateau. Les affectations de matériaux et les rampes de couleurs sont
enregistrées par source ; les armes ne participent pas à cette recoloration.
Les variantes ne changent ni modèle, ID, statistiques ou futurs prérequis.

Le joueur désignera les modèles à supprimer par leurs numéros de revue.
La préparation de ce catalogue n'autorise aucune suppression. Les dessins
précédents restent accessibles pour comparaison. L'accord sur le gabarit
n'accepte pas automatiquement les nouvelles robes, les alternatives du
Métamorphe ou les palettes proposées.

Le joueur prévoit des prérequis d'attributs pour équiper les armures,
à la manière de Diablo II. Cette direction mécanique est enregistrée ;
les attributs exigés et les valeurs restent à définir avant intégration.

Galerie : [ARMOR-FLAT01](../artifacts/refonte-agent-2026-10-06/armor-flat-review-01/index.html).

## 18. Révision du 10 octobre 2026 : diversité des tenues et progression

Le joueur demande des familles d'armures légères de magicien, légères,
moyennes, lourdes et très lourdes. Les cinq classes commencent avec des tenues
légères plus modestes : leurs anciennes apparences avancées peuvent servir à
des équipements ultérieurs. Cette direction n'attribue pas encore de valeurs
de défense ni de prérequis d'attributs à ces familles.

Les tenues `AR-02` (Tenue Cognefer), `AR-04` (Carapace composite) et `AR-13`
(Tenue d'aviateur) sont désignées pour suppression du futur catalogue.
Les identifiants de revue restent réservés ; les anciennes sources sont
conservées dans la revue précédente pour traçabilité. Cela ne supprime pas
automatiquement les objets de tête, mains ou pieds portant des noms apparentés.

Corrections demandées, encore à présenter : casque métallique ou cuivré sans
bleu parasite pour `AR-08` ; nouvelle palette de référence pour `AR-10` ;
remplacement de l'association cyan/rouge pour `AR-11` ; suppression ou refonte
complète de `AR-12`, dont l'apparence doit devenir réellement futuriste.

Le joueur apprécie explicitement la qualité du **Manteau de mutation `AR-N06`**.
Conserver ce dessin comme référence de qualité sans assimiler cette appréciation
à une autorisation d'intégration de ses règles, de son nom ou de ses palettes.

La diversité concerne les formes, pas seulement les couleurs : têtes et
casques réellement différents, coupes de robes ou de vestes, volumes de
protection, tissus, matières organiques et bottes. Le gabarit commun fixe
l'anatomie et les articulations ; il n'impose pas la même armure ou le même
casque. Toutes les positions de bras et les prises validées restent communes.

La revue `ARMOR-FLAT02` propose `AR-N07` à `AR-N11` comme cinq départs légers
et `AR-N12` à `AR-N14` comme pistes de progression. Ces désignations sont des
libellés de revue, pas de nouveaux noms d'objets validés. `AR-N06` reste intact.
Chaque tenue conserve trois palettes. Les nouveaux dessins sont à examiner
en 32 px et en agrandissement exact 64 px avant toute intégration active.

### Acceptation de ARMOR-FLAT02

« Je les valides et on continue :) » accepte les huit dessins `AR-N07` à
`AR-N14`, leurs trois palettes montrées et leurs compositions avec le gabarit
commun. Les fichiers exacts sont figés dans
[ARMOR-FLAT02-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-02/visual-acceptance-01.json).
Les libellés restent des repères de revue ; cet accord visuel ne crée pas de
valeurs de défense ni de prérequis. `AR-N06` conserve son statut précédent.
La suite porte sur les quatre corrections demandées `AR-08`, `AR-10`, `AR-11`
et `AR-12`, présentées séparément avant intégration.

### ARMOR-FLAT03 — corrections particulières

La revue conserve les ID `AR-08`, `AR-10` et `AR-11` : casque cuivré cohérent
pour le Plastron rapiécé, palette pétrole/acier pour la Tenue de membranes,
palette acier bleuté/ivoire pour Peau-Seconde. Les changements de palettes
conservent la géométrie source exacte. `AR-12` est entièrement redessiné en
tenue futuriste de technicien, avec les mêmes prises communes.

Après son accord, le joueur signale des artefacts jaunes sur `AR-N14`.
La cause est une affectation incomplète des ombres du matériau jaune dans
les variantes. La proposition corrige les couleurs du matériau peint en
protégeant l'acier, les joints et les contours ; la référence jaune et les
positions d'équipement restent identiques. Les autres dessins acceptés sont
conservés. Cette correction est versionnée et présentée avant intégration.

Galerie : [ARMOR-FLAT03](../artifacts/refonte-agent-2026-10-06/armor-flat-review-03/index.html).

### ARMOR-FLAT04 — raccords de couches et nom du Tablier de cuir

Le joueur précise que les traits gris désignent les lignes et petits triangles
parasites sur les manches et côtés du corps, pas les pièces métalliques.
L'audit retrouve des pertes d'alpha au raccord des couches : les découpes du
corps utilisaient les coordonnées mesurées des sources, mais les bras étaient
recalés dans le gabarit commun. Les dessins sources ne portent pas ces fentes.

La nouvelle proposition utilise les mêmes coordonnées pour les masques du
corps et des bras. Le recalage local des poignets reste nul aux frontières
des couches et conserve les prises d'équipement communes. Les gants dessinés
dans la source utilisent le même recalage ; aucune main rectangulaire n'est
ajoutée. Les portions de tablier sont exclues des couches de bras.

La revue couvre `AR-N13`, `AR-N14`, `AR-08`, `AR-10`, `AR-11` et `AR-12`, avec
trois palettes et seize configurations chacune. Les fichiers acceptés dans
ARMOR-FLAT02 restent figés. Cette correction d'assemblage est présentée avant
intégration visuelle active.

Le joueur demande explicitement de renommer `AR-08` en **Tablier de cuir**.
Ce nom remplace « Plastron rapiécé » dans la revue et « Tenue du ferrailleur »
dans la traduction française active du même objet `core:patched_plating`.
L'identifiant, la description et les caractéristiques sont conservés. Cet
accord sur le nom ne valide pas les nouvelles compositions visuelles.

Galerie : [ARMOR-FLAT04](../artifacts/refonte-agent-2026-10-06/armor-flat-review-04/index.html).

### EQ-TRANSMOG01 — apparence de torse indépendante

Le joueur demande un slot en haut à droite de la section silhouette de
l'inventaire, puis valide la synthèse par « parfait :) ». Le libellé est
**Transmogrification**. Un objet torse possédé peut sélectionner la tenue
complète visible à la place du torse réellement équipé. Le vrai équipement
continue à fournir toutes les statistiques et tous les bonus.

Cette sélection utilise le dessin et la palette de l'instance choisie. Elle
ne modifie ni les armes actives ni leurs prises. Aucun nouveau dessin d'armure
n'est créé ou validé par l'ajout du slot ; les revues du catalogue gardent leurs
statuts précédents.

Implémentation : le slot cosmétique référence une instance déjà possédée, sans
déplacer le vrai torse ni créer une copie. Cliquer sur le slot applique le torse
sélectionné ; le sélectionner à nouveau ou utiliser la croix retire cette
apparence. L'action **Transmogrification** est également accessible dans les
actions d'inventaire lorsque la sélection est un torse. L'apparence revient au
torse réel si l'objet utilisé comme visuel est déposé au sol.

Le choix est purement visuel et ne fait pas avancer la simulation. Il est
conservé dans les sauvegardes et dans le journal existant d'équipement ; les
calculs de statistiques ignorent le slot cosmétique. Les captures et contrôles
de cette fonction sont conservés dans `artifacts/transmogrification-2026-10-10/`.

### ARMOR-FLAT05 — catalogue consolidé après la transmogrification

Le joueur confirme le fonctionnement du slot puis demande de reprendre les
armures. La galerie rassemble les 28 tenues conservées et proposées, les
trois palettes et les seize configurations de chaque tenue, ainsi que les
18 illustrations séparées de tête, mains et pieds. Les numéros de revue ne
changent pas. `AR-02`, `AR-04` et `AR-13` restent retirés du futur catalogue.

Les compositions sont référencées directement depuis leur revue d'origine :
aucun dessin, masque, point d'attache ou palette n'est modifié par cette
consolidation. Les six corrections de ARMOR-FLAT04 sont proposées avec leur
statut explicite. Les six autres dessins acceptés de ARMOR-FLAT02 restent
identiques ; l'accord initial sur `AR-N13` et `AR-N14` reste conservé et leurs
corrections de raccords restent à examiner. Les références déjà acceptées
de Bastion et de l'Arcaniste ne sont pas remises en cause par la revue de
leurs variantes. La qualité appréciée de `AR-N06` est distinguée d'une
validation complète de ses palettes.

La galerie permet de choisir grands dessins ou seules vues exactes 32/64 px,
le groupe d'armures, la configuration d'équipement, la palette et le fond.
Cette étape prépare la revue du catalogue ; elle n'intègre pas de nouvelles
statistiques, de prérequis, de noms d'objets ni de ressources visuelles actives.

Galerie : [ARMOR-FLAT05](../artifacts/refonte-agent-2026-10-06/armor-flat-review-05/index.html).

### ARMOR-FLAT05 — acceptation et raccord des classes

La réponse « oui » accepte les six corrections `AR-08`, `AR-10`, `AR-11`,
`AR-12`, `AR-N13`, `AR-N14`, leurs trois palettes affichées et leurs seize
compositions. Les références exactes sont figées dans
[ARMOR-FLAT05-ACCEPT01](../docs/reviews/refonte-agent-2026-10-06/armor-flat-review-05/visual-acceptance-01.json).
Les autres candidats en attente gardent leur statut. Les noms et statistiques
de nouveaux objets ne sont pas validés par cet accord visuel.

Le joueur demande ensuite d'intégrer les cinq classes dans le menu de sélection.
Leurs départs utilisent les tenues légères acceptées `AR-N07` à `AR-N11` :
Bastion, Technomancien, Traqueur, Arcaniste et Métamorphe. Les quatre tenues
existantes corrigées conservent leurs identifiants d'objets. Le torse réel ou
sa transmogrification définit la tenue complète ; les autres pièces n'en
remplacent pas le casque. Les armes restent des couches indépendantes.

`tools/art/build_flat_agent_assets.py` conditionne les 576 compositions exactes
et les couches du gabarit commun. Aucun dessin source n'est repeint. Le corps
occupe 32 pixels logiques ; le rendu à 64 pixels conserve l'agrandissement ×2.
Les trois nouvelles armures de progression `AR-N12` à `AR-N14` sont présentes
dans la banque visuelle et attendent leurs définitions d'objets validées.

Le menu affiche désormais une vignette équipée pour chaque classe et un aperçu
plus grand pour la classe sélectionnée, à la demande du joueur. Ces images
utilisent les mêmes tenues, armes et points d'attache que le personnage en jeu.
Les attributs fixes et les réserves de départ sont raccordés. Les deux rangs
initiaux de compétence sont enregistrés ; leur exécution en combat et les arbres
de compétences restent une étape distincte, décrite dans
`artifacts/refonte-agent-2026-10-06/class-menu-integration-01/integration-scope.json`.

### Compétences — icônes compactes et illustrations de fiche

Le 10 octobre 2026, le joueur réserve les versions compactes aux petits
affichages : arbres, actions et barres de buffs/débuffs. La lisibilité à
32 pixels prime ; une illustration détaillée réduite ne remplace pas une
icône compacte dédiée.

Les versions détaillées servent d'illustrations plus grandes, centrées sous
la description de la compétence. Le joueur valide le 10 octobre 2026 la
disposition `SKILLS-UI02` : illustration de 192 pixels, après le texte RP et
avant les effets chiffrés, dans la zone défilante de la fiche. La validation
est consignée dans
`docs/reviews/refonte-agent-2026-10-06/skill-window-review-02/visual-acceptance-01.json`.
Les effets chiffrés, rangs et actions restent distincts de l'image.

La validation d'une destination ne valide pas les dessins encore en attente.
Conserver les sources détaillées et les validations antérieures ; poursuivre
les revues numérotées des icônes compactes indépendamment de celles des
illustrations.

Le joueur signale ensuite un écart de style entre les premières compactes et
les huit dernières. `SKILLS-IC12` reprend les candidats 13–20 à partir des
références compactes validées, avec contours anguleux et aplats colorés. Ces
révisions gardent leurs numéros et attendent une validation visuelle distincte.
Le 11 octobre, le joueur demande de reprendre les compactes de l'Artilleur
et de Furtivité. `SKILLS-IC13` présente les variantes `D59-C03` (16) et
`SF00-C03` (20), en conservant leurs numéros. Le joueur valide ces deux
compactes le 11 octobre (« ok je valide :) ») ; leurs exports sont ajoutés
à la banque native. La validation est consignée dans
`skill-icon-review-13/visual-acceptance-01.json`. Les
six autres candidats de `SKILLS-IC12` conservent leur état en attente, et les
illustrations détaillées déjà acceptées ne changent pas. Cette revue ne
modifie aucun autre dessin intégré ni effet de compétence.

La reprise suivante prépare `SKILLS-IC14`, numéros 21–28 : Éclaireur,
Mule, drone Traqueur, Gardien, Maintenance des drones, Réparation ciblée,
Blindage amélioré et Autodestruction. Les compactes sont examinées à
32, 64 et 24 pixels, avec les premières icônes acceptées comme références.
Le joueur valide les huit dessins le 11 octobre 2026 (« ok oui »), après
la présentation de cette planche. L'accord visuel est consigné dans
`skill-icon-review-14/visual-acceptance-01.json` ; ces compactes rejoignent
la banque native. Les autres candidats en attente, les illustrations,
les textes et les règles conservent leur état précédent.

`SKILLS-IC15` prépare ensuite six compactes, numéros 29–34 : Interférence,
Faille d'armure, Silence, Rupture de régénération, Désorientation et Contagion.
Cette série reste une proposition visuelle à examiner à 32, 64 et 24 pixels.
Elle ne modifie aucun effet, texte de fiche ni dessin intégré.

Le prototype `SKILLS-UI03` propose des icônes de cartes de 64 pixels, issues
d'exports de 64 pixels, et une icône d'en-tête de 48 pixels. Les dessins des
barres d'état restent affichés à 32 pixels. Cette révision de taille reste
à examiner ; la disposition illustrée validée dans `SKILLS-UI02` est conservée.

`SKILLS-UI04` présente la demande du 11 octobre 2026 : supprimer le rectangle
autour de chaque compétence, rapprocher son niveau de l'icône et le libellé
Actif/Passif de son nom. Le prototype propose un espace de 6 pixels entre
l'icône et le niveau, puis de 2 pixels entre le nom et le type. Les dessins,
les couleurs de rang et les fiches illustrées conservent leurs états de revue.

### Fenêtre native des compétences — 11 octobre 2026

Le joueur demande de reprendre les icônes et de remplacer l'ancien accès par U,
puis choisit explicitement « Nouveau catalogue, puis intégration des effets ».
La fenêtre native présente les 94 compétences dans les 17 branches validées.
U et K ouvrent cette même fenêtre pour les classes de l'agent ; les rangs lus
sont ceux de la partie, et non le profil de démonstration du prototype HTML.
La salle de test utilise également ce lecteur lorsqu'aucune classe n'est
enregistrée sur son personnage.

La disposition reprend les corrections demandées : icônes compactes de 64 px,
aucun rectangle autour d'une compétence, niveau à 6 px de son image, nom et
type centrés sur l'image sans surlignage du nom. L'illustration détaillée
acceptée est centrée sous le texte RP, dans la fiche défilante.

Le conditionnement `tools/art/package_agent_skill_window.py` vérifie et copie
uniquement les compactes et les illustrations déjà acceptées. Une
compétence sans compacte acceptée conserve son identifiant comme emplacement
neutre ; ce n'est pas une validation implicite d'un candidat en attente.
La reprise de la revue conserve la série `SKILLS-IC12`, numéros 13–20.

Cette étape est un lecteur du catalogue : les boutons d'apprentissage et
d'utilisation restent indisponibles tant que les nouveaux effets ne sont pas
raccordés. La consultation ne dépense ni tour ni point et ne modifie pas la
partie. Les coefficients présentés sont ceux des courbes validées ; le calcul
complet avec l'équipement, les attributs et les défenses accompagne le raccord
des effets au moteur, sans reprendre les chiffres fictifs du prototype.
Les captures réelles et les contrôles sont conservés dans
`artifacts/refonte-agent-2026-10-06/skill-window-native-01/`.
Les deux compactes acceptées dans `SKILLS-IC13` sont contrôlées dans les
captures de `skill-window-native-02/`, avec les fiches D59 et SF00 sélectionnées.
Les huit compactes de `SKILLS-IC14` sont contrôlées dans
`skill-window-native-03/`, avec une capture réelle par fiche sélectionnée.

Le 11 octobre, le joueur préfère la direction sombre essayée dans `SKILLS-IC16`
et demande de reprendre les autres dessins. `SKILLS-IC17` présente les trente
compactes restantes, à 32, 64 et 24 pixels. Après sa demande d'un vrai bâillon
pour Silence, la variante 31 devient `AF03-C03`. Le joueur valide la série
« je valide :) ». L'accord exact est consigné dans
`skill-icon-review-17/visual-acceptance-01.json` : ces trente compactes sont
intégrées sans retouche, en conservant leurs numéros et les anciennes sources.
La banque active comprend désormais trente et une compactes et seize
illustrations. Les quatre essais de `SKILLS-IC16` conservent leur état de revue
distinct ; l'accord sur la direction ne les intègre pas implicitement.
Les contrôles et captures natives sont conservés dans `skill-window-native-04/`.

Le retour sur `SKILLS-IC18` valide uniquement le Tir de harpon (39,
`DI05-C01`) et refuse les cinq autres dessins. Cet accord partiel est consigné
dans `skill-icon-review-18/visual-acceptance-01.json` ; seul le Harpon est ajouté
sans retouche à la banque active, qui comprend alors trente-deux compactes
et seize illustrations. Les contrôles et la capture de sa fiche figurent dans
`skill-window-native-05/`.

`SKILLS-IC19`, puis `SKILLS-IC20`, reprennent les cinq dessins refusés selon
les indications du joueur : munitions seules pour Feu nourri et Volée ;
projectile cuivré distinct des plaques bleutées pour Tir perforant, sans sang ;
crâne dans un viseur rouge sans projectile pour Tir de précision ; balle et
goutte de sang pour Tir vampirique. Les cartouches latérales de Volée sont
redressées dans IC20. Les numéros 35–38 et 40 sont conservés ; ces révisions
restent en attente d'un accord visuel propre. Le retour sur deux dessins ne
valide pas implicitement les trois autres.

### SKILLS-IC20 — corrections de tir validées (11 octobre 2026)

Le joueur valide les compactes 35, 36, 37, 38 et 40 : `DI01-C03`,
`DI02-C02`, `DI03-C03`, `DI04-C03`, `DI07-C02`. Leurs exports 64 px sont
intégrés exactement, sans changement de texte ni de mécanique. La banque
compte trente-sept compactes et seize illustrations. Accord figé dans
`skill-icon-review-20/visual-acceptance-01.json` ; contrôles natifs dans
`skill-window-native-06/`. Les quatre essais encore en attente de IC16
restent en attente ; les anciennes versions de tir refusées sont conservées.

### SKILLS-IC21 / IC22 — reprise des symboles de Survie

Le joueur demande de reprendre les candidats 41–43 de IC21 : Égide occulte
doit sembler plus magique et occulte, Barrière d’épine montrer de véritables
épines et Purge évoquer un effet bénéfique. IC22 propose respectivement
`B02-C02` (sceau rituel), `SU03-C02` (ronces) et `SU05-C02` (purification).
Les numéros restent identiques. Les dessins 44 et 45 sont conservés exactement
et restent en attente d’un accord propre. Cette revue ne modifie ni les textes,
ni les règles, ni la banque native. Les cinq candidats de IC22 attendent leur
validation visuelle explicite.

IC23 reprend uniquement Égide occulte (41 — `B02-C03`) à la demande du joueur :
supprimer la cible rouge, conserver le sceau et l’aura violette. Les quatre
autres dessins sont copiés exactement ; tous restent soumis à la revue.

### SKILLS-IC23 — Survie validée

Le joueur valide la planche 41–45 le 11 octobre 2026 (« ca me va on continue »).
Les compactes `B02-C03`, `SU03-C02`, `SU05-C02`, `SU06-C01` et `BD06-C01`
sont intégrées exactement. Accord dans `skill-icon-review-23/visual-acceptance-01.json`,
contrôles dans `skill-window-native-07/`. La banque compte quarante-deux compactes
et seize illustrations ; les textes et règles du catalogue ne changent pas.

IC24 prépare quatre propositions d’Esquive, numéros 46–49 : Leurre holographique,
Traversée de phase, Réflexes balistiques et Pas spectral. Ces compactes sombres
sont présentées à 32, 64 et 24 px ; elles attendent leur validation propre.

IC25 reprend les candidats 47–49 après le retour du joueur : remplacer le
schéma de Traversée de phase par des silhouettes montrant la traversée d’un
adversaire, représenter un mouvement d’esquive sans munition pour Réflexes
balistiques et corriger la pose de la botte de Pas spectral. Les variantes
deviennent `ES05-C02`, `ES06-C02` et `T02-C02`. Leurre holographique (46,
`ES03-C01`) reste identique et en attente ; aucun accord n’est déduit de son
absence dans le retour. Les quatre candidats de IC25 restent soumis à revue,
sans modification des textes, mécaniques ou ressources natives.

IC26 reprend uniquement 47 et 49 à la demande du joueur : Traversée de phase
(`ES05-C03`) conserve la silhouette violette seule ; Pas spectral (`T02-C03`)
remplace la botte par deux empreintes spectrales. Les candidats 46 et 48 sont
conservés exactement depuis IC25 et restent en attente d’accord. La nouvelle
planche reste une proposition visuelle, sans intégration au jeu.

IC27 reprend le candidat 49 (`T02-C04`) : les traces abstraites sont remplacées
par des empreintes de semelles de bottes, avec crampons et talons distincts.
Le halo violet reste discret. Les candidats 46–48 sont copiés exactement
depuis IC26, sans validation déduite. Cette révision attend son accord visuel.

### SKILLS-IC27 — Esquive validée

Le joueur valide la planche 46–49 le 11 octobre 2026 (« ok on valide »).
Les compactes exactes `ES03-C01`, `ES05-C03`, `ES06-C02` et `T02-C04` sont
intégrées sans retouche. Accord figé dans
`skill-icon-review-27/visual-acceptance-01.json`, contrôles natifs dans
`skill-window-native-08/`. La banque compte quarante-six compactes et seize
illustrations. Les textes, paramètres et anciens candidats restent distincts.

IC28 présente les quatre compactes manquantes de la branche Feu : Souffle de
feu (50 — `FE02-C01`), Mur de feu (51 — `FE03-C01`), Émanation soudaine
(52 — `FE04-C01`) et Combustion (53 — `FE06-C01`). Boule de feu (17 —
`FE05-C03`) est repris exactement depuis IC16 et conserve son numéro ; ce
dessin attendait encore un accord propre. Braise conserve son icône validée.
Les cinq candidats restent des propositions, sans modification de la banque
active, des textes ou des mécaniques.

### SKILLS-IC28 — Feu validé

Le joueur valide les cinq dessins le 11 octobre 2026 (« oui j'aime , je valide :) »).
Les compactes exactes `FE02-C01`, `FE03-C01`, `FE04-C01`, `FE05-C03` et
`FE06-C01` sont intégrées sans retouche. Boule de feu conserve le numéro 17
et Braise son dessin déjà validé. Accord figé dans
`skill-icon-review-28/visual-acceptance-01.json`, contrôles natifs dans
`skill-window-native-09/`. La banque compte cinquante et une compactes et seize
illustrations. Cette validation porte sur ces dessins ; les textes et les
mécaniques du catalogue sont conservés.

### SKILLS-IC29 — propositions de Froid

IC29 prépare cinq compactes sombres, numéros 54–58 : Aiguille de givre
(`FR01-C01`), Nova de givre (`FR02-C01`), Vague de froid (`FR04-C01`),
Blizzard (`FR05-C01`) et Explosion cristalline (`FR06-C01`). Le dessin accepté
de Mur de glace (`FR03-C03`, 18) reste inchangé et sert de référence de matériau.
Les propositions sont présentées à 32, 64 et 24 px, avec une vue de 128 px
et un prototype de disposition dans l'arbre à 64 px. Elles attendent leur
accord visuel propre. Les textes, mécaniques et ressources actives restent
conservés ; aucune de ces propositions n'est intégrée à ce stade.

IC30 reprend les seuls candidats 57 et 58 après le retour du joueur : Blizzard
(`FR05-C02`) représente une tempête de neige portée par des bourrasques ;
Explosion cristalline (`FR06-C02`) représente un crâne gelé, sans fragments
détachés autour de lui. Les candidats 54–56 sont copiés exactement depuis IC29
et restent en attente d'accord, sans validation déduite de leur absence dans
le retour. Les deux révisions conservent leurs numéros et sont proposées à
32, 64 et 24 px. Aucun texte, effet de compétence ou dessin actif ne change.

IC31 reprend uniquement Blizzard (57, `FR05-C03`) : le nuage reste en haut
et la tempête de neige descend vers le bas. Les candidats 54, 55, 56 et 58
sont copiés exactement depuis IC30 et restent en attente d'accord. Aucun
accord n'est déduit pour le crâne gelé d'Explosion cristalline. Cette révision
conserve les numéros, les textes, les mécaniques et tous les dessins actifs.

### SKILLS-IC31 — Froid validé

Le joueur valide la planche 54–58 le 11 octobre 2026 (« je valide on continue »).
Les compactes exactes `FR01-C01`, `FR02-C01`, `FR04-C01`, `FR05-C03` et
`FR06-C02` sont intégrées sans retouche. Mur de glace conserve son dessin.
Accord figé dans `skill-icon-review-31/visual-acceptance-01.json`, contrôles
natifs dans `skill-window-native-10/`. La banque compte cinquante-six
compactes et seize illustrations. Les textes et mécaniques restent inchangés.

IC32 présente six propositions de la branche Énergie occulte : Trait arcanique
(59 — `A01-C01`), Déflagration occulte (60 — `A02-C01`), Ponction spirituelle
(61 — `HY05-C01`), Prison sigillaire (62 — `OC04-C01`), Rayon vorace
(63 — `OC05-C01`) et Réanimation (64 — `OC07-C01`). Les matériaux peints
sombres, les sceaux sanglants et les accents violets suivent la direction
acceptée. Canalisation reste inchangée. Ces six dessins attendent leur accord
visuel propre ; aucune mécanique ni ressource active ne change pour IC32.

Le 11 octobre 2026, le joueur précise son accord : « en faite je valide sauf
pour l'image de déflagration occulte, je n'aime pas cette image. ». L'accord
initial est conservé dans `skill-icon-review-32/visual-acceptance-01.json`,
avec cette correction dans `visual-acceptance-02.json`. Seuls `A01-C01`,
`HY05-C01`, `OC04-C01`, `OC05-C01` et `OC07-C01` restent intégrés sans retouche.
`A02-C01` est retirée de la banque active ; ses sources restent archivées.
La banque compte 61 compactes et 16 illustrations. Tests et captures dans
`skill-window-native-11/`, avec distinction des captures avant/après correction.

IC33 reprend le numéro 60, variante `A02-C02`, à la demande : « dessine juste
une explosion occulte :) ». Une explosion violette seule, sans sceau ni
autre objet, est proposée à 32, 64 et 24 px. Cette nouvelle image attend
son accord propre ; les cinq autres icônes, les textes et règles restent
inchangés. L'accord sur cette direction ne vaut pas validation d'une image
générée ensuite. Génération intégrée Imagegen et prompt exact archivés.

### SKILLS-IC33 — Déflagration occulte validée

Le joueur valide « parfait on continue :) » en réponse à IC33. Le dessin
exact `A02-C02` (60) est intégré depuis son export 64 px, sans retouche.
L'ancien `A02-C01` reste refusé et archivé. Accord dans
`skill-icon-review-33/visual-acceptance-01.json`, tests et capture réelle
dans `skill-window-native-12/`. La banque compte 62 compactes et 16 illustrations.
Les autres icônes, textes et effets restent inchangés.

### SKILLS-IC34 — propositions de Transformations

IC34 présente les compactes 65–68 : Colosse biomécanique (`TR61-C01`),
Cuirassé (`TR63-C01`), Possédé (`TR64-C01`) et Abysse (`TR65-C01`). Leurs
cadrages rapprochés reprennent les formes vivantes existantes en conservant
la peinture sombre, les matériaux usés et les silhouettes lisibles à 32 px.
Mue prédatrice et Puissance métamorphe conservent leurs dessins validés.
Les quatre nouvelles images attendent leur accord propre ; les sprites,
textes, règles et ressources actives ne changent pas pour cette revue.
Planche à 128, 32, 64 et 24 px et contexte d'arbre à 64 px dans
`skill-icon-review-34/`. Imagegen intégré, une génération par compétence,
masters et prompts exacts archivés ; aucun changement de dessin par code.

### SKILLS-IC35 — Puissance métamorphe à quatre griffes

À la demande du joueur, l'icône 8 reçoit une proposition ciblée `M03-C03`
avec quatre griffes distinctes au lieu des trois de `M03-C02`. La peinture
sombre, les chairs rouges, les griffes osseuses et les accents violets
conservent la direction existante. Comparaison à 128 px et vues natives
32, 64 et 24 px dans `skill-icon-review-35/`. Génération intégrée Imagegen,
prompt et master archivés ; seuls les exports sont redimensionnés par code.
Cette variante attend sa validation visuelle propre. `M03-C02` reste
active ; l'illustration détaillée, les textes et les effets restent inchangés.
Cette demande ne valide pas les quatre transformations proposées dans IC34.

### SKILLS-IC36 — anatomie des griffes de Puissance métamorphe

Le joueur juge les griffes de `M03-C03` insuffisamment naturelles. La
proposition 8 `M03-C04` reprend leur attache aux doigts, leur courbure et
leurs pointes effilées, tout en conservant quatre griffes, la chair rouge,
les accents violets et la peinture sombre. Le dessin précédent est conservé
pour comparaison. Revue native dans `skill-icon-review-36/`, avec master
et prompt Imagegen archivés. Aucun changement de dessin par code.
La nouvelle proposition reste à valider ; la banque active, les textes,
l'illustration détaillée, les effets et les quatre propositions IC34 sont
inchangés.

### SKILLS-IC37 — symbole de Puissance métamorphe

Le joueur préfère une icône au rendu anatomique de la main griffue. La
proposition 8 `M03-C05` représente quatre griffures séparées, sans main,
en ivoire et bordeaux avec de discrets accents violets. Ce symbole simple
conserve la direction sombre et se présente à 32, 64 et 24 px, avec un détail
à 128 px. Master et prompt Imagegen archivés dans `skill-icon-review-37/`.
Aucune peinture par code ; les précédents essais sont conservés pour audit.
Le nouveau symbole attend sa validation propre. La banque active, les textes,
l'illustration détaillée et les effets restent inchangés ; les quatre
transformations proposées dans IC34 ne sont pas validées par cette demande.

### SKILLS-IC37 — symbole validé et intégré

Le joueur répond « oui ca me va :) » à IC37. Le symbole exact 8 `M03-C05`
remplace l'icône compacte `M03-C02` dans le jeu, sans retouche de dessin.
La version remplacée est conservée dans le registre des variantes antérieures ;
les essais `M03-C03` et `M03-C04` ne sont pas validés. L'accord figure dans
`skill-icon-review-37/visual-acceptance-01.json` ; les contrôles d'intégration
et la capture du jeu figurent dans `skill-window-native-13/`.
La banque conserve 62 compactes et 16 illustrations pour 94 compétences.
Textes, règles, illustration détaillée et autres icônes inchangés. Les quatre
propositions IC34 restent à valider séparément.

### SKILLS-IC38 — reprise de la revue des transformations

Après la validation de Puissance métamorphe, le joueur demande de continuer.
IC38 reprend les quatre candidats 65–68 `TR61-C01`, `TR63-C01`, `TR64-C01`
et `TR65-C01` depuis IC34, avec les mêmes masters et exports, sans nouvelle
génération ni retouche. Leurs numéros et variantes restent stables.
Seul le contexte de présentation est actualisé : il montre désormais le
symbole accepté `M03-C05` aux côtés de `M01-C02` et des quatre propositions.
Les vues 32, 64 et 24 px sont natives. Les quatre candidats attendent leur
validation propre ; la banque active, les textes et les effets sont inchangés.
Revue dans `skill-icon-review-38/`, sources et prompts originaux dans IC34.

### SKILLS-IC38 — transformations validées et intégrées

Le joueur répond « ok on peut continuer » à la revue des quatre candidats
65–68. Les dessins exacts `TR61-C01`, `TR63-C01`, `TR64-C01` et `TR65-C01`
sont intégrés sans retouche depuis les exports 64 px d'IC34. L'accord est
archivé dans `skill-icon-review-38/visual-acceptance-01.json` ; la capture
réelle du jeu et les contrôles figurent dans `skill-window-native-14/`.
Les quatre icônes sont reconnues dans la capture. Les trois tests ciblés,
la compilation et le format passent. La banque compte désormais 66 compactes
et 16 illustrations pour 94 compétences. Les textes et règles ne changent pas.

### SKILLS-IC39 — propositions de Pouvoirs hybrides

Trois nouveaux candidats : 69 Décharge mutagène (`M02-C01`),
70 Échange de position (`HY04-C01`) et 71 Lien de domination (`HY06-C01`).
Décharge déchirante, flèches opposées et crâne sous un signe de domination
conservent la peinture sombre, l'ivoire usé, le bordeaux et le violet.
Les vues natives 32, 64 et 24 px et le détail 128 px sont présentés dans
`skill-icon-review-39/`. Imagegen intégré, une génération par compétence,
masters et prompts exacts archivés. Aucun redessin par code. Ces trois
propositions attendent leur validation propre ; la banque active, les textes
et les effets ne sont pas modifiés par cette revue.

### SKILLS-IC40 — palette verte de Décharge mutagène

Le joueur propose une palette plus verte pour Décharge mutagène. La variante
69 `M02-C02` reprend la silhouette déchirante et la peinture sombre de `M02-C01`,
avec une énergie verte, un cœur vert pâle et des ombres bordeaux. Édition par
Imagegen intégré ; comparaison native à 32, 64 et 24 px dans
`skill-icon-review-40/`, avec détail 128 px, master et prompt exact archivés.
La nouvelle variante attend sa validation visuelle propre. Les fichiers d'IC39
et la banque active restent intacts ; `HY04-C01` et `HY06-C01` restent à valider.
Cette revue ne modifie ni texte, ni règle, ni animation de compétence.

### SKILLS-IC41 — Décharge mutagène sans rouge

Le joueur n'apprécie pas l'essai vert et bordeaux `M02-C02` : le rouge
s'accorde mal au vert et l'ensemble ne convainc pas. Cette variante est
conservée comme essai non retenu. Le candidat 69 `M02-C03` propose une
décharge verte plus compacte, avec un cœur pâle et des ombres vert foncé,
sans rouge. Le dessin est généré par Imagegen intégré, avec master et prompt
exact archivés dans `skill-icon-review-41/`. Vues natives 32, 64 et 24 px,
avec détail 128 px. Nouvelle proposition à valider, sans modification du
catalogue actif, du type de dégâts, de l'animation ni des autres propositions.

### SKILLS-IC41 — Décharge mutagène validée et intégrée

Le joueur valide « ok je valide ceci » avec l'image verte jointe. Ses pixels
RGB et alpha correspondent exactement au master `M02-C03`, indépendamment
de l'URL de galerie affichée dans le contexte du navigateur. Seul le dessin
69 `M02-C03` est intégré, sans retouche, depuis l'export 64 px validé.
L'accord figure dans `skill-icon-review-41/visual-acceptance-01.json` ;
tests, compilation et capture réelle dans `skill-window-native-15/`.
La banque compte désormais 67 compactes et 16 illustrations pour 94 compétences.
Les textes, règles, type de dégâts et animations restent inchangés.

### SKILLS-IC42 — deux pouvoirs hybrides encore à valider

Les dessins exacts 70 `HY04-C01` Échange de position et 71 `HY06-C01`
Lien de domination sont repris depuis IC39, sans génération, retouche ni
redimensionnement. Leur présentation à 32, 64 et 24 px et le détail 128 px
figurent dans `skill-icon-review-42/`. Décharge mutagène `M02-C03` apparaît
uniquement comme contexte validé. Les deux autres dessins attendent toujours
leur validation visuelle propre. La banque active et les effets ne changent
pas pour cette reprise de revue.

### SKILLS-IC42 — pouvoirs hybrides validés et intégrés

Le joueur valide « ok parfait on continue » en réponse à la planche IC42,
qui montrait seulement les candidats 70 `HY04-C01` Échange de position et
71 `HY06-C01` Lien de domination. Les deux dessins sont intégrés sans
retouche depuis IC39. Accord dans `skill-icon-review-42/visual-acceptance-01.json`.
La banque compte 69 compactes et 16 illustrations pour 94 compétences.
Tests, compilation et capture réelle dans `skill-window-native-16/`.
Textes, règles et animations restent inchangés.

### SKILLS-IC43 — mêlée, affaiblissements et buffs

Six nouveaux candidats : 72 `AF06-C01` Contagion, 73 `B01-C01` Frappe de
rupture, 74 `BD04-C01` Visée stable, 75 `BM01-C01` Lame incendiaire,
76 `BM03-C01` Tempo offensif et 77 `BM05-C01` Sceau de vindicte.
Peinture sombre, matières usées et grands symboles lisibles à 32 px.
Une génération Imagegen intégrée par dessin ; masters et prompts exacts
archivés dans `skill-icon-review-43/`. Détail 128 px et vues natives
32, 64 et 24 px présentés sur une planche de trois colonnes et deux lignes.
Les six dessins attendent leur validation propre et ne sont pas intégrés.
La banque active, les textes et les effets restent inchangés par cette revue.

### SKILLS-IC43 — six icônes validées et intégrées

Le joueur valide « ok on peut continuer » en réponse à la planche IC43
présentant les seuls candidats 72–77 : `AF06-C01`, `B01-C01`, `BD04-C01`,
`BM01-C01`, `BM03-C01` et `BM05-C01`. Les six dessins sont intégrés sans
retouche depuis leurs exports 64 px validés. Accord dans
`skill-icon-review-43/visual-acceptance-01.json`. La banque compte
75 compactes et 16 illustrations pour 94 compétences. Tests, compilation
et captures réelles dans `skill-window-native-17/`. Les textes, les règles
et les animations restent inchangés.

### SKILLS-IC44 — propositions Armes lourdes

Six nouveaux candidats : 78 `HL01-C01` Barrage, 79 `HL02-C01` Impact
assommant, 80 `HL03-C01` Déluge d’acier, 81 `HL04-C01` Maîtrise des armes
lourdes, 82 `HL05-C01` Déflagration amplifiée et 83 `HL06-C01` Drain
d’impact. Les projectiles massifs, impacts et explosions reprennent la
peinture sombre des compactes validées. Une génération Imagegen intégrée par
dessin ; masters et prompts exacts archivés dans `skill-icon-review-44/`.
Détail 128 px et vues natives 32, 64 et 24 px sur une planche de trois
colonnes et deux lignes. Ces six dessins attendent leur validation propre
et ne sont pas intégrés. La banque active, les textes et les effets restent
inchangés par cette revue.

### SKILLS-IC45 — signe de bonus, essai refusé

Le joueur préfère des signes explicites quand une icône signale un bonus.
Le candidat 81 `HL04-C02` remplaçait le chevron par un signe « + », via
Imagegen intégré. Le joueur refuse le rendu de l’ensemble puis précise :
« le + on dirait un soin ». Cette variante est archivée dans
`skill-icon-review-45/`, avec le master, les exports et le prompt exact,
mais elle n’est pas intégrée. Un « + » isolé n’est donc pas une convention
visuelle approuvée pour les bonus de dégâts. Le dessin de l’arme doit être
repris ; le prochain symbole sera également soumis à une nouvelle revue.
Les cinq autres candidats Armes lourdes restent à valider.

### SKILLS-IC46 — nouvelle proposition pour Maîtrise des armes lourdes

Après le refus de l’image et du « + » assimilé à un soin, le candidat
81 `HL04-C03` propose une arme lourde en acier simplifiée et un signe « % ».
Nouvelle génération Imagegen intégrée, avec `T03-C02` comme référence de
style seulement. Master, prompt exact et exports sont archivés dans
`skill-icon-review-46/`. Détail 128 px et vues natives 32, 64 et 24 px.
Le dessin et le signe attendent leur validation propre. Les essais
précédents restent archivés ; aucune icône active, aucun texte et aucun
effet de compétence ne sont modifiés.

### SKILLS-IC47 — chevron bleu et couverture des illustrations

Le joueur demande pour le candidat 81 `HL04` le même chevron bleu que
Maîtrise des attaques à distance. La variante `HL04-C04` remplace le signe
de pourcentage de C03 par le double chevron bleu de `T03-C02`, via Imagegen
intégré. Master, prompt et exports dans `skill-icon-review-47/`, avec revue
à 32, 64 et 24 px. Cette variante attend sa validation ; les dessins actifs
restent inchangés.

Le joueur confirme également que les fiches doivent suivre partout la
même politique : icône compacte dans l'arbre, illustration dédiée de
192 px centrée sous le texte RP. L'audit
`skill-icon-review-47/illustration-coverage-audit.json` vérifie les seize
illustrations actives et leurs sources acceptées. Les 78 autres fiches
n'ont pas d'entrée d'illustration dans le catalogue : leurs illustrations
restent à préparer et à valider indépendamment des compactes. Aucun
agrandissement automatique des compactes ne remplace cette série dédiée.

### SKILLS-IC48 — perspective du chargeur de Maîtrise des armes lourdes

Le joueur signale que le chargeur de `HL04-C04` paraît aplati. Le candidat
81 `HL04-C05` reprend ce chargeur en boîte épaisse, avec un côté et un dessus
visibles, en conservant le double chevron bleu et la composition de l'arme.
Édition Imagegen intégrée ; master, prompt exact et exports dans
`skill-icon-review-48/`. Revue à 32, 64 et 24 px, détail à 128 px.
Cette correction reste à valider. Les exports de C04 et les dessins actifs
sont contrôlés par empreinte et préservés ; aucun texte ni effet ne change.

### SKILLS-NATIVE18 — Maîtrise des armes lourdes validée, fiches sans images

Le joueur répond « validée » à la seule planche 81 `HL04-C05` d'IC48.
L'export 64 px exact rejoint la banque native ; accord dans
`skill-icon-review-48/visual-acceptance-01.json`. Les cinq autres candidats
Armes lourdes attendent leur propre validation. La banque contient désormais
76 compactes acceptées pour 94 compétences.

Le joueur demande ensuite : « en faite on va supprimer les images dans les
description des compétences :ca ne sert à rien ». Cette demande remplace
la disposition illustrée SKILLS-UI02 et le projet de compléter les images
manquantes. Les fiches affichent désormais le texte RP, puis les effets
chiffrés, sans illustration ni espace réservé. Les icônes compactes gardent
leur place dans les arbres, actions et barres d'état. Les anciens dessins
et accords sont conservés comme archives ; aucune nouvelle illustration de
description n'est à produire. Demande et contrôles dans
`skill-window-native-18/`. Textes et paramètres des 94 compétences inchangés.

### SKILLS-IC49 — reprise des cinq propositions Armes lourdes restantes

La reprise présente uniquement les candidats encore en attente d'IC44 :
78 `HL01-C01` Barrage, 79 `HL02-C01` Impact assommant, 80 `HL03-C01`
Déluge d'acier, 82 `HL05-C01` Déflagration amplifiée et 83 `HL06-C01`
Drain d'impact. Masters et exports copiés sans retouche, empreintes vérifiées,
dans `skill-icon-review-49/`. Les numéros et variantes restent identiques.
Planche avec détail 128 px et vues natives 32, 64 et 24 px.
Ces cinq dessins attendent leur validation propre ; `HL04-C05`, déjà accepté
et intégré, est exclu de cette planche. Aucune nouvelle génération, aucun
changement d'icône active, de texte, d'effet ni d'illustration de fiche.

### SKILLS-IC49 — cinq icônes Armes lourdes validées et intégrées

Le joueur répond « parfait on continue » à la seule planche IC49 présentant
les candidats 78, 79, 80, 82 et 83. Les dessins exacts `HL01-C01`,
`HL02-C01`, `HL03-C01`, `HL05-C01` et `HL06-C01` rejoignent la banque native
sans retouche. Accord dans `skill-icon-review-49/visual-acceptance-01.json`.
Les six compétences Armes lourdes disposent maintenant de leur icône ; la
banque compte 81 compactes pour 94 compétences. Tests ciblés, compilation,
contrôles d'empreinte et capture réelle dans `skill-window-native-19/`.
Les descriptions restent sans illustration. Textes et effets inchangés.

### SKILLS-IC50 — passifs de Furtivité et compétences de Mines

La revue présente huit nouveaux candidats compacts : 84 `SF01-C01`
Économie de mouvement furtif, 85 `SF02-C01` Discrétion renforcée,
86 `SM00-C01` Pose de mine, 87 `SM01-C01` Réseau de mines,
88 `SM02-C01` Mines de poison, 89 `SM03-C01` Mines de glace,
90 `SM04-C01` Explosion étendue et 91 `SM05-C01` Résidu persistant.
Les variantes élémentaires et les effets étendus reprennent par édition le
boîtier de Pose de mine. Génération Imagegen intégrée ; masters, prompts
exacts, références et exports sont archivés dans `skill-icon-review-50/`.
Planche avec détail 128 px et vues natives 32, 64 et 24 px.

Ces huit dessins attendent leur validation propre avant intégration.
Les 486 exports acceptés et 101 fichiers protégés restent inchangés.
Aucun texte, paramètre, effet mécanique ou visuel de combat ne change ;
les descriptions restent sans illustration. La banque native conserve
ses 81 icônes compactes acceptées sur les 94 compétences.

### SKILLS-IC50 — six icônes de Mines validées et intégrées

Le joueur valide « les autres » après avoir demandé des corrections pour
84 et 85 : l'accord porte uniquement sur 86 `SM00-C01`, 87 `SM01-C01`,
88 `SM02-C01`, 89 `SM03-C01`, 90 `SM04-C01` et 91 `SM05-C01`.
Les six dessins exacts rejoignent la banque native sans retouche ; accord
dans `skill-icon-review-50/visual-acceptance-01.json`. La banque compte
87 compactes pour 94 compétences. Tests ciblés, compilation, contrôles
d'empreinte et capture réelle dans `skill-window-native-20/`.
Textes, règles et animations inchangés ; fiches sans illustration.

### SKILLS-IC51 — batterie seule et silhouette entièrement furtive

Pour 84, `SF01-C02` conserve uniquement la batterie de C01, centrée et
agrandie. Pour 85, le joueur précise pendant la génération que la silhouette
doit être entièrement en mode furtif, comme la partie camouflée de Furtivité.
`SF02-C03` représente ce même personnage entièrement camouflé, sans traînées
latérales ; le portrait opaque C02 est archivé et exclu de la planche finale.
Les numéros 84 et 85 restent stables. Éditions Imagegen intégrées, masters,
prompts exacts et exports dans `skill-icon-review-51/`, avec détail 128 px
et vues natives 32, 64 et 24 px. Ces deux variantes attendent leur propre
validation ; elles ne sont pas intégrées. Les dessins déjà validés et les
règles restent inchangés.

### SKILLS-IC51 — deux corrections de Furtivité validées et intégrées

Le joueur répond « je valide » à la planche présentant uniquement 84
`SF01-C02` et 85 `SF02-C03`. Ces dessins exacts rejoignent la banque native
sans retouche ; accord dans `skill-icon-review-51/visual-acceptance-01.json`.
La banque compte désormais 89 compactes pour 94 compétences. Contrôles
d'empreinte, tests ciblés, compilation et capture réelle dans
`skill-window-native-21/`. Les propositions de Grenades restent indépendantes
de cet accord. Textes, effets et animations inchangés ; fiches sans illustration.

### SKILLS-IC52 — cinq propositions de Grenades

La dernière branche sans compactes présente cinq nouveaux candidats :
92 `SG00-C01` Lancer de grenade, 93 `SG01-C01` Grenade aveuglante,
94 `SG02-C01` Leurre, 95 `SG03-C01` Portée de lancer et
96 `SG04-C01` Puissance des grenades. Le boîtier industriel commun se
décline avec un flash blanc, des ondes sonores, une trajectoire de lancer
ou un double chevron bleu de bonus. La maîtrise des armes lourdes validée
sert uniquement de référence pour ce signe, sans reprise de son arme.
Générations et éditions Imagegen intégrées ; masters, prompts exacts,
références et exports dans `skill-icon-review-52/`. Détail 128 px et vues
natives 32, 64 et 24 px, planche de cinq propositions sur deux lignes.

Ces cinq dessins attendent leur validation propre avant intégration.
Les 526 exports acceptés et 109 fichiers protégés restent inchangés.
La banque native conserve ses 89 compactes sur 94 compétences. Aucun
texte, règle ou effet de combat n'est modifié ; fiches sans illustration.

### SKILLS-IC53 — Grenade aveuglante et Leurre cylindriques

Le joueur demande pour 93 et 94 des grenades cylindriques, sans haut-parleur
pour Leurre, puis précise que seules leurs couleurs doivent les différencier.
93 `SG01-C02` propose un boîtier cylindrique blanc cassé avec quincaillerie
en acier. 94 `SG02-C02` reprend ce même modèle en changeant seulement la
peinture du corps en bleu pétrole. Les icônes montrent uniquement l'objet,
sans flash, onde sonore ou symbole autour. Éditions Imagegen intégrées,
masters, prompts exacts, références et exports dans `skill-icon-review-53/`.
Planche de ces deux variantes avec détail 128 px et vues natives 32, 64 et 24 px.

Les deux dessins et leurs couleurs attendent leur validation avant intégration.
Les trois autres propositions de Grenades (92, 95 et 96) restent inchangées
et en attente de leur propre accord ; aucune validation n'est déduite de
cette demande de correction. Les trente fichiers des dessins précédents,
les 526 exports acceptés et 109 fichiers protégés sont préservés. La banque
native conserve 89 compactes pour 94 compétences. Textes et effets inchangés.

### SKILLS-IC53 — deux grenades cylindriques validées et intégrées

Le joueur répond « parfait, je valide :) » à la planche présentant uniquement
93 `SG01-C02` Grenade aveuglante et 94 `SG02-C02` Leurre. Les deux dessins et
leurs couleurs rejoignent la banque native sans retouche ; accord dans
`skill-icon-review-53/visual-acceptance-01.json`. La banque compte maintenant
91 compactes pour 94 compétences. Trois tests ciblés, formatage, compilation,
contrôles d'empreinte et capture réelle validés dans `skill-window-native-22/`.
Les trente dessins précédents de Grenades sont préservés. L'accord ne concerne
pas les candidats 92, 95 et 96. Textes, règles et animations inchangés ; les
descriptions restent sans illustration.

### SKILLS-IC54 — les trois propositions de Grenades restantes

La planche reprend sans retouche 92 `SG00-C01` Lancer de grenade,
95 `SG03-C01` Portée de lancer et 96 `SG04-C01` Puissance des grenades,
avec leurs numéros et variantes d'origine d'IC52. Masters et cinq exports par
candidat copiés à l'identique dans `skill-icon-review-54/` ; aucune génération
supplémentaire. Détail 128 px et vues natives 32, 64 et 24 px sur une ligne.
Ces trois dessins attendent leur propre accord avant intégration. Les
536 exports acceptés et 111 fichiers protégés restent inchangés.

### SKILLS-IC54 — trois dernières icônes validées ; catalogue compact complet

Le joueur répond « parfait je valide » à la planche présentant uniquement
92 `SG00-C01` Lancer de grenade, 95 `SG03-C01` Portée de lancer et
96 `SG04-C01` Puissance des grenades. Ces trois dessins exacts rejoignent
la banque native sans retouche ; accord dans
`skill-icon-review-54/visual-acceptance-01.json`.

Les 94 compétences du catalogue ont désormais chacune leur icône compacte
validée et intégrée. Les correspondances entre IDs de compétences, dessins
acceptés et fichiers du jeu sont vérifiées ; les entrées antérieures du
registre et leurs dessins restent inchangés. Les cinq icônes de Grenades
sont contrôlées sur une capture réelle de la fenêtre. Trois tests ciblés,
formatage et compilation réussis ; preuves dans `skill-window-native-23/`.
Les descriptions restent sans illustration. Cette intégration ne modifie
ni les textes, ni les paramètres, ni les animations ou effets de combat.

### ANIM-S01 — revue des animations par lots

Le 11 octobre, le joueur demande le même processus de revue que pour les
icônes, avec des aperçus animés et une validation après chaque lot. Le premier
prototype propose 01 `ME02-A01` Fauchage, 02 `FE01-A01` Braise et
03 `FR01-A01` Aiguille de givre en 32 et 64 px natifs, ainsi qu’une interface
de laboratoire `LAB-S01` à examiner séparément. Les trois animations et cette
interface attendent encore leur accord ; les images acceptées restent intactes.
Procédure et limites dans `REVUE_ANIMATIONS_COMPETENCES_AGENT.md`.

### ANIM-S01 — révision d’intensité demandée par le joueur

Le premier aperçu est jugé trop discret. Le joueur demande des animations
beaucoup plus marquées, conçues pour des cases de 64 px ; le 32 px disparaîtra
à terme. Les numéros sont conservés dans la revue suivante, avec les variantes
`ME02-A02`, `FE01-A02` et `FR01-A02`. Les rendus A01 ne sont pas acceptés et
restent archivés. Les variantes A02 attendent un accord visuel ; leur intensité
ne modifie pas leurs zones ou règles de combat. Le réglage 32 px du jeu actuel
reste disponible pendant cette préparation.

### ANIM-S01 — accord sur les trois variantes A02 et rappel du sang

Le joueur valide l’aperçu renforcé en répondant « c’est très bien » et rappelle
les effets de sang sur les blessures. Les variantes `ME02-A02`, `FE01-A02`
et `FR01-A02` sont acceptées ; empreintes dans
`skill-animation-review-02/visual-acceptance-01.json`. `LAB-S01` reste à revoir
séparément. Les rendus sont encore dans le prototype, sans ajout au combat.

L’application du sang aux sorts de feu et de glace est ensuite interrogée par
le joueur. Le choix proposé — blessures physiques et certains pouvoirs
occultes sanglants, effets élémentaires propres au feu et au froid — reste
en discussion. Ne pas transformer cette demande artistique en application
automatique du statut Saignement. Les machines gardent leurs impacts sans sang.

### ANIM-BLOOD01 — répartition artistique du sang acceptée

Le joueur valide ensuite cette proposition par « parfait :) ». Réserver le
sang aux blessures physiques des êtres de chair et aux pouvoirs occultes
qui déchirent la chair ou drainent la vie. Le feu utilise flammes, fumée et
braises ; le froid utilise givre et éclats de glace. Les attaques physiques
avec un élément peuvent combiner leurs rendus. Les machines restent sans sang.

La règle artistique est acceptée ; chaque nouveau rendu reste présenté dans
un lot animé avant intégration. Elle ne crée pas de statut Saignement.
Accord : `skill-animation-review-02/blood-policy-acceptance-01.json`.

### ANIM-S02 — lot de mêlée proposé

La revue suivante présente 04 `B01-A01` Frappe de rupture, 05 `ME03-A01`
Percée, 06 `ME04-A01-MUR` et 07 `ME04-A01-CIBLE` Repousser. Les deux derniers
aperçus distinguent les collisions déjà prévues contre un mur et un monstre.
Les effets de sang concernent les blessures des cibles de chair ; les éclats
de cloison restent métalliques. Candidats en 64 px natifs dans
`skill-animation-review-03/`, à valider avant intégration. Les sprites,
icônes, règles chiffrées et animations précédemment acceptés sont conservés.

### ANIM-S02 — accord visuel sur 04 à 07

Le joueur répond « tres bien :) » aux quatre GIF montrés. `B01-A01`,
`ME03-A01`, `ME04-A01-MUR` et `ME04-A01-CIBLE` sont acceptés, avec les
projections de sang et les deux collisions représentées. Accord et empreintes
dans `skill-animation-review-03/visual-acceptance-01.json`. La vue `LAB-S02`
reste proposée séparément ; les règles chiffrées ne changent pas.

### ANIM-S03 — derniers actifs de mêlée proposés

08 `ME05-A01` Charge et 09 `ME06-A01` Tourbillon poursuivent la revue en
64 px natifs. Le premier montre une ruée jusqu’au contact, le second une
frappe circulaire sur les huit voisins avec le saignement déjà prévu.
Ces rendus et la vue de laboratoire `LAB-S03` restent proposés dans
`skill-animation-review-04/`. Les accords sur 01 à 07 sont conservés.


### ANIM-S03 — Charge et Tourbillon acceptés

Le joueur répond « je valide oui :) » aux GIF 08 `ME05-A01` Charge et
09 `ME06-A01` Tourbillon. L’accord couvre les deux rendus exacts, y compris
les blessures et le saignement montrés. Empreintes dans
`skill-animation-review-04/visual-acceptance-01.json`. La vue `LAB-S03`
reste proposée séparément ; les règles chiffrées et le combat restent inchangés.

### ANIM-S04 — trois premiers sorts de Feu proposés

La revue propose 10 `FE02-A01` Souffle de feu, 11 `FE03-A01` Mur de feu
et 12 `FE04-A01` Émanation soudaine en cases natives de 64 px. Torrent
en cône, rideau traversable et gerbe ascendante restent distincts. Les
blessures utilisent flammes, fumée et braises, sans sang, selon
`ANIM-BLOOD01`. Le délai d'un tour d'Émanation est annoncé par des fissures
avant l'explosion ; le temps de lecture ne remplace pas ce tour de jeu.

Ces rendus et la vue `LAB-S04` restent proposés dans
`skill-animation-review-05/`, avant intégration. Les acteurs, icônes,
paramètres et animations précédemment acceptés demeurent inchangés.

### ANIM-STYLE01 — flammes dessinées proposées, règle générale à décider

Le joueur demande une cohérence visuelle avec les personnages et un nouvel
aperçu avant de fixer le style commun de toutes les animations. Les rendus
10 `FE02-A02`, 11 `FE03-A02` et 12 `FE04-A02` proposent des flammes dessinées
en images successives, avec contours irréguliers, masses orange et jaunes
et cœurs ivoire. La robe rouge acceptée sert de référence ; acteurs et
équipements restent intacts. Sources et prompts Imagegen intégré archivés
dans `skill-animation-review-06/`.

Ces aperçus restent à valider. Cette entrée décrit une proposition de revue,
sans établir une nouvelle règle artistique générale ou réviser les accords
sur les animations précédentes. La règle commune sera rédigée après l'avis
du joueur sur le rendu animé.

### ANIM-OVER01 — directions et essai de superposition

Les animations directionnelles doivent fonctionner vers la droite, la gauche,
le haut et le bas, indépendamment de la pose frontale des personnages.
Les corps ne tournent pas avec le sort. Le joueur autorise un essai de grande
animation complète, avec aperçu de zone très visible et clignotant, et murs
solides redessinés devant les flammes. Le feu reste derrière les personnages.
Les sols libres ne sont pas découpés par les ombres des obstacles ; seules les
cases de l'aperçu déterminent les effets. Le nouveau rendu A06 reste soumis
à revue dans `skill-animation-review-10/`. Les essais précédents sont conservés,
dont ANIM-OCC02 comme solution de secours. Cette proposition ne fixe pas encore
un style général accepté pour toutes les animations.

### ANIM-BURN01 — animation du statut Brûlure

Le joueur accepte les petites flammes peintes existantes comme animation
temporaire de Brûlure et demande qu'elles passent derrière le corps affecté.
Conserver leur dessin, leur palette et leur orientation verticale ; la visée
du sort ne fait pas tourner ces flammes de statut. L'accord exact et les
empreintes figurent dans `skill-animation-review-10/burn-visual-acceptance-01.json`.
Il ne modifie pas les paramètres mécaniques de Brûlure et ne valide pas le
grand souffle superposé. Le prototype applique la couche derrière les acteurs ;
le statut de la campagne reste à raccorder.

### ANIM-OVER02 — correction des règles de superposition acceptée

Le joueur valide le style peint d'ANIM-OVER01 et précise que chaque obstacle
doit couper l'animation là où il se trouve. La portion arrêtée ne doit pas
réapparaître derrière le mur. Cette instruction remplace la permission
précédente de garder du feu décoratif sur les sols protégés derrière une
cloison. L'aperçu clignotant disparaît dès la confirmation de l'attaque.
Brûlure reste visible sur tout personnage affecté, joueur ou monstre, avec
les petites flammes acceptées derrière son corps. Ne pas confondre ce statut
avec la durée d'affichage du grand souffle ni ajouter de dégâts alliés.

Les corrections A07 et le couloir long sont présentés dans
`skill-animation-review-11/` avant leur intégration. Les autres accords
restent valables. Ces instructions ne valident pas automatiquement les
animations encore à créer pour les autres compétences.

Le joueur précise ensuite que les flammes du statut doivent être sur le
monstre ou le joueur concerné. Les effets partent de la position de son
corps et suivent ses déplacements. Dans la revue A02, les flammes principales
restent derrière le corps et quelques petites langues du même dessin passent
devant la silhouette pour rendre le statut visible sur les robes. Ce nouvel
agencement de couches reste à examiner sur l'aperçu corrigé.

### ANIM-OVER03 — ouverture du couloir vers une pièce

Le joueur souhaite retrouver les formes peintes lorsque le souffle sort
d'un couloir dans une pièce, même si le dessin dépasse légèrement les cases
libres. Éviter de prolonger les murs du couloir par une coupure triangulaire
dans toute la pièce. Le nouvel essai A08 élargit la visibilité du dessin
depuis l'ouverture accessible, en gardant les arrêts sur les murs physiques.
Les dessins sources, les quatre directions, l'aperçu avant lancement et
Brûlure restent conservés. Cet habillage n'ajoute pas de cases de dégâts.
Comparaison animée A07/A08 et candidats numérotés dans
`skill-animation-review-12/`, à examiner avant intégration.

### ANIM-OVER03 — rendu accepté et règles des prochaines animations

Le joueur valide explicitement l'animation A08 par « cette animation est
validée :) on gardera les regles pour les prochaines animations :) ».
Les quatre directions de Souffle de feu et le rendu montré dans la comparaison
A07/A08 sont acceptés. Empreintes des GIF, dessins sources et code du rendu
dans `skill-animation-review-12/visual-acceptance-01.json`.
Les mentions de proposition ci-dessus décrivent les étapes antérieures.

Appliquer les règles suivantes aux prochains lots :

- Garder le style dessiné cohérent avec les personnages : contours lisibles,
  masses de couleur et ombres peintes. Concevoir des effets marqués pour
  des cases natives de 64 px.
- Conserver les formes du dessin dans les espaces libres. Un léger débordement
  visuel des frontières de cases est permis ; les cases de gameplay continuent
  seules à déterminer les effets.
- Pour un souffle ou un flux qui quitte un couloir, retrouver son ampleur à
  l'ouverture vers une pièce. Ne pas prolonger les murs latéraux par une coupe
  triangulaire artificielle dans tout l'espace ouvert.
- Respecter les obstacles qui bloquent la propagation : arrêter la portion
  concernée à leur emplacement et la garder arrêtée derrière la cloison.
  Le débordement visuel ne crée pas un passage au travers d'un mur.
- Afficher un aperçu de ciblage visible et clignotant avant confirmation,
  puis retirer entièrement son remplissage et ses contours au lancement.
- Permettre la visée dans les quatre directions, avec des personnages qui
  restent de face et des armes dont les points d'attache restent conservés.
- Garder les corps lisibles devant le grand effet. Brûlure reste attachée au
  joueur ou au monstre affecté : flammes principales derrière le corps et
  petites langues visibles sur la silhouette. Le statut continue après la fin
  du souffle et disparaît quand il prend fin ; ce n'est pas un feu de terrain.
- Garder le temps d'animation indépendant des tours, des durées de statut
  et des recharges. Conserver les noms de compétences affichés au-dessus
  du lanceur et la règle artistique du sang `ANIM-BLOOD01`.
- Présenter les nouveaux dessins et rendus en lots animés numérotés avant
  intégration. Cet accord fixe les règles communes, sans valider d'avance
  les animations des autres compétences ni une nouvelle règle de combat.

L'accord porte sur le rendu et ces règles artistiques. La vue de laboratoire
reste à examiner séparément ; le raccordement des effets au combat de la
campagne demeure une étape distincte.

### ANIM-S05 — deux prochains sorts de Feu proposés

11 / FE03-A03 Mur de feu et 12 / FE04-A03 Émanation soudaine reprennent
les flammes peintes existantes et les règles acceptées d'ANIM-OVER03.
La revue présente une zone ouverte et une cloison pour chacun. Le rideau
reste traversable et les flammes de Brûlure suivent le corps qui en sort.
Les fissures d'Émanation annoncent un tour de préparation avant les gerbes.
Les grandes flammes passent derrière les corps ; les petites langues de
statut restent visibles sur leur silhouette. Les obstacles solides et la
pièce séparée par une cloison complète restent sans flammes.

Ces nouveaux rendus et `LAB-S05` restent proposés avant intégration dans
`skill-animation-review-13/`. L'accord sur Souffle de feu est conservé.

### ANIM-TEMPS01 — mouvement en secondes, présence en tours

Le joueur valide le dessin et la cadence ralentie de la revue 14, puis
précise que l'animation doit rester tant que le tour de l'effet n'est pas
passé. Conserver la lecture des images en temps réel, mais piloter la
présence de l'effet par le compteur de tours. Un effet visuel d'un tour
continue à boucler pendant l'attente du joueur, jusqu'au passage de ce tour.
Un effet de plusieurs tours reste présent jusqu'à l'expiration de ses tours.
Une durée en secondes ne doit pas le retirer automatiquement pendant
l'attente. Maintenir son dessin ne réapplique pas ses dégâts.

La cadence de huit dessins par seconde de Mur de feu et Émanation est
acceptée, y compris les petites flammes de Brûlure montrées dans la revue.
Émanation doit corriger sa disparition automatique en secondes. La revue
15 propose ce maintien visuel, avec attente prolongée et passage manuel de
tour ; ce rendu corrigé reste à examiner. Les sources et captures précédentes
restent conservées. Cette consigne n'ajoute pas une durée de zone de dégâts
à un sort instantané et ne modifie pas les recharges ou les règles de combat.

Le joueur valide ensuite la revue 15 par « parfait je valide et on continue :) ».
Le maintien animé `FE04-A05-TOUR` et la règle de présence pilotée par les tours
sont acceptés. L'accord est enregistré dans
`skill-animation-review-15/visual-acceptance-01.json`. Appliquer ce maintien
aux prochaines animations d'effets ; chaque nouveau rendu reste à présenter.

### ANIM-S06 — prochains impacts de Feu proposés

13 / FE05-A01 Boule de feu et 14 / FE06-A01 Combustion conservent
ANIM-OVER03, ANIM-BLOOD01 et ANIM-TEMPS01. La boule dessinée et la grande
déflagration utilisent une nouvelle planche transparente à examiner dans
la revue 16 ; la Brûlure reste attachée aux corps avec son dessin accepté.
Les quatre directions et une cloison sont montrées. Les grands effets
restent derrière les silhouettes, les murs les coupent, et leurs impacts
restent animés tant que le tour suivant n'est pas passé. Les nouveaux
dessins et leur assemblage ne sont pas encore acceptés.

Le joueur précise que les prochains sorts de Feu ne doivent pas forcément
reprendre exactement les mêmes flammes : il demande un beau rendu dans le
même style. Conserver les contours dessinés, les masses de couleur lisibles
et la cohérence avec les acteurs, en permettant des formes et des dessins
propres à chaque compétence. Les nouvelles images restent proposées avant
validation ; les rendus déjà acceptés ne sont pas remplacés par cette liberté.

### ANIM-S06-A02 — projectile et collision de Boule de feu

Le joueur demande explicitement une boule visible qui part du lanceur puis
explose sur la cible qu'elle touche. La revue 17 garde le numéro 13 / FE05
et propose A02 : trajet plus lisible, collision sur le premier monstre de la
ligne, puis grande déflagration centrée sur lui. Une scène place D devant
la cible A ; D intercepte le projectile. Le contact d'une cloison est montré
à sa face libre, avec les mêmes coupures physiques du dessin. La cadence
et le maintien par tours restent conservés. Le nouveau trajet ne crée pas
de tour de préparation supplémentaire. Ce rendu et cette fixture de collision
restent à examiner avant leur raccordement au combat.

### ANIM-TEMPS02 — distinguer impact instantané et effet persistant

Le joueur précise que Boule de feu n'a pas d'effet de zone durant plusieurs
tours : une fois son projectile et son explosion terminés, l'animation prend
fin. Les impacts instantanés ne doivent donc pas être maintenus artificiellement
jusqu'au prochain tour. Leur mouvement et leur extinction sont visuels, en
temps réel, et ne font avancer ni les tours, ni les recharges, ni les statuts.
Seule une éventuelle Brûlure reste attachée au corps pendant ses propres tours.
Cette distinction s'applique également au choc bref de Combustion.

Le maintien déjà accepté d'Émanation `FE04-A05-TOUR` et les autres effets
persistants restent conservés. `ANIM-TEMPS02` précise la règle temporelle ;
il ne transforme pas un impact instantané en zone persistante.

Après la demande d'effets moins envahissants et plus transparents, la revue
19 propose `13 / FE05-A04` et `14 / FE06-A03` : projectile réduit, explosion
plus petite et semi-transparente, deux petites flammes ciblées pour Combustion,
sans couronne d'explosion. Les impacts se jouent une fois puis s'éteignent.
Les quatre directions, les coupures par cloison et la Brûlure attachée aux
corps sont conservées. Ces deux nouveaux assemblages attendent leur accord
visuel ; les brouillons et sources antérieurs restent archivés.

### ANIM-S06-A05 — dessiner un éclatement plutôt qu'une sphère

Le joueur précise que jouer avec la transparence ne signifie pas rendre
tout le sort transparent. Il demande une vraie explosion, et juge peu
convaincante la boule de feu représentée au moment de l'impact.

La revue 20 propose un dessin neuf en six étapes chronologiques : flash
compact, flammes projetées vers l'extérieur, séparation des fragments,
puis dissipation des dernières braises. L'impact se joue une seule fois,
sans retour des dernières images vers un noyau plein. Le début reste marqué ;
la transparence soutient les bords et la fin. La durée brève et l'extinction
automatique sont conservées. Les sources précédentes restent archivées.

`13 / FE05-A05` attend son accord visuel. Le projectile réduit, la Brûlure
attachée aux corps, les coupures par murs et `14 / FE06-A03` sont conservés
dans cet aperçu. Aucun effet de combat ou dessin d'acteur n'est modifié.

### Validation d'ANIM-S06 et ajustement du trajet

Le joueur valide la revue 20, puis demande d'accélérer légèrement le
déplacement de la boule avant son explosion. Le dessin et l'assemblage de
`13 / FE05-A05`, ainsi que `14 / FE06-A03` Combustion, sont acceptés.
L'accord et les empreintes sont conservés dans
`skill-animation-review-20/visual-acceptance-01.json`. La demande de vitesse
est distinguée de cette validation.

La revue 21 propose `13 / FE05-A06` : trajet de 0,50 seconde au lieu de
0,65 seconde. L'explosion garde ses six dessins, ses 0,55 seconde et son
extinction. Brûlure et Combustion gardent leur cadence et leurs compteurs
de tours. Le lecteur compare les deux trajets simultanément. Seul cet
ajustement de vitesse attend son accord visuel ; les autres accords restent
conservés. Aucun dessin n'est régénéré.

Le joueur demande ensuite « tu peux encore accelerer :) ». La revue 22
propose `13 / FE05-A07`, avec un trajet de 0,35 seconde, comparé à la version
précédente de 0,50 seconde. L'explosion garde son rythme et les images validées.
La vitesse de la revue 21 reste une proposition dépassée par cette demande ;
la nouvelle vitesse attend son aperçu et son accord.

### ANIM-RAPIDE01 — lancements vifs pour le mélange roguelike et hack'n slash

Le joueur précise : « quand c'est un rogue like croisé avec un hack'n slash
les sorts et autres compétences actives doivent s'activer rapidement ».
Viser des lancements vifs, des projectiles rapides et des impacts courts
et lisibles dans les prochains aperçus. Distinguer cette nervosité du
maintien visuel des zones et des statuts pendant leurs tours.

Cette direction ne crée aucun tour de préparation ni aucun délai de jeu
nouveau. Les préparations et durées mécaniques déjà définies demeurent
distinctes du mouvement visuel. Conserver les dessins déjà validés et
présenter les ajustements de cadence par lots avant leur intégration.

Le joueur valide ensuite la revue 22 par « ok je valide :) ». Le trajet de
0,35 seconde de `13 / FE05-A07` est accepté. L'accord et les empreintes des
dessins, captures et code présentés figurent dans
`skill-animation-review-22/visual-acceptance-01.json`. Les validations du
dessin de l'explosion et de Combustion dans la revue 20 restent conservées.
Cette validation fixe la vitesse de référence de ce projectile ; les autres
compétences restent soumises à leur propre aperçu.

### ANIM-S07 — Nova de givre et Mur de glace, candidats à examiner

La revue 23 propose `15 / FR02-A01` et `16 / FR03-A01`, en cases natives
64 px. Nova projette une onde givrée autour du lanceur en 0,45 seconde ;
elle se joue une seule fois puis disparaît. Le mur surgit en blocs
translucides en 0,22 seconde, reste pendant ses tours, puis se brise.
Chaque bloc peut être détruit séparément. Les dessins d'acteurs et les
images de Feu déjà acceptés sont conservés.

Les deux planches de six étapes proviennent d'imagegen intégré, avec
fond transparent, sources et prompts conservés dans la revue. Leur
assemblage conserve les mêmes points d'ancrage entre les étapes.
L'onde peut déborder sur le sol libre ; les murs physiques et une cloison
pleine coupent le dessin à leur surface. La prévisualisation cyan clignote
avant le lancement et disparaît à la confirmation. Les quatre orientations
cardinales sont montrées sans retourner les personnages.

`GEL-A03` propose des petits cristaux translucides aux pieds de la cible,
derrière et devant son corps, pour laisser les bras et la tête lisibles.
Ce nouveau rendu accompagne le lot ; il ne remplace pas encore le Gel
précédemment accepté pour Aiguille de givre. Aucun sang n'est ajouté au Froid.

La fixture montre des réussites de Gel préparées. Un monstre gelé peut
frapper le mur sans bouger ; le second coup préparé détruit un bloc.
Après la fin de son Gel, il passe par la brèche. Une Brûlure initiale
disparaît lorsqu'un Gel s'applique. Ce sont des exemples pour la revue,
sans calcul de dégâts, hasard, IA réelle ni modification de la campagne.

L'horloge des dessins ne fait avancer aucun tour, statut ou délai de recharge.
Dans le lecteur, les tours n'avancent qu'au clic sur « +1 tour » ; les GIF
de montage indiquent les cinq passages de tour préparés. Ces deux rendus,
leur cadence et `GEL-A03` attendent leur accord visuel. Les règles mécaniques
du catalogue restent distinctes de cette validation.

### FR03-A02 — varier les blocs du Mur de glace

Le joueur demande d'améliorer le mur, puis propose explicitement des blocs
qui ne sont pas identiques. La revue 24 conserve le numéro 16 / FR03 et
propose trois modules aux sommets et fissures différents, assemblés sur une
base commune. Leur dessin déborde légèrement sur le sol libre pour joindre
les modules ; les obstacles physiques coupent toujours la barrière.
Chaque case conserve son bloc destructible et ses règles déjà définies.

Les trois modules possèdent chacun un état intact, fissuré et brisé. Leur
apparition monte depuis le sol sur 0,22 seconde, avec une progression
continue du dessin. L'effondrement dure 0,30 seconde et reste propre au
bloc détruit. Ces mouvements visuels n'ajoutent pas de délai mécanique.

Le nouveau dessin est créé avec imagegen intégré. La planche source et
le prompt sont conservés dans la revue ; la préparation recadre les modules
à la même échelle et aligne mécaniquement leur pied sans les repeindre.
Nova, le Gel attaché aux pieds, les personnages et les règles de la fixture
sont conservés. L'ancien mur reste disponible pour la comparaison animée.

`16 / FR03-A02` est une proposition à examiner ; la demande d'amélioration
ne valide pas son dessin. Le rendu de Nova et du Gel n'acquiert pas d'accord
supplémentaire à travers cette révision du mur. Aucun raccordement de ces
nouveaux effets au combat de la campagne n'est effectué par cette revue.

Le 12 octobre 2026, le joueur valide la comparaison de la revue 24 par
« okay je valide :) ». `16 / FR03-A02` est accepté visuellement : les trois
formes de blocs jointes, leur base commune, la montée continue de 0,22 seconde,
les états fissurés et l'effondrement indépendant de 0,30 seconde deviennent
la référence du Mur de glace. L'accord et les empreintes des sources, atlas,
captures et code présentés sont conservés dans
`skill-animation-review-24/visual-acceptance-01.json`.
Cette comparaison concernait le mur ; elle ne donne pas de validation
supplémentaire à Nova, au Gel ou aux règles numériques de combat.

### ANIM-S08 — Vague de froid et Blizzard, candidats à examiner

La revue 25 propose `17 / FR04-A01` et `18 / FR05-A01`, en cases natives
64 px. La Vague est un front large de brume, de givre et de petits éclats.
La bande de départ, large de trois cases, est choisie autour de l'agent
avant le lancement. Son premier contact reste immédiat ; chaque tour
suivant la fait avancer d'une case, jusqu'au trajet de cinq cases défini.
Une interpolation visuelle de 0,18 seconde accompagne chaque déplacement.
Les portions arrêtées par un obstacle ne réapparaissent pas derrière lui.
La Vague ne laisse pas de zone persistante sur les cases traversées.

Blizzard est une grêle descendante, avec une brume légère et des frappes
dispersées. Le dessin de formation commence au lancement. Le délai d'un
tour complet explicitement défini pour les premières frappes reste conservé ;
la règle générale de démarrage des effets persistants ne devance pas ce
déclenchement différé. L'exemple au rang 11 prépare quatre cases frappées
par application, sur cinq applications. Les réussites de Gel sont préparées.
La neige qui reste animée entre les tours n'ajoute aucune application.

Les deux nouvelles planches proviennent d'imagegen intégré, avec leurs
sources et prompts conservés. La préparation recadre les cellules à
la même échelle et conserve leur transparence. Les personnages et leur
équipement restent inchangés. Le Gel aux pieds conserve le dessin de
la revue 23 ; aucun sang n'est ajouté aux sorts de Froid.

Les essais montrent les quatre orientations cardinales, des obstacles
partiels, des cloisons pleines et une bande placée à droite du même agent.
L'aperçu cyan clignote avant le lancement, puis disparaît. La Vague ne
prévisualise que sa bande initiale et sa flèche, sans ajouter un aperçu
de tout son trajet. Le laboratoire permet de tourner cette bande autour
du personnage sans déplacement ni dépense avant la confirmation.

Le lecteur emploie des boucles natives séparées après les impacts brefs,
pour maintenir les dessins sans rejouer les frappes ou faire avancer
les tours. Ces deux rendus attendent leur accord visuel. Cette fixture
ne calcule ni dégâts ni hasard et ne modifie aucune campagne, sauvegarde
ou IA réelle. Les règles finales de géométrie et des grandes cibles restent
distinctes de la revue des dessins.

Le 12 octobre 2026, après avoir vu les deux rendus et demandé si Blizzard
dure plusieurs tours, le joueur répond « ok parfait :) je valide :) »
à l'explication de sa durée. `18 / FR05-A01` est accepté visuellement.
La durée présentée est confirmée : trois tours au rang 1, cinq tours au
rang 11 ; l'exemple produit quatre frappes par tour actif après son tour
de préparation. La neige reste animée entre les actions, tandis que les
frappes et le décompte avancent seulement avec les tours de jeu.
L'accord et les empreintes des dessins, code et captures sont conservés
dans `skill-animation-review-25/blizzard-acceptance-01.json`.
Cette confirmation centrée sur Blizzard ne donne pas d'accord implicite
à Vague de froid, aux chiffres de dégâts ni à une intégration en campagne.

### ANIM-S09 — Explosion cristalline, candidat à examiner

La revue 26 propose `19 / FR06-A01`, en cases natives 64 px. Un éclatement
bref de glace projette des fragments autour de chaque source gelée visible.
Les six dessins chronologiques se jouent une fois sur 0,42 seconde, puis
disparaissent. Le centre reste ouvert et les personnages se dessinent devant
les fragments. Aucun sang, sphère lumineuse ou nouveau Gel n'est ajouté.

La planche provient d'imagegen intégré ; la source et le prompt sont conservés.
L'extraction mécanique recadre chaque cellule avec le même retrait de douze
pixels pour exclure les séparateurs de la planche, puis applique une échelle
commune. La transparence générée et les dessins d'acteurs sont conservés.

La fixture prépare trois sources proches, une source seule et des voisins
non gelés. Une cible au croisement de plusieurs explosions reçoit plusieurs
contacts préparés, sans devenir une nouvelle source. Le Gel des sources est
consommé ; les personnages de cet exemple survivent à leur emplacement.
Une source gelée cachée reste exclue et n'est pas affichée. Les contacts
ne calculent aucun dégât, défense, décès ou effet à la mort.

Les essais montrent quatre dispositions cardinales, un obstacle local et
une cloison pleine. Les murs physiques coupent les dessins et la cloison
protège les voisins situés de l'autre côté. L'aperçu cyan clignote avant
le lancement, puis disparaît à la confirmation. Le mouvement du dessin
ne consomme aucun tour supplémentaire, ne relance aucun contact et ne
réduit pas la recharge. Les tours restent commandés explicitement.

`19 / FR06-A01` attend son accord visuel. Les petites empreintes cardinales
de rayon un servent à la revue ; les diagonales, grandes cibles et règles
de géométrie finales ne sont pas implicitement validées par cette fixture.
Aucune campagne, sauvegarde, IA réelle ou statistique de combat n'est modifiée.

### ANIM-GEL01 — rendre le statut Gel animé sur le corps

Le joueur précise qu'il manque une animation du statut Gel, comparable à
la Brûlure attachée aux personnages. Les petits cristaux fixes aux pieds
ne répondent pas à cette demande. La revue 27 propose `20 / GEL-A04` : une
enveloppe de glace translucide autour du corps, avec des reflets lents et
quelques fragments de givre. Le même dessin est présenté sur l'agent et sur
un monstre en cases natives 64 px. Le voisin non gelé reste inchangé.

Les six dessins viennent d'imagegen intégré ; leur source et leur prompt
restent dans la revue. L'extraction des cellules et leur redimensionnement
uniforme préservent leur transparence. Les personnages et leur équipement
ne sont pas redessinés. Les murs physiques coupent le givre.

La boucle visuelle ne réduit aucune durée et ne répète aucune application.
Les tours explicites seuls réduisent le Gel. Son expiration, son remplacement
par Brûlure ou sa consommation par Explosion cristalline retirent le dessin
attaché. Les scènes préparent ces transitions sans changer les règles du
statut. `19 / FR06-A02` conserve l'explosion de la revue 26 et lui associe
ce nouveau Gel avant lancement ; aucune mort automatique n'est ajoutée.

Les anciens aperçus restent intacts. `GEL-A04` et son assemblage avec FR06
attendent leur validation visuelle. Le laboratoire reste une fixture isolée,
sans calcul de dégâts, IA réelle, hasard, sauvegarde ou intégration en campagne.

### GEL-A05 — placer le givre devant le corps

Après discussion du placement, le joueur répond « ok alors on fait ca :) »
à la proposition d'une glace semi-transparente devant le torse et les jambes,
avec quelques cristaux derrière et le visage lisible. Cet accord autorise
la préparation de cette direction ; le nouveau rendu reste à examiner.

La revue 28 conserve le numéro 20 et propose `GEL-A05`. Imagegen intégré
reprend les six dessins pour placer des plaques fines dans leur partie
centrale. Au rendu, le quart supérieur de cette couche est exclu pour
dégager la tête ; le quart inférieur de l'ancienne couche garde quelques
cristaux derrière les pieds. Le givre au premier plan devient dominant.
Les sources RGBA restent intactes ; seules l'extraction mécanique et la
composition par couches changent. Les anciens aperçus sont conservés.

`19 / FR06-A03` associe ce Gel à l'explosion déjà présentée, sans modifier
son éclatement, sa cadence ni ses contacts préparés. La fin du Gel, son
remplacement par Brûlure et sa consommation retirent toujours le givre.
Les murs coupent les dessins ; les compteurs avancent uniquement par tours.
Ces deux assemblages attendent leur validation visuelle. Aucun effet de
combat, statistique, dessin d'acteur ou sauvegarde de campagne n'est modifié.

Le 12 octobre 2026, le joueur valide la revue 28 par « je valide oui ».
`20 / GEL-A05` et `19 / FR06-A03` sont acceptés visuellement : givre
translucide au premier plan sur le torse et les jambes, tête dégagée,
quelques cristaux derrière les pieds, reflets persistants et retrait lors
de la fin du Gel, de Brûlure ou de sa consommation par Explosion cristalline.
Les rendus, sources, dépendances et captures acceptés sont identifiés par
leurs empreintes dans `skill-animation-review-28/visual-acceptance-01.json`.
Ces versions deviennent les références de ce statut et de son assemblage
avec l'explosion. Les brouillons antérieurs restent conservés ; les autres
revues gardent leur statut. Cette validation ne change aucune règle chiffrée
et ne constitue pas un raccordement au combat de la campagne.

### ANIM-S10 — Trait arcanique et Déflagration occulte

La revue 29 propose `21 / A01-A01` et `22 / A02-A01`, en cases natives
64 px. Trait arcanique est un trait violet court entouré de filaments
sombres, suivi d'un petit nœud noir à l'impact. Son trajet visuel dure
0,18 seconde et son impact 0,22 seconde. Ce trajet n'introduit aucune
préparation mécanique ni règle de perforation ou d'interception.

Déflagration occulte fait éclater un sceau rouge sombre au sol en fragments
noirs et violets. Ses six dessins se jouent une fois sur 0,55 seconde.
Le sceau se dessine d'abord au sol derrière les acteurs, puis les fragments
passent au premier plan avec des espaces transparents. Le rouge appartient
au tracé du sceau ; il ne crée aucun statut de Saignement ni blessure nouvelle.
Les deux effets sont des impacts brefs qui s'éteignent sans attendre un tour.

Les sources RGBA et prompts d'imagegen intégré sont conservés dans la revue.
L'extraction mécanique des cellules à échelle commune préserve leur alpha.
Les personnages, équipements, dessins et validations précédents restent
intacts. Les aperçus cyan clignotent avant lancement et disparaissent à
la confirmation ; les murs physiques coupent les effets.

Les exemples montrent les quatre orientations, un obstacle partiel, une
cloison pleine et l'agent proche de sa propre zone. La cloison refuse la
cible inaccessible du Trait ; la Déflagration peut viser la case libre de
son côté. L'agent est exclu des contacts de sa propre zone. Les recharges
et contacts restent des compteurs préparés, distincts de l'animation.
Les diagonales et règles finales de ciblage restent à traiter séparément.

Ces deux rendus attendent leur validation visuelle. Le prototype est isolé :
aucun calcul de dégâts, statut, hasard, IA réelle, sauvegarde ou raccordement
au combat de la campagne n'est ajouté. Les fiches et règles du catalogue
conservent leurs accords et limites précédents.

### ANIM-S10-COLOR01 — rendre le violet occulte lisible

Le 13 octobre 2026, le joueur apprécie les dessins de la revue 29, mais
signale que le violet est trop peu visible, puis précise que le problème
concerne ce violet très foncé. La revue 30 conserve les numéros 21 et 22
et propose `A01-A02` / `A02-A02` avec des reflets violets plus clairs.

La correction se fait dans le shader des effets du prototype. Les zones
violettes déjà colorées sont éclaircies vers le lavande ; les noyaux noirs
et les tracés rouges conservent leur rôle sombre. Aucun dessin n'est
régénéré et les textures RGBA, leur alpha, les dimensions, les points
d'ancrage et les cadences restent identiques. La correction est désactivée
par défaut et ne concerne que cet aperçu, avec son option explicite.

La galerie compare les deux palettes sur les mêmes scènes en cases natives
64 px. Les murs continuent à couper les effets, l'aperçu disparaît à la
confirmation et les impacts se jouent une seule fois. Les anciens fichiers
restent conservés. Ce rendu plus clair attend sa validation ; le retour
sur le violet foncé ne constitue pas un accord global sur les couleurs,
les autres effets ni une intégration au combat de la campagne.

### ANIM-S10-COLOR02 — lavande plus marqué

Le joueur demande un violet encore plus clair après la revue 30, car
il demeure trop peu visible. La revue 31 conserve les numéros 21 et 22
et propose `A01-A03` / `A02-A03`. Le lavande clair couvre une plus grande
partie des facettes violettes ; les pointes deviennent presque blanches.
Les noyaux noirs neutres et le sceau rouge gardent leur contraste.

Ce réglage est une option distincte du shader du prototype. Le réglage
précédent reste disponible ; les textures et leur alpha ne sont pas
retouchés. Dimensions, points d'ancrage, cadence, zone, murs et compteurs
restent conservés. Les deux palettes se comparent sur les mêmes scènes
en cases natives 64 px. Cette nouvelle couleur attend sa validation.
Elle n'est pas appliquée à la campagne ni aux autres effets du jeu.

### ANIM-S10-COLOR03 — violet franc retenu pour la suite

Le joueur demande un violet clair mais franc, « purple », plutôt que lavande.
La revue 32 propose `21 / A01-A04` et `22 / A02-A04` : les facettes violettes
et leurs pointes restent saturées, sans devenir pastel ou presque blanches.
Les noyaux noirs et le sceau rouge conservent leur rôle. Le shader ajuste
uniquement la couleur ; les dessins, leur alpha, la taille, les ancrages,
la cadence et les coupures par murs restent identiques.

Sur cet aperçu, le joueur répond « ca me semble mieux oui ». Ce violet
franc est retenu comme référence pour poursuivre la revue des animations
occultes. La décision et les empreintes du rendu sont conservées dans
`skill-animation-review-32/palette-acceptance-01.json`. Elle porte sur le
choix de couleur et ne vaut pas accord global sur les animations, les
textes ou les règles de combat. Les icônes ne sont pas modifiées et le
prototype demeure distinct de la campagne.

### ANIM-S11 — Ponction spirituelle et Rayon vorace proposés

Après le choix de violet franc, le joueur demande de poursuivre les lots.
La revue 33 propose `23 / HY05-A01` Ponction spirituelle et `24 / OC05-A01`
Rayon vorace, dans des cases natives 64 px. Leurs dessins proviennent de
deux nouvelles planches imagegen intégré, avec alpha conservé et extraction
mécanique de cellules à la même échelle.

Ponction contracte brièvement l'ombre sous la cible, puis aspire des éclats
rouges vers l'agent. La séquence se joue une fois en 0,48 seconde. Rayon
vorace conserve un fil noir veiné de rouge, bordé de facettes violettes,
pendant ses tours. Une pulsation d'essence rejoint l'agent uniquement à
chaque drain préparé ; l'attente garde le fil animé sans nouveau drain.
Sa rupture s'estompe en 0,12 seconde. Ces cadences restent purement visuelles.

La revue montre les quatre directions, un obstacle partiel, une cloison
qui refuse la cible inaccessible, et les interruptions du Rayon par un
déplacement ou un manque d'énergie. L'aperçu clignote avant lancement,
puis disparaît à la confirmation. Les personnages restent de face et
les murs coupent les dessins.

Les deux rendus attendent leur validation propre. Le prototype ne calcule
aucun PV, dégât, énergie, soin, hasard, statut ou comportement d'IA réelle.
La restitution de vie du Rayon et celle d'énergie de Ponction gardent leurs
règles précédentes ; les compteurs de cette revue sont préparés et ne
raccordent pas ces compétences au combat de la campagne. L'accord de
couleur précédent ne valide pas automatiquement ce nouveau lot.

### HY05-A02 — épaissir l'essence aspirée

Le joueur apprécie le lot 33 mais demande que Ponction soit plus épaisse,
car elle se lit difficilement. L'accord sur `24 / OC05-A01` Rayon vorace
est consigné dans `skill-animation-review-33/ray-acceptance-01.json` ;
`23 / HY05-A01` fait l'objet de la révision demandée.

La revue 34 propose `23 / HY05-A02`. Les éclats aspirés gardent leurs
dessins et deviennent deux fois plus épais perpendiculairement au trajet,
avec une longueur multipliée par 1,5. Leur placement, couleur, opacité,
vitesse et durée restent conservés, comme l'ombre initiale sous la cible.
Le réglage se fait au rendu, sans modifier les PNG source ni leur alpha.
La nouvelle épaisseur attend sa validation, avec une comparaison animée
sur les mêmes scènes natives 64 px. Rayon vorace et les compteurs préparés
restent identiques ; aucun raccordement à la campagne n'est ajouté.

### ANIM-S11-INTENSE01 — renforcer les deux drains

Le joueur précise que les deux animations restent trop légères : cette
demande concerne désormais aussi Rayon vorace. Son ancien accord reste
archivé, sans être étendu au nouveau rendu. La revue 35 propose
`23 / HY05-A03` et `24 / OC05-A02`, tous deux à valider.

Ponction utilise une ombre initiale plus marquée et plusieurs dessins
d'essence en mouvement pour former un flux dense. Rayon superpose des
filaments larges, avec un cœur noir et rouge et des facettes violettes,
puis une pulsation de retour plus présente à chaque drain. Ces dessins
composent uniquement le rendu d'une résolution : ils n'ajoutent aucun
drain, dégât ou contact. Les sprites source, leur alpha et les règles
restent conservés. La séquence de Ponction demeure brève, tandis que le
Rayon persiste pendant ses tours et se rompt avec les mêmes interruptions.

Les quatre directions, obstacles, aperçus et compteurs sont repris dans
les mêmes scènes natives 64 px. Les nouveaux flux débordent visuellement
des cases cibles mais les murs continuent à les couper ; l'emprise
mécanique ne change pas. La galerie compare les intensités avant accord,
toujours dans un prototype isolé de la campagne.

### ANIM-S11-INTENSE01 — deux drains validés

Le joueur répond « beaucoup mieux :) » sur le rendu renforcé de la revue 35.
`23 / HY05-A03` Ponction spirituelle et `24 / OC05-A02` Rayon vorace sont
retenus comme références visuelles : flux dense d'essence pour Ponction,
faisceau épais et pulsation de retour marquée pour Rayon. L'accord et les
empreintes des sources, images natives et animations sont enregistrés dans
`skill-animation-review-35/visual-acceptance-01.json`.

Cette validation porte sur ces rendus. Les règles chiffrées, les autres
compétences et l'intégration au combat de la campagne gardent leur statut.
Les anciens essais restent conservés avec leur historique de revue.

### ANIM-S12 — Prison sigillaire et Réanimation proposées

La revue 36 présente `25 / OC04-A01` Prison sigillaire et `26 / OC07-A01`
Réanimation en cases natives 64 px. Deux planches imagegen intégré fournissent
les segments sanglants, leur rupture, le sceau de marque, la pulsation de
réanimation et un petit repère de tête de mort ivoire. Les images RGBA sont
extraites à échelle commune, sans repeindre les sources. Les personnages
vivants et leurs dépouilles gardent les images du bestiaire existant.

Le cercle se trace rapidement au sol en huit segments indépendants, autour
d'une case centrale libre. Dans l'événement préparé, un segment se brise,
la cible emprunte la brèche et les sept autres segments restent jusqu'à
l'expiration. Le placement est refusé si le pourtour est bloqué.

Le sceau demeure sous la cible jusqu'à sa mort préparée. Une pulsation rouge
et une courte transition dépouille/personnage vivant accompagnent le retour
à la vie ; elles ne créent pas de délai mécanique. La tête de mort identifie
l'allié et suit son mouvement. Une avance et un petit choc vers un autre
ennemi illustrent son autonomie. La durée commence à onze tours au retour
à la vie, puis baisse uniquement aux tours explicites de l'aperçu.

Les quatre orientations, le pourtour bloqué, une cloison et une attente
sans événement sont proposés. La galerie se déroule par « +1 tour » ; le
GIF accélère les derniers tours pour montrer les expirations. Les murs
coupent les dessins et la prévisualisation clignotante disparaît au lancement.

Ces rendus, dont le repère de tête de mort, attendent leur validation. Les
événements restent préparés : aucun calcul de PV de segment, dégâts, IA
réelle, soin, loot, XP, nouvelle règle de résurrection ou interface Familiers
n'est intégré. Le prototype demeure distinct de la campagne. Le joueur
réinitialise la scène pour rejouer sa séquence complète.

### ANIM-S12 — visibilité et sang redessiné, en revue

Le joueur trouve les effets de la revue 36 trop sombres et trop fins. La
revue 37 propose `25 / OC04-A02` et `26 / OC07-A02` : rouge plus clair,
tracés épaissis, sceau et pulsation plus présents. Les sources restent
identiques et les règles ne changent pas. Cet aperçu demeure une proposition.

Le joueur demande ensuite un sang plus joli. La revue 38 prépare
`25 / OC04-A03` et `26 / OC07-A03`, avec des planches imagegen redessinées :
sang cramoisi plus fluide, volumes peints, ombres bordeaux et reflets courts,
moins de traits emmêlés. Les segments assemblés conservent un cercle et une
brèche distincte. Le sceau au sol reste séparé de la courte pulsation du
retour à la vie. La tête de mort alliée conserve exactement son ancien dessin.

Les PNG et prompts sont conservés dans la nouvelle revue ; leur mise en atlas
se limite au découpage, au redimensionnement uniforme et à la reprise exacte
de la cellule de tête de mort. Un réglage d'exposition propre à cet aperçu
conserve les nuances du sang ; les anciens rendus restent disponibles.
Les durées, recharges, placement, segments et événements préparés restent
identiques. Ces variantes attendent leur validation visuelle et ne définissent
pas un nouveau style global accepté. Aucune intégration à la campagne.

### ANIM-S12 — sang de la revue 38 validé

Le joueur répond « ca me semble bien mieux ui » après le dernier aperçu
de la revue 38. `25 / OC04-A03` Prison sigillaire et `26 / OC07-A03`
Réanimation deviennent les références visuelles retenues : cercle sanglant
plus fluide avec segments indépendants et brèche distincte, sceau au sol
et rubans de sang du retour à la vie. La tête de mort alliée conserve son
dessin dans cet aperçu. Les anciennes variantes et les essais intermédiaires
restent archivés.

L'accord et les empreintes des dessins consommés, des prompts, des images
natives présentées et des animations sont conservés dans
`skill-animation-review-38/visual-acceptance-01.json`. Cette validation
concerne ces deux rendus ; les règles chiffrées déjà retenues et les autres
compétences gardent leur statut. Le laboratoire reste un prototype isolé
et cet enregistrement ne raccorde pas les compétences au combat de la campagne.

### ANIM-S13 — deux corps en 64 px et leurs métamorphoses proposés

La revue 39 présente `27 / M01-A01` Mue prédatrice et `28 / TR61-A01`
Colosse biomécanique. Les corps `M01-BODY01` et `TR61-BODY01` reprennent les
silhouettes et palettes des références 62 et 61 dans le style BD : acier bleuté,
chair orangée et griffes ivoire pour le Prédateur ; grandes plaques ivoire,
chair rouge sombre et joints noirs pour le Colosse. Chaque PNG natif 64 px
provient directement d'un nouveau dessin haute définition. Les grandes vues
de la planche sont des zooms d'inspection explicites, pas les modèles du jeu.

Des rubans mutagéniques verts accompagnent Mue prédatrice ; des arcs violets
et quelques facettes cosmétiques accompagnent Colosse. La transition d'entrée
dure 0,38 seconde, celle du retour 0,30 seconde. Ces durées sont graphiques :
la forme est active dès le lancement et ses 22 tours de l'exemple au rang 11,
Volonté 10 ne diminuent qu'aux tours explicites. Le renouvellement préparé au
tour 10 remet sa durée à 22 sans afficher une forme humaine intermédiaire.
La prévisualisation de la case du joueur clignote avant lancement et disparaît
à la confirmation. Les quatre orientations et un mur proche sont présentés.

Les sources et prompts imagegen intégrés sont conservés avec les PNG natifs,
les atlases VFX et les captures. Le conditionnement se limite au découpage,
au redimensionnement uniforme et au placement des couches. La banque du
bestiaire existante reste conservée. Les nouveaux corps et leurs transitions
attendent des validations distinctes ; les attaques propres aux formes seront
présentées dans des lots suivants. Le laboratoire est un prototype visuel,
sans calcul de PV, statistiques, dégâts, soin, énergie, IA, loot ou sauvegarde,
et sans intégration à la campagne.

### ANIM-S13 — corps et transitions validés

Sur la galerie 39 présentée, le joueur répond « ok je valide on peut continuer ».
Les corps `M01-BODY01` et `TR61-BODY01`, ainsi que les transitions
`M01-A01` et `TR61-A01`, sont validés. La preuve et les empreintes des
dessins, prompts, captures natives et GIF sont conservées dans
`skill-animation-review-39/visual-acceptance-01.json`. Les attaques propres
ne sont pas incluses dans cet accord. Les règles numériques déjà retenues
gardent leur statut et la campagne n'est pas raccordée par cet enregistrement.

### ANIM-S14 — six attaques naturelles proposées

La revue 40 reprend exactement les deux corps validés en 64 px et propose
`29 / M01-GRIFFE-A01`, `30 / M01-LACERATION-A01`, `31 / M01-BOND-A01`,
`32 / TR61-COUP-A01`, `33 / TR61-ECRASEMENT-A01` et `34 / TR61-ONDE-A01`.
Les griffures à quatre tracés sont peintes en acier bleuté et ivoire,
avec quelques reflets ambrés. Les impacts lourds utilisent des éclats
ivoire et ardoise ; leur cercle s'étend brièvement pour l'Onde.
Le sang des blessures a ses propres gouttes cramoisies peintes. Les trois
sources imagegen, leurs prompts et leurs atlases sont conservés sans
repeindre les images. Une première capture au mauvais réglage de teinte
occulte reste archivée ; ce réglage est retiré de la proposition finale.

Le dessin du corps accompagne les frappes par un mouvement bref, sans
rotation de la silhouette frontale. Bond conserve son trajet libre de
deux cases, sans ajout de dégâts : la position logique change dès le
lancement et un saut compact de 0,26 seconde accompagne le déplacement.
Les griffures durent 0,30 seconde ; les impacts lourds de 0,30 à 0,40.
Ces cadences graphiques n'ajoutent pas de préparation mécanique.

Les blessures physiques projettent du sang sur les cibles touchées.
Un Saignement préparé laisse de petites gouttes sur sa victime pendant
trois tours ; l'assommement préparé a un repère ivoire vacillant pendant
un tour. Le temps réel termine les impacts mais ne diminue pas ces
statuts ou recharges. La prévisualisation clignote avant lancement et
disparaît à la confirmation. Les quatre directions, un obstacle partiel,
un front bloqué et l'attente sans tour sont proposés.

Ces six animations, leur sang de blessures et le repère d'assommement
attendent leur validation visuelle. Les impacts et succès sont préparés
dans un laboratoire isolé : aucun PV, dégât, énergie, hasard, IA, loot,
inventaire, sauvegarde ou changement de statistiques n'est calculé.
L'Onde ne repousse personne et le saut ne blesse aucun ennemi. Les
géométries de ces scènes de revue ne remplacent pas les futures règles
complètes de ciblage du jeu. Aucun raccordement à la campagne n'est ajouté.

### ANIM-S14 — six attaques et statuts visuels validés

Sur la revue 40 présentée, le joueur répond « tres bien :) ». Les candidats
29 à 34 sont validés : Griffe, Lacération, Bond, Coup lourd, Écrasement et
Onde de choc, avec le sang des blessures physiques, les petites gouttes du
Saignement et le repère d'assommement montrés. Les corps déjà acceptés
restent identiques. L'accord et les empreintes des éléments présentés sont
conservés dans `skill-animation-review-40/visual-acceptance-01.json` ; l'essai
interne à la mauvaise teinte est explicitement exclu. Les valeurs de combat
gardent leur statut et cet accord n'intègre pas le laboratoire à la campagne.

### ANIM-S15 — Cuirassé et Possédé proposés en 64 px

La revue 41 présente `35 / TR63-A01` Cuirassé et `36 / TR64-A01` Possédé.
Les nouveaux corps `TR63-BODY01` et `TR64-BODY01` reprennent les références
63 et 64 : chitinisation verte, joints violets et griffes ivoire pour le
Cuirassé ; chair rouge, jambes violettes et tête cornue ivoire pour le
Possédé. Ils proviennent directement de dessins BD haute définition, puis
d'un redimensionnement uniforme en 64 px, sans intermédiaire en 32 px.

Des plaques vertes et ivoire s'assemblent brièvement autour du Cuirassé ;
des rubans rouges bordés de violet accompagnent le Possédé. Le centre reste
libre pour lire le personnage. Entrée de 0,38 seconde et retour de 0,30,
formule de durée et recharge conservées : 22 tours au rang effectif 11 et
recharge 10 dans l'exemple. La forme est active dès le lancement ; attendre
ne diminue pas sa durée. Le renouvellement préparé au tour 10 restaure la
durée complète sans pose humaine intermédiaire. La prévisualisation clignote
avant confirmation et disparaît au lancement ; les murs coupent les VFX.

Les corps, leurs entrées, retours et renouvellements attendent la validation
du joueur. Les attaques propres seront proposées ensuite. La banque des
anciens corps reste intacte, ainsi que les rendus déjà validés. Le prototype
ne calcule pas les statistiques, PV, énergie, soins, réserve du Cuirassé,
dégâts, hasard, IA, inventaire, loot ou sauvegardes et reste isolé de la campagne.

### ANIM-S15 — validation des deux formes

Le joueur répond « ok je valide :) » après la revue 41. Les corps
`35 / TR63-BODY01` Cuirassé et `36 / TR64-BODY01` Possédé ainsi que les
transitions `TR63-A01` et `TR64-A01` montrées sont validés : entrée, retour,
renouvellement, quatre orientations de carte et découpe aux obstacles.
L'accord et les empreintes des fichiers sont conservés dans
`skill-animation-review-41/visual-acceptance-01.json`. L'essai interne de
palette reste exclu. Cet accord ne couvre pas les actions propres et
n'intègre pas les transformations à la campagne.

### ANIM-S16 — actions du Cuirassé et du Possédé proposées

La revue 42 présente six actions sur les corps validés, à taille native
64 px : `37 / TR63-HEURT-A01` Heurt, `38 / TR63-DECHARGE-A01` Décharge de
plaques, `39 / TR63-FERMETURE-A01` Fermeture des plaques,
`40 / TR64-TRAIT-A01` Trait corporel, `41 / TR64-PULSATION-A01` Pulsation
occulte et `42 / TR64-DRAIN-A01` Drain organique.

Heurt réutilise les dessins d'impact et de blessure physiques validés.
La décharge projette des plaques vertes et ivoire autour du corps ; une
scène prépare la réserve puis la vide une seule fois au lancement.
Fermeture propose une variante du dessin où les plaques recouvrent le
torse, avec un petit repère de plaque et un compteur de trois tours.
Attendre ne fait pas expirer la protection ; la scène « Se déplacer et
frapper » conserve les plaques fermées pendant un déplacement préparé au
premier tour et un Heurt au deuxième. Elles se rouvrent au troisième tour.

Trait corporel projette un trait rouge et noir bref vers sa cible, suivi
d'un impact local. Pulsation dessine une onde organique centrée sur le
Possédé. Drain réutilise le filament occulte épais validé et ramène son
flux vers le corps : effet bref, sans canalisation ni saignement ajouté.
Les deux sorts sont refusés dans la scène où le Possédé est réduit au
silence, tandis que son Trait ordinaire reste disponible. Cette scène ne
définit pas les autres interactions du Silence avec les transformations.

Les quatre directions, obstacles partiels, cloisons, attente, réserve,
déplacement sous protection et Silence sont montrés. La prévisualisation
clignote avant confirmation et disparaît au lancement ; les obstacles
coupent les effets. La géométrie préparée illustre les portées et rayons
acceptés, sans définir tous les cas de ciblage du jeu.

Les six compositions, la variante temporaire des plaques fermées et son
repère attendent l'accord visuel du joueur. Le laboratoire ne calcule pas
les PV, soins, énergie, dégâts, résistances, précision, hasard, IA,
inventaire, équipement, loot ou sauvegardes et reste isolé de la campagne.

### ANIM-REACTION01 — réaction commune aux compétences

Le joueur rappelle : « les effets negatifs qui touchent les monstres (ou meme
le joueur) doivent déclencher un petit mouvement d'aller retour », puis
précise : « ca doit concerner toutes les compétences d'ailleurs ».

Le petit aller-retour est une règle commune aux compétences qui touchent
une cible en lui infligeant des dégâts ou en lui appliquant un effet négatif.
Il doit être visible sur le personnage concerné, qu'il s'agisse du joueur
ou d'un monstre, y compris lorsqu'un effet négatif ne provoque pas de dégâts.
La réaction doit accompagner les effets propres de la compétence et les
repères des statuts déjà définis.

Le mouvement concerne uniquement le dessin : la position logique et les
collisions restent identiques et aucun délai ou tour supplémentaire n'est
ajouté. Le principe commun est demandé par le joueur ; l'amplitude, la
cadence et les cas d'interaction non présentés ne sont pas validés par ce
rappel. Les nouveaux rendus et les révisions de rendus restent soumis à
la procédure de revue. Les captures existantes ne sont pas modifiées par
cette inscription et leur validation ne devient pas une preuve de couverture
de cette nouvelle règle.

### ANIM-REACTION01 — raccordement du mécanisme commun

Le joueur demande ensuite de mettre en place cette règle et confirme son
accord visuel sur les deux formes Cuirassé et Possédé. L'accord de la revue
41 demeure inchangé ; les six actions de la revue 42 ne sont pas déduites
de cette confirmation des deux compétences de transformation.

Le client utilisait déjà une réaction aux dégâts. Le raccordement lit aussi
les applications effectives de statuts négatifs : malus de précision ou de
stabilité, fragilisation d'armure, entrave et effets qui blessent leur
porteur. La même réaction vaut pour le joueur et les autres acteurs, sans
liste de noms de compétences. Un effet ignoré ou bloqué ne crée pas de
réaction ; les protections et compteurs ne sont pas traités comme des
affaiblissements. Les cibles non visibles ne produisent pas de mouvement
susceptible de révéler leur présence. L'option de mouvements réduits conserve
son fonctionnement.

La courbe de réaction aux dégâts déjà présente en jeu est conservée :
extension rapide puis retour à l'ancrage sur 0,18 seconde, avec un maximum
de 17 % de la largeur de case. Le mouvement est calculé uniquement par le
client ; il ne modifie ni case, collision, tour, recharge, statut, PV ou RNG.
Les dégâts persistants continuent d'utiliser les événements de dégâts.

La revue 43 propose les variantes `37–42 / *-A02` des six actions, avec la
réaction des victimes attachée aux impacts et leurs effets visuels. Elle
ajoute quatre vues isolées `RX01-DMG-M-R01`, `RX01-DMG-J-R01`,
`RX01-STATUS-M-R01` et `RX01-STATUS-J-R01` pour montrer les dégâts et les
effets négatifs sans dégâts, respectivement sur un monstre et le joueur.
Deux contacts y sont préparés ; elles ne calculent pas les PV ni les
statuts réels. Les dessins et les anciens fichiers de revue sont conservés.
Les nouvelles compositions restent à examiner. Les compétences de combat
de la refonte ne sont pas rendues fonctionnelles par ce raccordement visuel.

### ANIM-REACTION01 — compositions de la revue 43 validées

Le joueur répond « parfait on peut continuer » devant
`skill-animation-review-43/cuirasse-possede-actions-64.gif`. Cet accord
valide les six compositions `37–42 / *-A02`, le corps temporaire aux plaques
fermées, son repère de protection et la réaction des victimes montrée.
Les quatre cas isolés de dégâts et d’effet négatif sur joueur et monstre
gardent leur place dans cette revue. L’acceptation est consignée dans
`skill-animation-review-43/visual-acceptance-01.json`, avec les empreintes
des fichiers finaux ; les essais de masques et de corrections sont exclus.
L’accord ne modifie pas les valeurs mécaniques précédemment validées et
n’implémente pas les six compétences dans le combat de la campagne.

### ANIM-S17 — Abysse, corps et métamorphose proposés

La revue 44 propose `43 / TR65-BODY02` et `TR65-A01`. Le corps reprend
l’identité de la référence 65 : œil unique ivoire à pupille verticale,
torse creux, épaules enroulées et bras allongés. Après la première
proposition, le joueur souhaite éviter le bleu cyan. BODY02 propose donc
de la chair violette sombre et des crêtes organiques ivoire, sans cyan.
Cette palette et ce nouveau dessin attendent leur accord visuel propre.
La proposition cyan BODY01 reste conservée comme essai écarté.

Les ombres dessinées accompagnent l’entrée en 0,38 seconde, le retour
en 0,30 seconde et le renouvellement. Les quatre orientations de carte
gardent le corps frontal ; les obstacles coupent les effets, et l’aperçu
clignotant disparaît au lancement. La forme conserve l’exemple accepté
de 22 tours et 10 tours de recharge : l’attente seule ne les diminue pas,
et un renouvellement prêt remet la durée complète sans passage humain.
L’entrée et le retour ne sont pas des blessures : aucune réaction de
dégâts ni projection de sang n’est ajoutée à cette métamorphose.

Ce lot présente le corps et les transitions uniquement. Allonge, Fente
abyssale et Pas d’ombre seront examinés ensuite. Aucune statistique,
énergie, IA, règle de PV, soin, dégât, loot ou sauvegarde n’est calculé
par ce prototype, qui ne remplace pas les dessins de campagne.

### ANIM-S17 — Abysse sans cyan validé

Le joueur répond « oui ca me plait bien mieux comme ca :) » après la revue 44.
Le corps `43 / TR65-BODY02`, sa palette violette et ivoire, ainsi que les
transitions `TR65-A01` montrées sont acceptés. Le reçu
`skill-animation-review-44/visual-acceptance-01.json` conserve les empreintes
du corps et des captures ; la proposition cyan reste exclue.
Les actions propres ne font pas partie de cet accord.

### ANIM-S18 — trois actions d’Abysse proposées

La revue 45 propose `44 / TR65-ALLONGE-A01`, `45 / TR65-FENTE-A01` et
`46 / TR65-PAS-A01`, avec le corps exact BODY02 accepté. Deux nouvelles
planches imagegen intégré fournissent le membre extensible violet et ivoire
d’Allonge, puis la lacération sombre de Fente. Pas d’ombre réutilise les
ombres dessinées de la revue 44 et une rémanence translucide du corps au
départ ; le personnage apparaît au point d’arrivée, sans traversée animée
du mur. Les trois nouvelles compositions attendent leur validation.

Les attaques naturelles reprennent leurs portées affichées de deux et
quatre cases, leur recharge de zéro et quatre tours. Pas d’ombre reprend
la portée de trois cases et la recharge de trois tours, une destination
libre, visible et voisine d’un obstacle opaque ; Silence le bloque sans
bloquer les deux attaques naturelles. Les cas du prototype sont axiaux :
ils ne définissent pas les diagonales ou une métrique générale de jeu.
Les aperçus clignotent avant la validation et disparaissent au lancement.
Les victimes des attaques réagissent avec le mécanisme commun de dégâts,
et les blessures physiques sont accompagnées de sang sans nouveau statut
Saignement implicite. Pas d’ombre ne provoque pas d’impact sur les voisins.

Les scènes montrent quatre orientations, une cloison, une ligne coupée
après deux cases, l’attente sans tour, Silence et des destinations refusées.
La scène de franchissement fournit explicitement une destination déjà
repérée : la visibilité est préparée, sans calcul de perception réelle
ou nouvelle capacité à voir au travers des murs. La scène cachée ne montre
ni acteur ni destination proposée dans les cases non visibles. Les ombres
disponibles peuvent être sélectionnées dans le laboratoire natif.

Ce lot reste un prototype de présentation : aucun dégât chiffré, PV,
énergie, IA, attribut, loot ou sauvegarde n’est résolu par ces scènes.
Il ne remplace pas les fichiers des lots précédents et ne raccorde pas
ces trois actions au combat de la campagne.

### ANIM-S18 — actions d’Abysse validées

Le joueur ajoute : « note que je valide la compétence qu'on vient de créer :)
elle est tres bien ». L’accord porte sur Abysse et ses trois actions montrées
dans la revue 45 : `44 / TR65-ALLONGE-A01`, `45 / TR65-FENTE-A01` et
`46 / TR65-PAS-A01`. L’acceptation et les empreintes des fichiers sont
consignées dans `skill-animation-review-45/visual-acceptance-01.json`.
Les valeurs gardent leurs accords précédents ; les trois actions ne sont
pas intégrées au combat de la campagne par cet enregistrement.

Le joueur réaffirme cet accord après la validation de la recoloration du
Possédé : « ok et je valide la compétence que tu as créé ».
La confirmation est conservée dans la revue 45, `confirmation-02.json`,
en référence à l'accord ANIM-S18 existant, sans élargir sa portée.

### ANIM-TR64-COLOR01 — Possédé couleur chair, proposition

Le joueur souhaite également retirer le violet du corps du Possédé présenté
dans la revue 41. Un premier essai `36 / TR64-BODY02` remplace ces parties
par du gris charbon ; le joueur demande plutôt « de la couleur chaire sur
son corps pour voir ». L'essai `36 / TR64-BODY03` conserve le rouge du
torse et des avant-bras et colore seulement les anciennes parties violettes
en chair. Le joueur précise ensuite « non mais sur les 2 parties de son
corps :) ». La revue 46 propose donc `36 / TR64-BODY04` : teintes chair
beiges et brunes sur le torse, les bras et les jambes, crâne et plaques
ivoire, yeux rouges conservés. La source est retouchée avec imagegen intégré,
puis exportée directement en 64 px dans les repères du corps validé.

La comparaison conserve la taille, la pose frontale et les repères du corps.
Elle montre l’avant et la proposition sur fonds sombre, clair et sol du jeu,
ainsi qu’une capture native auprès des autres acteurs. Les essais gris et
chair limitée au bas du corps sont conservés comme variantes écartées.
BODY04 attend sa validation propre :
aucune animation, action, statistique ou sauvegarde n’est modifiée, et le
nouveau corps ne remplace pas encore le dessin validé dans la campagne.

### ANIM-TR64-COLOR02 — Possédé, muscles rouges et tendons chair

Le joueur demande ensuite de colorer les muscles en rouge et les tendons
en couleur chair. Le candidat `36 / TR64-BODY05`, présenté dans la revue 47,
reprend le corps frontal de BODY04 avec cette distinction sur le torse,
les bras et les jambes. Les os restent ivoire et les yeux rouges.
La retouche est réalisée par imagegen intégré ; l'export direct 64 px
conserve l'enveloppe occupée et l'ancrage des pieds. La comparaison montre
BODY04 et BODY05 à taille native et en zoom ×3 explicitement indiqué.

Le 13 octobre 2026, le joueur confirme : « oui là je valide cette
transformation ». Le corps `36 / TR64-BODY05` devient l'apparence de
référence acceptée du Possédé : muscles rouges, tendons chair, os ivoire
et yeux rouges. L'accord sur la source et son export direct 64 px est
consigné dans `skill-animation-review-47/visual-acceptance-01.json`, avec
les empreintes des dessins et aperçus. Les variantes précédentes restent
conservées. Les règles et animations gardent leurs validations propres ;
cet enregistrement ne remplace pas les ressources actives de la campagne.

### BESTIARY-64-PLAN01 — adaptation et extension après les compétences

Le joueur demande de terminer les compétences et leurs animations avant
de reprendre le bestiaire. Il confirme ensuite l'adaptation des monstres
existants et la création d'un très grand nombre de nouveaux monstres.
Les PNJ existants restent également à contrôler et à adapter au rendu
64 px. Le rendu actif des monstres ordinaires utilise encore les sprites
32 px agrandis ; le choix des exports directs 64 px pour le joueur n'a pas
migré automatiquement ces banques. Cette limite vaut aussi pour les
ennemis et les drones utilisés comme cibles dans les aperçus d'animations.

L'adaptation devra conserver les identités et les tailles relatives,
avec des dessins cohérents avec les personnages, puis des candidats
numérotés soumis au joueur avant intégration. Cet ordre de travail
ne fixe ni nouveau bestiaire, ni statistiques, ni règles d'apparition.

### ANIM-S19 — Décharge mutagène et Autodestruction proposées

La revue 48 présente `47 / M02-A01` et `48 / HY07-A01` dans des cases
64 px, avec un laboratoire natif isolé. Deux nouvelles planches imagegen
fournissent un projectile organique vert et son impact bref. Décharge
mutagène est montrée humain et dans les cinq formes, avec le Possédé
BODY05 accepté ; une pulsation verte brève accompagne la prolongation
admissible. Les cas préparés sans PV retirés et au plafond ne prolongent
pas la forme. Les valeurs affichées déjà acceptées restent conservées.

Autodestruction rougit brièvement le cœur du drone, puis montre une
explosion de feu dessinée et des fragments métalliques. Elle réutilise
le feu accepté de la revue 20. Traqueur, Artilleur et Gardien sont montrés,
ainsi que les refus pour Éclaireur, Mule, drone caché, Silence ou forme
transformée. Les cibles ennemies touchées font la réaction commune ;
le joueur et l'autre drone voisin ne sont pas touchés. Les murs coupent
le rendu et l'empreinte préparée. Aucun statut Brûlure, Poison ou Saignement
n'est ajouté à ces deux compositions.

Les effets sont brefs ; les durées et recharges nécessitent les tours
manuels. Les aperçus disparaissent au lancement. Les positions, portées
et empreintes sont des cas préparés ; ils ne fixent pas de nouvelle
métrique générale ou de diagonales. Les deux rendus attendent l'accord
du joueur. Aucun combat, PV, énergie, IA, loot ou sauvegarde de campagne
n'est modifié par ce lot visuel.

### ANIM-S19 — deux compositions validées

Le 13 octobre 2026, le joueur répond « je valide » à la revue 48.
L'accord visuel porte sur `47 / M02-A01` Décharge mutagène et
`48 / HY07-A01` Autodestruction, avec les directions et les cas montrés.
Le projectile et l'impact verts, la pulsation des cinq formes, le cœur
rougissant du drone, l'explosion et les fragments deviennent leurs
références acceptées. L'accord et les empreintes des dessins, captures
et animations figurent dans `skill-animation-review-48/visual-acceptance-01.json`.
Les règles chiffrées conservent leurs accords précédents. La campagne
n'est pas modifiée par cet enregistrement, et les monstres et drones
provisoires restent à adapter selon BESTIARY-64-PLAN01.

### ANIM-S20 — Échange de position et Lien de domination proposés

La revue 49 propose `49 / HY04-A01` et `50 / HY06-A01`, dans un
laboratoire natif isolé, avec des cases 64 px. Deux nouveaux dessins
imagegen intégré fournissent six nuages de fragments sombres et un
symbole occulte violet. Les particules accompagnent une disparition
et une reformation rapides de l'agent et du drone aux deux positions,
sans trajet entre elles. La cargaison préparée de la Mule reste dans
le drone. Les valeurs affichées conservent leur accord précédent.

Le symbole de Domination reste droit au-dessus du monstre, dans les
quatre orientations de scène ; il pulse légèrement pendant les quatre
tours du statut et suit la réaction commune de la victime. Aucun
collier ni fil de contrôle n'est ajouté. Les cas préparés montrent
l'attente sans tour, le maintien après des dégâts, l'expiration et
le remplacement d'un lien antérieur. La liste de cibles du drone
retire l'allié dominé et retrouve l'ancien porteur lors du remplacement.
Il s'agit d'une liste préparée, sans IA ni attaque de campagne simulée.

Les scènes montrent les murs proches des deux positions d'échange et
une cloison qui bloque le trajet de Domination, sans définir le
franchissement d'obstacles intermédiaires pour Échange de position.
Les refus représentent les conditions déjà affichées ; les scènes
de boss ne déterminent ni coût d'un contrôle raté ni comptabilité
de ses ressources. Les destinations illégales et les distances sont
des cas préparés ; aucune nouvelle métrique générale n'est fixée.

Les transitions sont brèves, les aperçus disparaissent au lancement
et les compteurs n'avancent qu'avec les tours manuels. Les deux
compositions attendent la validation du joueur. Aucun combat, PV,
énergie, cargaison, RNG, IA, loot ou sauvegarde de campagne n'est
modifié ; les monstres et drones gardent provisoirement leurs dessins.

### ANIM-S20 — Échange de position et Domination validés

Le 13 octobre 2026, le joueur répond « parfait je valide :) » à la revue 49.
Les compositions `49 / HY04-A01` et `50 / HY06-A01` deviennent leurs
références visuelles acceptées : disparition et reformation des deux
silhouettes en fragments sombres, symbole occulte violet animé au-dessus
de la cible dominée, réactions et cas présentés. La preuve et les
empreintes des dessins, captures et animations sont conservées dans
`skill-animation-review-49/visual-acceptance-01.json`.
Les règles gardent leurs accords précédents ; aucun nouveau coût
d'échec, franchissement d'obstacle, IA ou raccordement à la campagne
n'est introduit par cette validation visuelle.

### ANIM-S21 — Buffs de mêlée proposés

La revue 50 présente `51 / BM01-A01` Lame incendiaire et `52 / BM02-A01`
Faim vampirique. Les icônes compactes acceptées persistent dans la barre
des buffs, avec leur timer en tours et le survol nom/effet/durée restante.
Lame incendiaire conserve l'arme visible : les dégâts de feu ajoutés se
lisent dans leur composante colorée et le journal, sans nouvelle flamme
sur l'arme ni Brûlure ajoutée implicitement.

Pour Faim vampirique, une planche imagegen à six dessins montre deux
mâchoires aux gencives rouges et dents ivoire. Elles apparaissent autour
de la cible, se ferment rapidement et disparaissent après 0,40 s. Le soin
flotte au-dessus du joueur. Le monstre garde la réaction commune et le
sang du coup physique. Aucun visage ni nouvelle silhouette de porteur
n'est ajouté. La bouche reste droite dans les quatre directions.

Le laboratoire natif permet lancement, frappe et tours manuels ; la
galerie sépare l'attente réelle des tours. Les exemples de dégâts et soin
sont préparés pour la lecture, sans nouveaux barèmes ni intégration au
combat. Les refus de lancement et de frappe, le survol et l'expiration
sont montrés. Les déclenchements en cas de miss, absorption, soin nul ou
plafond atteint restent à discuter. Ces compositions attendent la revue
visuelle du joueur ; les monstres actuels restent provisoires.

### ANIM-S21 — validation et navigation des galeries

Le 13 octobre 2026, « ok ca me va :) je valide » accepte Lame incendiaire
`51 / BM01-A01` et Faim vampirique `52 / BM02-A01`, avec les compositions
et cas présentés. L'accord est consigné dans la revue 50,
`visual-acceptance-01.json`. Aucun effet n'est raccordé à la campagne.

Les comparaisons de deux animations doivent ouvrir une page HTML qui
fonctionne directement dans le navigateur. Un lien .ps1 affiche son
texte et ne constitue pas un aperçu exécutable. La revue 50 propose
`cote-a-cote.html`, adapté à la fenêtre, avec des options de taille
native et de plein écran. Les animations validées restent inchangées.

### ANIM-S22 — Charges et renvoi magique proposés

`53 / BM03-A01` Tempo offensif et `54 / BM05-A01` Sceau de vindicte
réutilisent les icônes compactes, le coup physique et le projectile occulte
déjà acceptés. Aucun nouveau corps, arme portée ou dessin raster n'est créé.
Tempo compte ses charges dans la barre, avec le bonus au survol ; sa série
conditionnelle n'a pas de timer. Le renvoi reprend littéralement le même
projectile et la même taille dans la direction opposée. Le violet franc
accepté s'applique aux effets seulement, sans recolorer les acteurs.

Les coups et les renvois finissent rapidement ; l'attente ne rejoue pas
leurs impacts. Les blessures physiques gardent le sang accepté ; la magie
reste sans sang. La réaction commune concerne l'agent et le lanceur touché.
La galerie 51 propose sa vue côte à côte directement dans le navigateur,
avec taille native 64 px et plein écran. Les compositions et cas présentés
sont validés le 13 octobre 2026 ; les acteurs ennemis actuels restent provisoires.

### ANIM-S22 — validation des références visuelles

Le joueur accepte explicitement « je valide :) » pour Tempo offensif
`53 / BM03-A01` et Sceau de vindicte `54 / BM05-A01`, dans la comparaison
HTML de la revue 51. La barre de charges, le renvoi du projectile, les
réactions et les cas montrés sont figés dans `visual-acceptance-01.json`.
Le brouillon `native-glyph-draft` est exclu de l'accord. Les règles
chiffrées gardent leurs validations distinctes ; aucun effet n'est
raccordé au combat de campagne par cette validation artistique.

### ANIM-S23 — Visée et récupération proposées

La revue 52 présente `55 / BD04-A01` et `56 / BD06-A01` en cases natives
64 px. Le viseur rouge à contours sombres se resserre avec les cumuls ;
l'icône et son survol montrent le bonus de précision sans timer artificiel.
L'énergie récupérée rejoint sa jauge sous forme d'une étincelle violette
épaisse, avec son gain chiffré. Cette surcouche d'interface ne crée aucun
dégât hors des cases et aucun buff temporisé.

Les personnages, poses, armes, corps et icônes existants restent intacts.
Les impacts physiques réutilisent le sang accepté et la réaction commune.
Les nouvelles formes simples sont dessinées par le moteur, avec contours
et aplats ; aucun bitmap de personnage ou d'équipement n'est redessiné.
Les quatre directions et cas de compteurs sont proposés dans la comparaison
HTML, avec taille native et plein écran. Les compositions sont validées
le 13 octobre 2026 ; les PNJ provisoires seront adaptés après les animations.

### ANIM-S23 — références visuelles validées

Le joueur accepte explicitement « je valide :) » pour Visée stable
`55 / BD04-A01` et Récupération d’énergie `56 / BD06-A01`, dans la comparaison
HTML de la revue 52. Le viseur, les cumuls, l'étincelle de récupération,
les réactions et les cas montrés sont figés dans `visual-acceptance-01.json`.
Les brouillons `native-feedback-draft` et `native-menu-draft` sont exclus
de l'accord. Les règles chiffrées gardent leur statut distinct ; aucun
effet de combat de campagne n'est intégré par cette validation visuelle.

### ANIM-S24 — Marquage et maîtrise proposés

La revue 53 présente `57 / T01-A01` et `58 / T03-A01` en cases natives
64 px. Le viseur rouge se referme sur la cible ; sa marque garde son
indicateur de durée dans la barre de buffs et la bande de statuts préparée
de la cible. Le tir consommant la marque fait disparaître ces indicateurs.
L'application de la marque déclenche également la réaction hostile commune
sur la cible, indépendamment des blessures d'un tir ultérieur.
La maîtrise donne un cœur plus vif aux projectiles et montre son bonus
dans le total des dégâts, sans inventer de statut temporaire.

Les dessins de corps, armes, poses, icônes, sang et transformation utilisés
sont réutilisés sans retouche raster. Les cibles touchées gardent le
mouvement d'aller-retour commun ; le tir manqué ne crée pas de blessure.
Le drone du cas d'exclusion conserve son point vert d'allié. Les formes
simples, contours et aplats suivent la revue 52 déjà acceptée.
La comparaison HTML propose les quatre directions et les cas préparés,
avec taille native et plein écran. Les deux compositions restent à valider.

### ANIM-S24-A02 — Repère persistant de Marquage balistique

À la demande du joueur le 14 octobre 2026, `57 / T01-A02` ajoute l'icône
compacte de Marquage au-dessus de la tête de la cible. Le repère reste lisible
sur un fond sombre de 26 px et affiche le dessin existant en 24 px. Il suit
la cible pendant la réaction commune et reste présent entre les actions.
Il disparaît avec la marque et change de porteur lors du remplacement.

Cette icône est une indication de statut superposée au décor, distincte des
animations de dégâts découpées par les obstacles. Aucun corps, arme, icône
source ou règle chiffrée n'est modifié. La galerie 54 conserve T03-A01 à
l'identique et préserve la galerie 53. Le nouveau rendu reste soumis à la revue.


## ANIM-S27 — Déflagration amplifiée et Drain d’impact

La revue 57 propose **63 / HL05-A01** et **64 / HL06-A01**, dans une
comparaison HTML animée et dans le laboratoire natif. Les cases font 64 px.
Les deux compositions attendent une validation visuelle explicite.

Déflagration amplifiée reprend le projectile et l’explosion peinte acceptés.
Au rang 11, le rayon d’exemple passe de une à quatre cases, sans multiplier
les dégâts. Les murs coupent le dessin et ses effets ; une arme sans explosion
conserve son tir ordinaire. Ce passif ne crée ni buff temporaire ni lancement
séparé, et ne modifie pas la zone de ciblage de Déluge ou le rayon de contrôle
d’Impact assommant.

Drain d’impact reprend l’impact habituel, le sang physique et la réaction
commune des cibles blessées. Une brève impulsion verte entoure le joueur et
le gain « +X PV » apparaît au moment du soin. Le retour vient uniquement des
PV réellement retirés à la cible principale, avec le plafond par utilisation
et la limite des PV manquants déjà acceptés. L’impulsion se termine d’elle-même ;
aucun statut temporisé n’est ajouté à la barre de buffs.

Les scènes préparées montrent les quatre directions, une cloison, l’attente
sans action, les armes exclues ou sans explosion, les refus et les limites de
soin. Le cas à retardement attend une action avant l’explosion : ce délai d’un
tour sert d’exemple, sans fixer celui des futures familles d’armes. Le cas des
PV pleins montre un retour nul et ne tranche pas l’autorisation réelle du
lancement à vie pleine. Les montants, protections et acteurs restent préparés ;
aucune courbe réelle, règle d’arrondi ou priorité de résolution n’est ajoutée.

Entrée native : `--apercu-laboratoire-competences --lot-armes-lourdes-finale`.
Comparaison : `skill-animation-review-57/cote-a-cote.html`.
Les aperçus acceptés de la revue 56 restent intacts. Les monstres temporaires
seront adaptés après les animations. Aucun effet n’est raccordé au combat de
campagne par ce prototype.


## ANIM-S28 — Furtivité et Économie de mouvement furtif

La revue 58 propose **65 / SF00-A01** et **66 / SF01-A01**, en cases natives
64 px. Les nouvelles compositions attendent leur validation visuelle.

Le camouflage réutilise les couches exactes du personnage et de son arme.
Un bref balayage bleu acier accompagne leur passage à une légère transparence.
L’apparence furtive et l’icône SF00 restent présentes entre les actions, sans
durée artificielle. La transition visuelle ne retarde pas l’état du mode.
Le passif SF01 modifie le coût affiché et les dépenses des pas ; il ne crée
aucune activation supplémentaire ni seconde icône de buff temporisé.

Les deux panneaux comparent onze énergie par pas sans SF01 à une énergie au
rang dix. Les cas préparés montrent aussi le rang neuf et le minimum avec
un rang effectif douze. À zéro énergie, le pas demandé retire le camouflage
et est effectué normalement, conformément à la règle acceptée.

Les quatre directions gardent les personnages de face. L’attente, le menu,
un pas bloqué, un soin fourni et une action offensive préparée montrent la
conservation ou la levée du mode. Le soin de cinq PV est un événement de revue,
pas une nouvelle compétence ; l’action offensive lève le mode sans exécuter
une attaque. Aucun bruit, cône de détection, effacement de mémoire ou nouvelle
IA n’est simulé. Le survol reprend uniquement l’exemple accepté « Distance de
repérage : 2 cases ». SF02 n’est pas ajouté implicitement à ces profils.

Le cas d’une réserve positive inférieure au coût, celui de zéro énergie devant
un obstacle et l’ordre réel des dépenses, tours et régénération restent à
traiter séparément. Les compteurs sont préparés pour la revue. Les modèles
acceptés et les galeries précédentes restent intacts.

Entrée native : `--apercu-laboratoire-competences --lot-furtivite-economie`.
Comparaison : `skill-animation-review-58/cote-a-cote.html`.
Aucun effet n’est raccordé au combat ou à la perception de la campagne.


## ANIM-S29 — Attaque sournoise et Discrétion renforcée

La revue 59 propose **65 / SF00-A02**, révision du rendu de Furtivité, et
**67 / SF02-A01** Discrétion renforcée. Le camouflage accepté dans la revue 58
reste intact : mêmes couches, balayage bleu acier, opacité et retour à
l’apparence normale. SF01 conserve sa propre référence acceptée.

Le bonus de cinq pour cent par rang a été accepté sous SF00-SNEAK01-ACCEPT01.
L’aperçu de rang dix prend une base préparée de vingt dégâts : l’attaque
lancée en furtivité affiche trente, dont dix de bonus. Le mode et son icône
prennent fin au lancement ; le projectile conserve ce bonus jusqu’à l’impact.
L’attaque suivante à découvert affiche vingt. Une réactivation préparée de
Furtivité permet ensuite de montrer une nouvelle attaque renforcée.
Les rangs onze et douze montrent respectivement trente-et-un et trente-deux.
Ces nombres ne définissent aucune nouvelle statistique ou variance réelle.

Les tirs et frappes de mêlée reprennent les impacts physiques et le sang
acceptés. L’exemple de sort direct de feu reprend les dessins de projectile
et d’explosion acceptés, sans sang. Il illustre seulement un impact magique :
la zone, la Brûlure, le coût et la recharge complets de Boule de feu ne sont
pas simulés par cette scène. Tous les impacts déclenchent la réaction commune
sur la cible, sans déplacement logique de sa case. Les nombres montrent le
total, le bonus et la composante physique ou de feu avec son petit symbole.

Discrétion renforcée, au rang effectif onze, reprend l’exemple accepté d’une
case de repérage, contre deux sans ce passif. Le libellé est « Distance de
repérage ». La valeur est mise à jour dans le survol de l’icône SF00 ; le passif
ne crée ni activation distincte ni seconde icône de buff temporisé.
Les deux profils de revue possèdent Économie au rang dix : un pas préparé
coûte donc une énergie. Le rang de Furtivité et son bonus restent distincts
du rang de Discrétion renforcée et de sa distance.

La comparaison montre les quatre directions, attente sans tour, mêlée, sort,
attaque à découvert, rangs supplémentaires, déplacement avant l’attaque et
cloison. La scène bloquée rend l’attaque indisponible dans le prototype ; elle
ne tranche pas la règle générale des attaques refusées. Aucun calcul d’IA,
écoute, visibilité, mémoire ou réaction ennemie n’est ajouté. Les rafales,
zones, canalisations, dégâts différés, critiques et ordres de modificateurs
restent à raccorder au résolveur réel. Les compositions nouvelles attendent
leur validation ; aucune nouvelle règle chiffrée n’est déduite des exemples.

Entrée native : `--apercu-laboratoire-competences --lot-furtivite-sournoise-discretion`.
Comparaison : `skill-animation-review-59/cote-a-cote.html`.
Le dossier `native-label-draft` reste un brouillon non présenté, exclu de
l’accord à venir. Les monstres provisoires seront adaptés après les animations.
Aucun effet n’est intégré au combat de campagne par cette galerie.

### ANIM-S29 — attaque sournoise et Discrétion renforcée validées

Le joueur répond « parfait on continue » devant la comparaison de revue 59.
L’accord visuel porte sur 65 `SF00-A02` et 67 `SF02-A01`, avec les scènes
montrées. Le camouflage antérieur reste conservé ; les chiffres de l’attaque
sournoise gardent leur accord distinct `SF00-SNEAK01-ACCEPT01`.
Décision et empreintes : `skill-animation-review-59/visual-acceptance-01.json`.
Le brouillon `native-label-draft` et la copie HTML antérieure à la réparation
de navigation sont exclus de cet accord. Les cas de combat non simulés et
l’intégration à la campagne restent distincts.

### ANIM-S30 — pose et réseau de mines, nouveaux candidats

La revue 60 propose 68 `SM00-A01` Pose de mine et 69 `SM01-A01` Réseau de
mines. Nouveau boîtier métallique au sol, dérivé par Imagegen intégré de
l’icône SM00 déjà acceptée ; source, prompt et export transparent conservés.
Les explosions reprennent le dessin de feu accepté dans la revue 20.

Pendant la préparation, le joueur demande explicitement que la LED centrale
clignote « une fois qu’elle est posée ». Le voyant clignote donc dès la pose,
y compris avant l’armement, avec une proposition de cycle de trois quarts
de seconde. Cette nouvelle direction visuelle remplace uniquement l’ancien
début de clignotement à l’armement ; le délai mécanique d’un tour reste
conservé. Attendre anime le voyant sans armer la mine ni avancer le tour.

La comparaison montre pose et aperçu clignotant retiré à la confirmation,
armement, approche et contact ennemi, explosion brève, petit aller-retour
des victimes et dégâts affichés. Le survol de la mine centrale montre son
rayon. La capacité dans la fiche de pose passe d’une mine à cinq avec SM01
au rang effectif onze ou douze, sans nouveau buff temporisé. Le scénario de
capacité fournit des mines antérieures et montre le remplacement de la plus
ancienne ; le changement de couche retire les mines sans explosion.

Les cas ont des valeurs préparées : treize dégâts de feu avant défenses,
dix énergie, recharge et armement d’un tour. Le joueur voisin reste protégé.
Pas de sang, de Brûlure ni de résidu ajouté dans ce lot. L’exemple cardinal
de rayon un ne fixe pas la géométrie complète des coins et grandes cibles.
Courbes au-delà de douze, dégâts aux familiers, réactions simultanées et
inspection des objets superposés restent des raccordements distincts.

Tous ces nouveaux rendus restent à valider dans
`skill-animation-review-60/cote-a-cote.html` avant leur intégration au combat.
Le laboratoire natif les ouvre avec `--lot-mines-ouverture`. Monstres
provisoires conservés ; adaptation prévue après les animations.

### ANIM-S30 — pose et réseau validés, LED selon le type de mine

Le joueur répond « ca me va bien :) » devant la revue 60. L’accord visuel
porte sur 68 `SM00-A01` et 69 `SM01-A01`, tels que montrés. Le dessin de
mine, son voyant central clignotant dès la pose, l’explosion, le survol et
l’affichage de capacité sont conservés. Décision et empreintes dans
`skill-animation-review-60/visual-acceptance-01.json` ; le combat de campagne
et les cas mécaniques non simulés restent une étape distincte.

Le joueur demande également des couleurs de LED différentes du rouge pour
les mines de poison, de froid ou d’un autre type. La direction acceptée est
une couleur correspondant au type de chaque mine, avec le même clignotement
dès la pose et le délai d’armement conservé. Le rouge du Feu déjà présenté
reste la référence validée. Vert pour Poison et bleu glacé pour Froid sont
des propositions de palette à montrer dans les prochains aperçus ; leur
teinte précise n’est pas validée par cet accord. Aucun nouveau type de mine
ni effet mécanique n’est créé par cette règle de couleur.

### MINE-LED-TYPE01 — correspondance des couleurs acceptée

Le joueur répond « parfait :) » à la proposition : Feu rouge, Poison vert,
Gel bleu glacé, avec clignotement dès la pose. Cette correspondance des
couleurs est désormais acceptée, sous `MINE-LED-TYPE01-ACCEPT02` dans
`mine-led-type-color-palette-acceptance-01.json`. Les prochains aperçus de
Poison et de Froid appliqueront cette règle ; leurs nouveaux dessins et
rendus animés restent à présenter avant intégration. Le lot Feu déjà accepté
reste conservé, ainsi que le délai mécanique d’armement.

### MINE-VFX-TYPE01 — explosion élémentaire et résidu fixe

Le joueur demande également d’adapter l’explosion au type de dégâts de la
mine et à la présence de l’amélioration qui laisse un nuage statique.
L’impact bref doit donc évoquer Feu, Poison ou Froid selon la mine utilisée.
Avec Résidu persistant (SM05), il laisse un nuage ou résidu visuel adapté
au même type, fixe sur la zone au sol pendant sa durée prévue. Sans SM05,
aucun nuage de terrain persistant n’est ajouté par cette direction visuelle.

La zone conserve le rayon, la durée et la prise en compte des obstacles déjà
définis. Les statuts Empoisonné et Gel sur les victimes restent distincts
du nuage au sol : ils ne sont pas rendus dépendants de SM05. Aucun nouveau
type de dégâts ni règle chiffrée n’est créé. Les prochains lots doivent
montrer les explosions adaptées et les variantes avec ou sans résidu avant
intégration ; les nouveaux dessins ne sont pas validés par cette demande.
Décision : `mine-explosion-type-and-residue-visual-direction-decisions.json`.

### MINE-VFX-TYPE02 — formes de Résidu persistant précisées

Le joueur précise les trois rendus de SM05 : sol enflammé pour Feu,
nuage de gaz/poison pour Poison, sol gelé pour Froid. Ces zones restent
fixes à leur emplacement ; seul le résidu de Poison prend la forme d’un
nuage de gaz. Cette précision remplace toute lecture de la direction
précédente qui donnerait un nuage à toutes les variantes. Les durées,
rayons, dégâts, statuts et obstacles déjà définis restent conservés.
Les trois nouveaux rendus seront présentés dans le lot de Résidu persistant
avant intégration. Décision :
`mine-residue-elemental-ground-visual-direction-decisions.json`.

### ANIM-S31 — Mines de poison et Mines de glace proposées

La revue 61 présente 70 `SM02-A01` et 71 `SM03-A01`. Le boîtier et ses
points de placement restent ceux de la mine acceptée en revue 60 ; seul le
voyant central change de couleur au rendu, vert ou bleu glacé. Il conserve
le clignotement dès la pose. Nouvelle détonation toxique peinte produite
avec Imagegen intégré ; les éclats de glace et l’enveloppe de Gel reprennent
les ressources acceptées des revues 26 et 28.

Les explosions restent brèves, coupées par les murs, sans sang. Après le
premier impact, de petites vapeurs de poison suivent les victimes ; le Gel
apparaît devant leur corps. Repères de statut et compteurs restent visibles.
Le prototype montre l’empoisonnement après éloignement, le déplacement
refusé sous Gel et un échec de Gel qui conserve les dégâts de froid. Les
réussites sont préparées, sans jet ni changement des chances acceptées.

Le premier poison est immédiat et compte dans les cinq applications, avec
quatre applications suivantes. Attendre anime le statut sans modifier ses
compteurs ni répéter des dégâts. Les ennemis ont quatre cents PV préparés
pour montrer toute la séquence ; ce chiffre ne fixe pas le bestiaire.
Les phases exactes d’expiration restent à raccorder au moteur de campagne.

SM05 est absent de ce lot : aucun résidu fixe de terrain n’y est ajouté.
Le prochain lot dédié montrera sol enflammé, gaz toxique et sol gelé selon
la dernière précision `MINE-VFX-TYPE02`. Les nouveaux rendus Poison et
les palettes natives attendent leur validation dans
`skill-animation-review-61/cote-a-cote.html`. Laboratoire isolé :
`--lot-mines-variantes`, sans intégration au combat de campagne.

### ANIM-S31-REV02 — explosions reprises à la demande du joueur

Le joueur demande de faire mieux pour les deux candidats, puis précise
que sa remarque concerne « les explosions : formes et impact visuel ».
Les premiers dessins et captures de revue 61 sont conservés, avec ce retour
dans `visual-feedback-01.json` ; aucune validation de leur rendu n’est déduite.

La revue 62 propose donc 70 `SM02-A02` et 71 `SM03-A02`. Le Poison expulse
des bouffées de gaz plus épaisses, avec un centre qui s’ouvre avant leur
dispersion. Le Froid produit une onde de givre et de petits éclats, au lieu
des grandes pointes de la première proposition. Les deux planches sont
produites avec Imagegen intégré et conservées avec leurs prompts.
Durées visuelles proposées : 0,46 seconde pour Poison, 0,40 pour Froid.
Elles ne changent pas les délais ni les effets mécaniques des mines.

Les LED, statuts sur les corps, repères, compteurs et règles préparées
restent ceux de la revue 61. Les captures natives des statuts durant
l’attente sont réutilisées à l’identique, avec leur provenance conservée ;
les nouvelles captures concernent la séquence et ses explosions révisées.
Les obstacles coupent toujours les détonations. Ces nouveaux rendus restent
à valider dans `skill-animation-review-62/cote-a-cote.html`, sans intégration
au combat de campagne. Entrée du laboratoire : `--lot-mines-variantes-v2`.

### ANIM-S30-REV02 — Feu également repris, relais des résidus

Le joueur demande d’améliorer aussi le Feu. La revue 63 propose donc une
explosion `68 / SM00-A02`, comparée à la référence A01 conservée intacte.
La mine, le voyant rouge, les chiffres et la séquence restent identiques.
Seul l’impact est redessiné avec Imagegen intégré : front de feu plus marqué
et fragments qui se dispersent, durée visuelle proposée de 0,44 seconde.
Ce nouveau rendu reste à valider.

Le joueur précise également le relais visuel : avec SM05, l’explosion se
dissipe puis laisse place au sol enflammé, au gaz toxique ou au sol gelé.
La première application mécanique du résidu reste immédiate à sa création ;
ce relais de dessin ne crée pas un délai de dégâts. Les résidus seront
présentés dans leur lot dédié. Décision de direction :
`mine-residue-visual-handoff-direction-decisions.json`.

### MINE-VFX-GROUND03 — résidus sans fond uniforme

Le joueur demande de retirer le fond coloré uniforme de SM05, de décaler
les états et mouvements du gaz indépendamment dans chacune des neuf cases,
et de refaire le sol gelé pour qu’il évoque clairement la glace. Il confirme
cette synthèse le 14 octobre. Cette direction ne valide pas encore le rendu.

La revue 67 propose `73 / SM05-A03` pour les trois éléments. Les flammes
conservent leurs dessins ; le voile rectangulaire uniforme est supprimé.
Le gaz utilise des états choisis par case, des transitions douces et des
rythmes distincts, sans ordre régulier par ligne. Le bruit visuel dépend
uniquement de la case et du temps d’animation, sans toucher au hasard de combat.
Une nouvelle planche transparente produite avec imagegen intégré fournit
quatre sols de glace fissurée et givrée, dessinés directement en 64 px.

La surface reste carrée et coupée par les murs ; les durées, explosions,
premières applications et statuts gardent leur fonctionnement de revue.
La revue 65 et ses dessins restent conservés pour comparaison. La nouvelle
galerie animée et le laboratoire isolé attendent la validation du joueur,
sans intégration au combat de campagne. Le prompt et la source de glace
sont conservés dans `skill-animation-review-67/`.

### MINE-VFX-GROUND04 — neige et glace sans répétition par case

Le joueur trouve le nouveau sol joli mais trop répétitif et demande une
version mêlant neige et glace. La revue 68 propose donc `73 / SM05-A04-FROID` :
une seule surface continue, irrégulière et transparente de neige, givre et
petites plaques gelées remplace la répétition de quatre textures par case.
Le masque des cases affectées coupe toujours cette surface aux obstacles.
Feu et Poison conservent exactement leurs captures A03 de la revue 67.
Les sources et aperçus précédents restent conservés ; ce nouveau rendu
attend la validation du joueur, sans intégration en campagne.

### SM05-GEL-RESTORE01 — retrouver le Gel accepté

Le joueur signale que le Gel sur les monstres de la revue 68 ne correspond
pas au rendu déjà défini. Le prototype de résidu lisait à tort l’atlas de
six images de 256 px comme quatre images de 128 px, avec un autre placement
et une autre cadence. La revue 69 retire ce découpage et appelle directement
le rendu commun `GelArt::load_approved()` / `GelArt::draw()` de GEL-A05.

La glace translucide, le visage dégagé, les cristaux derrière les pieds,
les dimensions et les transitions sont ceux de la référence validée en
revue 28. Aucun nouveau dessin ni effet mécanique n’est créé. La surface
continue de neige/glace et les autres éléments conservent leur rendu.
Les anciennes captures restent archivées ; l’aperçu corrigé permet de
vérifier la restauration avant de reprendre la revue des résidus.

### SM05-ANIM05 — validation de la revue 69

Le 14 octobre 2026, le joueur répond « ok parfait je valide » à la revue 69.
Les rendus de Résidu persistant deviennent les références acceptées :
`73 / SM05-A03-FEU`, `73 / SM05-A03-POISON` et `73 / SM05-A05-FROID`.
Le fond coloré uniforme est absent ; le gaz possède des états décalés par
case, et le Froid mêle neige, givre et glace sur une surface continue.
Le Gel attaché reprend GEL-A05, déjà accepté, avec son visage dégagé et
ses couches devant le corps et derrière les pieds.

Les captures natives 64 px, les sources et leurs empreintes sont conservées
dans `skill-animation-review-69/visual-acceptance-01.json`. Les anciens
aperçus restent archivés. L’accord porte sur ce rendu ; les chiffres de
démonstration ne deviennent pas une nouvelle décision d’équilibrage.
Le raccordement au combat réel et l’aperçu étendu de SM04 gardent leur
périmètre distinct.

### SM04-ANIM01 — Explosion étendue, comparaison avant validation

La revue 71 présente `72 / SM04-A01` pour Feu, Poison et Froid : rayon
d’une case sans le passif, contre quatre cases au rang effectif onze.
La scène ouverte affiche les neuf cases de base et les quatre-vingt-une
cases agrandies ; une cloison complète les réduit à six et quarante-cinq.
Le placement et le survol de la mine utilisent cette même surface carrée.

L’impact élargi reprend les sources HD des explosions présentées et retenues
dans la revue 69. Les flammes basses, les mouvements indépendants du gaz,
la surface continue neige/glace et GEL-A05 gardent leurs dessins.
La neige élargie possède une seconde couche continue discrète pour éviter
des cellules vides ; aucun motif n’est répété par case pour le Froid.
Les cibles conservent les mêmes positions dans les deux profils, et le
joueur ne reçoit pas ses propres dégâts. Les compteurs sont préparés.

La galerie montre deux applications et un raccourci vers l’expiration ;
le laboratoire natif permet d’avancer tous les tours et de tourner la scène.
La revue 70 reste un brouillon non présenté, avec une approche de cible
inadaptée à la comparaison. La revue 71 est la proposition à examiner.
L’accord sur le passif ne valide pas encore cette composition agrandie.
Aucune compétence, règle de campagne ou sauvegarde n’est modifiée.


### Catalogue global 24 — tailles réelles et familles complémentaires

Le 17 octobre 2026, le joueur demande une grande image avec nom, code et sprite
à la taille réellement prévue pour le jeu, ainsi que le retour des anciens
cultistes dans la liste. `npc-monsters-review-01/CANDIDATS-MONSTRES.md` réunit
102 modèles des revues courantes, sans redimensionnement à la composition,
avec le joueur comme référence. La base d'affichage reste 64 px par case ; les
petites créatures, adultes, élites et boss gardent leurs différences de taille.
Le lot `CULTES-24` ajoute seize propositions : cultistes possédés, zombies,
esprits torturés, squelettes et créatures enflammées. Les flammes et silhouettes
spectrales présentées sont des dessins statiques à examiner. L'inscription au
catalogue ne valide pas individuellement ces nouvelles apparences, leurs noms
ou leurs règles ; les validations antérieures restent conservées.


### Nécromanciens 25 — lecture clinique et cohérence des accessoires

Le joueur demande des nécromanciens scientifiques du culte et démoniaques.
Pendant la création, il refuse le scientifique 132 r01 : seringue non droite,
tenue trop peu scientifique. Les nouvelles propositions emploient une blouse
à revers et poches avec proportions humaines ; la seringue du 132 r02 partage
un axe vertical. Il demande aussi de corriger le visage et le livre du 139 :
la r02 propose une tête organique assortie au corps et un livre ouvert orienté
vers son regard, soutenu par une main adaptée. Ces révisions restent à examiner
dans `npc-monsters-review-01/NECROMANCIENS-25.md`. Les noms et les règles restent
distincts ; le catalogue courant conserve 110 modèles à taille native.


### Reliquaire 26 — nécromancien insectoïde

Le joueur demande un artefact sordide à la place du livre du 137. La r02
propose un reliquaire charnel sous métal noirci et crochets d'os, conservant le
gabarit natif et la silhouette. Les pinces sont adaptées à sa prise. Il confirme
les sept autres nécromanciens pour leur présence dans la liste. La r02 reste
à examiner ; voir `npc-monsters-review-01/NEC-D02-ARTEFACT-26.md`.


### Bestiaire démoniaque 27 — ailes, vers et familles

Le joueur demande de nombreux vers et larves, des démons musclés ailés de
différentes couleurs, des minotaures ailés, des démons de peste et du froid,
ainsi que des formes presque humaines. Il autorise les tailles supérieures
à 64 px. La revue `BESTIAIRE-DEMONIAQUE-27.md` présente vingt-quatre modèles
140–163 à taille native sur base de 64 px par case, avec deux palettes chacun,
ombres séparées et contrôle sur fonds clair, sombre et texturé. Les gabarits
et envergures restent propres à chaque silhouette ; les nouveaux dessins,
noms et règles restent des propositions. Aucun boss ou pouvoir nouveau n'est
défini par cette famille graphique. Le catalogue courant contient 134 modèles.

### Palettes de matériaux 07 — extension validée

Le 17 octobre 2026, le joueur valide et demande l’intégration des 176 variantes
rouge-orangé et vertes des 88 autres modèles présentés sur six planches natives.
La revue `monster-material-palettes-07/review.json` conserve l’accord explicite.
Avec les quatre palettes antérieures des modèles 144 et 150, 90 modèles disposent
de cartes de couleurs de matériaux. Les dessins sources A/B, contours, alpha,
gabarits et points d’appui restent identiques. Les masques propres aux sources
protègent les tissus, les os et les métaux, y compris lorsqu’ils partagent un RGB
avec la chair. Les vêtements marron du 156 restent distincts de sa peau.

La génération 151 utilise ces variantes pour les rôles déjà intégrés. Les
apparences de la génération 150 restent identiques ; sauvegarde, inspection et
corps au sol conservent le modèle et sa palette. Les lanceurs de sorts réservés
attendent leurs pouvoirs. Cet accord visuel ne définit aucun nouveau pouvoir,
chiffre ou nom. Le catalogue courant conserve les apparences à taille native.

### Complément bleu 08 — validation et intégration

Le joueur demande ensuite des variantes bleues pour le gel et valide les 51
palettes E présentées sur six planches natives : « Valider et intégrer les 51
bleues ». Les 39 modèles qui avaient déjà un bleu, bleu-gris ou cyan conservent
ces dessins. Les cartes explicites utilisent les mêmes masques et points
d’appui que la revue 07, sans changer les tailles ni les matériaux protégés.
La génération 152 active ce complément ; les affectations des générations
150 et 151 restent identiques. L’accord est conservé dans
`monster-blue-palettes-08/review.json`. Les futurs dégâts de gel restent
distincts de cette validation visuelle.

### ART-FINISH-COMPLETE-06 — contrôle délégué des contenus existants

Le joueur délègue la finition et la validation des dessins restants par
« je te laisse tous les finires » puis « et les valider ». Cette délégation
concerne la refonte en cours des contenus existants ; elle n’active ni nouveaux
boss, ni pouvoirs, ni textes, ni règles chiffrées. Les preuves et le bilan
sont conservés dans `docs/reviews/refonte-histoire-2026-10-16/art-finish-completion-06/`.

Les variations d’un même monstre partagent désormais une même géométrie,
un même alpha et les mêmes appuis, y compris les variantes originales A/B.
Les couleurs sont compilées par matériau ; les vêtements, os, objets et
pièces métalliques protégés ne deviennent pas une peau colorée. Les nuances
portent une identité visuelle et ne créent aucun pouvoir de combat.

Les tenues utilisent leur dessin terminé pour les deux mains, les paires,
les armes lourdes et les armes à deux mains. Reprendre les membres peints
comme couches séparées avec leurs attaches ; conserver la tête, les jambes
et les pieds. Ne jamais retourner la tenue pour retourner une arme.
Les masques de palette distinguent le matériau principal, les tissus
secondaires, la peau et les métaux. Conserver une part du pigment d’origine
sur les nouvelles peintures pour éviter l’aplat saturé. Le visage et les bras
nus de Bastion restent protégés. La règle historique des icônes reprenant la
silhouette entière est remplacée pour les objets de torse par TORSO-ICONS-01
et TORSO-STARTING-02 ci-dessous ; les portraits de création déjà acceptés
restent conservés.

Valider au gabarit réellement utilisé, puis dans les lumières du moteur.
Une grande source détaillée n’autorise aucun agrandissement en jeu.
L’audit séparé du mobilier confirme le surdimensionnement des lits de la rue ;
leur remplacement n’est pas inclus dans cette activation des acteurs.

### TORSO-ICONS-01 / TORSO-STARTING-02 — icônes de torse et couleurs acceptées

Le joueur valide les quatorze premiers modèles par « ok , on peut les
integrer :) », puis les cinq tenues de départ par « Valider les cinq et
leurs couleurs ». Les preuves et accords sont conservés dans les deux
dossiers de revue correspondants sous `docs/reviews/refonte-histoire-2026-10-16/`.

Les 19 objets de torse, illustrations 101–114 et 200–204, ont une icône
représentant seulement la protection ou le haut du vêtement : buste isolé,
sans tête, mannequin, corps, jambes ni arme. Les robes et manteaux utilisent
un haut compact avec manches et ourlet. Le même objet apparaît au sol,
dans l’inventaire, les emplacements d’équipement et les fiches. Toujours
réduire la source native64 dans les petits emplacements ; ne pas reprendre
un ancien personnage entier ni agrandir une source 32 px.

L’équipement de torse continue à sélectionner la tenue entière portée.
Les 93 associations acceptées comprennent A (origine), B (bleu ardoise),
C (vert mousse), D (rouille) et E (rouge sombre). TOR-109 et TOR-203 sont
déjà rouges en A et ne reçoivent pas de doublon E. Une icône bleue doit
correspondre à une tenue bleue, avec la même famille de matériau.

Pour ces quatre nouvelles palettes seulement, appliquer la rampe acceptée
à 90 % et conserver 10 % du pigment source sur le matériau principal.
Les masques explicites protègent les panneaux secondaires, la peau et les
autres matières ; la recoloration s’applique aux couches portées séparées
sans teinter l’arme. Les nouveaux masques base/core de FIN-01 incluent le
tissu du torse de Bastion, avec le visage et les bras nus protégés.
Les anciennes palettes, leurs indices sérialisés, les identités d’objet,
les bonus et les portraits restent conservés. Les couleurs sont cosmétiques.

Comparer les pixels acceptés, puis contrôler dans le moteur l’objet au sol,
l’inventaire, la fiche, la tenue éclairée et les configurations de mains.
La galerie conserve les planches historiques et les captures actives.
