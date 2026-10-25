# CLASS-MELEE-GRIP04 — orientation des lames à une main

Le joueur demande de contrôler la prise du couteau et des armes de mêlée
à une main, avec le soupçon que le tranchant regarde le personnage.

Le couteau céramique (modèle 68) et le stylet courbe (79) présentent leur
biseau affûté vers le corps dans le montage courant. Ils sont retournés
horizontalement autour de l’axe de la poignée dans chaque main. Le retournement
intervient après coloration, sur l’arme seule. Les pixels, la taille et les
sources raster restent conservés ; corps, doigts et points de prise restent
identiques. Les autres dessins ne sont pas retournés par une règle générale
de famille : notamment les couteaux dont le tranchant est déjà extérieur et
les dagues à deux tranchants gardent leur orientation.

Le contrôle antérieur `knife_and_greatsword_cutting_edges_face_outward` couvre
la banque historique de 48 px, pas cette banque native active. Un contrôle
supplémentaire porte sur les sources réellement utilisées par les armures de
classe, sur la conservation des pixels et sur les biseaux des deux modèles.

La revue comprend les 27 modèles de mêlée actuellement portés à une main,
dans les deux mains, sur les 25 tenues (1 350 cas). L’export dédié ajoute
les prises sans arme, deux couteaux, deux stylets et les paires couteau/pistolet
dans les deux sens : 1 475 compositions. Le contrôle général conserve aussi
les 113 modèles × 25 tenues × 2 mains (5 650 points d’attache).

La comparaison des 3 100 compositions courantes avec leur état précédent
autorise uniquement les deux lames et les deux configurations qui utilisent
le couteau : 100 changements, 3 000 compositions identiques. Les résultats
effectifs et les captures du moteur sont dans `checks.json`.

La correction concerne le compositeur des cinq classes du joueur. Les
anciennes planches et les sources acceptées restent figées. Aucune règle,
statistique, identité d’objet ni sauvegarde ne change. Le correctif est présenté
à taille native dans la page courante, avec un zoom explicite du tranchant.
Les contrôles techniques ne valent pas validation artistique par le joueur.
