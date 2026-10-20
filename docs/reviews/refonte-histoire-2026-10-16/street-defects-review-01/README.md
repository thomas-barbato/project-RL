# Reprise après essai — DEFECTS-U01-01

19 octobre 2026. Le joueur signale un cercle invisible, une ambiance trop sage,
des collisions dans la rue et des portes qui semblent ouvertes puis apparaissent
en clignotant à l'approche. Les points ci-dessous ont des statuts distincts.

## Porte fermée qui apparaît à l'approche — correction du comportement accepté

La fermeture logique était correcte. Le panneau était pourtant dessiné seulement
si la case était dans la ligne de vue réelle du joueur. La façade, elle, utilisait
le cercle indépendant des obstacles : une porte fermée pouvait donc montrer le
fond d'une ouverture, puis faire apparaître son panneau en franchissant un angle.

La condition suit désormais la présentation du décor. L'état de la porte reste
celui du moteur ; son panneau n'utilise plus la visibilité réservée aux acteurs.
Ce correctif est actif dans le lancement normal. Il ne change ni le dessin, ni
l'interaction, ni le tour consommé, ni les sauvegardes. La revue compare les deux
conditions de rendu dans le même état, avec une porte fermée hors de la ligne de
vue des acteurs. Les toits suivent toujours la porte.

## Cercle hors du cadre — VISION-U01-05, accepté le 19 octobre

Réponse explicite du joueur : « 6 cases c'est bien ». La portée réelle de six
cases est intégrée en génération 158. Le décor hors du cercle est masqué et la
mémoire de la mini-carte reste conservée. La couronne douce comprend les bords des
cases perçues : elle ne doit pas rendre noirs les ennemis dont le centre est
exactement à six cases. Elle n'autorise pas la détection d'acteurs plus éloignés.

Les parties de l'agent déjà commencées (153–157) sont d'abord restaurées et
vérifiées avec leurs règles historiques. L'installation enregistre ensuite une
commande interne `UpgradePlayerVision`, sans tour, déplacement, tirage aléatoire
ni modification d'équipement. Le journal et le cache de récupération savent
recharger ce changement. Les parties antérieures à l'agent conservent leurs règles.
La vision propre aux monstres reste inchangée. Cette validation ne concerne ni
les 61 passages proposés, ni l'éclairage sombre, ni le nouveau décor de quarantaine.

### Diagnostic et comparaisons initiaux, avant adoption

La règle précédente utilisait un rayon de 12 cases (pas 8). À 64 px par case, le rayon
visuel extérieur est d'environ 723 px après la marge du shader. Dans le cadrage
1920 × 1080 contrôlé, la zone de jeu mesure environ 1218 × 704 px : son coin est
à 704 px du centre. Le cercle contient donc presque tout le cadre et sa limite
n'est pas visible. Les captures précédentes ne vérifiaient pas ce critère.

Deux comparaisons proposent 8 ou 6 cases. À 6, le cercle extérieur mesure environ
339 px de rayon et tient dans la zone de jeu. L'échelle des cases et des acteurs
reste inchangée. Le décor hors du cercle est masqué dans ces deux propositions,
au lieu de rester à 45 % d'intensité ; la mémoire de la mini-carte est conservée.
Les aperçus ne modifient que le masque de présentation ; les
règles de perception réelles restent à 12 pendant ces captures. Choisir une
nouvelle portée réelle doit être validé séparément avant intégration : cela
change la distance à laquelle le joueur voit les ennemis. Les règles de vue
propres aux monstres restent indépendantes.

## Passages de rue — COL-U01-01, proposition

Le générateur r06 convertissait un chevauchement opaque de seulement 96 pixels
sur les 4096 d'une case en mur complet. Des petites bordures de façade et des
ombres d'export condamnaient ainsi des espaces de contact dessinés libres.

La proposition explicite rouvre 61 cases de rue. Elle conserve les caisses,
l'armoire et le fût devant l'entrepôt ainsi que les gros objets et les limites
de carte. Une première liste de 62 cases avait une poche isolée en `[22,41]` ;
elle a été écartée après contrôle de raccordement. Aucun passage intérieur n'est
ajouté par cette liste. Les cases proposées figurent dans
`assets/source-art/first-sector-review-01/u01-corrections-r01/collision-proposal.json`.

L'analyse de silhouette sert au diagnostic, pas à un nouvel algorithme permanent
de collision. La liste est un relevé explicite à examiner sur le décor. Les tests
comparent toutes les cases, gardent les vrais obstacles bloqués et vérifient que
chaque passage proposé est relié. Le mode de revue applique cette liste seulement
avec `--ui-cold-street-defects`. Le lancement normal et les parties 157 ne sont
pas remappés silencieusement. L'intégration doit préserver leur contrat de reprise.

## Ambiance et violence — GRIM-U01-01, proposition

L'ambiance ne repose pas seulement sur un fond plus sombre. La première rue doit
montrer les conséquences de l'assaut : quarantaine, défenses militaires rompues,
victimes, matériel abandonné et quelques signes du culte. La corruption organique
peut gagner en importance en descendant, conformément à la progression de l'univers.

Un essai d'éclairage utilise une ambiance RGB `[0.28, 0.30, 0.36]` en conservant les
sources lumineuses réelles et les couleurs fonctionnelles des portes. C'est une
comparaison de rendu ; ce réglage ne change pas le camp.

**GRIM-U01-01** est une barricade de quarantaine détruite avec deux victimes adultes,
visages masqués et sang. Elle est dessinée entière et isolée, avec transparence,
dans le style et la projection existants. La source est préservée ; l'export proposé
mesure 128 × 64 px. La planche la compare au joueur sans zoom. Le skill imagegen
et l'outil intégré ont servi à sa création ; le prompt exact et les références
sont dans `imagegen-prompt.json`. Aucun décor n'est découpé dans une illustration
de rue. Ce candidat n'est pas placé dans le jeu.

## Portée de la revue

La portée de six cases est validée et intégrée. Les collisions proposées,
l'éclairage plus sombre et le nouveau dessin restent en attente d'accord,
conformément à AGENTS.md : « Ils doivent être validés explicitement par le joueur. »
Le correctif des portes restaure leur comportement visuel déjà accepté.

```powershell
target/debug/project-rl.exe --ui-cold-street-defects artifacts/street-defects-review-01/engine
python tools/reviews/show_street_defects_review.py
```

## Vérifications du 19 octobre 2026

- `cargo test --locked --bin project-rl prison_street -- --nocapture` : 12 tests
  réussis. La liste de 61 cases est contrôlée case par case, chaque case proposée
  est accessible et les vrais objets solides restent bloqués.
- `cargo fmt --all -- --check`, `cargo check --locked --release` et
  `git diff --check` : réussis ; compilation debug réussie.
- Capture GPU terminée avec succès : 9 scènes, chacune dans les 4 variantes,
  cellules de 64 px et empreinte de simulation conservées, retour réel au camp.
- Régression des portes reproduite dans un état identique : porte fermée hors de
  la visibilité des acteurs, panneau absent avec l'ancienne condition et présent
  avec le correctif. La comparaison contient 8 169 pixels modifiés.
- Planche des portes, passage devant l'entrepôt, cercle sombre de la rue centrale
  et décor GRIM-U01-01 examinés visuellement à leur taille native. Le cercle de
  6 cases est visible dans le cadre. Les nouveaux réglages restent des candidats.

Ces contrôles ciblent cette correction et cette revue ; ils ne constituent pas
une exécution de toute la suite du jeu ni une validation artistique par le joueur.

## Vérifications après adoption des six cases

- 4 tests `player_vision` réussis : nouvelle partie, règles historiques,
  sauvegardes 153 et 157 vérifiées avant adoption, combinaison avec la migration
  des mains, cache de récupération et rejeu après adoption, idempotence et refus
  d'un état altéré. Les fichiers de l'utilisateur ne sont pas utilisés par les tests.
- 1 test moteur `six_cell_vision` réussi : acteur à six cases visible, acteur à
  sept cases caché, exploration antérieure conservée, aucun tour ni tirage aléatoire,
  acteurs et événements inchangés.
- Les 12 tests de la rue réussissent avec la génération 158. L'assertion du test
  de la carte intégrée suit la génération courante ; les tests des cartes
  historiques 155/156 conservent leurs attentes propres.
- Compilation debug, contrôle release, formatage et `git diff --check` réussis.
- 6 captures GPU du lancement normal avec `--vision-six`, sans rayon de revue ni
  nouvelle ambiance. Le rapport confirme rayon réel 6, génération 158, cellules
  de 64 px, rendu sans mutation du monde et aller-retour au camp. La capture de
  la rue centrale a été examinée à taille native.

```powershell
target/debug/project-rl.exe --ui-cold-street-integration artifacts/street-defects-review-01/vision-six-integration --vision-six
python tools/reviews/show_street_defects_review.py
```
