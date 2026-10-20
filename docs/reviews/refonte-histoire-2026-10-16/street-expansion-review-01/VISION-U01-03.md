# Masquage commun des décors — VISION-U01-03

19 octobre 2026 · proposition visuelle, non activée en partie normale.

Le joueur précise que les aplats noirs sur les bâtiments et le contour de
portée le gênent tous les deux. Il rappelle que les décors intérieurs et
extérieurs doivent suivre le même traitement. VISION-U01-02 corrigeait une
grille d'intérieur ; elle ne réglait pas ce défaut général. Sa page présentait
en premier des vues témoins presque inchangées, ce qui rendait la comparaison
peu utile. Les intérieurs corrigés sont désormais placés en tête de cette page.

## Cause constatée

Le fond peint recevait un masquage par cases, puis un second passage après
les toits. Le bord de ce second masque pouvait réassombrir des décors déjà
découverts. De plus, plusieurs objets complets étaient regroupés dans le fond
de carte : la visibilité de la grille les découpait comme le sol. Une simple
frange plus douce ne suffit pas à supprimer ces découpes.

## Proposition

Un passage de brouillard commun au camp et à la rue remplace cette accumulation
dans le diagnostic. Le terrain découvert reste lisible à 45 % hors de la vue
actuelle, puis retrouve son éclairage normal à travers une transition progressive
de portée. Le sol inconnu reste noir.

Les ressources statiques isolées possèdent aussi un index de silhouette, dérivé
de leur alpha et de leur placement natif. Une observation de cette ressource
autorise son image complète, sans découvrir les cases, objets interactifs ou
acteurs placés derrière. Le même principe s'applique au mobilier du camp et aux
objets de la rue : la règle ne dépend pas d'une catégorie intérieur/extérieur.
Les 66 ressources de rue et 23 accessoires de camp indexés conservent leurs pixels.
Les toits ouvrables gardent le fonctionnement déjà accepté.

Une pièce entière est un assemblage, pas un accessoire. Les deux images composites
d'intérieur ne sont donc autorisées comme silhouette complète que sous leur toit
fermé. Dès que le toit est masqué, cette autorisation est coupée : une façade
aperçue ne doit pas dévoiler toute la pièce. Le sol et les obstacles restent
soumis à leur perception réelle. Cette revue ne promet pas la disparition de tout
noir : l'espace inexploré doit rester caché.

Les anciens repères d'exposition des deux nouveaux toits omettaient le bas de
leur façade sud. Le diagnostic utilise la silhouette complète comme témoin
commun du corps fermé et du toit, afin d'éviter une bande noire sous un toit
pourtant aperçu. Les cellules de mémoire du sol ne sont pas étendues.

Les index techniques et leurs manifestes sont dans
`assets/source-art/first-sector-review-01/u01-expansion-r06/visibility-surfaces-*-03.*`.
Ils sont reproductibles depuis les sources isolées, sans découpe dans une capture.
Les empreintes des sources sont enregistrées. Les textures et leurs états sont
mis en cache ; seules les données d'observation changées sont envoyées au GPU.

## Limites et vérification

Cette proposition exige une compilation debug et `--vision-unified`. La carte de
partie normale, les sauvegardes, les calculs de ligne de vue, la portée, les acteurs,
le combat et les lumières ne changent pas. Le correctif de grille VISION-U01-02 est
présent dans les deux côtés de la comparaison pour isoler le changement de rendu.

Les huit paires proviennent du framebuffer du moteur, dans le même état de partie,
avec portes et horloge identiques. Le contrôle de lecture seule compare les
empreintes de simulation ; les cases restent à 64 px. La première vue présente
la rue affectée, avec bascule avant/après au même emplacement. Le camp, la cour,
les deux pièces, une porte fermée, la prison et un angle de l'enceinte complètent
le contrôle. Voir `vision-checks-03.json`.

Deux tests ciblés vérifient que changer un terrain caché ne modifie pas le champ
de brouillard ni la simulation, et qu'un objet statique jamais observé n'est pas
autorisé par l'observation d'un autre. Les compilations et captures ne constituent
pas une acceptation visuelle du joueur.

```powershell
python tools/reviews/build_visibility_surfaces_review.py
cargo build --locked
target/debug/project-rl.exe --ui-cold-street-expansion artifacts/street-expansion-review-01/vision-03/engine --vision-layout --vision-unified
python tools/reviews/show_world_visibility_review.py
```

Aucun accord sur VISION-U01-03 n'est acquis. Le joueur propose ensuite une
direction plus simple : afficher tout le décor dans le cercle, indépendamment
des obstacles. [VISION-U01-04](VISION-U01-04.md) présente cette variante courante.
Conformément à AGENTS.md, le rendu modifié doit être accepté explicitement avant
activation normale.
