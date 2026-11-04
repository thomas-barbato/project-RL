# MONSTER-VERTICAL12-r01 — bras ailés, rampant et marche lourde

Le 3 novembre 2026, « je valide et on continue :) » accepte les n°31–33 et
poursuit la reprise des marches haut/bas avec trois autres modèles existants,
en variante 0. Ce lot prépare les n°34–36 pour une revue native séparée.

- 34 / MON-066-0 : deux pieds porteurs et deux bras ailés. Les mains se
  trouvent aux articulations hautes des ailes ; ne pas ajouter une seconde
  paire de bras libres. Conserver les membranes rouges basses, leur largeur,
  les montants clairs, le petit buste et les cornes. Marche au sol sans envol
  ni battement d’ailes.
- 35 / MON-067-0 : petit rampant très bas sur quatre courtes pattes, corps
  dorsal sombre et effilé, articulations claires et bouche ronde rose/rouille
  ouverte devant. Ne pas dessiner de bouche sur le dos, ni inventer de tête,
  de queue allongée ou de pattes supplémentaires.
- 36 / MON-068-0 : grand bipède massif orange/brun, tête et dos à carapace
  sombre recourbée, épaules larges et longues mains griffues noires. Garder
  les bras relâchés près du torse, les coudes abaissés et les pas courts.
  Les griffes sont des doigts, sans objet tenu ajouté.

Les références de face et de dos viennent des sprites installés, contrôlés
visuellement avec les anciennes fiches anatomiques. MON-061 accepté sert
seulement de guide d’appuis bipèdes ; MON-064 accepté sert de guide de contacts
quadrupèdes. Leur identité, leurs équipements et leurs couleurs ne sont pas
transférés. Six planches dessinées avec image_gen intégré en mode édition,
une par modèle et direction, avec fond alpha transparent. Le premier démon
massif était trop étroit, surtout de dos : deux planches ont été reprises
pour retrouver la largeur du modèle sans relever ses bras ni écarter ses
jambes. Les deux essais étroits restent archivés, sans être sélectionnés.
Prompts et seize empreintes d’entrée : `MONSTER-VERTICAL12-PROMPTS.json`
et `MONSTER-VERTICAL12-BREADTH-PROMPTS.json`. Les huit sorties, dont six
sources sélectionnées : `MONSTER-VERTICAL12-GENERATIONS.json`.

Après reprise, la largeur médiane du n°36 atteint 98,6 % de sa référence
de face et 92,1 % de sa référence de dos, à hauteur inchangée. La portée
des griffes varie avec les bras abaissés ; la revue native compare aussi
les épaules, le torse, les cuisses et les appuis. Le contrôle borne ce
gabarit à dix pour cent de la vue réelle, avec la tolérance du pixel entier.

Chaque direction comporte huit figures entières nouvelles à 400 ms par
avancée, soit 800 ms par cycle. Les appuis alternent avec passage bas et
prise de charge ; le rampant garde ses pattes porteuses et son corps près
du sol. Pas de membre greffé, bande de jambes déplacée ou cisaillement.
Les têtes suivent les torses ; ailes et carapaces gardent leurs attaches.
Une échelle uniforme par planche et une seule translation du cycle entier
calent les appuis, sans agrandissement variant selon la phase. Les deux
cellules d’arrivée conservent exactement le frontal installé ; le neuvième
dessin généré ne le remplace pas.

Des courbes RGB fixes par matériau et direction harmonisent les huit poses
avec la vraie référence installée de cette vue. Les matériaux frontaux ne
sont pas reportés sur le dos. Conserver alpha, géométrie, contours sombres
et repos. Aucun gain lumineux variable par phase ni modification des PNG
sources. Préparation des atlas et des palettes avant création des textures
sur le thread graphique ; aucune lecture lourde dans le dessin. Même décor,
cases de 64 px et gain d’acteur 1,48 pour les trois colonnes comparées.
Bilan des matériaux : `MONSTER-VERTICAL12-MATERIALS.json`.

Sources de dessin et manifeste :
`assets/source-art/animation-polish-01/monster-vertical-12/r01/`.
Les trente-trois modèles précédents restent sélectionnables, avec une seule
entrée pour chaque modèle. Les n°31–33 conservent leur validation et le n°28
sa lame acceptée. Les identifiants, règles, variantes, équipements mécaniques,
ressources déjà figées et dix marches latérales intégrées restent conservés.
Aucune partie ni préférence n’est chargée. Aucun nouveau cycle n’est activé
en partie normale. Après ce lot, 36 modèles sont proposés et 93 autres
restent à reprendre ; les variantes conservent leur propre revue.

```powershell
python tools/art/prepare_monster_vertical12.py --references
python tools/art/prepare_monster_vertical12.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo build --locked --bin project-rl
.\target\debug\project-rl.exe --ui-cold-monster-vertical12 artifacts/animation-polish-01/monster-vertical-12/r01/current
python tools/art/build_monster_vertical12_review.py
python tools/art/check_monster_vertical12.py
```

Statut : n°34–36 validés le 3 novembre 2026 par « oui ok je valide et on
contnue :) », dans les deux directions à 400 ms par avancée. Les dix-huit
ressources finales sont figées dans `MONSTER-VERTICAL12-ACCEPTANCE-r01.json`.
Les deux premiers essais étroits du n°36 restent non sélectionnés. Cet
accord visuel n’active pas les cycles en partie normale.

Vérification finale : compilation verrouillée et neuf tests Rust de palette
réussis, dont le contrôle des six modèles organiques des lots 11 et 12 sur
leurs vrais PNG. Les 48 poses sont distinctes ; les six repos sont identiques
aux frontaux installés. 198 captures, trois animations de 2 400 ms et 129
cartes natives ont été contrôlées. Les 194 ressources figées restent intactes,
dont les dix-huit ressources du lot 11 et les cinq du n°28 armé. Les dix atlas
latéraux installés restent inchangés. Syntaxe Python, format Rust et
`git diff --check` réussis.

Le lecteur charge les 36 sélections avec 66 captures chacune. Les six arrêts
manuels affichent le vrai repos frontal ; les trois nouveaux modèles passent
automatiquement de bas à haut à 400 ms. Le n°28 reste lié à sa lame acceptée
et les n°31–33 aux sources validées du lot 11. Aucune erreur JavaScript.
Le canevas de 576 × 320 px s’affiche sans réduction dans une fenêtre de
654 × 884 px. Preuves et contrôle du lecteur :
`artifacts/animation-polish-01/monster-vertical-12/r01/` contient
`native-proof.png`, `browser-proof.png`, les trois `browser-proof-34/35/36.png`
et `browser-checks.json`. Bilan durable : `MONSTER-VERTICAL12-VALIDATION.json`.
