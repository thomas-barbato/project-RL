# BASTION-W01 — première vague par personnage

21 octobre 2026 — **vague validée visuellement** : cinq modèles et quinze
variantes rouge, bleue et verte. Accord : `acceptance.json`.

Le joueur répond « ok je valide :) » après la planche des vingt vues natives.
L’accord porte sur P01-r01 à P04-r01 et P05-r03, leurs trois couleurs et la
tête commune telle qu’assemblée. Les exports exacts sont conservés dans
`assets/source-art/class-readability-01/bastion-wave-01/accepted/`.
La très lourde détaillée P05-r02 et les anciens essais de torses ne font pas
partie de ce choix. Toute simplification future des quatre premières tenues
sera une nouvelle révision ; ses pixels ne sont pas validés par cet accord.
Les correspondances avec les objets, les prises d’armes et le rendu moteur
restent à préparer. Cette étape enregistre le choix artistique.
Le joueur précise ensuite que l’intégration attendra toutes les armures,
les casques et les bonnes positions des mains avec les armes : cette vague
validée reste donc une référence de fabrication jusqu’à cette étape commune.

Le joueur demande de fabriquer par vagues, une à la fois par personnage,
et de valider chaque vague avant la suivante. La cible de 5 à 6 modèles par
type reste un objectif progressif. Cette première vague concerne le Bastion,
corps massif et humain renforcé associé à Frappe de rupture et Vitalité améliorée.
Les armes restent communes aux classes. Les autres personnages attendent la revue.

## Nouvelle règle de tête — préparation supplémentaire

Le joueur demande ensuite que l’armure de torse couvre aussi la tête par défaut,
avec remplacement visuel par un casque lorsqu’il en équipe un. Il précise
explicitement que cela concerne aussi le Bastion. Les cinq corps validés
P01-r01 à P04-r01 et P05-r03 sont conservés ; chacun reçoit une proposition
de tête assortie dans `../bastion-heads-02/`, validée en 64 × 64 le 21 octobre
par « ok j evalide ». L’accord exact est dans `../bastion-heads-02/acceptance.json`.
Retirer un casque rétablit
l’apparence de tête du torse. Le casque ne change pas le corps.

Les vingt compositions acceptées restent conservées avec leurs empreintes
d’origine ; les cinq nouvelles têtes disposent de leur propre accord distinct.
Voir `../head-appearance-rule.json`. Aucune intégration n’est effectuée ici.

## Règle utilisée lors de la validation : corps complet, tête fixe

Le joueur corrige les premiers essais : l’armure de torse couvre tout le corps
sauf la tête. Les bras, avant-bras, mains/gants, pantalon, jambes et bottes
doivent donc évoluer avec la tenue. La progression va du très léger au très
lourd en conservant le thème de la mêlée. Cette règle est directement demandée ;
elle ne change pas les statistiques ou le contenu distribué.

La tête CL01-r04 sert de référence commune pour cette proposition. Le calque
est copié avec exactement les mêmes pixels RGBA sur les cinq corps : l’implant
ne peut plus varier lors d’un changement d’armure. Ces pixels de tête sont
validés dans les compositions de cette vague ; cet accord ne s’étend pas
aux autres classes, casques ou postures.

## Progression complète produite

- 01 / BA-W01-P01-r01 : très légère, textile et protections souples.
- 02 / BA-W01-P02-r01 : légère, renforts de torse, mains, genoux et chevilles.
- 03 / BA-W01-P03-r01 : intermédiaire, plaques et protections de membres.
- 04 / BA-W01-P04-r01 : lourde, bras et jambes presque entièrement couverts.
- 05 / BA-W01-P05-r02 : très lourde, blindage de mêlée intégral, gants fermés.

Ces libellés sont des repères de revue. Aucun nom d’objet, palier chiffré ou bonus
n’est ajouté. Les associations avec les objets du catalogue restent à définir.

## Fabrication présentée et retenue : simplification et couleurs

Le joueur demande si les modèles sont trop détaillés pour 64 px. La très lourde
BA-W01-P05-r03 réduit les petites divisions et retire les textures fines au profit
de grandes masses. La comparaison avec la r02 est présentée à taille native,
sur trois fonds et en zoom entier ×3. La r03 est retenue pour le cinquième modèle.
Les quatre autres tenues sont validées dans leur version détaillée présentée.

Le premier essai de couleurs ne portait que sur la r03 très lourde. Le joueur
précise ensuite que **chaque armure** doit avoir ses variantes : rouge, bleue
et verte. Le catalogue courant présente donc cinq modèles × trois couleurs,
soit quinze variantes et vingt vues en comptant les originales. Les trois
couleurs de la très lourde sont réutilisées, douze autres sont produites.
La très légère colore son gilet protecteur et ses renforts clairs, sans ajouter
de plaques rigides ni recolorer la peau ou le pantalon ordinaire. Les autres
modèles recolorent leurs panneaux et plaques de protection.
Toutes les plaques utilisent une seule teinte, avec des valeurs d’ombre et de
lumière ; peau, tissus et articulations gardent leur palette. Les têtes sont
identiques au pixel près. Ce sont trois palettes d’un même modèle, sans nouvelles
statistiques. Les masques de plaques et palettes du moteur restent à produire
après revue. Les petites différences de silhouette entre les dessins de couleur
sont mesurées dans `all-armor-colors.json` (et `colors.json` pour le premier essai) ;
ces images ne sont pas présentées comme des
recolorations de production strictement identiques à chaque pixel.

Le rappel du joueur impose de juger chaque tenue et chaque variante en 64 × 64
réels. La planche principale conserve cette taille sans redimensionnement CSS ;
les zooms entiers servent uniquement à inspecter les pixels. La très lourde
simplifiée donne actuellement les masses les plus nettes ; une éventuelle
simplification des quatre premières sera soumise séparément. Le contrôle natif des couleurs conserve
la tête commune exacte. Les contours des variantes sont comparés visuellement,
sans considérer un score de recouvrement comme une preuve de recoloration exacte.

## Historique de l’implant crânien

Le joueur apprécie l’idée de l’implant mais trouve la petite plaque du front
difficile à comprendre. CL01-r02 a été écarté lors du contrôle : son module
à l’oreille évoque un casque audio. CL01-r03 propose des plaques à la tempe
et à la pommette mais reste trop discrète à 64 px. CL01-r04 essaie une zone
métallique plus large et continue autour d’un œil mécanique. Les pixels de tête
utilisés dans les vingt compositions sont retenus avec cette vague.

## Historique des trois torses seuls — remplacé

- 01 / BA-W01-01-r01 : Tenue du technicien, `core:veste_de_veille`.
- 02 / BA-W01-02-r01 : Tenue Fantôme, `core:combinaison_fantome`.
- 03 / BA-W01-03-r01 : Armure Bastion, `core:plastron_bastion`.

Ces interprétations utilisaient la tête r01 et conservaient à tort les mêmes
jambes, gants et bottes. Elles ne sont plus les candidats courants de la vague.
Aucun nouvel objet, nom, chiffre ou effet de jeu n’est ajouté.
Le tablier refusé reste exclu. Les têtes et accessoires seront choisis
séparément ; cette vague n’impose pas un casque par torse.

## Fabrication et limites

Dessins et retouches sémantiques : outil intégré image_gen. Les prompts exacts
et références sont dans `prompts.json`. Sources durables dans
`assets/source-art/class-readability-01/`, sorties de revue dans
`artifacts/class-armor-review-01/bastion-wave-01/`.

Le script courant est `tools/art/prepare_bastion_progression_01.py`. Les corps
sans tête sont générés par image_gen, cadrés avec alpha conservé et réduction
isotrope Lanczos en 50 px, puis assemblés avec la tête commune dans une case
64 × 64. Le domaine de la tête est [0,12) en Y, celui du corps [12,64).
Le raccord de cou est Y=12, l’axe de tête X=32 et la ligne d’appui Y=61.
La silhouette assemblée mesure 58 px. Les contrôles comparent les pixels de tête
et vérifient des régions distinctes sur avant-bras/mains, jambes et bottes.
Les prises d’armes et poses finales restent à mesurer et à produire.

Les scènes sont des montages sur un extrait vide de la capture de rue acceptée,
pas des validations du nouvel éclairage dans le moteur. Aucune ressource active,
statistique, sauvegarde ou règle n’est changée. Les empreintes de contrôle et
mesures sont dans `progression.json`, `readability-pilot.json` et `colors.json`.
Les prompts sont dans `progression-prompts.json`, `readability-pilot-prompt.json`
et `color-prompts.json`. Aucun test Cargo n’est nécessaire pour
ces sources et aperçus isolés. Les autres classes attendent la revue du joueur.

L’extension de couleurs utilise `all-armor-color-prompts.json` et le manifeste
`all-armor-colors.json`. `tools/art/prepare_bastion_all_colors_01.py` prépare
les vingt vues natives et les comparaisons par modèle ; exécuter ensuite
`prepare_bastion_progression_01.py`, puis `prepare_class_readability_review_01.py`
pour réutiliser la galerie courante. Aucun nouveau modèle statistique n’est créé.
