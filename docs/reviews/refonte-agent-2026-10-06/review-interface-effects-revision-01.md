# Affichage futur des effets sur le personnage

6 octobre 2026. Le joueur propose d'ajouter plus tard un espace du HUD
pour les buffs et debuffs qui affectent le personnage. Il n'est pas encore
intégré au jeu ; l'emplacement et le rendu doivent être présentés avant
intégration.

### HUD01 Icônes de buffs et debuffs

> Consultez les effets qui vous affectent et leur durée restante.

**Confirmé :** afficher des icônes des buffs et debuffs du personnage.
Passer la souris sur une icône montre le nom, l'effet et le temps restant
en nombre de tours. Le joueur évoque le haut à droite de la zone de jeu
comme emplacement possible, sans fixer ce placement.

**Proposition de contenu :** le tooltip emploie le nom validé de l'effet,
sa description courte et ses valeurs actuelles lorsque pertinentes,
puis la durée restante. Il doit afficher le temps réel de l'effet, sans
créer un nouveau compteur indépendant du combat. Consulter une icône
n'avance pas le temps ; les menus ne font pas décroître cette durée.

L'icône disparaîtrait à la fin de l'effet. Un état permanent ou conditionnel
ne reçoit pas un faux décompte : une indication telle que « Permanent »
ou sa condition pourrait le remplacer. Le choix des états permanents
à afficher, les cumuls et la priorité de lecture sont à définir avec
la maquette du HUD, sans obliger à afficher chaque passif appris en continu.

Les icônes, noms affichés, textes, couleurs et emplacement attendent
une revue visuelle. Ce besoin est prévu pour plus tard ; la discussion
actuelle reste centrée sur les compétences à retenir.
