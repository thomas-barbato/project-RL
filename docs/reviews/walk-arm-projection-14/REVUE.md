# ARM14-r01 — Bras équipés pendant la marche

27 octobre 2026. **Essai présenté, puis généralisation autorisée.**

Après les comparatifs, le joueur répond « ok tu peux le faire pour tous les
personnages ? Sait-on jamais :) ». `acceptance.json` conserve cet accord.
L'intégration applique la méthode aux 25 tenues techniques, dont les cinq
identités conservées pour les sauvegardes de l'ancien Technomancien. Cela ne
rétablit pas sa classe dans la création. `integration-validation.json` consigne
les contrôles de couverture et les builds ; les sections suivantes décrivent
l'essai exact présenté avant cet accord.

Le joueur signale des bras trop volumineux en trois quarts et un décalage des
armes. Le compositeur WALK12 conserve en effet les bras et prises de sa référence
de repos WALK11 (phase 8), puis translate cet assemblage sur le corps WALK12.
Pour le Bastion 1/5, la somme des surfaces des deux bras sources vaut 325 pixels
opaques à gauche et 277 à droite, contre 140 et 160 dans la première pose WALK12
(mesure à alpha 96, sur les toiles de composition 128, pour une case native 64).
La transformation de portée conserve la largeur des manches : les bras de
référence trop épais restent donc épais une fois pliés sur les nouvelles poses.

Le candidat conserve une seule prise pendant les six phases, mais la sélectionne
parmi les six poses WALK12 de la tenue et de la direction. La pose retenue minimise
la plus grande distance épaule-paume, afin d'écarter les bras pendants anormalement
allongés de certains dessins. Les bras, gants et repères viennent de cette même
pose. Les armes conservent leur dessin et leur échelle existants. Le petit décalage
du torse est appliqué à tout l'assemblage, comme auparavant.

Lors de la proposition, l'essai était accessible uniquement dans un diagnostic Rust `#[cfg(test)]`, via
`ARM_PROJECTION14`. Le jeu normal conservait la référence 8. Aucun PNG accepté,
manifeste d'animation ou dessin frontal n'est remplacé. Les jambes, corps et
casques des 720 compositions comparées restent identiques octet par octet au
compositeur actuel. Le mouvement vertical et les monstres ne sont pas modifiés.

## Comparatifs animés à taille native

Chaque GIF présente les cinq tenues : face, actuel gauche/droite, puis candidat
gauche/droite. Une case fait 64 pixels. Un déplacement prend 400 ms et trois
phases ; le suivant emploie les trois autres. La pause de lecture de 600 ms n'est
pas une modification du jeu. L'arrivée montre la vraie vue de face.

| Personnage | Fusil 002 | Hache 067 | Deux armes 068 + 101 |
|---|---|---|---|
| Bastion | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Bastion-rifle-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Bastion-axe-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Bastion-dual-natif64.gif) |
| Traqueuse | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Traqueuse-rifle-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Traqueuse-axe-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Traqueuse-dual-natif64.gif) |
| Arcaniste | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Arcaniste-rifle-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Arcaniste-axe-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Arcaniste-dual-natif64.gif) |
| Métamorphe | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Metamorphe-rifle-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Metamorphe-axe-natif64.gif) | [Voir](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Metamorphe-dual-natif64.gif) |

[Détail Bastion 1/5 en zoom x2](C:/Users/User/Desktop/project-RL/artifacts/walk-arm-projection-14/r01/Bastion-axe-detail.gif).

Les PNG `*-comparatif.png` sont des instantanés fixes de ces compositions.
Les atlases `CLASS-*-current.png` et `CLASS-*-candidate.png` proviennent directement
du compositeur Rust ; leur orientation d'export est corrigée uniquement pour
la présentation. Les sources de dessin restent intactes.

## Contrôles et limites

- 20 tenues des quatre classes jouables, trois configurations, deux directions,
  six phases : 720 poses candidates et autant de comparaisons de corps.
- Chaque îlot de main d'au moins deux pixels à alpha 96 recouvre l'arme à alpha 64 :
  **720 cas, zéro échec de contact**. Ce contrôle ne remplace pas la revue visuelle.
- Le test de stabilité des bras/armes du jeu normal passe (26,72 s).
- `cargo check --locked --all-targets`, formatage et `git diff --check` passent.
- Les 113 modèles d'armes, les couleurs d'équipement et les casques ne sont pas
  tous recroisés dans cette première revue. Le Technomancien retiré et les monstres
  ne font pas partie du candidat.
- Aucun build Release ne contenant ce candidat n'est livré.

L'HTML autonome `apercu.html` est généré, mais son exécution dans le navigateur
n'a pas été vérifiée : la politique du navigateur refuse les URL `file:`. Aucune
autre voie d'ouverture du navigateur n'est utilisée. Les GIF et PNG constituent
les références de cette revue.

La validation visuelle était requise avant intégration par `AGENTS.md` et le guide
`project-rl-lateral-walk-review` : « Obtain explicit review before integration ».
L'accord reçu conserve la règle de sélectionner les bras dans le même cycle
et la même projection que le corps, sans reprendre une référence plus volumineuse.

## Intégration terminée

Le rendu normal Debug et Release utilise ARM14 pour les 25 tenues techniques.
Le choix de pose est calculé depuis les métadonnées déjà chargées, une seule
fois par tenue/direction ; il ne décode pas six images à chaque nouvelle arme.
La comparaison historique WALK11 et le marqueur de repos 8 gardent leur recette.

Contrôles finaux : 5650 contacts (25 tenues ×113 armes ×2 directions), sans échec ;
720 poses de marche avec trois configurations, sans échec de contact ; stabilité
des six phases, six casques sur les 25 tenues et retour frontal aux trois vitesses.
Les 120 atlases de l'essai validé sont reproduits à l'octet près.

Le diagnostic natif du moteur passe avec déplacement réel d'une case, 25 tenues,
six configurations d'équipement, deux éclairages et retour de face. Les images
des tenues 1/5 et 5/5, ainsi que le pas à 200ms et l'arrivée à400ms ont été inspectés.
Les en-têtes historiques de ces planches portent encore WALK11 ; le lecteur actif
emploie bien WALK12 et ARM14. Les contrôles de compilation de toutes les cibles,
formatage et diff passent ; les exécutables Debug et Release sont à jour.
Aucun dessin accepté, fichier de sauvegarde ou préférence personnelle n'est modifié.
Le paquet portable n'est pas reconstruit. Voir `integration-validation.json`.
