# EG-EAU01-r01 — eau et berges des égouts

Suite du 28 octobre : le joueur apprécie l'eau et demande plus de canaux,
reliés en un seul réseau avec des passages praticables. La proposition courante
est décrite dans [EG-CAN02](CANAUX02.md). L'aperçu animé courant est réutilisé ;
la référence de l'eau r01 est conservée sous `references/water-r01-native64.*`.
Le joueur valide ensuite les tracés et exige de préserver la mobilité du joueur
et des monstres. L'accord est dans `acceptance.json` ; l'intégration en génération
167 et la compatibilité des anciens plans sont décrites dans `CANAUX02.md`.

Demande du 27 octobre 2026 : améliorer les égouts et animer leur eau avec un
mouvement très simple. Proposition préparée dans le moteur, sans activation
dans la partie normale, conformément à la revue des nouveaux effets dans
`AGENTS.md`. Aucun nouvel effet mécanique ou texte de jeu n'est proposé.

## Comparaison native

**01 — rendu actuel.** Eau acceptée, avec son léger défilement existant et sa
découpe intérieure de 56 × 56 px par case de 64 px.

**02 — EG-EAU01-r01.** Même texture source, avec un courant lent d'environ
1,35 pixel natif par seconde, une oscillation latérale inférieure à un pixel et
de petites rides lumineuses. La teinte est un vert sombre moins chargé de
contrastes fins. L'assombrissement au bord situe la surface dans son canal.
Les cases voisines se raccordent sans conserver de berge artificielle entre
elles. Les berges externes de quatre pixels et les ponts opaques restent fixes.

Les positions des obstacles, les plans, l'éclairage existant, les décors et les
dessins de personnage sont conservés. Les lampes, rats et personnage sont
figés dans ce comparatif pour isoler l'eau. Les vues sont des cadrages de revue
du vrai renderer à taille native, sans interface complète ni simulation de combat.
Le mode « mouvement réduit » conserve une surface immobile.

## Aperçu à examiner

Depuis la racine du projet :

```powershell
tools/reviews/show_sewer_water_review_01.cmd
```

Touches 1, 2 et 3 pour les trois variantes ; M pour le mouvement réduit ;
Échap pour fermer. Le programme de revue n'ouvre pas de campagne sauvegardée
et n'écrit pas de préférences.

La comparaison animée courante et les trois planches natives se trouvent dans
`artifacts/sewer-water-review-01/`. La commande de capture est
`--ui-cold-sewer-water artifacts/sewer-water-review-01`, puis le script
`tools/art/publish_sewer_water_review_01.py` encode l'aperçu. Les images gardent
leurs pixels natifs ; le lecteur HTML permet le défilement sur un écran étroit.

## Contrôles

Les captures des trois variantes conservent exactement leur état sérialisé.
La comparaison des deux panneaux ne relève aucun pixel modifié hors des cases
d'eau : ni acteur, ni pont, ni objet, ni mur ou sol sec. La comparaison animée
contient 80 images à 20 images par seconde, sur quatre secondes, en 1 360 × 672 px.
Chaque panneau mesure 640 × 448 px, avec des cases de 64 × 64 px.

Deux captures prises à des temps différents en mouvement réduit sont identiques
à l'octet près. Les 292 454 400 octets du flux RGBA temporaire sont retirés par
le publieur après vérification de l'encodage. Les sorties de capture courantes
sont réutilisées ; aucune nouvelle ressource de jeu n'est requise.

Les résultats de cette première proposition sont conservés dans `validation.json`.
L'accord ultérieur sur l'eau et les tracés reliés est dans `acceptance.json`.
Le bilan de leur intégration au jeu est dans `integration-validation.json`.
