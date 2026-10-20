# Agrandissement des Abords de la prison — revue r06

19 octobre 2026 · **MAP-U01-EXP-01-r06 / LUM-U01-08-r06**.

## Intégration pour essai en jeu — génération 157

Le joueur demande : « ok integre la map pour l'instant j'ai envie de la tester
en jeu :) ». Cet accord autorise l'activation du candidat r06 et de
VISION-U01-04 pour un essai dans le lancement habituel. Il ne constitue pas
une validation artistique définitive de tout le quartier.

Les nouvelles parties reçoivent la rue de 26 × 66 cases depuis le camp. Les
parties des générations 155/156 conservent leur rue de 26 × 18 cases, visitée
ou encore différée. Le renderer choisit les ressources selon la carte réelle ;
les coordonnées des anciennes parties ne sont pas déplacées. Pour tester
l'extension, créer un nouveau personnage.

Les exports utilisés sont dans `assets/maps/prison-street-expanded-01/`.
Les sources restent préservées. `integration-assets.json` consigne l'accord et
les empreintes des copies. Le masque circulaire courant utilise
`src/painted_scene_visibility.rs` dans les builds debug et release, sans charger
les index expérimentaux VISION-U01-03. Le décor ne dépend d'aucun fichier sous
`artifacts/`. Les collisions et la visibilité des acteurs gardent leurs règles.

La correction des quatorze faux murs d'intérieur est incluse, avec les mêmes
obstructions de déplacement et leur restauration après sauvegarde. Les trois
portes ouvrables, le masquage des toits et le retour au camp sont conservés.
Le passage nord reste réservé : U02 n'est pas encore générée ni raccordée.

Vérification d'intégration : onze tests de rue réussis, compilation debug et
contrôle release réussis, vingt et une vues GPU et trois séquences d'animation
capturées sans mutation, retour au camp confirmé. La reprise des générations
155/156 et celle de la carte agrandie sont vérifiées. Les contrôles sont dans
`integration-checks.json`. Le logement ouvert et le portail reproduisent
exactement les pixels du prototype VISION-U01-04 sur la zone de jeu comparée.

Diagnostic de l'intégration, avec le même fournisseur de carte et le même
rendu que le lancement normal :

```powershell
target/debug/project-rl.exe --ui-cold-street-integration artifacts/street-expansion-review-01/integration/engine
```

## Révision courante — contours complets et conduits variés

Le signalement sur les ombres est étudié séparément dans
[VISION-U01-04](VISION-U01-04.md), la comparaison visuelle courante demandée par
le joueur : afficher tout le décor dans le cercle, en conservant les obstacles
pour les acteurs, déplacements et attaques. L'essai précédent de silhouettes
complètes est décrit dans [VISION-U01-03](VISION-U01-03.md), et la correction des
faux murs dans [VISION-U01-02](VISION-U01-02.md). Ces prototypes restent sans
activation en partie normale.

Le joueur signale trois ressources encore tronquées : l'entrepôt en L et
les deux intérieurs réutilisés. Des accessoires traversaient la limite de
la silhouette ; une marge transparente autour de leur source ne les rendait
pas entiers. Les deux intérieurs conservaient aussi une ancienne découpe.

La r06 reprend ces trois architectures sans leurs accessoires extérieurs.
Une caisse de 46 × 40 px, une armoire de 40 × 48 px et un fût de 24 × 32 px
sont dessinés individuellement puis placés entiers devant l'entrepôt. Les
anciens luminaires peints au-dessus des entrées des deux intérieurs sont
retirés pour ne plus laisser de fragment quand le moteur ouvre la porte et
affiche déjà une lampe sur le mur voisin.

Le joueur précise ensuite que le prolongement sud doit être conservé :
seuls ses coins mal placés et ses gravats étaient en cause. L'aile gauche
garde donc sa longueur ; son toit bleu se poursuit jusqu'à une extrémité
fermée par un mur complet, avec les deux coins à leurs vraies extrémités.
Les gravats sans justification sont retirés. La tentative qui raccourcissait
le bâtiment est abandonnée ; son origine reste dans l'historique du prompt.
Le joueur accepte ensuite l'emplacement des coins, mais signale la façade
sud trop basse. **MUR-SUD-03** reprend sa hauteur apparente depuis la façade
du garage, avec des piliers continus jusqu'à la fondation. La base et la
largeur sont conservées ; la bande basse du toit laisse place au mur plus
haut. Le joueur valide cette version le 19 octobre 2026 : « parfait là
c'est bien :) ». Les pixels exacts de l'entrepôt sont enregistrés dans
`warehouse-south-wall-acceptance-r03.json`. Cet accord concerne l'entrepôt ;
il ne vaut pas validation globale de la carte agrandie.

À la demande suivante du joueur, les conduits sont variés : cuivre terni
sur l'entrepôt, rouille sur le garage et acier gris sur le bâtiment long.
Certains conduits rouges sont conservés. Les couleurs fonctionnelles des
voyants de porte ne changent pas.

Les huit nouvelles sources ou reprises de cette révision restent isolées
et transparentes. La bibliothèque conserve les codes KIT-U01-001 à 017,
ajoute les deux intérieurs 018/019 et les accessoires 020 à 022. Les sources
propres précédentes sont partagées, sans duplication. Les six ruines et les
quinze autres accessoires réutilisés ont été examinés : aucune autre coupe
comparable de boîtier ou de caisse n'a été repérée dans ce contrôle.

La grille recalculée sur ces contours contient **695 cases libres reliées**.
Les portes, passages et limites gardent leurs règles ; la carte de partie
normale reste inchangée. Les autres ressources modifiées restent en revue.

Voir `contour-checks-r06.json`, `candidate-r06.json`, `render-r06.json`,
`scenery-library-r06.json`, `imagegen-prompts-r06.json` et `validation-r06.md`.
Les originaux de l'outil intégré imagegen et leurs exports sont dans
`assets/source-art/first-sector-review-01/u01-expansion-r06/`.

```powershell
python tools/reviews/build_street_contours_review.py
target/debug/project-rl.exe --ui-cold-street-expansion artifacts/street-expansion-review-01/engine-r06
python tools/reviews/show_street_north_review.py
```

L'aperçu courant et la bibliothèque conservent les mêmes adresses sous
`artifacts/street-expansion-review-01/`. La planche
`contours-corriges-native.png` montre les trois bâtiments séparés et légendés
à taille native ; `warehouse-continuous-native.png` montre leur assemblage
dans la cour. Aucun montage de coins éloignés n'est utilisé comme carte.

Le repère courant **MUR-SUD-03** désigne la façade à hauteur corrigée de
l'entrepôt. `entrepot-mur-sud-03.png` l'isole à taille native avec son code.
Le précédent MUR-SUD-02 ne sert que d'historique ; sa source est conservée
dans `warehouse-south-r02-source.png`. Le prompt et l'accord limité aux
positions des coins sont consignés dans `warehouse-south-wall-r03.json`.
La bibliothèque ajoute l'empreinte de chaque image à son adresse pour que
le rechargement utilise les pixels courants. Le fichier actuel ne comporte
plus la cavité remplie de gravats de l'ancienne image signalée par le joueur ;
l'origine de cet ancien affichage n'a pas été établie.

## Historique — ressources indépendantes et raccords r05

Après l'audit r04, le joueur autorise les corrections, puis précise que la
méthode des éléments dessinés séparément doit s'appliquer à tous les décors.
Le contrat est consigné dans [PRODUCTION_DECORS.md](../../../PRODUCTION_DECORS.md).

La r05 reprend **17 éléments** depuis leurs propres images transparentes :
cinq bâtiments, l'entrepôt en L, deux accessoires de cour, la végétation,
un mur effondré, un lampadaire abattu, le portail nord, un luminaire de rue
et quatre couches de toit. Les nouvelles sources ne sont plus extraites de
panneaux illustrant une rue. Les 23 ressources réutilisées gardent leur
provenance ; le départ historiquement approuvé n'est pas présenté comme
entièrement redessiné. Les dégâts des toits font partie de chaque variante.

Les quatre coins de l'enceinte sont raccordés aux segments du kit du camp.
Le décor et les 46 anciennes cases de sol derrière la prison sont retirés,
avec le portail conservé à sa position d'arrivée. Deux luminaires sont
supprimés, six sont éteints et leurs lentilles sont sombres. Les sources
restantes sont calculées par le moteur : **29 actives sur 35 configurées**,
intérieurs et voiture compris. Aucun ajout ne recouvre les lentilles
contrôlées. La benne est déplacée pour ne plus croiser le bord d'un impact.

La grille contient **692 cases libres reliées**, trois bâtiments visitables
et le passage nord réservé. Elle garde des cases de 64 px et la disposition
du départ. La carte de partie normale et ses empreintes restent inchangées.
Le candidat n'est chargé que par `--ui-cold-street-expansion` en développement.

L'aperçu courant est `artifacts/street-expansion-review-01/index.html`.
`bibliotheque.html` présente les ressources séparées à taille native.
Le plan complet est continu. Un ancien diagnostic accolant quatre coins
sans séparation a été retiré : le joueur refuse ses coupures, et confirme
qu'il ne refuse pas le dessin des décors pour cette raison. Cette précision
ne constitue pas une validation générale des pixels r05.

Sources, prompts de l'outil intégré imagegen, manifestes et bilans :
`assets/source-art/first-sector-review-01/u01-expansion-r05/`,
`imagegen-prompts-r05.json`, `scenery-library-r05.json`,
`candidate-r05.json` et `render-r05.json`.

```powershell
python tools/reviews/build_street_refined_review.py
python tools/reviews/show_street_scenery_library.py
target/debug/project-rl.exe --ui-cold-street-expansion artifacts/street-expansion-review-01/engine-r05
python tools/reviews/show_street_north_review.py
```

Les nouveaux dessins restent à valider avant activation dans la partie normale.

## Historique — quartier ravagé et densifié r04

Le joueur demande une zone de guerre, puis davantage de maisons dans les
grands vides de l’extension. La r04 conserve les routes continues de la r03,
ajoute **six bâtiments** endommagés et remplace l’intention de pillage par
des destructions de combats : effondrements, impacts, épaves et positions
détruites. Elle comporte **718 cases libres reliées**, avec des obstructions
réelles pour les gros débris et les trois anciens bâtiments visitables.
Les nouvelles maisons sont condamnées, avec leurs véritables lampes rouges.

Le détail, les sources et la reproduction historique figurent dans
[damage-direction-r04.md](damage-direction-r04.md). Les bilans de cette version sont
`candidate-r04.json` et `render-r04.json`. Les données r01/r02/r03 et la
description ci-dessous conservent l’historique des propositions antérieures ;
elles ne valent pas acceptation visuelle. La r04 est en attente de validation.

## Historique — disposition continue r03

## Direction confirmée et limites de l’accord

Le joueur corrige explicitement le sens de l’extension : « je veux que ca parte
de l'ancien départ de la carte […] à partir de là tu aggrandis la carte vers
le haut ». L’ancienne rue et la sortie de prison restent donc dans la partie
sud. Le nouvel espace est ajouté au nord, sans agrandir les cases, les meubles
ou les personnages. Le périmètre est fermé par des murs ou des bâtiments ;
un passage nord est réservé pour la prochaine carte U02.

La r01, étendue sur les côtés et au sud, est remplacée pour sa disposition.
Les dessins sources réutilisés dans la r02 restent des candidats ; ce
changement de plan ne les valide pas.

Le joueur refuse ensuite les raccords de la r02 : quatre morceaux trop
visibles, sans continuité entre eux, et des routes de largeurs incohérentes.
Il confirme la synthèse de cette remarque par « oui :) ». Cette réponse
autorise la correction du candidat ; elle ne valide pas le visuel r02.
La r03 reste à présenter et à valider avant activation normale, selon AGENTS.md.

## Proposition concrète

- **26 × 66 cases**, contre 26 × 18 ; échelle inchangée à **64 px par case**.
- **886 cases praticables reliées**, contre 173 dans la rue active.
- Un seul sol commun et une avenue continue de 160 px, raccordée à l’axe
  existant au sud. Même largeur pour les branches secondaires ; les bordures
  sont calculées sur le masque routier global, y compris aux croisements.
  Les textures ont une phase commune dans le repère de la carte.
- Les nouveaux bâtiments sont des modules natifs individuels. Aucun panneau
  de carte complet n’est assemblé dans l’extension. L’ancienne rue est utilisée
  une seule fois ; son bord arrière suit les bâtiments et la végétation.
- Ancienne disposition conservée avec un décalage global de 48 cases vers
  le bas dans le nouveau repère. Aucune rotation, réflexion ou modification
  d’échelle. Le portail de départ conserve exactement ses pixels sources.
- Trois bâtiments visitables au total : l’ancienne maison et deux nouveaux
  intérieurs. Le toit se masque à l’ouverture de la porte et reste masqué
  lorsque le joueur est dedans.
- **Jaune : porte utilisable**, même lorsqu’elle est physiquement fermée.
  **Rouge : bâtiment condamné**. La maison voisine du premier logement reçoit
  bien cette lumière rouge. Aucun système de clé ou crochetage n’est ajouté.
- Les nouvelles sources réutilisent les shaders et la lumière ambiante du
  camp et de la rue. Lentilles, couleurs et irradiance sont calculées ;
  l’interface reste hors du composite lumineux.
- Animations proposées : ventilation lente, filet de vapeur sur une grille,
  et quelques braises sur la voiture. Elles ne consomment aucun tour et
  ne modifient pas la simulation.

Le point de transition nord est `[18, 1]`, avant la rangée de bord extérieure,
selon le contrat des cartes générées du moteur. Le portail représente un seul
passage de deux cases. Le contour logique reste fermé et tous les lieux sont
reliés. **U02 n’est pas encore dessinée ni raccordée comme destination
jouable** : cet aperçu réserve son accès sans inventer une carte de campagne.

## Ressources et reproduction

Les sources et données du candidat sont dans
`assets/source-art/first-sector-review-01/u01-expansion-r03/` ; certains
modules de maison et leurs toits réutilisent les sources r01. Le dossier
`assets/maps/prison-street-01/` reste identique. L’aperçu et ses captures
sont reproductibles dans `artifacts/street-expansion-review-01/`.

Les panneaux `north-source-r02.png` et `middle-source-r02.png` ont été créés
avec le skill **imagegen**, par l’outil intégré, puis copiés dans le projet.
Les prompts demandent une projection orthogonale de dessus identique à la
rue approuvée, des cases de 64 px, des silhouettes simplifiées à contours
foncés, des luminaires variés sans halos peints, une rue traversante et des
compositions différentes. Leur assemblage complet r02 a été refusé. La r03
en extrait uniquement les bâtiments et accessoires individuels à leur taille
native ; ces sources restent conservées. `layout-plan.json` décrit leurs
positions et empreintes, le repère unique et le tracé routier. Le sol commun
réutilise les textures de la rue approuvée, sans changer l’échelle du départ.

```powershell
python tools/reviews/build_street_unified_review.py
# Ce générateur reproduit la géométrie historique r03.
# Le mode de revue et la page courants utilisent maintenant r05 (voir ci-dessus).
```

Le script numérique utilise Pillow et NumPy ; le runtime Python fourni par
Codex les contient. Le mode de revue est réservé aux builds de développement
et ne charge ce candidat que par son argument explicite. Les sauvegardes du
joueur, ses préférences et les données de `artifacts/map-editor/` ne sont
pas écrites. Les captures utilisent une partie isolée sans récupération
automatique.

`candidate-r03.json` conserve la géométrie et les empreintes.
`render-r03.json` conserve les contrôles et captures réels du moteur.
Les rapports r01/r02 restent historiques ; ils ne prouvent pas une validation
visuelle du joueur. Les scripts r01/r02 restent disponibles pour reproduire
ces anciens essais et ne sont pas utilisés par la r03.
Le plan d’ensemble utilise une composition numérique des mêmes données ;
les gros plans et animations de la page viennent du framebuffer du jeu.
