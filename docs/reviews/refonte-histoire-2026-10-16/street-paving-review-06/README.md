# PAV-U01-06 — dallage de la rue

Le 19 octobre 2026, le joueur répond « Valider et intégrer ce sol » à la revue
PAV-U01-06, révision des joints irréguliers. Le sol est intégré au lancement
normal. La finition STYLE-MAP-U01-04 et les vrais éclairages restent actifs.

Le joueur relève des traits supplémentaires sur les dalles et une touffe
d'herbe presque à chaque répétition. Le morceau de sol utilisé dans
`build_street_unified_review.py` contient un joint décalé et une touffe
intégrée ; la composition des bordures y superpose une autre grille.

## Correction validée

- Une dalle de 64 × 64 px, sans trait intérieur ni plante intégrée, déclinée
  avec des joints interrompus ou presque effacés. Quelques joints plus nets
  restent présents. La composition varie les versions et leurs orientations
  carrées selon un hachage fixe des coordonnées, hors du RNG du jeu. Elle
  n'ajoute aucun tirage ni travail de composition pendant les images du jeu.
  Le traitement d'usure validé reste appliqué par le moteur.
- Quelques touffes séparées de 12 × 10 px, en petits groupes irréguliers près
  des bordures et murs, sans modifier les collisions.
- Même plan de rue, modules entiers, maisons, toits, portes et vrais éclairages.
  La composition sud du départ et les sols intérieurs sont conservés.

La recomposition sans remplacement doit être identique pixel pour pixel au
fond actif avant de produire le candidat. Les contrôles de rendu utilisent
les mêmes neuf positions, une horloge figée et des parties isolées. Ils
vérifient que dessiner ne modifie pas l'état de jeu et que le retour au camp
fonctionne. L'export entier sert uniquement à examiner le décor ; les vues
jouables gardent la vision réelle de six cases.

## Ressources et reproduction

Sources complètes, exports natifs et candidat :
`assets/source-art/first-sector-review-01/u01-corrections-r06/`.
La dalle, ses variantes de joints et l'herbe sont produits séparément avec imagegen ; aucun bâtiment
n'est découpé ou redessiné pour cette correction. `prompts.json` conserve les
prompts exacts et les sources de génération retenues. `joint-prompts.json`
conserve les deux modifications ciblées de joints demandées pendant la revue.

Le fond normal `assets/maps/prison-street-expanded-01/street.png` reprend
exactement le composite validé. Son empreinte et l'accord sont conservés dans
`acceptance.json`. Seuls `background_sha256` et `paving_review` changent dans
le manifeste de la rue. Aucun champ de collision, de porte ou d'éclairage
n'est modifié ; les quatorze autres ressources de la carte sont identiques.

Le chargement spécial du candidat a été retiré du moteur. La rue se dessine
directement depuis ses ressources normales sous `assets/maps/`. Aucun fichier
sous `artifacts/` ou le répertoire de cette proposition n'est requis par le
jeu pour ce sol. Les variations sont précalculées dans l'image, sans travail
de composition ni aléatoire supplémentaire pendant les images du jeu.

## Vérification et reproduction

Les neuf vues natives et la carte entière du rendu normal correspondent
pixel pour pixel aux captures approuvées. Le retour au camp et le caractère
non modifiant du rendu sont vérifiés. `integration-checks.json` conserve les
mesures, les empreintes et la reconstruction du manifeste précédent prouvant
que les deux seuls champs graphiques mentionnés ci-dessus ont changé.
`integrated-native.png` conserve une vue utile du rendu normal ; les sorties
de contrôle redondantes sont supprimées après comparaison.

1. Pour reconstruire le composite source, exécuter
   `tools/reviews/build_street_paving_review.py` avec Pillow et NumPy. La source
   antérieure r06 est la référence de recomposition après intégration.
2. `cargo build --locked`.
3. `target/debug/project-rl.exe --ui-cold-street-paving-integration artifacts/street-defects-review-01/engine07`.
4. `python tools/reviews/check_street_paving_integration.py`.
5. `python tools/reviews/show_street_paving_review.py` remet l'aperçu validé à jour.

Page courante : `artifacts/street-defects-review-01/index.html`.
Captures natives : `artifacts/street-defects-review-01/engine06/`.
Les comparaisons validées précédentes restent conservées dans engine04/05.
`checks.json` conserve les contrôles de la revue avant intégration et les
mesures de différences entre l'ancien sol et le sol accepté.
