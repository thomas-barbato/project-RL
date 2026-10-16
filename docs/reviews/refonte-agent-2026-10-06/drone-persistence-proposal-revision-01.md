# Drones sans durée d'activité limitée

6 octobre 2026. Révision de la [proposition des drones](REVUE_DRONES_AGENT_LOT_05.md).
Ce document prépare une décision de conception ; aucune modification du jeu
ni des sprites n'est intégrée.

## Décision exprimée et recommandation

Le joueur demande une durée illimitée pour la Mule et demande un avis sur
l'extension de cette règle à tous les drones. La première demande est retenue ;
l'extension aux quatre autres modèles reste proposée.

### D57 Invocation de la Mule

La Mule ne disparaît plus après un nombre de tours. Sa capacité de transport
et ses PV continuent à progresser avec sa compétence et les modificateurs.

La proposition précédente prévoyait déjà de déposer son chargement au sol
à l'expiration : les objets ne devaient pas disparaître. Retirer le délai
évite néanmoins d'imposer des déploiements et des transferts répétés.

### DRP01 Persistance des cinq modèles

**Recommandation à valider :** retirer le délai d'expiration des cinq modèles.
Ils conservent leurs PV et peuvent toujours être détruits. La limite déjà
acceptée reste cinq drones déployés, avec un seul exemplaire par modèle.

Le déploiement coûte de l'énergie, sans entretien en énergie à chaque tour.
Le rappel conserve les PV restants et le chargement ; redéployer un drone
rappelé ne le soigne pas. Les drones suivent le joueur lors d'un changement
de couche en conservant ces données. Ces commandes restent proposées.

Si la Mule est détruite, son chargement est déposé au sol pour être récupéré.
Le rappel et les transitions ne détruisent, ne dupliquent et ne transfèrent
pas automatiquement les objets dans l'inventaire du joueur.

### DP01 Remplacement proposé d'Autonomie des drones

Le passif de durée n'aurait plus d'effet avec cinq drones persistants.
Proposition : le remplacer par **Maintenance des drones**, un passif qui
répare progressivement leurs PV hors combat. Les dix rangs appris et les
bonus d'équipement amélioreraient la réparation.

Le nom, le texte destiné au joueur, les valeurs, la définition exacte du
hors combat et le rendu de la réparation restent à présenter et à valider.
L'identifiant DP01 est conservé pour suivre cette révision.

Blindage des drones (DP02) conserve son rôle de réduction des dégâts reçus
en pourcentage ; Lien renforcé (O03) conserve son bonus de dégâts en pourcentage.
Les autres propositions chiffrées restent soumises à leur revue distincte.

## État de la revue

La demande de durée illimitée de D57 est enregistrée. DRP01 et le remplacement
de DP01 attendent l'accord du joueur. Aucun nouveau descriptif roleplay ni
rendu visuel n'est approuvé par cette discussion. Les anciennes tables de
durée restent l'historique de la proposition précédente ; les tables et
documents liés seront révisés selon la décision retenue.
