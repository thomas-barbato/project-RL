# CULTES-24 — cultistes possédés et morts-vivants

Propositions du 17 octobre 2026, préparées à la demande du joueur. Les anciens
cultistes sont réunis au [catalogue complet](CANDIDATS-MONSTRES.md), sans changer
leurs sources ni leurs validations. Ce nouveau lot comporte seize dessins et
leurs deux palettes à sélectionner ultérieurement.

Les cultistes possédés reprennent la capuche, le masque physique ivoire et
l'uniforme charbon/bordeaux. Les transformations portent sur la posture et les
membres. Les zombies alternent adultes debout, rampant et silhouette massive.
Les esprits gardent des contours lisibles et des attitudes de supplice. Les
squelettes sont des variantes de morts-vivants, et les flammes font partie du
dessin statique proposé. Aucune compétence ni règle de vol, de brûlure ou
animation n'est définie ici.

Le trait suit les sources acceptées du joueur et de ses armes : contours
sombres, grandes masses et ombres simples. Les tailles varient ; les dimensions
ci-dessous comprennent toute la silhouette visible, flammes incluses. Les ombres
de contact sont des fichiers séparés. A/B conservent le même alpha et les mêmes
contours. Affichage natif uniquement, base de 64 px par case.

| Numéro | Code | Nom provisoire | Famille | Corps natif |
| --- | --- | --- | --- | --- |
| 116 | CULT-04-r01 | Adepte distordu | Cultistes possédés | 41 × 65 px |
| 117 | CULT-05-r01 | Psalmodiant possédé | Cultistes possédés | 42 × 70 px |
| 118 | CULT-06-r01 | Dévot rampant | Cultistes possédés | 41 × 43 px |
| 119 | CULT-07-r01 | Adepte aux quatre bras | Cultistes possédés | 43 × 68 px |
| 120 | ZOM-01-r01 | Détenu réanimé | Zombies | 38 × 64 px |
| 121 | ZOM-02-r01 | Ouvrier mort-vivant | Zombies | 36 × 62 px |
| 122 | ZOM-03-r01 | Cadavre rampant | Zombies | 56 × 30 px |
| 123 | ZOM-04-r01 | Boursouflé | Zombies | 59 × 74 px |
| 124 | ESP-01-r01 | Âme entravée | Esprits torturés | 26 × 72 px |
| 125 | ESP-02-r01 | Hurleur sans voix | Esprits torturés | 45 × 70 px |
| 126 | ESP-03-r01 | Esprit recousu | Esprits torturés | 27 × 66 px |
| 127 | ESP-04-r01 | Supplicié du gel | Esprits torturés | 41 × 76 px |
| 128 | SQUE-01-r01 | Squelette de maintenance | Squelettes | 40 × 64 px |
| 129 | SQUE-02-r01 | Squelette entravé | Squelettes | 42 × 66 px |
| 130 | BRA-01-r01 | Charbon ardent | Créatures enflammées | 62 × 64 px |
| 131 | BRA-02-r01 | Bête des braises | Créatures enflammées | 58 × 48 px |

## Planches du nouveau lot

![Propositions natives demons-116-119](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/cultes-24/demons-116-119.png)

![Propositions natives demons-120-123](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/cultes-24/demons-120-123.png)

![Propositions natives demons-124-127](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/cultes-24/demons-124-127.png)

![Propositions natives demons-128-131](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/cultes-24/demons-128-131.png)

## Sources et reproduction

Génération par l'outil intégré `image_gen`, seize appels distincts. Les sources
sont conservées sous `assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/`.
Les prompts exacts, références, empreintes, codes et exports se trouvent dans
[cultists-zombies-spirits-24.json](cultists-zombies-spirits-24.json).

```powershell
python tools/art/show_demon_gallery_review.py --review cultists-zombies-spirits-24.json --output cultes-24
python tools/art/show_monster_catalogue.py
python tools/art/validate_monster_catalogue.py
```

L'ajout à la liste ne vaut pas validation visuelle individuelle, du nom ou des
règles. Aucun contenu nouveau n'est intégré au jeu. Les nouveaux exports n'ont
pas de version 32 px. Le catalogue conserve les pixels des anciens exports
natifs et distingue les apparences déjà acceptées.
