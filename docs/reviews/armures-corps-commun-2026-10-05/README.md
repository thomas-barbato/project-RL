# Aperçu : corps commun et couleurs secondaires

Proposition du 5 octobre 2026, à examiner avant intégration. Aucun fichier actif du jeu n'est remplacé par cette proposition.

## Ce qui change

Les 17 tenues conservent leur numéro. Le dessin sans arme sert de référence pour les parties fixes : casque, centre du torse et jambes. Les bras, les mains et les armes proviennent des poses existantes. Les armes à deux mains peuvent masquer une partie du torse.

Les couleurs secondaires sont plus claires et occupent des zones plus larges. Exemples : CREUSET vert/bleu, Cognefer cuivre/bleu, membranes olive/bleu et Peau-Seconde rouge/turquoise. Les variantes violettes utilisent un turquoise clair ; les jaunes et roses utilisent un bleu clair. Les noms, statistiques et règles de jeu ne changent pas.

Le corps conserve sa hauteur native de 30 pixels dans une image transparente de 48 pixels. Les armes peuvent dépasser. Cette proposition n'intervient pas dans les collisions.

## Consulter

- `galerie.html` : 17 tenues, sept palettes, huit poses, affichage natif et agrandi.
- `coherence-01-06.png`, `coherence-07-12.png`, `coherence-13-17.png` : chaque tenue avant puis après, avec les huit poses.
- `propositions-*.png` : vues synthétiques des propositions.
- `variantes-*.png` : toutes les couleurs.
- `fiches/01.png` à `fiches/17.png` : chaque tenue avec toutes ses palettes et poses.
- `sols-*.png` : captures Macroquad à taille native, sur béton, métal, grille sombre et sol avec sang.

Dans les comparatifs, « avant » désigne l'aperçu bicolore précédent, pas un état intégré au jeu. La copie `../armures-bicolores-completes-2026-10-05/galerie-coherente.html` est accessible depuis le serveur de revue existant. Son ancienne `galerie.html` est conservée.

## Méthode et limites

Il s'agit d'un essai de rendu déterministe avec les dessins existants, pas de 17 nouveaux dessins générés. `previous.rs` conserve la proposition précédente ; `proposal.rs` ajuste les palettes ; `stable.rs` assemble la référence commune avec les membres et armes de la pose choisie.

Les masques fixes et les exceptions de superposition sont définis dans `stable.rs`, en coordonnées natives. La preuve d'identité des pixels porte sur ces zones précises, hors occlusion. Elle ne garantit pas une segmentation artistique parfaite de chaque plaque située sur les bras ou aux frontières des masques. Ces raccords restent à juger visuellement avant une éventuelle intégration.

Le retournement ne concerne que les membres et les armes dans cet aperçu. Une intégration nécessiterait d'adapter le cache des sprites et le retournement du rendu actif pour conserver cette propriété.

## Vérification

- Compilation et exécution des deux programmes isolés : inspection CPU et aperçu Macroquad.
- 3 808 configurations examinées : 17 tenues × huit poses × sept palettes × deux résolutions × deux orientations.
- Aucune différence dans les zones fixes avec leur référence commune, hors occlusion autorisée.
- Corps natif de 30 pixels conservé ; membres et armes conservés depuis la pose recolorée hors zones fixes.
- Aucun nouveau morceau significatif déconnecté détecté par le contrôle alpha des poses armées en palette d'origine (`controle-connexions.json`).
- 656 fichiers actifs comparés à leur empreinte de départ : aucun changement (`preservation.json`).
- Planches numérotées et captures natives examinées ; galerie ouverte et sélection/pose vérifiées dans le navigateur.

Ces contrôles portent sur la proposition isolée. Ils ne constituent pas une validation de son intégration ni une exécution de la suite complète du jeu.
