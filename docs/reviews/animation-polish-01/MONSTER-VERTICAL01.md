# MONSTER-VERTICAL01 — reprise de tout le bestiaire

Le 2 novembre 2026, le joueur demande de reprendre les animations haut/bas de
tous les monstres : « je dirais tous ». Le catalogue actuel contient 129 modèles
et 471 couples modèle/variante. Cette demande étend le chantier ; elle ne valide
pas les nouveaux dessins et ne modifie pas les dix marches latérales intégrées.

## Premier lot r01 à examiner

1. MON-014-0 : cultiste sans arme.
2. MON-164-0 : cultiste au fusil, prises conservées dans le même port d’arme.
3. MON-144-0 : démon ailé au sol, ailes portées et appuis alternés.

Huit figures entières par direction, quatre poses par pas, 400 ms par pas et
800 ms par cycle. Le parcours de revue couvre deux cases de 64 px ; un arrêt
de 400 ms permet de comparer le vrai frontal. Aucun frontal intermédiaire n’est
inséré entre les deux pas. Le frontal retenu est copié exactement depuis les
atlas latéraux intégrés, y compris les identités préférées des cultistes.

Les sources sont produites avec l’outil intégré image_gen. Une reprise du démon
élargit sa carrure et ses ailes après comparaison native ; ses deux sources
retenues portent le suffixe `-broad-source.png`. Les premières sources sont
conservées pour la traçabilité. Les planches sont mises à l’échelle uniformément
par direction, au plus proche voisin ; aucun zoom par phase, décalage de bande de
jambe, cisaillement ou greffe de membre ne simule le pas.

## Revue et vérifications

L’aperçu courant contient le comparatif frontal conservé / marche actuelle /
nouvelle marche, ainsi que 129 cartes natives de l’apparence actuelle face/dos.
Ces cartes constituent un inventaire, pas une validation du bestiaire redessiné.
Le classement anatomique ancien reste indicatif : quadrupèdes, rampants, volants
et formes flottantes suivront leur locomotion propre. Les variantes de couleur
restent dans la couverture et demanderont leur contrôle avant intégration.

Les captures utilisent le décor et le composite d’éclairage du moteur. Le gain
historique des monstres reste 1,48. Les paramètres Discret du joueur sont hors
de ce lot. Les textures de revue sont préparées avant les boucles de dessin.
Le drapeau Debug `--ui-cold-monster-vertical01` est le seul point d’entrée ;
aucune sélection normale des nouveaux atlas n’est ajoutée.

Le contrôle natif vérifie 198 captures, 48 poses mobiles distinctes, six cellules
frontales exactes et leurs six retours au repos, 38 empreintes d’approbation,
les dix atlas latéraux intégrés inchangés et les 129 cartes d’inventaire.
Compilation Debug, test ciblé du périmètre des bipèdes, formatage Rust et
contrôle des espaces Git passent. La partie sauvegardée n’est pas chargée.

Vérification du navigateur effectuée après rechargement : les trois sélections
chargent leurs 66 captures ; les deux directions, la lecture automatique, la
pause, les phases, le frontal final, le ralenti et le zoom entier fonctionnent.
Les 129 cartes sont présentes dans l’inventaire. Aucune erreur de console sur
le rechargement final. Preuve et relevé dans
`artifacts/animation-polish-01/monster-vertical-01/r01/browser-proof.png` et
`browser-checks.json` du même dossier.

Sources et atlas :
`assets/source-art/animation-polish-01/monster-vertical-01/r01/manifest.json`.
Prompts : `MONSTER-VERTICAL01-PROMPTS.json`,
`MONSTER-VERTICAL01-ALPHA-PROMPT.txt`,
`MONSTER-VERTICAL01-PROPORTIONS-PROMPTS.json`.
Périmètre : `MONSTER-VERTICAL01-COVERAGE.json`.
Contrôles : `MONSTER-VERTICAL01-VALIDATION.json`.

Reproduction :

```powershell
python tools/art/prepare_monster_vertical01.py
cargo build --locked --bin project-rl
& target/debug/project-rl.exe --ui-cold-monster-vertical01 artifacts/animation-polish-01/monster-vertical-01/r01/current
python tools/art/build_monster_vertical01_review.py
python tools/art/check_monster_vertical01.py
```

Retour du joueur : « c'est bien mieux oui :) ». Le premier lot est conservé
comme base pour la suite ; ses douze sources, atlas et frontaux retenus sont
figés dans `MONSTER-VERTICAL01-FEEDBACK.json`. Le lot suivant reprend le possédé,
le démon massif et le cultiste à la hache. L’intégration normale reste distincte.
