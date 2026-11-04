# MONSTER-VERTICAL07-r01 — trois gabarits compacts

Le 3 novembre 2026, « ok je valide et on continue :) » accepte le lot 16–18
de MONSTER-VERTICAL06-r01 et poursuit la reprise des marches haut/bas.
L’accord, les treize empreintes et la palette de MON-025 sont conservés dans
`MONSTER-VERTICAL06-ACCEPTANCE-r01.json`.

Ce septième lot propose les variantes 0 de :

19. MON-026-0 : possédé à la tunique ocre, court et voûté, aux longs avant-bras.
20. MON-028-0 : démon massif aux longs bras, peau olive et ventre plissé.
21. MON-032-0 : petit démon à crête courbe, dos sombre et ventre clair.

Ces noms servent à la revue. Aucun rôle, objet, rencontre, dégât, capacité ou
statut de ressource ne change. MON-026 et MON-028 gardent deux jambes
porteuses ; les longs bras du démon ne deviennent pas des appuis. La première
proposition avait aussi classé MON-032 comme bipède : le joueur corrige cette
erreur et retire son accord pour cette locomotion. Le n°21 est repris sur
quatre appuis dans `MONSTER-VERTICAL21-QUAD01.md`. La tête
courbe de MON-032 reste attachée au buste et se replie vers l’avant ; son
ventre et ses yeux sont masqués en vue arrière. Les dimensions propres aux
trois dessins et leurs proportions basses sont conservées.

Huit figures entières par direction, deux appuis alternés à 400 ms par pas
et 800 ms par cycle. Les membres sont dessinés avec le corps, sans bandes
de jambes décalées, greffes ni tête flottante. Une échelle uniforme par
direction et une translation commune du cycle alignent l’appui moyen.
Le repos conserve exactement la cellule 12 du VERT18 installé, son ancrage
et son ombre réels. Les dix-huit sélections précédentes restent disponibles.

Sources, frontaux et atlas :
`assets/source-art/animation-polish-01/monster-vertical-07/r01/manifest.json`.
Les planches sont dessinées avec l’outil intégré image_gen, en mode édition
avec conservation d’identité. Prompts et références exacts dans
`MONSTER-VERTICAL07-PROMPTS.json`. Les références de mouvement ne servent
pas à remplacer l’identité, les proportions ni l’équipement du modèle.

La première comparaison native a montré les dessins de MON-026 et MON-032
trop étroits : environ 37 contre 45 px pour le possédé, et 27 contre 39 px
pour le petit démon, à hauteur conservée. Leurs figures entières sont reprises
avec les proportions ramassées du frontal, sans étirement de pixels ni
écartement accru des jambes. Ces reprises sont décrites dans
`MONSTER-VERTICAL07-HYBRID-PROMPTS.json` et
`MONSTER-VERTICAL07-COMPACT-PROMPTS.json`. Les premiers dessins sont conservés
comme sources initiales et ne sont pas sélectionnés pour la revue finale.
Le possédé garde le gros avant-bras sombre à droite anatomique, et le plus
petit avant-bras nu à gauche. Le dos du petit démon montre le crâne arrière
fermé plutôt que son œil frontal.

Le diagnostic `MONSTER-VERTICAL07-COLORS.json` décrit séparément les matériaux
visibles de face et de dos. Il ne modifie aucun pixel et ne confond pas les
différences de surface visible avec un changement de lumière. Aucun nouveau
filtre de palette n’est ajouté à ce lot.

La préparation vérifie les 117 empreintes antérieures : 38 latérales, 12 du
premier lot vertical, 12 du deuxième accepté, quatre du rouge harmonisé,
13 du troisième, 13 du quatrième, 12 du cinquième et 13 du sixième acceptés.
Les dix atlas latéraux installés restent inchangés. Les ressources sont
préparées avant les captures, avec création des textures dans le thread
du contexte graphique, sans lecture lourde dans le dessin.

Statut : n°19–20 acceptés ; validation du n°21 retirée par le retour suivant
« je valide sauf le 21 ». Les huit ressources acceptées et les quatre fichiers
de l’essai rejeté sont distingués dans `MONSTER-VERTICAL07-ACCEPTANCE-r01.json`.
La décision initiale et son amendement sont conservés dans
`MONSTER-VERTICAL07-AMENDMENT-r02.json`. Aucune activation dans la partie normale.
Vingt et un modèles proposés au total, 108 autres à reprendre, puis les
variantes restantes à contrôler. Les captures ne chargent aucune partie
ni préférence de l’utilisateur.

```powershell
python tools/art/prepare_monster_vertical07.py --references
python tools/art/prepare_monster_vertical07.py
python tools/art/audit_monster_vertical07_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical07 artifacts/animation-polish-01/monster-vertical-07/r01/current
python tools/art/build_monster_vertical07_review.py
python tools/art/check_monster_vertical07.py
```

Vérification finale du 3 novembre 2026 : compilation verrouillée et quatre
tests de palette existants réussis (MON-144, MON-151, MON-006 et MON-025).
Les 198 captures natives présentent 48 figures mobiles distinctes et six
retours au frontal exact, sous les mêmes décor et lumière. Les 117 empreintes
antérieures et les dix atlas latéraux installés sont conservés. Le contrôle
de largeur des silhouettes tient compte d’un pixel de quantification ; il
complète la revue visuelle, sans la remplacer. Syntaxe Python, formatage
Rust ciblé et `git diff --check` vérifiés.

Dans l’onglet courant, les vingt et une sélections chargent chacune 66
captures et l’inventaire conserve ses 129 fiches. Les six arrêts des trois
nouveaux modèles, le bouton de pose suivante et l’aller-retour automatique
à 400 ms par pas ont été exercés. Le comparatif reste à 576 × 320 pixels
natifs, sans redimensionnement CSS. Aucune erreur console après l’ouverture
finale. Bilan : `MONSTER-VERTICAL07-VALIDATION.json`.

Provenance des dix générations et empreintes :
`MONSTER-VERTICAL07-GENERATIONS.json`. Les entrées initiales de MON-026 et
MON-032 pointent sur les copies conservées ; les reprises sélectionnées
pointent sur les sources finales. Les captures de navigateur, preuves
natives et bilan d’interaction sont sous
`artifacts/animation-polish-01/monster-vertical-07/r01/` :
`browser-proof.png`, `browser-back-proof.png`, `native-proof.png` et
`browser-checks.json`. Elles ne servent pas de ressources à la partie normale.
