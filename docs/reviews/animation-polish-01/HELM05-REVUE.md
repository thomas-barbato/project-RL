# HELM05 — les cinq casques portés, comparatif natif

MOTION02 est refusé pour sa taille excessive et son rendu trop doux. L’aperçu
courant propose les dimensions natives et le placement sans interpolation de
`HELM05-MOTION03-REVUE.md`, toujours avant accord visuel.

MOTION01 est refusé par le joueur. L’aperçu courant propose désormais les têtes
casquées complètes de `HELM05-MOTION02-REVUE.md`, avant nouvel accord visuel.

Retour MOTION01 du 31 octobre : la tête semble bouger séparément du corps,
dans toutes les vues. La proposition de raccord tête–cou–torse est consignée
dans `HELM05-MOTION01-REVUE.md` ; elle remplace le placement phase par phase
décrit ci-dessous dans l'aperçu courant, et reste à juger visuellement.

Retour EYE01 du 31 octobre : le joueur relève que l'optique du modèle 02 ne
garde pas sa position entre les dessins et disparaît dans un trois-quarts
encore frontal. La cohérence visuelle annoncée pour r02 n'est pas établie.
Les tests techniques précédents ne validaient pas cette correspondance.
`HELM05-EYE01-RETOUR.json` consigne le défaut et la reprise ciblée.

Le 31 octobre 2026, « oui ca me va :) » accepte les cinq icônes HELM04.
`HELM04-ACCEPTANCE.json` conserve cet accord. Ce lot prépare leurs nouvelles
vues portées sans intégrer de dessin encore non présenté au rendu normal.

## Dessins et identité

Les cinq modèles disposent de quatre dessins : face, vrai dos, côté gauche et
côté droit. Les côtés suivent les trois-quarts de la marche latérale actuelle.
La même coque, ses matières, sa palette, ses ouvertures et ses détails servent
sur les quatre classes ; seuls la taille, le placement et le roulis du port
changent. Le casque léger a son optique et son capteur sur sa tempe gauche :
à droite sur l'image frontale, à gauche de dos, proche du spectateur lorsqu'il
regarde vers la droite. Les côtés ne sont pas fabriqués par miroir.

Les premières feuilles avaient des asymétries incohérentes dans les cellules
de face/dos des modèles 01, 02 et 04 ; les reprises r02 les corrigent. Le centre
du masque 03 était devenu transparent : sa reprise r02 restitue le bouclier
facial sombre opaque de l'icône. Les sources initiales et les reprises restent
dans `assets/source-art/animation-polish-01/helmet-worn-r01/`, avec leurs prompts
dans `HELM05-PROMPTS.json`, `HELM05-RETOUCH-PROMPTS.json` et
`HELM05-W03-RETOUCH.json`. Le mode utilisé est **imagegen intégré**.

Le conditionnement découpe les quatre cellules de chaque feuille par leur
enveloppe alpha, conserve une même échelle pour les quatre vues d'un modèle,
puis les place dans quatre cellules 32 px avec pivot de nuque `[16, 24]`.
La case du monde demeure 64 px ; ces cellules sont des couches d'équipement.
Les cellules latérales sont remises dans l'ordre gauche/droite selon leurs
poses réellement dessinées, sans miroir ni repeinture. Les tailles et
empreintes sont conservées dans `HELM05-SOURCES.json`.

## Port et corpulence

Le calibrage de revue emploie les facteurs de porteur Bastion 1,08, Traqueuse
0,91, Arcaniste 0,97 et Métamorphe 1,03, multipliés par le degré d'armure
1 / 1,035 / 1,07 / 1,105 / 1,14. Ces valeurs sont proposées pour la revue.
Elles restent identiques pour un même équipement entre les phases et angles.
Le bas du casque suit la nuque ; le torse, les bras et les armes ne sont pas
redimensionnés avec lui. La tête d'origine reste visible dans les modèles
ouverts, tandis que les modèles fermés remplacent sa zone peinte sous la coque.

Le lot couvre les quatre classes et leurs cinq degrés d'armure : 20 tenues,
chacune avec les cinq casques, soit 100 combinaisons. Chaque combinaison
dispose des six phases de face, six de dos, six de gauche, six de droite et
du vrai retour frontal. La mitrailleuse 31 conserve le dessin commun et la
réduction modérée C02-A acceptée ; C01 recale les figures entières.
Les nouveaux casques sont fixes dans ce lot afin de juger leur dessin et leur
placement. Le clignotement reste visible dans l'essai d'icônes précédent.

## Aperçu et moteur

La section `#casques-portes` de l'aperçu courant propose les cinq armures,
les quatre orientations, six phases et le repos frontal. Son format compact
rapproche les quatre colonnes de la capture du moteur par recopie 1:1, sans
modifier les figures. La planche complète, le zoom entier ×2 et les dessins
de tête au zoom ×3 restent disponibles. Le camp montre les cinq casques en
colonnes, les quatre classes en lignes, sur leurs tenues très lourdes, sous
ses vrais matériaux et éclairages.

La préparation CPU et la création des textures sont terminées avant le dessin.
Le diagnostic conserve un seul degré d'armure à la fois (quatre témoins et
vingt configurations casquées), avec cinq petits atlas de coque. Le lecteur
prépare sept images pour sa sélection courante, sans lire les pixels à chaque
rafraîchissement. Les ressources incluses à la compilation sont dans `assets/`
et aucune capture `artifacts/` n'est utilisée par le jeu.

La sélection HELM05 est un état temporaire limité au diagnostic Debug, remis
en place après chaque préparation. La compilation Release et le rendu normal
gardent leur chemin antérieur. Les choix d'objets existants ne sont pas encore
remappés de six modèles vers cinq ; ce lot ne change ni équipement, ni règles,
ni sauvegardes. Le Technomancien historique est hors du pilote à quatre classes.

## Reproduction

```powershell
python tools/art/prepare_helmet_worn_review.py
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-helmet-worn artifacts/animation-polish-01/helmet-worn-r01/current
python tools/art/build_animation_polish_review.py
```

Les dessins portés et le calibrage restent proposés avant leur accord selon
`AGENTS.md`. L'accord sur les cinq icônes demeure acquis.
