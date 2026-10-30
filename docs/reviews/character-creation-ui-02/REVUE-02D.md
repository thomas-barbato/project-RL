# CREATION-UI02-D r03 — séparations retirées

Le joueur valide r03 par « oui ca me plait, tu peux l'adapter :) ». La version
est maintenant intégrée au lancement normal Debug et Release, y compris
`Jouer.cmd`, sans argument de revue. Les contrôles et l'empreinte du Release
sont dans `integration-02D-r03.json`.

À la demande du joueur, r03 retire la séparation sous le titre avec son court
accent bleu, ainsi que la séparation au-dessus des boutons. La disposition,
les textes, les zones cliquables et la scène restent ceux de r02.
Les captures actuelles sont dans `industrial-02D/r03/` sous l'aperçu courant.
Les comparaisons en petit format et Terminal ne diffèrent que sur les lignes
retirées ; le grand format bleu comporte aussi 12 pixels de libellé différents.
Le contrôle du rendu normal est identique. Les empreintes et résultats sont
dans `bar-removal-checks-02D-r03.json`.

Les mesures de lecture et de cache ci-dessous ont été effectuées sur r02,
dont la géométrie et les ressources restent inchangées. Le registre courant
est `PROPOSITION-02D.json`, l'ancien `PROPOSITION-02D-r02.json` est conservé.

## Révision r02

Maquette native du 30 octobre 2026. Le joueur apprécie le premier rendu
industriel, puis demande de retirer les traits épais qui parasitent l'écran.
La révision r02 conserve la disposition compacte, les matières, les attributs
en deux lignes et la petite scène du personnage.

Le contour de chaque plaque est désormais simple. Les ombres décalées de
trois pixels sont retirées ; les reflets utilisent les pixels du contour au
lieu d'ajouter un trait parallèle. Les accents de deux ou trois pixels passent
à un pixel. Le logement des classes perd son contour supplémentaire et les
petites marques d'usure du cadre extérieur sont retirées.

Les images finales sont dans
`artifacts/character-creation-review-02/current/industrial-02D/r02/` :
`metamorphe/`, `arcaniste-small/`, `terminal/` et `reduced/`.
`native/` conserve les références r01 mentionnées par le retour du joueur.

Les contrôles natifs sélectionnent les 16 fiches des quatre classes :

- **1280 × 800, bleu et Terminal :** toutes les cartes et les 16 descriptions
  tiennent sans défilement ; la scène est visible.
- **960 × 540 :** toutes les cartes et 15 descriptions tiennent ; seule Mue
  prédatrice conserve un défilement de 208 px. La scène est masquée.
- La partie, le tour et le générateur aléatoire restent inchangés.
- Les trois ressources de scène sont préparées dans le cache commun avant
  affichage : 278 528 octets de textures, dans un cache borné à 128 Mio.
  La petite fenêtre ne prépare initialement aucune ressource de scène.
  L'agrandissement puis le retour au petit format sont contrôlés.
- La réouverture réutilise les ressources. Les 48 lectures de fiche n'ajoutent
  ni lecture de source ni création de texture au chemin de dessin.
- Le personnage et le décor de la scène sont identiques pixel pour pixel à
  r01. Le contrôle normal reste également identique. L'image avec mouvement
  réduit présente 12 pixels différents dans les libellés ; sa scène est
  identique. L'égalité complète de ces deux images n'est pas revendiquée.

Le build Debug, la vérification de compilation Release et le formatage des fichiers
modifiés passent. Dix tests ciblés du chemin normal sont passés avant la seule
correction des épaisseurs ; les contrôles natifs ci-dessus portent sur r02.
La suite complète n'a pas été exécutée. Les sources du camp, les sept fichiers
personnels contrôlés et l'exécutable Release disponible sont inchangés.

Une première génération r02 a été lancée avant la fin de compilation et a
bloqué le remplacement de l'exécutable sous Windows. Elle a été retirée et
entièrement régénérée après compilation réussie. Le script de capture impose
désormais cette séquence et conserve les empreintes du code et de l'exécutable.

Ces contrôles de prototype précèdent l'accord du joueur sur r03 et son
intégration normale, consignés dans `acceptance-02D-r03.json` et
`integration-02D-r03.json`.
