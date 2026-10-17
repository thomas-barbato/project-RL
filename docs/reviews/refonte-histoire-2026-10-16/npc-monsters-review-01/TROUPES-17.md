# TROUPES-17 — petits démons ordinaires

17 octobre 2026. Propositions visuelles, sans intégration au jeu.

Le joueur demande davantage de petites troupes démoniaques, de type « chair
à canon », avec plusieurs modèles et des variantes de couleur. Les boss de
secteur conservent leur rôle de chefs et les 20–21 leur proposition d'élites.
Cette série vise uniquement des démons ordinaires ; les chiffres de combat,
compétences, rencontres et densités ne sont pas déterminés par les dessins.

La sélection contient douze bases et douze variantes cosmétiques. Les deux
palettes de chaque modèle gardent exactement la même silhouette, le même
alpha et les mêmes contours sombres. Les noms sont des libellés de revue.

| Numéro | Dessin proposé | Corps natif, largeur × hauteur |
| --- | --- | --- |
| 28 | Démon voûté, r02 à la stature du joueur | 67 × 65 px |
| 29 | Larve de chair | 35 × 20 px |
| 30 | Coléoptère des scories | 38 × 26 px |
| 31 | Mante des conduits | 46 × 36 px |
| 32 | Palpeur aveugle | 37 × 40 px |
| 33 | Ver du gel noir | 41 × 24 px |
| 34 | Diablotin des braises | 42 × 41 px |
| 35 | Gueule rampante | 44 × 32 px |
| 36 | Griffeur des fosses | 41 × 52 px |
| 37 | Tique de charnier | 33 × 24 px |
| 38 | Rampant abyssal | 48 × 32 px |
| 39 | Rôdeur membraneux | 51 × 44 px |

Les 29–39 occupent des canevas transparents de 64 × 64 px, avec l'ancrage
commun [32, 60]. À la demande suivante du joueur, le 28 est réexporté
directement de sa grande source à la stature du joueur, sans agrandir son
ancien petit PNG : canevas 128 × 128, ancrage [64, 96]. Sa source et ses PNG
r01 restent conservés. Voir `demon-28-scale-revision-02.json`.
Les ombres se composent sous les corps, sur des calques
séparés. La planche ne redimensionne aucun personnage : les démons, leurs
variantes et la tenue acceptée du joueur sont collés à leur taille native.
Le fond provient d'un sol de référence existant, sans nouvelle création de décor.
Aucun zoom n'est fabriqué pour cette revue.

Les sources sont générées séparément par **image_gen intégré**, avec des
références de style acceptées. Le conditionnement exporte directement chaque
grand dessin à sa taille finale, avec alpha binaire et seize couleurs maximum.
Le 33 réserve trois teintes dérivées des plaques bleues de sa source pour
préserver leur lecture sur le petit corps charnu. Les variantes B sont des
correspondances de palette indexée : leur carte de couleurs et le nombre de
pixels concernés sont conservés dans le manifeste. Aucun membre n'est redessiné.

Les planches reproductibles sont :

- `artifacts/npc-monsters-review-01/petits-demons-28-33.png` ;
- `artifacts/npc-monsters-review-01/petits-demons-34-39.png`.

## Magicien possédé conservé séparément

Le joueur apprécie la source `TROUP-04-r01.png`, d'abord présentée comme patient
altéré, et propose qu'elle serve de magicien possédé. Le dessin reste identique.
Son export de revue mesure 32 × 54 px dans le même canevas 64 × 64. La fiche
`magicien-possede-25.png` montre les deux palettes à taille native, avec ombre
et joueur de référence. Le dessin source est apprécié ; les nouveaux exports,
la variante de palette et les sorts ne sont pas considérés comme déjà validés.
La demande exacte et l'empreinte sont dans `magicien-possede-25-direction.json`.

Les cinq premiers essais hors sujet 22–24, 26 et 27 restent des sources de
travail, sans approbation déduite. Ils ne sont pas inclus dans les planches
démoniaques courantes. Le manifeste conserve leur origine et leur statut.

Fabrication : `python tools/art/show_rank_and_file_review.py`.
Prompts, sources et exports : `petites-troupes-17.json`.
Contrôles : `troupes-17-validation.json`.
Ces contrôles décrivent les exports r01 historiques ; la correction de taille
du 28 est vérifiée dans `demons-style-20-validation.json`.
