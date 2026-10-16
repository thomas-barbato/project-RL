# Drones sans durée d'activité limitée

6 octobre 2026. Révision de la [proposition des drones](REVUE_DRONES_AGENT_LOT_05.md).
Ce document enregistre les décisions de conception acceptées ; aucune modification du jeu
ni des sprites n'est intégrée.

## Décisions acceptées

Après sa demande concernant la Mule et la recommandation pour les cinq modèles,
le joueur répond : « oui ok tu peux continuer :) ». Cet accord retient les
règles présentées et le remplacement du passif Autonomie par Maintenance.

### D57 Invocation de la Mule

La Mule ne disparaît plus après un nombre de tours. Sa capacité de transport
et ses PV continuent à progresser avec sa compétence et les modificateurs.

La proposition précédente prévoyait déjà de déposer son chargement au sol
à l'expiration : les objets ne devaient pas disparaître. Retirer le délai
évite néanmoins d'imposer des déploiements et des transferts répétés.

### DRP01 Persistance des cinq modèles

**Règle acceptée :** retirer le délai d'expiration des cinq modèles.
Ils conservent leurs PV et peuvent toujours être détruits. Le 15 octobre,
le joueur remplace la limite de cinq drones et d’un exemplaire par modèle
par une capacité de contrôle commune qui progresse avec la branche Drones.
Plusieurs exemplaires d’un modèle sont permis, sauf l’Éclaireur et la Mule,
chacun limité à un exemplaire actif. La précision sur la Mule est confirmée
par `D57-LIMIT01-ACCEPT01` le 15 octobre.
Les valeurs restent à examiner dans la [révision Armée et provocation](ARMEE_DE_DRONES_ET_PROVOCATION.md).

Le déploiement coûte de l'énergie, sans entretien en énergie à chaque tour.
Le rappel conserve les PV restants et le chargement ; redéployer un drone
rappelé ne le soigne pas. Les drones suivent le joueur lors d'un changement
de couche en conservant ces données. Le coût et l'interface des commandes
restent à détailler ; aucun rappel ne soigne gratuitement.

Si la Mule est détruite, son chargement est déposé au sol pour être récupéré.
Le rappel et les transitions ne détruisent, ne dupliquent et ne transfèrent
pas automatiquement les objets dans l'inventaire du joueur.

### DP01 Maintenance des drones

Le passif de durée est remplacé par **Maintenance des drones**, qui répare
progressivement leurs PV hors combat. Le nom et ce rôle sont acceptés.
Les dix rangs appris et les bonus d'équipement améliorent la réparation.

Le nouveau texte, les valeurs, la définition exacte du hors combat et le
rendu de la réparation sont proposés dans la
[fiche DP01 révisée](REVUE_DRONES_AGENT_LOT_05.md#dp01-maintenance-des-drones).
Ils attendent une validation distincte. L'identifiant DP01 est conservé.

Blindage des drones (DP02) conserve son rôle de réduction des dégâts reçus
en pourcentage ; Lien renforcé (O03) conserve son bonus de dégâts en pourcentage.
Les autres propositions chiffrées restent soumises à leur revue distincte.

## État de la revue

La persistance des cinq modèles, la conservation des PV et du chargement,
le dépôt du chargement à la destruction de la Mule et le nouveau rôle de
DP01 sont acceptés. Aucun nouveau descriptif roleplay, barème de réparation
ni rendu visuel n'est approuvé par cette discussion. La
[première revue des drones](REVUE_DRONES_AGENT.md) conserve l'historique des
anciennes durées ; les fiches actuelles utilisent désormais la persistance.
