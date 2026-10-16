# Nettoyage du 16 octobre 2026

Opération autorisée par le joueur après examen du rôle de `artifacts/` et `target/`.
Gain net estimé après conservation des sources : **8,3 Go**. La mesure de place
libre peut varier légèrement avec l’IDE et les autres applications.

| Dossier | Avant | Après |
| --- | ---: | ---: |
| `target/` | 7,01 Gio | environ 281 Mio |
| `artifacts/` | 3,04 Gio | 2,8 Mio |

Les 39 328 fichiers inventoriés ont été supprimés. Les neuf fichiers du dernier
contrôle natif, recréés pour vérifier le jeu après nettoyage, ont ensuite été
supprimés également. Les 13 jonctions historiques ont été retirées sans parcourir
leurs cibles. Les cartes, préférences et récupérations de l’éditeur, la page
d’aperçu courante et les exécutables principaux sont conservés.

Les images nécessaires à la compilation ont rejoint `assets/animations/agent/`.
Les dessins sources supplémentaires sont dans `assets/source-art/`, les décisions
et validations dans `docs/reviews/`, les scripts historiques dans `tools/archive/`.
Les doublons d’images identiques réutilisent une même copie. Les archives de dessin
conservent leur statut historique : une proposition refusée ne devient pas active.

Les 6 071 images distinctes préservées ont été vérifiées par SHA-256. Aucun pixel
ni règle de jeu n’a été modifié. Aucun fichier requis par les inclusions Rust ne
dépend encore de `artifacts/` et aucune inclusion n’est manquante.

Vérifications : compilation debug, contrôle release, 949 tests moteur et 231 tests
ciblés d’interface réussis ; formatage et contrôle du diff réussis. Le laboratoire
a été exécuté avant et après suppression, sans erreur ni modification de la
campagne. La suite globale historique, dont des échecs restaient à examiner,
n’a pas été rejouée.

L’IDE a recréé un cache de vérification après la suppression. Ce cache récent est
laissé en place ; la compilation incrémentale et les informations de débogage
sont désactivées dans les profils de développement et de test pour réduire
l’accumulation future. Les nouveaux aperçus et captures sont ignorés par Git.

Le dossier `.git` reste inchangé et occupe environ **6,8 Gio**. Ses anciens objets
ne sont pas supprimés par un nettoyage de `target/` et `artifacts/`. Aucun commit,
ajout à l’index, push ou changement de l’historique n’a été effectué.

`plan.json` conserve les correspondances de chemins et empreintes d’origine.
`validation.json`, `post-cleanup.json` et `summary.json` conservent les résultats.
Les grandes listes de fichiers temporaires ont été retirées du registre une fois
leur suppression vérifiée ; leurs nombres et totaux restent consignés.
