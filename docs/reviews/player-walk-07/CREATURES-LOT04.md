# WALK07-M21 à M26 — quatrième lot, cinq candidats acceptés

« Ok on continue :) » accepte le troisième lot M15–M20.
Les six sources sélectionnées,84 poses natives et leurs calages exacts
sont figés dans `visual-acceptance-creatures-lot03.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot03/`.
Les19 cycles acceptés et le placement WALK07-P01 restent conservés.
Le joueur accepte les cinq candidats présentés par « ok parfait :) je valide :) ».
Leur accord exact et leurs70 poses natives sont figés dans
`visual-acceptance-creatures-lot04.json` et
`assets/source-art/player-walk-review-07/accepted/creatures-lot04/`.
M23 reste différé : les essais ne conservent pas encore ensemble les deux
directions et le côté du bras gris. Il n'entre pas dans la présentation.

| Candidat | Identité existante | Particularités à conserver |
|---|---|---|
| M21 | MON-081-0 | Quatre pattes courtes, couverture dorsale olive, dessous rose et appendice arrière |
| M22 | MON-092-0 | Quatre appuis, dos pâle arqué, plis rouges et griffes sombres |
| M23, différé | MON-127-0 | Bras gris et orientations encore à corriger, absent de l'aperçu |
| M24 | MON-139-0 | Capuche verte, bras squelettiques, livre ouvert et base enroulée sans jambes humaines |
| M25 | MON-147-0 | Deux jambes, deux bras, deux ailes violettes, cornes et tenue sombre |
| M26 | MON-148-0 | Deux jambes, deux bras, deux ailes, corps brun massif, grandes cornes et pilosité sombre |

Les libellés servent à la revue et ne renomment pas le contenu du jeu.
La catégorie technique ancienne `float` de MON-139-0 ne suffit pas à définir
sa locomotion : la base enroulée est conservée et progresse par contractions
mesurées. Le livre n'est pas une nouvelle arme ou un nouvel objet de gameplay.
Il reste soutenu du même côté du corps pendant les deux orientations ;
la main libre et les appendices ne deviennent pas des supports humains.
L'esprit M23 reste à reprendre sans pieds ajoutés ; les deux ailés marchent au sol.
La larve M12 reste différée, hors de ce lot.

## Méthode et ressources

Fabrication avec l'**outil image_gen intégré**, sans CLI de génération.
Les sources sélectionnées sont enregistrées dans
`assets/source-art/player-walk-review-07/<identité>-cycle-lot04-r*.png`.
La sélection est `MON-{081,092,139,147}-0-cycle-lot04-r01.png` et
`MON-148-0-cycle-lot04-r02.png`, dans ce dossier de sources.
`creature-jobs-lot04.json` désigne les fichiers réellement choisis et les
paramètres de découpe. Les références actuelles exactes restent dans
`references/<identité>-native-current.png` ; les agrandissements de fabrication
et les cycles acceptés servent respectivement à l'identité et au mouvement.

Les entrées et prompts complets sont conservés dans `creature-jobs-lot04.json`,
`prompts/<identité>-lot04-r01.txt` et `generation-creatures-lot04-r01.json`.
Les corrections de direction et de propriété du bras sont documentées dans
`creature-corrections-lot04-r02.json` et leur journal de génération.
Le premier dessin de M26 regardait du même côté sur les deux rangées ;
sa reprise r02 présente deux orientations distinctes. Les essais r01/r02 de
M23 échangent le côté de leur élément de bras gris ; r03 conserve ce côté,
mais ses deux rangées regardent encore à droite. Ils sont écartés.
Les prompts et sorties de cette reprise restent dans `creature-arm-lot04-r03.json`,
`generation-creatures-lot04-r03.json` et `creature-deferred-lot04.json`.
Ces sorties ne sont pas considérées comme acceptées.

Chaque créature possède14 figures entières : un vrai repos et six phases
pour chacune des deux directions proches de la face. Le conditionnement
extrait, réduit uniformément et aligne les figures complètes ; il ne greffe
ni tête ni jambes et ne déforme pas les membres pour fabriquer la marche.
Une source touchant le bord extérieur avec des pixels opaques est refusée
avant export. Les variantes et silhouettes des autres lots gardent leurs
identités, dimensions et accords propres.

## Placement et comparaison

Les petits monstres gardent le centrage vertical demandé : un seul décalage
calculé sur l'enveloppe de toutes les poses si sa hauteur est inférieure
à64px. La figure entière et l'ombre partagent ce calage, sans recentrage image
par image. Les plus grands gardent l'ancrage lié à leur taille. Le détail×2
reprend les pixels de la scène native et sa grille ; la case reste64×64px,
le canevas128 ne sert qu'aux marges et débordements.

L'aperçu courant de créatures est réutilisé : dessin actuel exact à gauche,
nouveau cycle à taille native, puis zoom×2. Sols sombre/clair/texturé, pause,
pas manuel,400ms par défaut et choix120/200/400ms sont conservés. Trois phases
accompagnent une case, les trois suivantes la suivante. Le repos est un dessin
dédié ; les520ms de pause de démonstration ne sont pas un délai du jeu.
Les déplacements verticaux, tours et collisions restent inchangés.

## Vérification et limites

`creature-packing-lot04.json` et `creature-validation-lot04.json` conservent les
sources, le nombre de figures, les marges, les échelles, les cadrages et les
calages. Les266 poses des trois lots acceptés et leurs sources sont comparées
aux références figées. Le lecteur est vérifié avec Node et DOM/canvas simulé ;
aucune exécution réelle de ces candidats en navigateur ou en jeu n'est
revendiquée. Les planches complètes sont examinées à taille native.
Les dimensions, empreintes et différences de pixels ne prouvent pas à elles
seules une bonne démarche, une direction cohérente ou l'identité d'un membre.

Les contrôles du lecteur restent dans `renderer-checks.json` ; ceux des sources
et des atlas WALK06 actifs, dans `source-manifest.json` et
`bastion-transfer-checks-r07.json`. Comparaison native :
`artifacts/player-walk-review-07/creature-lot04-comparison-native.png`.

Ce lot est accepté visuellement et reste hors du rendu actif. Aucun dessin
du rendu actif, aucune statistique ni règle de gameplay n'est remplacé.

Résultats du24 octobre :70 poses proposées pour M21, M22, M24, M25 et M26.
Les hauteurs natives sont31–34,37–43,76–78,82–84 et91–94px ; les décalages
verticaux fixes sont−15,−10,0,0 et0px. Les sources sélectionnées ont toutes
14 figures entières et aucun pixel opaque au seuil128/255 sur leur bord
extérieur. Les266 poses acceptées restent exactes. Les2949 sélections des
données de revue passent, ainsi que les300 fixtures du compositeur ; les601
instants du contrôleur gardent la cadence acceptée. Les six aperçus courants
restent sous1Mo. Ces contrôles techniques ne constituent pas l'accord
artistique du joueur, enregistré séparément pour les cinq candidats montrés.
