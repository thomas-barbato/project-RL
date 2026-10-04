Aperçu graphique original - 4 octobre 2026 - décors, objets, butin et inventaire

Ouvrir apercu.html dans un navigateur. Aucune installation requise.

Contenu :
- 3 classes existantes : BRÈCHE, VIGIE, CREUSET.
- 3 apparences par classe : tenue de base, référence équipée, autre équipement.
- 17 identités de créatures établies, neutres comprises.
- 6 silhouettes humaines proposées et 1 zombie proposé.
- Comparaison dans des cases à 24, 32, 48 et 64 pixels.
- 16 variantes de sols : asphalte, béton, métal et ruines.
- 4 matières de murs, bordures des voisins reliées dans la scène.
- 133 modèles d'équipement de base : 61 armes à distance, 40 armes de mêlée,
  32 protections (14 corps, 6 tête, 6 mains, 6 pieds).
- Les anciens modèles encore présents dans les définitions sont inclus.
- Les affixes et suffixes ne multiplient pas les modèles visuels.
- 17 autres objets : trousse de secours, munitions, batterie, composants,
  pièces et outil d'ingénierie, charges, mine, leurre, camouflage et balise.
- 150 références ramassables couvertes, en réunissant les deux catalogues.
- Essai interactif : objets au sol réduits à 20 pixels dans les cases de 32,
  comparaison possible à 24 et 32 pixels, images du sac conservées à 32 pixels ;
  sélectionner, ramasser, reposer et réinitialiser les objets de démonstration.
- Les mêmes images sont également visibles comme butin dans la grande scène.
- Les « repair kits » sont présentés comme des trousses de secours,
  conformément à la demande ; l'identifiant core:repair_patch est conservé.

Fichiers des nouvelles planches :
sols-32-review-v1.png
murs-32-review-v1.png
equipment-firearms-1-v2.png
equipment-firearms-2-v2.png
equipment-melee-v1.png
equipment-armor-v1.png

equipment-catalog.json relie chaque identifiant de base à une case de planche.
equipment-catalog.js rend ce catalogue accessible même en ouvrant le HTML
directement depuis le disque. build_equipment_catalog.py relève les définitions
locales. check_equipment_catalog.py vérifie les cases sources et leur couverture.
equipment-coverage.json conserve le résultat : 133 identifiants et 133 cases
distinctes, aucune case vide, aucune case source strictement identique.

Le premier essai d'une grande planche d'armes n'avait pas assez de dessins.
Il a été écarté ; les deux banques v2 couvrent les 61 références. Il reste dans
le dossier des images générées, sans être chargé par cet aperçu.

Référence Crawl : tile_cell_pixels = 32 pixels logiques à l'échelle 1.
https://github.com/crawl/crawl/blob/master/crawl-ref/docs/options_guide.txt
Le catalogue des parties de personnage y sépare corps, bottes, tête, mains, etc.
https://github.com/crawl/crawl/blob/master/crawl-ref/source/rltiles/dc-player.txt

Ces planches sont des concepts produits avec imagegen intégré et présentés à
32 pixels par réduction. Elles ne constituent pas encore des assets de pixel art
finalisés nativement à 32 pixels ni des calques d'équipement assemblables.
L'aperçu change entre des apparences complètes pour examiner le résultat visuel.
La scène est un catalogue de présentation, pas une capture du client de jeu.
Les détails du matériel sont artistiques ; les protections de VIGIE et CREUSET
sont proposées. Le plastron BRÈCHE est montré porté sans changer la règle
actuelle, qui le place initialement dans le sac.

Les sources, règles, sauvegardes et paramètres du jeu ne sont pas modifiés.
Toute intégration attend l'acceptation visuelle de l'utilisateur.
PROMPTS.txt contient les consignes exactes utilisées.
PROMPTS-DECOR.txt et PROMPTS-EQUIPEMENT.txt contiennent les nouvelles consignes
exactes, exécutées avec imagegen intégré.

Les équipements sont actuellement illustrés individuellement. Leurs calques
portés, les points de prise en main, les orientations et l'assemblage réel sur
les trois classes restent à préparer avant intégration. Le catalogue n'affirme
pas que ces étapes sont déjà terminées. L'exactitude artistique de chaque
référence doit encore être examinée ; le contrôle de couverture ne la prouve pas.
Les nouvelles matières de terrain sont des concepts affichés à 32 pixels,
pas une validation finale de textures natives ni de tous les raccords du moteur.
apercu-sprites-v1.html conserve le premier aperçu sans ces nouvelles sections.

Les 17 nouveaux dessins sont enregistrés individuellement sous pickup-*-v1.png.
pickup-catalog.json et pickup-catalog.js relient chaque dessin à son identifiant.
build_pickup_catalog.py vérifie que tous les identifiants locaux d'objets et
d'armes figurent dans les catalogues réunis. check_pickup_catalog.py contrôle
transparence, couverture et doublons ; pickup-coverage.json conserve le résultat.
loot-inventory.js utilise le même modèle source au sol et dans le sac. À la
demande de l'utilisateur, le rendu au sol est désormais plus petit : 20 pixels
logiques centrés dans une case de 32 pixels, avec moins de petits détails.
Cette réduction est effectuée avant les agrandissements de la carte ; les
images du sac gardent leur version de 32 pixels. Un choix 20/24/32 permet
de comparer les trois tailles. Les petits marqueurs décoratifs de sol ont
été retirés pour alléger les objets.
Le rendu de toutes les petites images utilise également des aplats : sept
couleurs au maximum, regroupement des pixels de texture isolés, prélecture
à 24 pixels puis affichage dans les cases et lignes d'inventaire. Les planches
de référence ne sont pas réécrites par ce traitement de présentation.
La trousse de secours, les munitions et la batterie ont été redessinées avec
des formes et signes plus simples sous pickup-*-v2.png. Ces trois dessins sont
chargés à la place de leurs versions v1, conservées pour comparaison. Les
consignes exactes sont dans PROMPTS-SIMPLIFICATION.txt ; imagegen intégré a
été utilisé pour chacun de ces trois nouveaux dessins.

Révision actuelle après retour de l'utilisateur :
- Les propositions v2 de trousse et munitions ont été rejetées.
- Le soin devient une boîte rigide claire, grande croix et côtés sombres :
  pickup-repair_patch-v4.png ; nom de présentation « Boîte de soins ».
- Les munitions retrouvent leur volume et deux cartouches réelles en façade :
  pickup-weapon_matter-v3.png ; textures de rayures et rivets retirées.
- Ces deux sprites conservent leurs couleurs et ombres dessinées, sans
  traitement d'aplats automatique. Affichage 20 pixels au sol, 32 dans le sac.
- La batterie reste en v2 ; les autres références conservent leurs sources.
- PROMPTS-BOITE-SOINS-MUNITIONS.txt conserve les consignes exactes.
- « trousse », « repair kit » et « kit de soins » retrouvent aussi le soin
  dans la recherche du catalogue, sans changer l'identifiant de sauvegarde.

Dernier ajustement de contraste, sur demande de l'utilisateur :
- Le noir de la boîte de soins est remplacé par du rouge :
  pickup-repair_patch-v5.png. La face claire et la croix sont conservées.
- Les munitions sont représentées par une simple caisse verte :
  pickup-weapon_matter-v5.png. La forme reste volumétrique, sans accessoires.
- Ces deux v5 sont désormais les sources actives ; les anciennes versions
  restent disponibles pour comparaison. Taille au sol : 20 pixels par défaut.
- PROMPTS-LOOT-ROUGE-VERT.txt conserve les consignes imagegen intégrées exactes.
Les nombres de piles maximales sont relevés dans les définitions existantes.
PROMPTS-OBJETS.txt contient la consigne commune et les 17 sujets exacts ;
un appel imagegen intégré distinct a été utilisé pour chaque objet.
Cette simulation de ramassage ne change aucun inventaire, tour ou fichier de
sauvegarde de la partie. Le rendu réel dans le moteur attend encore validation.

Essai natif, après validation de la distinction plateau / grandes illustrations :
- Ouvrir apercu-natif.html (lien ajouté en tête de la section #loot précédente).
- Quatre objets sont dessinés directement sur une grille de 20 x 20 : boîte
  de soins rouge et ivoire, caisse verte, Brenek A-80 et Plastron Bastion.
- BRÈCHE existe en trois variantes sur une grille de 32 x 32. Les sols et
  le mur de cet échantillon sont également dessinés sur une grille de 32 x 32.
- native-art.js contient les coordonnées et palettes choisies au pixel près.
  build-native-assets.cjs exporte ces grilles en PNG RGBA sans redimensionner
  d'illustration, sans anticrénelage et sans filtrage de couleurs.
- native/ contient les PNG, les grilles sources et un manifeste de dimensions.
- L'inventaire affiche les illustrations de référence en 64 x 64 ; la cible
  en bas à droite les affiche en 128 x 128. Ces deux usages lisent directement
  les sources originales, pas un ancien sprite réduit à 32 pixels.
- Les quatre références au sol, BRÈCHE, les sols et les murs peuvent être
  ciblés par survol ou clic. Le ramassage et la remise au sol sont simulés.
- Les portraits et grandes illustrations restent des références antérieures :
  leur cohérence exacte avec les sprites natifs doit encore être examinée.
- Ce prototype ne convertit pas les 150 références du catalogue. Il ne fournit
  pas encore toutes les pièces d'équipement portées ni les ennemis natifs.
- Cette proposition reste indépendante du moteur Rust et attend l'avis visuel
  de l'utilisateur avant toute intégration.

Révision actuelle après retour sur l'essai natif :
- Les objets de 20 pixels et le nouveau personnage ont été rejetés.
- Les quatre objets sont redessinés directement en 32 x 32, sans agrandir
  simplement les grilles de 20 pixels. PNG actifs : native/*-32.png.
- La boîte de soins est un rectangle vu de dessus. Après rejet du rectangle
  vert, les munitions deviennent une cartouche de fusil vue de profil au sol.
  Le fusil est montré de profil et le plastron de face, sans perspective.
- BRÈCHE reprend exactement les trois cases de classes-v1.png utilisées par
  l'aperçu précédent, avec le même échantillonnage à 32 pixels. La planche
  classes-v1.png et les grandes images de l'inventaire restent inchangées.
- Le générateur n'exporte plus les essais natifs de personnages. Les anciens
  PNG de 20 pixels et essais de personnages sont conservés pour l'historique,
  mais ne sont ni dans le manifeste actif ni affichés dans ce prototype.
- La proposition courante demeure limitée à quatre objets avant extension.
- L'illustration de la caisse verte est conservée dans l'inventaire et dans
  la cible ; seule la représentation des munitions sur le plateau est modifiée.

Proposition de vues de face :
- apercu-face.html contient les trois classes, leurs neuf apparences et
  les 24 silhouettes du bestiaire précédent, avec une vue de face commune.
- Vue « Version précédente » permet de comparer les orientations avec les
  mêmes tailles, terrain, objets et éclairage de présentation.
- Les options de classe et d'équipement donnent accès aux neuf personnages.
  Chaque créature peut être examinée sur le plateau et dans la zone de cible.
  Le catalogue permet de voir tous les dessins à 32 pixels et agrandis x 2.
- classes-face-v1.png et bestiaire-face-v2.png sont des retouches générées
  par imagegen intégré ; les planches originales sont conservées.
- PROMPTS-FACE.txt contient les prompts exacts et les rôles des références.
- CRAWL-ETUDE.md conserve les observations des fichiers et du code officiels.
  Le dossier reference-crawl/ sert à l'étude ; aucun sprite de Crawl n'est
  affiché comme personnage de Project RL.
- Les nouvelles planches sont des propositions affichées à 32 pixels pour
  examen ; elles ne sont pas présentées comme des grilles natives dessinées
  à la main ni comme des pièces d'équipement modulaires déjà finalisées.
- Aucun changement du moteur ou du contenu de jeu dans cette proposition.
- La v1 frontale du bestiaire est conservée pour l'historique. La v2 est
  active après correction d'espacement par imagegen intégré. Les rectangles
  face-bestiary-frames.json évitent de couper les têtes et les corps quand
  les lignes de la planche générée ne correspondent pas à des cases égales.
- PROMPT-FACE-ESPACEMENT.txt conserve la consigne de correction ;
  check-face-atlases.py vérifie les 33 rectangles sources actifs.

Révision des ombres et de tous les équipements au sol :
- apercu-face.html utilise désormais 133 dessins natifs : les 61 armes à
  distance, les 40 armes de mêlée et les 32 pièces d'armure du catalogue.
- ground-equipment.js contient les profils de chaque modèle, les palettes
  et les silhouettes dessinées sur la grille finale de 32 x 32. Les formes
  sont regroupées par famille avec des profils propres aux modèles.
  Les illustrations d'inventaire servent de références visuelles et ne sont
  ni réduites ni filtrées pour fabriquer ces nouveaux dessins au sol.
- build-ground-equipment.cjs exporte native-equipment/ : 133 PNG RGBA,
  les grilles sources, le manifeste de couverture et une planche de contrôle.
- Le catalogue affiche chaque référence d'inventaire à côté du dessin au sol
  à sa taille réelle. Cliquer un modèle le place sur le plateau ; deux listes
  permettent aussi de choisir directement l'arme et l'armure à examiner.
- Les grandes images d'inventaire et de cible utilisent toujours les
  illustrations originales. Soin, munitions, terrain et planches de
  personnages/bestiaire approuvées sont conservés.
- Une petite ombre de contact est dessinée sous chaque personnage, humain
  ou créature présent sur le plateau. Son emplacement et sa largeur suivent
  le bas du sprite ; ce calcul est effectué une seule fois au chargement.
  Le bouton Ombres permet une comparaison avec et sans.
- Cet essai complet d'équipements reste une proposition dans l'aperçu,
  avant acceptation et intégration au jeu. Les anciens aperçus sont conservés.
