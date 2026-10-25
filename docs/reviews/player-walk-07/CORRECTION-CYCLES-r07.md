# WALK07-r07 — retour aux poses entières du Bastion exact

**Complément courant : [CORRECTION-MATERIAUX-r07f.md](CORRECTION-MATERIAUX-r07f.md).**
Le joueur demande désormais les deux jambes en métal. Les sources et contrôles
de matériau décrits plus bas retracent les étapes historiques ; la correction
r07f remplace explicitement l'exigence d'un seul mollet métallique, avec la
démarche actuelle conservée comme cible. Aucune nouvelle intégration.

Le 24 octobre2026, le joueur rejette r06 : « la tête de la traqueuse n'est plus
dans le bon sens, et ses jambes ne bougent presque plus » ; « la démarche de la
metamorphe ne fonctionne pas ». Il demande de nouveau de s'inspirer du Bastion.

## Cause de la régression et reprise

r06 avait remplacé le mouvement accepté par un petit lever de pied presque de
face, puis tourné la tête de la Traqueuse sans tourner sa silhouette entière.
Les gardes techniques d'asymétrie et de pied décollé ne validaient pas une démarche.
Cette méthode et ces candidats sont rejetés, sans nouvelle intégration au jeu.

Le seul gabarit de mouvement de r07 est désormais le **Bastion du canvas02
original**, rangée7 vers la gauche et rangée1 vers la droite, avec ses six phases
et ses repos. Les images ont été extraites sans exécuter le fichier joint et
sont identiques aux sources conservées (`exact-canvas02-comparison.json`).

Chaque pose complète sert de cible pour dessiner le personnage entier dans la
même orientation. La source canonique propre à la classe fournit seulement
l'identité, l'armure et les matériaux. Une référence de tête sert à préciser
la direction au dessin ; aucune tête n'est collée ou tournée après génération.
Il n'y a ni guide schématique, ni déplacement de bandes de jambes, ni greffe,
ni transformation d'un membre pour simuler le pas.

Les figures sont extraites entières et ramenées au gabarit par une seule échelle
uniforme par direction. Les variations de hauteur native restent au plus2px.
Le bassin, les jambes et les pieds conservent les déplacements de ces nouvelles
poses ; ils ne sont pas ramenés à des jambes frontales immobiles.

## Périmètre de la revue

- **WALK07-C11-r07f** : Traqueuse, tenue1, deux directions, repos et six phases.
- **WALK07-C25-r07** : Métamorphe, tenue5, même couverture.

À gauche : Bastion original «02», taille native. Au centre : essai avec
l'équipement choisi, taille native. À droite : détail×2. Les deux personnages
et le Bastion suivent la même horloge, trois phases par case, avec repos réel.
400ms reste réglable à120/200/400ms. Le panneau d'équipement conserve les117
configurations de revue, dont les113 modèles d'armes réels du jeu.

Les autres huit tenues des deux familles restent en attente de reprise après
examen de ces essais. Le bestiaire et les autres cas d'équipement gardent leur
statut antérieur. Les déplacements verticaux et les cinq atlas WALK06 actifs
ne changent pas ; aucune approbation n'est déduite de la production de r07.

## Sources et prompts

Outil **image_gen intégré**, transfert de poses et retouche locale de matériau,
sans CLI de génération. Sources sélectionnées dans
`assets/source-art/player-walk-review-07/` :

- `BASTION-TRANSFER-10-r07b-left.png` et `BASTION-TRANSFER-10-r07-right.png`.
- `BASTION-TRANSFER-24-r07-left.png` et `BASTION-TRANSFER-24-r07-right.png`.

Le premier transfert gauche de la Traqueuse avait copié l'orange des greaves
du Bastion : `BASTION-TRANSFER-10-r07a-left-material-rejected.png` est écarté.
La retouche suivante restaure uniquement le tissu bleu gris, sans demander
de redessiner le mouvement. Les sources effectivement utilisées et leurs
empreintes figurent dans `source-manifest.json`.

Le joueur signale ensuite que le métal change toujours de jambe dans cette
rangée gauche. Le défaut est localisé aux cellules3,5,7 (repos compris dans le
comptage) : le mollet sous la cuisse portant la poche cargo devient du tissu,
tandis que les cellules voisines montrent la plaque métallique. La source
`BASTION-TRANSFER-10-r07-left.png` est donc retirée à son tour. r07b retouche ce
matériau sur la même chaîne hanche–genou–cheville, avec une cible explicitement
annotée ; la jambe arrière garde le pantalon. Les figures restent entières.
Références et consigne exactes : `prosthesis-job-r07b.json`,
`prompts/CLASS-10-prosthesis-r07b.txt` et
`references/CLASS-10-r07b-prosthesis-guide.png`.

Prompts complets et entrées : `bastion-transfer-jobs-r07.json`,
`bastion-material-job-r07.json`, `prompts/BASTION-TRANSFER-*-r07-*.txt` et
`prompts/BASTION-TRANSFER-10-r07-left-material.txt`. Les cibles exactes sont
`references/EXACT-BASTION-r07-left-motion.png` et la version `right`.
Leurs versions natives conservent les pixels source sans conversion artistique.

## Composition, contrôles et limites

Les armes restent des couches réelles rigides. Les bras sont séparés par
propriété des pixels, sans modifier le dessin entier recomposé. Les jambes
sont protégées par des masques explicites ; aux contacts gant/cuisse, la borne
inspectée des hanches distingue le gant mobile et le bas fixe. Ces masques sont
propres à ces deux essais et ne deviennent pas une règle universelle de découpe.

`bastion-transfer-checks-r07.json` vérifie3276 corps fixes exportés
(117 équipements×14 poses×2 personnages), leur conservation selon l'arme,
les masques de jambes exclus des bras/doigts et les empreintes inchangées des
cinq atlas actifs. Les silhouettes du bas changent effectivement entre les poses.
Ces mesures ne constituent pas une preuve automatique de démarche réussie.
Un contrôle ciblé ajouté pour r07b vérifie la présence de la plaque sur le mollet
identifié dans les sept poses gauche :18 à23 pixels clairs dans sa zone native,
contre0 à1 dans la zone de l'autre jambe. Les cellules fautives n'en avaient
qu'un avant correction. Ce garde de matériau complète l'examen du dessin ; il
ne remplace pas le suivi anatomique ni la revue animée.

Les deux exports Rust de recettes et compositions ont réussi. Les contrôles
JavaScript dans un DOM simulé et les204 compositions comparées restent dans
`renderer-checks.json` ; différence maximale2/255 aux bords filtrés. Le même
contrôleur conserve601 instants de la référence de cadence et les six fragments
restent chacun sous1Mo. Les tests du rendu accepté sont une base antérieure ;
aucune modification Rust de fonctionnement n'a été nécessaire dans r07.

Les planches `CLASS-10-r07-whole-poses-x4.png`, `CLASS-24-r07-whole-poses-x4.png`
et `r07-bastion-and-equipped-pilots-x2.png`, sous `artifacts/player-walk-review-07/`,
ont été examinées image par image, ainsi que les armes à taille native.
La tentative de lecture dans le navigateur intégré a été **refusée par sa
politique d'accès**, car les URL `file:` locales ne sont pas autorisées.
Aucun serveur ni autre navigateur n'a été utilisé pour contourner ce refus.
La lecture réelle en navigateur et l'acceptation de l'enchaînement animé ne
sont donc pas confirmées ; le joueur doit encore examiner la comparaison.
