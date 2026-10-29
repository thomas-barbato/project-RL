# POLISH01 — scène témoin de finition

27 octobre 2026. **Version 02 validée avec les éclats, intégration autorisée sur
toutes les cartes.** Après la comparaison, le joueur confirme « avec . » à la
question d'intégration, puis précise « il faut donc l'adapter à toutes les map
du jeu :) ». L'accord et les références exactes sont dans `acceptance.json`.
Les sections suivantes décrivent le pilote montré ; le bilan d'intégration
figure à la fin de ce document et dans `integration-validation.json`.

Le comparatif se trouve dans
`artifacts/visual-polish-review-01/apercu.html`, avec une séquence GIF de trois
secondes et des PNG natifs. Les colonnes 01 et 02 utilisent le même morceau de
rue, le même Bastion avec sa hachette, un zombie 120 et un cultiste 014, les mêmes
phases de marche et l'animation B01 existante. Une case reste de 64 pixels.

La référence 01 est une composition contrôlée des renderers existants, avec
l'irradiance actuelle, pas une capture complète du HUD normal. Les deux ennemis
sont placés sur des cases praticables et réellement visibles par l'état isolé.
Le prototype ne lance pas d'IA ni de combat simulé : les mouvements et effets
présentent une frappe sans infliger de dégâts. Le retour frontal à l'arrêt et
la durée de marche de 400 ms conservent leurs dessins et paramètres actuels.

## Composantes de la revue

- **POLISH01-L01 : matières et éclairage.** Contraste plus marqué, accent froid
  dans les ombres et renforcement des empreintes chaudes des lampes déjà calculées.
  Un traitement limité au grain de faible contraste conserve les joints, fissures
  et contours. Aucun flou général ou agrandissement des images.
- **POLISH01-A01 : présence des acteurs.** Éclaircissement relatif au décor et
  liseré intérieur orienté sur un pixel, sans halo autour de la silhouette. Le
  corps, les armes et les phases sources restent ceux du compositeur du jeu.
- **POLISH01-FX01 : contact.** Accent lumineux local sur la chaussée pendant
  160 ms et six éclats brefs pendant 240 ms, synchronisés avec le contact B01.
  Ils sont désactivés dans la variante à mouvement réduit. Ces ajouts sont
  cosmétiques et n'indiquent ni nouveau type de dégâts ni changement de règles.

L'accord sur une partie ne vaut pas validation des autres. Le rendu des armures
entre orientations et l'interface complète ne sont pas redessinés dans ce lot ;
ce comparatif ne prétend pas résoudre leur harmonisation par un filtre.

L'accord final couvre L01, A01 et FX01. Toute modification ultérieure de dessin
ou d'effet conserve sa propre procédure de revue.

## Exécution et vérification

Prévisualisation native : `tools/reviews/show_visual_polish_review_01.cmd`.
Espace rejoue la séquence ; M active le mouvement réduit ; Échap ferme la revue.
Le mode est compilé uniquement en Debug et se termine avant l'ouverture de
l'application normale. Il ne lit aucune campagne et n'écrit aucun réglage.

Régénération depuis la racine du projet :

```powershell
cargo build --locked --bin project-rl
target/debug/project-rl.exe --ui-cold-visual-polish artifacts/visual-polish-review-01
python tools/art/publish_visual_polish_review_01.py
```

Le comparatif exécuté dans le moteur conserve à l'octet près la sérialisation
de l'état avant/après. Le GIF est contrôlé après encodage : 75 images, 25 images
par seconde, 3 secondes, 1360 × 800 pixels ; chaque panneau mesure 640 × 576.
Les vues de repos, de contact et de mouvement réduit sont inspectées. La version
initiale trop discrète est remplacée dans le même aperçu par le rendu final.

`capture.json` décrit l'exécution et ses limites ; `publication.json` conserve
les empreintes des sorties finales. Les 326 400 000 octets du flux RGBA temporaire
sont supprimés par le publieur après contrôle de l'encodage. Les ressources du
jeu et les originaux restent conservés. L'HTML est fourni comme lecteur local ;
sa navigation dans un navigateur n'est pas validée par la seule capture moteur.

## Intégration au rendu commun

Le jeu normal applique la finition dans `world_readability` et les matériaux
partagés de `camp_atmosphere`. Il n'y a pas de sélection par nom de carte : les
campements, rues, intérieurs, variantes d'égouts et sols générés utilisent ces
mêmes chemins. Les dessins, les palettes sources et les cases de 64 px restent
conservés. Les effets colorés, repères et interfaces gardent leurs passes ; les
masques de perception ne reçoivent pas la correction des surfaces.

Les décors peints conservent le traitement accepté et leurs cartes d'irradiance,
avec compensation avant l'éclairage final. Les sols générés déjà très sombres
conservent leurs détails de faible luminosité ; leurs acteurs, déjà dessinés en
pleine exposition, reçoivent le liseré sans le gain destiné aux acteurs éclairés
par la passe des décors peints. Cette adaptation évite de noircir les sols ou de
surexposer les silhouettes sur ces cartes.

La Frappe de rupture conserve son animation existante et reçoit l'accent lumineux
de 160 ms et les six éclats de 240 ms. Leur début suit le contact existant, avec
le même temps de présentation en secondes que le pilote. La lumière est découpée
sur les sols praticables, réellement visibles et accessibles en ligne de vue :
elle ne traverse pas les murs et ne dévoile pas une pièce fermée. Le mouvement
réduit supprime ces deux ajouts.

`--ui-cold-polish-integration` vérifie le vrai renderer sur 17 cas : camp, rue,
intérieur, trois variantes d'égouts, deux surfaces générées, trois familles de
sols souterrains, nuances de gris, frappe, frappe en mouvement réduit et trois
autres classes jouables. Les états sérialisés restent identiques ; les pixels
de l'interface hors pointeur restent identiques. Seuls des arrondis d'un niveau
sur 255 sont tolérés dans la frange noire du brouillard ; les masques et la
perception simulée restent conservés. Le radar utilise l'horloge de présentation
existante pour rendre ces captures comparables, sans modifier sa vitesse normale.

L'ancien aperçu demeure une comparaison du pilote accepté : sa portée désactive
la finition normale pendant la fabrication des colonnes, afin de ne pas appliquer
la correction deux fois. Le flux RGBA temporaire reste supprimé après publication.

Validation finale : 19 tests ciblés réussis (16 de présentation de combat et 3
de contact), compilation verrouillée de toutes les cibles, formatage et diff
sans erreur ; builds Debug et Release réussis. Les 17 cas de capture passent
avec les pixels d'interface conservés et sans changement d'état moteur. Le
Release de `Jouer.cmd` est reconstruit. L'inventaire conserve ses 1 815 ressources,
les empreintes des vues acceptées restent identiques et aucun paquet portable
n'est reconstruit. La suite complète de tests n'est pas exécutée pour cette
modification de rendu.

Le changement d'exposition entre les poses de quatre PNJ du camp est corrigé
dans [POLISH01-WALK-LIGHT01](WALK-LIGHT01.md). Toutes leurs poses dans le monde
utilisent le matériau des acteurs ; les dessins et la respiration acceptée
restent conservés. La validation de cette correction est enregistrée séparément
dans `walk-lighting-validation.json`.
