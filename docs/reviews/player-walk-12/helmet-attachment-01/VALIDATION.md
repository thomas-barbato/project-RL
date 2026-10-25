# WALK12 — attache des casques pendant la marche

Révision courante : WALK12-HELMET-ORIENTATION-r02, 25 octobre 2026.

Le joueur signale que le casque équipé flotte à côté du personnage pendant
le déplacement et demande de le prendre en compte dans l'animation.

## Cause et correction

`walk_helmet` plaçait le casque sur le centre du torse du rig, avec la hauteur
fixe de tête de la tenue frontale. Les figures WALK12 ont leurs propres positions
de tête selon l'orientation et la phase ; les exports du compositeur reproduisent
le casque décalé horizontalement et parfois au-dessus de la tête.

La première correction, WALK12-HELMET-ATTACHMENT-r01, ne traitait que le point
d'attache. Le joueur précise ensuite « il doit aussi suivre le mouvement ».
Elle laissait une coque frontale sur une tête en trois quarts ; les modèles
ouverts copiaient également des pixels de visage frontal depuis la tenue fixe.
Leurs tests de contact ne contrôlaient pas cette orientation.

Le point d'attache reste mesuré sur le haut de la tête de la figure
de marche, avant ajout du casque. La mesure ignore les franges d'alpha faibles
et s'arrête avant le cou et les épaules. Le bord supérieur du casque suit le
sommet de cette tête, et son centre horizontal suit la calotte. La transformation
utilise les mêmes centres de pixels que les autres accessoires de marche.

La révision02 projette la coque existante sur une surface arrondie orientée
vers le côté de marche. Elle suit aussi l'inclinaison mesurée entre le haut de
la tête et le cou dans chaque phase, sans ajouter d'oscillation indépendante.
La coque réutilise les matières existantes ; le côté nouvellement visible
reprend son bord sombre. Les ouvertures restent transparentes et laissent
apparaître le visage réellement dessiné pour cette pose. La copie du visage
statique est supprimée. Aucun nouveau dessin ni aucun changement des ressources
acceptées n'est requis.

Les six modèles de casque, leurs matériaux et les dessins acceptés sont
réutilisés. La correction s'applique aux six phases et aux deux orientations
des25 tenues. Le chemin frontal, les déplacements verticaux, la cadence,
les dessins de marche et les règles d'équipement sont conservés.

## Contrôles

- Le diagnostic Rust avant correction utilise le compositeur existant et
  reproduit le décalage. Les exports après correction utilisent le compositeur
  normal, sans remplacement graphique dans un navigateur.
- Le test `equipped_helmets_stay_on_the_painted_head_through_every_pose` contrôle
  2 100 poses équipées :25 tenues × six casques × deux directions × sept poses,
  dont les deux références de repos latéral. Il vérifie la hauteur du casque,
  son contact avec la silhouette de tête et l'absence de modification du torse
  ou des jambes. Il contrôle également600 montages avec hache à deux mains
  ou paire d'armes à une main : l'arme ne déplace pas la tête équipée.
- Le test distingue les projections gauche/droite pour les six modèles et
  vérifie que les visières cyan des deux casques techniques pointent du côté
  de la marche. Pour les modèles ouverts, les pixels laissés visibles par la
  coque doivent rester ceux du personnage de marche, sans visage frontal ajouté.
- 175 comparaisons de vue frontale sont identiques avant/après. Les25 atlas
  complets sans casque sont également identiques, pixel pour pixel.
- Les51 empreintes des ressources WALK12 acceptées sont inchangées.
- Les planches couvrent les six casques sur les cinq tenues de chaque classe,
  à taille native64, dans les deux sens et les six phases. Le GIF compare les
  cinq classes avec la tenue1 et le casque5, avant/après la correction
  d'orientation. Il conserve400ms par
  case ; les pauses de300ms permettent seulement de lire l'arrêt dans l'aperçu.

Les résultats du test, de compilation et l'empreinte de l'exécutable livré
sont consignés dans `validation.json`. L'export de200 atlas orientés réussit
en76,33s. Le formatage et `git diff --check` sont également contrôlés.

Les aperçus proviennent des pixels du compositeur Rust, contrôlés hors fenêtre.
La correction n'a pas été testée par contrôle interactif d'une partie ni par
chargement de la sauvegarde du joueur.

Reproduction :

```powershell
$env:RL_WALK_HELMET_OUTPUT = 'artifacts/walk-helmet-02/after'
cargo test --locked --bin project-rl walk_helmet_alignment_diagnostic -- --ignored --test-threads=1
cargo test --locked --bin project-rl equipped_helmets_stay_on_the_painted_head_through_every_pose -- --test-threads=1
C:\Python314\python.exe tools/diagnostics/review_all_walk_helmets.py
```

Le nettoyage demandé le25 octobre2026 retire les exports bruts, les anciens
comparatifs et les journaux temporaires après validation. Les résultats chiffrés,
empreintes et accords restent ici ; l'aperçu d'ensemble des six casques reste
disponible. La commande ci-dessus régénère les montages courants. Le script
historique `review_walk_helmets.py --oriented` requiert en plus les anciennes
images avant correction et ne constitue pas le contrôle de régression courant.
Le registre est dans `docs/maintenance/nettoyage-2026-10-25/`.
Le jeu ne dépend d'aucun de ces fichiers de diagnostic.
