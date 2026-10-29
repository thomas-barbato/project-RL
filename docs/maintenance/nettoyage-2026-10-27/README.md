# Nettoyage du 27 octobre 2026

À la demande du joueur pendant l'examen graphique, les sorties remplacées sont
retirées après contrôle du Release et de l'inventaire des ressources.

381 fichiers, soit 79 776 515 octets, passent dans la Corbeille sans vidage :
anciennes itérations de création, captures Terminal antérieures à la nouvelle
police, première capture du retrait du Technomancien, fragments temporaires de
sa proposition retirée, atlas régénérables ARM14, captures répétées et caches
Python. Les sept références finales de création, les comparaisons ARM14 et ses
quatre captures finales contrôlées restent disponibles.

La suppression définitive des exports a été refusée par le contrôle automatique
avec le motif « blocked by policy ». Le passage en Corbeille constitue le repli
réversible ; ces octets restent sur le disque jusqu'à son vidage.

`cargo clean --profile dev` retire séparément 1 281 fichiers, représentant
1 147 425 306 octets avant l'opération. Le Release est conservé. Une prochaine
compilation Debug reconstruira ce cache ; l'éditeur peut aussi en régénérer une
partie. La compilation Release réussit avant et après le nettoyage.

La vérification des 23 540 fichiers protégés réussit sans changement, ainsi que
l'inventaire et l'intégrité du paquet existant. Le cache Debug est vide au moment
de cette vérification, puis l'éditeur commence à le reconstruire. La dernière
mesure et le gain net correspondant sont dans `cleanup-result.json` ; ce cache
actif n'est pas supprimé en boucle.

Les chemins absolus, tailles, empreintes et ancêtres sans jonction sont vérifiés
avant le déplacement de chaque fichier. Les sources, décisions, propositions
encore en attente, ressources du jeu, données de l'éditeur de cartes, sauvegardes
et préférences sont conservées. Aucun commit ni paquet portable n'est créé.

`plan.json` détaille les cibles et leur justification ; `cleanup-result.json`
enregistre le déplacement et le nettoyage Cargo. `preserved-before.json` et
`preservation-after.json` portent sur les 23 540 fichiers protégés. L'inventaire
actuel contient 1 815 ressources. L'intégrité du paquet portable existant est
vérifiée séparément ; il conserve sa version antérieure.

L'examen visuel utilise des captures fraîches sous
`artifacts/visual-audit-2026-10-27/`. Le diagnostic du camp réussit. Celui de la
rue produit deux vues, puis échoue avec `Joueur absent` ; cela ne valide pas le
parcours complet. Les graphismes et les règles du jeu ne sont pas modifiés.
