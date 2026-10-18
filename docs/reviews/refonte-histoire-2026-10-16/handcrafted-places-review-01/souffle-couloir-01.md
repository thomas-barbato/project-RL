# SOUFFLE-COULOIR-01 — Attribution corrigée et confirmée

Le joueur confirme explicitement « oui ! » : Braise reste un projectile de feu
monocible. Le couloir précédemment décrit concerne Souffle de feu.
L’accord courant est `souffle-couloir-01-acceptance.json`.

## Braise FE01

Restauration du catalogue précédemment accepté : texte, portée initiale de
5 cases et progression existante, dégâts, coût, recharge, brûlure et cible unique.
Le survol ne consomme rien. Un clic confirmé sur une case vide ne lance pas ce
projectile ; sa résolution nécessite toujours un ennemi admissible.
Le dessin suit la direction lanceur–cible, puis l’impact reste dans sa case.

## Souffle de feu FE02

Couloir cardinal de 3 cases de large. Longueur de 2 cases au rang 1, augmentée
d’une case par rang effectif : 5 au rang 4, 11 au rang 10. Les murs arrêtent
les bandes. Les ennemis ne bloquent pas les suivants. Les cases des coins sont
visables. Le lanceur ne fait pas partie des cases du couloir.

La canalisation, sa durée maximale, les dégâts et brûlures, leurs règles de
cibles et de progression, le coût par tour et l’annulation existants sont
conservés. Ce correctif ne remplace pas Souffle de feu par un lancement instantané
utilisant les dégâts ou le coût de Braise. Le descriptif précédemment accepté
« Vous déchaînez un torrent de flammes devant vous. » est conservé.
La fiche affiche la largeur et la longueur ; les anciens paramètres d’angle
d’éventail sont retirés. Les blocs actuel/prochain rang restent inchangés dans
leur principe et ne répètent pas les valeurs constantes.

L’effet conserve la cible sélectionnée pour calculer sa direction : il ne la
remplace plus par la position du lanceur, ce qui produisait une orientation vers
la droite. Ses trois bandes de feu utilisent les dessins déjà présentés, ajustés
aux cases de l’effet actif. Elles restent animées pendant la canalisation.
La prévisualisation disparaît à la confirmation.

## Validation technique

Les contrôles couvrent Braise monocible et sa brûlure, les dimensions et coins
du couloir aux rangs 1 à 12, les quatre directions réellement conservées par la
canalisation, les coûts par tour, les murs et la sauvegarde de l’effet.
Le rapport `souffle-couloir-01-integration-native.json` rassemble les résultats.
L’aperçu courant reste `artifacts/walk-preview-review-01/effets-zones.html`.
