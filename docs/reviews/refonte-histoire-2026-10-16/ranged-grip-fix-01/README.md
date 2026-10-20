# GRIP-FIX-01 — faux troisième bras avec une arme de tir

20 octobre 2026. Signalement du joueur : « quand on équipe une arme de tir,
il semble que le joueur ait un 3eme bras ».

## Cause et correction

Les anciens masques des bras réutilisés par ART-FINISH-COMPLETE-06 descendent
au-delà des poignets et comprennent des pixels du haut des jambes. La pose
générique à deux mains transforme ces fragments avec le bras : le fusil de
patrouille révèle particulièrement une excroissance diagonale sous sa main
de soutien. Le défaut existe sur les cinq apparences de classe, y compris
leurs variantes de couleur.

Le compositeur conserve désormais le bras jusqu'à la marge des doigts déjà
définie autour du point d'attache, et exclut les fragments situés plus bas.
Les trois poses peintes séparément FIN-03/04/05 disposent déjà de bras isolés
et conservent leurs coudes fléchis. Cette correction réutilise les dessins
acceptés et leurs points d'attache : aucun PNG source, palette, nom, objet,
statistique, équipement ou format de sauvegarde n'est remplacé.

Le chemin commun au plateau et à l'inspection applique ce découpage avant la
transformation des bras. Les poses exactes acceptées, les poses à une main
et les personnages sans arme restent identiques. Les prochaines extractions
doivent isoler les membres sans inclure les jambes ou le bas du vêtement.

## Vérification

- Comparaison de quarante exports du compositeur réel : cinq classes,
  deux couleurs, sans arme, au pistolet, au fusil de patrouille et au
  lance-flammes. Les vingt poses sans arme ou au pistolet sont identiques ;
  les vingt poses à deux mains sont corrigées. Tête et bas des jambes restent
  identiques dans les quarante cas.
- Contrôle spécifique de la région du membre parasite : pixels présents
  avant correction, absents après sur les dix comparaisons du fusil.
- Test de non-régression sur les seize tenues utilisant ces anciens masques,
  avec trois palettes ; tests existants de prises, couleurs, jambes fixes,
  apparence de classe et reprise.
- 37 tests automatiques réussis ; le diagnostic natif, ignoré par défaut,
  a également été exécuté explicitement avant et après correction.

La planche `comparison-native.png` présente GRIP-01 (Bastion) et GRIP-02
(Traqueur) à leur échelle de jeu. `pixel-checks.json` conserve les quarante
comparaisons. Les exports CPU utilisent la convention verticale inversée
de `Image::export_png` ; l'outil de présentation restitue leur orientation
normale sans rééchantillonner la planche native. Les vues agrandies servent
uniquement au diagnostic. Il ne s'agit pas d'une capture d'une partie jouée.
