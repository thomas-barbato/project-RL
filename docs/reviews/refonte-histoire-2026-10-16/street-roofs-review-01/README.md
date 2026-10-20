# ROOF-U01-01 — toits des bâtiments

Revue du 18 octobre 2026, acceptée le 19 octobre 2026.

Réponse explicite : « Valider les toits et ce fonctionnement ». Les pixels et
les trois conditions de masquage sont figés dans `acceptance-r01.json`.
Intégré dans les nouvelles parties de génération 156.

Le joueur demande de conserver la pénombre et les vraies lumières de la rue,
puis de dessiner les toits des maisons, y compris celles que l'on peut visiter.
Il propose de masquer le toit à l'ouverture de la porte plutôt qu'après l'entrée.

## Comportement accepté

- Dehors, la maison garde son toit et sa porte reste identifiable.
- L'ouverture de la porte dévoile l'intérieur.
- Le toit reste masqué tant que la porte est ouverte ou que le joueur est dedans.
- Fermer la porte derrière soi ne masque donc pas le personnage à l'intérieur.
- Dehors, après fermeture de la porte, le toit est de nouveau visible.
- Un bâtiment fermé non visitable garde son toit ; un toit effondré conserve ses dégâts.

Ce comportement réutilise les portes existantes du jeu. Il ne définit pas de
verrouillage, de clé ou de délai supplémentaire. Les toits sont des couches de
dessin ; ils ne remplacent ni les murs, ni les collisions, ni la visibilité.

## Éclairage

SCENE-U01-07 / LUM-U01-07 et STYLE-RL-2026-10-18-r01 restent la référence.
La capture normale `lighting-recheck-r08` confirme une ambiance active dans le
moteur, des cases de 64 px, un rendu sans mutation et le retour au camp.
L'image source brute ne présente pas cette passe GPU et ne doit pas servir
d'aperçu final de la lumière.

Les nouveaux pixels et l'aperçu du masquage ont été présentés avant activation,
conformément à AGENTS.md. Aucun changement du quartier suivant n'est engagé
pendant cette correction.

## Fabrication et reproduction

- Dessin : outil imagegen intégré, prompt conservé dans `roof-prompt-r01.txt`.
- Extraction technique : `python -X utf8 tools/reviews/build_street_roofs_review.py`.
- Capture isolée : `target/debug/project-rl.exe --ui-cold-street-roofs-integration artifacts/street-roofs-review-01/engine-integrated-r01`.
- Relancer le script d'extraction pour reconstruire la page et les planches à partir des captures.

L’ancienne proposition du moteur montrait toute l’architecture pour examiner
les toits. Les captures d’intégration utilisent la vision normale du jeu.
Les toits reçoivent la lumière ambiante extérieure validée ; les lampes
intérieures n'éclairent pas leur face supérieure. Les autres surfaces et
le personnage conservent la passe GPU de la rue acceptée.

Les sources sont dans `assets/source-art/first-sector-review-01/street-roofs-r01/`.
Seuls les deux polygones de toit sont extraits du nouveau dessin. Aucun pixel
de la rue approuvée n'est remplacé par une génération complète. Les personnages,
voitures, murs extérieurs, lampes et dégâts existants restent leurs sources antérieures.
L’ancienne capture `engine-r01` est la proposition visuelle acceptée. La capture
`engine-integrated-r01` utilise la vraie porte, les interactions clavier/clic et
la visibilité normale du jeu. La scène n’y est pas dévoilée artificiellement.

## Intégration

Les deux PNG sont extraits sans redimensionnement vers
`assets/maps/prison-street-01/`. Le manifeste référence leurs empreintes,
leurs cellules extérieures et l’intérieur de la maison. Le toit n’est pas
un terrain : observer sa surface ne découvre pas le sol, les acteurs ou les
objets sous celui-ci. Une surface extérieure déjà observée peut être
mémorisée avec l’assombrissement habituel.

La nouvelle génération 156 place une porte fermée sur le seuil existant.
Elle utilise les interactions ordinaires, sans nouvelle clé, verrouillage
ou texte. Le petit panneau métallique fermé réemploie les pixels d’une
porte déjà présente dans le décor approuvé, à la même échelle.

Les parties de génération 155 conservent leur entrée de sol ouverte, y
compris si la rue était encore différée. Le toit de cette maison reste alors
masqué comme pour une ouverture permanente. Les sauvegardes ne sont pas
réécrites ni migrées silencieusement ; les états de porte ordinaires sont
déjà sérialisés et aucun état de toit n’est ajouté.

## Vérifications d’intégration

- Six tests ciblés `prison_street` passent, dont deux tests de présentation.
- Les dix tests `cryogenic_camp_app::tests` passent : douze tests distincts au total.
- Compilation debug, vérification release, format et contrôle du diff réussis.
- Cinq captures du jeu vérifient la porte réelle : vue distante, fermeture
  extérieure, ouverture au clavier sans survol, fermeture par clic depuis
  l’intérieur, puis fermeture extérieure après sortie.
- Le monde reste inchangé pendant le rendu. Le retour au camp fonctionne.
- Les sources et les réglages d’éclairage approuvés gardent leurs empreintes ;
  une plage de chaussée conserve les mêmes pixels avant et après ouverture.

La comparaison approuvée du prototype est conservée dans
`assets/source-art/first-sector-review-01/street-roofs-r01/references/accepted-roof-comparison-native.png`.
Le prototype Rust qui dévoilait artificiellement toute la carte est supprimé.
Les commandes de diagnostic des toits utilisent désormais le vrai jeu.
La page courante est reconstruite par
`python -X utf8 tools/reviews/show_street_roofs_integration.py`.
Le bilan courant est `integration-render-r01.json`, complété par
`integration-checks-r01.json`. Les anciens bilans de proposition restent datés
et distincts ; ils ne décrivent pas l’état actuel de la porte.

Cinq anciennes sorties du prototype ont été supprimées après contrôle de
l’intégration (7 057 632 octets), en conservant les sources et la référence
native acceptée. Les captures actuelles restent dans la même revue.

## Correctif DOOR-U01-FIX-01 — 19 octobre 2026

Le joueur signale qu’après l’ouverture, la lumière demeure à la place de
la porte du premier bâtiment à gauche. Le dessin source et sa copie de
premier plan contenaient en effet le luminaire au milieu du seuil.

Le correctif réemploie les pixels approuvés : sol neutre dans l’ouverture,
luminaire sur le mur à droite, panneau métallique dans le passage uniquement
quand la porte est fermée. Le premier plan ne garde que les deux montants
latéraux. Les sources d’art et les silhouettes de toit restent intactes.
La lumière réelle du seuil suit désormais le luminaire sur le mur ; les
treize autres sources, leurs réglages et l’ambiance restent identiques.

La géométrie vient de
`assets/source-art/first-sector-review-01/u01-doorway-fix-r01/doorway.json`.
Les données sont reconstruites avec
`python -X utf8 tools/reviews/build_u01_lighting_review.py --doorway-fix`,
puis conditionnées avec `python -X utf8 tools/reviews/build_u01_playable.py`.
L’option évite de modifier les données historiques validées de LUM-U01-07.
Aucune règle de porte, case logique, collision ou sauvegarde ne change.
Le correctif visuel s’applique aussi aux reprises, sans nouvelle partie.
Une capture supplémentaire montre le personnage sur le seuil ouvert.
