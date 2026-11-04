# MONSTER-VERTICAL14-r01 — trois démarches bipèdes

Après validation des n°37–39, trois nouvelles marches haut/bas en variante 0 :

- 40 / MON-072-0 : démon trapu pâle, masse d'épaule rouge à droite de l'image
  de face et à gauche de dos, tête rentrée et bras bas.
- 41 / MON-073-0 : démon pâle voûté, manteau de chair rouge, avant-bras sombre
  du même côté anatomique dans les deux vues.
- 42 / MON-074-0 : démon sombre à deux grandes griffes dorées recourbées,
  avec crête enroulée. Le premier frontal animé réduisait trop la portée
  des bras ; la planche corrigée retrouve cette largeur avec des pas courts.

Huit figures entières par direction, à 400 ms par pas et 800 ms par cycle.
Le vrai frontal installé revient à l'arrêt. Courbes de couleur fixes pour
chaque vue, avec préparation avant la création des textures. Les sources
PNG, les poses de repos et l'éclairage du comparatif sont préservés.

Dessins via image_gen intégré. Six sources retenues, un premier essai
archivé. Prompts : `MONSTER-VERTICAL14-PROMPTS.json` et
`MONSTER-VERTICAL14-BREADTH-PROMPTS.json`. Provenance et empreintes :
`MONSTER-VERTICAL14-GENERATIONS.json`. Sources et manifeste dans
`assets/source-art/animation-polish-01/monster-vertical-14/r01/`.

Les 39 modèles précédents restent sélectionnables. Les n°40–42 sont
présentés à taille native, avec retour au repos, avant intégration.

Contrôles réussis : compilation verrouillée, neuf tests Rust de palette,
48 poses distinctes, six repos conformes aux frontaux et 198 captures natives.
Les 230 ressources figées restent intactes. Le lecteur charge les 42 sélections
et alterne face et dos à 400 ms sur les trois nouveaux modèles, sans erreur
JavaScript. Preuves dans `artifacts/animation-polish-01/monster-vertical-14/r01/` ;
bilan durable : `MONSTER-VERTICAL14-VALIDATION.json`.
