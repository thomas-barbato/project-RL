# Contenu visuel

Pour les cartes, décors et personnages, appliquer la règle de cohérence visuelle
`VIS-COH01-RULE01` dans `docs/STYLE_VISUEL_REFERENCE.md`. Lire ce guide avec
`docs/DIRECTION_ARTISTIQUE_TILES.md` avant tout nouveau dessin ou reprise.
Comparer décors et acteurs ensemble à taille native avec les références validées.

Avant de créer ou modifier des personnages, armes, équipements portés ou monstres,
lire `docs/DIRECTION_ARTISTIQUE_TILES.md` et appliquer sa procédure de revue.
Dungeon Crawl Stone Soup reste la référence de lisibilité et de qualité.
Depuis l'accord du 17 octobre 2026, le jeu et les nouveaux dessins sont en
64 × 64 uniquement ; la version d'affichage 32 px est abandonnée.
Conserver les formes lisibles et la composition par couches avec points d'attache.
Présenter les nouveaux candidats numérotés à taille native avant leur intégration,
sauf si le joueur a déjà autorisé explicitement l'intégration de ces candidats.
Conserver les identités de contenu et les règles de jeu lors d'un changement visuel.

# Textes et effets de la refonte

Pour la refonte de l'agent cryogénisé, présenter tous les textes destinés au
joueur et tous les effets mécaniques ou visuels nouveaux ou modifiés avant
leur intégration. Ils doivent être validés explicitement par le joueur.
Conserver des identifiants de revue stables et distinguer l'accord sur le
texte, les règles chiffrées et le rendu visuel. La validation d'un nom,
d'une silhouette ou d'une orientation ne valide pas les autres éléments.
Les décisions déjà acceptées restent valables ; une révision doit être
soumise de nouveau pour la partie modifiée. Les documents de proposition
et les prototypes de revue peuvent être préparés avant cet accord.

# Rédaction des compétences

Présenter les descriptifs destinés au joueur sans tableau : texte descriptif,
effets actuels et effets au prochain rang achetable, en blocs courts et chiffrés.
Ne mentionner le temps d'exécution que si l'action ne s'applique pas dès
son activation, par exemple lorsqu'elle exige une préparation ou un délai.
Ne pas rappeler le déclenchement immédiat ni la consommation d'un tour pour
une action ordinaire. Les durées d'effets et les recharges restent indiquées.
Cette règle concerne la rédaction et ne modifie pas le fonctionnement des actions.

Les compétences actives d'arme de mêlée exigent une arme de mêlée équipée
et active ; celles d'arme à distance exigent une arme à distance équipée
et active. Si l'arme active ne convient pas, la fiche doit indiquer le type
d'arme requis. Une autre arme simplement équipée ne satisfait pas cette condition.

Exception validée pour la configuration à deux armes à une main : ses deux
armes sont actives ensemble. Une paire mêlée + distance permet les compétences
des deux types, avec sélection automatique de l'arme requise. L'attaque ordinaire
utilise la mêlée au contact et le tir plus loin. L'arme à deux mains en réserve
ne fait pas partie de cette paire ; l'activer met les deux armes à une main
en réserve. Chaque attaque ordinaire utilise une seule arme.

Remplacer les explications de progression et les formules par une comparaison
des effets actuels avec ceux du prochain rang, comme demandé par le joueur.
Le bloc « Prochain rang » affiche uniquement les valeurs qui changent ;
les effets inchangés restent dans le bloc actuel ou les informations communes.
L'aperçu conserve l'équipement et les autres modificateurs actuels. À dix rangs
appris, indiquer le plafond atteint plutôt qu'un rang supplémentaire achetable ;
les bonus d'équipement continuent à augmenter le rang effectif au-delà de dix.
Les armes doivent avoir une variance de dégâts ; les valeurs actuelles des
compétences d'arme doivent afficher une plage minimum–maximum calculée avec
l'arme active et les modificateurs applicables. Ne pas inventer une amplitude
ou une distribution générale de variance sans revue.

# Chargement et fluidité

Appliquer à tout le jeu la règle `LOAD01-RULE01`, validée le 29 octobre 2026.
Faire évoluer un mécanisme commun de préparation des ressources et de cache,
réutilisable par les cartes actuelles et futures, les animations, les effets
et les interfaces.

Préparer les ressources nécessaires à la destination avant son premier affichage
et anticiper les besoins proches. Conserver les ressources et décors récemment
utilisés dans un cache borné en mémoire, avec libération des éléments éloignés
ou inutilisés. Limiter le préchargement aux besoins utiles et proches.

Sortir les lectures et décompressions lourdes du chemin de dessin. Respecter
le thread du contexte graphique pour la création des textures et répartir
les préparations restantes afin de limiter les pointes de charge.
Conserver l'état des zones, les règles de jeu et les sauvegardes indépendamment
du cache de présentation.

Vérifier les premières visites, les allers-retours et les premières utilisations
des effets, en distinguant lecture, décodage, création des textures et rendu.
Les décisions et l'état d'intégration sont dans
`docs/reviews/loading-study-01/REGLE.json` et `ANALYSE.json`.

# Fichiers générés et rangement

Les ressources utilisées à la compilation ou par le jeu vont dans `assets/`,
les décisions et validations durables dans `docs/reviews/`. Ne pas ajouter de
dépendance du jeu à une capture ou à un dossier temporaire sous `artifacts/`.
Réutiliser l’aperçu courant et supprimer ses sorties inutiles après validation,
en conservant les dessins sources et les règles acceptées. Les cartes,
préférences et récupérations de `artifacts/map-editor/` sont des données de
travail à préserver. Voir `docs/GESTION_FICHIERS_GENERES.md`.
