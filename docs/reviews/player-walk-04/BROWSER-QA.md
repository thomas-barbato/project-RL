# Revue navigateur — WALK04-r01

Contrôles exécutés dans le navigateur intégré sur l'aperçu HTTP local.

- `armes.html` : chargement des six atlas, fusil et hache à deux mains,
  trois quarts gauche/droite, face en haut/bas, vue native et zoom ×2.
  Capture conservée : `artifacts/player-walk-review-04/WALK04-armes-corrigees.png`.
- `catalogue.html` : compteur 659 / 659, sélecteurs de classe et d'équipement,
  hache à deux mains, une case vers la gauche, sol de camp avec autres acteurs
  et objet au sol, zoom entier ×2.
- Catégorie PNJ/formes : sélection d'une transformation, 120 ms par case,
  sol clair et déplacement vers le haut avec la vue explicitement de face.
- Les planches natives des 25 tenues et des modèles du bestiaire ont été
  inspectées. Les grands corps ont leurs planches sans réduction.
- Un défaut de chargement lié à la CSP de la nouvelle enveloppe armée a été
  corrigé en déclarant les seules images `data:` autorisées ; chargement
  et rendu confirmés ensuite. Le sandbox conserve uniquement `allow-scripts`.
- Le contrôle de la vue de face ignore seulement les composantes RGB sous
  alpha nul. Diagnostic : les différences initiales étaient invisibles ;
  les pixels visibles et l'alpha sont conservés.
- Après correction du cadrage et du repérage de hauteur des anciens acteurs,
  les contrôles complets passent : 659 entrées, 25 656 images d'animation,
  24 264 comparaisons du haut du corps, aucune erreur.

La revue navigateur locale, les contrôles techniques et la publication Sites
ne constituent pas une approbation artistique du joueur ni une intégration
dans le jeu. Les six familles équipées ne couvrent pas les 113 armes séparément.
Le fichier autonome exporté conserve son sandbox et sa CSP ; son ouverture
automatique en `file://` n'est pas utilisée pour cette vérification.
