# MONSTER-VERTICAL13-r01 — six pattes, quatre appuis et huit pattes

Le 3 novembre 2026, « oui ok je valide et on contnue :) » accepte les n°34–36
et poursuit la reprise haut/bas avec les n°37–39, trois modèles existants en
variante 0. Les nouveaux cycles restent des candidats avant intégration.

- 37 / MON-069-0 : petit corps rouge et sombre, six pattes en trois paires,
  articulations claires, pointes noires et dos segmenté. Appuis alternés en
  deux trépieds ; chaque patte garde son segment d’origine.
- 38 / MON-070-0 : thorax et tête effilés olive/ocre, centre segmenté rosé,
  quatre pattes anguleuses porteuses et deux avant-bras recourbés distincts.
  Les faux sont des bras, sans fonction porteuse. Le thorax reste dressé,
  les pieds avancent bas et les bras ne font que suivre doucement le corps.
- 39 / MON-071-0 : arachnide rouge et anthracite, gros abdomen, petite tête,
  huit pattes en quatre paires et extrémités ocre clair. Garder les paires
  distinctes pendant les appuis alternés ; les mandibules ne sont pas des
  pattes supplémentaires.

Les références natives de face et de dos sont extraites des sprites
installés, puis comparées aux fiches anatomiques existantes. Les deux vues
du même monstre servent aux dessins, sans copie d’une autre espèce. Six
planches initiales ont été produites avec image_gen intégré en mode édition.
Le premier dos du n°38 était encore trop frontal : il a été redessiné de dos,
avec la pointe de tête courbée vers la droite de l’image comme sa référence.
Les deux planches du n°39 ont été reprises pour montrer une quatrième paire
de pattes distincte. Les trois premiers essais restent archivés sans être
sélectionnés. Prompts et dix-huit empreintes d’entrée :
`MONSTER-VERTICAL13-PROMPTS.json` et `MONSTER-VERTICAL13-ANATOMY-PROMPTS.json`.
Les neuf sorties, dont six sources finales sélectionnées :
`MONSTER-VERTICAL13-GENERATIONS.json`.

Chaque direction propose huit figures entières nouvelles à 400 ms par
avancée, 800 ms par cycle. Les corps et les appendices restent attachés,
avec une hauteur portée stable, des passages bas et des appuis chargés.
Pas de membre greffé, bande de pattes déplacée ou cisaillement. Une seule
échelle par planche et une translation du cycle entier calent les appuis
sur le repos. Le vrai frontal installé est conservé exactement dans les
deux cellules d’arrivée ; le neuvième dessin généré ne le remplace pas.

Trois familles de matériaux sont préparées séparément : chitine rouge et
plis, articulations claires/olive, carapace neutre. Les courbes RGB sont
fixes pour les huit poses d’une direction et prennent la vraie référence
installée de cette vue. La carapace sombre n’est pas mélangée aux pointes
dorées. Le calcul des rangs des petites plages colorées utilise le même
arrondi que le contrôle Rust ; les règles précédemment acceptées conservent
leur préparation historique. Alpha, contours les plus sombres, géométrie
et repos restent intacts. Aucun gain lumineux variant selon la phase,
aucune modification des PNG sources. Bilan :
`MONSTER-VERTICAL13-MATERIALS.json`.

Atlas et palettes sont lus avant création des textures sur le thread
graphique ; aucune lecture lourde dans le dessin. Les comparatifs montrent
le même décor, des cases de 64 px et un gain d’acteur de 1,48. Les neuf tests
Rust de palette contrôlent les vrais pixels, les alphas, les repos et les
rangs des matériaux des lots concernés. La revue du joueur reste nécessaire
pour l’anatomie et la qualité du mouvement à taille native.

Sources et manifeste :
`assets/source-art/animation-polish-01/monster-vertical-13/r01/`.
Les 36 modèles précédents restent disponibles, dont les n°34–36 validés et
le n°28 avec sa lame acceptée. Les 212 ressources figées et les dix atlas
latéraux installés sont conservés. Les identifiants, variantes, règles et
équipements mécaniques ne changent pas. Aucune partie ni préférence n’est
chargée. Aucun nouveau cycle n’est activé en partie normale. Après ce lot,
39 modèles sont proposés et 90 autres restent à reprendre ; les variantes
gardent leur propre revue.

```powershell
python tools/art/prepare_monster_vertical13.py --references
python tools/art/prepare_monster_vertical13.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical13 artifacts/animation-polish-01/monster-vertical-13/r01/current
python tools/art/build_monster_vertical13_review.py
python tools/art/check_monster_vertical13.py
```

Statut : n°37–39 validés le 3 novembre 2026 (« ok je valide tu peux continuer »).
Les dix-huit ressources finales sont figées dans
`MONSTER-VERTICAL13-ACCEPTANCE-r01.json`. Accord visuel avant intégration.

Vérification finale : compilation verrouillée et neuf tests Rust de palette
réussis, dont le contrôle des neuf modèles des lots 11–13 sur leurs vrais
PNG. Le premier contrôle avait détecté un écart de rang médian sur une
petite plage claire du n°37 : la préparation du lot 13 utilise désormais
l’arrondi du moteur pour ces rangs, et les neuf tests passent sur les sources
finales. Les huit règles précédentes et leurs ressources restent conservées.

Le contrôle natif valide 48 poses distinctes, six cellules de repos exactes,
198 captures, trois animations de 2 400 ms et 129 cartes d’inventaire.
Les 212 ressources figées restent intactes, dont les dix-huit du lot 12 ;
les dix atlas latéraux installés sont inchangés. Les dix-huit références de
génération et les neuf sorties, dont trois essais non sélectionnés, sont
vérifiées par empreinte. Syntaxe Python, format Rust et `git diff --check`
réussis.

Le lecteur charge les 39 sélections avec 66 captures chacune. Les six arrêts
manuels affichent le frontal conservé ; les trois nouveaux modèles passent
automatiquement de bas à haut à 400 ms. Les n°34–36 restent liés aux sources
validées du lot 12 et le n°28 à sa lame acceptée. Aucune erreur JavaScript.
Le canevas de 576 × 320 px s’affiche sans réduction dans une fenêtre de
654 × 884 px. Les preuves et le bilan du lecteur sont dans
`artifacts/animation-polish-01/monster-vertical-13/r01/` : `native-proof.png`,
`browser-proof.png`, les trois fichiers `browser-proof-37/38/39.png` et
`browser-checks.json`. Bilan durable : `MONSTER-VERTICAL13-VALIDATION.json`.
