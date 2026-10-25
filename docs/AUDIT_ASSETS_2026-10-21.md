# Audit du stockage et du système d’assets — 21 octobre 2026

**Le volume est excessif pour le contenu jouable actuel. La cause principale est l’accumulation des fichiers de développement et des sources graphiques, davantage que le poids des dessins effectivement utilisés en jeu.** Des améliorations du chargement des images sont également justifiées. Il est possible de travailler sur ces deux sujets en conservant la qualité graphique actuelle.

Audit du dépôt `C:\Users\User\Desktop\project-RL`, au commit `fc5f5632` et avec les fichiers locaux présents au moment des mesures. Ce rapport propose des interventions ; il ne les met pas en œuvre. Aucune ressource du jeu n’a été modifiée ou supprimée.

**1. Ce qui occupe réellement le disque**

Les tailles ci-dessous sont des tailles logiques de fichiers, hors effets de l’allocation et de la compression NTFS. Un Mio vaut 1 048 576 octets ; un Gio vaut 1 073 741 824 octets. Les chiffres sont arrondis.

| Élément | Taille | Rôle |
|---|---:|---|
| `.git/` | **7,65 Gio** | Historique et stockage interne de Git |
| `artifacts/` | **4,67 Gio** | Captures, aperçus et résultats de tests |
| `target/` | **1,95 Gio** | Compilation, dépendances et exécutables |
| `assets/` | **1,79 Gio** | Ressources, mais aussi sources et prototypes |
| `docs/` | **458 Mio** | Documentation et dossiers de revue |
| Code et autres fichiers | Environ 30 Mio | Sources, outils, contenu et configuration |
| **Total du projet** | **16,55 Gio** | Dossier de développement complet |

**Git, les sorties de tests et la compilation représentent ensemble 86,3 % du projet.** Optimiser seulement les images du jeu laisserait donc l’essentiel de ce volume en place.

Le nettoyage précédent a aussi placé environ **755 Mio** dans l’archive externe `C:\Users\User\.codex\archives\project-RL\cleanup-2026-10-20`. Cette archive n’est pas comprise dans les 16,55 Gio. Le déplacement a réduit la taille du projet, sans libérer ces octets sur le disque.

**2. Le dossier `assets` mélange ressources jouables et fabrication**

| Contenu d’`assets/` | Taille |
|---|---:|
| `source-art/` | **1 704,3 Mio**, soit **92,8 % des assets** |
| `maps/` | 37,2 Mio |
| `prototypes/` | 34,6 Mio |
| `animations/` | 34,3 Mio |
| `sprites/` | 24,0 Mio |
| `skills/` | 1,9 Mio |
| Audio, polices et autres | Environ 0,9 Mio |

Les 8 262 fichiers de `source-art/` comprennent notamment des dessins sources, étapes de fabrication et variantes de revue. Tous sont actuellement suivis par Git. Les deux groupes les plus volumineux sont les sources de la refonte de l’agent, environ **738 Mio**, et celles de la refonte de l’histoire, environ **246 Mio**.

La recherche des références littérales Rust et des dépendances des scènes normales ne retrouve pas **8 251 de ces fichiers, représentant 1,66 Gio**. Ce sont des candidats à exclure d’un paquet joueur. Ce résultat ne signifie pas qu’ils peuvent être effacés : les outils de fabrication, les revues et les dessins maîtres ont d’autres besoins que le jeu.

Il reste de vraies dépendances du jeu à ce dossier. Le manifeste du camp désigne notamment quatre dessins de PNJ acceptés dans `source-art/`, et le [rendu du camp](C:/Users/User/Desktop/project-RL/src/cryogenic_camp_render.rs:27) les charge. La première étape est de placer leurs copies de production dans les ressources du jeu et de corriger les références, en conservant les originaux et leurs validations.

La règle de rangement existante dans [GESTION_FICHIERS_GENERES.md](C:/Users/User/Desktop/project-RL/docs/GESTION_FICHIERS_GENERES.md) est pertinente. Il manque surtout son application au paquet distribué : une liste explicite de ce qu’il faut livrer, indépendante de tout ce que contient le dépôt.

**3. La taille du jeu distribué peut être beaucoup plus faible**

| Fichier ou ensemble mesuré | Taille |
|---|---:|
| Exécutable Debug existant | **115,0 Mio** |
| Exécutable Release existant | **54,1 Mio** |
| PNG présents dans le Release | **32,8 Mio**, déjà compris dans ses 54,1 Mio |
| Dépendances externes identifiées des scènes normales | **35,0 Mio**, pour 123 fichiers |
| Contenu et configuration des mods présents | Environ 0,4 Mio |

De nombreuses images sont incorporées à l’exécutable avec `include_bytes!`. Il n’est donc pas nécessaire de joindre systématiquement leur copie externe. Le comportement de cette macro est documenté par [Rust](https://doc.rust-lang.org/std/macro.include_bytes.html).

**Le total indicatif est d’environ 89,5 Mio pour le Release existant et l’ensemble externe relevé.** Cet ensemble reste conservateur : il comprend quelques fichiers également incorporés et des manifestes. Inversement, une dépendance construite dynamiquement peut échapper à une analyse statique. Ce n’est pas encore une distribution complète validée.

L’exécutable Release date du **20 octobre à 19 h 22, heure de Paris**, avant le commit actuel. Il faut le recompiler avant de fixer une taille de référence. Le lanceur [Jouer.cmd](C:/Users/User/Desktop/project-RL/Jouer.cmd:3) construit et utilise actuellement le profil Debug ; la comparaison des deux exécutables existants ne mesure donc pas uniquement l’effet du profil, puisque leurs versions diffèrent aussi.

**Une première cible de 100 à 150 Mio décompressés paraît raisonnable pour le contenu actuel**, à confirmer par une compilation récente et un test du paquet autonome. Ce chiffre est un objectif d’ingénierie, pas une taille garantie.

Un obstacle concret à corriger est la résolution des chemins. Plusieurs chargeurs lisent les fichiers depuis `env!("CARGO_MANIFEST_DIR")`, donc le chemin du projet fixé lors de la compilation : [camp](C:/Users/User/Desktop/project-RL/src/cryogenic_camp_render.rs:20), [rue](C:/Users/User/Desktop/project-RL/src/prison_street_render.rs:18), [éclairage](C:/Users/User/Desktop/project-RL/src/camp_atmosphere.rs:43). Le [chargement du contenu](C:/Users/User/Desktop/project-RL/src/ascii_app.rs:25072) utilise aussi cette approche. Il faut un répertoire de données résolu pour l’installation, puis vérifier le lancement sans accès au dépôt d’origine.

**4. Les doublons existent, mais la recompression seule rapporte peu**

L’empreinte SHA-256 de chaque asset révèle **655 groupes de fichiers strictement identiques**, pour **61,2 Mio de copies supplémentaires**. Par exemple :

- Le fond de la rue agrandie existe dans `maps/` et dans les sources, pour **8,8 Mio par copie**.
- Le fond d’entrée des égouts existe lui aussi dans ces deux ensembles, pour **8,6 Mio par copie**.
- Une autre image de rue apparaît trois fois, à **2,1 Mio par copie**.

Une référence commune ou un catalogue d’assets permettrait de partager ces données tout en conservant les identifiants de contenu. Le gain de 61,2 Mio recoupe en partie l’exclusion des sources : **il ne faut pas additionner ces deux gains intégralement**.

Un essai sans perte a porté sur les **32 plus gros PNG** parmi les images du Release et les dépendances externes relevées :

| Échantillon | Résultat |
|---|---:|
| Taille initiale | 50,54 Mio |
| Gain de recompression | **0,83 Mio** |
| Réduction | **1,64 %** |

L’essai recompresse uniquement les données PNG avec zlib au niveau 9, conserve les filtres et autres blocs, et vérifie l’identité des données décompressées. Il est effectué en mémoire ; aucun PNG n’est réécrit. Ce n’est pas un benchmark exhaustif de tous les optimiseurs et filtres PNG. Il montre néanmoins que cette opération simple ne résout pas le problème de volume.

Une réduction de résolution ou une compression avec perte n’est pas justifiée comme première intervention. Les dessins natifs 64 × 64, contours, transparences et points d’attache acceptés peuvent être conservés.

**5. L’historique Git contient une part importante du surplus**

Le stockage Git représente **7,65 Gio**. L’inventaire retrouve d’anciens fichiers de test très volumineux dans son historique : par exemple `artifacts/lisibilite-sprites-2026-10-05/preview.pdb`, **137,2 Mio avant compression Git**, ainsi que plusieurs autres fichiers de symboles de débogage de plus de 130 Mio. Ces tailles individuelles ne sont pas leurs tailles occupées après compression Git.

Aujourd’hui, seuls **quatre fichiers d’`artifacts/`, totalisant moins d’un Mio**, sont encore suivis dans l’état courant. L’essentiel des captures actuelles est déjà ignoré. Ajouter davantage de règles ne supprimera pas les anciennes données : [`.gitignore` ne retire pas les fichiers déjà suivis](https://git-scm.com/docs/gitignore).

La recherche en lecture seule relève **17 313 objets**, représentant environ **4,42 Gio de stockage d’objets**, non atteignables depuis les références, les reflogs et l’index examinés. Elle trouve aussi **47 fichiers temporaires `tmp_obj_*`**, environ **29,7 Mio**, signalés par Git comme fichiers parasites.

Ces chiffres indiquent un potentiel de maintenance important. Ils ne constituent ni une liste de suppression ni une garantie de récupération exacte : les délais de conservation, références d’autres espaces de travail, processus en cours et relations de compression restent à contrôler. Une intervention doit passer par les outils de maintenance de Git, après sauvegarde et contrôle d’intégrité. La [documentation de `git gc`](https://git-scm.com/docs/git-gc) décrit ces protections. Réécrire l’historique partagé serait une opération distincte, avec ses propres conséquences.

**6. Le chargement des images mérite aussi une optimisation**

Ce sujet concerne surtout la mémoire et la fluidité. Un PNG peu volumineux sur disque peut devenir une grande image décompressée : par exemple, `flat-agents/reviewed-128.png` pèse **1,17 Mio**, mais ses **2 048 × 4 608 pixels** occupent **36 Mio en RGBA8**.

Le [démarrage](C:/Users/User/Desktop/project-RL/src/main.rs:77) prépare plusieurs banques avant le menu. Pour **39 images conservées par les chargeurs examinés**, le calcul des buffers RGBA8 atteint déjà **184,7 Mio**, hors autres images, copies temporaires, allocations et textures GPU. C’est une estimation statique des buffers, pas une mesure de RAM du processus.

La [banque de monstres](C:/Users/User/Desktop/project-RL/src/game_art_campaign.rs:16) décode notamment plusieurs atlas et leurs masques dès son initialisation. Certaines anciennes banques de petites images restent également chargées. Il faut examiner leurs usages avant de les retirer : un suffixe `128`, `192` ou `256` peut correspondre à une composition ou à une grande créature, et pas seulement à un ancien mode d’affichage.

Le code dispose déjà de bonnes bases : [LazyTexture](C:/Users/User/Desktop/project-RL/src/effect_geometry.rs:10) diffère le décodage jusqu’au premier usage, et certains caches de compositions [sont limités à 192 entrées](C:/Users/User/Desktop/project-RL/src/game_art_worn.rs:248). Ces protections ne sont toutefois pas partagées par tous les chargeurs. Le même atlas de flammes est déclaré dans plusieurs modules ; plusieurs instances de cache peuvent donc produire plusieurs textures GPU pour les mêmes données. Le cache d’[images de finition](C:/Users/User/Desktop/project-RL/src/game_art_finish.rs:15) n’a pas de budget mémoire explicite. Cela ne démontre pas une fuite mémoire.

Les grandes cartes ajoutent une autre dépense. À partir des dimensions des images et des [cibles de rendu](C:/Users/User/Desktop/project-RL/src/camp_atmosphere.rs:78), le sous-ensemble « fonds, éclairage et cible couvrant toute la carte » représente environ **43,4 Mio pour le camp** et **80,9 Mio pour la rue agrandie**, en RGBA8. Ces estimations partielles ne sont pas des mesures de VRAM et ne décrivent pas tout ce qui est simultanément résident.

Les pistes pertinentes sont un cache partagé par identité d’asset, des budgets en octets, un chargement progressif anticipé et, pour les cartes, des cibles adaptées à la vue ou un découpage par zones. Le chargement doit être anticipé avant l’action pour conserver la continuité des animations. Les gains réels nécessiteront un profilage en jeu ; ces changements ne font pas disparaître automatiquement des fichiers du disque.

**7. Ordre recommandé des interventions**

| Priorité | Intervention | Gain attendu et validation |
|---|---|---|
| **1 — Distribution** | Construire un paquet Release avec une liste explicite de ressources, corriger les chemins absolus et les dépendances à `source-art/`. | Exclure environ **1,66 Gio de sources** du paquet, ainsi que les outils, prototypes inutiles et fichiers de développement. Viser **100–150 Mio**, puis vérifier le lancement autonome et les scènes. Aucun espace n’est libéré sur le SSD en conservant simplement ces sources hors paquet. |
| **1 — Stockage local** | Revoir les sorties de test après validation, garder une seule version utile des aperçus et nettoyer les compilations devenues inutiles. | Réservoir à examiner : **4,67 Gio d’artefacts** et **1,95 Gio de compilation**. Ce ne sont pas des gains intégralement garantis. Préserver `artifacts/map-editor/`, les revues en attente, les dessins sources et les décisions durables. |
| **1 — Git** | Sauvegarder et contrôler le dépôt, puis définir une maintenance adaptée. | Potentiel de plusieurs Gio, avec **4,42 Gio d’objets non atteignables selon l’inventaire**. Mesurer le gain après maintenance ; traiter séparément une éventuelle réécriture d’historique. |
| **2 — Assets partagés** | Unifier les copies identiques avec des références stables. | Jusqu’à **61,2 Mio** dans l’arbre actuel, avec recoupement des gains liés aux sources. Vérifier manifestes, compilation et rendus. |
| **2 — Mémoire** | Partager les textures et charger progressivement les banques nécessaires. | Réduire le décodage anticipé et les copies ; établir une mesure de RAM, VRAM et temps de première utilisation avant/après. |
| **3 — Compression** | Optimiser les PNG sans perte une fois le paquet établi. | Gain simple mesuré : **1,64 % sur l’échantillon**. Intervention secondaire, sans baisse de qualité. |

Les plus gros dossiers d’artefacts à examiner sont `skill-learn-buttons-review-01` (**1,55 Gio**), `performance-globale-2026-10-18` (**665 Mio**) et `walk-preview-review-01` (**464 Mio**). Les **458 Mio de documentation** comprennent aussi des preuves visuelles : les décisions et références acceptées doivent rester identifiables lors du tri. Un archivage sur le même disque réduit le dossier de travail, mais ne libère pas de capacité.

Le critère de réussite du premier chantier serait un dossier joueur autonome, généré de manière reproductible, avec une taille mesurée et aucune dépendance au dépôt d’origine. Le contrôle doit couvrir démarrage, camp, rues, égouts, personnages, équipement, compétences, audio et sauvegarde/rechargement. Les pixels, règles et identifiants acceptés doivent être conservés.

**Méthode et limites du rapport**

Mesures effectuées : inventaire des fichiers, empreintes des **10 383 assets**, dimensions et blocs PNG, recherche de références Rust, parcours des manifestes de scènes, extraction des PNG réellement présents dans les exécutables existants, essai de recompression en mémoire et inventaire Git en lecture seule.

Les mesures détaillées ont été produites dans un dossier temporaire ignoré par Git. Après la mise en œuvre, ses sorties jetables sont retirées ; ce rapport conserve les résultats initiaux utiles. Voir le [bilan de validation](C:/Users/User/Desktop/project-RL/docs/reviews/asset-optimization-01/VALIDATION.md) pour les résultats après intervention et le [guide de distribution](C:/Users/User/Desktop/project-RL/docs/DISTRIBUTION.md) pour reconstruire le paquet.

Aucune nouvelle compilation, exécution du jeu, mesure mémoire en direct, suppression, modification de l’index ou maintenance Git n’a été effectuée pour cet audit. Les dépendances statiques constituent une base d’analyse, pas une preuve exhaustive qu’un fichier est inutile. Les tailles ne préjugent pas de la durée des travaux et les gains qui se recoupent ne doivent pas être additionnés.
