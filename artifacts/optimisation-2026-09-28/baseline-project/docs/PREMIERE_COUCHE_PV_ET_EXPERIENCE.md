# Première couche : PV et expérience en expédition

28 septembre 2026 — diagnostic en génération 122.

## Question et protocole

Les nouveaux PV et le soin complet à la montée de niveau changent-ils les difficultés observées dans les [essais de génération 120](PREMIERE_COUCHE_PRESSION_ET_EQUIPEMENT.md) ?

Commande reproductible :

```powershell
cargo test --locked --bin project-rl first_layer_level_progression_sample -- --ignored --nocapture
```

Douze parcours fixes : graines 0 et 1, trajets court et long, approches mêlée, distance et évitement. Aucun remplacement des morts. Même kit A et mêmes décisions que le diagnostic historique : couteau de camp, fusil de patrouille, douze munitions, deux soins de six PV, sans veste ni affixe. Les objets rencontrés sur le chemin peuvent être ramassés ; le pilote ne change pas son équipement et ne dépense pas ses points de compétence.

**Limite essentielle : le pilote commence au niveau 1, directement dans la ville de la première couche souterraine. Il ne simule pas la progression préalable en surface.** Le kit est un cas contrôlé, pas un tirage d'équipement de campagne. Le pilote connaît la carte et son trajet, mais ne cible que les hostiles visibles. Ces résultats ne sont ni des taux de victoire de joueurs, ni une validation de l'équilibre de toute la couche.

L'expérience est réellement accordée par le moteur, sans niveau forcé. Le relevé distingue victoires attribuées au joueur, XP, niveau final, maximum de PV, tours de montée de niveau, PV restaurés par ces montées et PV restaurés par les consommables. L'XP et le niveau relevés dans les événements sont comparés à la progression réelle. Avec ces armes sans effet spécial, une restauration doit provenir soit d'une utilisation d'objet, soit d'une action rapportant un niveau ; le diagnostic échoue si cette attribution devient ambiguë.

Les PV minimum sont relevés après chaque commande entièrement résolue, pas à chaque événement interne. Le maximum conservé à la mort est la dernière capacité observée. La reprise au premier contact est comparée par commandes, événements et snapshot ; les passages ne doivent restaurer ni PV, ni munitions, ni consommables.

## Résultats

**Cinq allers-retours complets, un repli et six morts**, comme dans les douze cas du kit A en génération 120. Aucun arrêt technique ni commande refusée. Les durées, dépenses de soins et tirs restent identiques sur cet échantillon. Cela ne signifie pas que les deux règles de progression sont équivalentes.

| Graine | Trajet | Approche | Issue | Victoires | XP | Niveau final | PV finaux / max. | PV rendus par niveau |
|---|---|---|---|---:|---:|---:|---:|---:|
| 0 | Court | Mêlée | Mort | 0 | 0 | 1 | 0 / 20 | 0 |
| 0 | Court | Distance | Aller-retour | 1 | 7 | 1 | 18 / 20 | 0 |
| 0 | Court | Évitement | Aller-retour | 0 | 0 | 1 | 8 / 20 | 0 |
| 0 | Long | Mêlée | Mort | 0 | 0 | 1 | 0 / 20 | 0 |
| 0 | Long | Distance | Mort | 1 | 6 | 1 | 0 / 20 | 0 |
| 0 | Long | Évitement | Mort | 0 | 0 | 1 | 0 / 20 | 0 |
| 1 | Court | Mêlée | Repli | 0 | 0 | 1 | 4 / 20 | 0 |
| 1 | Court | Distance | Mort | 1 | 5 | 1 | 0 / 20 | 0 |
| 1 | Court | Évitement | Mort | 0 | 0 | 1 | 0 / 20 | 0 |
| 1 | Long | Mêlée | Aller-retour | 0 | 0 | 1 | 20 / 20 | 0 |
| 1 | Long | Distance | Aller-retour | 2 | 10 | 2 | 23 / 23 | 9 |
| 1 | Long | Évitement | Aller-retour | 0 | 0 | 1 | 20 / 20 | 0 |

Le seul niveau gagné arrive au tour 308, sur le trajet long de la graine 1, avec l'approche à distance. Le maximum passe à 23 PV et le soin de niveau rend neuf PV. Le personnage termine à 23/23. Le test exige au moins une montée de niveau effectivement accompagnée d'un soin : une exécution sans aucun déclenchement ne peut pas être présentée comme cette validation.

Les pics précédents restent présents : huit PV pour un impact de production, jusqu'à dix PV retirés après une commande impliquant plusieurs sources dans les ateliers. Aucune nouvelle puissance ennemie n'a été appliquée pour ce diagnostic.

## Ce que cela change pour la suite

- La progression des PV fonctionne dans un parcours avec de vrais combats, pas seulement dans un test isolé de montée de niveau.
- Elle ne peut pas corriger un combat perdu avant la première victoire. Les essais de mêlée blessés n'ont gagné aucune XP.
- Inversement, partir artificiellement au niveau 1 sous terre ne justifie pas une réduction générale des monstres de cette couche. Il faut d'abord mesurer l'état réellement obtenu après la surface : niveau, PV, équipement et réserves.
- Le trajet long de la graine 1 reste traversable sans combat pour deux approches. La valeur de ses détours et son intérêt ne sont pas validés par la seule survie.

La priorité suivante est donc un parcours continu depuis la surface, avec les gains et pertes conservés à la descente. Il devra garder les mauvais tirages et les morts, sans accorder un niveau ou un affixe de secours. Les réglages éventuels viseront ensuite les rencontres responsables, en préservant leurs rôles, plutôt qu'un affaiblissement global ou une obligation de trouver du matériel magique.

Ce prolongement dispose désormais d'un [pilote continu utilisant les vrais passages du client](EXPEDITION_CONTINUE_SURFACE_PROFONDEURS.md). Ses conditions de départ et ses décisions diffèrent de ce relevé historique ; les résultats doivent rester séparés.

Cette passe ajoute des mesures et un diagnostic reproductible, pas un rééquilibrage des monstres. Les versions historiques, les seuils d'XP et les règles de campagne restent inchangés. L'exécutable natif a été recompilé avec les règles 122 ; une ancienne suspension conserve toujours ses règles historiques.

## Vérifications

- Douze parcours exécutés, avec contrôle de reprise au premier contact et absence de recharge aux passages.
- Neuf tests moteur de progression des PV et deux tests client de règles historiques et de journal réussis.
- Tests des kits ordinaires et de tir effectif du pilote réussis.
- Compilation native, vérification de toutes les cibles, formatage et contrôle des différences réussis ; les treize groupes du validateur documentaire passent.
- Pas de suite complète relancée pour cette passe de diagnostic. Aucun changement visuel ni contrôle visuel revendiqué.
