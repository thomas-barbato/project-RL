# VERT18-C03 — armures lourdes et armes à une main

Premier lot équipé, préparé le 29 octobre 2026 après validation de C02.
**Lot validé visuellement par « je valide :) », désactivé dans le jeu normal.**
Les fichiers exacts et les limites de cet accord sont dans `acceptance-C03.json`.
L'objectif demandé par
le joueur couvre toutes les versions avec armure, armes et avec/sans casque ;
le suivi complet reste dans `COUVERTURE-EQUIPEMENT.json`.

## Ce qui est montré

1. Bastion, tenue 04 : plaques cuivre et acier.
2. Traqueuse, tenue 14 : protection intégrale bleu argent.
3. Arcaniste, tenue 19 : robe rouge renforcée, capuche et masque d'origine.
4. Métamorphe, tenue 24 : protections crème, feuilles et col de fourrure.

Chaque tenue possède six phases de face et six vrais dos. Les figures sont
dessinées entières, sans fabrication de jambes par bandes ou cisaillement.
Les repères de main appartiennent à chaque phase. Les armes gardent leur
dessin partagé, sont déplacées vers ces repères et restent séparées du corps.
La main de premier plan reprend les pixels exacts de la figure, sans déplacer
une manche ou un morceau de vêtement indépendamment.

Le comparatif présente les quatre tenues sans arme, avec le pistolet 104,
avec deux couteaux 70, deux pistolets 104 et la paire 104 / 70. Quand on passe
dans le dos, les objets changent de côté à l'écran tout en restant dans la
même main anatomique. L'arrivée utilise le compositeur frontal habituel et
conserve les objets équipés. La cadence est celle du cycle validé : 400 ms
par case, trois phases par pas, appuis alternés.

Les prises des 39 modèles à une main sont vérifiées dans chacune des deux
mains, sur les douze phases de chacune des quatre tenues. Ce contrôle de
contact ne remplace pas la revue visuelle des silhouettes et recouvrements.
Les cinq configurations présentées sont capturées dans le moteur. La paire
pistolet/couteau est aussi montrée dans deux séquences du camp, avec ses
matériaux et ses lumières réels.

## Limites et étapes restantes

Le lot conserve les **protections de tête d'origine** de ces quatre dessins
d'armure. Un emplacement de casque additionnel vide ne signifie donc pas
encore une vraie variante non casquée. Il reste à dessiner et contrôler cette
variante, puis les six casques équipables de face et de dos. Les capuches et
les parties de vêtement doivent rester cohérentes avec ces changements et
avec le dessin de repos. La précision « casquées / non casquées » du joueur
est obligatoire pour chaque tenue, pas seulement pour les tenues légères.

Les autres niveaux d'armure, le Technomancien conservé pour les sauvegardes,
les palettes applicables et les prises à deux mains restent à traiter.
Ce lot ne valide pas l'application d'une prise à une main à une arme à deux
mains. Les PNJ conservent leurs tenues propres ; les armures de classe sont
réservées au joueur. L'accord C02 ne valide pas ces nouveaux dessins C03.
La proposition DUAL17-C02 reste séparée.

## Reproduction et fichiers

```powershell
python tools/art/pack_vertical_walk_equipment18.py
cargo build --locked
& ./target/debug/project-rl.exe --ui-cold-vertical-equipment artifacts/vertical-walk-review-18/current/equipment-C03/engine
python tools/art/publish_vertical_walk_equipment18.py
```

Sources : `assets/source-art/vertical-walk-review-18/*-lourd*-r01-source.png`.
Les quatre fichiers sont issus d'**imagegen intégré**, avec transparence,
en conservant les vêtements du catalogue. Les prompts, références et
empreintes sont dans `prompts-C03.json` ; les dessins conditionnés, leurs
repères et leur échelle uniforme sont dans `candidate-C03/manifest.json`.
La découpe suit les séparations transparentes réelles, y compris quand
elles sont plus étroites que dix pixels. Elle ne coupe aucune silhouette.

L'aperçu est dans `artifacts/vertical-walk-review-18/current/equipment-C03/`.
Les animations WebP sont sans perte et leurs pixels décodés sont comparés
aux images capturées. Les pauses de lecture de 650 ms sont propres à l'aperçu.
Les caches communs restent bornés à 4 Mio pour les sources et 8 Mio pour les
figures préparées. Chaque configuration est préparée avant sa capture, avec
création des textures sur le thread graphique et sans lecture/décodage dans
le dessin. Aucune sauvegarde de campagne ni préférence utilisateur n'est écrite.
