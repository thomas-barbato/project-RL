# ART-FINISH-01 — revue groupée de la finition des acteurs

19 octobre 2026. Le joueur demande d'appliquer toutes les recommandations
d'ART-QUALITY-01 puis de montrer l'ensemble pour acceptation ou refus.
**Statut : lot complet accepté le 19 octobre 2026, puis intégré.**
Réponse du joueur : **« Valider le lot complet »**. L'accord porte sur les
treize dessins, six configurations de mains et sept palettes montrés, avec
les deux comparaisons en carte. Les pixels sont figés dans `acceptance.json`.

Le lot pilote recommandé par l'audit couvre toutes ses catégories : Bastion,
Cerbère, trois armures encore historiques (Cognefer, carapace composite et
veste matelassée), deux cultistes, un possédé, deux démons ailés, le drone 57,
un fusil et un pistolet. Les treize codes FIN-01 à FIN-13 sont stables.
Il ne s'agit pas d'une refonte déjà réalisée des centaines de variantes du
bestiaire : les autres familles seront harmonisées par lots.

## Intégration après accord

Trente exports sont copiés sans modification dans
`assets/sprites/art-finish-01/` et embarqués par `include_bytes!`. Le plateau
et l'inspection utilisent les mêmes sources. Le rendu normal ne charge aucun
fichier de `artifacts/` ou `assets/source-art/` pour ces nouveaux dessins.
Le remplacement fonctionne aussi en compilation release.

Les liaisons sont : Bastion (tenue 200), Cerbère (torse 101 / tenue 212),
Cognefer (102), carapace composite (104), veste matelassée (113), monstres
164 A, 165 A, 005 A, 144 A/B/C/D, 145 A/C/D, drone `agent:d57`, icônes
du fusil d'assaut (illustration 2) et du Cogneur P01 (150).

Le lot comprend les cinq tenues sans arme et les prises P01/fusil de Bastion
et Cerbère en matériaux d'origine. Les autres prises et peintures d'équipement
gardent leurs liaisons précédentes : aucune arme portée n'est supprimée ou
remplacée par une pose sans arme. Le bleu supplémentaire 145 E, généré mais
absent des sept échantillons soumis, n'est pas activé par cet accord.

Les armes et les doigts restent des couches séparées en jeu. La composition
d'inspection est calculée depuis ces mêmes couches ; le cache évite de les
recréer à chaque image. La respiration existante garde les pieds fixes.
Les identifiants, caractéristiques, règles de combat et de sauvegarde ne
changent pas. La convention existante de corps dérivé des monstres de campagne
reste appliquée ; le corps mécanique historique du drone reste inchangé.

`integration-checks.json` conserve la comparaison des 26 échantillons et des
deux vues de carte entre la revue figée et le rendu normal. Les références
utiles sont dans `references/`. Les contrôles de compilation et de tests de
l'intégration sont dans `integration-validation.json` ; les bilans ci-dessous
décrivent la préparation historique avant l'accord.

## Fabrication et revue avant l'accord

Les nouveaux maîtres ont été dessinés avec le mode intégré **imagegen**, une
ressource par appel. Les ressources courantes servent de cibles et le corps
civil CIV-U01-04 de référence de matière. Les prompts exacts, sorties et
reprises sont sous `assets/source-art/art-finish-review-01/`.

Les trois anciens torses et le drone sont reconstruits depuis de nouveaux
dessins détaillés. Ils ne sont pas des agrandissements lissés du 32 px.
Les tenues reconstruites gardent leurs identités de contenu et leurs familles
de couleur ; leurs protections et casques réinterprétés doivent être jugés
dans les comparaisons, pas considérés comme des formes déjà approuvées.
Les noms et codes des planches désignent les ressources existantes.

Le premier essai du drone comportait un corps civil copié depuis la référence
de matière. Il a été écarté du conditionnement, puis redessiné isolément en
FIN-11-r02 avec la seule référence du robot. Son image incorrecte n'est ni
présentée ni utilisée. Son résultat et le motif de rejet restent dans le registre.

## Échelle, couches et couleurs

Cases de 64 px. Les humains sont proportionnés à une silhouette d'environ
60 px, les démons gardent leurs grandes emprises et le drone son petit
gabarit (limite de 60 × 42 px depuis sa silhouette historique affichée).
Les grandes toiles de 128 px fournissent des marges et un pied à (64,96) ;
elles ne sont pas ajustées automatiquement pour remplir la case.

Bastion et Cerbère disposent chacun d'un corps fixe, d'une pose de manches
pour le fusil et de couches d'arme/doigts indépendantes. Les parties des
bras sont extraites de dessins de pose dédiés. Aucun gant ou morceau de corps
n'est peint avec des primitives. Le conditionnement vérifie que la tête et
les jambes restent pixel pour pixel identiques entre les configurations.
L'aperçu GPU dessine les couches séparément ; les PNG composés servent au
contrôle, pas à masquer un remplacement de silhouette complète à chaque arme.

Sept couples avant/candidat montrent les palettes des deux démons. Les
masques et cartes de couleurs sont nouveaux et restent soumis à revue.
Les recettes d'auteur existantes sont réutilisées, avec un rouge-orangé
plus distinct du rouge peint pour ce lot. Cornes, griffes, vêtements et
pixels non sélectionnés sont conservés. Aucun statut ou type de dégâts
n'est attribué par ces couleurs.

## Vérification dans le moteur

Le mode debug explicite `--ui-cold-art-finish-review` produit les planches
et deux comparaisons dans le vrai éclairage de la rue. Des figurants de
présentation sont dessinés dans l'une des scènes ; ils ne sont pas ajoutés
au monde simulé. Le personnage présenté est le même Bastion au pistolet
avant/après. Le décor, l'horloge, les portes, la vision et les lumières restent
identiques ; les empreintes de partie sont contrôlées à chaque capture.

Les détours de revue sont désactivés par défaut, absents de la compilation
release, et ne lisent les candidats que depuis `assets/source-art/`.
`production-before.json` fige les ressources normales examinées. Un contrôle
séparé du rendu normal reproduit la planche d'audit initiale à zéro pixel
différent. Les ressources normales et les règles n'avaient pas été remplacées
à ce stade de préparation.

La page courante est réutilisée :
`artifacts/art-quality-audit-01/index.html`.
Elle regroupe les treize codes, six configurations de mains, sept palettes,
les deux scènes et un brouillon de retour par code. Ce brouillon local n'est
pas une validation automatique ; la décision du joueur dans la conversation
a été reçue depuis dans la conversation et inscrite ci-dessus. La page courante
affiche désormais le lot intégré, avec ses références conservées.

Reproduction depuis la racine du projet :

```text
python tools/reviews/integrate_art_finish_review.py
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-finish-integration artifacts/art-quality-audit-01/finish01-integration
python tools/reviews/verify_art_finish_integration.py
python tools/reviews/show_art_finish_review.py
```

Le conditionnement est désormais verrouillé pour ne pas écraser les exports
acceptés. La commande historique de revue est figée ; le contrôle d'intégration
utilise le chemin de dessin normal.

L'extension aux autres familles, aux autres poses et à l'ensemble des modules
d'armure suivra cette finition validée, avec de nouveaux lots. Le lot ne change pas les identifiants,
le comportement des armes, les bonus, les dégâts, les rencontres ou les morts
de combat. Le PNJ et le corps acceptés restent des repères visuels inchangés.

## Bilan de préparation, avant intégration

Compilation debug et contrôle release réussis ; 41 tests graphiques existants
passent. Le contrôle du rendu ordinaire retrouve zéro pixel différent sur
la planche initiale. Les 48 ressources normales surveillées sont inchangées.
Les 52 zones d’image des comparaisons sont copiées sans agrandissement depuis
les captures GPU. Les figures du logement et de la rue ont été inspectées
avec leurs vraies lumières. Les liens locaux, le format du diff et la syntaxe
JavaScript sont vérifiés. Ce bilan ciblé ne prétend pas valider toute la suite
de tests du projet ; les commandes et limites sont dans `validation.json`.
