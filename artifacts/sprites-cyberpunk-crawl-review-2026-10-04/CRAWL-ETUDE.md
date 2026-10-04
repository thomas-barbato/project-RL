# Référence de qualité : Dungeon Crawl Stone Soup

Étude du dépôt officiel le 4 octobre 2026, révision `092a6158924bad4ba6ae303f0cc2b3196c42be04`.

Les huit images examinées sont des PNG de 32 × 32 pixels : un corps humain, une pièce d'armure, un gobelin, un orc, un ogre, un rat, un loup et un serpent. Elles sont enregistrées dans `reference-crawl/` pour l'étude avec leurs sources et les mentions de licence disponibles. Elles ne sont pas utilisées comme sprites de Project RL.

- [Corps humain](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/source/rltiles/player/base/human_m.png)
- [Armure portée](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/source/rltiles/player/body/plate.png)
- [Orc](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/source/rltiles/mon/humanoids/orcs/orc.png)
- [Loup](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/source/rltiles/mon/animals/wolf.png)
- [Définition des pièces du personnage](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/source/rltiles/dc-player.txt)
- [Choix des pièces et de l'équipement](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/source/tilepick-p.cc)
- [Options de taille d'affichage](https://github.com/crawl/crawl/blob/092a6158924bad4ba6ae303f0cc2b3196c42be04/crawl-ref/docs/options_guide.txt)

Le code sélectionne des parties distinctes : corps de base, torse, jambes, bras, bottes, objets dans les mains, casque, cheveux et cape. Le dessin du personnage équipé s'appuie donc sur des pièces prévues pour sa pose, plutôt que sur une icône d'inventaire placée sur son torse. Ce principe reste pertinent pour l'assemblage futur de nos équipements.

Lecture artistique des exemples, et non règle technique universelle de Crawl :

- La silhouette et les différences de valeur entre la tête, le torse, les bras et les pieds permettent la reconnaissance à petite taille.
- Les changements de matière sont exprimés par des groupes de pixels ; les très petits détails ne portent pas seuls l'identité.
- Le visage de face et les membres séparés donnent une lecture claire aux humains. Les monstres de Crawl ont des poses variées ; notre vue frontale commune est une demande propre à Project RL.
- Le fond transparent permet de juger le dessin sur le vrai terrain, à 32 pixels, puis de l'agrandir sans ajouter de détail.

Application dans cette proposition : les trois classes et leurs trois équipements sont retouchés de face, ainsi que les 24 silhouettes du bestiaire précédent. Les couleurs, la morphologie et les rôles sont conservés ; les humains supplémentaires et le zombie demeurent des propositions visuelles. Les objets et l'inventaire de l'essai précédent sont conservés.

Les nouvelles planches sont produites avec imagegen intégré à partir des propositions précédentes. Elles sont affichées à 32 pixels dans `apercu-face.html`, avec une comparaison aux anciens dessins et un catalogue complet. Ce sont des retouches générées de grande taille, pas des PNG natifs 32 × 32 dessinés manuellement pixel par pixel ni un système modulaire d'équipement déjà terminé. La similitude de qualité doit être jugée dans l'aperçu, avant intégration dans le moteur.

Prompts complets : `PROMPTS-FACE.txt` et `PROMPT-FACE-ESPACEMENT.txt`. Planches proposées : `classes-face-v1.png` et `bestiaire-face-v2.png`.

La première planche frontale du bestiaire avait des silhouettes qui dépassaient les cases régulières. La v2 corrige l'espacement ; ses lignes générées restent de hauteurs légèrement différentes. Les rectangles de découpe de `face-bestiary-frames.json` permettent donc de conserver les têtes et les corps complets, avec des proportions préservées à l'affichage. Le contrôle `check-face-atlases.py` vérifie que les parties opaques des 33 sujets ne touchent pas les bordures de leurs rectangles sources.
