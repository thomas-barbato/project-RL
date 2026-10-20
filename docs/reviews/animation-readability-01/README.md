# ANIM-READ-01 — rythme et lisibilité

**Accord du 20 octobre 2026 :** le joueur valide le rendu plus visible, puis le rythme ralenti et son application à toutes les compétences. L’accord, sa portée et le contrôle du raccordement sont dans [`../animation-policy-01/`](../animation-policy-01/). Le contenu ci-dessous conserve la proposition présentée avant cet accord.

Proposition du 20 octobre 2026, **en attente de validation visuelle du joueur**.
Le joueur constate des animations trop rapides et parfois peu visibles dans
les environnements sombres. Cette revue prépare huit comparaisons natives
avant de toucher au lancement normal.

L’aperçu courant reste `artifacts/refonte-agent-2026-10-06/apercu-animations.html`.
Les images reproductibles vont dans son sous-dossier `animation-readability-01/`.

## Candidats

1. **ANIM-READ-01-01 / A01** : projectile occulte, 400 → 600 ms ; contraste de
   l’effet ×1,30.
2. **ANIM-READ-01-02 / OC05** : drain occulte, 300 → 450 ms ; contraste ×1,40.
3. **ANIM-READ-01-03 / FR06** : éclatement de glace, 420 → 630 ms ; contraste ×1,12.
4. **ANIM-READ-01-04 / SM02** : explosion de poison, 460 → 690 ms ; contraste ×1,18.
5. **ANIM-READ-01-05 / B01** : frappe de rupture et fin des éclats, 840 → 1 134 ms ;
   contraste ×1,18.
6. **ANIM-READ-01-06 / FE05** : boule de feu, trajet et explosion,
   900 → 1 170 ms ; intensité identique.
7. **ANIM-READ-01-07 / ACTOR-GLIDE** : pas ordinaire, 80 → 120 ms ; amplitude et
   dessin identiques.
8. **ANIM-READ-01-08 / HIT-REACTION** : réaction à un coup, 180 → 252 ms ;
   amplitude identique.

Ce sont des durées de présentation réelles, pas des tours de jeu ni une
modification du temps d’exécution décrit dans les fiches.

## Fabrication et portée

Le diagnostic appelle les fonctions de rendu de production avec leurs dessins
actuels, dans une scène isolée de 7 × 5 cases de 64 px. Il capture 72 phases
à 60 phases par seconde, avec un sol du jeu et des acteurs actuels. Deux
éclairages de revue sont contrôlés : sombre et plus clair. Ils ne prétendent
pas reproduire toute la campagne ni son éclairage dynamique.

Le prototype compose une scène avec effets et la même scène sans effets.
Le contraste suit `sans_effet + (avec_effet - sans_effet) × gain`, borné à
0–1. Les pixels sans effet gardent ainsi leur valeur ; les acteurs et le
décor ne reçoivent aucune correction générale. Cette méthode de composition
est une **proposition de rendu**, pas une architecture de production imposée.
Elle n’ajoute aucun halo, nouvelle silhouette ou extension de zone.

La galerie ralentit seulement la lecture des phases capturées. Le mode
« phase identique » permet de juger le contraste séparément ; désactiver le
contraste permet de juger seulement le rythme. Les deux colonnes restent à
taille native et utilisent les mêmes sources et le même éclairage.

Cette proposition n’est pas intégrée au lancement normal. L’accord devra
préciser les candidats acceptés, avec rythme et contraste distingués.
Textes de compétence et règles mécaniques ne sont pas concernés. La
respiration, les menus, les statuts et zones persistants restent à contrôler
séparément avant de conclure sur l’ensemble des animations.

Après accord, le raccordement doit conserver la synchronisation entre
projectile, impact, recul, marqueurs et suppression des animations terminées.
Les nouvelles commandes ne doivent pas attendre une animation ; les pas
rapprochés repartent de la position affichée, sans accumulation. Les règles
de visibilité, obstacles et mouvements réduits doivent être conservées.

## Reproduction

```powershell
cargo build --locked
.\target\debug\project-rl.exe --ui-cold-animation-readability artifacts/refonte-agent-2026-10-06/animation-readability-01
python tools/reviews/build_animation_readability_review.py
```

La capture compare l’état moteur complet avant et après sans exécuter de
commande de jeu. Elle ne lit ni n’écrit une sauvegarde de campagne. Les
résultats effectifs des contrôles sont consignés dans `validation.json`.
Le script utilise Pillow pour encoder les phases RGBA temporaires en PNG ;
ces fichiers bruts sont supprimés uniquement après réussite des contrôles.
