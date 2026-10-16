# Variantes de couleurs — 5 octobre 2026

Mise à jour : cette revue préparatoire est archivée. Les variantes ont depuis été intégrées aux nouvelles parties (génération 145). Voir [l'intégration et sa validation](../palettes-integration-2026-10-05/VALIDATION.md). Les constats de préservation ci-dessous décrivent l'étape de proposition.

Cette revue ajoute des palettes volontairement fantaisistes aux propositions d'armes et aux tenues existantes : rouge, bleu, violet, turquoise, jaune et rose. Les rayures, mécanismes, contours noirs, parties métalliques et ombres conservent le style industriel du jeu.

## Résultat à examiner

- `palettes-armes.png` : quatre modèles avec six couleurs chacun, soit 24 propositions. Cogneur, Brise-ligne, Enfonceur et Fournaise ; une ligne représente un même objet. Les noms des armes restent provisoires.
- `palettes-armures.png` : trois modèles avec six couleurs chacun, soit 18 propositions. Armure Cerbère, robe de la Seconde Ombre et tenue du technicien ; une ligne représente un même objet.
- Les petites figures sous les illustrations montrent la réduction native : armes cadrées en 32 pixels et corps d'armure de 30 pixels dans un cadre de 48 pixels.
- `controle-taille.json` : les 18 corps sont mesurés à 30 pixels.
- Les sources avec alpha conservé sont `armes-palettes-source-v1.png` et `armures-palettes-source-v1.png`.

Les images utilisent **imagegen intégré**. Les prompts exacts sont dans `PROMPTS.md`. Python effectue uniquement les duplications de cibles, découpes, nettoyage d'alpha, changements de taille et mises en page, sans peindre ni recolorer les objets.

## Même objet, couleur individuelle

Le contrat préparé dans `palettes.json` associe une palette à une instance d'objet. La couleur ne crée ni nouveau modèle, ni nouvelle entrée de catalogue, ni nouveau nom. Elle ne change pas les statistiques et ne représente pas la rareté. Les noms colorés de rareté de l'interface restent un indicateur distinct.

Lors de l'intégration, une nouvelle arme ou armure choisira sa palette dans un tirage cosmétique séparé du générateur des propriétés et du combat. La palette sera conservée au ramassage, à l'équipement, après dépôt au sol, voyage, sauvegarde/reprise, amélioration et vente/rachat. Les illustrations d'inventaire, d'inspection, de terrain et les dessins portés devront lire la même valeur. Arme gauche, arme droite et tenue auront chacune leur palette propre : recolorer une tenue ne doit pas recolorer ses armes.

Le rendu devra cibler les zones de tissu ou de peinture ; peau, yeux, métal exposé, contours et ombres restent protégés. Le système couvrira aussi les armes de mêlée et les modèles absents de cette planche. Les trois couleurs de classe sans armure restent celles déjà retenues ; les variations présentées concernent les objets équipables.

## État de cette étape

Cette étape est une **revue visuelle**, dans la continuité du redesign des armes montré avant intégration. Les palettes sont préparées, mais le tirage au butin et sa persistance ne sont pas encore branchés dans le moteur. `palettes.json` est un contrat de préparation, pas un contenu chargé par le jeu.

L'intégration nécessitera une donnée cosmétique distincte des affixes sur les instances et leurs transferts, une compatibilité versionnée des instantanés et du rejeu, ainsi que des vérifications de conservation de couleur et d'absence de consommation du RNG de gameplay. Les anciens objets devront conserver leur palette d'origine par défaut.

Les empreintes dans `preservation.json` confirment que **68 fichiers actifs de graphismes, rendu, inventaire, règles et sauvegarde sont conservés**. Aucun paramètre, objet ou sauvegarde du jeu n'est modifié dans cette revue. Aucun commit ou push.
