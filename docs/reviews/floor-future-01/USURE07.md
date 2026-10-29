# SOL-FUT01-r03 — 07, finition usée du caillebotis

Statut : **07 validée et intégrée** le 28 octobre 2026, après la réponse
« euh oui :) ca me va bien :) ». Les deux couches du décor normal reprennent
exactement les PNG 07 montrés ; les salles gardent les pixels 05 acceptés.

Le joueur répond « oui je valide » pour 06 et demande de le faire plus usé
« de part et d'autres ». La finition 07 ajoute de l'oxydation mate, des traces
de métal frotté et des éclats irréguliers. Leur intensité est plus forte près
des murs, avec quelques traces plus discrètes dans les passages.

L'usure provient d'une nouvelle couche transparente obtenue avec l'outil
intégré imagegen. La source et le prompt sont conservés dans
[PROMPT-USURE07.md](PROMPT-USURE07.md) et
`assets/source-art/floor-future-01/corridor-wear-07/wear-source.png`.

Le dessin 06 de la maille n'est pas régénéré. La couche d'usure est composée
uniquement sur les surfaces claires des barres existantes, dans le masque de
circulation déjà accepté. Les ouvertures sombres, les salles 05, les alphas,
les murs et équipements hors du sol restent inchangés. Aucun obstacle, objet,
éclairage, animation ni effet mécanique n'est ajouté.

Le calage de la couche utilise les bords réels du sol visible, avec un champ
d'usure continu par tronçon et une accentuation progressive près des murs.
Il n'existe ni tirage aléatoire à l'exécution ni répétition par case de 64 px.

## Reproduction

```powershell
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-corridor-wear artifacts/floor-continuous-review-01/corridor-wear-07
python tools/art/publish_floor_continuous_review_01.py --corridor-wear
.\target\debug\project-rl.exe --ui-cold-corridor-wear-integration artifacts/floor-continuous-review-01/corridor-wear-07/integration
```

L'[aperçu courant](../../../artifacts/floor-continuous-review-01/apercu.html)
compare la source conservée 06 et la finition 07, avec les mêmes personnages,
caméra et éclairages à taille native64. Les contrôles portent sur les huit vues,
les cinq salles inchangées, les pixels hors de la circulation, les ouvertures
de la maille et l'état de jeu inchangé.

Le contrôle d'intégration compare le rendu normal à la source 07, sans
différence dans les huit vues. Les références acceptées sont conservées dans
`assets/source-art/floor-future-01/accepted-07/references/` ; elles servent aussi
à contrôler les captures normales. Le générateur de matière est figé après
accord pour empêcher toute modification involontaire des dessins acceptés.

Accords : [acceptance-06.json](acceptance-06.json),
[acceptance-07.json](acceptance-07.json).
Fabrication : [wear-07-conditioning.json](wear-07-conditioning.json).
Contrôle avant accord : [wear-07-validation.json](wear-07-validation.json).
Intégration : [integration-07.json](integration-07.json).

L'accord porte sur cette finition montrée, sans nouvelle règle de jeu ni
nouvel effet lumineux. Les sources 06 et 07 restent conservées séparément.

Les captures brutes reproductibles passent dans la Corbeille après comparaison,
sans vidage. Les huit références acceptées, les sources et les planches
natives utiles restent conservées. Bilan :
[cleanup-07-integration.json](cleanup-07-integration.json).
