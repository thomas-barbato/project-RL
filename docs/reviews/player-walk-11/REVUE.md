# WALK11 · Proportions de face et de marche

Statut : **intégré le 25 octobre 2026**, après « ok tu peux integrer ».
Les 25 tenues, les six réparations de monstres et les variantes colorées utilisent
désormais ces ressources dans le rendu normal. Accord exact : [acceptance.json](acceptance.json).

Correction **WALK11-GRIP-r03** : les prises utilisent les mêmes centres de pixels
pour la main et pour l'arme, y compris après miroir ou rotation. Les derniers
contours de gant laissés dans le corps rejoignent leur bras ; les petits fragments
du poignet suivent la main sans s'étirer. Le gant est composé une seule fois pour
éviter un double contour semi-transparent. Les dessins d'origine sont conservés.

Le contrôle des prises couvre 2 690 compositions, avec les armes à une main,
à deux mains et les paires d'armes. Il exige maintenant une superposition réelle
des pixels de main et d'arme ; la tolérance d'un pixel de r02 est retirée.
Les 700 mains sont présentes et les 350 figures sans arme sont conservées.
[Voir le comparatif des mains du Bastion](grips/Bastion-prises-avant-apres.gif).

Les 25 tenues utilisent maintenant leur propre dessin de face comme référence de
gabarit. Les anciennes animations, qui avaient parfois épaissi les corps, ne
définissent plus la taille ou la silhouette à reproduire. Les figures sont dessinées
entières ; le conditionnement applique uniquement une réduction uniforme, le
placement et la séparation des couches. Les jambes ne sont ni déformées ni greffées.

La revue conserve les vraies armes et leurs couleurs, les deux mains compatibles et
les casques. La Traqueuse conserve ses deux jambes métalliques. Les mouvements vers
le haut et le bas restent ceux déjà acceptés, de face.

Ouvrir **Voir_animations.cmd** à la racine du projet. **Version intégrée** montre WALK11 ;
**Ancienne version** conserve la comparaison avec WALK09. Les onglets donnent accès aux
25 tenues, aux 471 modèles/couleurs de campagne et aux formes/drones. Chaque vue
individuelle montre la face de référence, les deux directions et les 14 poses.
La durée reste sélectionnable : 120, 200 ou 400 ms par case ; 400 ms est présélectionné.
Chaque animation parcourt exactement une case de 64 px. Les 520 ms de repos de
l'aperçu facilitent l'examen et ne constituent pas une pause ajoutée au jeu.

Comparaisons animées à l'échelle native, avec la vraie hache 67 équipée :

- [01 · Bastion, cinq tenues](Bastion-native64-axe.gif)
- [02 · Technomancien, cinq tenues](Technomancien-native64-axe.gif)
- [03 · Traqueuse, cinq tenues](Traqueuse-native64-axe.gif)
- [04 · Arcaniste, cinq tenues](Arcaniste-native64-axe.gif)
- [05 · Métamorphe, cinq tenues](Metamorphe-native64-axe.gif)
- [06 · Les six monstres corrigés](Monstres-native64.gif)

Les monstres 014, 015, 115 et 124 retrouvent les proportions de leurs dessins de
face. Le 014 porte son pistolet abaissé pendant la marche ; le 015 conserve le
bâton dans la même main. Les petits cultistes 022 et 024 sont agrandis de façon
uniforme dans les deux états, avec recalcul du centrage vertical. Leurs poses
précédemment acceptées sont conservées. Les autres monstres gardent leurs dessins
acceptés ; leurs variantes colorées sont animées dans la revue. Les neuf formes et
drones déjà proposés restent disponibles pour examen.

Contrôles initiaux des proportions : 200 configurations de corps/casque, soit 2 800 poses, et 565
configurations d'armes échantillonnées, soit 2 260 poses supplémentaires. Les
25 tenues n'ont plus d'écart supérieur à deux pixels avec leur hauteur de face.
La comparaison de silhouette se rapproche de la face ; une variation de largeur
du Technomancien en tenue 2 reste signalée pour examen visuel, car les bras sont
placés différemment. Ce signal ne remplace pas le jugement visuel.
Après la correction des prises r03, les25 atlas équipés de la hache ont été
rafraîchis, ainsi que les2690 compositions du contrôle de contact. Les captures
du moteur couvrent54 scènes ; les25 tenues et les cinq paires d'armes de la tenue5
ont aussi été examinées dans ces captures.

Le contrôle couvre aussi les 471 modèles/couleurs de monstres et vérifie que la
coloration ne change pas la géométrie. Il ne constitue pas un essai exhaustif de
toutes les combinaisons tenue × casque × arme × pose. Les PNJ et les anciennes
silhouettes statiques déjà vérifiées ne reçoivent pas de nouveau dessin.

Les sources, les instructions exactes de génération et les mesures sont conservées
dans ce dossier et dans `assets/source-art/player-walk-review-11/`. Les ressources
de l'aperçu se trouvent dans `assets/animations/walk-11/`. Les banques utilisées par
le jeu normal comprennent désormais WALK11 ; l'exécutable Release est reconstruit.
Les neuf cycles de formes/drones WALK08 gardent leur revue distincte.
Les sorties de diagnostic remplacées sont nettoyées après les contrôles d'intégration.

Lancement normal : **Jouer.cmd**. La validation du moteur et les captures utiles
sont conservées dans `integration-validation.json` et `integration-native/`.
Le registre `integration-cleanup.json` décrit les exports PNG reproductibles
retirés ; les sources, les comparaisons animées et la dernière revue native
restent conservées. Les planches brutes des prises se régénèrent avec le test
ignoré `walk11_grip_contact_diagnostic`, en définissant `RL_WALK11_GRIP_OUTPUT`
sur `docs/reviews/player-walk-11/grips/r03/after`, puis avec
`tools/art/review_walk11_grips.py`.
