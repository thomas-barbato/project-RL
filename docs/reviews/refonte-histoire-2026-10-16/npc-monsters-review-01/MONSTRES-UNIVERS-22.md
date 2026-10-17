# UNIVERS-22 — monstres imaginés depuis notre univers

17 octobre 2026. Douze propositions initiales, numéros 92–103, dont neuf
restent candidats après suppression explicite des 94, 95 et 97. Voir
`removed-monsters-23.json` et `MONSTRES-UNIVERS-23.md` pour cette décision.

Le joueur demande de conserver les douze précédents parmi les candidats,
puis d'imaginer une série directement depuis l'univers de Project RL.
L'ajout à la liste est distinct d'une acceptation individuelle. Le regroupement
et la sélection restent prévus pour plus tard.

## Point de départ créatif

La référence est l'histoire actuelle, `docs/HISTOIRE_CAMPAGNE.md`, et son
cadrage de refonte. Les dispositifs d'asservissement brisés, les conversions,
la surveillance détournée, les forges, les rituels technologiques et la prison
du Gel Noir donnent ici les motifs. Aucun jeu extérieur n'est utilisé comme
référence de design dans ces nouveaux prompts. Le trait reste celui du joueur
et des armes actuels, avec `BD-C01-comic-master.png` comme référence graphique.

Les motifs ci-dessous servent à imaginer les dessins. Les nouvelles espèces,
leurs origines individuelles et leurs associations aux secteurs sont des
propositions, sans ajout au canon de la campagne. Elles ne remplacent pas les
premières rencontres de cultistes humains ordinaires. Toutes ces créatures
sont proposées comme troupes ordinaires ; aucun nouveau chef de secteur n'est créé.

## Les neuf idées conservées

- **92 — Traîne-carcan.** Un démon bas dont le cou porte encore l'anneau de
  contrainte cassé. Le corps reste vivant et charnu ; le fragment de machine
  rappelle la tentative d'asservissement du culte.
- **93 — Brise-entrave.** Une forme lourde portant des manchettes ouvertes et
  quelques gros maillons rompus. Les deux dessins pourraient partager le thème
  de l'asservissement sans avoir la même silhouette.
- **96 — Gorge-relais.** Un haut-parleur occupe le cou d'un corps possédé.
  Cela traduit l'usage des voix et des corps par la présence du réseau ; le
  dessin ne définit aucun sort, effet sonore ou comportement d'IA.
- **98 — Soufflet vivant.** Un torse en grands plis évoque un soufflet de forge,
  encadré de larges ferrures. La matière vivante reste majoritaire.
- **99 — Arrache-rivet.** Un bras se termine par une pince industrielle, l'autre
  reste une griffe. L'outil appartient au corps ; aucune attaque n'est définie.
- **100 — Pénitent de suie.** Un serviteur démoniaque masqué, étroit et doté de
  quatre bras, enveloppé d'un tissu simple. Sa silhouette rituelle reste celle
  d'un subalterne et ne reprend pas les attributs d'un chef.
- **101 — Encensoir rampant.** Une coque votive ajourée est greffée au dos d'une
  créature charnue. Son association à la cathédrale est proposée ; aucune fumée,
  aura ou compétence n'est créée.
- **102 — Étreigneur du gel.** De très grandes mains couvertes de glace noire
  forment des étaux. La proposition cherche à évoquer l'emprisonnement par la
  forme, sans définir une immobilisation ni des dégâts de froid.
- **103 — Gueule de banquise.** Une bête basse conserve une chair pâle, avec une
  mâchoire et deux plaques dorsales prises dans une matière de glace sombre.
  Le royaume glacé inspire l'anatomie et les matières.

## Dimensions réelles des exports

| Numéro | Libellé provisoire | Corps natif, largeur × hauteur |
| --- | --- | --- |
| 92 | Traîne-carcan | 53 × 44 px |
| 93 | Brise-entrave | 57 × 78 px |
| 96 | Gorge-relais | 45 × 68 px |
| 98 | Soufflet vivant | 74 × 52 px |
| 99 | Arrache-rivet | 58 × 72 px |
| 100 | Pénitent de suie | 36 × 68 px |
| 101 | Encensoir rampant | 77 × 54 px |
| 102 | Étreigneur du gel | 77 × 80 px |
| 103 | Gueule de banquise | 71 × 46 px |

## Revue et conservation

Dessins créés séparément avec **image_gen intégré**. Une source par modèle,
export direct vers son gabarit natif sur une base de 64 px par case, canevas
transparent 128 × 128 et ancrage [64, 96]. Alpha binaire, au plus 24 couleurs.
Les palettes A/B emploient exactement les mêmes pixels de silhouette et les
mêmes contours sombres. Les ombres de contact sont sur des calques séparés.

Les trois planches présentent les monstres à taille native, avec le joueur
accepté au même niveau de sol. Aucun acteur n'est agrandi lors de la composition.
Le fond est le sol de référence existant ; aucun nouveau décor n'est créé.

- `artifacts/npc-monsters-review-01/universe-22/demons-92-98.png` ;
- `artifacts/npc-monsters-review-01/universe-22/demons-99-102.png` ;
- `artifacts/npc-monsters-review-01/universe-22/demons-103-103.png`.

Prompts exacts, références, sources, intentions et empreintes :
`universe-monsters-22.json`. Sources et ombres conservées dans
`assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/`.
Contrôles historiques avant retraits : `universe-22-validation.json`.
Contrôles après suppressions : `universe-23-validation.json`.

Reproduction :
`python tools/art/show_demon_gallery_review.py --review universe-monsters-22.json --output universe-22`.

Les apparences, noms, palettes, gabarits et nouvelles hypothèses d'origine
restent à sélectionner. Les accords précédents restent conservés. Aucun
acteur, texte joueur, règle de combat, compétence, effet, animation ou rencontre
n'est intégré au jeu.
