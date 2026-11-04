# MONSTER-VERTICAL05-r01 — cultistes avec objets portés

« on peut continuer » accepte le lot précédent 10–12 et poursuit la reprise
des marches haut/bas. Le cinquième lot propose, uniquement en variante 0 :

13. MON-015-0 : cultiste au bâton violet, capuche sombre bordée de rouge.
14. MON-016-0 : cultiste au bouclier rectangulaire et à la lame courte.
15. MON-017-0 : cultiste au bâton orné, capuche rouge et petite gemme violette.

Ces libellés servent à la revue ; les identités et règles du jeu ne changent
pas. Huit figures entières par direction, avec deux appuis alternés à 400 ms
par pas et 800 ms par cycle. Les bottes restent lisibles sous le bas des robes.
La tête, le torse, les bras et les objets restent cohérents ; aucune pièce ni
bande de pixels n’est déplacée séparément pour fabriquer la marche.

Les bâtons restent dans la main gauche anatomique : à droite de l’écran de
face, à gauche de dos. MON-016 garde le bouclier au bras gauche et la petite
lame dans la main droite. Le revers du bouclier et ses attaches se voient
de dos, sa face à bande rouge se dirigeant alors à l’opposé du spectateur.
Les capuches fermées montrent leur arrière sans masque ni visage copié.

Le repos conserve exactement la cellule 12 du VERT18 installé, avec son
ancrage et son ombre réels, dans les deux cellules de repos des propositions.
Les douze sélections précédentes restent disponibles dans l’aperçu courant.

Sources, frontaux et atlas :
`assets/source-art/animation-polish-01/monster-vertical-05/r01/manifest.json`.
Six planches produites avec l’outil intégré image_gen. Prompts et références :
`MONSTER-VERTICAL05-PROMPTS.json`. L’assemblage emploie une échelle uniforme par
direction et une translation de la figure entière pour son ancrage moyen.
Le diagnostic de couleurs est dans `MONSTER-VERTICAL05-COLORS.json` ; il
complète la comparaison visuelle native et ne modifie aucun pixel source.

La préparation vérifie les 92 empreintes antérieures : 38 latérales, 12 du
premier lot vertical, 12 du deuxième accepté, quatre du rouge harmonisé,
13 du troisième accepté et 13 du quatrième accepté. Les dix atlas latéraux
installés restent inchangés. Les textures de revue sont préparées avant
les captures, avec création dans le thread du contexte graphique.
Après l’accord du joueur, la préparation vérifie également les douze
ressources acceptées de ce cinquième lot.

Statut : le joueur accepte les cultistes 13–15 par « je valide :) ».
L’accord porte sur MON-015/016/017 en variante 0, les deux directions, huit
poses par direction, leurs objets portés et le repos frontal conservé, à
400 ms par pas et 800 ms par cycle. Les douze ressources retenues sont
figées dans `MONSTER-VERTICAL05-ACCEPTANCE-r01.json`, sans intégration dans
la partie normale. Quinze modèles proposés au total, 114 autres à reprendre,
puis les variantes restantes à contrôler. Les captures ne chargent aucune
partie ni préférence de l’utilisateur.

```powershell
python tools/art/prepare_monster_vertical05.py --references
python tools/art/prepare_monster_vertical05.py
python tools/art/audit_monster_vertical05_colors.py
cargo test --locked --bin project-rl monster_vertical_review::tests
cargo run --locked --bin project-rl -- --ui-cold-monster-vertical05 artifacts/animation-polish-01/monster-vertical-05/r01/current
python tools/art/build_monster_vertical05_review.py
python tools/art/check_monster_vertical05.py
```

Vérification finale du 3 novembre 2026 : build verrouillé et trois tests de
palette réussis ; 198 captures natives avec six retours au repos frontal
exact. Les 92 empreintes antérieures et les dix atlas latéraux installés
restent inchangés. Le contrôle de formats Python, le formatage Rust ciblé et
`git diff --check` passent.

Le lecteur a été exécuté dans l’onglet courant : les quinze sélections
chargent chacune 66 captures, l’inventaire conserve ses 129 fiches, et les
trois nouveaux modèles retrouvent le repos dans chaque direction avec le
curseur de phase. Le bouton de pose suivante et l’aller-retour automatique
à 400 ms par pas ont également été vérifiés. Le comparatif reste à
576 × 320 pixels, sans redimensionnement CSS. Aucun message d’erreur console
après l’ouverture finale. Bilan dans `MONSTER-VERTICAL05-VALIDATION.json` ;
preuves de face et de dos, et compte rendu navigateur, dans
`artifacts/animation-polish-01/monster-vertical-05/r01/browser-proof.png`,
`browser-back-proof.png` et `browser-checks.json`. Ces sorties de revue ne sont
pas des dépendances du jeu.
