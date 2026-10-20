# ART-FINISH-COMPLETE-06 — finition déléguée

19 octobre 2026. Le joueur demande « je te laisse tous les finires » puis
« et les valider ». Il délègue la poursuite et la validation visuelle des
dessins restants. Cette instruction remplace, pour ce chantier, le passage
systématique devant le joueur avant intégration. Les contrôles visuels et
techniques à taille native restent obligatoires ; leurs preuves sont conservées.

Le travail porte sur la finition des contenus existants : bestiaire et variantes,
configurations d'armes portées et icônes qui jurent encore avec les acteurs
acceptés. Les silhouettes, identités, couleurs de matériaux, tailles et règles
de jeu restent inchangées. Les boss de secteur restent reportés. Les PNJ du camp
déjà cohérents sont conservés. L'audit d'échelle du mobilier suit les acteurs.

`inventory.json` et les journaux de génération permettent de reprendre sans
recommencer les sources terminées. Les nouvelles sources sont produites avec
le skill imagegen et l'outil intégré ; les prompts exacts et chemins de sortie
sont conservés sous `assets/source-art/art-finish-completion-06/`.
Les ressources réellement utilisées vont dans `assets/sprites/`, jamais dans
`artifacts/`. Les cartes de `artifacts/map-editor/` restent intactes.

## Bilan terminé — 20 octobre 2026

Intégré et accepté sous la délégation ci-dessus ; voir [acceptance.json](acceptance.json).

- **119 nouveaux dessins de monstres**, FIN-32 à FIN-151 sauf FIN-106. Le modèle
  113, Étoile des fosses / FIN-106, conserve son dessin accepté après deux refus
  de l'outil de génération ; aucun nouvel essai n'a été poursuivi.
- **129 modèles existants, 471 apparences** : les neuf modèles finis dans les
  lots précédents sont inclus, avec les variantes existantes et les palettes
  rouge-orangé, vertes et bleues. Les variantes conservent la silhouette,
  l'alpha, les pieds et les matériaux secondaires du nouveau dessin.
- **19 tenues et 113 modèles d'armes** raccordés aux configurations de mains :
  main droite, main gauche, paires et armes à deux mains. Trois nouvelles poses
  de bras au fusil complètent les sources déjà acceptées. Les configurations
  exactes acceptées dans les premiers lots gardent la priorité.
- **19 icônes de tenue** reprennent les silhouettes actuelles. Les portraits de
  création déjà acceptés restent raccordés. Les autres icônes d'armes et pièces
  lisibles ont été contrôlées et conservées, sans prétendre les avoir redessinées.
- **Quatre PNJ du camp** comparés au nouveau personnage et au cultiste, conservés
  après contrôle à taille native.

Les identités de contenu, tailles prévues, règles de combat et sauvegardes ne
changent pas. Les treize lanceurs de sorts réservés n'obtiennent pas de nouveaux
pouvoirs ni de nouveaux placements. Les boss de secteur restent reportés.

## Revue et validation

Les preuves montrent les pixels réellement dessinés par le moteur : vingt
planches natives couvrent les 471 apparences, dix-neuf planches de tenues
regroupent **437 configurations**, cinq autres couvrent les **113 armes**.
Huit scènes utilisent l'éclairage réel de la rue, avec des figurants de revue
qui ne sont pas ajoutés à la simulation. Les palettes des dix-neuf tenues ont
été comparées en protégeant la peau et les matériaux secondaires.

**109 126 pixels opaques** comparés aux ressources source : erreur RGB maximale
**0**. Les empreintes de la simulation restent identiques entre les captures.
Les **630 ressources antérieures** suivies dans production-before.json sont
inchangées. Les **965 nouveaux PNG utilisés par le jeu** sont figés dans
[accepted-assets.json](accepted-assets.json) ; les preuves durables sont indexées
dans [proof-hashes.json](proof-hashes.json).

Contrôles réussis sur la version finale :

- `cargo test --locked --bin project-rl game_art` : **46 tests réussis**, aucun échec.
- `cargo build --locked` et `cargo check --locked --release`.
- `cargo fmt --all -- --check` et `git diff --check`.
- Capture native et scènes réelles via `--ui-cold-art-finish-completion`.
- Diagnostic `--ui-cold-performance`, résultats dans [performance-debug.json](performance-debug.json).

La suite complète n'a pas été relancée ; ces résultats concernent les tests
graphiques ciblés. Un échec de suspension sans rapport était connu dans des lots
précédents. Le diagnostic de performance est une mesure **debug**, pas une
comparaison démontrant un gain : les pointes de premier chargement restent
observées. La scène de jeu stabilisée mesure environ 4,4 ms CPU médian ; les
démonstrations de compétences environ 14–15 ms, avec des premiers affichages
sensiblement plus coûteux.

## Audit du mobilier

L'[audit d'échelle](AUDIT_MOBILIER.md) est terminé : les lits des deux logements
de rue sont trop grands, surtout celui du logement nord. Le lit médical du camp
et ses autres meubles sont cohérents avec la taille humaine actuelle. Les cuves
conservent leur encombrement technique. Les longues étagères représentent
plusieurs modules.

Les deux lits devront être remplacés par des objets isolés à la bonne échelle,
puis contrôlés avec leur emprise au sol. Ce lot ne redimensionne ni les maisons,
ni leurs collisions, ni leur éclairage.

## Sources et rangement

Les **122 nouvelles sources actives** (119 monstres et trois poses de bras) ont
été produites par l'outil **imagegen intégré**. Les prompts exacts, références,
chemins et empreintes figurent dans
[prompts-used.json](../../../../assets/source-art/art-finish-completion-06/prompts-used.json).
Les sources et anciens cadrages servant de références de révision sont
conservés sous `assets/source-art/art-finish-completion-06/`.

Les ressources compilées restent dans `assets/sprites/art-finish-completion-06/`.
Le jeu ne dépend pas des captures sous `artifacts/`. La galerie courante contient
les planches compactes et les scènes utiles ; les grandes captures redondantes
et planches intermédiaires sont supprimées après archivage des preuves. Le détail
du rangement figure dans cleanup.json. Les données de `artifacts/map-editor/`
et les anciens dessins encore référencés sont préservés.
