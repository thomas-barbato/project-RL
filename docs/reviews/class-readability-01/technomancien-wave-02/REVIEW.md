# TECHNOMANCIEN-W02 — deuxième vague par personnage

21 octobre 2026 — **les cinq modèles et leurs quinze couleurs sont refusés**.

Le joueur juge les vingt vues illisibles à 64 px et trop proches des armures
du Bastion. Il rappelle que chacune des cinq classes doit posséder sa propre
identité. Ce refus porte sur toute cette première série ; les contrôles de taille
ou de tête fixe ne constituent pas une validation de sa lisibilité. La décision
et les empreintes des dessins refusés sont dans `rejection.json`.

La reprise commence par un seul témoin d’identité, avant de redécliner une gamme.
Un manteau technique et de grandes pièces mécaniques constituent une nouvelle
proposition, sans accord présumé sur sa coupe ou ses pixels. Le Bastion accepté
reste intact. Les sections suivantes décrivent la série refusée.

Le joueur apprécie ensuite ce nouveau dessin et demande de concentrer le travail
sur le **design des armures**, en reportant toutes leurs variantes de couleur.
La reprise prépare donc cinq tenues originales autour du manteau technique et
des prothèses cuivrées. Le témoin ID01 sert au degré intermédiaire ; les quatre
autres coupes sont de nouvelles propositions à examiner en 64 × 64. Aucune
couleur supplémentaire n’est produite. Rouge, bleu et vert n’étaient pas une
palette obligatoire pour cette classe.

La planche de corps précédente est `design-candidates.json`, préparée par
`tools/art/prepare_technomancien_designs_02.py`.

**Suite du 21 octobre :** la première progression de coiffes est interrompue
après le retour « non mais c'est les memes formes que ceux du bastion, tu dois
faire different . ». Un seul nouveau témoin à capuche et col drapés est proposé
dans `../technomancien-heads-01/`. Cette reprise change la géométrie de la coiffe
et du raccord du manteau avant une nouvelle gamme. Les cinq corps ci-dessous
restent des propositions antérieures non validées ; ils ne sont pas régénérés.

Cette ancienne planche présente :

- 01 / TE-W02-D01-r01 : veste d’atelier courte, jambes peu couvertes.
- 02 / TE-W02-D02-r01 : manteau léger au-dessus des genoux, renforts et bottes.
- 03 / TE-W02-ID01-r01 : le témoin apprécié, manteau long et prothèses cuivrées.
- 04 / TE-W02-D04-r01 : manteau long renforcé de grandes protections graphite.
- 05 / TE-W02-D05-r02 : manteau rigide blindé, gros gantelets et bottes couvrantes.

Le premier D05-r01 restait trop proche d’un manteau de tissu ; il est remplacé
par la r02 aux grandes coques rigides. Les prompts et tentatives sont conservés
dans `design-prompts.json`, le dessin témoin dans `identity-prompt.json`.
Les cinq compositions gardent la tête commune exacte et restent des propositions
à revoir en 64 × 64. La planche compare aussi la nouvelle tenue témoin au Bastion
accepté et au Technomancien refusé. Aucun masque de recoloration ni nouvelle
variante colorée n’est fabriqué dans cette reprise.

Le joueur modifie ensuite la règle de tête : le torse fournit également une
apparence de tête par défaut, remplacée par celle d’un casque équipé. Les cinq
dessins actuels restent des références de corps avec leur tête commune historique ;
leurs éléments de tête assortis restent à préparer et à revoir. Le retrait du
casque rétablit la tête du torse sans changer le corps. Cette nouvelle règle
concerne aussi les cinq armures du Bastion déjà acceptées ; voir
`../head-appearance-rule.json`. Les nouveaux dessins de tête ne sont pas déduits
de l’appréciation du manteau ou d’un accord antérieur sur un corps.

Après l’accord sur le Bastion, le joueur demande de passer à la classe suivante.
Cette vague propose les cinq degrés de couverture déjà employés : très légère,
légère, intermédiaire, lourde et très lourde. Chaque modèle est présenté en
version originale et avec les protections rouges, bleues et vertes, soit
vingt vues en **64 × 64 réels**. Ces repères ne créent aucun objet ni bonus.

## Identité conservée

Le Technomancien part de CL02-r04 : un humain dont presque tout le corps est
remplacé par de la robotique. Ses compétences de départ sont l’Invocation
de l’Artilleur et Algorithmes de combat améliorés, relues dans le contenu actif.
La direction est celle d’un technicien cybernétique spécialiste des drones,
avec un châssis sombre, des carénages clairs, quelques témoins ambre et du
tissu bleu sombre. L’inspiration de prêtre-machine reste une direction générale ;
aucun symbole ou dessin sous licence n’est repris.

La mécanique visible est son corps, sans gain automatique de statistiques.
La protection ajoutée devient progressivement plus couvrante. Aucun drone,
outil porté, membre supplémentaire ou arme intégrée n’est ajouté. Les mains
restent vides pour cette revue des tenues ; leurs prises seront traitées séparément.

## Cinq propositions

- **01 — Très légère** : châssis exposé, petit gilet protecteur, poignets et pieds
  peu couverts ; les bras et les jambes restent visiblement mécaniques.
- **02 — Légère** : veste renforcée, petites protections de membres et de genoux.
- **03 — Intermédiaire** : carénage de torse, brassards, gants et protections
  de jambes plus continus ; les articulations restent dégagées.
- **04 — Lourde** : enveloppe de protection plus complète, épaules et avant-bras
  fermés, cuisses et tibias couverts.
- **05 — Très lourde** : coque industrielle complète, volumes de protection
  plus larges et bottes renforcées, en conservant l’identité du châssis robotique.

Les identifiants exacts de révision, les sources et leurs empreintes sont dans
`candidates.json`. Les versions remplacées pendant les contrôles restent tracées
dans `prompts.json` ; elles ne sont pas des candidats courants.
Le premier essai très lourd restait trop proche des modèles intermédiaires
à 64 px. **TE-W02-P05-r03** le remplace : épaules, avant-bras, cuisses et bottes
ont des coques plus larges, avec moins de divisions internes. Ses trois couleurs
ont été refaites à partir de ce corps ; les anciennes r02 ne sont pas présentées.

## Couches et lisibilité

Le torse définit tout le corps sous la tête, bras, mains, jambes et pieds inclus.
La tête vient d’une seule copie de CL02-r04, domaine Y=[0,14), strictement
identique en pixels RGBA sur les vingt vues. Le corps appartient à Y=[14,64).
La réduction est isotrope, à 48 px de hauteur pour le module de corps, avec
une ligne de pieds Y=61. Les proportions ne sont pas étirées pour remplir la case.

Les masses et les articulations utiles prennent le pas sur les petites divisions.
Les planches sont affichées à leur taille intrinsèque ; les zooms sont entiers ×3.
La revue présente des fonds sombre, clair et de pénombre, un comparatif avec
CL02-r04 et un montage sur un sol extrait de la capture de rue acceptée.
Ce montage ne valide pas l’éclairage du moteur.

Les variantes colorent les protections claires, et le gilet pour la très légère.
Le métal structurel sombre, le tissu bleu sombre, les témoins ambre et la tête
conservent leur palette. Les retouches image_gen peuvent présenter de petits
écarts de contour : les palettes et masques stricts du moteur restent à produire.

## Fabrication, contrôles et suite

Dessins et retouches : **outil intégré image_gen**, avec prompts et références
conservés dans `prompts.json`. Les exports présentant un halo de fond ont été
soumis à une retouche de détourage avant les variantes colorées. Les opérations
Python se limitent au cadrage, à la réduction, à l’assemblage et aux planches.

Sources : `assets/source-art/class-readability-01/technomancien-wave-02/`.
Aperçu : section `technomancien-colors` de la galerie courante.
Conditionnement : `tools/art/prepare_technomancien_wave_02.py`, puis appel de
`gallery` dans `prepare_class_readability_review_01.py` pour mettre à jour la page.
Le manifeste vérifie les vingt exports natifs, la tête fixe et les empreintes
des modèles du Bastion déjà acceptés ainsi que des ressources actives concernées.

Les casques, les prises d’armes, les correspondances avec les objets et le rendu
moteur restent des étapes distinctes. Les armes garderont leurs modèles communs
aux classes. **L’intégration attend toutes les armures, les casques et les bonnes
positions des mains avec les armes**, conformément à la demande du joueur.
Le Bastion accepté est conservé sans retouche ; les classes suivantes attendent
la revue de cette vague. Aucun fichier Rust, règle ou sauvegarde n’est modifié.
