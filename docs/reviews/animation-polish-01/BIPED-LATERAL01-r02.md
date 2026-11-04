# BIPED-LATERAL01-r02 — reprise du démon ailé uniquement

Le premier lot est rejeté. Le joueur précise les défauts : « 1 et 3 », soit
des jambes qui pédalent ou montent trop et un corps animé artificiellement.
L'accord sur la direction des trois classes CLASS-LATERAL02-r02 est conservé ;
il ne vaut pas acceptation des essais de monstres ou de port du fusil.

Cette proposition reprend seulement MON-144-0, palette d'origine, sans arme :
petits pas rasants, faible flexion du genou et une silhouette supérieure plus
stable. Les deux directions sont dessinées dans deux sources 3×3 distinctes,
avec huit poses entières de marche et un repos latéral. Un premier essai 9×2
est écarté pour sa variation des ailes. Les sources finales et les prompts
sont conservés sous les mêmes identifiants r02.

Le conditionnement extrait et réduit chaque figure entière. L'échelle est
fixée une fois par direction avec le repos. La position de la couronne et
le centre de la partie supérieure fixent l'ancrage : les variations des pieds
ne déplacent plus toute la silhouette pour recaler le pied le plus bas.
Aucun membre n'est greffé, déformé ou déplacé indépendamment.

Le trajet de comparaison est raccourci de 40 à 12 px par cycle pour accompagner
la faible amplitude. La cadence reste 650 ms par pas, 1 300 ms par cycle.
Ces distances et la pause de revue ne changent pas les commandes ou règles
du jeu. La colonne précédente réaffiche r01 refusé au même petit trajet et
à la même cadence pour comparer les poses. Le frontal d'arrivée reste celui
du rendu normal. L'exposition du monstre et le décor sont identiques entre
les colonnes, sans reprise du réglage de lumière réservé au joueur.

L'aperçu actif propose uniquement ce candidat, les deux côtés, le vrai arrêt,
une comparaison avec r01 refusé ou le frontal actuel, et un zoom entier.
Les captures de 480×192 px sont présentées par défaut en deux colonnes natives
de 160 px (320×192), adaptées à un panneau étroit sans réduction des acteurs.
Le mode trois colonnes conserve les 480 px, le zoom est un multiple entier.
Les 66 captures sont préparées avant lecture dans un cache borné.

État : nouvelle proposition, non intégrée et non validée visuellement. Les
autres monstres du lot restent refusés ; l'essai de port du fusil reste non
validé. Les suites attendent ce contrôle de la démarche, selon la procédure
de revue de marche. Le bestiaire entier, les armes et les palettes ne sont
pas déclarés corrigés.

Reproduction : `--ui-cold-biped-lateral-r02 OUTPUT`, puis
`tools/art/build_biped_low02_review.py`. Les sources durables sont dans
`assets/source-art/animation-polish-01/biped-lateral-01/r02/`, les captures et
preuves sous `artifacts/animation-polish-01/biped-lateral-01/r02/`.
