# UNIVERS-23 — retraits demandés et nouvelles formes

17 octobre 2026. Revue visuelle, sans intégration au jeu.

## Choix explicite du joueur

Les images jointes identifient le 97 — Œil décroché, le 95 — Brancard rampant
et le 94 — Greffé de triage. Le joueur demande leur suppression directe.
Leurs sources, copies de génération, palettes natives et ombres sont supprimées.
Les anciennes planches sont retirées et les neuf autres modèles sont présentés
sur des planches reconstruites. La trace textuelle et les empreintes restent
dans `removed-monsters-23.json`, sans archive graphique de ces trois modèles.

Les neuf conservés sont **92, 93, 96, 98, 99, 100, 101, 102 et 103**. Ils
rejoignent la liste comme candidats, sans validation individuelle de leurs
apparences, noms, gabarits, palettes ou règles. Le refus porte sur les trois
dessins montrés ; aucune interdiction générale des hybrides n'est déduite.

## Nouvelle série

Les dix dessins disponibles poursuivent les pistes de notre univers :
conversions aberrantes, contrainte, entrelacs, matières des forges, membranes
des profondeurs et faune du Gel Noir. Leurs origines et associations sont des
propositions, sans canonisation. Le trait reste celui du joueur et des armes.

- **104 — Rebut de cuve :** une forme basse et lourde, à la tête enfouie sous
  un pli du corps, pour une petite troupe issue des conversions.
- **105 — Dévié :** un corps adulte asymétrique, dont les épaules et le torse
  sont décalés, avec une silhouette plus fine.
- **106 — Ligaturé :** de larges bandes vivantes forment des liens sur le dos
  et les épaules, sans équipement posé dessus.
- **107 — Cocon marcheur :** une grande enveloppe organique sur quatre pattes,
  avec seulement quelques grandes séparations lisibles.
- **108 — Oreillard des failles :** une forme étroite encadrée par deux grands
  lobes, pour évoquer la perception sans appareil greffé.
- **109 — Tresse rampante :** un organisme bas à six appuis, avec de grands
  entrelacs vivants qui rappellent un réseau.
- **111 — Porte-lest :** un corps penché sous une masse sombre soudée à une
  épaule et au dos, inspirée des matières accumulées dans les forges.
- **112 — Voilé des cryptes :** des membranes repliées évoquent un manteau
  autour d'un corps étroit, sans lui donner un rôle de chef.
- **113 — Étoile des fosses :** une proposition non humanoïde à cinq appuis,
  asymétrique et ramassée au sol.
- **115 — Veilleur du givre :** une silhouette haute et mince, avec une
  grande plaque de glace sombre qui suit l'échine.

Les briefs **110 et 114 n'ont produit aucune image** après deux tentatives
chacun, bloquées par la modération de sortie du générateur. Ils restent hors
de la liste des candidats dessinés ; ce n'est pas un refus du joueur. Les
prompts exacts et tentatives restent dans le manifeste. Les numéros ne sont
pas réattribués et les trous de numérotation restent explicites.

## Dimensions natives

| Numéro | Libellé provisoire | Corps natif, largeur × hauteur |
| --- | --- | --- |
| 104 | Rebut de cuve | 58 × 38 px |
| 105 | Dévié | 52 × 70 px |
| 106 | Ligaturé | 61 × 48 px |
| 107 | Cocon marcheur | 54 × 56 px |
| 108 | Oreillard des failles | 38 × 64 px |
| 109 | Tresse rampante | 53 × 40 px |
| 111 | Porte-lest | 73 × 76 px |
| 112 | Voilé des cryptes | 42 × 72 px |
| 113 | Étoile des fosses | 52 × 38 px |
| 115 | Veilleur du givre | 40 × 82 px |

## Fabrication et contrôle

Sources créées séparément avec **image_gen intégré**, une source par modèle,
avec la source acceptée du joueur armé comme référence du trait uniquement.
Export direct vers les gabarits finaux sur une base de 64 px par case :
canevas transparents 128 × 128, ancrage [64, 96], alpha binaire, au plus
24 couleurs. Les palettes A/B conservent exactement la silhouette, l'alpha
et les contours sombres. Les ombres restent des calques séparés.

Les planches natives sont dans `artifacts/npc-monsters-review-01/universe-23/` :

- `demons-104-107.png` ;
- `demons-108-112.png` ;
- `demons-113-115.png`.

Le joueur accepté est montré à la même échelle ; aucun acteur n'est agrandi
pour composer les planches. Le sol de référence existant est réutilisé.

Prompts, sources, intentions et empreintes : `universe-monsters-23.json`.
Liste : `monster-candidate-list.json` et `CANDIDATS-MONSTRES.md`.
Contrôles des suppressions, de la conservation et des exports :
`universe-23-validation.json`.

Reproduction :
`python tools/art/show_demon_gallery_review.py --review universe-monsters-23.json --output universe-23`.

Les dix nouvelles apparences, les noms, tailles, palettes et hypothèses de
conception attendent la sélection ultérieure. Les accords antérieurs restent
conservés. Aucun acteur, effet, texte joueur, sort, statistique ou rencontre
n'est intégré au jeu.
