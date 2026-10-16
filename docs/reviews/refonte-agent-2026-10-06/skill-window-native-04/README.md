# SKILLS-NATIVE04 — compactes sombres intégrées

Le joueur valide les trente compactes présentées dans SKILLS-IC17 le 11 octobre
2026 (« je valide :) »), après la révision du bâillon de Silence AF03-C03.
L'accord exact est enregistré dans ../skill-icon-review-17/visual-acceptance-01.json.

La copie passe par tools/art/package_agent_skill_window.py. Les trente fichiers
compactes actifs correspondent exactement aux exports 64 px acceptés. Les
anciennes sources, les seize illustrations et le catalogue des quatre-vingt-quatorze
compétences sont conservés. La banque active contient trente et une compactes.
Les quatre essais de SKILLS-IC16 restent une revue distincte.

Les fichiers acceptance-before.json, direction-before.json et catalogue-before.json
conservent les données précédentes. Le registre garde les variantes remplacées.
Cette étape ne raccorde aucun nouvel effet ni apprentissage au moteur de combat.

Les trois tests existants de la fenêtre des compétences passent, ainsi que les
compilations debug et release, cargo fmt --check et git diff --check. La compilation
release émet les quatre avertissements déjà présents dans les contrôles précédents.
Les empreintes de 236 exports acceptés et des 47 dessins conditionnés sont vérifiées.

capture_native_icons.ps1 produit une capture isolée du jeu par fiche à 960 × 540 px,
sans lire ou écrire la partie et les réglages du joueur. verify_native_captures.cjs
localise chaque icône sélectionnée dans sa capture et la compare au dessin validé,
avec une tolérance pour le filtrage des positions sous-pixel. Les icônes extraites
restent à 64 px réels dans native-selected-icons.png.

index.html donne accès aux captures. Les résultats définitifs et les preuves sont
répertoriés dans integration-validation.json et native-icon-matches.json.
