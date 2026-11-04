# BIPED-LATERAL04-r01 — trois ailés, frontal conservé

Accord du joueur : « je valide :) ». Les trois cycles montrés sont acceptés,
dans les deux directions, en palette d'origine et à 400 ms par pas. Les douze
fichiers sources, atlas et fronts sont figés dans
`BIPED-LATERAL04-ACCEPTANCE-r01.json`. Dix modèles de monstres sont maintenant
validés ; l'intégration normale reste distincte.

L'accord « je valide on continue » accepte les marches de MON-164 et MON-165
montrées en BIPED-LATERAL03-r04. Il porte le total à sept identités de monstres
validées, en variante 0 et palette d'origine. Les empreintes correspondantes
restent figées dans les accords r03 partiel et r04. L'intégration normale reste
distincte de la revue des dessins.

Nouveau lot proposé : 01 MON-039 petit démon ailé, 02 MON-146 ailé doré,
03 MON-151 ailé pâle au brassard. 04 MON-144 reste la référence validée.
Les trois nouveaux fronts proviennent exactement des sprites actuels du jeu.
Ils ne sont ni redessinés ni remplacés par la neuvième pose générée. Les mêmes
pixels sont conservés à l'arrêt après une marche dans chacun des deux sens.

Les huit poses mobiles par direction suivent le cycle du Bastion : deux appuis
alternés, passage bas du pied, écartement compact et tête solidaire du buste.
Les ailes restent portées ; ce sont des déplacements au sol. La petite taille
de MON-039, la carrure dorée de MON-146 et le brassard unique sur le bras droit
anatomique de MON-151 font partie de l'identité à examiner dans chaque sens.

Sources créées avec l'outil intégré image_gen, conditionnées en figures entières
par réduction au plus proche voisin et alignement sur une ancre commune. Aucun
membre n'est découpé, déformé ou greffé. La hauteur native vient de chaque
frontal d'origine. Les cadres de 160 px et panneaux de 192 px donnent des marges
aux ailes ; les cases du monde restent de 64 px.

Les nouveaux candidats sont placés sur des pixels entiers dans cette capture.
Cela évite un échantillonnage divergent aux demi-pixels entre la texture du jeu
et sa copie avec marges plus larges. La référence validée MON-144 conserve
exactement ses positions précédentes pour permettre le contrôle pixel à pixel.
L'ombre de contact des trois nouveaux monstres reprend aussi la largeur de
leur fiche actuelle, afin de comparer frontal du jeu et frontal conservé avec
le même rendu jusqu'aux pieds. La référence MON-144 garde son ombre de revue.

Cadence : 400 ms par pas, 800 ms par cycle complet, avec les poids de phases
validés. La pause de revue de 400 ms à chaque extrémité n'est pas une règle de
jeu. Les comparatifs montrent le frontal conservé et la nouvelle marche,
la référence du jeu et la marche, ou les trois colonnes. Le zoom est entier ×2.

Sources et atlas : `assets/source-art/animation-polish-01/biped-lateral-04/r01/`.
Prompts : `BIPED-LATERAL04-PROMPTS-r01.md`. Capture Debug :
`--ui-cold-biped-winged04 OUTPUT`, puis
`tools/art/build_biped_winged04_review.py` et
`tools/art/check_biped_winged04.py`.

Ces trois cycles ont été présentés à taille native dans
le décor du camp, avec l'exposition habituelle des monstres. Les sept dessins
acceptés, le rendu normal, les règles, sauvegardes et préférences sont préservés.

Vérifications réussies : compilation Debug verrouillée, rustfmt ciblé,
compilation Python et diff sans erreur. Les 264 captures couvrent quatre modèles,
avec huit poses distinctes par sens. Les 26 empreintes figées sont intactes ;
les 66 captures de la référence validée sont identiques. Les 198 comparaisons
entre frontaux actuels et copies conservées, ainsi que les six arrivées des
candidats, sont identiques pixel par pixel. Chaque export animé dure 2 400 ms.

Le lecteur a été contrôlé sur les quatre modèles, avec les deux arrêts des
trois candidats, leurs comparatifs par défaut et la référence signalée validée.
Paire native 384 × 192, trois colonnes 576 × 192 et zoom entier 1 152 × 384
vérifiés. La planche native mesure 768 × 576. Preuves dans
`artifacts/animation-polish-01/biped-lateral-04/r01/` :
`native-validation.json`, `browser-checks.json` et `browser-proof.png`.
Le joueur a accepté la qualité et la continuité des trois nouvelles marches.
