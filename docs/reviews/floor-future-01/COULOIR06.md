# SOL-FUT01-r02 — 06, caillebotis dans les couloirs

Statut : **06 validé et intégré** le 28 octobre. Le joueur demande ensuite
un peu plus d'usure ; la finition 07 présentée séparément est ensuite validée
et intégrée, voir [USURE07.md](USURE07.md). Les sources 06 sont figées après accord.

Le joueur valide 05 par « oui c'est bien mieux » et demande ensuite un sol en
grille dans le couloir. 05 est intégré à l'identique dans le jeu normal ; 06
présente séparément cette modification des zones de circulation.

Les cinq salles conservent exactement les pixels de 05, murs et pieds de décor
compris. Seul le sol visible de la galerie principale et des petits passages
de liaison reçoit un caillebotis fin. Les seuils suivent les contours acceptés,
sans nouvelle découpe des murs ni des boîtiers. Les trous restent des détails
de matière : aucune chute, collision, propriété de terrain ou règle de jeu
n'est ajoutée.

La source `assets/source-art/floor-future-01/corridor-grate-01/grating-source.png`
réemploie le dessin créé précédemment avec l'outil intégré imagegen. Son essai
dans les salles avait été refusé ; son utilisation proposée ici se limite aux
couloirs. Le nouveau conditionnement réduit la maille à 192 px, atténue son
contraste à 0,5 et accorde sa teinte au métal de 05. Le prompt d'origine est
conservé dans [PROMPT-GRILLE.md](PROMPT-GRILLE.md). Aucune nouvelle génération
de dessin n'a été nécessaire pour cette proposition.

## Contrôles

Le masque accepté du sol est repris. Les cinq rectangles complets des salles
en sont retirés : tous leurs pixels doivent rester identiques à 05.
Une fine rive d'un pixel des plaques 05 est conservée aux seuils pour éviter
que l'échantillonnage de la maille déborde sur les derniers pixels des salles.
Les alphas des deux couches restent identiques. Aucun pixel hors des zones de circulation
ne peut changer. La lumière large de l'architecture d'origine est conservée,
et le moteur applique ses éclairages habituels.

L'aperçu compare le rendu normal 05 à gauche et 06 à droite, à taille native64.
Il vérifie les salles, la galerie, le sas et les raccords aux petites salles.
L'état simulé et le HUD doivent rester identiques dans chaque paire.

```powershell
& 'C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe' tools/art/build_corridor_grate_review_01.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-corridor-grate artifacts/floor-continuous-review-01/corridor-grate-06
python tools/art/publish_floor_continuous_review_01.py --corridor-grate
```

L'[aperçu courant](../../../artifacts/floor-continuous-review-01/apercu.html)
présente 06. Les ressources candidates restent dans `assets/source-art/` ; le
jeu normal utilise les ressources 05 validées sous `assets/maps/`.

L'accord 05 est conservé dans [acceptance-05.json](acceptance-05.json).
Le conditionnement 06 est détaillé dans
[corridor-grate-conditioning.json](corridor-grate-conditioning.json), et les
contrôles dans [corridor-grate-validation.json](corridor-grate-validation.json).
Selon `AGENTS.md`, seule la partie visuelle modifiée doit être soumise de
nouveau ; les salles 05 ne demandent donc pas de nouvelle validation.
