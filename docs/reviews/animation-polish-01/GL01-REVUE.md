# C05-GL01-r01 — modèle de lance-grenades repris

Le 30 octobre 2026, le joueur accepte le premier lot avec une réserve : le modèle
de lance-grenades W023 reste beaucoup plus massif que les autres armes.
L'accord est conservé dans `acceptance-partial.json`. La réponse suivante
« 01 — réduction modérée » retient C02-A pour les autres armes lourdes.

Le nouveau dessin propose un petit corps sombre, un canon court de gros calibre
et une crosse ajourée olive. Son encombrement source passe de 52 × 24 à 44 × 18 px.
Il conserve deux surfaces de prise peintes, repositionnées selon son dessin.
Le corps du porteur, les sources acceptées et le modèle W023 normal restent
conservés. La proposition originale est dans `assets/source-art/animation-polish-01/`.

Le dessin est obtenu avec l'outil intégré imagegen, puis conditionné en alpha
prémultiplié. Le prompt exact et les chemins sont dans `GL01-PROMPT.json`.
Une retouche supplémentaire de la poignée a été refusée par le filtre
« violence » de cet outil ; aucun dessin n'en est issu. Le premier dessin est
donc utilisé tel quel, sans déplacement manuel de ses poignées.

La revue 03 compare le modèle actuel, sa réduction précédente 02 et GL01, dans
les deux marches latérales de quatre personnages en armure lourde, avec la tête
de la tenue ou le casque 120. Les prises suivent les nouvelles attaches ; la
mitrailleuse sert de témoin à l'échelle native. Une composition dans le sol et
les structures acceptés du camp complète le contrôle.

Le nouveau dessin et ses prises restent à valider. Les poses face/dos et les
couleurs ne sont pas proposées dans ce lot : les dessins verticaux actuels ont
un autre écartement des mains et devront être adaptés à l'arme retenue. La revue
latérale ne vaut pas validation de cette adaptation ni intégration normale.

```powershell
python tools/art/prepare_grenade_launcher_review.py
cargo build --locked --bin project-rl
& .\target\debug\project-rl.exe --ui-cold-grenade-redraw artifacts/animation-polish-01/grenade-r01/current
python tools/art/build_animation_polish_review.py
```
