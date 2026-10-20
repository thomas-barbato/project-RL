# Audit du décor U01 r04

**AUDIT-U01-04-01 — 19 octobre 2026.**

Périmètre : candidat `MAP-U01-EXP-01-r04` présenté dans l’aperçu courant.
Inspection du plan complet, des dessins à taille native, des captures du moteur,
des placements, de la grille et des scripts de fabrication. Les empreintes des
sources correspondent au manifeste r04. Aucun décor ni code du jeu n’est modifié
par cet audit ; seuls ce rapport et ses extraits de diagnostic sont ajoutés.

**Conclusion : les trois défauts signalés par le joueur sont confirmés.**
La composition reste à reprendre avant validation artistique. La continuité des
rues et les contrôles de déplacement ne suffisent pas à valider les raccords
graphiques et les limites visibles de la scène.

## Défauts prioritaires

### 1. P1 — Des accessoires emportent des morceaux de leur ancien environnement

**Confirmé.** Dans la cour de l’atelier, la machine et la benne proviennent de
découpes qui contiennent encore leur sol, des végétaux, des morceaux de mur et
des accessoires voisins. Le contour coupe cet environnement de façon nette.
La benne paraît ainsi entourée d’un fragment de scène rapporté ; la machine
conserve un socle rectangulaire dont les dalles ne se raccordent pas au sol commun.

Ce défaut explique directement une partie de l’impression « découpée ». Il
existe dans les pixels du plan complet, indépendamment du brouillard de visibilité.

**Cause :** extraction par polygone opaque dans
[build_street_unified_review.py](C:/Users/User/Desktop/project-RL/tools/reviews/build_street_unified_review.py:105).
Ce masque conserve tout le contenu à l’intérieur du polygone, au lieu d’isoler
la silhouette de l’objet. Les sélections elliptiques de végétation et certains
bords de bâtiments doivent aussi être revus.

**Correction recommandée :** détourer les objets seuls ; construire le sol et
les bordures dans le repère de la carte. Redessiner les éléments incomplets
plutôt que dissimuler leur coupe par un autre tas de débris.

![Découpes et raccords dans la cour de l’atelier](C:/Users/User/Desktop/project-RL/artifacts/street-expansion-review-01/audit-r04/accessoires.png)

### 2. P1 — Les quatre angles de l’enceinte ne sont pas construits

**Confirmé.** Les murs horizontaux et verticaux s’arrêtent ou se croisent sans
véritable pièce de coin. Leurs épaisseurs et leurs faces ne forment pas un
volume continu, particulièrement visible dans les deux angles sud.

**Cause :** le périmètre répète des segments droits du camp ; aucune pièce de
coin n’est placée dans
[build_street_unified_review.py](C:/Users/User/Desktop/project-RL/tools/reviews/build_street_unified_review.py:173).
La fermeture logique de la grille existe, mais son dessin est incomplet.

**Correction recommandée :** quatre coins adaptés à leurs orientations, avec
épaisseur, face visible et contour raccordés aux segments droits. Contrôler
les quatre angles à taille native, avec et sans éclairage.

![Les quatre coins de l’enceinte](C:/Users/User/Desktop/project-RL/artifacts/street-expansion-review-01/audit-r04/coins-enceinte.png)

### 3. P1 — Une rue existe encore derrière le portail de la prison

**Confirmé dans l’image et dans la carte.** Au sud du portail, une bande de
chaussée, un trottoir et un second mur restent visibles. Les rangées logiques
63 et 64 contiennent chacune 23 cases de sol : **46 cases sont encore
praticables derrière le point d’arrivée**, situé en `[7, 62]`.

La rue source a été conservée avec cette bande arrière. Sa grille a également
été recopiée dans
[build_street_unified_review.py](C:/Users/User/Desktop/project-RL/tools/reviews/build_street_unified_review.py:205).
Le rendu applique le brouillard normal ; aucun masque propre à l’arrière de
la prison ne retire ce décor de la scène.

**Correction recommandée :** faire du portail la limite sud de la scène
extérieure. Retirer le décor situé derrière et aligner sa visibilité et ses
cases accessibles sur cette limite. Préserver le passage de retour vers le
camp et vérifier l’arrivée côté ville. Cacher uniquement l’image laisserait
un espace de déplacement invisible et ne résoudrait pas complètement le défaut.

![Bande de rue conservée derrière la prison](C:/Users/User/Desktop/project-RL/artifacts/street-expansion-review-01/audit-r04/arriere-prison.png)

### 4. P1 — Certains dégâts ne sont pas intégrés au volume du bâtiment

**Confirmé.** Sur la première grande maison de l’extension, le morceau de
béton posé au milieu du toit ressemble à un objet déposé sur une couverture
intacte. Les lignes du toit continuent autour du motif ; aucun bord réellement
effondré ne relie les deux. Certaines fenêtres et certains rideaux métalliques
rapportés ne remplacent pas clairement une ouverture existante.

**Cause :** superposition de motifs génériques dans
[build_street_aftermath_review.py](C:/Users/User/Desktop/project-RL/tools/reviews/build_street_aftermath_review.py:271).
Le positionnement est contrôlé dans l’image, mais pas par la structure du
bâtiment représenté. Les motifs sont également répétitifs : le même trou de
toiture apparaît cinq fois, le même pan de mur effondré cinq fois.

**Correction recommandée :** fabriquer les dégâts comme des variantes du
bâtiment concerné : lignes de couverture interrompues, bord arraché, structure
exposée et gravats reliés à l’effondrement. Conserver une couche de toit séparée
pour les bâtiments visitables.

![Motif de destruction posé sur un toit intact](C:/Users/User/Desktop/project-RL/artifacts/street-expansion-review-01/audit-r04/toit-rapporte.png)

### 5. P1 — Des ajouts recouvrent des luminaires encore actifs

**Confirmé par les placements et les silhouettes opaques.** Quatre intersections
avec des zones de lentille sont détectées :

| Ajout | Luminaire concerné | Recouvrement opaque mesuré |
| --- | --- | ---: |
| Rideau métallique du garage ouest | `west-garage-door` | 147 pixels |
| Pan de mur près de l’atelier | `post-4` | 37 pixels |
| Décombres devant le bâtiment sud-est | `east-south-door` | 329 pixels |
| Même amas, sur le bord du lampadaire voisin | `post-5` | 3 pixels, marginal |

Les sources lumineuses restent actives aux anciennes positions. Le calcul du
masque d’émission peut aussi rencontrer les nouveaux pixels des gravats à la
place de la lentille. L’éclairage réel du moteur est présent, mais certains
montages ne correspondent plus à un appareil visible et correctement fixé.

**Correction recommandée :** protéger l’emprise du luminaire complet lors du
placement ; si un appareil doit être détruit, modifier ensemble son dessin,
son masque et son émetteur. Revoir la cour de l’atelier et la façade sud-est.

## Défauts secondaires et points à revoir

### 6. P2 — Quelques ruines suggèrent un accès que la carte ne permet pas

**Confirmé pour le fonctionnement ; jugement visuel pour sa lisibilité.** Les
six nouveaux bâtiments montrent des pièces et des brèches, mais sont traités
comme des emprises bloquées, sans intérieur jouable ni porte interactive.
La lumière rouge renseigne la porte ; elle n’explique pas toujours pourquoi
une grande ouverture dans le mur est infranchissable.

Rendre les brèches condamnées lisibles par leur propre géométrie : décombres,
structure encore fermée ou obstacle évident. Aucun nouvel intérieur n’a besoin
d’être inventé pour corriger cette ambiguïté.

### 7. P2 — Le sol et la répartition des dégâts gardent un effet de montage

**Appréciation artistique.** Les épaves et les nouvelles maisons améliorent
nettement la densité. Cependant, les dalles très régulièrement répétées restent
souvent intactes jusqu’au bord d’un impact isolé. Certains amas n’ont pas de
lien visible avec le mur ou l’infrastructure censés les avoir produits.

Il faut relier les dégâts à leur origine : rupture de bordure, prolongement
d’une fissure, façade manquante correspondant au tas voisin. Varier les grandes
silhouettes de dégâts au lieu de multiplier le même motif.

Un export mérite également une reprise de sa découpe : le lampadaire abattu
`WAR-U01-07` touche encore le bord gauche de sa sélection source sur une
portion opaque de 13 pixels. Ce constat ne signifie pas que tous les nouveaux
sprites transparents sont mal détourés.

### 8. P2 — Éclairage trop systématiquement en service

**Ajout demandé par le joueur :** moins de luminaires, dont certains éteints.
Le manifeste du candidat configure **37 sources ponctuelles, toutes actives**.
Ce nombre inclut aussi les intérieurs et la voiture ; il ne correspond donc
pas à 37 lampadaires. Visuellement, la répétition des halos et des façades
alimentées donne au réseau électrique une régularité peu cohérente avec les
destructions représentées.

**Direction recommandée :** supprimer quelques appareils redondants, garder
plusieurs luminaires éteints ou endommagés et regrouper les lumières restantes
de façon irrégulière. Un appareil éteint conserve son boîtier sombre, sans
émission de lentille ni halo ; retirer uniquement sa source ne suffit pas si
son dessin reste lumineux.

Conserver les repères fonctionnels jaunes et rouges des accès, ainsi que la
lisibilité du départ et des passages. Réduire surtout l’éclairage d’ambiance
redondant ; les témoins de porte peuvent rester lisibles avec un halo plus
localisé. La lumière ambiante évite de transformer les secteurs éteints en
surfaces illisibles. Cette direction ne fixe aucun nombre final de sources
et n’ajoute pas de système de panne ou d’interrupteur.

## Ce qui peut être conservé

- Le tracé continu de la rue et sa progression depuis l’ancien départ.
- La densification apportée par les six nouveaux bâtiments et leur direction
  de destruction, à harmoniser avec les anciens modules.
- La projection orthogonale, l’échelle de jeu de 64 px et les épaves limitées
  à 128 × 64 px.
- La pénombre et le système d’éclairage moteur ; revoir les appareils
  recouverts, leur montage, leur densité et leur état allumé ou éteint.
- Les portes jouables, les couches de toit et le retour au camp, à revérifier
  après la reprise de la limite sud.

## Ordre conseillé pour la correction

1. Refaire les limites : arrière de la prison et coins de l’enceinte.
2. Nettoyer les découpes et raccords des modules existants.
3. Reprendre les dégâts architecturaux, les luminaires recouverts et la
   répartition des appareils conservés, supprimés ou éteints.
4. Harmoniser le sol, les amas et la lecture des brèches condamnées.
5. Présenter de nouveaux gros plans natifs et contrôler le passage de retour.

Les tests de déplacement précédemment réussis restent des preuves de
fonctionnement du candidat testé ; ils ne valident pas sa finition artistique.
Aucune nouvelle campagne de tests n’est nécessaire pour ce rapport sans
modification du jeu.

Les extraits et mesures sont dans
[observations.json](C:/Users/User/Desktop/project-RL/artifacts/street-expansion-review-01/audit-r04/observations.json).
