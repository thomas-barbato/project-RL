# INTERFACE-BLUE-TERMINAL01

Le 26 octobre 2026, le joueur demande puis confirme la suppression des thèmes
supplémentaires : conserver le bleu pour l'interface classique et l'habillage
Terminal pour l'interface terminale.

Le menu Affichage propose désormais un seul choix d'habillage : **Classique bleu**
ou **Terminal PC**. La ligne indépendante de couleur, son enum et son état de
rendu sont retirés. Les contrôles suivants et leurs actions sont décalés ensemble.
Les surfaces, icônes et confirmations existantes sont conservées.

Le chargeur accepte les quatre anciennes valeurs de `interface_theme`, ignore
uniquement ce champ retiré et conserve tous les autres réglages, y compris
`interface_style: terminal`. Les fichiers nouvellement enregistrés n'écrivent
plus de couleur. Les champs inconnus et anciennes couleurs invalides restent
refusés. Le rendu du monde, les règles et les sauvegardes de partie sont inchangés.

Les diagnostics et lecteurs réutilisent les deux habillages réellement disponibles.
Les contrôles de migration couvrent les quatre anciennes couleurs, les deux
habillages et la conservation des autres préférences. Les tests de menu couvrent
le clic, les flèches, l'application, la confirmation, l'annulation, le retour aux
valeurs par défaut et le retour sans application.

Les captures natives de référence et leurs journaux sont conservés sous
`artifacts/interface-blue-terminal-01/`. Aucun réglage personnel n'est réécrit
par ces diagnostics.

## Police Terminal

Le joueur ajoute une demande de lisibilité le même jour. La police de secours
du mode Terminal est remplacée par **JetBrains Mono Regular et Bold**, sans
changer Atkinson Hyperlegible dans l'interface classique. Les mesures, le dessin,
le centrage et la préchauffe des caractères français utilisent les fontes actives.
La présentation monospace, les surfaces phosphore et les tailles de texte sont
conservées. Les accents et la ponctuation ne dépendent plus de la police de secours.

Les fontes proviennent du dépôt officiel `JetBrains/JetBrainsMono`, commit
`19371302b95d218af43299bce79ddbddd0bc364d`. Les fichiers sources, leurs URL et
empreintes sont consignés dans `assets/fonts/JetBrainsMono/SOURCE.json`.
Le copyright et la licence SIL OFL 1.1 sont conservés avec les fontes et ajoutés
à l'inventaire de distribution. Les fontes sont embarquées dans l'exécutable.

Captures finales Terminal : `terminal-font-menu/`, `terminal-font-small/`,
`terminal-font-largeui/`, `terminal-font-creation/` et
`terminal-font-inventory-live/`. Le rendu de l'interface bleue est vérifié dans
`graphics-blue/`, `graphics-blue-small/` et `main-blue/` ; les premières captures
Terminal sans préfixe `terminal-font-` précèdent le changement de police.

Le vieux diagnostic générique `inventory` ne peut pas construire sa fixture car
il sélectionne une classe Brèche retirée de la création. Cet échec est conservé
dans `terminal-font-inventory/stderr.log` ; il ne valide pas l'inventaire.
Le diagnostic de rendu réel `terminal-inventory-live-terminal` le remplace pour
cette revue : code 0, image inspectée et `state_unchanged: true`.

Validation finale : **22 tests ciblés réussis**, compilation verrouillée de toutes
les cibles, formatage et diff sans erreur. Les builds Debug et Release intègrent
les fontes ; la version de `Jouer.cmd` est à jour. L'inventaire de distribution
est vérifié (1815 fichiers, avec les deux notices ajoutées). Aucun paquet portable
n'est reconstruit. Les empreintes et résultats sont consignés dans `validation.json`.
