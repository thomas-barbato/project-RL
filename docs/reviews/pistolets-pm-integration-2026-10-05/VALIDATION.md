# Validation de l'intégration pistolets / PM

5 octobre 2026. Sources dans `C:\Users\User\Desktop\project-RL`.

## Comportement et compatibilité

- `tests-engine.log` : 817 tests moteur réussis, aucun échec. Deux assertions
  de taille du catalogue ont été mises à jour : 140 bases et 20 familles
  éligibles, après l'ajout de 12 modèles dans deux familles.
- `tests-firearms.log` : six tests client ciblés réussis. Ils couvrent les six
  niveaux de chaque famille, textes, art et slots ; 4 000 récompenses de monstre
  avec les douze modèles et les deux qualités ; stock blanc et paris enchantés
  aux six profondeurs ; dégâts, coût payé une fois par tir/rafale ; refus sans
  munitions ou à travers un mur ; équipement des trois slots et reprise des
  configurations active/inactive ; anciennes suspensions et empreintes.
- Pour les douze modèles, une attaque avec la main gauche laisse les deux autres
  slots équipés et n'emploie que l'arme choisie. Le tir coûte une munition pour
  le pistolet et trois pour le PM. Le fixture de mesure désactive la précision et
  la protection pour contrôler les dégâts bruts par projectile ; les règles
  normales s'appliquent en partie.
- Une reprise des configurations paire / deux mains conserve les instantanés
  moteur et les couleurs du rendu. Le fixture laisse 24 places pour ramasser les
  deux nouvelles armes avec l'équipement de départ.
- Les vraies suspensions 143, 144 et 145, enregistrées avant l'ajout du contenu,
  sont reprises et comparées à des parties régénérées : empreintes de règles,
  catalogue de butin et état exactement identiques. Aucun des douze IDs nouveaux
  n'entre dans les règles ou le butin de ces anciennes parties.

## Compilation et conditionnement

- `check-final.log` : `cargo check --locked --all-targets` réussi.
- `build-debug.log`, `build-release.log` : compilations debug et release réussies.
  Les avertissements de références inutilisées sont conservés.
- `rustfmt --check` sur les dix sources Rust modifiées et le nouveau module :
  réussi. Les fichiers étrangers à cette étape n'ont pas été reformatés.
- `git diff --check` : réussi.
- `build_cyberpunk_assets.py --check` et `build_ranged_families.py --check` :
  réussis. Les 12 entrées sont ajoutées sans déplacer les anciens index.
- `preservation.json` : 624 fichiers existants comparés ; aucune suppression,
  aucun PNG modifié, aucun ancien fichier de contenu modifié, aucun changement
  inattendu. Les dix sources concernées, le manifest et le README sont les
  seuls fichiers existants modifiés parmi les trois ensembles comparés.

## Captures natives

Les six captures indiquées dans README.md ont été produites par le client debug,
sans montage. Elles montrent la paire de tir, les deux orientations de paire
mixte, le fusil actif et les tooltips des deux nouveaux objets. Les illustrations
de pistolet jaune et de PM turquoise sont visibles dans le sac, les slots et le
tooltip. Les trois slots restent équipés ensemble ; la paire ou les deux mains
sont actives selon la sélection. Les scènes sont isolées des sauvegardes normales.

## Suite complète et performances

`tests-client-complets.log` : 649 tests exécutés avec huit threads, **605 réussis,
29 échecs, 15 ignorés**. Les 27 noms d'échec de l'étape précédente sont toujours
présents, sans disparition. `comparaison-tests-client.json` conserve les noms et
les résultats des deux contrôles isolés suivants :

- Le test de tir/reprise des anciens fusils utilisait le catalogue de butin
  historique 139 pour construire une suspension déclarée actuelle 146. La
  reprise refusait correctement cette incohérence. Le fixture utilise maintenant
  le catalogue actuel pour la reprise actuelle ; les tests de distributions
  historiques gardent leur projection. `tests-firearm-resume-final.log` : réussi.
- Le test de remplacement/reprise du menu a rencontré une carte aléatoire invalide
  (zone marchable déconnectée près de 149,15). Sa relance seule passe, sans
  modification du générateur ou du test (`tests-menu-resume-final.log`). Cette
  intermittence n'est pas déclarée corrigée ni prouvée préexistante.

La suite complète n'a pas été relancée après le seul changement de fixture ; son
résultat reste celui ci-dessus. Le contrôle de couverture des 162 objets passe
(`tests-coverage-final.log`). Les 13 tests graphiques, cinq tests de mains et
quatre tests d'inventaire passent dans la suite complète. La suite client n'est
donc pas entièrement verte.

`--ui-cold-performance` vérifie dix scènes de 120 frames et l'absence de mutation
de l'empreinte moteur. Résultat final dans `performance-final/timings.json`,
comparé au dernier diagnostic de l'étape précédente :

| Scène | Médiane CPU avant | Médiane CPU finale | Première frame avant | Première frame finale |
|---|---:|---:|---:|---:|
| Carte | 8,11 ms | 8,39 ms | 354,19 ms | 362,15 ms |
| Inventaire | 10,94 ms | 11,61 ms | 34,03 ms | 34,87 ms |

Ces mesures debug locales montrent respectivement +0,28 et +0,67 ms de médiane,
sans pic de décodage ajouté à l'ouverture de l'inventaire. Elles ne sont pas des
mesures release. Le premier diagnostic `performance` a été lancé pendant une
compilation/test ciblé ; il est archivé mais n'est pas utilisé pour la comparaison
finale, réalisée après leur arrêt. `comparaison-performance.json` garde les dix
scènes.

Aucun commit, push ou déploiement.
