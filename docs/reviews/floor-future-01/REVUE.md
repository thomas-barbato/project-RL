# SOL-FUT01-r01 — 05, pont technique

Statut au 28 octobre : **05 validé et intégré**. Le joueur demande ensuite un
caillebotis dans les couloirs ; cette partie révisée est présentée séparément
dans [COULOIR06.md](COULOIR06.md). Le reste de cette page décrit la revue 05.

Le joueur écarte le béton 04 : « le sol est mal découpé », avec des dallages
encore visibles aux extrémités de la salle. Il demande ensuite « un sol un peu
futuriste ». L'extérieur 02 déjà accepté et intégré reste conservé. Le
caillebotis 03 reste refusé ; cette proposition n'en utilise aucun dessin.

## Direction présentée

05 propose un pont technique en métal bleu-gris mat : de grandes plaques de
proportions variées, des joints fins et décalés, des détails de maintenance
rares et une trappe affleurante dessinée dans le sol. Les petites marques
ambre et cyan sont peintes, sans émission lumineuse. Il s'agit d'une matière
de sol : aucun nouvel objet interactif, obstacle, piège ou éclairage n'est ajouté.
Les dessins de personnages, accessoires, murs et luminaires restent ceux du jeu.

La source [deck-source.png](../../../assets/source-art/floor-future-01/deck-source.png)
est issue de l'outil intégré **imagegen**, avec le
[prompt exact et les références](PROMPT.md). Elle est conservée dans le projet
à sa résolution originale. Les scripts conditionnent sa taille et recomposent
les pixels existants ; ils ne dessinent pas une nouvelle architecture.

## Contours du sol

Le nouveau masque est indépendant de la séparation logique entre `floor.png`
et `structure.png`. Les cinq salles et la galerie ont chacune des fenêtres de
contour définies en pixels de source. Les tracés suivent l'encre du dessin au
pied des murs, grilles d'aération et boîtiers ; les petits passages suivent
leurs montants réels. Les deux couches sont recomposées ensemble, puis
réparties de nouveau avec leurs alphas d'origine.

Les bandes de sol anciennement laissées au bas des salles sont contrôlées
par cinq sondes distinctes dans les zones de sol connues, jusqu'au bord visible.
Ces sondes doivent être entièrement remplacées. Les dessins des luminaires et
des deux montants jaunes restent protégés. Aucun rectangle de protection
large comme une case n'est utilisé pour les petits couloirs.

Le contour détaillé et les sondes sont conservés dans
[ground-contours.json](ground-contours.json). Les pixels hors du masque de sol,
les rectangles d'équipement protégés et les alphas des deux couches sont
contrôlés à l'identique. Ces contrôles complètent l'examen visuel des captures ;
ils ne remplacent pas l'accord du joueur sur le rendu.

## Aperçu et reproduction

```powershell
& 'C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe' tools/art/build_floor_future_review_01.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-future-floor artifacts/floor-continuous-review-01/future-r04
python tools/art/publish_floor_continuous_review_01.py --future
```

L'[aperçu courant](../../../artifacts/floor-continuous-review-01/apercu.html)
compare 04 à gauche et 05 à droite, à taille native de 64 px, sans
redimensionnement : arrivée, galerie, réserve, salle de sécurité, infirmerie,
atelier, porte du sas et passage vers une salle. Les personnages, l'heure de
présentation, la caméra et l'éclairage sont identiques dans chaque paire.
La simulation et le HUD sont vérifiés sans ouverture d'une sauvegarde existante.

Les sources candidates sont
`assets/source-art/floor-future-01/floor.png` et
`assets/source-art/floor-future-01/structure.png`.
Les preuves sont dans [conditioning.json](conditioning.json) et
[validation.json](validation.json).

Après accord, les deux images 05 ont été copiées à l'identique dans les
ressources actives du camp. L'architecture d'origine est conservée dans les
sources, et les huit captures acceptées sont figées sous
`assets/source-art/floor-future-01/accepted-05/references/`.
Les salles ne changent plus dans 06. L'accord et les empreintes se trouvent
dans [acceptance-05.json](acceptance-05.json).
