# Vérification de l’essai d’expédition

2 octobre 2026. L’essai propose une sortie dans la campagne existante avec une sauvegarde indépendante. Le lanceur est `Essayer_expedition.cmd` à la racine du projet ; le guide est `docs/ESSAI_EXPEDITION.md`.

## Vérifications effectuées

- `cargo test --locked --bin project-rl playable_trial -- --nocapture` : 4 tests réussis, 0 échec. Comparaison intégrale de la suspension avant et après activation du profil ; création de personnage normale ; reprise vérifiée et redémarrage dans le même emplacement ; déplacement réel jusqu’au secteur industriel, ramassage, retour en ville et restauration identique de l’état moteur. Le repère de direction suit l’accès au sud-est, le retour surface puis le retour en ville.
- `cargo check --locked --all-targets` : réussi.
- `cargo fmt --all -- --check` et `git -c core.autocrlf=false diff --check` : réussis.
- `cargo build --locked --release --bin project-rl` : réussi. Deux avertissements de code inutilisé dans les modules existants, sans erreur de compilation.

La suite complète n’a pas été exécutée. Les essais de trajet utilisent la graine initiale ; ils ne valident pas toutes les rencontres ni l’équilibrage des trois protocoles.

## Captures du rendu natif

Les images viennent du framebuffer du jeu, avec les diagnostics `--ui-cold-playtest-*`. Les déplacements de préparation passent par les commandes du moteur ; aucun acteur n’est retiré, aucun inventaire n’est injecté et aucune téléportation n’est utilisée. Les diagnostics utilisent un emplacement temporaire pour la suspension, sans charger ni écrire la sauvegarde utilisateur.

Captures finales examinées :

- `captures/menu-v1/cold-start.png` : menu en 1280 × 800.
- `captures/menu-960-v1/cold-start.png` : menu en 960 × 540.
- `captures/creation-v1/cold-start.png` : choix normal du protocole.
- `captures/ville-v2/cold-start.png` et `captures/ville-960-v2/cold-start.png` : objectif et direction vers le souterrain, aux deux tailles.
- `captures/friches-v2/cold-start.png` : sortie de ville.
- `captures/souterrain-v2/cold-start.png` : arrivée, ramassage et repère du retour surface.
- `captures/retour-v2/cold-start.png` : retour en ville confirmé, puis poursuite libre.

Les captures de ville, friches, souterrain et retour en `v1` précèdent la correction du repère de direction. Utiliser leurs versions `v2` pour examiner le résultat final.

## Limites de l’essai

Le parcours traverse les grandes cartes actuelles. Il faut encore juger en jouant la distance, le rythme, les combats et l’intérêt du butin. Le trajet automatique de capture rentre avec 4 PV sur 20, sans utiliser les soins ni acheter de compétences : cela confirme un retour possible dans ce scénario précis, sans prouver que la sortie est bien équilibrée.

Le mode texturé et l’enrichissement de l’éditeur restent en pause. L’essai conserve le format de sauvegarde, la génération et le contenu de la campagne.
