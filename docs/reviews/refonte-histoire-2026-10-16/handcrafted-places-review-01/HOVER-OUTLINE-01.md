# HOVER-OUTLINE-01 r01 — contour de la cible survolée

18 octobre 2026. Proposition visuelle à examiner à taille native avant activation.
La demande conserve les ajouts CAMP-NPC-01 et propose une règle générique de
survol suivant la silhouette des cibles.

## Ajouts PNJ validés et intégrés

Le joueur confirme : « alors j'aime (et il faut garder les ajouts) ». Les quatre
pictogrammes de dialogue/services et la respiration CAMP-NPC-01 r01 sont donc
activés dans le démarrage normal du client. Les dessins acceptés sont conservés.

Le joueur précise qu'il attendait aussi le symbole du PNJ de quête. L'officier
affiche maintenant le « ! » déjà employé par l'interface, à côté de sa bulle de
dialogue. Les véritables marqueurs de quêtes gardent la priorité et leurs états
existants. Ce repère de contact de mission ne crée pas de quête ni de progression
fictive dans le journal. Il n'ajoute aucun dialogue.

Le joueur ajoute : « et les symboles au dessus des pnj doivent être un peu
animées aussi, comme avant :) ». Les pictogrammes reprennent le flottement des
marqueurs existants : 0 à 3 px en 2,4 secondes, sans déplacement logique. Ils
suivent aussi le déplacement affiché de l'acteur. Les mouvements réduits figent
la respiration et le flottement, tout en conservant les symboles visibles.

## Contour proposé — validation visuelle en attente

- Un trait extérieur de **2 px natifs**, sans remplissage de la cible ni halo.
- **Vert** pour le joueur, les alliés, les familiers contrôlés et les PNJ de
  services ou de quête. **Rouge** pour un ennemi. La relation actuelle est
  consultée, y compris le contrôle d'un familier ; la couleur de son dessin
  n'influence pas cette décision.
- Une seule cible survolée. Aucun clic, tour, coût ou changement de sélection
  tactique. Une case vide, une cible cachée ou un objet ne reçoit pas de contour.
- Le joueur précise ensuite que **seules les icônes de statuts doivent apparaître
  au-dessus des ennemis**. Les anciens « ! » d'alerte, yeux/oreilles de perception
  et marqueurs de quête sur un ennemi sont donc retirés du plateau. Les données
  de perception, les champs F2 et les capteurs restent disponibles ; les symboles
  de dialogue, mission et services des PNJ sont conservés.
- Le contour suit le dessin complet et sa pose actuelle, armes comprises, avec
  les vrais points d'appui des monstres plus grands. L'ombre au sol, les
  pictogrammes et le rectangle transparent du sprite sont exclus.
- Le décor et les acteurs placés devant peuvent le masquer comme le corps.
  Il ne révèle pas une silhouette cachée derrière un obstacle. Le brouillard et
  la visibilité réelle restent prioritaires. Portraits et inspection inchangés.

Le contour est générique : il utilise le rendu composé de l'acteur, sans
dessiner une variante de chaque PNJ ou monstre. Le tampon GPU est réutilisé et
seule la cible survolée est traitée. Aucun dessin source n'est modifié.

L'activation du contour reste limitée à `--ui-cold-hover-outline`. La galerie
courante montre cinq scènes du client et une vérification isolée de silhouettes
humaines, d'une arme tenue, d'un drone et d'un grand démon. Les créatures placées
dans ces captures sont des témoins de revue et ne sont pas ajoutées au camp.

`AGENTS.md` demande de présenter les effets visuels modifiés et de recueillir une
validation explicite avant intégration. L'accord sur les ajouts PNJ ne vaut donc
pas accord sur ce nouveau contour. Les preuves seront conservées dans
`hover-outline-01.json` ; les captures reproductibles restent dans `artifacts/`.
