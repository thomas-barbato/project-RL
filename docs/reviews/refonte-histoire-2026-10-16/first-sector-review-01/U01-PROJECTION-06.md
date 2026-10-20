# U01-PROJECTION-06 — recaler la vue sur le camp accepté

18 octobre 2026. **Projection corrigée acceptée ; intégration en cours.**

Le joueur avait accepté l’intérieur de SCENE-U01-05, le portail sud ouvert
et le nom « Abords de la prison ». Il revient ensuite sur l’angle :
« je pense que l'angle est encore mauvais, ca n'est pas le meme angle que
sur la carte d'arrivée qui là est correcte ».

L’acceptation des pixels de la rue est donc rouverte pour sa projection.
Le portail au sud comme raccord au camp, le nom et la possibilité d’entrer
dans le premier bâtiment à gauche restent demandés. Il n’y a pas de nouvelle
quête ni de condition de mission ajoutée : le système de quêtes reste différé.

La référence prioritaire est le camp accepté, avec axes du sol parallèles,
cases carrées et faces avant dessinées sans fuite de perspective. Le dessin
précédent sert seulement à conserver les lieux et leur variété, pas sa caméra.

Pendant la correction, le raccord et le rendu de rue restent désactivés pour
le lancement normal en génération 154. Après présentation, le joueur répond
« Valider cette perspective » puis précise : « pour l'instant c'est ok . mais
reste au maximum le plus proche de la salle de départ. ». Cette acceptation
est figée dans `visual-acceptance-r06.json`. La génération 155 raccorde la
rue corrigée ; les générations antérieures gardent leur fournisseur historique.
Les sources de SCENE-U01-04 et 05 sont conservées pour traçabilité.

## Proposition corrigée

SCENE-U01-06-r01 prend le camp comme référence prioritaire. Les rues,
bâtiments et clôtures sont redessinés sur des axes horizontaux et verticaux ;
le portail est maintenant horizontal. La première habitation à gauche est
montrée en coupe avec son entrée sud dégagée. La boutique, le garage et le
bâtiment effondré gardent des formes distinctes.

La source est conservée intacte dans
`assets/source-art/first-sector-review-01/u01-scene-r03/SCENE-U01-06-r01.png`.
Création avec imagegen intégré ; prompt exact dans `u01-projection-prompt-r06.txt`.
Le nom et les intentions de passage précédemment validés sont conservés.
Les nouveaux pixels ont reçu cette validation propre ; la référence stricte
du camp reste applicable aux dessins suivants.

La page courante montre `u01-projection-native-r06.png` et
`u01-projection-home-native-r06.png`, avec le personnage inchangé et une
voiture de 128 × 64 px. Ces compositions sont des aperçus graphiques, pas
des captures du moteur ni une preuve des collisions de la nouvelle géométrie.
Reproduction : `python -X utf8 tools/reviews/build_u01_city_review.py --revision r03`.
