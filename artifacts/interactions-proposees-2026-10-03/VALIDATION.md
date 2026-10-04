# Vérification des propositions

- Lecture des commandes, des installations, de l'intrusion et de l'ingénierie
  actuelles, puis comparaison avec la direction d'exploration et de progression.
- Cinq propositions rédigées ; aucun changement des sources du jeu, de son
  contenu actif, de ses réglages ou des fichiers de partie.
- Aperçu de l'alimentation relu à 736 et 320 pixels : deux états distingués,
  trajet alternatif toujours affiché et fermeture visible du raccourci.
- Contrôle dans Chromium : boutons et clavier fonctionnels, aucun texte
  débordant ou superposé, aucune erreur de script. Résultats dans
  `preview-validation.json`, contrôles dans `verify-preview.cjs`.
- Les images et le document autonome `preview.html` servent au contrôle local.
  L'aperçu de conversation est un schéma de conséquences ; il ne valide aucun
  nouveau dessin ni aucune mécanique intégrée au jeu.

Les contrats moteur, la génération, la persistance et l'équilibrage des cinq
interactions restent à mettre en œuvre après sélection des propositions.
Aucun commit ni push.

## Révision inspirée de Cogmind

- Manuel officiel en texte consulté : Beta 17.1. Ancien PDF Beta 7 identifié
  et écarté comme référence des règles actuelles.
- Articles du développeur consultés sur l'information de carte, les commandes
  d'installations, les branches facultatives et la simplification de fabrication.
  Les liens et dates des articles figurent dans `INSPIRATION_COGMIND.md`.
- A reçoit une variante de terminal sur le terrain ; E propose une diversion
  sonore locale. D distingue explicitement ses règles proposées de celles des
  stations de réparation de Cogmind.
- `before-cogmind/` conserve les deux documents avant cette révision.
- L'aperçu d'alimentation reste inchangé. Aucun nouvel accessoire graphique,
  commande moteur ou paramètre d'équilibrage n'est ajouté au jeu.
- Cette révision documentaire n'appelle pas de nouvelle compilation ou de
  nouvelle exécution de la suite moteur. Les propositions exigent encore une
  validation de conception avant leur mise en œuvre.
