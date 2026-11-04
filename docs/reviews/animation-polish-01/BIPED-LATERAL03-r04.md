# BIPED-LATERAL03-r04 — marcher avec l'identité du frontal retenu

Accord du joueur : « je valide on continue ». Les deux marches proposées
ci-dessous sont désormais acceptées dans le périmètre montré. Empreintes
figées dans `BIPED-LATERAL03-ACCEPTANCE-r04.json` ; l'intégration reste distincte.

Le joueur constate encore une autre apparence entre face et marche pour
02 MON-164 et 03 MON-165, et préfère cette fois leur face. La vue affichée
était « Frontal harmonisé / marche » de r03. Ces deux fronts sont retenus
exactement ; les nouvelles phases mobiles doivent en reprendre l'identité.

« sinon le reste est ok » valide les cinq autres modèles du lot affiché :
MON-145, 005, 014, 156 et 144, palette d'origine, directions et repos montrés.
Cela inclut le frontal harmonisé de MON-014. Les empreintes des dessins et
des deux faces retenues sont figées dans
`BIPED-LATERAL03-ACCEPTANCE-r03-partial.json`. Aucune intégration normale
n'est impliquée par cet accord visuel.

R04 ne redessine que les marches de 02 et 03. Chaque frontal r03 est l'unique
référence d'apparence : capuche, visage, carrure, manches, gants, pantalon,
bottes, tabard et arme. Le Bastion guide uniquement les huit poses de pas.
Les figures sont entières, les bras conservent leurs deux prises et la tête
suit le buste. Le visage de 03 reste une ouverture noire sous la capuche ;
celui de 02 conserve son gris sombre. Pas de transfert des protections ou de
la peau du joueur de référence. Le contrôle inclut le sens de la capuche.

Les deux sources par monstre sont produites avec l'outil intégré image_gen.
Le conditionnement réduit et aligne les figures entières ; aucun membre n'est
cisaillé ou greffé. La neuvième pose générée n'est pas utilisée : la face
native retenue est conservée telle quelle dans les deux lignes de l'atlas,
ainsi que comme ressource de repos commune aux deux directions. Les cinq
autres atlas et le frontal de MON-014 restent référencés sans modification.

Le comparatif par défaut affiche frontal retenu / nouvelle marche. La paire
marche r03 / marche r04 et les trois colonnes permettent aussi la comparaison
à phase égale. Pour les modèles validés, les comparatifs précédents sont
conservés. Cadence : 400 ms par pas, 800 ms par cycle, 400 ms de pause de revue
à chaque bout du trajet. Cette pause n'est pas une règle de jeu.

Sources et atlas : `assets/source-art/animation-polish-01/biped-lateral-03/r04/`.
Prompts : `BIPED-LATERAL03-PROMPTS-r04.md`. Capture :
`--ui-cold-biped-frontmatch-r04 OUTPUT`, puis
`tools/art/build_biped_frontmatch_r04_review.py` et
`tools/art/check_biped_frontmatch_r04.py`.

Les deux marches ont été présentées à taille native dans le camp,
avec même ancre, position et exposition habituelle. Les autres palettes,
le rendu normal, les règles, sauvegardes et préférences sont conservés.

Vérifications réussies : compilation Debug, rustfmt ciblé, compilation Python
et diff sans erreur. 462 captures natives ; 18 empreintes figées contrôlées.
Les 330 captures des cinq modèles validés restent identiques pixel par pixel.
Les 132 vues du frontal retenu et les 132 panneaux de la marche r03 sont
également identiques aux références. Les quatre arrivées de 02/03 reprennent
exactement leur frontal. Les sept exports restent à 2 400 ms par aller-retour.

Le lecteur a été contrôlé sur les sept modèles : nouvelles marches en revue,
autres modèles validés ; comparatifs et sélection par défaut adaptés.
Passage des pieds et vrai repos dans les deux sens pour 02/03, affichage natif
320 × 192, trois colonnes 480 × 192, zoom entier 960 × 384 et planche
640 × 384 contrôlés. La lecture est active à 400 ms sans erreur console.
Preuves dans `artifacts/animation-polish-01/biped-lateral-03/r04/` :
`native-validation.json`, `browser-checks.json` et `browser-proof.png`.
Le joueur a accepté la continuité visuelle de ces deux nouveaux dessins.
