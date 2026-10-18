# DIALOGUE-BUBBLE-01 — propositions visuelles

## Révision courante r02 — terminal choisi

Le joueur apprécie la version terminal (03), mais demande une vraie jonction
entre le rectangle et la pointe, un peu de transparence et la suppression du
bouton « Fermer [Échap] ». Il demande ensuite le retrait des renforts aux coins.
Ce choix fixe le style à retravailler ; le rendu modifié r02 reste à examiner.

Le contour de 1 px suit maintenant le périmètre complet, sans ligne entre le
corps et la pointe. Les deux surfaces translucides partagent une arête sans
se recouvrir : aucun assombrissement double n'apparaît à la jonction. Le fond
a une opacité de 224/255 (environ 88 %), tandis que le texte et le contour
restent opaques. Les coins ne sont plus renforcés. Aucun bouton de fermeture
ni instruction n'est affiché dans la bulle, et l'espace du bouton est retiré.

La fermeture demandée est préparée pour cette simple bulle sans choix : clic
gauche dedans, clic gauche à l'extérieur ou Échap. La trame d'entrée est
consommée pour ne pas déclencher aussi un déplacement, une attaque ou le menu
derrière. Le diagnostic vérifie les trois chemins et l'absence de mutation du
jeu. L'aperçu de la galerie permet aussi de refermer la bulle au clic ou avec
Échap ; son bouton de réouverture est un contrôle de revue hors du jeu.

Le texte apparaît progressivement à 40 caractères par seconde, sans déplacement
ni redimensionnement de la bulle. Le curseur bloc suit le texte, puis clignote
à la fin (0,5 seconde visible, 0,5 seconde invisible). En mouvement réduit,
le texte complet et un curseur fixe remplacent l'animation. La fermeture reste
possible pendant la frappe.

La correction du joueur précise que les pictogrammes de quête sont des
notifications : ouvrir le dialogue les acquitte. Fermer la bulle ne les
rétablit pas. Seule une nouvelle information liée à une étape de quête pourra
justifier une nouvelle notification ; relire le même dialogue n'en crée pas.
Le joueur demande ensuite un symbole différent lorsque l'objectif est rempli
et que la quête doit être rendue au PNJ. Proposition visuelle : « ? » doré,
distinct du « ! » de nouveauté, visible tant que la remise est à effectuer.
Ce nouvel état n'est pas masqué par l'acquittement de l'ancienne notification.
Il disparaît pendant l'échange et revient après fermeture si la quête reste
à rendre ; la remise effectuée le retire. La capture de revue simule seulement
le pictogramme, sans créer de quête ou de récompense.
Cela ne valide, n'accepte ni ne termine une quête. Les icônes de statut restent
indépendantes. Les repères de service cachés pendant l'échange peuvent revenir
après sa fermeture. L'état consulté de cet aperçu est isolé : l'enregistrement
persistant par notification sera raccordé lors de l'intégration validée.

Le texte complet DIAL-CRYO-01 r02 reste inchangé. La capture sans bulle et la
capture avec bulle partagent exactement le même fond GPU ; le contrôle des
pixels vérifie la transparence, la jonction sans barre et les coins fins.
Les sorties restent dans le même dossier temporaire `dialogue-bubbles-01/`,
et la trace durable est `dialogue-bubbles-02.json`. L'affichage normal et les
services existants ne sont pas modifiés tant que la révision visuelle n'est
pas acceptée. Le contour de survol reste une revue distincte, toujours en attente.

Le choix sonore provisoire du 03 autorise ensuite un essai jouable de cette
bulle avec son dans le client réel. Ce test passe par le lanceur séparé
`Tester_dialogue_terminal.cmd` et sa propre sauvegarde ; il ne vaut pas
acceptation définitive du rendu ou du son et ne remplace pas l’interface
des parties ordinaires.

## Propositions initiales r01 — historique

18 octobre 2026. Le joueur demande des bulles de dialogue stylisées, avec
plusieurs solutions à accepter ou refuser avant leur intégration.

La revue emploie le client réel, le camp accepté, le joueur à son échelle
habituelle et le dialogue complet DIAL-CRYO-01 r02, sans réécriture. Les trois
propositions gardent le nom de l'officier, le même texte et la fermeture.
Elles conservent visibles le joueur et son interlocuteur et restent dans la
surface de jeu. Les bulles ne sont pas actives pendant une partie normale.

- **01 — Bulle sombre.** Bulle près de l'interlocuteur, fond charbon, liseré
  chaud et pointe orientée vers celui qui parle. Apparence sobre et lecture
  directe au sein du décor.
- **02 — Bulle blindée.** Cartouche métallique au bas de la scène, texture
  actuelle de l'interface et raccord visuel vers l'officier. Sa largeur laisse
  davantage de place aux longs échanges.
- **03 — Bulle terminal.** Bulle vert phosphore, lignes d'écran, angles
  techniques et pointe vers le personnage. Ce candidat rappelle le second
  style d'interface déjà disponible, sans imposer ce style à tout le HUD.

L'aperçu ne propose ni frappe progressive du texte, ni pagination, ni délai,
ni changement de tours, de dialogue, de quête ou de service. Aucune nouvelle
réponse du joueur n'est inventée. Les vrais choix et les services resteront
à adapter une fois le rendu choisi ; commerce, soins et amélioration ne
perdent aucune fonction par cette proposition.

Le diagnostic `--ui-cold-dialogue-bubbles` vérifie l'identité de tous les mots
affichés, l'absence de troncature et de chevauchement avec le bouton, la
visibilité de l'officier et l'absence de mutation du jeu pendant le rendu.
Les captures natives sont reproductibles sous `artifacts/`. Le choix visuel
et toute éventuelle révision restent soumis à l'accord explicite du joueur,
conformément à `AGENTS.md`.
