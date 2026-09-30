# Ressources des armes

## Première intégration, génération 123

Les armes à projectiles utilisent une réserve commune de **munitions**. Les tirs
consomment directement cette réserve : aucun calibre à trier et aucun chargeur
à remplir manuellement. Les munitions sont une pile dans l'inventaire,
conservée lors des voyages et dans les sauvegardes. Un emplacement supplémentaire
compense la pile de départ.

Valeurs provisoires :

| Ressource ou arme | Réglage |
| --- | --- |
| Munitions au départ | 40 |
| Fusils de patrouille, de guetteur, à induction et à nerf tendu | 1 munition par tir |
| Lance-aiguilles | 1 munition et son coût énergétique existant de 5 par tir |
| Lance-flammes | 3 munitions par tir |
| Fusil de parallaxe | 6 énergie par tir |
| Fusil de l'horizon fendu | 8 énergie par tir |
| Récupération d'énergie | 1 par tour terminé, limitée à la capacité du joueur |
| Munitions récupérées sur un ennemi compatible | 2 à 6 |

Les modificateurs énergétiques existants continuent de s'appliquer aux coûts.
La récupération concerne la réserve d'énergie commune, donc également celle
employée par les compétences. Attendre permet de récupérer, mais fait avancer
le monde et les ennemis. Un menu, une commande refusée ou un chargement ne donne
aucune énergie. La récupération ne ranime pas un personnage mort.

## Butin et règles de dépense

Les ennemis hostiles dotés d'un système électronique, ainsi que ceux portant
effectivement une arme à projectiles, laissent une petite pile au sol à leur mort.
Le joueur doit la ramasser. Elle peut coexister avec l'arme lâchée et le butin
déjà présent sur la case. La faune ordinaire et les personnages non hostiles
ne produisent pas ce butin. Une même mort ne produit qu'un seul tirage.

Une salve paie chaque projectile. Une réserve insuffisante refuse l'action
avant de dépenser le temps, les ressources ou l'aléatoire. Changer d'arme ou
équiper une autre copie ne remplit pas la réserve.

Le HUD affiche les munitions restantes et le coût du tir ; la fiche d'arme précise
aussi ce coût. Cette tranche n'ajoute ni artisanat, ni stock de marchand dédié,
ni gestion de munitions pour l'intelligence artificielle.

## Compatibilité

Les nouvelles parties utilisent la génération 127. Les sauvegardes 122 et
antérieures conservent leurs anciens chargeurs et leur absence de récupération
automatique. L'adaptateur retire uniquement les nouvelles règles, les métadonnées
de ravitaillement, le matériau et sa pile initiale. Le snapshot conserve son
format : les munitions utilisent les piles d'inventaire existantes.

La génération 124 avait réduit la récupération du fusil de parallaxe à une
action au lieu de 40. La génération 125 supprime cette récupération pour les
deux fusils énergétiques (parallaxe et horizon fendu) : chaque tir coûte son
énergie et un tour, sans action d'attente ou déplacement imposé entre deux tirs.
Les anciennes sauvegardes conservent leur durée pour préserver la relecture ;
le laboratoire utilise toujours le fonctionnement actuel.

La génération 127 fournit aussi les 40 munitions communes après l'application
du paquetage de classe, qui remplaçait auparavant l'inventaire générique et
supprimait cette réserve. Ce complément n'est ajouté qu'une fois, ne remplace
pas un stock explicitement défini par la classe et ne remplit pas une ancienne
sauvegarde au chargement. L'aperçu d'une attaque de zone vérifie désormais ses
coûts : le cône reste visible, mais indique « MUNITIONS INSUFFISANTES » ou
« ÉNERGIE INSUFFISANTE » si le tir ne peut pas être confirmé.

Le nom affiché devient « Munitions », sans renommer l'identifiant interne
`core:weapon_matter`. La jauge sous l'arme représente une réserve de munitions
sur une échelle visuelle par tranches de 40 (ce n'est pas une capacité maximale),
ou l'énergie sur sa capacité réelle. La récupération restante est visible.

Les anciens diagnostics de survie restent explicitement sur les chargeurs
historiques. Ils ne constituent pas une validation d'équilibrage de cette
nouvelle économie. Les coûts et quantités devront être ajustés en jouant.
