# Rue entière — STYLE-MAP-U01-04

19 octobre 2026. **Rendu validé puis intégré au lancement normal de la rue étendue.**

Le joueur répond « oui je valide » après consultation de cette revue de la carte
entière. `acceptance.json` fige le rendu accepté, les quatre décors au sol montrés
et les empreintes. Les descriptions de proposition ci-dessous conservent la
chronologie ; le statut courant et les vérifications d'intégration sont dans
`integration-checks.json` et `INTEGRATION.md`.

Le joueur ne valide pas la direction sur le petit échantillon précédent :
« je ne peut pas valider, je veux voir la map et comment ca rend avec ce style
visuel ». Cela ne constitue ni un refus du style ni une approbation des pixels.
La revue présente maintenant toute la rue et neuf vues du moteur à taille de jeu.

## Ce qui est montré

- Carte complète, 26 × 66 cases, export natif de 1664 × 4224 px avec l'éclairage
  du moteur et les toits dans leur état réel au moment de la capture.
- Vue d'ensemble zoomable et déplaçable, comparaison actuelle/proposition à
  cadrage identique, et bouton de retour à l'échelle de 64 px par case.
- Neuf comparaisons natives : prison, rue sud, cour, rue centrale, maison fermée,
  rue nord, entrepôt, sortie nord, maison ouverte.

Les personnages vivants, leur équipement, les lumières réelles, le plan, les
collisions et les règles de jeu restent ceux de la partie actuelle. La vue
d'ensemble est un export du décor complet avant le masque de présentation.
Elle ne change pas la perception du personnage. Les neuf vues jouables conservent
le vrai cercle de six cases et n'ajoutent aucun acteur hors de sa visibilité.

## Méthode et limites de la proposition

L'essai étend la direction des matières usées à la carte entière par un matériau
de surface commun : couleur légèrement ternie, salissures mates, usure de peinture
et petits groupes d'éclats. Le masque de matière est dessiné par **imagegen
intégré**, puis exporté techniquement à 512 × 512 px. Sa source et son prompt
exact sont dans :

`assets/source-art/first-sector-review-01/u01-corrections-r04/`

Le matériau est appliqué dans le moteur au décor avant les acteurs et avant
l'éclairage, ainsi qu'aux toits et à la voiture comme surfaces séparées. Les
contours sombres, le vide noir et les lentilles émissives sont protégés. Les
formes et teintes utiles restent reconnaissables. Les intérieurs reçoivent la
même finition sans modifier les portes ni le fonctionnement des toits.

Les bâtiments existants **ne sont pas tous redessinés**. Cette proposition est
un traitement de leurs surfaces, permettant de juger l'ensemble en situation.
Le petit patch rectangulaire STYLE-U01-03 de la revue précédente est désactivé
dans ce mode : le conserver formerait une île de sol différente du reste de la
rue. L'entrepôt et le sol reprennent donc leur géométrie de production, avec la
nouvelle finition commune. Les sources de l'ancien essai restent conservées.

Aucune nouvelle découpe dans une illustration de carte n'est utilisée. Les
modules de production restent entiers et à leurs positions actuelles. La texture
commune est une matière plane, pas une illustration dont on extrait des objets.
Le procédé complète la composition par modules et n'en change pas les emprises.

GRIM-U01-01 et CORP-U01-01/02/03 restent montrés à titre de proposition près de la
prison. Leur état de validation ne change pas du fait de cette nouvelle demande.
GRIM-U01-02, la station médicale refusée pour cette rue, reste absente.

## Reproduction

```powershell
cargo build --locked
target/debug/project-rl.exe --ui-cold-street-style artifacts/street-defects-review-01/engine04
python tools/reviews/show_street_map_style_review.py
```

L'aperçu réutilise `artifacts/street-defects-review-01/index.html`. Les captures
précédentes restent des preuves historiques, sans constituer une validation.
Les nouveaux pixels et le matériau attendent le retour du joueur ; AGENTS.md
demande une validation explicite avant activation normale.

Le mode de revue et ses matériaux sont limités aux compilations debug. Les
textures supplémentaires sont chargées uniquement lorsque cet essai est activé.
Aucune ressource du jeu ne dépend d'`artifacts/`, aucun changement de sauvegarde
ou de génération n'est nécessaire, aucun PNJ vivant ni arme n'est retouché.

## Vérification

La capture compare chaque lieu au même endroit, avec les mêmes sources de
lumière, et contrôle l'empreinte du monde avant/après rendu. Elle ouvre réellement
la maison en y entrant, puis retourne à la prison et au camp. Les données exactes,
positions et dimensions sont dans `capture-checks.json`. Les contrôles de build,
du format et des fichiers sont consignés dans `validation.json`.

Les animations sont figées au même instant dans les deux versions. Un contrôle
de 3 236 pixels rouges du personnage répartis dans les neuf vues ne relève
aucune modification RGB entre l'actuel et la proposition. Les pixels de bord
et les autres couleurs ne sont pas couverts par ce contrôle ciblé ; les assets
du personnage restent les mêmes. Voir `actor-pixel-checks.json`.

La compilation debug, le contrôle release, le formatage, `git diff --check`,
la capture GPU des vingt images et le contrôle de syntaxe JavaScript réussissent.
La page interactive n'a pas fait l'objet d'un contrôle par navigateur automatisé ;
les images exportées par le moteur ont été examinées directement.

La validation porte sur cet aperçu ciblé ; toute la suite de tests du jeu n'est
pas rejouée pour ce changement de rendu debug.
