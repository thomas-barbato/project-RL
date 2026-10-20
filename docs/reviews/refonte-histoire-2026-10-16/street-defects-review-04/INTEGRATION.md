# Intégration de STYLE-MAP-U01-04

19 octobre 2026. Confirmation directe du joueur : « oui je valide », alors que
la revue de la carte entière est ouverte. L'accord concerne le rendu présenté,
y compris la barricade et les trois corps visibles près de l'entrée de prison.

## Ressources et rendu

La texture de matière et les quatre décors sont copiés sans rééchantillonnage
ni changement d'alpha dans `assets/maps/prison-street-expanded-01/decor/`.
Leurs empreintes correspondent aux sources montrées. `street.json` contient la
référence à la matière et les positions exactes des décors. Les ressources
sources et prompts restent conservés dans `assets/source-art/`.

Le matériau de surface est désormais disponible en debug et en release. Sa
formule et ses valeurs restent celles de la proposition acceptée. Il est chargé
une fois avec l'atmosphère de la rue, et les textures des décors avec la carte.
Le rendu normal ne lit plus le manifeste temporaire de candidats. Aucune
ressource de production ne dépend d'`artifacts/`.

Les corps sont placés dans la couche du sol, avant les personnages vivants. Le
matériau agit sur les rues, murs, bâtiments, mobilier, intérieurs, toits et
voiture. Les lentilles lumineuses, personnages, interface et animations de feu
conservent leur traitement. Les toits gardent leur masquage à l'ouverture et
lorsque le joueur est dans le bâtiment.

La configuration reste propre à la rue étendue : le camp et les anciens formats
de petite rue gardent leurs ressources. Les collisions, la génération 159, les
sauvegardes et les règles ne changent pas. Une partie existante affichant déjà
la rue étendue bénéficie du rendu au prochain chargement.

Le petit patch rectangulaire r03 et son entrepôt redessiné ne sont pas activés.
GRIM-U01-02 reste refusé pour cette rue. L'ancienne ambiance GRIM-L02 n'est pas
activée : les données d'éclairage restent celles montrées dans la revue complète.

## Contrôle d'intégration

La capture d'intégration parcourt les neuf mêmes lieux, entre réellement dans
la maison puis retourne au camp, dans une partie isolée. Pour chaque lieu, elle
réactive le mode normal sans forcer le matériau ni les décors de revue, puis
capture l'image. L'empreinte du monde doit rester identique pendant le rendu.

La vérification compare les neuf vues normales et la carte entière avec les
captures acceptées. Les résultats exacts, ainsi que les builds et tests ciblés,
sont consignés dans `integration-checks.json`. Ce contrôle ne remplace pas une
exécution complète de toute la suite du jeu.

Résultat : la carte entière normale est identique pixel par pixel à l'export
accepté. Les neuf vues de scène ne présentent aucune différence de décor.
Dans la vue de maison ouverte, l'ancienne capture contient en plus un fin cadre
d'interface de 64 × 64 px, hors du cercle de vision, sur fond noir. Ses 252 pixels
de bord sont isolés explicitement par le contrôle ; la différence brute reste
consignée, sans tolérance appliquée aux pixels du décor ni de la carte entière.

Les douze tests ciblés de la rue réussissent, dont portes/toits, trajets, retour
au camp, anciennes générations et reprise de sauvegarde. La compilation debug,
le contrôle release, le formatage et `git diff --check` réussissent également.

```powershell
target/debug/project-rl.exe --ui-cold-street-style-integration artifacts/street-defects-review-01/engine05
python tools/reviews/check_street_style_integration.py
```
