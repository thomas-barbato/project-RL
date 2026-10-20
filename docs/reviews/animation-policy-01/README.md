# ANIM-POLICY-01 — lisibilité, rythme et contours des compétences

Le 20 octobre 2026, le joueur valide les versions « plus visibles », puis les
ralentissements proposés et demande d’appliquer cette politique à toutes les
compétences. Les trois messages et les références natives sont conservés dans
`acceptance.json`. Cet accord porte sur le rendu et le temps de présentation.

Le raccordement utilise les mêmes dessins embarqués dans `assets/`. Aucun
texte de compétence, paramètre de combat, tour, coût, dégât, rayon mécanique,
recharge, statut, tirage aléatoire ou format de sauvegarde n’est modifié.

## Politique intégrée

- Les séquences transitoires durent généralement 50 % plus longtemps. Boule
  de feu conserve le facteur proposé de 1,30 et Frappe de rupture celui de
  1,35. Un pas dure 120 ms ; une réaction à un coup, 252 ms. Les cycles des
  effets persistants durent 30 % plus longtemps, indépendamment des tours.
- Les échéances de suppression et les retards des réactions suivent le même
  temps que le projectile. Une nouvelle commande reste immédiatement possible.
  Un nouveau pas repart de la position affichée, sans file d’attente.
- La matière des effets est éclaircie séparément du décor et des acteurs.
  Gain général de 1,18, projectile occulte 1,30, drain occulte 1,40, glace 1,12.
  Le gain agit sur la couleur de l’effet avant mélange ; le prototype de
  contraste utilisait une différence entre deux scènes composées. Le moteur
  n’a besoin ni d’une copie du framebuffer ni d’une correction générale du sol.
- Blizzard reprend la palette de glace et les opacités présentées : 0,42
  pour l’avertissement, 0,94 pour la tempête. Sa séquence entière garde une
  emprise nominale stable et le débordement de 10 % accepté.
- Les explosions de glace, feu, poison et les autres dessins de zone gardent
  leur image entière et leurs proportions. Le dessin peut déborder de la zone
  mécanique. Son masque dépend du sol actuellement visible et de l’occlusion
  physique, avec une texture conservée en cache ; les cellules de dégâts ne
  servent plus de ciseaux. Les marques de statut restent attachées au porteur.
- Les mouvements réduits conservent une pose lisible. Le masque commun est
  recalculé avec les cellules autorisées actuelles : une porte ou la perception
  qui change ne réutilise pas une ancienne autorisation de débordement.

## Contrôles

`pixel-checks.json` et `production-captures.json` conservent les résultats
des comparaisons de production sur sols sombre et clair. Les images natives
sont dans `integration-native/`. La galerie existante reste
`artifacts/refonte-agent-2026-10-06/apercu-animations.html` ; le jeu n’en dépend pas.

```powershell
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-blizzard-readability artifacts/refonte-agent-2026-10-06/animation-policy-01/blizzard-visibility-01
.\target\debug\project-rl.exe --ui-cold-animation-shape artifacts/refonte-agent-2026-10-06/animation-policy-01/animation-shape-01
.\target\debug\project-rl.exe --ui-cold-animation-readability artifacts/refonte-agent-2026-10-06/animation-policy-01/animation-readability-01
python tools/reviews/finalize_animation_policy.py
.\target\debug\project-rl.exe --ui-cold-skill-demo artifacts/refonte-agent-2026-10-06/animation-policy-01/all-skills --animation-policy
```

Le dernier diagnostic rejoue chaque compétence active dans sa démonstration
intégrée, en mode normal puis mouvements réduits, sans modifier l’état, le RNG
ou l’historique de la partie hôte. Les contrôles et leurs limites sont consignés
dans `validation.json`. Les captures RGBA sont temporaires et les sources des
dessins acceptés restent dans `assets/`.
