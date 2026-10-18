# Lieux préparés à la main — premier aperçu

## Corrections après essai du camp — 18 octobre

Le passage entre le camp et son secteur adjacent permet désormais l'aller-retour,
y compris après sauvegarde et reprise. Les anciens parcours gardent leurs règles.
Le joueur a validé les pictogrammes de dialogue et de services ainsi que la
respiration [CAMP-NPC-01 r01](CAMP-NPC-01.md), désormais intégrés. Sa précision
ajoute le « ! » de l'officier et le flottement connu des symboles.
La revue [HOVER-OUTLINE-01 r01](HOVER-OUTLINE-01.md) prépare séparément le nouveau
contour vert/rouge des silhouettes survolées, encore à valider visuellement.
Les anciens repères d'alerte et de perception au-dessus des ennemis sont retirés
à la demande du joueur ; leurs icônes de statuts restent affichées.
La revue [DIALOGUE-BUBBLE-01](DIALOGUE-BUBBLE-01.md) présente trois bulles de
dialogue dans le client réel, avec le texte complet de l'officier inchangé.
Le joueur choisit ensuite le style terminal : la r02 en revue corrige la
jonction de la pointe, ajoute une légère transparence et retire le bouton
de fermeture ainsi que les coins renforcés. Clic ou Échap ferment l'aperçu.
Le texte s'écrit progressivement puis laisse un curseur bloc clignotant.
Les notifications consultées restent masquées ; un « ? » distinct est proposé
pour une quête à rendre. Trois essais sonores originaux sont proposés dans
la même page sous DIALOGUE-AUDIO-01, avant tout ajout au jeu.

Le son 03 reste provisoire. La bulle terminal r02 de l’officier est désormais
raccordée au **lancement habituel** (`cargo run`, `Jouer.cmd`), avec l’audio
activé par défaut. Le joueur la teste dans sa partie normale. `Tester_dialogue_terminal.cmd` ouvre une partie distincte à côté
de l’officier ; E permet d’écouter la frappe dans la bulle r02. Voir
`dialogue-audio-selection-01.json` et `dialogue-audio-trial-validation-01.json`.

## Raccordement au départ de campagne — génération 153

Le joueur confirme que cette carte sera le point de départ du personnage.
Les nouvelles parties utilisent désormais `core:cryogenic_camp`, au réveil
près de la cuve ouverte, en conservant les protocoles et équipements initiaux.
Les générations 1 à 152 gardent leur construction historique à la reprise.

Les ressources du camp validé sont copiées sans retouche dans
`assets/maps/cryogenic-camp-01/`. Le client utilise son interface, sa caméra,
ses personnages équipés, sa mémoire du terrain et sa visibilité habituelles.
L’éclairage et l’ambiance r09b sont appliqués au décor et aux acteurs ; ils
ne produisent aucune commande de simulation. L’option d’animations réduites
fige les trois animations.

L’officier utilise DIAL-CRYO-01 r02, avec la phrase demandée par le joueur :
« Vous n’avez de toute façon pas le choix. » Le commerce, les soins et
l’amélioration réutilisent les règles et tarifs existants. Les quêtes et
objectifs de l’ancien quartier ne sont pas installés dans ce nouveau camp.

Le raccord de sortie utilise provisoirement le secteur industriel existant,
avec ses règles de déplacement actuelles, en attendant le nouveau premier
secteur. Une colonne de mur technique hors du dessin conserve la fermeture
de la grille exigée par le moteur ; aucun élément visible ni aucune position
du plan validé n’est déplacé. Le dessin reste de 38 × 18 cases à 64 px.

`Jouer.cmd` compile et ouvre le client normal. Le prototype de référence
reste disponible séparément par `Voir_camp_prototype.cmd`.

Les obstacles fixes du mobilier sont reconstruits à partir du plan accepté
avant de vérifier l’empreinte d’un cache de sauvegarde : cette couche de
`Map` est volontairement absente des anciens formats de sérialisation.
Le format des sauvegardes n’est pas étendu et la reprise conserve les
collisions, la progression et les services du camp.

`--ui-cold-cryogenic-camp` produit six captures isolées par le client réel,
après le même parcours de création que le menu normal. Le diagnostic contrôle
le cadrage de l’éclairage, l’échelle de 64 px, le noir d’une case inconnue et
l’absence de mutation du jeu au rendu. `show_cryogenic_camp_integration.py`
vérifie les copies des ressources, les shaders acceptés et enrichit la galerie
courante avec ces captures. Les preuves sont dans `campaign-integration-01.json`.

Vérification finale : 966 tests moteur, les 5 tests du camp et 28 scénarios
de régression affectés passent. Compilation du client, contrôle de compilation
en release, formatage et vérification du diff passent également. La suite
globale n’est pas déclarée entièrement verte : ses 28 échecs déjà consignés
dans `bestiary-integration-06/validation.json` restent suivis séparément.

## Carte validée — PLACE-CAMP-01 r09b

Le 17 octobre 2026, le joueur confirme : « parfait comme ca :) je valide cette
carte :) ». Le plan, les décors et leur échelle, le mobilier, les emplacements
des PNJ, l’éclairage et les trois animations renforcées sont acceptés comme
référence du camp de départ. La salle de cryogénie et ses sept cuves,
l’officier à son entrée, les trois lits et les ouvertures dégagées en font partie.

`acceptance-r09b.json` conserve cet accord et les empreintes des ressources,
du rendu et des aperçus montrés. Le prototype est toujours distinct du client
de campagne ; son raccordement constitue l’étape suivante. Cet accord visuel
ne tranche pas de nouveaux textes, services ou règles de jeu.

## r09b — animations plus visibles

Le joueur trouve les animations trop discrètes. Le même prototype renforce
**ANIM-CAMP-01 à 03** : voyants de cuves sur deux rangées avec une pulsation
plus contrastée, balayage des écrans plus lumineux et plus rapide, vapeur
plus ample et plus dense toutes les six secondes. Les lampes, le plan et
l’échelle native restent identiques. La capture et la galerie courantes sont
régénérées ; les boucles portent le suffixe `r09b` pour éviter l’ancien cache.
Le détail de cet ajustement est conservé dans `atmosphere-visibility-r09b.json`.

## r09 — éclairage et animations d’ambiance actifs

Le joueur autorise la mise en place des idées présentées en r08 :
**LUM-CAMP-01**, **ANIM-CAMP-01**, **02** et **03**. L’accord est conservé dans
`atmosphere-acceptance-r09.json`. Aucun nouvel accord n’est demandé pour les
effets déjà acceptés.

`examples/handcrafted_camp/atmosphere.rs` ajoute une passe GPU au prototype :
compensation colorimétrique des anciens halos, éclairage des treize luminaires
sur le décor et les acteurs, puis voyants, écrans et vapeur. Les plans,
personnages, meubles, dimensions et règles restent ceux de r08. Les textures
de personnages et de décors ne sont pas réécrites : le traitement est effectué
au rendu. Les ombres et reliefs propres aux matériaux restent peints ; il ne
s’agit pas d’une reconstruction physique complète de leurs matériaux.

`prepare_camp_atmosphere_r09.py` produit deux textures de données numériques
dans `assets/prototypes/handcrafted-camp-01/r09/`. Le champ d’éclairage tient
compte des murs de la grille réelle : le premier mur reçoit la lumière et
arrête les rayons. Les occultations sont précalculées pour ce plan fixe ; les
personnages mobiles échantillonnent ce champ dans la passe GPU. Il n’y a pas
d’ombre dynamique projetée par le mobilier. Cette première intégration ne
généralise pas encore la préparation à toutes les cartes procédurales.

Les sept voyants de cuves pulsent avec des phases différentes. Deux écrans
présentent une ligne de balayage lente. La cuve ouverte produit une petite
bouffée transparente occasionnelle. L’horloge cosmétique ne fait avancer aucun
tour et les fonctions de rendu empruntent l’état de jeu sans le modifier.
Les lampes restent stables ; **L** permet de les couper pour la comparaison,
**P** fige les animations. Les autres touches conservent leur rôle.

Contrôles : le parcours du vrai moteur passe. La capture sans effets est
identique pixel par pixel à r08 et retrouve les sprites natifs inchangés.
Les captures éclairées/éteintes confirment une variation sur le joueur et
dans la pièce éclairée ; le point témoin derrière le mur ne reçoit pas cette
lumière. Les boucles WebP sont comparées sans perte aux 24 images GPU sources,
avec vérification distincte des voyants, écrans et vapeur.

Le benchmark enregistre la taille réelle de la fenêtre, la médiane et le
95e percentile du temps par image, présentation comprise. La demande 4K est
ramenée à la taille disponible par le bureau de test : ces chiffres ne valent
donc pas validation d’une fenêtre 4K réelle. Les rayons et textures numériques
ne sont ni recalculés ni envoyés au GPU à chaque image. Les mesures et limites
sont conservées dans `full-camp-validation-r09.json`.

Reproduction courante :

```text
python tools/art/prepare_camp_atmosphere_r09.py
cargo test --locked --example handcrafted_camp_full_review
cargo build --locked --example handcrafted_camp_full_review
target/debug/examples/handcrafted_camp_full_review.exe --capture artifacts/handcrafted-places-review-01/engine-r09
target/debug/examples/handcrafted_camp_full_review.exe --performance artifacts/handcrafted-places-review-01/engine-r09/performance-1600x1000.json
python tools/art/show_camp_atmosphere_r09.py
```

`Voir_camp_prototype.cmd` ouvre r09. `--legacy-r08` conserve la comparaison avec
le décor à lumière peinte. La galerie réutilise `index.html`, affiche les
boucles natives et conserve `camp-r08.html` comme référence précédente.

## r08 — mobilier, ouvertures du bas et propositions d’ambiance

Le joueur aime le dialogue r01 mais demande de remplacer le sort des détenus
précédents par l’absence de choix. **DIAL-CRYO-01 r02** garde les parties aimées
et termine par « Vous n’avez de toute façon pas le choix. ». Le texte courant
est présenté dans `camp-adjustments-r08.json` et la galerie. Le récit global
sur les autres détenus n’est pas modifié par cette correction de réplique.

Les trois chaises DEC-CAMP-02 sont retirées. Deux instances du lit accepté
DEC-CAMP-10 s’ajoutent dans l’infirmerie, qui comporte maintenant trois lits.
Les PNJ et les autres meubles restent à leurs emplacements. Les accès à la
secouriste, au chariot et à la sortie restent praticables.

Le joueur signale aussi que les deux salles du bas présentent une lumière
en guise de porte. Imagegen corrige deux détails de 256 × 256 px : ouverture
centrale dégagée, lampes déplacées sur les murs latéraux. Le conditionnement
remplace seulement deux rectangles de 144 × 64 px, avec raccord de bord.
Les pixels du décor hors de ces rectangles restent identiques. Les sources
et prompt sont conservés dans `r08/` sous le dossier source et `generation-r08.json`.

**LUM-CAMP-01** propose de remplacer les halos peints par une passe d’éclairage
2D du moteur, sur le décor et les acteurs, arrêtée par les murs. Les boîtiers
restent dessinés. Cette passe n’est pas encore implémentée : retirer d’abord
les halos existants évitera de doubler l’éclairage. Les trois animations
proposées sont **ANIM-CAMP-01** (voyants des cuves), **02** (écrans) et **03**
(petites bouffées de ventilation). Le choix conseillé pour commencer est
l’éclairage stable avec les deux premières animations. Elles restent des
propositions avant activation, indépendantes des tours et de la visibilité tactique.

Le test de parcours du moteur passe, ainsi que la compilation et les captures.
Les 15 contrôles d’acteurs retrouvent leurs pixels opaques exacts. Une comparaison
des captures r07/r08 confirme que les changements sont limités aux meubles
demandés et aux deux corrections d’ouverture. `full-camp-validation-r08.json`
conserve les preuves ; les nouveaux effets d’ambiance ne sont pas activés.

Reproduction courante :

```text
python tools/art/prepare_camp_adjustments_r08.py
cargo test --locked --example handcrafted_camp_full_review
cargo build --locked --example handcrafted_camp_full_review
target/debug/examples/handcrafted_camp_full_review.exe --legacy-r08 --capture artifacts/handcrafted-places-review-01/engine-r08
python tools/art/show_camp_adjustments_r08.py
```

Le lanceur ouvre r08. `--legacy-r07` reproduit l’étape précédente ; les autres
options historiques restent disponibles. La galerie conserve une seule page
courante et référence `camp-r07.html` pour la comparaison précédente.

## r07 — l’officier attend à l’entrée de la cryogénie

Le joueur demande le militaire déjà dessiné, à l’entrée de la pièce, pour
expliquer le réveil du détenu. Le candidat retrouvé est **NPC-01 r03**, officier
casqué à veste olive et baudrier ivoire, validé dans
`../npc-monsters-review-01/visual-acceptance-01.json`.

Il est déplacé de son bureau `[18,4]` vers `[6,7]`, juste au nord du passage
de la cryogénie. Il reste un seul NPC-01, sans nouvelle identité ou nouveau
dessin. Sa taille, ses pixels et son ombre séparée sont conservés. Le joueur
peut l’approcher en `[6,8]`, puis continuer dans le hall. Les trois autres PNJ,
les cuves, le mobilier et l’architecture gardent leurs emplacements et fichiers.

`cryo-officer-placement-01.json` conserve la demande, le placement et le rôle.
Il contient aussi la proposition **DIAL-CRYO-01 r01**, présentée dans la galerie,
sur la pénurie de soldats, le Puits et la liberté promise. Ce texte est proposé
à partir de `HISTOIRE_CAMPAGNE.md`, section 2 ; il n’a pas encore été approuvé.
L’instruction d’`AGENTS.md` exige une validation explicite des nouveaux textes
avant intégration. Le prototype ne déclenche donc pas encore de conversation,
de quête ou de récompense.

Le test de parcours rejoint toujours les quatre PNJ et la sortie. Compilation,
formatage et captures GPU passent. Les 15 contrôles des sprites retrouvent leurs
pixels opaques inchangés. Une comparaison avec r06 confirme que les seuls pixels
modifiés de la capture initiale sont ceux de l’officier et de son ombre, à son
ancien et son nouvel emplacement. Voir `full-camp-validation-r07.json`.

Reproduction courante :

```text
python tools/art/prepare_camp_officer_r07.py
cargo test --locked --example handcrafted_camp_full_review
cargo build --locked --example handcrafted_camp_full_review
target/debug/examples/handcrafted_camp_full_review.exe --legacy-r07 --capture artifacts/handcrafted-places-review-01/engine-r07
python tools/art/show_camp_officer_r07.py
```

`Voir_camp_prototype.cmd` ouvre maintenant r07. Les options `--legacy-r06` et
`--legacy-r05` reproduisent les deux versions précédentes. La galerie courante
réutilise `index.html` ; `camp-r06.html` conserve la revue précédente.

## r06 — salle de cryogénie à gauche et départ du joueur

Le joueur valide le camp complet r05 par « c'est tres bien comme ca » et
demande une salle de cryogénie à gauche, avec plusieurs cuves puisque l’agent
n’est pas le dernier candidat. **`acceptance-r05.json`** conserve cet accord
sur l’architecture, l’agencement et les proportions montrées. Les mentions
« à examiner » dans les paragraphes historiques r05 décrivent leur état avant
cette réponse ; elles ne révoquent pas la validation.

La revue **r06** ajoute une salle reliée à l’ancien accès gauche, avec six
cuves fermées et une cuve ouverte. Le joueur apparaît à côté de cette dernière.
Les silhouettes anonymes derrière le verre des cuves fermées suggèrent les
autres candidats ; elles ne sont pas de nouveaux acteurs de simulation.
Les candidats **DEC-CAMP-15** et **DEC-CAMP-16** mesurent respectivement
44 × 96 et 48 × 96 px, face au joueur inchangé de 65 px de haut.
Ces nouveaux dessins et l’extension restent soumis à la revue visuelle.

`prepare_camp_cryo_r06.py` conditionne les sources imagegen et place le camp
accepté 640 px à droite, sans changement de ses pixels ou du mobilier.
La carte comporte 38 × 18 cases. Le passage relie la cryogénie au hall :
l’ancien bord gauche devient cette liaison intérieure et le bord droit reste
la sortie de l’aperçu. Les cuves bloquent leurs emplacements ; aucune interaction,
animation de réveil, règle de mort/réapparition ou ressource n’est ajoutée.

Le test de parcours du vrai `GameState` rejoint les quatre PNJ et la sortie
depuis la cryogénie. Compilation, formatage et trois captures GPU passent.
Les 15 vérifications d’acteurs retrouvent tous leurs pixels opaques d’origine.
La capture du joueur dans le hall, recadrée sans zoom de 640 px, est exactement
identique à la capture r05 validée sur les 1 792 × 1 152 px du camp existant.
Voir `full-camp-validation-r06.json` pour les preuves et empreintes.

Les sources intactes, le premier essai de passage et sa correction sont
conservés sous `assets/source-art/.../handcrafted-places-review-01/r06/` ;
`generation-r06.json` conserve les trois prompts du **built-in imagegen**.
Les ressources consommées par le prototype sont dans `assets/prototypes/`.

Reproduction courante :

```text
python tools/art/prepare_camp_cryo_r06.py
cargo test --locked --example handcrafted_camp_full_review
cargo build --locked --example handcrafted_camp_full_review
target/debug/examples/handcrafted_camp_full_review.exe --legacy-r06 --capture artifacts/handcrafted-places-review-01/engine-r06
python tools/art/show_camp_cryo_r06.py
```

`Voir_camp_prototype.cmd` ouvre maintenant r06, avec départ dans la nouvelle
salle. `--legacy-r05` reproduit l’ancien camp complet. La page courante reste
`artifacts/handcrafted-places-review-01/index.html` ; `camp-r05.html` conserve
l’aperçu accepté. Les captures et les planches sont présentées à taille native.
Le prototype reste séparé du client et des sauvegardes de campagne.

## r05 — le camp complet de la référence choisie

Le joueur désigne précisément l’illustration r02 : « c'est à dire celle-ci : ».
Il confirme ensuite l’adaptation du mobilier à la taille réelle du jeu.
La cible est donc cette architecture à quatre pièces, avec son passage central,
ses volumes de murs et sa lumière ambrée. Voir `reference-selection-r05.json`.
Le retour sur r04 reste conservé : style encore différent et fragment de porte
dans le vide. Choisir la référence ne valide pas ces défauts ni la nouvelle capture.

L’exemple **`handcrafted_camp_full_review`** affiche désormais le camp complet.
Imagegen a séparé les meubles de l’architecture de référence et dégagé ses accès ;
les sources intactes et les deux prompts sont conservés dans `r05/` sous le
dossier source et dans `generation-r05.json`. L’architecture de ce lieu fixe
est conditionnée en couches sol/structure de 1 792 × 1 152 px. Le mobilier,
les ombres de contact et les acteurs sont dessinés séparément. Il ne s’agit
pas encore d’une bibliothèque de bâtiments pour générer tous les secteurs.

Les huit candidats **DEC-CAMP-07 à 14** reprennent les meubles de la référence.
Le bureau mesure 90 × 50 px, le lit 34 × 72 px, le chariot 26 × 40 px et le
coffre 42 × 34 px. Le siège DEC-CAMP-02 r04 est conservé. Les cinq acteurs
acceptés restent inchangés : joueur, responsable des missions, armurier,
secouriste et technicienne, tous à leur taille native. Ces nouvelles proportions
et leur implantation restent à examiner sur la planche et les captures.

Les quatre liaisons entre pièces et hall sont des **passages ouverts**.
La carte logique de 28 × 18 cases utilise le vrai `GameState`, avec emprises
explicites du mobilier et occupation par les PNJ. Le test rejoint les quatre
PNJ et les deux bords de sortie en exécutant les commandes du moteur.
Les sorties n’emmènent pas encore vers un autre secteur. Les PNJ sont neutres,
immobiles et sans service ajouté. Aucune sauvegarde de campagne n’est utilisée.

Le défaut de la porte r04 venait du découpage : une barre restait détachée
de son montant. L’atlas r05 raccorde chaque partie aux murs voisins. L’ancien
essai à deux pièces le vérifie avec `--corrected-doors` ; les reproductions
r03 et r04 sont conservées. Ce test de porte est distinct des passages du camp.

**31 tests passent** entre les deux exemples. Les deux compilations, le
formatage et les captures GPU passent. Le contrôle des PNG retrouve exactement
les pixels opaques des cinq acteurs dans les trois captures du camp, puis des
trois acteurs de l’essai de porte. `full-camp-validation-r05.json` conserve
ces preuves ; elles ne constituent pas une validation artistique automatique.

Reproduction courante, depuis la racine :

```text
python tools/art/prepare_camp_reference_r05.py
cargo test --locked --example handcrafted_camp_review --example handcrafted_camp_full_review
cargo build --locked --example handcrafted_camp_full_review --example handcrafted_camp_review
target/debug/examples/handcrafted_camp_full_review.exe --legacy-r05 --capture artifacts/handcrafted-places-review-01/engine-r05
target/debug/examples/handcrafted_camp_review.exe --corrected-doors --capture artifacts/handcrafted-places-review-01/door-fixture-r05
python tools/art/show_camp_reference_r05.py
```

`Voir_camp_prototype.cmd` ouvre maintenant **le camp complet r05** : flèches,
ZQSD ou WASD pour marcher, G pour la grille, R pour recommencer, Échap pour quitter.
La page `artifacts/handcrafted-places-review-01/index.html` montre le camp et
les détails sans zoom. `engine-r04.html` conserve l’ancienne page de comparaison.
Les paragraphes suivants décrivent les étapes historiques.

## r04 — kit illustré, T peints et caméra fixe

Le joueur écarte visuellement r03 : T absents, murs centraux à l’envers et
écart avec la maquette montrée. Voir `engine-feedback-r03.json`.
Les tests fonctionnels de r03 ne valident pas son dessin.

La revue courante **r04** reprend les matériaux de l’illustration r02 avec
un nouveau kit imagegen, conditionné sous `assets/prototypes/handcrafted-camp-01/r04/`.
Il contient seize murs dont quatre T peints : W07, W11, W13, W14. Les jonctions
du plan sélectionnent W14 en haut et W11 en bas. Le nouveau module
`examples/handcrafted_camp/scenery.rs` lit directement les orientations,
sans retourner leurs reliefs. Sols, portes, mobilier et lampes ambrées sont
refaits ; les proportions proposées en r03 et les trois acteurs sont conservés.
La teinte lumineuse statique du sol est une proposition visuelle, sans effet
de jeu. Le kit, l’ambiance et le lieu restent **à valider**.

Les sources intactes et prompts sont conservés dans le dossier source
`handcrafted-places-review-01/r04/`, `generation-r04.json` et
`generation-additions-r04.json`. Le conditionnement retire les fragments
étrangers aux morceaux de murs et cale leurs ports de raccord. `kit-r04.json`
enregistre les empreintes et `engine-review-r04.json` les captures réelles.

**29 tests passent**, dont la continuité des trois branches de chaque T,
l’absence de fragments séparés, les ports de raccord et le centre transparent
des portes ouvertes. Compilation, formatage et trois captures GPU passent ;
les PNG conservent exactement les pixels opaques des acteurs. La galerie montre
aussi les raccords supérieur et inférieur séparément à taille native.
Le prototype reste séparé de la campagne et de ses sauvegardes.

Reproduction courante :

```text
python tools/art/prepare_handcrafted_camp_kit.py
cargo test --locked --example handcrafted_camp_review
cargo build --locked --example handcrafted_camp_review
target/debug/examples/handcrafted_camp_review.exe --capture artifacts/handcrafted-places-review-01/engine-r04
python tools/art/show_handcrafted_camp_engine_review.py
```

`Voir_camp_prototype.cmd` ouvre r04. Pour reproduire r03, ajouter
`--legacy-r03` à l’exécutable et `--revision r03` au script de galerie.
Les revues r02 et r03 restent accessibles comme références historiques.

## r03 — deux pièces assemblées dans le moteur

Le joueur demande de continuer après avoir signalé les accès murés et le
mobilier disproportionné de r02. La première preuve technique est **r03** :
un prototype natif avec deux pièces, une porte ouvrable, un joueur mobile,
l’armurier NPC-02 r02 et la secouriste NPC-03 r02 acceptés.

`examples/handcrafted_camp_review.rs` utilise le vrai `GameState` du jeu et
les modules de rendu modulaires déjà partagés par l’éditeur et l’essai de
surface. Il reste un exécutable de revue séparé du client de campagne.
Les PNJ sont des acteurs neutres immobiles ; leurs services, dialogues et
compétences ne sont pas implémentés par cette revue. Aucune sauvegarde de
campagne n’est lue ou écrite.

Le fond illustré r02 n’est plus utilisé pour cette scène. Les sols, murs,
coins et portes viennent de `assets/prototypes/surface64/` ; six meubles de
cette bibliothèque sont conditionnés séparément dans
`assets/prototypes/handcrafted-camp-01/`, puis affichés à leur taille native.
Le banc a remplacé le coffre de l’illustration pour cet essai réduit.
Le lit mesure 68 px de long, le siège 28 px de haut et l’armoire 56 px,
contre 60 px pour les deux PNJ et 65 px pour le joueur, sans changement
de leurs sprites. Ces dimensions sont des **propositions de proportions**,
pas des tailles acceptées pour le contenu de campagne.

La page courante présente trois extraits natifs de captures GPU : porte
ouverte, porte fermée, joueur après franchissement. Une bande séparée
compare les mêmes sprites et meubles à la même échelle. Les captures
complètes sont conservées avec les preuves de parcours. Elles montrent
un essai technique de proportions et de circulation ; l’habillage détaillé
de l’illustration r02 reste à reconstruire avec des éléments séparés.
Le plan du camp entier et les nouveaux dessins restent en revue.

Validation : **27 tests de l’exemple passent**, dont les collisions de porte,
de mobilier et de PNJ ; compilation de l’exemple ; trois captures GPU avec
égalité des pixels opaques des acteurs, également vérifiée sur les PNG
enregistrés. `engine-review-r03.json` conserve le périmètre et les preuves,
`native-furniture-r03.json` les sources et dimensions proposées.

Reproduction depuis la racine du projet :

```text
python tools/art/prepare_handcrafted_camp_fixture.py
cargo test --locked --example handcrafted_camp_review
cargo build --locked --example handcrafted_camp_review
target/debug/examples/handcrafted_camp_review.exe --legacy-r03 --capture artifacts/handcrafted-places-review-01/engine-r03
python tools/art/show_handcrafted_camp_engine_review.py --revision r03
```

Pour jouer dans le prototype : `Voir_camp_prototype.cmd`. Flèches, ZQSD ou
WASD pour marcher, E à côté de la porte pour l’ouvrir ou la fermer, G pour
la grille, R pour réinitialiser et Échap pour quitter.

Les scripts r01 et r02 reproduisent leurs revues historiques ; exécuter
`show_handcrafted_camp_engine_review.py` en dernier pour restaurer la page
courante r03. Aucun dessin du client de campagne n’est remplacé.

## r02 — décor et PNJ, composition à examiner

Demande du joueur : « voyons ce ca donne en vrai (avec les décors et les NPC) ».
La page courante montre désormais une première **composition habillée** du camp,
avec un décor créé avec imagegen et les quatre PNJ déjà acceptés. Il s’agit
d’un aperçu visuel, pas d’une capture du moteur ni d’une carte jouable.

Le poste de missions et l’armurerie occupent les pièces du haut ; les soins,
l’atelier et le coffre celles du bas. La technicienne est placée dans l’espace
initialement réservé au coffre : cette association reste une proposition.
Les emplacements ne créent aucun dialogue, stock, coût, interaction ou effet.

Les sprites natifs du joueur et des PNJ sont composés sans redimensionnement,
avec les ombres de contact existantes dans une couche séparée. Le contrôle
des pixels opaques est enregistré dans `scene-validation-r02.json`.
La source du décor, conservée dans `assets/source-art/`, est conditionnée au
canevas 1792 × 1152 avant cette composition. Les extraits des quatre pièces
restent à la même échelle, sans zoom.

Le décor conserve l’organisation logique du schéma ; **les coordonnées des murs
illustrés diffèrent du plan r01**. Cette image ne constitue donc pas une preuve
des collisions ou de la circulation du futur niveau. Plan, décor et éventuels
textes du lieu restent à valider explicitement. Aucune acceptation nouvelle
n’est déduite de cette demande d’aperçu.

Retour du joueur sur r02 : accès visiblement murés et mobilier à une autre
échelle que les personnages. **Cette composition ne valide donc pas l’échelle
du décor.** Le contrôle des pixels des acteurs vérifie seulement leur fidélité
aux sources. La prochaine revue doit prendre le corps adulte d’environ 60 px,
et non son canevas transparent de 128 px, comme référence pour le mobilier.
Les dimensions précises des meubles restent à proposer et à examiner.
Voir `scene-feedback-r02.json`.

- `scene-generation-r02.json` : prompt exact, références et méthode imagegen.
- `npc-reference-selection-r02.json` : identités et fichiers des PNJ acceptés.
- `scene-validation-r02.json` : sources, empreintes et vérification des pixels.
- `tools/art/show_handcrafted_camp_scene.py` : composition reproductible.

Pour reconstruire toutes les sorties, exécuter d’abord le script du schéma r01,
puis celui de la scène r02 ; ce dernier rétablit la page courante habillée.
Les images de la scène n’ajoutent aucune dépendance du jeu à `artifacts/`.

## r01 — schéma conservé

17 octobre 2026 · **HANDCRAFTED-PLACES-01**.

Le joueur demande à voir les lieux faits à la main avant validation.
Cette revue commence par **PLACE-CAMP-01 r01**, un candidat pour le premier
refuge. Son plan d’implantation et ses dimensions sont proposés, pas acceptés.

Le dessin du plan est dans `camp-01-r01.txt` ; `review.json` conserve l’échelle,
les accès et les réservations. Le plan utilise un hall central reliant l’arrivée
et l’accès au secteur, avec quatre espaces de services autour. Le point de
voyage est seulement réservé. Les emplacements ne définissent aucun nouveau
dialogue, stock marchand, coût de voyage ou effet.

La page courante `artifacts/handcrafted-places-review-01/index.html` contient :

- Une vue d’ensemble schématique, sans image d’acteur réduite.
- Le même plan à **64 px par case**, avec une image acceptée du joueur copiée
  sans redimensionnement pour comparer les distances et espaces.

Les surfaces unies représentent le plan ; **ce ne sont pas les décors définitifs**.
La création et la revue de ces décors accompagneront l’évolution de la carte.
Les marqueurs de services sont des annotations de revue, pas des modèles de PNJ.
Les futurs lieux obligatoires et arènes suivront le même principe de présentation.

`validation.json` contrôle la connexité des cases de ce schéma, l’accès aux
emplacements et les pixels du joueur à taille native. Il ne valide ni le
rendu du moteur, ni les rencontres, ni la sauvegarde d’une nouvelle carte.

Reproduction : `python tools/art/show_handcrafted_places_review.py`.
Les sorties courantes sont réutilisées ; aucune ressource du jeu n’est remplacée.
