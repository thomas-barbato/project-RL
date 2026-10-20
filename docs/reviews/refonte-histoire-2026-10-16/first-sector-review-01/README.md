# FIRST-SECTOR-01 — premier parcours après le camp

18 octobre 2026. **Proposition de plan et de décors, pas encore intégrée.**

## Recadrage ACT1-CITY-01 — 18 octobre 2026

Le joueur précise que le premier chapitre doit montrer une ville futuriste
en pleine apocalypse : lieux détruits, abîmés et abandonnés. La campagne
conserve une organisation par chapitres inspirée de Diablo II et la descente
prévue par le récit. Le camp de cryogénie accepté demeure le point de départ.

MAP-ACCESS-01-r01 reste un essai industriel non validé. Il ne représente plus
la proposition d'ensemble du premier chapitre. Les sources et le kit candidat
DEC-ACCESS-01-r01 sont conservés ; une éventuelle réutilisation dans des
intérieurs ne vaut pas validation graphique. La prochaine proposition doit
d'abord articuler les zones urbaines entre elles et avec les accès souterrains.
Leurs noms, leur nombre, leurs plans et leur rendu restent à soumettre.

La direction urbaine est enregistrée à partir de la demande du joueur ; aucun
nouveau dialogue, effet, ennemi, quartier jouable ou trajet n'est intégré ici.
La description ci-dessous conserve le contenu de l'essai pour référence.

La nouvelle proposition [ACT1-LAYOUT-01-r01](ACT1-LAYOUT-01.md) présente
l'enchaînement des quartiers et les détours avant les plans détaillés.
Son aperçu est `artifacts/first-sector-review-01/act1-city-layout-r01.png`.
Le joueur accepte ensuite cet agencement précis par « oui ca me va bien :) ».
La trace est dans `act1-layout-acceptance-r01.json`. Plans détaillés et décors
restent à présenter séparément, en commençant par U01.

## Revue courante — U01

[U01-STYLE-LUMIERES-07](U01-STYLE-LUMIERES-07.md) conserve le style final
accepté : surfaces simplifiées, contours affirmés, pénombre et émissions
lumineuses du moteur, sur la perspective du camp. La rue est active pour
les nouvelles parties de génération 155. L’accord et les images exactes
sont dans `visual-acceptance-r07.json`. Le joueur demande des cartes suivantes
plus grandes, en gardant l’échelle et ce style. Référence durable :
[STYLE_VISUEL_REFERENCE.md](../../../STYLE_VISUEL_REFERENCE.md).

[U01-PROJECTION-06](U01-PROJECTION-06.md) reprend la perspective directement
sur le camp d’arrivée après le refus de l’angle de la rue. SCENE-U01-06-r01
est montré avec le personnage à taille réelle, la maison de gauche ouverte
et le portail sud horizontal. Le joueur valide cette correction en demandant
de rester au plus près du camp. La génération 155 raccorde cette rue jouable ;
les propositions ci-dessous sont antérieures. Accord : `visual-acceptance-r06.json`.
Le [bilan d’intégration](U01-INTEGRATION-06.md) précise le portail sud,
l’habitation visitable, les anciennes générations et les contrôles.

[U01-ILLUSTRATION-04](U01-ILLUSTRATION-04.md) présente une illustration
d’ensemble en vue de dessus : nouvel agencement asymétrique, bâtiments
différenciés et mobilier propre à chaque lieu. Elle répond aux demandes
de changer la disposition et d’éviter les répétitions. Cette nouvelle
composition est une proposition visuelle, distincte du prototype précédent.

[U01-REVIEW-03](U01-REVIEW-03.md) corrige les bâtiments en vue de dessus après
le refus de la perspective frontale : toitures dominantes, murs courts,
garage et toiture effondrée. La voiture est aussi vue de dessus, toujours
sur 2 × 1 cases avec proposition de flammes. Le joueur accepte ensuite les
apparences corrigées : trace dans `visual-acceptance-r03.json`. Les plans
détaillés et les nouveaux décors de l’illustration restent à examiner.
Les précédentes revues ci-dessous conservent leur contexte.

[U01-REVIEW-02](U01-REVIEW-02.md) applique la contrainte de voiture de 2 × 1 cases
et présente sa boucle de flammes, ainsi que trois premiers modules de façade
et un matériau de toiture. Le plan révisé est MAP-U01-01-r02. Les nouveaux
dessins et l'animation restent candidats ; l'agencement du chapitre est conservé.

[U01-REVIEW-01](U01-REVIEW-01.md) présente le plan concret MAP-U01-01-r01
(40 × 28 cases), le kit DEC-U01-01-r01 et la voiture 17 r02, à taille native.
Les matériaux de rue sont assemblés ; les bâtiments restent schématiques.
Il s'agit de la revue antérieure. La page courante est réutilisée dans
`artifacts/first-sector-review-01/index.html`.

## Essai industriel antérieur — MAP-ACCESS-01-r01, non validé

Le joueur demande de préparer la première zone après une passe de stabilité
et un nettoyage. Cette revue sépare deux décisions :

- **MAP-ACCESS-01-r01** : plan de 44 × 30 cases et fonctions des cinq modules.
- **DEC-ACCESS-01-r01** : seize dessins natifs de 64 × 64 px et leur assemblage.

## Parcours proposé

M01 est l’arrivée depuis le camp accepté, avec un retour au camp.
M02 est la jonction principale, plus large, avec deux massifs pour couper les
lignes de tir. M03 est un détour de maintenance, relié par deux chemins qui
reviennent avant M05. M04 est une petite alcôve facultative. M05 est le poste
de contrôle sur le parcours obligatoire, avant la sortie vers la suite.
Ces appellations décrivent la revue et ne constituent pas de nouveaux noms
validés destinés au jeu.

Le trajet le plus court couvre 38 cases depuis le point de départ de revue.
Les corridors de liaison font trois cases de large. Le plan reste un petit
gabarit représentatif, destiné à juger les proportions avant de choisir la
taille et le nombre définitifs des cartes du secteur.

Les 496 cases praticables sont reliées. Le retour est possible ; supprimer le
détour laisse la sortie accessible. Supprimer M05 coupe la progression vers
la sortie. Les deux liaisons de maintenance ont été vérifiées séparément.
Ces contrôles portent sur les données du prototype Python, sans certifier
un générateur de campagne ou un comportement de combat.

## Premiers décors

Le générateur d’images intégré produit un atlas unique à partir du camp
accepté comme référence de style. Sa source est conservée sans modification
dans `assets/source-art/first-sector-review-01/DEC-ACCESS-01-source-r01.png`.
Le prompt exact est dans `imagegen-prompt.txt` ; les exports et empreintes
sont dans `decor-manifest.json`.

DEC-A01 à A04 : béton, métal, grille et seuil ouvert. DEC-A05 à A08 : les
quatre orientations de mur. DEC-A09 à A12 : les quatre angles. DEC-A13 à A16 :
les quatre jonctions en T. La planche montre les exports 64 px sans zoom.
L’assemblage de revue utilise aussi des bandes et des points de raccord issus
de ces dessins ; leur aspect est soumis avec le kit. Il ne prétend pas que
chaque orientation ou chaque raccord est déjà validé.

Le joueur et le mobilier existants gardent leur source à l’échelle 1 et leur
ancrage (64,96) sur le canevas transparent de 128 px, comme le camp accepté.
Le corps du joueur fait environ 64 px ; le canevas de débordement n’est pas
réduit à une case. Aucun nouvel acteur n’est créé.

## Limites et prochaine vérification

Cette revue prépare le plan et les premières pièces graphiques. Elle n’active
aucune nouvelle carte jouable, quête, rencontre, récompense, compétence,
réapparition ou règle de voyage. Le peuplement commencera par les cultistes
humains, conformément à la direction acceptée, mais ses placements et valeurs
ne sont pas fixés ici. Le camp actuel et les données de l’éditeur restent intacts.

Après les accords distincts sur le plan et l’art : importer le gabarit dans
le prototype moteur, confronter dessin et collisions, tester les raccords et
la circulation des acteurs, puis préparer les modules variables. L’éclairage
et le mobilier supplémentaires seront ajoutés selon les besoins réels.

La page courante est `artifacts/first-sector-review-01/index.html`. Le navigateur
automatisé a refusé son ouverture en raison du protocole local `file://` ; aucun
serveur ou autre contournement n’a été utilisé. Les PNG sont directement
consultables. Reconstruction : `python tools/art/build_first_sector_review.py`.

## Toits acceptés le 19 octobre 2026

ROOF-U01-01A/B et le masquage à l’ouverture de la porte sont acceptés dans
[la revue des toits](../street-roofs-review-01/README.md). Les nouvelles parties
de génération 156 utilisent la porte de la maison ; les reprises antérieures
gardent leur ouverture d’origine. SCENE-U01-07 et LUM-U01-07 restent les
références du décor et de son éclairage.
