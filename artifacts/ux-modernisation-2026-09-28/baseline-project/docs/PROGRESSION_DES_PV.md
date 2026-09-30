# Progression des points de vie

Gain de PV validé le 27 septembre 2026 (génération 121). Soin complet à la montée de niveau validé le 28 septembre 2026, pour les nouvelles parties de génération 122.

Chaque niveau après le premier apporte **3 PV maximum**. Ce gain est permanent et indépendant de la Résilience et des bonus des objets équipés. Le profil actuel commence toujours à 20 PV : sans changement de Résilience ni d'équipement, il atteint 32 PV au niveau 5 et 41 au niveau 8.

La montée de niveau restaure entièrement les PV après l'augmentation du maximum : **12/20 devient 23/23**. Plusieurs niveaux gagnés à la fois s'additionnent avant le soin. Un gain d'expérience sans nouveau niveau n'apporte aucun soin. Un personnage détruit ne revient pas à la vie. Le soin complet fonctionne aussi lorsque le maximum a atteint son plafond numérique. Il ne recharge pas les autres jauges et ne retire pas les états : les dégâts ultérieurs du même tour peuvent toujours blesser le joueur.

La Résilience conserve son apport actuel de cinq PV maximum par point, par rapport à la valeur de référence, ainsi que son rôle dans la Stabilité. Les bonus de PV des objets restent supplémentaires. Retirer ou remettre un équipement recalcule le maximum sans soigner et sans perdre les PV dus au niveau.

Le gain de niveau est dérivé du niveau enregistré : il n'est pas ajouté au corps ou aux objets. La règle moteur `player_hit_points_per_level` vaut trois à partir de la génération 121. `player_full_heal_on_level_up` active le soin complet à partir de 122. Les parties de génération 121 conservent leur ancien gain partiel (12/20 devient 15/23) ; celles de génération 120 et antérieures restent sans progression automatique des PV. Aucune migration silencieuse ni guérison au chargement ou à l'import de progression. Le format du snapshot n'a pas changé.

Le journal de montée de niveau affiche également « PV maximum +3 ». Les jauges et la fiche lisent le maximum réel du moteur.

Les statistiques, niveaux et dégâts des monstres ne sont pas modifiés par cette passe. Les diagnostics fixes en génération 120 restent des références historiques ; les prochains essais d'équilibrage doivent inclure cette nouvelle progression, sans supposer un bon tirage d'équipement ni déclarer les anciens combats équilibrés à eux seuls.

Les [douze parcours comparatifs en génération 122](PREMIERE_COUCHE_PV_ET_EXPERIENCE.md) suivent désormais les gains réels d'XP et les soins de niveau. Un parcours atteint le niveau 2 et reçoit neuf PV de soin ; les onze autres restent au niveau 1. Ce départ artificiel directement sous terre doit être complété par une progression continue depuis la surface avant de conclure sur l'équilibre de campagne.
