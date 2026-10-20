# ANIM-FLOW-01 — contours naturels des animations

**Accord du 20 octobre 2026 :** le joueur valide le rendu plus visible, puis le rythme ralenti et son application à toutes les compétences. L’accord, sa portée et le contrôle du raccordement sont dans [`../animation-policy-01/`](../animation-policy-01/). Le contenu ci-dessous conserve la proposition présentée avant cet accord.

Retour du joueur le 20 octobre 2026 : il refuse la forme en croix et, plus
généralement, les animations découpées géométriquement par la taille des
cases. Il accepte que le dessin dépasse la zone concernée et demande un
rendu plus naturel. Cette direction ne valide pas encore de nouveaux pixels,
les chiffres de combat ou les réglages précédents de vitesse et de contraste.

## Proposition présentée avant intégration

- **ANIM-FLOW-01-01** : éclatement de glace sur sol libre, dessin entier au
  lieu de sa découpe en croix.
- **ANIM-FLOW-01-02** : explosion de feu, dessin entier avec flammes et fumées
  libres autour de sa zone.
- **ANIM-FLOW-01-03** : explosion de poison, volutes complètes sans bord de case.
- **ANIM-FLOW-01-04** : même éclatement de glace contre une cloison physique ;
  elle arrête la portion concernée sans réduire ni étirer toute l’explosion.

Les quatre propositions restent **en attente de validation visuelle**. Les
paramètres de dégâts et les zones mécaniques sont inchangés. L’aperçu peut
afficher leur contour séparément pour distinguer la zone touchée du dessin.
Pour FR06 au rayon un, le moteur utilise actuellement un losange de cinq
cases ; sa silhouette de dégâts n’oblige plus le dessin à former une croix.

## Méthode et limites

Réutilisation des atlases existants sans modification des PNG. Les six
images complètes de chaque séquence sont dessinées avec un cadrage commun
et un facteur d’échelle uniforme. Leur contour peint, leurs marges
transparentes et leurs proportions sont conservés. Le gabarit proposé de
la masse peinte est de 215,04 px pour une emprise nominale de 192 px : il
s’agit d’un débordement visuel de revue, pas d’une nouvelle portée.

Le masque de rendu autorise le sol libre visible et en ligne de vue depuis
le centre. Il ne reprend pas le masque des cellules qui reçoivent les dégâts.
Les cloisons et les cellules non visibles restent interdites. La scène est
un diagnostic contrôlé de 7 × 5 cases natives de 64 px, pas une capture de
campagne. Le raccordement futur devra respecter les masques de perception
et de propagation de chaque effet, ainsi que les corps et les toits du moteur.

Les deux colonnes ont la même phase et la même intensité. Le ralentissement
proposé précédemment s’applique aux deux colonnes et peut être désactivé ;
il n’est toujours pas accepté. Le contraste n’est pas modifié dans cette
revue afin de permettre un jugement propre de la forme.

Les fonctions de combat et de rendu de production restent intactes. Le
diagnostic ajoute seulement un point d’entrée debug et réutilise le décor,
les personnages et les captures de revue. Aucun fichier de sauvegarde de
campagne n’est lu ni écrit ; l’état moteur est comparé avant et après.

## Reproduction

```powershell
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-animation-shape artifacts/refonte-agent-2026-10-06/animation-shape-01
python tools/reviews/build_animation_shape_review.py
```

L’aperçu courant est toujours `artifacts/refonte-agent-2026-10-06/apercu-animations.html`.
Les captures remplacent son contenu de revue ; les choix antérieurs restent
documentés dans `docs/reviews/animation-readability-01/` et leurs sources sont
conservées. Les mesures et contrôles effectifs vont dans `validation.json`.
