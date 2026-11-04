# BIPED-LATERAL03-r03 — même dessin de face et en marche

Le joueur préfère l'apparence des cultistes en mouvement et constate une autre
identité de face. La cause est dans le lecteur natif : à l'arrêt, r02 appelait
encore le frontal VERT18 du jeu, alors que les nouveaux atlas contiennent déjà
leur propre pose frontale de repos. Ce n'est pas une différence d'éclairage.

Pour MON-164-0, MON-165-0 et le cultiste sans arme MON-014-0 affiché dans
l'aperçu, r03 sélectionne la neuvième figure entière de la première ligne
de leur atlas courant : un vrai repos frontal, deux pieds au sol.
Cette même image est utilisée à l'arrivée depuis les deux côtés. Les trois
figures sont extraites sans retouche ni redimensionnement dans
`assets/source-art/animation-polish-01/biped-lateral-03/r03/MON-164-front.png`
ainsi que `MON-165-front.png` et `MON-014-front.png`. Le gabarit, l'ancre [64,112], l'offset et le filtrage
nearest sont ceux de leur cycle. Aucun nouveau dessin généré ni pièce greffée.

Les sept atlas de marche restent référencés à leur emplacement précédent,
avec les mêmes empreintes. Les 192 captures en mouvement des trois cultistes
doivent rester identiques à r02 ; seuls leur frontal de référence et leur
retour au repos changent. Les quatre autres modèles gardent entièrement leur
rendu. La cadence reste de 400 ms par pas, 800 ms par cycle ; la pause de
revue de 400 ms n'est pas un délai de jeu.

Le comparatif des trois cultistes montre l'ancien frontal, le frontal issu
du nouveau dessin, puis la marche conservée. Par défaut, les deux dernières
colonnes permettent de comparer l'identité au repos et en mouvement. À la
phase Repos, les deux images correspondent exactement. L'ancien frontal
reste visible dans la comparaison à trois colonnes et sur la planche fixe
qui montre aussi les deux directions. Les autres modèles gardent le joueur
et leur frontal courant comme références. Les images nécessaires sont
préparées avant lecture ; le lecteur conserve au plus deux banques.

La préférence du joueur porte sur l'identité de marche, pas sur une nouvelle
intégration ni sur tout le bestiaire. Le frontal nouvellement affiché reste
à examiner dans cette revue. Les déplacements verticaux, règles, sauvegardes,
préférences, autres palettes et rendu normal du jeu sont conservés.

Reproduction : `tools/art/prepare_biped_fronts_r03.py`,
`--ui-cold-biped-fronts-r03 OUTPUT`,
`tools/art/build_biped_fronts_r03_review.py`,
`tools/art/check_biped_fronts_r03.py`.

Contrôles réussis : compilation Debug et rustfmt ciblé, compilation Python,
diff sans erreur. Les sept empreintes d'atlas sont conservées. Les 192 captures
en mouvement des trois cultistes restent identiques pixel par pixel à r02 ;
les 264 captures des quatre autres modèles sont également identiques. Les six
arrivées utilisent exactement la nouvelle face de référence et diffèrent de
l'ancien frontal. Les 198 panneaux de l'ancienne face restent inchangés.
Les sept animations exportées durent 2 400 ms par aller-retour.

Le navigateur charge les trois cultistes, montre le repos dans les deux sens,
et conserve l'affichage natif 320 × 192, les trois colonnes 480 × 192,
le zoom entier 960 × 384 et la planche 640 × 576. La comparaison des autres
modèles garde son frontal habituel. Lecture à 400 ms active, sans erreur
console observée. Preuves : `native-validation.json`, `browser-checks.json`
et `browser-proof.png` sous `artifacts/animation-polish-01/biped-lateral-03/r03/`.
