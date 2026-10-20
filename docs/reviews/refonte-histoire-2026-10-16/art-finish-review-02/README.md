# ART-FINISH-02 — personnages et armures

19 octobre 2026. **Lot accepté : « Valider le lot complet ».** Le joueur apprécie le
premier lot accepté et demande de poursuivre la refonte graphique des
personnages, armures et équipements dans cette finition.

## Lot présenté

- FIN-14 : Technomancien — `core:technomancien`, tenue 201.
- FIN-15 : Traqueur — `core:traqueur`, tenue 202.
- FIN-16 : Arcaniste — `core:arcaniste`, tenue 203.
- FIN-17 : Métamorphe humain — `core:metamorphe`, tenue 204.
- FIN-18 : Carapace Chrysalide — `core:carapace_chrysalide`, torse 103.
- FIN-19 : Combinaison fantôme — `core:combinaison_fantome`, torse 105.
- FIN-20 : Manteau de mue — `core:manteau_de_mue`, torse 106.
- FIN-21 : Plastron Bastion — `core:plastron_bastion`, torse 107.

Chaque tenue est présentée sans arme, au Cogneur P01 et au fusil d'assaut :
vingt-quatre configurations au total. Les armes sont celles du premier lot
accepté. Leurs images d'icône ne sont pas étirées pour remplir la main :
le pistolet porté mesure environ 17 px de long, le fusil environ 34 px.

Les autres armes, prises et peintures ne sont pas réputées acceptées par
extension. Ce lot ne propose aucun nouveau nom de contenu, pouvoir, bonus,
type de dégât, statistique, équipement jouable ou changement de sauvegarde.

## Fabrication et composition

Le mode intégré **imagegen** fournit un maître par tenue et un dessin dédié
à sa pose de bras au fusil. La tenue actuelle sert de cible ; Bastion et
Cerbère acceptés servent uniquement de références de finition. Tous les
prompts et dessins sont dans `assets/source-art/art-finish-review-02/`.
Les noms de sortie exacts sont conservés dans `generation-results.json`.

Trois premiers essais sont repris avant la revue : capuche ivoire du
Métamorphe, plaques plus anguleuses pour Chrysalide, casque à visière
horizontale et protection plus massive pour le plastron Bastion. Cela évite
que les deux armures soient de simples copies colorées de Cerbère.
Ces reprises font partie des dessins présentés puis acceptés dans ce lot.

Le conditionnement ne peint pas de membre ou de gant : il extrait les
manches des poses sources et conserve le corps central du maître. Le haut
de la tête et le bas du corps sont enregistrés exactement, pixel pour pixel,
entre les trois configurations. Les zones protégées sont dans `review.json`.
L'arme, les doigts et le corps restent trois couches effectivement séparées
dans le rendu GPU. Les points de prise et le chevauchement des doigts sur
l'arme sont contrôlés sur les exports ; aucune arme n'est collée dans la
peinture du corps.

Les figures entières sont réduites sans étirement dans une toile transparente
de 128 × 128 px, pied à (64,96), pour une case de 64 px et une hauteur humaine
voisine de 60 px. Les marges ne changent pas cette échelle. Le conditionnement
retire le voile alpha parasite des exports sources avant réduction ; Lanczos
conserve des transitions propres à taille native, le moteur reste en Nearest.
Il n'existe pas de nouvelle version d'affichage 32 px.

## Revue dans le moteur

La planche principale montre huit couples actuel/candidat et quatre repères
inchangés : Bastion, Cerbère, un corps civil et le PNJ du camp. Deux autres
planches montrent les vingt-quatre configurations. Les 64 panneaux comparatifs
sont copiés du GPU sans changement d'échelle.

Quatre couples avant/après utilisent les vraies lumières de la rue et du
logement. Les silhouettes de présentation ajoutées à la rue sont dessinées
uniquement ; elles ne rejoignent ni la simulation, ni les rencontres, ni
les sauvegardes. Les empreintes de partie sont vérifiées à chaque capture.
La position, l'heure, les portes, les décors et les lumières sont les mêmes
avant et après. La projection reste celle du camp et de la rue acceptés.

Les détours de revue sont désactivés par défaut et exclus du code release.
Ils lisent seulement les sources dans un mode debug explicite pour comparer
les pixels avec le rendu normal. Celui-ci embarque les 56 exports exacts dans
`assets/sprites/art-finish-02/`, sans dépendance aux sources ni à `artifacts/`.
Les 528 fichiers normaux surveillés par `production-before.json` restent
identiques. Le contrôle du rendu normal est conservé dans `validation.json`.
Le premier lot ART-FINISH-01 garde ses ressources et son accord exacts.

La page courante est réutilisée :
`artifacts/art-quality-audit-01/index.html`. Les planches et comparaisons utiles
sont également conservées ici. L'accord explicite couvre les huit tenues,
leurs vingt-quatre configurations et les quatre comparaisons en carte.
`acceptance.json` fige les empreintes ; les références acceptées sont conservées
sans réécriture. Le plateau et l'inspection utilisent le même fournisseur.
Le mouvement de respiration déjà présent est conservé, pieds fixes.

## Menu de création — complément demandé par le joueur

Les cinq vignettes et le grand portrait de classe passent par les mêmes
dessins de tenue sans arme validés : FIN-01 et FIN-14 à FIN-17. L'ancien
aperçu reconstruisait les prises des armes de départ, qui n'avaient pas été
revues dans ce lot et sélectionnaient encore les anciennes ressources.
L'équipement réel est conservé dans les fiches et à la création de la partie.
`creation-menu-checks.json` et `creation-menu-portraits.png` conservent les cinq
captures du menu normal et leurs correspondances. Aucun nouveau dessin n'est
créé ou réputé accepté par cette correction du menu.

## Reproduction

```text
python tools/reviews/integrate_art_finish_review_02.py
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-02-integration artifacts/art-quality-audit-01/finish02-integration
python tools/reviews/verify_art_finish_integration_02.py
```

`prompts.json`, `pose-prompts.json` et `correction-prompts.json` décrivent
la fabrication avec le mode intégré. Les scripts de revue ne relancent
pas l'outil de génération d'images. Le conditionnement et l'ancienne commande
de revue refusent de réécrire le lot une fois l'accord figé.
`integration-checks.json` compare les 32 panneaux du rendu normal et les quatre
vues éclairées aux références acceptées. `integration-validation.json` conserve
les résultats des tests et de la compilation effectués pour cette intégration.
