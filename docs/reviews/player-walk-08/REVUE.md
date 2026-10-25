# WALK08 — catalogue des corrections, avant validation

Le joueur demande de corriger les lacunes recensées dans l'audit WALK07, puis
précise : « tu me montreras toutes les animations, que je puisses les valider
ou non ». Les corrections sont donc accessibles uniquement dans une revue
isolée. Le lancement normal conserve le lecteur précédent et les pixels
WALK07 acceptés. La banque rejetée WALK04 reste désactivée.

## Ouvrir la revue

Lancer `Voir_animations.cmd` à la racine du projet. Le programme construit
l'exécutable Debug et ouvre `--apercu-animations`. Ce mode ne crée ni ne reprend
de campagne et n'écrit pas les préférences du joueur.

Trois ensembles sont parcourables :

- Joueurs : cinq classes, cinq tenues chacune, les113 modèles d'armes, les
  configurations à deux armes compatibles avec leur prise, les six casques et
  les couleurs d'armes existantes. Les matières des tenues restent celles des
  sources acceptées ; aucune nouvelle recoloration globale de leur corps.
- Monstres : les471 couples modèle/couleur actuellement définis, soit129 bases
  et342 variantes. Les variantes utilisent la même géométrie et le même alpha
  que leur cycle de base ; leurs différences de teinte sont transférées depuis
  leurs images canoniques. Les nuances obtenues restent à revoir visuellement.
- Formes/drones : six transformations et quatre drones. Le canonnier conserve
  son cycle accepté. Les neuf autres cycles sont des propositions nouvelles
  produites avec l'outil de génération intégré, conditionnées sans déformation
  de membres, avec leurs sources et consignes conservées.

Le catalogue affiche les deux sens simultanément et les14 poses : vrai repos
et six phases par direction. Chaque essai parcourt exactement une case,
avec trois phases puis le vrai repos à l'arrivée. L'essai suivant alterne
l'appui avec les trois autres phases. Le défaut reste400ms pour cette seule
case, avec choix120/200/400ms. La pause
de démonstration ne devient pas une attente du moteur de jeu.

Les boutons « Valider ce cas » et « À revoir » enregistrent uniquement la
configuration affichée dans `user-decisions.json`. Ils n'activent rien dans le
lancement normal. Le code visible du cas permet aussi de signaler un défaut
dans la conversation. Les accords plus anciens restent distincts des nouvelles
variantes et compositions présentées ici.

## Préparation technique

Le lecteur candidat utilise les350 poses de tenue entières existantes, avec
leurs bras et doigts possédés par chaque dessin. Les50 banques de parties sont
copiées à l'identique dans `assets/animations/walk-08/player/`. Les armes réelles
et leurs prises sont composées avec ces poses ; aucun fusil peint n'est
substitué à une arme incompatible. Les jambes ne sont jamais cisaillées,
déplacées par bandes ou greffées à un autre corps.

La cadence candidate conserve la durée choisie et espace les répétitions
clavier en conséquence. Les commandes explicites restent immédiatement
transmises au moteur ; les tours, les sauvegardes et le déplacement logique
restent indépendants de la présentation. Même en cas de commandes rapprochées,
le rendu candidat ne cumule pas le trajet inachevé avec la case suivante.
Les déplacements haut/bas existants
ne sont pas remplacés et aucune vue de dos n'est ajoutée.

Les anciennes banques et les PNJ ne sont pas réactivés par cette correction.
Le périmètre du catalogue concerne les cinq classes et les créatures définies
ci-dessus, sans nouveau dessin de décor, d'objet ou d'effet de compétence.

## Sources et contrôles

- `authorization.json` et `review-scope.json` distinguent l'autorisation de
  correction de la demande ultérieure de validation préalable.
- `player-source-proofs.json` conserve les empreintes des50 fichiers exacts.
- `extra-generation.json`, `extra-cycle-jobs.json` et `extra-conditioning.json`
  conservent les neuf sources, les références, les consignes et les facteurs
  uniformes de conditionnement. Les originaux sont dans
  `assets/source-art/player-walk-review-08/`.
- `checks.json` conserve le bilan final des contrôles. Les captures courantes
  restent sous `artifacts/player-walk-review-08/` pendant la revue.

Statut : **propositions à examiner ; corrections non activées dans le jeu
normal**. La réussite des contrôles techniques ne vaut pas accord artistique.

## Hachette — révision AX67-r02, à examiner

Le joueur signale le retour des anciens manches épais. La comparaison des
sources retrouve WR-AX01, la hache d'armes modèle66 au manche de bois déjà
accepté, tandis que la Hachette modèle67 conserve sa source ancienne. Les
modèles61–65 gardent aussi leurs dessins distincts existants ; leur fer n'est
pas remplacé par celui du modèle66.

Le nouvel essai AX67-r02 présente une hampe de bois affinée pour le modèle67,
avec son fer compact propre. Le dessin complet est généré avec l'outil intégré,
réduit uniformément puis composé comme une couche rigide avec ses deux prises.
Il est disponible seulement dans la revue. Le modèle66 accepté conserve ses
pixels ; le lancement normal garde le modèle67 précédent.

`AX67-prompts.json`, `AX67-r02-conditioning.json` et
`AX67-r02-comparison.png` conservent les consignes et la comparaison. Les
sources sont dans `assets/source-art/player-walk-review-08/` et les ressources
de revue dans `assets/animations/walk-08/weapons/`. Les cas modifiés portent
le suffixe `AX67-r02` : leurs anciennes décisions restent conservées et ne
valident pas automatiquement ce nouvel essai.

## Prédateur — association des directions r02, à examiner

Le signalement « la forme de prédateur se déplace à l'envers. » révèle une
association inversée : sa première ligne dessinée regarde à droite et la
seconde à gauche. `left_row:1` raccorde les deux séries entières au bon sens,
y compris leurs repos, sans retourner les figures ni changer les phases.
Les neuf atlas gardent exactement leurs pixels. La distance reste une case,
400ms par défaut ; le lancement normal conserve son rendu précédent.
Le cas corrigé porte le suffixe `PREDATOR-r02`, pour sa propre validation.
Preuves et contrôles : `PREDATOR-r02.json`.

## Technomancien tenue1 — série droite r03, à examiner

Le joueur signale un buste tourné à gauche pendant le trajet droit. L'examen
des cinq tenues retrouve ce défaut sur CLASS-05 : ses deux séries dessinaient
une orientation gauche. Les quatre autres tenues présentent deux sens distincts.

TECH01-r03 propose sept figures droites entières : un repos et six phases,
avec tête, buste, genoux et pieds cohérents. L'outil de génération intégré
reprend l'identité mécanique sombre et ivoire ; le conditionnement réduit
uniformément chaque figure complète avant la découpe de propriété des bras.
Les49 banques conservées, dont la série gauche, restent exactes.
Les armes équipées utilisent les mêmes dessins partagés et prises adaptées.
Les sept poses droites révisées restent à valider dans le catalogue isolé ;
elles ne modifient pas le lancement normal.

Source : `assets/source-art/player-walk-review-08/TECH01-right-r03-source.png`.
Consignes et contrôles : `TECH01-r03-prompts.json`, `TECH01-r03.json`.
Cas de revue : `TECH01-r03`. Distance : une case ;400ms réglables.

## Technomancien tenue5 — échelle droite r02, à examiner

Le joueur signale un changement de taille pendant la marche droite. Les trois
dernières phases de CLASS-09 descendaient de56–57px à48px. TECH05-r02 reprend
ces trois figures entières depuis leur source haute définition existante et
les exporte avec une échelle uniforme par figure, correspondant à la pose de
référence56px. Aucun redessin, étirement d'un membre ou changement de démarche.
La variation restante est d'un pixel ; les pieds gardent leur ancrage.

Les347 autres liaisons de poses, dont les sept poses gauches et les quatre
premières poses droites, restent exactes. Seules les trois phases corrigées pointent
vers `assets/animations/walk-08/player/CLASS-09-1-r02.png`. Le nouveau cas
`TECH05-r02` reste à examiner dans la revue isolée. Source et contrôles :
`TECH05-r02.json` ; comparaison : `TECH05-r02-comparison-x2.png`.
