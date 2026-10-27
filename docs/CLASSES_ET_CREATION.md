# Classes et création de personnage

## Catalogue actuel — 26 octobre 2026

Les nouvelles parties proposent quatre classes, dans cet ordre : **Bastion**,
**Traqueuse**, **Arcaniste** et **Métamorphe**. Le joueur demande explicitement
de supprimer la classe jouable Technomancien. Les profils, dessins et règles
des quatre classes restantes conservent leurs identifiants et leur fonctionnement.

Le Technomancien n'est plus proposé à la création, à la souris comme au clavier.
Le grand portrait à droite est retiré, à la demande du joueur ; les vignettes
64px dans les choix, la description, l'équipement et les attributs sont conservés.
L'équipement et les deux compétences de départ sont présentés en cartes avec
leurs icônes, noms, quantités et rangs. Le survol ouvre les statistiques ou les
descriptions et effets actuels ; les fiches longues défilent à la molette et
le prochain rang n'affiche que les valeurs qui changent. Les valeurs proviennent
du véritable départ de la classe sélectionnée, sans modifier la partie ouverte.
Finition et vérifications : `reviews/creation-starting-cards-01/`.
La mise en page occupe presque toute la fenêtre et équilibre les deux colonnes
verticalement ; les boutons restent en bas. Le thème, les marges, les icônes
encadrées et la hiérarchie des textes sont communs aux cartes et aux fiches.
Sa définition garde `available_at_creation: false` afin de résoudre les anciens
identifiants de sauvegarde avec leurs règles exactes. Les anciennes parties
restent reprenables sans conversion de classe, de statistiques ou d'équipement.
Les compétences de drones et d'affaiblissement restent dans le catalogue commun,
accessibles aux autres agents ; retirer ce départ ne les supprime pas.

Le champ optionnel `available_at_creation` des fichiers `classes/*.json5` vaut
`true` par défaut, y compris pour les mods existants. Le catalogue de création
filtre les profils retirés ; la recherche par identifiant garde la compatibilité.
Décision et contrôles : `reviews/technomancien-retirement-01/`.

Les sections ci-dessous documentent les premiers jalons historiques et leurs
formats de suspension, et ne décrivent pas le catalogue jouable actuel.

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
