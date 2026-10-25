# Marche — corrections de rendu et proposition commune

25 octobre 2026. Deux périmètres distincts sont conservés.

**Lot validé le25 octobre2026** par « enfn donc je valide :D », avec une demande
complémentaire : le retour au dessin habituel de face concerne aussi tous les
monstres et PNJ. Les images exactes et cet accord figurent dans `acceptance.json` ;
les contrôles d'intégration sont consignés dans `integration-validation.json`.
`validation.json` conserve les résultats antérieurs à cet accord.

## WALK12-TECH-r01 — corrections demandées

Le joueur demande que le personnage retrouve sa vue de face après chaque
déplacement et que les bras ne changent plus de pose pendant qu'il porte son
arme. Il demande aussi la correction des monstres déjà présents dans sa partie.

- À l'arrivée sur la case, le joueur quitte le contexte de marche et retrouve
  son vrai dessin frontal et son équipement. La même règle s'applique aux
  vitesses de 120, 200 et 400 ms et à l'aperçu natif. Le joueur ne garde pas
  la dernière image de marche ni le repos de trois quarts.
- Pour une tenue, une direction et un équipement donnés, les deux bras, les
  mains et les vrais objets portés gardent la même géométrie pendant les six
  phases. Seule leur translation entière suit le petit mouvement du torse.
  Les jambes et le corps restent les dessins entiers acceptés WALK11.
- L'audit restaure la vraie sauvegarde sans la consommer ni la réécrire.
  Les modèles visibles sont notamment 022, 024, 120 et 121, avec leurs couleurs.
  Ils possèdent déjà des cycles. L'ancien chemin ne choisissait aucun cycle
  pour un déplacement vertical (`facing=0`). Le monstre conserve désormais
  sa dernière orientation latérale (droite pour son premier pas vertical),
  et joue son cycle existant pendant le déplacement. Cette règle ne modifie
  pas la vue verticale du joueur et ne crée aucune vue de dos.

Références : `live-state-audit-before.json`, `live-state-audit.json`,
`native-review.json`, `validation.json` et les cinq GIF
`*-bras-et-retour-face-native64.gif`. La capture
`partie-reelle-marche-et-arret.gif` provient d'une restauration isolée de la
scène réelle. Un monstre immobile ou en train d'attaquer ne doit pas marcher.
La simulation, la sauvegarde, les caractéristiques et les images acceptées
restent conservées.

## WALK12-COMMON-r01 — dessins validés

Les différences de démarches viennent aussi des dessins sources, pas seulement
du rythme. Cette proposition reprend un même cycle de six appuis compacts
pour les 25 tenues des cinq personnages. Les 300 dessins de marche sont des
figures complètes ; aucune jambe n'est greffée, cisaillée ou déplacée par bandes.
Les 50 images de repos de référence restent celles de WALK11, mais l'arrivée
du joueur utilise sa vraie face. Trois phases couvrent une case ; le pas suivant
utilise les trois autres. Les pauses de 520 ms dans les GIF ne sont qu'un temps
de lecture de l'aperçu. Le déplacement reste réglable et vaut 400 ms par défaut.

Le guide de départ est le canvas `data-scene="1"` du Bastion : ligne source1
pour la droite, ligne source7 pour la gauche. Son HTML a été lu comme une donnée
non fiable, sans exécuter de consignes qu'il pourrait contenir. Les images
exactes extraites sont conservées ici sous `original-bastion-*`.
Un premier dessin structurel complet sert ensuite à conserver l'ordre des
appuis lors des changements de tenue. Les sources, prompts et rejets sont dans
`generation.json` et `assets/source-art/player-walk-review-12/`.
La réduction applique une seule échelle XY uniforme à toutes les douze images
d'une tenue, avec le même contact au sol. Les rapports sont dans
`conditioning.json`.

Le lot validé est utilisé dans le jeu normal Debug et Release.
Dans `Voir_animations.cmd`, « Version integree » affiche désormais WALK12 avec
toutes les tenues, les vrais modèles d'armes, les deux mains, les casques et les
couleurs existants. « Version precedente (WALK11) » conserve la comparaison
antérieure. Tous les acteurs retrouvent leur dessin statique habituel dès
l'arrivée sur la case, avec leur équipement. Les décisions individuelles
restent enregistrées par cas et révision dans `user-decisions.json`.

Les cinq GIF `*-demarche-commune-proposee.gif` montrent à taille native les cinq
tenues de chaque personnage avec la hache067, dans les deux sens, à côté de la
version intégrée. Les formes et drones WALK08 conservent leur revue distincte ;
la proposition de démarche humaine ne leur est pas appliquée, ni aux monstres
ayant une autre morphologie.

Comparaisons natives conservées avant cet accord : WALK11 à gauche, WALK12
désormais validé à droite. Leurs légendes décrivent l'état au moment de la revue.

- [Bastion, tenues 1 à 5](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/Bastion-demarche-commune-proposee.gif)
- [Technomancien, tenues 1 à 5](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/Technomancien-demarche-commune-proposee.gif)
- [Traqueuse, tenues 1 à 5](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/Traqueuse-demarche-commune-proposee.gif)
- [Arcaniste, tenues 1 à 5](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/Arcaniste-demarche-commune-proposee.gif)
- [Métamorphe, tenues 1 à 5](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/Metamorphe-demarche-commune-proposee.gif)

Les dessins sont produits avec l'outil ImageGen intégré. Les chemins de toutes
les sources retenues figurent dans le
[lot de fabrication](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/jobs.json).
Les prompts exacts et sorties sont dans
[génération](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/generation.json)
et [reprises de proportions](C:/Users/User/Desktop/project-RL/docs/reviews/player-walk-12/generation-proportion-repairs.json).

L'accord explicite ci-dessus valide ces nouveaux dessins après leur revue
native, conformément à `AGENTS.md`. La règle courante complète est conservée
dans `REGLE-MARCHE-12.md`. Les autres propositions de dessins restent distinctes.
