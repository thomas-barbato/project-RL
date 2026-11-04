# ANIM-POLISH01 — reprise après essai en jeu

Revue ouverte le 30 octobre 2026. Statut au 31 octobre : proportions des casques
04 acceptées, lance-grenades encore en revue, prochaines retouches des casques
à préciser ; aucune intégration au rendu normal.

## Accord reçu après cette revue

« ca me va, il faudrait cependant refaire le model de lance-grenade qui semble
bien plus gros que toutes les autres armes :) . » accepte C01, C03 et C04 ainsi
que la direction générale des armes lourdes, avec reprise du lance-grenades.
La réponse suivante « 01 — réduction modérée » retient C02-A pour les autres
armes lourdes. Le joueur demande aussi des casques proportionnés au porteur.
Voir `acceptance-partial.json`
et `GL01-REVUE.md` pour le nouveau dessin C05-GL01-r01.
La proposition de casques 04 / C06-HELM01-r01 est dans `HELM01-REVUE.md`.
Elle reçoit le 31 octobre « oui ca me convient, ensuite on va faire des
modifications sur les casques ». Les proportions montrées sont acceptées ;
les modifications suivantes restent à préciser. Voir `HELM01-ACCEPTANCE.json`.
Le joueur précise ensuite l'inventaire, la réduction possible du nombre de
modèles visuels et leur adaptation à la classe, avec cohérence de face, de dos
et de côté. La section d'inventaire est décrite dans `HELM02-REVUE.md` ; le
nombre de modèles à garder et les nouveaux dessins restent à choisir.
Le calibrage suivant doit aussi prendre en compte l'armure portée pour éviter
un casque trop petit ; les proportions 04 restent la base de comparaison.
Le retour suivant juge les icônes trop semblables et demande une refonte moins
générique. Le joueur donne la priorité aux icônes avant les visuels portés,
avec un essai de visières animées : voir `HELM03-REVUE.md`.
La suite HELM04 demande cinq icônes, une par degré de poids visuel d'armure.
Le joueur révise aussi la direction portée : conserver un dessin de casque
commun, fidèle à l'icône, et adapter seulement son port à la carrure et à l'armure.
Les nouveaux dessins seront jugés ensemble ; voir `HELM04-REGLE.json`.
Les cinq icônes reçoivent ensuite « oui ca me va :) ». HELM05 présente leurs
vues portées communes, ajustées aux quatre classes et cinq degrés d'armure,
sans nouvelle intégration normale. Voir `HELM04-ACCEPTANCE.json` et `HELM05-REVUE.md`.

Le joueur signale une tête trop mobile, des armes lourdes de tir trop grosses,
des icônes au sol trop grandes, un décalage tête–corps dans le portrait du
panneau « Visée », et des prises parfois étranges pendant les déplacements.
Il autorise le début du travail et précise que la réduction concerne surtout
les armes lourdes de tir. Cette autorisation ne vaut pas acceptation des
candidats ci-dessous.

## Premier lot

- **ANIM-POLISH01-C01** : essai de stabilisation par translation entière de la
  figure, bras, mains, arme et casque compris. Compensation maximale de 2 px
  horizontalement et 1 px verticalement. Aucun découpage de la tête ni des
  jambes. Les phases, la durée des pas et les règles de déplacement restent
  identiques. Ce recalage réduit le déplacement global ; il ne redessine pas
  les variations de silhouette présentes dans les sources.
- **ANIM-POLISH01-C02-A (colonne 01)** : formes des armes lourdes plus compactes.
  Les extensions au-delà des deux prises utilisent un facteur 0,80 ; l'épaisseur
  hors de la bande verticale des prises, 0,85. Le rectangle contenant les deux
  prises conserve ses dimensions ; les canons horizontaux restent droits.
  Ces facteurs ne représentent pas une réduction uniforme
  de la taille totale de l'arme.
- **ANIM-POLISH01-C02-B (colonne 02)** : même principe avec facteurs 0,65 et 0,75.
  Les deux attaches et leur écartement sont conservés. Le résultat doit encore
  être jugé sur sa lisibilité, ses matières et son aspect massif.
- **ANIM-POLISH01-C03** : destination des icônes au sol de 64 à 40 px, centrée
  dans la case de 64 px. Les icônes du panneau et de l'inventaire ne sont pas
  modifiées par cet essai. Les captures utilisent les sources 64 px.
- **ANIM-POLISH01-C04** : portrait composé à partir du repos frontal entier
  utilisé sur la carte. Le chemin actuel du portrait reprend encore la
  composition frontale antérieure, avec des proportions et une pose différentes.
  Cette divergence est reproduite ; le décalage intermittent précis signalé par
  le joueur n'est pas encore isolé comme une cause unique.

L'avis sur C01, C02-A/B, C03 et C04 reste distinct. Accepter une compaction ne
valide pas automatiquement les autres corrections.

## Périmètre et suites

La revue animée présente les quatre profils Bastion, Traqueuse, Arcaniste et
Métamorphe en armure lourde, avec la tête de la tenue ou le casque 120. Quatre
armes lourdes représentatives couvrent mitrailleuse, lance-roquettes,
lance-grenades et lance-flammes ; le fusil d'assaut sert de témoin.

La méthode de compaction vise 28 modèles appartenant à ces quatre familles ;
les autres fusils, pistolets et armes de mêlée conservent leur dessin. La revue
pilote ne constitue pas une validation de toutes les déclinaisons.

Le travail sur les positions étranges des bras et des mains reste ouvert :
les prises des armes lourdes gardent leurs attaches, mais le contact des pixels
ne prouve pas leur anatomie. Examiner ensuite les configurations précises,
notamment l'ambidextrie, les autres tenues, les casques et les transitions de
direction, après le choix des proportions. Aucun nouveau dessin n'a été créé.

## Reproduction

```powershell
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-animation-polish artifacts/animation-polish-01/current
python tools/art/build_animation_polish_review.py
```

L'aperçu courant pointe vers `artifacts/animation-polish-01/apercu.html`.
Les captures plates combinent natif 64 px et zoom entier ×2. Le camp utilise
les matériaux et l'éclairage du moteur, à taille native. La préparation des
images est terminée avant leur affichage ; le prototype conserve un seul sujet
et ses trois versions en mémoire à la fois. Il ne lit aucune capture à partir
du rendu normal. Les sauvegardes, préférences, sources acceptées et ressources
du jeu restent indépendantes de cette revue.
