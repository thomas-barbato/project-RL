# POLISH01-WALK-LIGHT01 — exposition des poses de PNJ

Correction du changement de luminosité signalé par le joueur le 27 octobre 2026,
après l'intégration de la finition visuelle commune.

## Cause reproduite et correction

Les quatre dessins `CampNpc(1..=4)` utilisaient deux chemins de rendu. La pose
sans respiration héritait du matériau du décor via un dessin direct ; la pose
respirante passait par `world_readability::texture`, avec le matériau des acteurs.
Le déplacement désactive la respiration et pouvait ainsi changer l'exposition
de la silhouette. La bascule pouvait aussi se produire au repos, entre les deux
phases respiratoires. La finition commune rendait l'écart beaucoup plus visible.

Toutes leurs poses dans le monde passent désormais par le matériau des acteurs,
y compris lorsque le décalage respiratoire vaut zéro. Les portraits d'interface
conservent leur chemin. Les dessins sources, la silhouette, le décalage respiratoire
accepté, les règles et les sauvegardes restent conservés.

Le changement de chemin historique des PNJ de service `Actor(20..=25)` a également
été examiné : le diagnostic de proportions ne reproduit aucun écart sur ses
24 comparaisons. Ce chemin n'est pas modifié. Les quatre classes jouables et
quatre monstres examinés passent déjà par le matériau des acteurs dans les poses
contrôlées ; aucune correction arbitraire de leurs palettes sources n'est appliquée.

## Vérification dans le moteur

Commande de régression, compilée uniquement en Debug :

```powershell
cargo build --locked --bin project-rl
target/debug/project-rl.exe --ui-cold-walk-lighting artifacts/visual-polish-review-01/walk-lighting-fix/after --expect-fixed
```

Le diagnostic utilise le vrai compositeur à position fixe et taille native,
sans ouvrir de campagne. Il contrôle 18 acteurs, deux contextes d'éclairage
(sol généré et décor peint) et 16 états de présentation : repos, absence
d'horloge de repos, respiration, six phases dans chaque direction et retour
de face. Cela représente 576 captures et contrôles de chemins de rendu.
Il vérifie aussi l'identité des pixels entre les mêmes poses avec et sans
horloge, puis au retour de face.

Pour les quatre PNJ concernés, huit comparaisons supplémentaires alignent les
pixels du haut du corps après son déplacement respiratoire de deux pixels,
et comparent les pixels du bas du corps à position fixe. La couture de la
taille est exclue pour ne pas confondre le recouvrement accepté avec l'éclairage.

Avant correction : 120 états héritent du mauvais matériau, tous parmi les
quatre PNJ du camp. Après correction : aucun écart de chemin sur les 576 états,
aucun écart sur les huit comparaisons de pixels alignés. Après le nettoyage du
29 octobre, les dix planches finales utiles restent sous
`artifacts/visual-polish-review-01/walk-lighting-fix/after/` : les quatre PNJ du
camp en deux éclairages, le Bastion et le zombie 120 éclairés. Les captures
répétitives avant/après se régénèrent avec le diagnostic ; les matrices
numériques complètes et les sources restent conservées.
Leurs six colonnes sont : repos, sans horloge, respiration, marche gauche,
marche droite et retour de face.

La matrice de 17 cartes et situations de `--ui-cold-polish-integration` est
rejouée après correction. Les résultats de cette correction, les compilations
et l'empreinte du nouveau Release sont consignés dans
`walk-lighting-validation.json`. Le comparatif artistique accepté reste conservé
dans `references/` ; il n'est pas remplacé par les captures de diagnostic.
