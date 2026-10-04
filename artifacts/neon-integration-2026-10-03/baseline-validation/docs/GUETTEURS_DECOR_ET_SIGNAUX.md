# Guetteurs — terrain et transmission lisibles

27 septembre 2026. Première tranche visuelle locale de la
[descente vers le numérique](DESCENTE_VERS_LE_NUMERIQUE.md), après la
[répartition des milieux](REPARTITION_DES_PROFONDEURS.md).

## En jeu

Dans les régions de réseau qui contiennent des Guetteurs, de petites stries
ramifiées apparaissent sur les sols secs proches de leur placement initial.
Elles donnent un repère au lieu sans remplacer le décor de toute la couche.
Leur inspection indique simplement « Stries ramifiées · passage libre ».

Les marques sont fixes, ne suivent pas les créatures et ne disparaissent pas
avec elles. Elles ne désignent donc pas une présence actuelle hors de vue.
Ce sont des traces décoratives, pas des câbles à couper, des pièges ou une
nouvelle ressource. Elles ne changent ni la marche, ni la vision, ni l'eau,
ni les cases protégées ; les caches, portes et autres repères restent prioritaires.

Quand un Guetteur alerte réellement un voisin, une brève série de points
directionnels montre la transmission. Son aspect est distinct d'un éclair,
d'un projectile offensif ou d'une explosion. Le journal conserve l'annonce
« Le Guetteur alerte ses voisins. » : la compréhension ne repose pas seulement
sur cette animation fugitive.

## Règles conservées

La [transmission existante](GUETTEUR_ET_SPECTRE.md) garde sa portée, ses limites,
ses destinataires et sa dernière position connue. Aucun dégât, renfort,
pouvoir de synchronisation ou mécanisme de coupure n'est ajouté. Les Guetteurs
restent des organismes, pas des robots.

L'événement du moteur fournit maintenant les liaisons réellement parcourues,
avec les positions au moment de l'alerte. Le client ne les reconstruit pas à
partir de créatures ayant déjà bougé et ne dessine pas de raccourci entre la
source initiale et un destinataire atteint par un relais intermédiaire.

Une liaison n'est dessinée que si ses deux extrémités et tout son trajet sont
visibles. Le rendu revérifie la visibilité à chaque image ; la mémoire de carte
ne reçoit jamais de signal actif. L'option de mouvement réduit donne un repère
fixe et bref. Quitter une zone efface les signaux visuels transitoires ordinaires.

## Persistance et périmètre

- La génération reste en version 117 : tirages, acteurs, statistiques et butins
  sont inchangés. Seule la présentation s'enrichit.
- Les stries sont enregistrées avec le décor de la zone et respectent sa mémoire
  explorée. Le nouveau type de décor est ajouté après les anciens discriminants
  sérialisés, sans les déplacer.
- Les décors déjà sauvegardés sont conservés. Les stries sont posées lors de la
  génération de nouvelles régions de réseau contenant ces créatures.
- Les positions de transmission sont des événements transitoires, pas un nouvel
  état d'IA à sauvegarder. Une reprise ne recrée pas artificiellement une alerte.
- Aucun changement de ville, de faction, de population ou de mode graphique.

## Vérifications

- Liaisons réelles sur deux relais, sans dépassement des destinataires autorisés.
- Masquage complet si une extrémité ou une case intermédiaire est cachée ;
  extinction temporelle, mouvement réduit et effacement au changement de zone.
- Décor statique après déplacement, terrain inchangé, préservation de l'eau,
  des obstacles, protections et caches ; sérialisation et indices historiques.
- Scène utilisant les profils du catalogue : alerte réelle, reprise du combat
  et mêmes événements après snapshot. Comme l'ancien diagnostic du Guetteur,
  elle réduit uniquement la perception du deuxième individu pour isoler la
  réception de l'alerte de sa propre vision du joueur ; le contenu du jeu ne change pas.
- Génération régionale Guetteur/Spectre sur 48 cas et contrôles des passages sûrs,
  habitats, marques au sol et limites de population.

Les captures natives isolées montrent le repos, l'alerte en 1280 × 800 et l'alerte
en 960 × 540. Ce ne sont pas des captures d'une campagne parcourue manuellement.
Commandes de diagnostic : `--ui-cold-watcher-habitat`,
`--ui-cold-watcher-habitat-signal`, `--ui-cold-watcher-habitat-signal-960`, suivies
d'un dossier de sortie neuf. Elles ne remplacent pas la sauvegarde du joueur.

Validation de cette tranche : 748 tests moteur et 45 tests client ciblés réussis
(Guetteurs/Spectres, signaux, effets, vue terminal et compatibilité 117), dont
quatre nouveaux tests client. La suite client complète n'a pas été relancée.
Compilation native, contrôle de toutes les cibles, formatage et `git diff --check`
réussis. Captures finales :

- `target/ui-watcher-habitat-20260927-final/cold-start.png` ;
- `target/ui-watcher-signal-20260927-final/cold-start.png` ;
- `target/ui-watcher-signal-960-20260927-final/cold-start.png`.
