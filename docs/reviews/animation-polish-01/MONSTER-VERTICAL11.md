# MONSTER-VERTICAL11-r01 — quatre appuis et corps enroulé

Le 3 novembre 2026, « ok on peut continuer :) » poursuit la reprise après
la validation du n°28 armé. Ce lot propose trois modèles existants en
variante 0 ; il ne les active pas dans la partie normale.

- 31 / MON-063-0 : quadrupède rouge à queue, quatre pattes griffues porteuses,
  dos voûté, plages claires aux épaules et cuisses. Conserver la queue
  courbée à droite de l’image de face, à gauche de dos.
- 32 / MON-064-0 : quadrupède pâle et trapu, quatre membres fléchis, très
  bas sur le sol, dos bombé et tête basse. La plaque rouge unique reste à
  droite de face et à gauche de dos, sur le même côté anatomique.
- 33 / MON-065-0 : créature brune au corps serpentin enroulé, sans jambes,
  avec deux bras griffus libres. La propulsion vient des spires en contact
  avec le sol. Garder le ventre clair devant et les crêtes sombres derrière.

Chaque direction comporte huit figures entières nouvelles : appuis et
passages courts pour les quadrupèdes, onde continue de la base pour le
serpentin. Aucun membre greffé, bande de jambes déplacée ou cisaillement.
Les corps restent bas, les têtes suivent les torses et les contacts au sol
portent la figure. Cadence retenue : 400 ms par avancée, 800 ms par cycle.
L’arrivée affiche le véritable frontal installé, conservé exactement dans
les deux cellules de repos. Le neuvième dessin généré ne le remplace pas.

Six planches ont été produites avec image_gen intégré en mode édition,
sur fond alpha transparent. Les références de face et de dos proviennent
des sprites installés. Pour les quadrupèdes, MON-037 accepté sert seulement
de guide de contacts, sans reprendre son identité. Pour le serpentin,
l’autre vue du même modèle précise les matériaux et l’anatomie.
Prompts et douze empreintes d’entrée : `MONSTER-VERTICAL11-PROMPTS.json`.
Sorties originales et six sources sélectionnées :
`MONSTER-VERTICAL11-GENERATIONS.json`. Sources de dessin et manifeste :
`assets/source-art/animation-polish-01/monster-vertical-11/r01/`.

Une échelle uniforme est appliquée à chaque planche et une seule translation
de la figure entière centre le cycle sur les appuis du repos. L’échelle ne
varie pas selon la phase. Les PNG sources restent intacts. Des courbes RGB
fixes par matériau et direction harmonisent les images avec leur référence
installée correspondante. Le dos utilise son vrai dos comme référence,
pour ne pas y reporter la matière ventrale claire. Les huit poses partagent
les mêmes courbes ; aucune modulation lumineuse par phase. Les alphas, les
contours sombres et les repos sont préservés. Bilan des matériaux :
`MONSTER-VERTICAL11-MATERIALS.json`.

Atlas et palettes sont lus et préparés avant la création des textures sur
le thread graphique ; aucune lecture lourde dans le dessin. Les comparatifs
natifs conservent le même décor, les cases de 64 px et le gain d’acteur 1,48.
Le test Rust vérifie les vrais pixels, les alphas, les repos et les quantiles
des matériaux de chaque direction. La validation visuelle de l’anatomie et
du mouvement reste celle du joueur.

Les trente modèles précédents restent sélectionnables, avec une seule
entrée du n°28 utilisant la lame, les bras et les couleurs acceptés. Les
ressources figées, les dix atlas latéraux installés, les identifiants,
variantes, règles et équipements mécaniques restent conservés. Aucune
partie ni préférence du joueur n’est chargée. Après ce lot, 33 modèles sont
proposés et 96 autres restent à reprendre ; leurs variantes gardent leur
revue distincte.

```powershell
python tools/art/prepare_monster_vertical11.py --references
python tools/art/prepare_monster_vertical11.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical11 artifacts/animation-polish-01/monster-vertical-11/r01/current
python tools/art/build_monster_vertical11_review.py
python tools/art/check_monster_vertical11.py
```

Statut : n°31–33 validés le 3 novembre 2026 par « je valide et on continue :) »,
dans les deux directions à 400 ms par avancée. Les dix-huit ressources finales
(atlas, frontaux, planches sources, palettes et guides dorsaux de couleur)
sont figées dans `MONSTER-VERTICAL11-ACCEPTANCE-r01.json`.
Cet accord visuel n’active pas les cycles en partie normale.

Vérification finale : neuf tests Rust de palette et compilation verrouillée
réussis. Contrôles natifs : 48 poses distinctes, six cellules de repos exactes,
198 captures, trois animations de 2 400 ms et 129 cartes d’inventaire.
Les 176 ressources antérieures protégées sont conservées, dont les cinq
ressources du n°28 corrigé et les huit ressources acceptées des n°29–30.
Les douze références de génération et les six planches retenues sont
vérifiées par empreinte. Syntaxe Python, format Rust et `git diff --check`
réussis.

Le lecteur charge les 33 sélections avec 66 captures chacune. Les six arrêts
manuels affichent le repos frontal ; les trois nouveaux modèles passent
automatiquement de bas à haut à 400 ms. Les libellés décrivent les appuis
quadrupèdes ou les spires, avec un libellé distinct pour le repos. Aucune
erreur JavaScript. Le n°28 pointe toujours vers sa version armée acceptée.
Le canevas de 576 × 320 px est affiché sans réduction, dans une fenêtre de
654 × 884 px. Les preuves et le contrôle du lecteur sont dans
`artifacts/animation-polish-01/monster-vertical-11/r01/` : `native-proof.png`,
`browser-proof.png`, `browser-proof-32.png`, `browser-proof-33.png` et
`browser-checks.json`. Bilan durable : `MONSTER-VERTICAL11-VALIDATION.json`.
