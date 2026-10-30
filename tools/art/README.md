# Pipeline visuel actif

Lire `docs/DIRECTION_ARTISTIQUE_TILES.md` avant tout ajout.

Le camp **PLACE-CAMP-01 r09b** accepté est intégré au lancement normal.
Ses ressources utilisées sont dans `assets/maps/cryogenic-camp-01/`, ses dessins
sources et les anciens conditionnements dans `assets/source-art/`.
Le rendu actif est `src/cryogenic_camp_render.rs` avec `src/camp_atmosphere.rs`.
Le dialogue terminal et son 03 provisoire sont partagés par le jeu et le
contrôle `--ui-cold-dialogue-terminal <dossier>`.

`show_camp_review.py` publie l’aperçu compact courant : une vue du camp,
le dialogue intégré, les trois essais sonores et les contours encore en revue.
`prepare_dialogue_audio_review.py` reproduit les sons depuis la trace durable
de revue ; `show_target_outline_review.py` vérifie les captures des contours.
Aucune ressource utilisée par le jeu ne dépend de cette page ou de ses captures.

Les anciens générateurs, maquettes, exemples et lanceurs du camp sont conservés
pour traçabilité sous `tools/archive/handcrafted-places-review-01/`.
Ils ne font plus partie des exemples compilés ni du pipeline actif. Leur README
précise les anciens chemins et les conditions d’une reproduction historique.
Le registre de nettoyage est dans `docs/maintenance/nettoyage-2026-10-18/camp-dialogues/`.

L’étude du monde `study_diablo2_world.py` réutilise les tables de l’installation
locale extraites en lecture seule. `--check` vérifie les résultats dérivés de
PM01, dont 4 192 champs structurels compilés par archive et 81 paramètres de
labyrinthes. Les données tierces restent dans `artifacts/diablo2-study-2026-10-17/` ;
les résultats et propositions sont dans `world-generation-study-01/`. Aucun
plan ni code de Diablo n’est intégré au jeu.

Le bestiaire de campagne courant utilise `build_campaign_bestiary.py` et
`validate_campaign_bestiary.py`. `assets/sprites/campaign-monsters/manifest.json`
fige 129 dessins et 240 variantes natives, auxquels s'ajoutent 231 palettes
de matériaux calculées par le rendu. Treize dessins de lanceurs de sorts restent
en réserve jusqu'à la revue de leurs pouvoirs. Les deux boss de secteur restent
uniquement dans le catalogue de revue. Les 32 px sont abandonnés pour les nouveaux dessins.

Contrôles reproductibles : `python tools/art/build_campaign_bestiary.py --check`
puis `python tools/art/validate_campaign_bestiary.py --after-cleanup`.
`prepare_monster_material_palettes.py --check` vérifie les cartes RGB explicites,
les masques de matériaux et les 180 exports rouge-orangé/verts sans retoucher les dessins sources.
La revue `monster-material-palettes-07` valide les 176 nouvelles palettes sur
88 autres modèles. Elles entrent dans les nouvelles parties de génération 151 ;
les choix de la génération 150 restent identiques. La revue 08 accepte ensuite
51 palettes bleues E, qui rejoignent les 39 modèles ayant déjà une option bleue.
`prepare_monster_blue_palettes.py --check` contrôle ce complément, actif à partir
de la génération 152 ; les choix des générations 150 et 151 sont figés et testés.
Les couleurs ne confèrent
aucun pouvoir. `show_monster_palette_review.py` conserve six planches natives ;
`--ui-cold-monster-palettes <dossier>` contrôle le rendu réel et ses exports GPU.
Le nettoyage a retiré les sorties obsolètes après préservation de leurs exports
utiles ; `cleanup-result.json` en conserve le bilan effectif.
`show_monster_catalogue.py` reconstruit la planche courante de 1620 px de large
à partir des exports durables, sans redimensionner les personnages. Les scripts
historiques de génération mentionnés ci-dessous sont archivés dans
`tools/archive/refonte-histoire-2026-10-16/` ; leurs anciens chemins se retrouvent
dans `docs/reviews/refonte-histoire-2026-10-16/bestiary-integration-06/cleanup-plan.json`.

`show_sector_bosses_review.py` prépare les grands gabarits en revue de
SECTOR-BOSSES-11 : export direct du Maréchal et nouveau dessin de Mère Suture.
Il utilise `sector-bosses-11.json`, place les boss à taille native près d'une
tenue du joueur acceptée et écrit l'aperçu courant dans `artifacts/`.
Il ne remplace aucun PNG accepté, atlas ou fichier du jeu.
Après validation, il réutilise les PNG figés à l'identique ; les révisions de
taille sont des variantes distinctes dont les précédents accords restent conservés.

`build_flat_agent_assets.py` conditionne les tenues frontales BD acceptées dans
ARMOR-FLAT02 et ARMOR-FLAT05. Les 576 cellules de revue sont copiées exactement ;
les corps, doigts et 113 modèles d'armes existants sont conditionnés séparément
avec les points d'attache communs. `assets/sprites/flat-agents/manifest.json`
conserve les sources et empreintes. Le catalogue Rust est généré par ce script.
Depuis ACTOR-RES01, `reviewed-128.png` conserve les 576 exports directs validés
pour les cases de 64 px ; `reviewed-64.png` garde les dessins antérieurs pour
les cases de 32 px. Le canevas porté couvre toujours deux cases, et la résolution
fait partie de la clé du cache. Les placements d'armes ne sont pas recalculés.
`--resolution64-only` conditionne uniquement la banque de résolution validée ;
`--check` contrôle ses cellules et les images de revue 32 px préservées.
`package_agent_class_starts.py` installe les cinq départs déjà validés dans
START-ALL01 et START-GEAR01 ; les rangs initiaux sont distincts des points achetés.

`apply_spectral_step_range_policy.py` conserve SPECTRAL-RANGE01 : la portée de
Pas spectral est doublée et définit un maximum, avec une arrivée plus proche
au choix. Le conditionnement des compétences applique aussi cette décision ;
la fiche source courante est `review-individual-description-T02-03.json`.

`package_agent_skill_window.py` conditionne les 94 fiches du catalogue validé
et vérifie les empreintes des cinquante et une icônes compactes et seize illustrations
acceptées avant copie dans `assets/skills/agent/`. Il ne débloque aucun effet
ou achat. La fenêtre native lit les rangs réels de la partie ; les candidats
non validés restent dans les galeries de revue.

`prepare_agent_artillery_stealth_compact_review.cjs` prépare uniquement les
deux propositions compactes de `SKILLS-IC13` (16 et 20), avec exports et
comparaison aux tailles natives. Il vérifie les sources acceptées et conserve
la galerie précédente, la banque intégrée et le registre de validation.
Après validation, la revue est figée et ce script refuse de la régénérer.
`prepare_agent_compact_batch_review.cjs` prépare les nouvelles séries
compactes, dont `SKILLS-IC14` (21–28) et `SKILLS-IC15` (29–34). Il conserve les empreintes
des exports acceptés, du catalogue et de la banque native, et refuse les
revues déjà validées. Il produit les tailles natives et la galerie sans
écrire dans le registre des validations ni dans les ressources du jeu.
La validation visuelle des huit compactes 21–28 est consignée dans
`skill-icon-review-14/visual-acceptance-01.json`. Elles sont ajoutées à la
banque native par le conditionnement habituel, avec vérification des
dessins précédents et captures par fiche dans `skill-window-native-03/`.
Les captures natives peuvent sélectionner une fiche avec
`--ui-cold-agent-skills-technomancien-960 <nouveau-dossier> --capture-agent-skill=D59`
(ou `SF00`). Cette sélection est réservée au diagnostic des builds debug.

Le joueur valide le 11 octobre la reprise sombre `SKILLS-IC17` : trente
compactes, dont Silence `AF03-C03` avec un vrai bâillon. L'accord est enregistré
dans `skill-icon-review-17/visual-acceptance-01.json` et les anciennes variantes
acceptées restent traçables dans le registre. Les exports 64 px sont copiés sans
retouche dans la banque native. Cette validation ne modifie ni les illustrations
ni les effets ; elle n'étend pas son périmètre aux quatre essais de `SKILLS-IC16`.
Les contrôles et captures de cette intégration figurent dans
`skill-window-native-04/`.

La validation partielle de `SKILLS-IC18` intègre uniquement le Tir de harpon
`DI05-C01` (39), avec contrôle natif dans `skill-window-native-05/`. Les cinq
autres dessins sont refusés. Leurs révisions `SKILLS-IC19` et `SKILLS-IC20`
gardent les numéros 35–38 et 40 et attendent un nouvel accord visuel.

Le joueur valide ensuite les cinq corrections de `SKILLS-IC20` : 35–38 et 40.
Les dessins exacts sont intégrés dans la banque native, avec accord dans
`skill-icon-review-20/visual-acceptance-01.json` et contrôles dans
`skill-window-native-06/`. Les versions refusées restent archivées.

La planche de Survie `SKILLS-IC23` (41–45) est ensuite validée et intégrée
exactement : Égide sans cible, Barrière d’épine, Purge, Dernier sursaut et
Récupération d’énergie. Accord dans `skill-icon-review-23/visual-acceptance-01.json`,
contrôles natifs dans `skill-window-native-07/`.

SKILLS-IC27 valide les compactes 46–49 : Leurre holographique, Traversée de
phase, Réflexes balistiques et Pas spectral. Les quatre exports approuvés sont
copiés exactement ; accord dans `skill-icon-review-27/visual-acceptance-01.json`
et contrôles dans `skill-window-native-08/`.

SKILLS-IC28 valide les cinq compactes de Feu 50–53 et 17 : Souffle de feu,
Mur de feu, Émanation soudaine, Boule de feu et Combustion. Les exports sont
copiés sans retouche ; accord dans `skill-icon-review-28/visual-acceptance-01.json`
et contrôles dans `skill-window-native-09/`. Braise conserve son dessin validé.

`build_bestiary_assets.py` intègre la sélection visuelle validée du 6 octobre :
65 modèles avec leurs cadavres/épaves, dont cinq drones, cinq transformations
et quatre boss natifs de 64 px. Les PNG sont copiés sans changement dans
`assets/sprites/bestiary/` et les atlas, indices stables et contacts d'ombre
sont générés à partir de ces références. Exécuter la commande avec `--check`
pour vérifier les empreintes, les cellules, les 23 correspondances existantes
et la reproductibilité. Le numéro 60 est retiré au profit du 66.
Cette banque n'ajoute aucune règle de jeu ni apparition de nouvelle espèce.

`build_crawl_assets.py` conditionne les sources acceptées : corps natifs de 32 px,
couches portées de 48 px, masques de matériaux, objets et catalogue Rust.
Exécuter `python tools/art/build_crawl_assets.py --check` pour vérifier les sorties.
Les anciens scripts de packaging délèguent à ce point d'entrée.

Les sources et pivots sont traçables dans
`artifacts/crawl-integration-2026-10-05/README.md`. Les IDs et indices proviennent
de son `catalog-source.json` ; les familles sont explicitement sélectionnées
dans `family_for`. Pour un nouveau modèle, ajouter une entrée avec son ID de
contenu, son index et sa prise, puis une correspondance vers une famille existante
ou une nouvelle source examinée à taille native. Ne pas modifier le catalogue
Rust généré à la main, ni sélectionner une prise depuis la portée d'attaque.

Les PNG de matériaux sont l'autorité par pixel pour le runtime : 0 protégé,
1 principal, 2 secondaire. Pour une nouvelle tenue, déclarer ses matériaux,
protéger ses pixels de peau et lumière, puis vérifier toutes les palettes.
Le corps demeure identique entre configurations ; seule la couche portée se
retourne à gauche. Les mains disposent de leur masque de remplacement explicite.

`show_elite_monsters_review.py` conditionne les candidats 20–21 de la nouvelle
campagne depuis leurs grandes sources image_gen conservées, à leurs nouveaux
gabarits d'élites. Il produit la revue native, son zoom entier et les ombres
séparées des variantes de monstres courantes. `review_contact_shadows.py`
reprend uniquement la primitive d'ombre à bandes du runtime ; aucun pixel de
corps n'est peint. Ces scripts ne modifient ni les atlas ni le code du jeu.

`show_rank_and_file_review.py` produit la revue des petits démons `TROUPES-17`
en 64 × 64, avec des corps plus petits selon l'anatomie. Les planches collent
les sprites à taille native uniquement, sans zoom. Chaque variante B est une
correspondance de couleurs indexées sur les mêmes pixels que A. Les ombres
sont séparées. Le magicien possédé 25 est montré sur sa fiche propre ; les
premiers essais humains et de machine écartés restent hors du lot démoniaque.

`show_demon_gallery_review.py` présente une galerie depuis son manifeste et la
correction de taille du 28. La galerie courante est `DEMONS-20`, candidats
60–79, avec les designs appréciés repris dans le style du joueur et des armes.
Reproduction :
`python tools/art/show_demon_gallery_review.py --review demons-style-20.json --output demons-20`.
Les paramètres permettent aussi de relire les essais antérieurs 18 et 19,
dont seuls les dessins effectivement produits sont exportés. Les corps ont
des tailles différentes sur une base de 64 px
par case ; les canevas de 128 px autorisent les débordements. La composition
des planches ne redimensionne aucun acteur. Les palettes B conservent l'alpha
et les contours sombres, les ombres sont séparées et les dessins restent
en attente de la sélection individuelle du joueur.


`show_monster_catalogue.py` rassemble les révisions courantes, y compris les
anciens cultistes et les boss, en une image nommée et codée. Il réutilise les
exports acceptés ou courants, à un pixel pour un pixel, et génère une vue HTML
défilante à largeur fixe de 920 px. Les refus et les briefs sans image restent
hors catalogue. Sorties : `artifacts/npc-monsters-review-01/catalogue/` ;
placements et empreintes : `catalogue-layout-24.json` dans la revue.
`validate_monster_catalogue.py` contrôle les sources, pixels natifs collés,
identités, validations conservées, gabarits et refus supprimés. Le lot 24 se
reproduit avec `show_demon_gallery_review.py --review
cultists-zombies-spirits-24.json --output cultes-24`. Le paramètre facultatif
`saturation_floor` permet de teinter les sources entièrement désaturées ; sa
valeur par défaut de zéro conserve les anciennes correspondances de palette.


Le catalogue emploie `latest_batch`, `catalogue_review_id` et `catalogue_layout`
du registre pour ajouter le lot courant sans écraser le bilan précédent.
Le validateur suit aussi le dernier lot. Pour `NECROS-25`, il compare les 102
modèles précédents et leurs validations à l'instantané conservé, autorise
uniquement le remplacement de la grande planche reproductible, et vérifie
les huit nouveaux modèles, leurs seize variantes et huit ombres séparées.


Le lot `DEMONS-27` emploie `show_codes` et `family_sheet_titles` pour afficher
code, nom et dimensions natives sur ses planches. `combined_preview` assemble
les planches sans redimensionnement ; `native_floor_review` compose les vingt-
quatre variantes A sur trois fonds, avec le joueur à même échelle. Le validateur
contrôle aussi l'identité de chaque pixel opaque dans ces montages. Les anciens
lots gardent leurs sorties inchangées. `expected_new_numbers`,
`expected_catalogue_count` et `preservation_snapshot` explicitent le périmètre
de validation du nouveau manifeste, sans changer les contrôles précédents.

## VERT18-C06 : armes et bestiaire, revue verticale

`pack_vertical_monsters18.py` conditionne les figures entières des 129 monstres.
Son option `--only NUMERO` reprend un modèle sans toucher les autres sorties.
`pack_vertical_weapons18.py` prépare les corps et doigts des huit tenues de C05
pour les prises à une main et les trois familles à deux mains.
`compose_vertical_extension18.py` conserve les armes partagées et leur échelle
fixe pendant tout le cycle ; `verify_vertical_extension18.py` contrôle les
palettes, les casques et les paires compatibles.

`audit_vertical_extension18.py` produit les comparatifs natifs.
`publish_vertical_extension18.py` réutilise l’aperçu courant sous
`artifacts/vertical-walk-review-18/current/extension-C06/`. Le diagnostic Debug
`--ui-cold-vertical-extension` prépare ses ressources depuis `assets/`, puis
dessine les figures dans le vrai passage de finition d’acteur. Le filtre
`RL_VERTICAL_REVIEW_PREFIX` limite une reprise aux identifiants concernés.
`publish_vertical_extension_engine18.py` produit les copies WebP sans perte ;
`--only IDENTIFIANT` reprend une capture. Les PNG et les dessins sources restent
conservés tant que la revue est en cours.

Les outils de composition d’armes utilisent Pillow et NumPy du runtime Python
de travail. Les prompts exacts, la provenance builtin_imagegen, les décisions
et les limites sont dans `docs/reviews/vertical-walk-18/REVUE-C06.md`.
Le catalogue seul reste un aperçu. Après l’accord C06, `install_vertical18.py`
vérifie les empreintes acceptées C05/C06 et installe des atlas compacts, sans
rééchantillonnage, dans `assets/animations/vertical-18/`. Le rendu normal utilise
ces ressources et le service commun `asset_preparation`, avec préparation CPU
avant dessin et création des textures sur le thread graphique. Les sources
de revue et captures ne sont pas distribuées. L’intégration et les limites
de couverture sont consignées dans `integration-C06.json`.

Les tests `game_art::vertical::tests` vérifient la sélection et la conformité
des pixels. `--ui-cold-vertical-integration DOSSIER` vérifie le rendu normal
sans activer de remplacement de revue, les retours frontaux, le cache et le
vrai rendu du camp. Ce diagnostic exige les assertions Debug.

`publish_pistol_orientation18.py` prépare le comparatif C07 du pistolet
horizontal avec les pixels C06 figés ; il ne modifie pas le jeu normal.

`pack_vertical_armor18.py` et `publish_vertical_armor18.py` préparent C10 :
les douze tenues intermédiaires, leurs cycles entiers, six casques partagés
et cinq ports représentatifs à une main. `record_vertical_armor18.py` conserve
les sources Imagegen intégrées et leur provenance. Les prises à deux mains
de ces nouvelles tenues restent une étape distincte après revue des silhouettes.
Le diagnostic `--ui-cold-vertical-extension` accepte
`RL_VERTICAL_REVIEW_CASES=assets/source-art/vertical-walk-review-18/armor-C10/engine-cases.json`
pour cette fixture isolée, sans remplacer les ressources C06/C09 du jeu normal.
`audit_vertical_armor18.py` contrôle les 420 compositions, les retours frontaux,
les quatre groupes dans le vrai camp et les empreintes des banques acceptées.
Périmètre et limites : `docs/reviews/vertical-walk-18/REVUE-C10.md`.

`publish_knife_contact18.py` prépare le comparatif initial C11 sur les vrais
doigts. L'accord du joueur concerne sa face seulement. `publish_knife_back18.py`
prépare la reprise arrière sans modifier les sept cellules frontales validées ;
`audit_knife_back18.py` vérifie les 1 176 copies exactes et les captures natives.
La reprise arrière réutilise des figures entières C12, sans greffe de bras.

`pack_vertical_twohand18.py` et `compose_vertical_twohand18.py` préparent C12 :
36 cycles, ouvertures de prise examinées sur les doigts, suivi commun des deux
attaches et 74 armes partagées par tenue. `vertical_pose_fast18.py` est vérifié
contre le calcul scalaire C08. `audit_vertical_twohand18.py` contrôle 1 104
configurations natives et douze groupes dans le vrai camp. Le catalogue isolé
est `assets/source-art/vertical-walk-review-18/grips-C12/engine-cases.json`, passé
via `RL_VERTICAL_REVIEW_CASES` au diagnostic `--ui-cold-vertical-extension`.
Le catalogue nommé conserve la compatibilité des anciennes listes C06/C10.
`publish_vertical_review_updates18.py` publie les pages après leurs audits.
Les décisions et limites restent séparées dans `REVUE-C11-back-r02.md` et
`REVUE-C12.md`. Après les accords C11/C12, `install_vertical_intermediate18.py`
ajoute douze profils : 74 armes à deux mains, avec sept couleurs et sept états
de tête, et les deux configurations C11 figées en palette d'origine. Le rendu
normal utilise des couches compactes et des compositions C11 exactes sous
`assets/animations/vertical-18/intermediate/`, sans dépendance aux aperçus.
Les huit profils C09 et leurs images sont préservés ; les autres prises à une
main restent sur leur rendu précédent. Les générateurs refusent d'écraser C12
après acceptation. État et vérifications : `integration-C12.json`.
Le choix de conserver A est consigné dans `decision-C07.json` ; le script
préserve cette décision lors d’une republication.

`publish_knife_grip18.py` prépare C08 : couteau actuel, lame légèrement vers
le bas ou vers le haut, en conservant le pistolet A. Les trois compositions
partagent les sources C06, les mêmes mains et points d’attache ; seules les
rotations des couteaux/dagues W068–W079 changent. La page prépare ses atlas
avant l’animation et utilise un cache borné. Ce comparatif n’active aucun
nouveau candidat dans le rendu normal.

Après l’accord C08-B, `finalize_knife_grip18.py` fige les 24 axes de lame
présentés et 112 atlas de référence, sans nouveau dessin, sous
`assets/source-art/vertical-walk-review-18/candidate-C08/`. Il utilise un
échantillonneur bilinéaire scalaire indépendant pour les compositions de
contrôle. `install_vertical18.py` vérifie l’accord et son empreinte, puis
applique seulement les rotations acceptées au manifeste normal. Les 1604
images du lot et les autres prises restent identiques. Les empreintes du
manifeste C06 initial restent conservées ; celles de l’installation C08
sont dans `runtime-assets-C08.json`.

Le test complet de `game_art::vertical::tests` compare les 112 configurations
affectées aux références C08 et les 1479 autres aux références C06. Le test
d’attaches compare aussi tous les champs aux recettes C06, pour préserver
les points de prise, les tailles, les couches et la prise A des pistolets.

`publish_pistol_tilt18.py` dérive C09 du même aperçu pour comparer le pistolet
actuel avec une ouverture rigide de 10° vers l’extérieur. Les deux colonnes
conservent le couteau C08-B. Le script vérifie les banques C06/C08 et les
ressources normales, réutilise leurs images et ne modifie aucun fichier du
jeu. La proposition et les contrôles sont dans `proposal-C09.json` et
`checks-C09.json` ; son activation attend un accord distinct.

Cet accord C09-B est désormais dans `acceptance-C09.json`.
`finalize_pistol_tilt18.py` fige les angles et 168 atlas de référence sous
`assets/source-art/vertical-walk-review-18/candidate-C09/`. Il conserve le
couteau C08-B et reprend l’échantillonneur indépendant de C08, y compris les
six casques. `install_vertical18.py` applique les deux corrections acceptées
dans cet ordre, puis écrit `runtime-assets-C09.json` sans remplacer les
empreintes historiques C06/C08. Seul le manifeste d’angles du jeu change.

Le contrôle complet compare 1599 configurations : 168 références C09,
104 références C08 et 1327 références C06. Il comprend maintenant la paire
104/69 de l’aperçu validé, en plus de 104/70, 104/104 et 68/79. Le diagnostic
normal couvre ces configurations et NPC-01, sans remplacement de revue.
