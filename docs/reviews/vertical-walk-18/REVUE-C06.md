# VERT18-C06 — toutes les armes et le bestiaire courant

Demande du 29octobre 2026 : « ok maintenant il faut faire la meme chose pour
toutes les variantes d'armes et tous les monstres ».

Statut : **lot validé et intégré au rendu normal Debug et Release**.
Accord du joueur : « ok, :) je valide :) ». La décision visuelle est dans
`acceptance-C06.json` ; l’intégration et ses contrôles dans `integration-C06.json`.
Les accords C01/C02/C03/C05 restent distincts et leurs ressources sont préservées.

L’aperçu courant est `http://127.0.0.1:63780/extension-C06/apercu.html`.
Il présente les dessins à un pixel pour un pixel, avec deux déplacements
verticaux, un arrêt frontal après chaque case, et l’inspection des 13 poses.
Les identifiants `W000`–`W112` et `MON-nnn` restent stables dans les sélecteurs,
les sources, les captures moteur et les décisions à venir.

## Couverture de ce lot

- 113 dessins d’armes partagés :39 à une main et 74 à deux mains, sept couleurs
  canoniques et les 152 liaisons de main. Aucun dessin d’arme n’est réinventé.
- Huit tenues présentes dans C05 : les quatre classes actuelles, en tenue de
  base ou armure lourde. Pour chacune : une prise à une main et trois prises
  à deux mains dessinées en figures entières, les six casques et la tête nue.
- Les 1521 paires ordonnées d’armes à une main sont sélectionnables pour chaque
  tenue. Le corps et les doigts ne sont composés qu’une fois par pose.
- Les 129 modèles du bestiaire de campagne actuel, avec leurs471 couples
  modèle/couleur. Les 13 dessins de lanceurs de sorts réservés sont inclus
  comme dessins seulement ; aucune capacité, rencontre ou apparition ajoutée.

Cela ne déclare pas terminées les 12 tenues intermédiaires encore absentes de
C05, les cinq tenues de compatibilité du Technomancien, les transformations,
les drones, les autres PNJ ou les anciens dessins de repli. Les boss18/19
retirés du catalogue courant ne sont pas réintroduits. Le suivi global reste
dans `COUVERTURE-EQUIPEMENT.json`.

## Composition et cohérence

Les bras à deux mains sont dessinés avec le corps entier. Les prises d’armes
utilisent une rotation et une translation rigides, à échelle fixe pour toute
la séquence face/dos/repos. Le petit écart entre les deux points de prise est
réparti entre les deux poings sans étirer l’arme. Les doigts proviennent de la
pose concernée ; aucun ancien bras n’est ajouté sous la nouvelle manche.

De dos, une arme tenue devant le torse passe derrière le corps. Sa symétrie
horizontale et ses points d’attache tournent ensemble ; le chargeur ne se
retrouve pas au-dessus du fusil. Les armes à une main conservent le contrat
de latéralité de C05. Les casques remplacent seulement la zone de tête.

Chaque morphologie de monstre possède ses figures entières. Les espaces
transparents réels des sources servent de limites de découpe, notamment pour
les ailes. L’agrandissement éventuel du canevas ajoute du vide, sans grossir
le monstre. MON021 et MON071 ont de vrais dos sans mandibules frontales.
MON022 conserve le même sens de fusil entre le repos et toutes les poses de
marche ; le fusil tourne avec l’anatomie entre la face et le dos.
MON142 montre son axe longitudinal nord/sud : sa longueur physique est calibrée
sur la grande dimension de la référence de profil, et non sur sa faible hauteur.

Pour le ver sans membres MON141, trois figures cohérentes par direction sont
répétées lors de la seconde action de déplacement. Les figures inversant le
sens de la tête et de l’enroulement sont exclues. La pose de repos est un
enroulement complet posé au sol. Un même calibrage fixe est appliqué à toutes
les figures sélectionnées, sans adaptation d’échelle en cours d’animation.

## Vérifications du prototype et limites

`ANALYSE-C06.json` rassemble les résultats. Les contrôles couvrent les 1216
liaisons tenue/arme/main, la conservation de l’alpha pour les 1064 dessins
arme/couleur/main, les 2496 superpositions de casque sans modification du bas
du corps, et les 129 gabarits de monstres. Les 12168 choix de paires sont
contrôlés par composition des liaisons indépendantes ; ils ne constituent
pas12168 dessins différents examinés humainement.

Le diagnostic isolé du moteur a dessiné1591 atlas, soit 20683 poses natives,
et vérifié6364 retours frontaux. Il utilise le même passage de finition
d’acteur pour toutes les poses.342 planches de contact sont conservées,
avec copies WebP dont les pixels sont vérifiés identiques aux PNG. MON141
a fait l’objet d’une reprise ciblée dans le moteur après la sélection finale,
ainsi que MON022 après correction du port de fusil.

La préparation lit et décode sur un thread de travail ; la création de texture
reste sur le thread graphique, avant dessin. Le cache de textures est borné
à 32 Mio ; l’aperçu utilise aussi un cache borné à 32 Mio et prépare l’assemblage
avant l’animation. Ces contrôles du prototype ne mesurent pas les performances
d’une partie et ne constituent pas une intégration du préchargement normal.
Aucune campagne, préférence, règle de combat, statistique ou sauvegarde modifiée.

## Intégration normale après accord

Le moteur prépare les acteurs visibles avant le dessin et anticipe quatre
acteurs proches. Le service commun `asset_preparation` lit, décode et compose
sur un thread CPU ; il crée progressivement les textures sur le thread du
contexte graphique. Ses caches sont bornés à 64 Mio de sources décodées et
128 Mio de textures composées, avec éviction des moins récemment utilisés.
Ces budgets concernent ce service, pas toute la mémoire du jeu. Les anciens
fonds, interfaces et effets restent à raccorder à cette politique commune.

Les figures sont installées sans rééchantillonnage sous
`assets/animations/vertical-18/`, sans dépendance aux captures ni aux sources
de travail. Un cadrage commun aux 13 poses conserve le gabarit et l’ancrage.
Le rendu normal réemploie la finition lumineuse et l’ombre de contact des
acteurs. À la fin du déplacement ou en mouvement réduit, il retrouve le
repos frontal du cycle. Les marches latérales restent inchangées.

La comparaison de 1591 configurations, soit 20683 poses, respecte une tolérance
maximale de 2/255 sur les couleurs prémultipliées et 1/255 sur l’alpha, liée
aux arrondis de composition. Le diagnostic du rendu normal couvre aussi
NPC-01, 41392 dessins et 19104 retours frontaux, puis huit captures dans
le vrai rendu du camp. Les vérifications ciblées ne constituent pas une
exécution de toute la suite de tests du projet.

Le signalement ultérieur sur le pistolet orienté vers le bas est suivi
séparément dans `proposal-C07.json`. Le 30 octobre, le joueur conserve A et
ne retient pas B horizontale (`decision-C07.json`). Sa précision sur une lame
de couteau légèrement inclinée vers le bas ou le haut donne lieu au comparatif
C08. Le joueur choisit B, la lame légèrement vers le bas
(`acceptance-C08.json`). Cette correction porte uniquement sur les rotations
des couteaux et dagues présentés ; les sources C06 restent figées. Son état
d’intégration et ses contrôles sont dans `integration-C08.json`.

Le joueur valide ensuite C09-B par « Oui, intégrer B » : les pistolets
s’ouvrent de 10° vers l’extérieur depuis ce port bas, et le couteau C08-B
reste identique. Cette révision utilise toujours les mêmes images et points
de prise. Les sources C06/C08 restent figées ; voir `acceptance-C09.json`
et `integration-C09.json` pour le périmètre et les contrôles courants.

## Sources et reproduction

Les sources sont sous `assets/source-art/vertical-walk-review-18/extension-C06/`.
Le mode de dessin utilisé est **builtin_imagegen**. Les prompts exacts et les
images de référence sont dans `prompts-C06-monsters.json`,
`prompts-C06-weapons.json`, `prompts-C06-corrections-r02.json`,
`prompts-C06-corrections-r03.json`, `prompts-C06-correction-MON-141.json`,
`prompt-C06-crab-back-r02.json` et `prompt-C06-MON-076-r02.json`.
La correction du fusil est dans `prompt-C06-correction-MON-022.json`.
`generation-C06/` conserve la provenance de chaque dessin et correction.

1. Exporter les références avec les tests ignorés `export_vertical_catalog18`
   et `export_vertical_weapon_palettes18` de `cargo test --locked --bin project-rl`.
2. Exécuter `pack_vertical_monsters18.py`, `pack_vertical_weapons18.py`,
   puis `compose_vertical_extension18.py` et `verify_vertical_extension18.py`.
   Les deux derniers et le conditionnement d’armes exigent le Python du runtime
   de travail avec NumPy ; ils ne nécessitent aucune installation supplémentaire.
3. Exécuter le test ignoré `prepare_vertical_monster_palettes18`, puis
   `audit_vertical_extension18.py` et `publish_vertical_extension18.py`.
4. Après compilation Debug, lancer `project-rl.exe --ui-cold-vertical-extension`
   avec un dossier de sortie, puis `publish_vertical_extension_engine18.py`.

Les sources et entrées du diagnostic moteur sont dans `assets/`. Les captures
sont dans `artifacts/` et ne sont jamais une dépendance du jeu normal.
Ne pas relancer les générateurs d’images pour reproduire un aperçu : les
sources dessinées sont conservées. Ne pas remplacer une validation par un
nouveau dessin portant le même nom. Pour reproduire l’intégration acceptée,
exécuter `install_vertical18.py`, puis les tests de `game_art::vertical::tests`
et le diagnostic `--ui-cold-vertical-integration`. Les nouveaux candidats
ultérieurs suivent leur propre validation, conformément à `AGENTS.md`.
