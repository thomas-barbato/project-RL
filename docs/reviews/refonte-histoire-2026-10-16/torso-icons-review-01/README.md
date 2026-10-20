# TORSO-ICONS-01 — icônes de torse intégrées le 20 octobre 2026

Le joueur a accepté le lot par « ok , on peut les integrer :) ». Les
quatorze objets de torse 101–114 utilisent leurs nouvelles icônes dans le
loot au sol, l’inventaire, les emplacements d’équipement et les fiches.
Équiper le torse sélectionne toujours la tenue entière du personnage.

L’inventaire initial avait omis cinq vrais objets équipables de départ.
L’affirmation précédente selon laquelle ces cinq tenues n’étaient pas des
objets était incorrecte. Elles ont fait l’objet d’une revue et d’un accord
distincts : [TORSO-STARTING-02](../torso-starting-review-02/README.md).
L’ensemble intégré comprend **19 modèles et 93 associations icône / tenue**.

## Dessins et palettes acceptés

Le premier lot conserve ses **14 modèles et 69 associations**, dont
55 nouvelles couleurs. A est l’original, B le bleu ardoise, C le vert mousse,
D la rouille et E le rouge sombre. La Robe de la Seconde Ombre TOR-109 est
déjà rouge en A, sans doublon E. Le complément de départ apporte cinq
modèles et 24 associations ; la Tenue rituelle TOR-203 garde aussi son rouge A.

Les 19 dessins représentent le haut d’un vêtement ou une protection de
buste, sans personnage miniature. Les sources imagegen et les [prompts
exacts](../../../../assets/source-art/torso-icons-review-01/prompts.json)
restent conservés. Les PNG d’objet sont natifs64, avec silhouette entière
et marges transparentes. Les **112 fichiers A–D** du premier lot ont été
préservés lors de l’ajout des rouges, comme consigné dans
[red-amendment-02.json](red-amendment-02.json).

Les quatre nouvelles palettes appliquent 90 % de la rampe acceptée et
10 % du pigment original au seul matériau principal. Les panneaux
secondaires, matières neutres, peau et armes restent protégés. Les
anciennes palettes et leur interpolation restent inchangées ; leurs
indices sérialisés sont préservés. Les quatre indices supplémentaires
sont ajoutés à la suite. Aucun nom, identifiant d’objet, bonus, statistique
ou règle de combat n’est modifié.

Les nouvelles couleurs rejoignent le choix cosmétique des seuls 19 torses,
en conservant les couleurs antérieures. Le tirage utilise toujours un seul
appel au générateur cosmétique, indépendant de celui du combat et du loot.
La couleur suit l’instance d’objet lors du ramassage, de l’équipement,
de la sauvegarde et de l’abandon au sol.

## Ressources et contrôles

Les **114 PNG utilisés à l’exécution** sont dans
`assets/sprites/torso-icons-01/` : 93 icônes, 19 cartes de matériaux d’icône
et deux masques de tissu pour FIN-01. Le catalogue Rust les embarque ;
aucune ressource du jeu ne dépend de `artifacts/` ni des sources imagegen.
Les empreintes sont dans [runtime-assets.json](runtime-assets.json), les
pixels acceptés dans [accepted-candidates.json](accepted-candidates.json).

[integration-checks.json](integration-checks.json) consigne les tests ciblés,
la compilation, les comparaisons de pixels et les captures. Ces contrôles
couvrent les 93 icônes, les 74 nouvelles couleurs portées et la conservation
des armes, de la transparence et des matières protégées. La suite complète
du projet n’est pas revendiquée.

La [galerie courante](../../../../artifacts/torso-icons-review-01/index.html#integration)
montre les captures réelles : 19 planches moteur à 64/40/28 px, six
configurations de mains par association, six scènes éclairées avec objet
au sol et six inventaires avec fiche d’objet. Les parties de diagnostic
sont isolées et leur capacité d’inventaire est augmentée uniquement pour
les contrôles. Les empreintes de partie restent identiques pendant le rendu.

Les planches historiques de validation sont conservées, avec leur statut
de candidature d’alors. Elles ne sont pas présentées comme des captures
de la version intégrée. Les sources et règles durables restent sous
`assets/` et `docs/`, les sorties consultables sous `artifacts/`.
