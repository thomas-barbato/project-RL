# Décors usés et corps au sol — FIXES-U01-03

19 octobre 2026. Proposition artistique, sans activation dans une partie normale.

**Suite : STYLE-MAP-U01-04.** Le joueur demande de voir la carte entière pour
pouvoir juger ce style. Aucun accord n'est déduit de sa réponse. La revue r04
remplace l'entrée courante par une vue globale et neuf vues natives ; le petit
patch local ci-dessous reste l'historique de la proposition r03.

Le joueur trouve les décors trop parfaits, propres et reconnaissables comme
générés. Il précise : « Pour l'instant les décors de facon générale, on verra si
ca me plais et si ca jure trop avec les personnages auquel cas il faudra aussi
retravailler les personnages. » Les personnages vivants, armes et équipements
restent donc les repères actuels. La vue de dessus orthogonale, les cases de
64 px et la référence du camp sont conservées.

## Décisions et périmètre de la revue

- GRIM-U01-01 : le joueur apprécie le dessin des deux morts, mais trouve le
  placement arbitraire. L'apparence est conservée ; la position change et attend
  son avis, près de la défense brisée de la prison.
- GRIM-U01-02 : la station médicale de contrainte est refusée pour cette rue.
  Elle est retirée de la revue courante et du manifeste du rendu de revue.
  Sa source reste conservée comme proposition refusée pour cet emplacement.
- CORP-U01-01 : civil adulte mort au sol, tenue de travail gris olive.
- CORP-U01-02 : cultiste adulte mort au sol, capuche bordeaux et masque physique.
- CORP-U01-03 : militaire adulte mort au sol, casque et gilet mats.
- STYLE-U01-01 : essai d'entrepôt complet, tôles bleues usées, béton écaillé,
  maçonnerie salie et conduits rouillés. La source est créée comme objet entier
  transparent, pas découpée dans une scène. L'export garde les proportions de
  cette nouvelle source sur un canevas de 651 × 403 px ; la géométrie exacte
  diffère légèrement et demanderait un calage avant tout remplacement normal.
- STYLE-U01-02 : matériau de sol, quatre dalles de 64 px, gris brun terne,
  fissures et salissures irrégulières. Un échantillon de rue est assemblé avec
  ces tuiles, l'entrepôt entier et les accessoires isolés existants.

La demande de corps ne valide pas encore leurs pixels. L'accord sur le dessin
de GRIM-U01-01 ne valide ni le nouvel emplacement, ni les trois nouveaux corps,
ni les matériaux, ni l'éclairage GRIM-L02 proposé précédemment. Les deux A/B
actuels gardent exactement les mêmes sources lumineuses et la même ambiance.
Aucun texte de jeu, règle de combat, quête, collision ou butin n'est ajouté.

## Échelle et placement

Les corps isolés ont un canevas de 80 × 48 px. Leur silhouette couchée est
comparée au personnage existant sans zoom ; les pixels sources ne sont jamais
agrandis. La barricade conserve son export de 128 × 64 px.

Positions en pixels du monde, avec 64 px par case :

- GRIM-U01-01 : [256,3584], approche ouest de la défense et traces d'impacts.
- CORP-U01-01 : [192,3776], près de la clôture, hors du couloir de sortie.
- CORP-U01-02 : [576,3648], approche du dispositif de défense.
- CORP-U01-03 : [384,3520], près du point de résistance et des impacts.

Le couloir de sortie de prison n'est pas couvert. Les corps sont dessinés dans
la couche du sol, sous les acteurs vivants, avec le même éclairage et le même
masque de visibilité. Ils ne créent ici aucune nouvelle collision ni interaction.
Ces positions constituent une proposition visuelle ; elles n'inventent pas de
nouvel événement narratif destiné au joueur.

Le patch de matériaux commence en [128,192], mesure 704 × 512 px et contient
l'entrepôt ainsi que les trois accessoires entiers. Ses limites évitent la
haie voisine et le mur effondré. Une réserve transparente en bas à droite
préserve le bâtiment ruiné qui commence en [448,576], y compris son volume situé
hors du patch ; aucun objet à cheval sur le patch n'est coupé.
Les raccords de sol à ses limites sont ceux d'un échantillon de revue, pas un
remplacement final de la rue entière.

## Sources et reproduction

Outil utilisé : **imagegen intégré**, un appel par corps et matériau, avec une
réparation du cadrage de l'entrepôt pour conserver toutes ses extrémités.
Toutes les sources retenues et exports sont conservés dans :

`assets/source-art/first-sector-review-01/u01-corrections-r03/`

- `generation-prompts.json` : prompts exacts des trois corps.
- `style-prompts.json` : prompts des matériaux et correction du cadrage.
- `conditioning.json` : cadrage alpha, dimensions des corps et export technique.
- `composition.json` : assemblage des modules entiers et dimensions du patch.
- `review-layout.json` : identités, fichiers, empreintes SHA-256, dimensions et
  placements proposés. Le manifeste exclut explicitement GRIM-U01-02.

Le rendu de revue n'est chargé que dans le chemin de capture debug qui l'active.
Il n'ajoute aucune dépendance du jeu à `artifacts/`. Les sources compilées ou
chargées par le moteur restent dans `assets/`. Les ressources de production de
la carte restent inchangées.

```powershell
python tools/reviews/prepare_street_style_review.py
cargo build --locked
target/debug/project-rl.exe --ui-cold-world-fixes artifacts/street-defects-review-01/engine03
python tools/reviews/show_street_style_review.py
```

L'entrée courante est réutilisée :
`artifacts/street-defects-review-01/index.html`.

La revue est préparée conformément à AGENTS.md : « Ils doivent être validés
explicitement par le joueur. » L'autorisation porte ici sur la préparation des
candidats ; les nouveaux pixels ne sont pas considérés comme acceptés.

## Vérification

Les résultats de la capture réelle sont enregistrés dans `capture-checks.json`.
Les images A/B gardent le même éclairage, la même position du personnage et la
même caméra. Le contrôle de capture vérifie que le rendu ne modifie pas le monde,
que les cases sont de 64 px, que le PNJ corrigé est visible derrière le mur,
que le trajet 16,31 → 7,31 consomme neuf tours et que le retour au camp fonctionne.
Les contrôles de compilation et le contrôle des fichiers sont consignés dans
`validation.json` une fois terminés. Il ne s'agit pas d'une nouvelle exécution
complète de tous les tests du jeu.
