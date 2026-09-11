# Ville fixe et extérieur aléatoire — première carte jouable

Lancer avec `cargo run`. Le client démarre sur la place centrale. Le nom de la ville, les personnages et les services restent à définir ; les noms des bâtiments sont des repères de prototype, pas des décisions narratives définitives.

## Parcours de test

- Se déplacer avec la disposition choisie dans les options. **V** interagit avec une porte ou une console adjacente ; la touche réelle apparaît dans l'aide et peut être réattribuée.
- Au nord de la place : l'atelier possède une porte ordinaire. Ouvrir révèle son intérieur ; refermer coupe la vision. La mémoire déjà explorée reste atténuée.
- Plus à l'est : les archives sont verrouillées. Leur console, devant le bâtiment, déverrouille l'accès. Il faut ensuite ouvrir la porte. L'opération ne simule pas encore de piratage.
- Au sud-est, le checkpoint commence hors tension. Un récupérateur `c` va chercher le régulateur posé dans la ville, ouvre au besoin la porte ordinaire du dépôt et le livre. Le joueur peut le devancer : rejoindre le régulateur près de l'atelier, le ramasser avec **E**, entrer dans le dépôt de maintenance, se placer contre la caisse marquée comme dépôt puis utiliser **V**. La quantité demandée quitte réellement l'inventaire. Le régulateur est signalé comme `MATÉRIEL ATTRIBUÉ · PRISE SIGNALÉE SI OBSERVÉE` avant la prise, car la carte de test n'accorde aucun droit initial au joueur. Une autre expédition ou un mod peut déclarer une autorisation ; l'objet resterait attribué mais sa prise ne créerait alors ni souvenir ni alerte. Ici, un travailleur affilié qui voit directement l'action s'en souvient et entre temporairement en alerte. Si le joueur le voit, son glyphe conserve sa forme mais reçoit un badge `!` agrandi et une double bordure ; un cadre pulsé entoure la carte et un bandeau pleine largeur indique clairement le nombre de sources et la durée restante. L'état apparaît aussi dans l'inspection, le panneau de détection et le journal. Une alerte cachée ne fuit pas dans l'interface. Cette observation n'applique pas encore de sanction, de transmission ni de réputation. Le technicien `m` prélève ensuite la pièce, répare le relais puis rétablit réellement la porte de service et son capteur. Une prise non autorisée située dans la portée et la ligne de vue de ce capteur opérationnel déclenche désormais une `ALARME DE SÉCURITÉ` distincte ; le parcours automatisé décrit plus bas en fournit la preuve sans ajouter artificiellement un second objet à la partie normale. Attendre consomme des tours et permet d'observer les étapes ; ouvrir un menu ne fait pas avancer le travail.
  La première réponse déclarée du capteur demande désormais à l'actionneur de verrouiller temporairement la porte du checkpoint. Celle-ci reçoit un badge `!`, une double bordure et une durée lisible à l'inspection. Avant de fermer, le moteur teste la carte sur une copie et annule la réponse si le joueur n'aurait plus aucune sortie de zone atteignable.
- À l'est de l'enceinte : ouvrir la porte de la ville pour rejoindre les friches. Le bandeau distingue « Ville protégée » et « Zone hostile ». Les ennemis restent dehors et la ville n'est pas une position de tir invulnérable.
- Quelques cases après la porte est, un passage à marches jaunes mène au **Secteur industriel, profondeur 1**. Se placer dessus ou juste à côté, puis **V** (ou l'affectation choisie). La destination apparaît dans l'aide. Le passage d'arrivée permet le retour par la même action ; marcher dessus ne déclenche pas le voyage.
- Dans ce nouveau secteur généré : ramasser le butin près de l'entrée, explorer, revenir en ville puis repartir. Les objets ramassés ne réapparaissent pas ; les portes, ennemis et cases explorées de chaque zone sont conservés.
- **E** ramasse les objets au sol. Inventaire, équipement, techniques, rapports et combat restent ceux du moteur existant.
- **R** commence actuellement une nouvelle partie avec la seed suivante : la ville reste identique, le décor extérieur change. Ce raccourci de prototype réinitialise la partie active.
- **Échap → Sauvegarder et quitter** suspend toute l'expédition pour reprise unique. Son fichier reste `city-test-run.json` ; l'ancienne suspension ASCII n'est pas modifiée. Les versions 1 sont vérifiées puis passent en version 2 multi-zone ; les versions 2 gardent leur contenu fixe, les versions 3 leur butin pondéré, les versions 4 leurs définitions de monde sans installation, les versions 5 leur circuit antérieur aux données sociales, les versions 6 leurs témoins sans alerte rétroactive, les versions 7 leurs capteurs sans alarme installée et les versions 8 leurs alarmes sans réponse automatique. Les nouvelles parties utilisent la version 9, qui rejoue et vérifie aussi les verrouillages temporaires.

Les deux piles du secteur industriel sont maintenant tirées dans une table modifiable pour les nouvelles parties : réparation, lance-aiguilles ou lame, sans statistiques aléatoires à ce stade. Les anciennes expéditions conservent leurs piles d'origine. La profondeur, le type de carte et la source filtrent les entrées et leurs poids ; voir [BUTIN.md](BUTIN.md) pour les règles, les valeurs d'essai et l'exemple de mod.

## Affichage

Le rendu utilise des glyphes géométriques pixelisés sur des cases carrées, sans textures externes. La caméra suit le joueur. Survoler une case connue donne son rôle ou son état mémorisé ; sans souris, la cible sélectionnée alimente la même inspection. Une minimap des zones explorées et un résumé des détections visibles apparaissent lorsque la fenêtre est assez large. La longue légende permanente a été retirée : `F1` ouvre à la demande la légende du rendu actif. Comme seul le rendu Terminal à glyphes existe actuellement, elle n'affiche aucun onglet ni choix Graphique.

**Échap → Options → Affichage → Taille des cases** règle le zoom du monde indépendamment de la taille des menus. Les petites fenêtres réduisent les cases pour conserver l'empreinte des capteurs standard. Le plein écran fenêtré reste le réglage initial. Le style « Textures » n'est pas encore disponible.

## Limites et validation

Les zones déjà visitées continuent d'évoluer **pendant les tours du joueur**, pas en temps réel : les ennemis mobiles patrouillent, les effets, leurs durées et les travaux de maintenance avancent. Les menus et actions gratuites ne les font pas évoluer. Rien de ces déplacements ou travaux distants n'est ajouté à la carte connue ni au journal ; il faut revenir observer. Les zones non visitées ne sont pas encore actives. Les ennemis et travailleurs ne traversent pas les passages. Une arrivée occupée refuse le voyage sans consommer de tour : attendre ou explorer permet de laisser évoluer la situation.

Ce n'est pas encore un hub complet : le récupérateur et le technicien sont des agents fonctionnels de test, pas encore des personnages avec dialogue, affiliation ou routine sociale. Il n'y a ni boutique, garde, quête ni piratage simulé. Les objets massifs du décor bloquent réellement passage et vision, mais ne sont pas encore destructibles par le joueur.

Le validateur indépendant contrôle la navigation après décoration et les dépendances consoles/portes. Les tests comparent plusieurs seeds : ville identique, extérieur différent, même seed reproductible. Les interactions acceptées, dont la livraison au dépôt, participent au rejeu de suspension ; les refus ne consomment rien.

Les tests d'expédition couvrent aussi 64 seeds de génération, les aller-retours, l'absence de réapparition du butin, les identifiants stables, les morts, effets et réparations hors écran, l'absence de fuite visuelle, les transferts refusés sans mutation et le rejeu multi-zone. Les paramètres de la fixture et son `hub_facility` sont dans `content/core/worlds/expedition.json5` ; le moteur expose `ZoneBlueprint`, `WorldState` et `FacilityBlueprint` pour d'autres fournisseurs. Le chargeur commun découvre les définitions `worlds/*.json5` du cœur et des mods, borne leur coût, résout leurs tables de butin et valide les références de matériaux. Le décor général et les acteurs hostiles de la fixture restent encore codés dans son adaptateur.

`cargo run -- --ui-smoke <nouveau-dossier>` exécute un parcours isolé et produit les captures. Le plan complet de diagnostic est explicitement hors jeu : il n'ajoute aucune connaissance à la partie du joueur.

Pour contrôler le texte sans préchauffage de police ni capture préalable :
`cargo run -- --ui-cold-pause <nouveau-dossier>`. Les variantes `--ui-cold`,
`--ui-cold-options`, `--ui-cold-graphics`, `--ui-cold-controls`,
`--ui-cold-resume`, `--ui-cold-inventory` et `--ui-cold-skills` vérifient les
autres écrans dans des processus indépendants. Un libellé noir fait échouer le
diagnostic ; aucune partie ou préférence utilisateur n'est chargée ni modifiée.
Le correctif de bibliothèque associé est décrit dans
`vendor/miniquad/PROJECT_RL_PATCH.md`.

`--ui-cold-resume-error` vérifie au clic le refus d'une suspension volontairement
incohérente, sa conservation et la lisibilité de l'encadré d'erreur. Ce fichier
de test est créé uniquement dans le dossier de diagnostic.

`--ui-cold-expedition`, `--ui-cold-expedition-return` et
`--ui-cold-expedition-revisit` parcourent réellement le passage, ramassent un
objet et capturent respectivement le secteur industriel, le retour aux friches
et une nouvelle visite sans réapparition du butin. Ces parcours isolés ne
chargent aucune suspension utilisateur et ne téléportent pas le joueur.

`--ui-cold-maintenance <nouveau-dossier>` joue le trajet et 96 tours réels dans
une partie isolée, exige que la réparation soit achevée puis capture le
checkpoint rétabli. `--ui-cold-maintenance-delivery <nouveau-dossier>` fait
ramasser puis livrer le régulateur par le joueur, par les commandes normales,
et capture le dépôt après le transfert. `--ui-cold-maintenance-inventory`
contrôle séparément sa catégorie et l'aide de livraison dans l'inventaire.
`--ui-cold-maintenance-alert` capture la réaction locale directement sur la
carte et exige que son état textuel soit présent dans le journal.
`--ui-cold-maintenance-security-alarm` répare d'abord le relais par les tours
normaux, place un lot attribué uniquement dans cette fixture isolée puis vérifie
que le capteur alimenté le détecte, que sa porte se verrouille sans enfermer le joueur et que l'alarme comme sa réponse sont visibles sur toute l'UX.
`--ui-cold-legend` capture la légende du rendu Terminal,
notamment les deux rôles de maintenance, le matériau au sol et les états
d'installation.

Une recompilation ne bloque plus à elle seule la reprise : le format, les règles,
chaque commande et l'état complet reconstruit doivent toujours correspondre.
Pour vérifier explicitement un fichier réel **sans le consommer ni le modifier** :

```powershell
$env:PROJECT_RL_CHECK_SUSPENSION = 'C:\chemin\city-test-run.json'
cargo test check_suspension_file_without_consuming -- --ignored --nocapture
```
