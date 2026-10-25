# WEAPON-READABILITY03 — intégration

Accord : « ok ca me semble bien visible :) on peut donc tout integrer :) ».

Les six modèles présentés (2, 17, 23, 31, 51, 66) utilisent leurs pixels natifs, prises, tailles par classe et ombre de séparation validés.
Les armes, doigts et ombre restent des couches ; les têtes, visages ouverts et bottes gardent leurs sources.
Le même chemin sert au plateau et aux portraits. Le cache final évite de reconstruire les couches à chaque image.
Les banques embarquées viennent de assets/sprites/weapon-readability-03/ ; aucune dépendance du jeu à artifacts/.
Les palettes d’équipement existantes restent prises en charge avec le même classement de matériaux ; les couleurs originales restent exactes.
Correction demandée ensuite : les armes de mêlée à deux mains réutilisent la découpe complète des anciennes mains/manches, déjà employée pour les armes longues. Cela retire les fragments du Bastion et de l’Arcaniste sans repeindre leurs armures ni leurs armes.
Les dessins des autres modèles du catalogue, leurs identités, statistiques et sauvegardes restent inchangés.

Contrôles chiffrés : integration-03-cpu-checks.json et integration-03-gpu-checks.json.
Aperçu courant : artifacts/weapon-readability-review-01/index.html#weapon-integration-03.

Le joueur confirme ensuite l’intégration et demande le nettoyage des fichiers
inutiles. Le Release lancé par `Jouer.cmd` est reconstruit. Les sources, accords,
comparaisons natives utiles et empreintes des 3 100 compositions de référence
sont conservés ; les exports bruts et anciens aperçus sont retirés.
Registre : `docs/maintenance/nettoyage-2026-10-22/armes/`.
