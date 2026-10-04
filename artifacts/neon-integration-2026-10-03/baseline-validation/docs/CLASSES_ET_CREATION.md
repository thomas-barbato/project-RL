# Classes et création de personnage

## Statut du premier jalon

Une nouvelle partie passe désormais par la création du personnage avant le premier tour. Le joueur choisit un **protocole de restauration**, puis confirme ou modifie sa répartition de statistiques. Cette appellation relie les classes au protagoniste — une IA restaurée dans un châssis — sans transformer la classe en faction, en peuple ou en métier définitif.

Les trois premiers noms sont un catalogue jouable initial et pourront encore être affinés avec le vocabulaire définitif du monde :

| Protocole | Fonction lisible | Profil recommandé | Matériel initial |
|---|---|---|---|
| **BRÈCHE** | Assaut et rupture | PUI 8, COO 5, RÉS 7, PER 4, TRA 4 | Lame d'intégrité, plastron rapiécé, 2 patchs de réparation |
| **VIGIE** | Distance et reconnaissance | PUI 4, COO 8, RÉS 4, PER 8, TRA 4 | Lance-aiguilles, 2 patchs de réparation |
| **CREUSET** | Contrôle de zone et survie | PUI 4, COO 5, RÉS 7, PER 5, TRA 7 | Lance-flammes industriel, 3 patchs de réparation |

Le sous-titre fonctionnel reste toujours affiché avec le nom d'univers. Le joueur n'a donc pas besoin de deviner ce que « CREUSET » signifie mécaniquement.

## Règles de création

- Le protocole est choisi depuis **Nouvelle partie**, avant que le joueur puisse agir.
- Son profil de statistiques n'est qu'une recommandation. Les 28 points peuvent être redistribués entre Puissance, Coordination, Résilience, Perception et Traitement.
- Chaque statistique reste comprise entre 3 et 8 à la création. La partie ne commence pas tant que le total n'est pas exactement 28.
- La souris sélectionne les protocoles et les boutons `−` / `+`. Le clavier utilise les commandes de navigation réattribuables, Entrée pour continuer et Échap pour revenir à l'étape précédente.
- Choisir une classe ne verrouille aucune progression future et n'interdit aucun équipement trouvé pendant la partie.
- Les trois protocoles laissent actuellement les 2 points de compétence initiaux libres. Aucun apprentissage de classe n'est simulé tant que les disciplines correspondantes ne sont pas toutes exécutables ; une classe ne doit jamais accorder un pouvoir fictif ni créer des points supplémentaires.

## Données et mods

Les définitions résident dans `content/core/classes/*.json5`. Chaque paquet de mod peut ajouter ses propres fichiers `classes/*.json5` dans son namespace. Le chargeur commun vérifie avant le lancement :

- l'identifiant namespacé et l'absence de doublon ;
- les clés de nom, de fonction et de description ;
- le budget et les bornes du profil recommandé ;
- l'existence des armes et objets référencés ;
- les quantités positives et l'absence de doublons ;
- l'appartenance de chaque arme équipée au matériel réellement accordé.

Le moteur applique ensuite la définition à `GameRules` avant de construire le premier état. L'algorithme de combat ne contient aucune branche `BRÈCHE`, `VIGIE` ou `CREUSET` : seules les statistiques, les armes et les règles déjà déclarées produisent leurs effets. Un mod peut donc ajouter un protocole sans remplacer le moteur.

## Fiche de personnage et suspension

La commande **Personnage** (`J` par défaut) ouvre une fiche modale sans consommer de tour. Elle affiche le protocole, les cinq primaires, leur description complète, le niveau, l'expérience, les points de compétence, les PV, le Blindage, l'énergie et les valeurs de combat effectivement raccordées au moteur pour l'emplacement d'arme actif. Elle n'expose aucune statistique cachée d'un adversaire.

Quand un niveau est gagné en jeu, l'écran des compétences s'ouvre automatiquement après la résolution complète du tour. Son bandeau indique le niveau atteint et le total de points disponibles ; Échap le ferme sans dépense ni passage du temps. Une reprise de suspension rejoue les événements pour validation sans rouvrir cet écran.

Depuis la suspension version 34, l'identifiant du protocole et la répartition choisie sont enregistrés. À la reprise, le moteur recharge la définition, reconstruit les mêmes règles puis vérifie l'empreinte et le rejeu complet avant de consommer le fichier. Les suspensions 1 à 33 restent sans classe et affichent « Restauration antérieure » dans la fiche ; elles ne reçoivent jamais rétroactivement une classe. La version 35 ajoute le Blindage sans modifier ce contrat : une suspension 34 conserve au contraire son ancien modèle de dégâts. La version 36 ajoute le plastron au sac initial de BRÈCHE, sans l'équiper automatiquement ; une suspension 35 garde la définition antérieure de ce protocole et son ancien butin.

## Suite prévue

Le déblocage horizontal de nouveaux protocoles entre les parties reste à concevoir séparément. Des orientations comme intrusion, guerre électronique ou drones ne seront ajoutées au catalogue jouable qu'avec leurs systèmes réels, leur matériel minimal et leurs validations. Les protocoles restent distincts des factions et des peuples du monde.
