# CITY-NORTH01 — plan original de grande ville PLAN09 et témoin de finition 08

Demandes du 5 novembre 2026. **Proposition séparée du jeu normal.**

Le joueur juge r01 trop symétrique, trop étroit et trop proche d'un tracé
américain. Il demande un carré plus large évoquant une ville européenne,
puis des intérieurs identifiables, des murs logiques et des sols variés.
La version r02 reprend ce périmètre. Les données r01 sont conservées comme
source de comparaison sous `assets/maps/north-city-01/history/` ; elles ne
sont pas actives. Leur ancien tracé n'est pas une référence acceptée.

## Validation en deux étapes

Le joueur précise : « j'aimerais bien entendu voir le résultat avant de le
valider :) (ca sera une pré-validation, la véritable validation se fera en jeu ^^ ) ».

- Pré-validation du tracé PLAN09 : **acceptée** le 5 novembre 2026.
- Pré-validation des dessins natifs : **B01 / 10-1 r04 accepté** ; autres dessins en attente.
- Validation visuelle définitive après inspection du joueur en jeu : **en attente**.
- Intégration à la campagne normale : **non effectuée**.

Les tests et les captures automatiques ne remplacent aucune de ces validations.
Après DECOR04, le joueur juge le dessin très inférieur à la première ville et
signale les couvertures extérieures puis leurs volumes incohérents. Le dessin
03–04 n'est donc pas validé. Le joueur refuse ensuite **PAINT05-r04** pour ses
problèmes de perspective. Le joueur juge ensuite la perspective de PROJECTION06
meilleure, demande une correction d’échelle, puis signale des contours noirs
incompatibles avec la première ville. Le joueur refuse ensuite MAP07 pour la
qualité, les perspectives, les décorations et les décors. Voir
`MAP07-USER-REJECTION.json`. La clarification suivante demande une grande ville originale : la première
carte fixe la qualité du dessin, pas son contenu ou son aménagement. La revue
courante présente **PLAN09**, avec 100 bâtiments sur 112 × 112 cases, et le
témoin local de finition **SECTEUR08-r03**. Voir [URBANISME09.md](URBANISME09.md)
et [SECTEUR08.md](SECTEUR08.md). Le petit secteur 08 n'est pas un modèle à
répéter. Les cartes jouables existantes restent conservées, distinctes des
propositions. Le joueur pré-valide ensuite le tracé (« le schéma du tracé
de la ville me plait bien »). Voir `PLAN09-ACCEPTANCE-r01.json`. Les dessins
natifs et la validation définitive en jeu restent à examiner.
La production commence avec B01 sur son emprise conservée. Après le
signalement de perspective incorrecte sur r01, le joueur précise la direction
futuriste, grimdark, sale et abîmée, et demande la reprise de tous les décors
à l'exception des égouts. Le dessin **B01 / 10-1 r04** est ensuite pré-validé
(« oui c'est bien comme ca :) ») et devient une référence de direction pour
la suite, sans intégration à la campagne : [PRODUCTION10-B01.md](PRODUCTION10-B01.md).
Périmètre de la reprise générale : [CYBERPUNK11.md](CYBERPUNK11.md).

## Reprise du dessin — CITY-NORTH01-STYLE03-r01

Le joueur rappelle que le style de dessin diffère de la première map et que
la politique est déjà écrite. Elle existe bien dans
`docs/STYLE_VISUEL_REFERENCE.md`, règle `VIS-COH01-RULE01`, et
`docs/DIRECTION_ARTISTIQUE_TILES.md`. La base générique du prototype ne
respectait pas cette comparaison ; aucune nouvelle règle n'est nécessaire.

La reprise porte sur les contours, les volumes courts, les matières mates,
les surfaces calmes et l'usure localisée. Trois atlas sources sont conservés
sous `assets/source-art/north-city-01/skin-r03/`, avec leurs références et
empreintes dans `manifest.json`. Outil intégré et briefs : `STYLE03-PROMPTS.md`.
Les murs ont des coins en L chanfreinés, des faces séparées des couvre-murs
et des profils de raccord communs aux lignes, coins, T et croix. Le grillage
et les portes utilisent cette même présentation. Tous les objets des six
variantes sont couverts ; aucun repli vers le kit générique n'est conservé.

L'onglet « 03 · Dessin et raccords » montre côte à côte la référence et un
intérieur réellement rendu à 64 px/case, puis les quatre coins reliés à des
murs droits. Les dix vues et six plans ont été actualisés. L'éclairage uniforme
de la fixture n'est pas l'éclairage chaud final de la référence. Le personnage
et les corps existants restent les mêmes ressources ; cette retouche ne les
redessine pas.

À cette étape STYLE03, les dix JSON de carte et le modèle de simulation avaient
exactement les empreintes de r02 : tracé, identifiants, placements, portes, collisions, décors et choix
des sous-sols sont conservés. Trois tests graphiques supplémentaires vérifient
les profils, l'égalité RGBA des raccords et la couverture de tous les objets et
variantes. Les quatre tests moteur passent toujours. Preuves : `VALIDATION-r03.json`.

## Décor continu et circulation — CITY-NORTH01-DECOR04-r01

Le joueur refuse les murs incomplets, les bandes de sol exposées du mauvais côté
des cloisons et les sols sans logique. Il demande un vrai décor dans lequel se
déplacer. Cette reprise conserve les dessins sources, les objets, leurs positions,
les pièces et les variantes, et remplace la construction des surfaces et murs.

Le fond architectural est maintenant préparé comme un décor continu : chaque
sol appartient à sa pièce, se prolonge jusqu'à la face de ses cloisons et suit
les seuils. Les frontières visibles ne sont plus découpées à 8/56 px autour de
murs de 28 px. Les sols sont ancrés dans leur pièce ; le parquet et les dalles
de circulation ont une échelle intérieure. Les toits mitoyens rejoignent leur
façade, sans bande de parquet extérieure. Les portes fermées ont la profondeur
de leurs murs, et leur état ouvert reste celui du moteur.

La capture du bâtiment entier a également révélé **six murs réellement absents**
dans les données : `(28,92)`, `(29,92)`, `(30,92)` sur B01 et `(62,54)`, `(63,54)`,
`(64,54)` sur B05. Ces seuls six enregistrements ont été ajoutés au JSON de ville,
avec des identifiants stables `north_city:wall:DECOR04:X:Y`. Aucun ancien élément
n'a été supprimé ou déplacé. En retirant seulement ces six enregistrements, on
retrouve les octets exacts du fichier r02/r03, fins de ligne comprises. Les neuf
autres JSON et le modèle moteur sont inchangés. Preuve : `REPARATIONS-r04.json`.

`decor04-logement.png` montre B01 entier à **1536 × 960 px, 64 px/case**.
`decor04-parcours.webp` est un assemblage sans perte de **50 captures successives**
du moteur : entrée, cuisine, chambre, séjour, salle d'eau, retour dehors. Chacune
suit une commande de déplacement adjacente réelle. Les portes sont ouvertes par
interaction, les collisions sont celles de `WorldState` et aucune téléportation
n'est utilisée dans ce parcours. Les cinquante images décodées de l'animation
ont été comparées pixel par pixel aux captures originales. Le cadrage en coupe
sert à inspecter l'ensemble des pièces ; ce n'est pas une validation du brouillard
de guerre ou de l'animation du joueur.

Pour l'essai interactif : **`Apercu_logement_nord.cmd`**, ou
`Apercu_quartier_nord.cmd --logement`. Le personnage commence devant la porte
est de B01. Le quartier entier reste accessible. La pré-validation visuelle et
la validation finale par le joueur en jeu restent ouvertes.

Les fonds natifs sont préparés par morceaux proches de 512 × 512 px dans le
cache graphique commun borné de 128 Mio. Leurs bords n'altèrent ni le dessin ni
les collisions : un test compare ces morceaux au rendu d'une toile continue et
exige l'égalité exacte des pixels, murs et sols compris. Les lectures/décodages
et la composition se font sur le worker de préparation, avant affichage ; les
textures sont créées sur le thread graphique. La palette CPU du peintre est unique
et vérifiée sous une limite fixe de 12 Mio ; les sources décodées gardent le cache
commun de 64 Mio. Le registre de plans de cette revue est borné à neuf zones et
ne contient pas d'état de jeu. Les sauvegardes n'en dépendent pas.

Onze tests ciblés passent : quatre de navigation/état, trois de raccords/couverture
des dessins, quatre de surfaces et d'architecture. Les façades de tous les bâtiments
visitables sont contrôlées, ainsi que leurs portes. Les dix vues natives et les six
plans de sous-sols sont actualisés. Captures, limites et empreintes : `VALIDATION-r04.json`.

Pour actualiser l'aperçu courant :

```text
cargo run --locked --example north_city_review -- --seed 20261105 --capture artifacts/north-city-01
python tools/assemble_north_city_walk.py
python tools/show_north_city_review.py
```

L'assemblage de l'animation conserve les pixels et supprime uniquement ses images
intermédiaires identifiées après comparaison. Les sources et données de carte
restent dans `assets/` ; aucune dépendance du prototype à `artifacts/` n'est ajoutée.

## Tracé proposé

Le quartier fait **112 × 112 cases**, à 64 px par case, soit 40 % de largeur
supplémentaire. Deux passages
réciproques reprennent les coordonnées de la rue demandées par le joueur :

- rue `(18, 1)` ↔ quartier `(41, 108)` ;
- rue `(18, 2)` ↔ quartier `(41, 109)`.

Les rues ont des largeurs variées, des décalages et des embranchements.
Neuf ensembles de façades mitoyennes prolongent les îlots existants, avec
des cours ouvertes par des passages. La petite place des commerces est
décentrée ; les voies étroites ne reçoivent pas de ligne centrale. Les
rares marquages blancs suivent la route large effectivement dessinée.
Le parcours rejoint la caserne au nord-est, à l'entrée prévue `(99, 4)`.
Cette entrée est une sortie de conception, **sans intérieur de caserne ni
transition vers une zone inexistante**. Elle n'est pas présentée comme une
caserne terminée. Les rues secondaires bouclent entre les îlots ; les bords
de toutes les cartes sont fermés. Les anciennes coordonnées de sortie
`(41,4)` appartiennent uniquement à r01.

Les dix repères de bâtiments de l'aperçu restent stables :

- B01 `south_west` : immeuble éventré visitable ;
- B02 `south_east` : logements abandonnés visitables ;
- B03 `apartments` : immeuble d'habitation visitable ;
- B04 `collapsed` : tour effondrée inaccessible ;
- B05 `shop_home` : commerce et logement visitables ;
- B06 `fire_station` : caserne de pompiers, garage et locaux ;
- B07 `school` : école, salles et cour ;
- B08 `police` : accueil du commissariat et accès aux autres niveaux ;
- B09 `checkpoint` : ancien poste militaire ;
- B10 `sealed_block` : immeuble incendié inaccessible.

Les corps adultes civils, les corps en équipement et le sang réutilisent les
ressources existantes de la rue. Ils n'ajoutent aucune collision ni règle.
Les petits cadavres de monstres restent à dessiner et à soumettre ; aucun
corps humain n'est renommé en monstre pour simuler cet ajout. Le peuplement,
le butin et l'équilibrage ne font pas partie de cette première fixture.

## Intérieurs r02

Les dix identités B01–B10 sont conservées. Les bâtiments ont été déplacés
avec leurs portes et décors dans le nouveau quartier. Les deux ancres
urbaines des escaliers du commissariat suivent son déplacement ; les ancres
internes des autres niveaux et le mécanisme de choix des variantes restent
identiques.

36 pièces sont identifiées par des repères stables :

- logements : entrée, chambre, cuisine, séjour et salle d'eau ;
- école : deux classes, couloir, salle des professeurs et sanitaires ;
- pompiers : dortoir, vestiaires, circulation et remise/atelier ;
- commissariat : bureaux, briefing, attente/dépôt, accueil et cage d'escalier ;
- boutique et poste militaire : surfaces de vente ou de contrôle et réserves.

Les pièces de vie et les classes utilisent le parquet, les cuisines et
sanitaires les carreaux ivoire, les distributions l'ardoise, les ateliers
et réserves le béton. Les portes de distribution sont de vraies portes du
moteur. Les marges transparentes au pied des murs reprennent le matériau du
bon côté, pour éviter de faire déborder le parquet dehors ou dans une pièce
carrelée. Les masses de toiture bloquantes ne participent plus comme de
fausses rangées de jonctions au dessin de leur façade.

Les identifiants des 113 objets et 129 décors de rue sources sont conservés.
Le mobilier supplémentaire complète les usages. La reprise ciblée de l'école
concentre les bureaux et les chaises dans les classes et la salle des
professeurs ; un casier des pompiers a été replacé dans le vestiaire.

Les sols de l'étage et des six sous-sols ont également été repris. Leurs
murs, objets, passages, variantes et sels de tirage sont strictement
conservés ; seule la couche des sols et sa description changent. Les six
plans emploient la même famille de matériaux de service et de circulation.
Les comparaisons de données sont dans `CHANGEMENTS-r02.json` et `SOLS-r02.json`.

## Sous-sols à variantes — POLICE-LAYOUT01

À la demande du joueur, le commissariat comprend deux sous-sols avec **trois
agencements de 36 × 28 cases par niveau**. Le RDC et l'étage restent fixes.
Les variantes partagent leurs matériaux, leurs modèles et quantités de mobilier
et de décors ; les cloisons, couloirs et placements varient. Les accès verticaux
gardent des ancres communes. Le sous-sol −2 est accessible depuis le −1.

La graine de partie et un sel propre à chaque étage choisissent la variante
**lors de la construction initiale du monde**, avant toute visite. Le choix
est indépendant du générateur de combat et de butin. L'identifiant de la zone
contient le numéro retenu (`review:police_basement_layout01`, par exemple) et
fait partie du véritable état `WorldState`, y compris dans sa sauvegarde avant
la première visite. Visiter, quitter ou restaurer n'effectue aucun nouveau
tirage. Les cartes, portes et obstacles sont conservés indépendamment du cache
graphique. Les obstacles statiques dérivés sont reconstruits après lecture de
la sauvegarde, conformément au format de carte existant.

Cette construction est active dans le prototype ; son raccordement au flux
de création de personnage de la campagne attend l'intégration après revue.
Les identifiants `review:` n'entrent pas dans les sauvegardes du joueur.

## Retouches locales — EDIT-SCOPE01

La politique générale est consignée dans `AGENTS.md`. Les cartes sont des
documents éditables distincts sous `assets/maps/north-city-01/`. Les bâtiments,
murs, meubles, passages et décors ont des identifiants stables. Le manifeste
`review.json` décrit les bâtiments, décors, liaisons et variantes. Il est lu
au lancement ; une retouche de données ne demande pas de nouvelle génération.

Les deux générateurs refusent de remplacer un candidat existant : leurs sorties
deviennent les données de travail faisant autorité. Il ne faut pas supprimer
ces sorties pour relancer un générateur après une inspection.

Les migrations r02 sont elles aussi ponctuelles et refusent de se relancer
sur une version déjà retouchée. Les changements ultérieurs doivent viser
les identifiants existants, sans reconstruire le quartier entier.

`tools/edit_north_city_review.py` permet de lister une famille, d'inspecter un
identifiant et de préparer le diff d'une seule cible. `--apply` applique ce diff.
Les propriétés imbriquées sont fusionnées : modifier un décalage conserve
les dimensions, collisions et autres propriétés. Chaque niveau aléatoire
exige son numéro `--variant` explicite. La preuve de conservation compare
l'ensemble du document avant/après annulation de cette seule retouche.
Les contrôles de navigation du prototype doivent suivre une modification
de collision ou de structure. Un changement d'emprise de bâtiment nécessite
une retouche cohérente des données physiques et de présentation concernées,
sans régénérer les bâtiments voisins.

## Aperçu et essai

- `Apercu_quartier_nord.cmd` : exploration isolée dans le moteur ;
- `Apercu_quartier_nord.cmd --seed 42` : choix d'agencements d'une autre partie ;
- `artifacts/north-city-01/apercu.html` : plan repéré, dix vues à taille native,
  six plans de sous-sols et comparaison avec la rue validée ;
- `tools/show_north_city_review.py` : actualise cette même page après de nouvelles
  captures, sans toucher aux cartes.

Commandes : flèches/ZQSD pour marcher, `E` pour un passage, `Tab` pour le plan,
clic sur le plan pour parcourir un chemin avec les commandes du moteur, `R`
pour afficher les intérieurs de revue, `Échap` pour fermer. La fixture expose
volontairement le quartier pour inspection et ne valide pas le brouillard de
guerre, les animations, le combat ou une sauvegarde de campagne réelle.

Les captures se régénèrent avec :

```text
cargo run --locked --example north_city_review -- --seed 20261105 --capture artifacts/north-city-01
python tools/show_north_city_review.py
```

## Contrôles réalisés

Quatre tests ciblés passent avec
`cargo test --locked --example north_city_review north_city_tests -- --nocapture` :

1. Raccords exacts `(18,1)` et `(18,2)` dans les deux sens, accès aux vues et à
   l'entrée prévue de la caserne, sans faux passage vers une caserne absente.
2. Allers-retours aux niveaux du commissariat, portes conservées, restauration
   d'état et des obstacles, chemin principal possible sans faire le donjon.
3. Choix reproductible par graine, variantes sauvegardées avant toute visite,
   et validation moteur sur 48 graines couvrant les six plans.
4. Accès effectif à chaque pièce meublée et présence de plusieurs matériaux
   adaptés aux fonctions de chaque bâtiment visitable.

Les captures du moteur montrent dix lieux à **64 px/case** et six plans
complets à échelle réduite. L'aperçu HTML et son changement d'onglet ont été
vérifiés dans le navigateur. L'essai manuel par injection de touches n'a pas
pu être terminé : l'outil n'a pas pu activer la fenêtre du prototype. Aucun
autre jeu ouvert n'a reçu d'entrée. La validation manuelle du joueur reste
donc explicitement ouverte.

Les nouvelles images et leurs assemblages sont préparés sur un worker puis
envoyés au GPU avant affichage. Le kit générique n'est plus chargé. Les
textures de carte, du personnage et des décors emploient le
service partagé `asset_preparation` : lecture/décodage sur worker, envoi GPU
sur le thread graphique, cache de sources de 64 Mio et cache de textures de
128 Mio. Ces budgets locaux existants ne sont pas une validation d'un budget
global de mémoire. Les mesures r02 restent dans `VALIDATION-r02.json` et celles
de STYLE03 sont dans `VALIDATION-r03.json`. Les coûts lecture,
décodage, création de textures et dessin sont séparés dans `checks.json`.
Ces temps CPU locaux ne garantissent pas une cadence GPU ni une distribution.

Les sources de la campagne, sa génération, la distribution, les préférences,
les sauvegardes et les données préexistantes de l'éditeur ne sont pas modifiées.
