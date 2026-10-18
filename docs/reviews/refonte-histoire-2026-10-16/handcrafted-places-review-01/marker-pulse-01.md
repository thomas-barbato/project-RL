# MARKER-PULSE-01 — Remplissage animé des petits repères

Proposition du 18 octobre 2026, en réponse à la demande d'un remplissage
coloré clignotant dans les petits cercles des zones et des trajets.
Ce nouveau rendu n'est pas encore intégré : il attend la validation visuelle.

L'aperçu `artifacts/walk-preview-review-01/cercles-animes.html` présente deux
animations APNG issues du moteur, sans redimensionnement ni retouche :
`marker-pulse/cercles-progressifs.png` et `marker-pulse/cercles-francs.png`.
Le prototype SVG initial est remplacé par ces captures du rendu réel.
Le mode `--ui-cold-marker-pulse <dossier>` active le remplissage seulement
pendant cette capture de revue. Il est inactif au lancement normal ; ses
crochets et son état temporaire ne sont pas compilés en release.
Les cases restent à 64 px. Le remplissage reprend la teinte actuelle de chaque
contour : bleu/cyan valide, rouge invalide et bleu pâle de déplacement.
Les contours, hachures, silhouettes et la coupure finale du trajet restent fixes.
Les numéros restent visibles. Aucun texte de jeu ni effet mécanique ne change.

Le cycle proposé dure 1,1 seconde. Le choix initial est une pulsation progressive ;
une option de revue permet de comparer un clignotement franc au même rythme.
Une case permet de revoir le rendu actuel et un bouton de suspendre l'animation.
Le remplissage sous les numéros monte moins haut pour conserver leur contraste.
L'option de réduction des animations doit donner un remplissage fixe.

Les commandes de déplacement et la dalle E validée sont déjà intégrées sous
WALK-VIS-03-E ; cette nouvelle proposition ne remet pas cette intégration en attente.

Les 12 images natives de 1280 × 400 sont encodées sans perte. Chaque image
décodée est identique aux pixels PNG issus du moteur. Entre la phase minimale
et maximale, les 2 042 pixels modifiés restent tous dans les voisinages des
petits repères ; le décor et les silhouettes sont conservés.
Le remplissage est dessiné sous les chiffres par le moteur. Le fichier
`marker-pulse-01-native.json` conserve les contrôles et le statut de revue.
L'ouverture automatisée du fichier local a été bloquée par la politique du
navigateur ; aucune route alternative n'a été utilisée pour contourner ce
blocage. Les aperçus animés natifs sont fournis directement comme fichiers.

## Décision suivante

Le joueur retient la pulsation progressive et signale un défaut de centrage.
Le choix du rythme est acquis. La correction ciblée est soumise dans
MARKER-PULSE-02 ; il ne faut pas reposer le choix progressif/franc.
