# Nettoyage du 30 octobre 2026

À la demande du joueur, retrait des fichiers générés devenus inutiles après
les validations des animations, puis des caches de compilation temporaires.

- **8 500 fichiers supprimés**, soit **2 559 319 787 octets logiques** :
  6 541 exports (448 969 676 octets) et 1 959 fichiers Cargo
  (2 110 350 111 octets).
- Suppression directe des cibles inventoriées et contrôlées ; aucune nouvelle
  archive et aucun vidage de la Corbeille.
- **33 800 fichiers protégés vérifiés avant/après, aucune modification** :
  sources, dessins de fabrication, ressources, décisions, revues encore en
  attente, cartes de l'éditeur, sauvegardes, préférences et distribution.
- Release reconstruit avec `cargo build --locked --release --bin project-rl`,
  puis exécutables et lanceurs conservés. Le prochain outil Debug recompilera
  son profil. La distribution existante n'est pas reconstruite.
- Inventaire actuel : **3 664 ressources**, sans dépendance aux captures.
  Les **1 845 empreintes** de la banque verticale C12 sont intactes ; le paquet
  existant passe son contrôle d'intégrité.
- Les aperçus C11/C12 restent accessibles : 930 ressources de navigateur et
  84 références de fabrication vérifiées, aucun fichier manquant. Les pages
  historiques C02–C10 conservent quelques références natives et donnent accès
  aux démonstrations intégrées. Le contrôle HTTP local réussit.
- La finition **CREATION-UI02-B reste en attente de validation** et désactivée
  dans le jeu normal. Ses GIF, WebP, PNG et captures finales sont conservés,
  ainsi que la disposition 02-A acceptée. DUAL17-C02 et le trajet souris encore
  en revue restent également disponibles.

Les tailles sont celles des fichiers, pas une mesure de variation de l'espace
libre du volume : les liens physiques Cargo et les nouveaux bilans peuvent
faire différer l'espace effectivement récupéré.

Le plan complet, avec les chemins et empreintes retirés, est `plan.json`.
Les résultats sont dans `exports-result.json`, `cache-result.json`,
`preservation-check.json`, `validation.json` et `result.json`.
Les anciennes références aux captures supprimées peuvent être retrouvées dans
ce plan ; les scripts d'origine permettent de régénérer les exports.

`git diff --check` réussit. Aucun commit, push ou changement des règles de jeu.
