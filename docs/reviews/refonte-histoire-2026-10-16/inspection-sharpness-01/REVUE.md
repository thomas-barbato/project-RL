# INSPECTION-SHARPNESS-01 — cadrage et netteté

Le joueur demande que le personnage, les drones et les monstres aient une
taille et une qualité cohérentes dans Inspection, puis précise : « tout dans
le jeu doit avoir la même netteté ». Cette exigence s'applique également aux
prochaines propositions de PNJ, monstres, équipements, décors et interface.

Le diagnostic retrouve deux erreurs de présentation : la présence d'une
analyse réduisait le cadre de 96 à 64 unités, et les marges du canevas de
composition réduisaient le personnage d'environ moitié. Le ciblage direct
du joueur pouvait aussi perdre son apparence, contrairement au survol.

La correction réutilise exclusivement les dessins déjà intégrés. Les portraits
du joueur, des drones, des humains armés, du bestiaire et des transformations
partagent un cadrage qui contient tous les pixels visibles, armes comprises,
sans conserver les marges transparentes. L'analyse ne modifie plus sa taille.
La barre de vie prend place sous le nom, à côté du portrait, pour conserver
la place des défenses. Les icônes de la fenêtre des compétences emploient aussi
un échantillonnage net. Les poses et proportions sur le plateau restent les mêmes.

Les limites des sources sont mesurées dans `sources.json` : les 23 références
historiques actives sont natives 32 px et leurs anciens portraits 128 px sont
des agrandissements exacts ×4. La banque du bestiaire comprend 61 dessins
32 px et quatre boss 64 px. Quatre drones utilisent leurs remplacements
détaillés, mais D57 utilise encore sa référence 32 px. Ces comptes concernent
des banques qui peuvent représenter les mêmes créatures, et ne s'additionnent
pas comme un nombre d'espèces.

Le cadrage corrige la taille et garde un rendu sans flou ; il ne crée aucun
détail absent des sources. L'uniformité de détail reste à obtenir par des
sources adaptées au 64 px pour les anciennes références. Cette demande ne
constitue pas la validation de nouveaux dessins : les futurs remplacements
restent soumis à la revue numérotée prévue par `AGENTS.md`. Les choix 32/64 px
du plateau conservent leurs règles acceptées.

Diagnostic reproductible : `project-rl.exe --ui-cold-inspection <sortie>`.
Il utilise le rendu commun et une partie isolée dont l'empreinte de simulation
doit rester identique. Il produit les comparaisons avec/sans analyse, les deux
thèmes, et des captures dans la vraie disposition du jeu en 1920 × 1080,
2560 × 1440 et 3840 × 2160. Ces captures sont des sorties temporaires ; le jeu
ne dépend d'aucune d'elles. Le bilan des contrôles est dans `validation.json`.
