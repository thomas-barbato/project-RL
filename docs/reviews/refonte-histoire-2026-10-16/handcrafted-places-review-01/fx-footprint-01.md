# FX-FOOTPRINT-01 — Rendu accepté et attribution des sorts corrigée

Le joueur accepte le 18 octobre 2026 les contours et chemins légèrement
transparents, les attaques orientées vers leur cible et les effets adaptés à
leur emprise réelle. Il clarifie ensuite explicitement : **Braise FE01 reste un
projectile monocible ; le couloir concerne Souffle de feu FE02**. Voir
`souffle-couloir-01-acceptance.json` et `souffle-couloir-01.md`.
L’ancienne attribution BRAISE-COULOIR-01 est annulée, pas la correction visuelle.

## Rendu actif

Les traits des contours et chemins utilisent un multiplicateur d’opacité de
82 %, leur sous-trait sombre 45 %. Les chiffres et cercles numérotés restent
fixes et opaques. Les petits repères conservent la pulsation centrée acceptée.

Braise conserve son projectile et sa case d’impact. L’émission et le dessin
suivent l’axe lanceur–cible. Souffle de feu utilise les trois bandes de flammes
présentées, dans un couloir de 3 cases de large, 2 de long au rang 1, puis +1
par rang effectif. Sa canalisation et ses chiffres propres sont conservés.
La visée est effacée après confirmation, avant l’animation.

Les autres adaptateurs (A01/A02, FE03/FE04/FE05, froid, explosions et attaques
de formes) conservent l’ajustement aux cases réelles. Les effets attachés aux
corps et les trajets ne sont pas étirés pour remplir une zone au sol. Les
marges transparentes des atlas sont mesurées une fois sur l’image CPU, sans
lecture GPU pendant le dessin. Un changement de caméra ne réduit pas la taille
réelle d’une grande zone : elle reste découpée au champ de jeu.

## Contrôles et reproduction

Le bilan courant est `souffle-couloir-01-integration-native.json`.
`fx-footprint-01-native.json` conserve la trace historique de la première revue ;
ses scènes de Braise en couloir ne représentent plus les règles acceptées.

Les contrôles ciblent Braise monocible, Souffle canalisé, les dégâts et brûlures,
les murs, les quatre directions, le ciblage des coins, la sauvegarde et la
confirmation souris/clavier. Les captures utilisent de vraies commandes dans
des parties isolées et comparent événements, état et RNG avec le moteur seul.
Les contrôles de pixels refusent les impacts hors de l’emprise prévue.

Page courante : `artifacts/walk-preview-review-01/effets-zones.html`.
Les deux planches principales montrent désormais Souffle. La planche Braise
montre le projectile dans quatre directions. Les diagnostics secondaires
superposent volontairement un contour technique, absent après confirmation
dans le jeu. Surfaces natives de 1280 × 1000 et 1920 × 1050, cases de 64 px.

Capture ciblée : `--ui-cold-souffle-corridor <dossier>`.
Diagnostic complet : `--ui-cold-effect-footprints <dossier>` ; cadrage des grandes
zones : `--ui-cold-effect-footprints-fire <dossier>`.
Assemblage sans redessiner ni redimensionner :
`python tools/art/package_effect_footprint_review.py`.
Les ressources du jeu restent dans `assets/`, sans dépendance à `artifacts/`.
