# ACTOR-READABILITY01 — cinq pistes de lisibilité

## Intégration autorisée le 22 octobre

Le joueur demande ensuite : « tu peux appliquer tout ca au jeu (les nouveaux
visuels de classe, les aides à la visibilités etc) », avec correction des morceaux
d’armure parasites lorsqu’une arme de tir à deux mains est tenue. Il précise
ensuite que leur taille doit s’adapter à chaque personnage et confirme la synthèse :
dessin commun, échelle et position adaptées à la corpulence, deux mains sur les prises.

Les 25 tenues déjà acceptées restent actives. Leurs sources et les prises à une
main ne sont pas repeintes. Les armes longues utilisent un facteur d’échelle
par classe, avec transformation commune de l’arme et des deux attaches. Les
avant-bras possèdent une découpe dédiée qui retire les anciennes mains et les
bords de manche avant de replacer les membres. Le reste des poses est conservé.

Le shader d’éclaircissement est commun aux acteurs vivants : joueur, PNJ et
monstres, dont les 471 apparences du catalogue de campagne. Les trois nouvelles
formes choisies sont embarquées sous `assets/sprites/actor-readability-02/` ; les
variantes 2/3/4 du Fendeur conservent leurs identifiants et les palettes de matériau
existantes, adaptées à ses nouvelles plaques. Aucun autre dessin de monstre n’est
annoncé comme refait : les 126 modèles restants conservent leur art actuel et
reçoivent le traitement global de lisibilité en attendant leurs prochains lots.

Les repères cyan du joueur et rouges de la cible hostile visible sont activés
par défaut et désactivables. Le filtre « Monde en niveaux de gris » est facultatif,
désactivé par défaut, et conserve les repères, les effets animés et l’interface en
couleur. Les choix sont enregistrés avec les autres réglages d’affichage et
profitent de la même confirmation et du même rétablissement automatique. Les
anciens fichiers de préférences reçoivent ces valeurs par défaut sans perdre
leurs autres choix.

Le filtre est appliqué aux couches du monde avant les effets. Le composite
d’éclairage du camp et de la rue neutralise ses lumières lorsque ce mode est
actif, sans désaturer le framebuffer qui contient aussi les sorts et les repères.
Les nouvelles ressources n’utilisent aucun chemin sous `artifacts/` à l’exécution.

La preuve courante est `artifacts/actor-readability-integration-02/index.html`.
`integration-02.json` conserve l’autorisation et les limites ;
`integration-02-checks.json` et `integration-02-validation.json` consignent les
comparaisons du composeur et les contrôles du moteur. Les fichiers `checks.json`
et `checks-02.json` décrivent les présentations antérieures, avant cette autorisation.

## Sélection du 22 octobre — complément 02

Le joueur retient **02 — silhouettes éclaircies**, **04 — repères au sol** et
**05 — nouvelles formes de monstres**. Il demande ensuite d’étendre cette
direction à **tous les monstres**, variantes comprises. L’accord et ses limites
sont conservés dans `selection-02.json`. L’inventaire du catalogue de campagne
est dans `bestiary-scope-02.json` ; il distingue les trois formes présentées
des autres modèles qui restent à retravailler. Aucun monstre retiré ou boss
différé n’est réintroduit par cet inventaire.

Deux choix indépendants sont prévus dans les options d’affichage : repères au
sol désactivables et monde en nuances de gris activable. Le joueur précise que
les **repères doivent aussi rester en couleur**, comme les sorts et animations.
Le filtre du monde concerne les décors et les corps, y compris pendant leurs
déplacements ; il ne doit pas désaturer les effets de combat. Le HUD reste hors
du filtre du monde. Les libellés et valeurs par défaut sont des propositions
de menu montrées dans l’aperçu, sans modification des préférences du jeu.

Le preset **09** réunit 02/04/05 ; **10** ajoute les nuances de gris sélectives.
Le témoin reste coloré. La comparaison réutilise les atlas de Brûlure, Gel et
Boule de feu actuellement chargés par `agent_fire_visuals` / `agent_magic_visuals`,
avec les tailles, points d’ancrage et couches avant/arrière de ces rendus.
Les effets sont rejoués dans un montage navigateur ; ce n’est pas une exécution
du moteur ni une simulation des dégâts, collisions, portées ou zones de sort.
Le curseur d’instant et la pause permettent de comparer exactement la même phase.
Les images natives restent 320 × 256, sur une grille de 64 px.

Les PNG d’animation sont copiés à l’identique avec leurs empreintes, sans création
de nouvel effet artistique. Les anciens presets et leurs captures sont conservés
pour comparaison. Les contrôles du complément sont dans `checks-02.json`.

## Première présentation

Le joueur demande d’essayer les cinq propositions : contour fin, silhouettes
mieux éclairées, décor moins contrasté, repères au sol et grandes formes de
monstres plus distinctes. Cette demande autorise des comparaisons et des études,
pas leur validation artistique automatique ni leur intégration au jeu.

La page interactive est `artifacts/actor-readability-01/index.html`. Le témoin
et l’essai utilisent deux toiles natives de 320 × 256 px, sans redimensionnement
CSS par défaut. Le zoom explicite ×2 conserve le même nombre de pixels source.
Les acteurs sont placés aux mêmes appuis dans les deux vues et sur une grille
logique de 64 px. Les trois créatures conservent leurs dimensions natives du
jeu : leurs images de 128 px comprennent les marges et les débordements actuels.

Les options 01–04 sont des traitements du rendu dans le navigateur. Le contour
utilise un voisinage d’un pixel et adapte sa valeur au fond ; l’éclairage relève
les ombres et les bords supérieurs ; le décor atténue les hautes fréquences sans
changer la position des objets ; les arcs sont placés sous le joueur et la cible.
Chaque piste peut être utilisée seule. Les combinaisons 06, 07 et 08 sont aussi
proposées. L’intensité, le fond, la classe, les cinq armures, le gris et le zoom
sont réglables ; un clic sur un monstre change le repère de cible.

La rue est un extrait sans acteur de la référence réelle éclairée, cadré
en `[0,128,320,384]`. Le sol clair est un montage répétant une dalle vide de
la capture du camp. Les personnages sont les compositions réelles des 25 tenues
actuelles avec le fusil de patrouille ; les monstres témoins sont les ressources
effectivement utilisées par `game_art_finish::campaign_image` pour 007, 008 et 010.
Il s’agit d’un montage interactif de revue, pas d’une partie exécutée dans le moteur.

VIS05-A, VIS05-B et VIS05-C sont trois études imagegen issues des sources de
ces mêmes monstres. Elles conservent leurs identités et leurs équipements :
possédé massif au bras mécanique, Fendeur organique et possédé tireur. Les
surfaces sont simplifiées, les volumes et les appuis ajustés pour la lecture.
La réduction conserve les hauteurs actuelles et la ligne des pieds. Les sources,
exports et empreintes sont dans `assets/source-art/actor-readability-01/` et
`candidates.json` ; les prompts exacts sont conservés dans `prompts.json`.
La vue en noir aide à comparer les formes indépendamment de la peinture.

Les essais restent à examiner par le joueur. Aucune source approuvée, statistique,
identité de contenu, règle de jeu, sauvegarde ni code du rendu ordinaire n’a été
modifié pour cette revue. Les vérifications effectives sont dans `checks.json`,
`pixel-checks.json` et `browser-class-checks.json`.

Pour régénérer le conditionnement :

```powershell
python tools/art/prepare_actor_readability_01.py
```

Le script lit les sources de la revue, conserve les alpha, copie le renderer HTML
et prépare les images natives. Les réglages visuels sont dans
`tools/art/actor_readability_review_01.js`. Les PNG numérotés `street-00` à `08`
et `camp-00` à `08` sont exportés depuis le navigateur à l’aide des liens visibles
« Enregistrer le témoin / l’essai ». Les planches `*-all-options-native.png`
assemblent ces exports sans mettre les sprites à une autre échelle.
