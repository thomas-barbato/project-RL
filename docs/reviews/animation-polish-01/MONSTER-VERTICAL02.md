# MONSTER-VERTICAL02-r01 — deuxième lot haut / bas

Après « c'est bien mieux oui :) », le premier lot 01–03 reste la base de travail.
Ses pixels et sources sont conservés par les douze empreintes de
`MONSTER-VERTICAL01-FEEDBACK.json`. La suite du chantier de tout le bestiaire
propose trois modèles en variante 0 et palette d’origine :

4. MON-005-0 : possédé, silhouette voûtée et longs bras.
5. MON-156-0 : démon massif, forte carrure et lourds appuis griffus.
6. MON-165-0 : cultiste à la hache, deux prises et côté anatomique conservés.

Chaque direction compte huit dessins entiers : deux pas alternés, 400 ms par
pas et 800 ms par cycle. Le frontal intégré est conservé exactement au repos.
Les sources sont mises à l’échelle uniformément par direction ; une translation
constante du cycle entier aligne son appui moyen avec le frontal. La hache ne
doit pas tirer le corps de côté quand le cultiste tourne le dos. Aucun membre,
aucune tête ni bande de jambes n’est déplacé indépendamment.

Le dos du cultiste a reçu une reprise supplémentaire : le manche porté devant
le torse est masqué par le corps. Le fer reste au-dessus de l’épaule droite
anatomique, donc à droite de l’écran de dos et à gauche de face. Le dessin de
la hache et ses deux prises frontales restent cohérents pendant la marche.

Les dessins sont produits par l’outil intégré image_gen. Sources et atlas dans
`assets/source-art/animation-polish-01/monster-vertical-02/r01/manifest.json`.
Prompts complets dans `MONSTER-VERTICAL02-PROMPTS.json` et
`MONSTER-VERTICAL02-OCCLUSION-PROMPT.json`.

Le rendu natif reprend le décor, la case de 64 px et le gain historique 1,48 des
monstres. Les nouveaux atlas sont préparés avant les dessins de capture, dans
le seul mode Debug `--ui-cold-monster-vertical02`. Le rendu normal du jeu ne
sélectionne aucun nouveau cycle de ce lot. Les dix latéraux intégrés et les
48 poses du premier lot restent préservés.

L’aperçu courant conserve les six sélections 01–06 et les 129 cartes d’inventaire
face/dos. Le lot suivant ne remplace pas le premier. Les contrôles natifs sont
consignés dans `MONSTER-VERTICAL02-VALIDATION.json` ; compilation Debug, test
ciblé du périmètre des bipèdes, formatage Rust et contrôle des espaces Git ont
été exécutés. Aucune partie sauvegardée n’est chargée par la capture.

Le navigateur a été vérifié sur les six modèles : 66 captures chargées par
sélection, chemins du premier lot conservés, aller-retour automatique, pause,
phases, repos frontal haut/bas, deux ou trois colonnes natives, ralenti et zoom
entier. Les 129 cartes sont présentes ; aucune erreur de console après le
chargement final. Preuve et relevé :
`artifacts/animation-polish-01/monster-vertical-02/r01/browser-proof.png` et
`browser-checks.json` dans le même dossier.

```powershell
python tools/art/prepare_monster_vertical02.py
cargo build --locked --bin project-rl
& target/debug/project-rl.exe --ui-cold-monster-vertical02 artifacts/animation-polish-01/monster-vertical-02/r01/current
python tools/art/build_monster_vertical02_review.py
python tools/art/check_monster_vertical02.py
```

Statut : deuxième lot validé par « je valide :) », avec une réserve sur la
couleur mobile du démon ailé 03 du premier lot. Les douze empreintes de
`MONSTER-VERTICAL02-ACCEPTANCE-r01.json` conservent les dessins exacts de 04–06.
Six modèles sont proposés au total, 123 autres restent à reprendre, puis les
variantes de couleur à contrôler avant intégration. La correction de 03 est
suivie séparément dans `MONSTER-VERTICAL-COLOR01.md`.
