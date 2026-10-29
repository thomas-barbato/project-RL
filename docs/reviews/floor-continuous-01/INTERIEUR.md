# SOL-CONT01-r03 — 04, béton continu avec les pieds de décor conservés

Statut : **04 refusé** le 28 octobre ; jamais intégré.
Le joueur signale encore des dallages aux extrémités et demande une direction
plus futuriste. La proposition suivante est
[05, pont technique](../floor-future-01/REVUE.md). Le reste de cette page décrit
l'essai historique 04.

Le joueur valide le béton 02, puis signale des éléments qui semblent effacés
en bas et des restes de damier. Il suggère un caillebotis dans les salles, puis
refuse cet essai 03. La proposition 04 reprend exclusivement la matière de béton
déjà acceptée, dans les salles comme dans les couloirs.

## Cause et correction

L'architecture d'origine avait été découpée en deux couches à partir des cases
logiques : `floor.png` et `structure.png`. Les silhouettes réelles ne suivent
pas cette découpe. Certains pieds de murs et boîtiers se trouvent dans la couche
du sol ; des marges de sol et morceaux de damier se trouvent dans celle des murs.
Le premier essai modifiait uniquement le sol et protégeait des fragments par
leur couleur. Cela pouvait effacer une partie du matériel tout en laissant du
carrelage dans l'autre couche.

La correction recompose d'abord l'architecture d'origine entière, conserve les
contours et intérieurs des éléments métalliques, puis applique le béton continu
sur le sol visible dans les deux couches. Le joint peint de l'ancien raccord
du sas est retiré. Les images sont ensuite réparties dans leurs deux couches
d'origine, avec leurs alphas inchangés. Les accessoires et personnages séparés
conservent leurs dessins et placements.

La source de matière et son prompt restent ceux de
[PROMPT.md](PROMPT.md), obtenus avec l'outil imagegen intégré pour la revue 02.
Aucune nouvelle matière de caillebotis n'est utilisée dans 04.

## Reproduction

```powershell
& 'C:/Users/User/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe' tools/art/build_camp_floor_review_03.py
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-camp-floor-03 artifacts/floor-continuous-review-01/interior-r03
python tools/art/publish_floor_continuous_review_01.py --interior-r03
```

Sources candidates :
`assets/source-art/floor-continuous-01/interior-r03/floor.png` et
`assets/source-art/floor-continuous-01/interior-r03/structure.png`.
L'aperçu est [apercu.html](../../../artifacts/floor-continuous-review-01/interior-r03/apercu.html).

Huit comparaisons montrent le béton précédent 02 à gauche et la correction 04
à droite : les cinq salles, la galerie, la porte du sas et un passage vers une
salle. Ce sont des captures du moteur à 64 px, sans redimensionnement, avec les
mêmes personnages, caméra et éclairages dans chaque paire. La simulation et
l'interface sont contrôlées ; aucune sauvegarde existante n'est ouverte.
Les résultats sont dans [interior-r03-conditioning.json](interior-r03-conditioning.json)
et [interior-r03-validation.json](interior-r03-validation.json).

La version normale du camp conserve ses ressources actuelles. Le remplacement
de ses deux couches attend l'examen de 04, selon la règle d'`AGENTS.md` : les
visuels nouveaux ou révisés doivent être « validés explicitement par le joueur ».
L'extérieur 02 reste intégré et validé indépendamment.
