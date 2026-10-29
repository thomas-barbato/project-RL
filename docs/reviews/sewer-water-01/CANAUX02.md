# EG-CAN02-r01 — un réseau d'eau relié

Le 28 octobre 2026, le joueur apprécie le rendu EG-EAU01 et demande davantage
de couloirs d'eau. Il précise ensuite : « relie les portions d'eau ensemble »
et demande que la construction aléatoire respecte cette règle, avec des passages
pour le joueur et les monstres. La finition de l'eau est conservée ; le nouvel
agencement fait l'objet du comparatif 02 courant.

## Règle de construction

Chaque plan doit posséder **un seul réseau d'eau**, relié par les quatre côtés
des cases. Un contact diagonal ne suffit pas. Le réseau continue sous les
grilles, qui restent des cases praticables. Toutes les zones sèches, les
personnages et les accès de salle doivent rester accessibles depuis l'escalier.
Une construction qui viole une de ces contraintes est refusée.

Le jeu choisit actuellement entre trois agencements existants selon la graine.
Il ne fabrique pas encore de nouvelles salles arbitraires. La proposition
reprend ces trois agencements et raccorde leurs portions d'eau, en ajoutant
quelques traversées. Les futurs plans doivent passer le même constructeur et
son contrôle de connectivité, plutôt que dépendre d'un simple contrôle visuel.

Les canaux et ponts historiques, la géométrie des murs, le mobilier, les corps,
les populations, leurs positions et l'escalier sont conservés. Les nouveaux
canaux évitent les emprises des objets. Des grilles supplémentaires préservent
les chemins secs et les trajets des rats. La zone de dégagement de l'escalier
est préservée. L'eau reste infranchissable et les grilles praticables, selon
les règles déjà employées par les égouts.

## Proposition sur les trois variantes

- Variante 1 : 60 → 113 cases d'eau, 16 → 26 cases de grille ; les onze anciens
  ensembles d'eau constituent désormais un réseau unique.
- Variante 2 : 68 → 118 cases d'eau, 16 → 29 cases de grille ; les douze anciens
  ensembles d'eau constituent désormais un réseau unique.
- Variante 3 : 52 → 108 cases d'eau, 15 → 33 cases de grille ; les dix anciens
  ensembles d'eau constituent désormais un réseau unique.

Une passerelle peut occuper plusieurs cases de grille ; ces nombres comptent
des cases, pas des ouvrages distincts. Les 85, 104 et 78 emplacements de monstres
et habitants restent praticables et accessibles. Les nouvelles contraintes
spatiales peuvent changer les détours tactiques ; les chiffres de combat et
les récompenses ne sont pas modifiés.

## Sources et vérification

`tools/reviews/build_sewer_canals_02.py` prépare les trois plans sous
`assets/source-art/sewer-water-02/` et refuse un réseau incomplet. Il ne modifie
pas les plans livrés. `canals-plan-validation.json` conserve les empreintes,
les chemins ajoutés et les contrôles d'accès.

Le constructeur partagé dans `sewer.rs` contrôle aussi les deux graphes : eau
et sol praticable. La revue utilise ce constructeur strict pour les plans
candidats. Les anciens plans et anciennes sauvegardes conservent leur voie de
construction historique distincte pour conserver leurs collisions d'origine.

Les tests contrôlent 1 000 tirages de graine couvrant les trois agencements.
Ils incluent des contre-exemples : bassin isolé, contact diagonal insuffisant
et canal coupant les deux rives sans passerelle. Le même canal avec une grille
praticable doit permettre de rejoindre les deux rives.

Le lanceur courant `tools/reviews/show_sewer_water_review_01.cmd` présente les
deux réseaux côte à côte avec **la même nouvelle eau**. Les trois comparatifs
natifs et l'animation courante sont sous `artifacts/sewer-water-review-01/`.
Les cartes entières sont exportées en 3 008 × 2 816 px ; les vues d'ensemble
réduites sont explicitement légendées à l'échelle 1:4. Les vues de détail et
l'animation conservent les cases de 64 px.

L'export vérifie qu'aucun pixel ne change hors des anciennes cases d'eau et des
nouvelles cases de canal ou de grille, y compris sur les cartes entières. Les
états de simulation sont identiques avant et après capture ; l'eau reste figée
en mouvement réduit. Aucun combat ou parcours d'IA n'est simulé par cet export.

Cette étape de proposition est conservée dans `canals-validation.json`. Le
28 octobre, le joueur valide ensuite explicitement les tracés et ajoute la
contrainte de ne coincer ni le joueur ni les monstres. `acceptance.json`
conserve son accord, sa précision et les empreintes des pixels montrés. Les
références de l'eau appréciée restent dans `references/water-r01-native64.*`.

## Intégration — génération 167

Les nouvelles parties utilisent trois identifiants de zone distincts,
`core:prison_sewer_canals_01` à `03`. Le choix dépend de la même graine et ne
consomme pas l'aléatoire de combat. Les plans exacts acceptés et la grille
existante sont conditionnés sous `assets/maps/prison-sewer-canals-02/`. Le jeu
ne dépend pas des fichiers de revue ou de `artifacts/`.

Le constructeur applique les obstacles puis refuse toute zone sèche isolée,
tout personnage placé dans un obstacle, toute grille infranchissable ou tout
réseau d'eau interrompu. Le graphe des cases praticables doit contenir au moins
un déplacement et rester intégralement relié à l'escalier. Ce contrôle couvre
les emplacements du joueur, des monstres et des habitants. Il concerne le
décor statique ; les acteurs continuent à se déplacer selon les règles du jeu.

Les anciens identifiants conservent les plans d'origine, y compris lorsqu'une
sauvegarde possède un passage encore inexploré. La restauration reprend les
obstacles du plan correspondant à l'identifiant, sans déplacer les personnages.
Un joueur ou un monstre présent sur un ancien sol qui devient de l'eau dans le
nouveau tracé retrouve ainsi son sol d'origine. Les nouveaux réseaux sont
recontrôlés après remise en place des obstacles de sauvegarde.

Le matériau d'eau et le dessin des nouvelles berges sont extraits du prototype
vers `sewer_water.rs`. La même fonction sert désormais à la revue et au jeu.
Le mouvement réduit conserve l'horloge d'ambiance à zéro. Les anciennes cartes
reçoivent ce rendu d'eau tout en gardant leurs collisions historiques.

Les résultats finaux, les tests de mobilité et de reprise, les captures du
renderer normal et l'empreinte du Release sont enregistrés dans
`integration-validation.json`. Le test graphique part d'une rue isolée près de
la trappe ; il contrôle l'entrée et le retour réels puis les vues des canaux.
Il ne simule pas une traversée de combat de la ville ou un équilibrage.

Validation finale : 30 tests ciblés réussis, dont 1 000 tirages de graine et les
reprises avec un joueur et un monstre sur d'anciens sols devenus canaux dans le
nouveau tracé. Les trois planches natives acceptées sont reproduites à l'octet
près par le renderer partagé. Les 12 captures du jeu normal passent pour les
trois variantes. Les builds Debug et Release, le contrôle de toutes les cibles,
le formatage et les espaces du diff passent. La suite complète n'est pas lancée.

Une installation isolée sans répertoires `assets/` ou `content/` charge le
Release depuis `data.rlpak` : intégrité, contenu, menu, audio, 1 369 textures PNG
et sept atmosphères sont contrôlés. L'inventaire compte 1 819 ressources logiques
et 1 764 fichiers empaquetés après déduplication. Le paquet temporaire est
supprimé ; la distribution existante sous `dist/` n'est pas remplacée. Le Release
de `Jouer.cmd` est reconstruit pour jouer depuis le projet.
