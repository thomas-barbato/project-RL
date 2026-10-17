# DEMONS-20 — designs conservés, dessin dans le style du jeu

17 octobre 2026. Galerie de candidats avant sélection individuelle et intégration.

Le joueur apprécie les nouveaux designs inspirés des bestiaires de Diablo II
et Grim Dawn, puis demande que leur dessin corresponde au joueur et aux armes
actuels. Cette reprise garde donc les anatomies et les silhouettes ; elle
reprend les contours sombres, les aplats et les ombres simples du jeu.
Les références officielles observées sont consignées dans `demons-arpg-19.json`.

Les quinze sources r01 des 60–74 restent intactes comme références de design.
Elles sont redessinées avec **image_gen intégré**, chacune avec sa source
précédente comme cible et `BD-C01-comic-master.png` comme référence de trait.
Les cinq derniers modèles 75–79 sont de nouvelles créations directement dans
ce style. Le joueur et ses armes ne sont pas modifiés.

## Candidats, dimensions réelles

| Numéro | Dessin proposé | Corps natif, largeur × hauteur |
| --- | --- | --- |
| 60 | Diablotin ferrailleur | 41 × 48 px |
| 61 | Bouc des failles | 43 × 68 px |
| 62 | Décharné | 48 × 66 px |
| 63 | Dévoreur de soufre | 68 × 46 px |
| 64 | Rampant du charnier | 72 × 32 px |
| 65 | Serpent des tranchées | 67 × 65 px |
| 66 | Charognard membraneux | 82 × 59 px |
| 67 | Sangsue de cendre | 49 × 28 px |
| 68 | Briseur cornu | 68 × 82 px |
| 69 | Coléoptère sanglant | 51 × 36 px |
| 70 | Mante de dépouille | 73 × 74 px |
| 71 | Tisseur des fosses | 83 × 58 px |
| 72 | Cracheur des failles | 57 × 64 px |
| 73 | Suture aberrante | 52 × 70 px |
| 74 | Rôdeur du vide | 65 × 68 px |
| 75 | Quadrupède des scories | 85 × 52 px |
| 76 | Tentaculaire des boyaux | 84 × 62 px |
| 77 | Démon à membres rompus | 70 × 76 px |
| 78 | Cuirassé des abysses | 94 × 79 px |
| 79 | Lame de chair | 57 × 64 px |

Les numéros sont stables ; les noms restent des libellés de revue. Une grande
stature ne détermine ni un rôle de boss ni une puissance de combat.

## Présentation et fabrication

Chaque grande source est exportée directement à son gabarit final, dans un
canevas transparent 128 × 128, sur une base de 64 px par case. L'ancrage commun
est [64, 96]. Les débordements du dessin ne définissent aucune collision.
Les corps utilisent un alpha binaire et au plus 24 couleurs. Les ombres de
contact restent sur des calques séparés et reprennent les bandes du rendu
existant. Aucun membre ou détail de créature n'est peint par le script.

Chaque modèle possède deux palettes A et B. La seconde est une correspondance
indexée des mêmes pixels : alpha, silhouette et contours sombres identiques.
Les cartes de couleurs et les pixels concernés sont conservés dans le manifeste.
Les variantes n'impliquent aucun changement de règle.

Les cinq planches montrent les dessins à taille native, sans agrandissement,
à côté du joueur accepté : corps du joueur haut de 65 px, même ancrage au sol.
Le fond est un sol de référence existant. Les fichiers courants sont :

- `artifacts/npc-monsters-review-01/demons-20/demons-60-63.png` ;
- `artifacts/npc-monsters-review-01/demons-20/demons-64-67.png` ;
- `artifacts/npc-monsters-review-01/demons-20/demons-68-71.png` ;
- `artifacts/npc-monsters-review-01/demons-20/demons-72-75.png` ;
- `artifacts/npc-monsters-review-01/demons-20/demons-76-79.png`.

Le 28 est réexporté depuis son dessin source inchangé à 67 × 65 px, soit la
hauteur du joueur. Ses anciens petits exports r01 restent intacts. Sa fiche
est `demon-28-taille-joueur.png` dans le même dossier d'aperçu ; trace de la
correction dans `demon-28-scale-revision-02.json`.

Reproduction :
`python tools/art/show_demon_gallery_review.py --review demons-style-20.json --output demons-20`.

Sources et prompts exacts : `demons-style-20.json`. Dessins sources et ombres :
`assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/`.
Contrôles : `demons-style-20-validation.json`.

## Statut de revue

Les designs de la série précédente sont appréciés globalement. Le joueur
choisira plus tard les candidats à garder ou refuser. Cette appréciation ne
valide pas chaque nouvelle apparence, palette, taille ou nom. Les accords
plus anciens restent inchangés. Aucun nouveau monstre, PNJ, boss, sort,
statistique, placement, animation ou ressource d'exécution n'est intégré.
