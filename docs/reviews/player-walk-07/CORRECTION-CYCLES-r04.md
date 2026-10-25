# WALK07-r04 — taille, appuis et directions

Reprise présentée le24 octobre2026 ; avis du joueur en attente.

Le joueur relève un changement de taille du Métamorphe puis une mauvaise
animation des jambes de la Traqueuse. Il signale également que deux nouvelles
planches regardent dans le même sens sur leurs deux lignes. Ces deux planches
ont été écartées ; elles ne deviennent pas des candidats acceptés.

## Diagnostic

La source UNARMED-24-r01 contient des corps coupés à mi-jambe dans sa dernière
ligne : les figures de cette ligne mesurent207–209px contre323–332px pour les
figures complètes. Le conditionnement précédent les traitait comme des corps
entiers. La Traqueuse possède aussi des poses tronquées sur certaines tenues.
Les contrôles d'alpha et de recomposition des bras ne détectaient pas ce défaut
artistique. Leur réussite technique ne validait donc pas les cycles.

Les nouvelles planches à deux lignes ont ignoré la consigne gauche/droite.
La fabrication est désormais séparée par direction, à partir de la pose de
repos propre à cette direction. Une planche qui ignore le sens est écartée.

Enfin, les premiers cycles séparés répétaient trop le même appui. La seconde
moitié est redessinée avec le rôle des jambes inversé. Sur la Traqueuse, le
changement est explicite : la jambe au tibia sombre porte le poids pendant que
la jambe à jambière argentée passe derrière, puis revient devant.

## Deux essais avant de refaire la famille

- WALK07-C11-r04 : CLASS-10, Traqueuse tenue1.
- WALK07-C25-r04 : CLASS-24, Métamorphe tenue5, qui contenait la coupe majeure.

Les deux orientations sont produites et contrôlées séparément. Chaque pose
reste un dessin du personnage entier. La combinaison choisit des dessins
entiers pour les trois premières puis les trois autres phases : aucune greffe
de jambes, aucun étirement d'un axe ou déformation de bandes.

Chaque bande source reçoit une réduction uniforme conservant les proportions,
avec ancrage du torse et du pied au sol. Un contrôle bloque toute figure qui
touche le bord ou toute chute de hauteur dans la bande. La hauteur native varie
d'au plus2px sur le cycle complet. Cette mesure ne remplace pas la revue des
genoux, des pieds et de la fluidité.

Les aperçus de ces deux familles présentent temporairement uniquement ces deux
essais. Les huit autres tenues restent à refaire après contrôle de ces bases.
La présentation ajoute le Bastion accepté, synchronisé sur le même déplacement,
afin de comparer directement les appuis. Les armes réelles restent disponibles.
La durée400ms réglable, les trois phases par case et le repos à l'arrivée restent
conservés. Le jeu reste sur la banque WALK06 déjà acceptée.

## Sources et contrôle

Sources intégrées à la proposition : `assets/source-art/player-walk-review-07/`
avec `UNARMED-10` et `UNARMED-24`, suffixes `r03-left/right` et
`r04-opposite-left/right`. Génération avec l'outil image_gen intégré.
Prompts complets et provenance : `cycle-repair-generation.json` ; références
directionnelles et Bastion : `references/`. Les demandes à deux lignes écartées
sont répertoriées dans `cycle-repair-rejected.json`.

Mesures et ancrages : `cycle-repair-packing.json`.
Cette révision attend l'avis du joueur ; elle n'est pas intégrée au jeu.

Les deux essais couvrent chacun117 configurations réelles, soit234 couples
tenue/équipement et3276 poses. Les exports Rust, avec et sans recettes compactes,
ont réussi. Les six tests du rendu accepté passent. Le contrôle des aperçus
compare204 poses au total, dont les deux essais repris, avec un écart visible
maximal de2/255 par canal. Il retrouve les601 instants du cycle accepté.
L'exécution JavaScript reste un contrôle Node à DOM/canevas simulés ; aucun essai
dans le navigateur ou dans une partie n'est annoncé. Les vues gauches et droites
ont été examinées dans les dessins et les planches natives avant présentation.
