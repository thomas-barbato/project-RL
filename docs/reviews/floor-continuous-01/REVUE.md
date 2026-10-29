# SOL-CONT01-r01 — sol continu du camp et de la rue

Statut au 28 octobre : extérieur 02 validé et intégré ; intérieur en correction.
Le caillebotis 03 a été refusé. Le béton corrigé 04 est présenté dans
[INTERIEUR.md](INTERIEUR.md).

Demande : réduire le damier du camp et supprimer le quadrillage décoratif à
l'extérieur, parce qu'il suggère de fausses cases. La précision du joueur est
« oui pas de cadrillage du tout à l'exterieur. » Aucune variante de petites
dalles extérieures n'est proposée.

01 désigne le rendu actuel ; 02 désigne le sol continu proposé. Les cinq
comparaisons sont des captures du moteur, à 64 px par case logique, sans
redimensionnement : arrivée du camp, galerie du camp, arrivée en ville,
place et galerie urbaine. Le personnage, la caméra et l'heure de présentation
sont identiques dans chaque paire.

## Fabrication et sources

- Matière : [material-source.png](../../../assets/source-art/floor-continuous-01/material-source.png),
  produite en mode **outil intégré imagegen**, avec le prompt exact conservé
  dans [PROMPT.md](PROMPT.md). La sortie originale 1254 × 1254 est conservée.
- Les outils Python conditionnent cette matière, conservent l'alpha et la
  lumière large des sols existants, puis recomposent les modules natifs.
  Ils ne dessinent pas de nouveaux bâtiments, personnages ou objets.
- La matière continue s'étend sur plusieurs cases de 64 px ; les rotations
  par case de l'ancien compositeur sont compensées pour empêcher les raccords
  de faire réapparaître une grille.
- L'asphalte de l'extension réutilise la source acceptée `ROAD-U01-09-native.png`,
  conditionnée avec des bords raccordables.
- La reconstruction de la rue actuelle est vérifiée identique pixel par pixel
  avant substitution des sols. Son arrivée sud est une ancienne image complète :
  sa sélection de sol suit les contours visuels, avec les objets et bordures
  protégés. L'entrée existante des égouts conserve ses différences de contenu.
- Les fichiers candidats sont sous `assets/source-art/floor-continuous-01/`.
  Les sources r01 sont figées après accord. Les trois images extérieures
  acceptées ont été copiées à l'identique dans les ressources de la partie normale.
  Les deux couches originales du camp restent actives en attendant la revue 04.

Les bordures de trottoir, marquages routiers et contours d'objets restent des
limites physiques du décor. Le quadrillage visé est celui qui découpait la
surface du sol en faux carreaux réguliers.

## Exécution et contrôles

Ancienne commande de capture r01 (le générateur r01 est désormais figé) :

```powershell
.\target\debug\project-rl.exe --ui-cold-continuous-floor artifacts/floor-continuous-review-01
python tools/art/publish_floor_continuous_review_01.py
```

Les captures sont prises par le rendu de production, avec ses matériaux et
éclairages. L'option de sol candidat est désactivée au démarrage, ne s'active
que pendant les captures et est absente du binaire release. Les captures
vérifient une simulation inchangée, aucune modification du HUD hors pointeur
et des cases natives de 64 px. Aucune sauvegarde existante n'est ouverte et
aucune préférence n'est enregistrée.

L'aperçu courant est [apercu.html](../../../artifacts/floor-continuous-review-01/apercu.html).
Les résultats détaillés et les empreintes des fichiers sont dans
[validation.json](validation.json) et [conditioning.json](conditioning.json).

## Validation visuelle attendue

Le 28 octobre, le joueur répond « oui je valide » et signale des pieds de
décor effacés et des restes de damier à l'intérieur. L'extérieur accepté est
intégré ; ses captures dans le rendu normal reproduisent les trois vues natives
validées. Les originaux et références sont conservés dans les sources.
L'entrée des égouts reçoit une nouvelle image nommée d'après son empreinte ;
son ancien fond reste conservé. Aucun tracé ni collision n'est modifié.
Accord : [acceptance-r01.json](acceptance-r01.json).
Preuve d'intégration : [exterior-integration.json](exterior-integration.json).

La suggestion de caillebotis est ensuite refusée par « bon je suis pas fan de la
grille, désolé. » Les essais 03 sont retirés, avec leur décision conservée dans
[interior-r02-rejection.json](interior-r02-rejection.json). Le retour au béton
ne modifie pas l'accord sur l'extérieur.
