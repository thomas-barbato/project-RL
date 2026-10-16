# Redesign des armes à distance — propositions du 5 octobre 2026

Première planche de dix designs originaux dans la direction retenue pour les armes de mêlée : industriel, récupération et militaire. Métal usé, gros mécanismes, plaques et peinture écaillée remplacent les armes propres à lumières cyan.

## Livrables de revue

- `dix-armes-proposees.png` : dix illustrations avec noms provisoires, familles et essais de réduction à 32 pixels. Ces réductions servent à examiner la lisibilité et ne sont pas des icônes de sol intégrées.
- `comparatif-armes-portees.png` : actuel/proposition sur Brèche et Cerbère, dans les poses pistolet levé, fusil à deux mains et deux pistolets. Chaque case comporte un portrait et un aperçu à l'échelle native.
- `dix-armes-source-v1.png` et `personnages-tir-source-v1.png` : les deux sources imagegen, transparence conservée.
- `pistolet-fusil-reference.png` : les deux dessins proposés pour les armes portées communes, Cogneur et Brise-ligne. Les autres familles garderaient ces dessins communs selon leur catégorie, comme demandé, tandis que leurs illustrations d'inventaire pourraient être variées.
- `reference-prise-pistolet-utilisateur.png` : la référence fournie par l'utilisateur, utilisée uniquement pour la prise et l'orientation des mains.

Les noms sont des propositions : Cogneur (pistolet), Crache-fer (PM), Enfonceur (fusil à pompe), Brise-ligne (fusil d'assaut), Long-croc (fusil long), Broyeuse (mitrailleuse), Arc de rupture (fusil à énergie), Fournaise (lance-flammes), Bélier (lance-roquettes) et Fracasseur (lance-grenades).

Les dessins de pistolet et de PM restent des concepts pour les familles futures ; aucun objet correspondant n'est ajouté au contenu actuel. Cette revue propose une direction par famille, pas 61 variantes individuelles déjà prêtes.

## Vérification et état

Le cadrage de la revue utilise le même outil que les figures actuelles : corps de 30 pixels et cadre transparent de 48 pixels pour laisser dépasser les armes. `controle-taille.json` mesure 30 pixels dans chacune des six nouvelles figures. Les PNG de portrait et de taille native sont conservés séparément.

Les images ont été générées avec **imagegen intégré**, puis découpées et arrangées avec Pillow. Aucun dessin d'arme ou de main n'est produit par Python. Les prompts exacts sont dans `PROMPTS.md` et `PROMPTS-PORTEES.md`. `build_previews.py` permet de reconstruire les planches depuis les sources.

Les banques et fichiers de rendu actifs sont vérifiés par empreinte dans `preservation.json`, à partir de `runtime-avant.json`. Cette étape reste une proposition avant validation : aucun dessin actif, paramètre, nom, règle d'équipement ou sauvegarde du jeu n'est remplacé. Aucun commit ou push.
