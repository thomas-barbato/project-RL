# Drones et transformations — revue du 6 octobre 2026

Le joueur a accepté les 55 références du bestiaire précédent à l'exception
du **09, Pillard**, jugé trop médiéval. Les 54 acceptations et les empreintes
des images concernées sont enregistrées dans
`../bestiaire-proposition-2026-10-05/review-decisions-2026-10-06.json`.
Elles restent valables pour ces dessins ; leur intégration ne demandera pas
une nouvelle approbation identique.

Cette revue contient le Pillard repris, six machines destinées au joueur,
cinq transformations et six épaves corrigées de robots ennemis. Les numéros
restent stables entre les révisions ; le nouveau gardien porte le numéro 66.

| Numéro | Nom proposé | Silhouette |
| --- | --- | --- |
| 09 | Pillard | Casque, respirateur et tenue industrielle |
| 56 | Éclaireur | Drone volant avec deux nacelles latérales |
| 57 | Mule | Une caisse sur deux chenilles, sans visage ni bras |
| 58 | Traqueur | Quadrupède mécanique |
| 59 | Artilleur | Drone chenillé avec tourelle et deux canons |
| 60 | Rempart | Châssis industriel et grand bouclier distinct du corps |
| 61 | Colosse biomécanique | Corps massif, plaques ivoire et muscles rouges |
| 62 | Prédateur | Corps élancé, jambes pliées et griffes |
| 63 | Cuirassé | Carapace, ventre segmenté et deux pinces |
| 64 | Possédé | Corps démoniaque, deux cornes et mains griffues |
| 65 | Abysse | Œil unique, torse creux et deux vrilles arrière |
| 66 | Gardien | Robot trapu avec grand bouclier frontal |

Les noms 56–66 sont des propositions. Les fonctions évoquées par leur silhouette
ne créent pas de nouvelles capacités de jeu. Les arbres de talents, statistiques,
règles de transformation et compatibilités d'équipement seront traités plus tard,
conformément à la demande du joueur.

## Examiner la proposition

- [Galerie locale](http://127.0.0.1:8769/galerie-allies-transformations.html), tant que le serveur est actif.
- [Galerie HTML autonome](C:/Users/User/Desktop/project-RL/artifacts/allies-transformations-proposition-2026-10-06/galerie-allies-transformations.html).
- [09 : Pillard avant/après](C:/Users/User/Desktop/project-RL/assets/source-art/armures-bicolores-completes-2026-10-05/allies-transformations-proposes/review/09-pillard.png).
- [56–60 et 66 : les six drones révisés](C:/Users/User/Desktop/project-RL/assets/source-art/armures-bicolores-completes-2026-10-05/allies-transformations-proposes/review/56-60-et-66-drones-r2.png).
- [21–24, 42 et 43 : épaves sèches des robots ennemis](C:/Users/User/Desktop/project-RL/assets/source-art/armures-bicolores-completes-2026-10-05/allies-transformations-proposes/review/21-24-et-42-43-epaves.png).
- [61–65 : les cinq transformations](C:/Users/User/Desktop/project-RL/assets/source-art/armures-bicolores-completes-2026-10-05/allies-transformations-proposes/review/61-65-transformations.png).

La galerie présente les tiles natives 32 × 32, des zooms entiers sans lissage,
le personnage actuel comme repère de taille, les versions mortes/hors service,
plusieurs fonds et des compositions de revue sur les sols du jeu. Ces compositions
ne sont pas des captures après intégration active.

Les transformations sont présentées sans armes pour juger d'abord leur corps.
Le drone volant est placé six pixels au-dessus de la ligne de contact au sol.
Les machines entièrement mécaniques ne saignent pas. Les drones et les robots
ennemis 21–24, 42 et 43 disposent d'une épave sèche, sans liquide substitué au
sang. Les êtres organiques et hybrides avec de la chair conservent leurs
flaques de sang. La règle est aussi inscrite dans la direction artistique.
Les sources et les fichiers de la première revue sont conservés dans
`revisions/r1/`. Les dessins vivants des ennemis mécaniques et des drones 56/58
restent identiques ; seuls 57/59/60 sont redessinés et 66 est créé.

## Fichiers et fabrication

- `sources/` : sorties originales copiées depuis imagegen ; éventuelles révisions conservées séparément.
- `references/` : références du joueur et du style, agrandies au plus proche.
- `tiles/` : les PNG natifs 32 × 32 proposés.
- [prompts.json](C:/Users/User/Desktop/project-RL/docs/reviews/allies-transformations-proposition-2026-10-06/prompts.json) : jeu complet des prompts, références et provenance.
- [revision-prompts.json](C:/Users/User/Desktop/project-RL/docs/reviews/allies-transformations-proposition-2026-10-06/revision-prompts.json) : prompts exacts des douze corrections/créations avec l'outil intégré imagegen.
- `manifest.json` : identités de revue, tailles, points de contact et statut proposé.
- `validation.json` : contrôles des fichiers et conservation des ressources actives.

Les dessins utilisent l'outil intégré **imagegen**, avec une génération par modèle.
Le pipeline de conditionnement réutilise la découpe par sujets complets de la
proposition précédente, puis effectue le cadrage, le redimensionnement au plus
proche et la limitation de palette sans dithering. Il ne redessine pas les corps.

Pour reconstruire la revue, exécuter `build_review.py` avec Python et Pillow.
`prepare.py` a servi à initialiser les références et les empreintes ; ne pas le
relancer sur ce dossier déjà préparé.

Les changements de cette étape concernent les propositions, leurs galeries et
la règle documentaire sur les machines sans sang.
Aucun ID de contenu actif, aucune statistique, collision, sauvegarde ou règle
de talent n'est modifié. La vérification Rust n'est pas requise pour cette étape
qui ne change pas le code du jeu ; les contrôles portent sur les images livrées.

## Contrôles effectués

`validation.json` consigne le contrôle des dimensions, de l'alpha binaire,
des marges, du contenu et des palettes (24 couleurs au maximum). Il compare
les 765 empreintes de référence ; la seule différence documentaire autorisée
est tracée dans `document-revisions.json`. Aucun code ni asset actif du jeu
ne change. Les dessins organiques de cette revue et les dessins vivants non
concernés par une refonte sont vérifiés par empreinte.

Les contrôles de la galerie révisée sont enregistrés dans
`browser-validation-r2.json` et les captures de revue dans `review/`.
Les boutons permettent de vérifier la taille native, les trois fonds,
la recherche par numéro et les scènes. Les captures de la première revue
restent conservées dans `revisions/r1/review/`.
