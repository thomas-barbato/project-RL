# AGGRO-START-01 — agressivité des monstres

20 octobre 2026. Demande du joueur : « certains monstres n'attaquent toujours
pas, ils doivent etre aggressifs :) ».

## Défauts reproduits avant correction

- Les profils ordinaires de mêlée et de tir de niveau 1, et de mêlée de
  niveau 2, ne détectaient pas le Traqueur à découvert à six cases. Sans
  mouvement ni compétence de furtivité, huit attentes ne déclenchaient
  aucune attaque : le score passif de Coordination suffisait à le cacher.
- Un tireur de niveau 1, placé à trois cases du Bastion immobile, attaquait
  pendant dix tours puis cessait au onzième. Son ancien cycle de poursuite
  imposait un retour au poste et une période d'attente, malgré la cible visible.
- La limite de distance depuis le poste initial empêchait également certaines
  poursuites, même lorsque le monstre voyait le joueur.

Ces deux premiers cas ont été reproduits par des tests en échec utilisant les
profils réellement construits par le peuplement des égouts et les cinq classes.

## Correction demandée et portée

Les monstres hostiles des populations `core:street_population_01` et
`core:sewer_population_04` engagent le joueur à découvert dans leur portée
réelle de détection. Les attributs seuls et le simple voisinage d'un mur ne
rendent plus le joueur invisible. Les bonus explicites de dissimulation,
le camouflage, la furtivité active et le brouillage restent pris en compte.

Une cible visible renouvelle la poursuite et interrompt immédiatement les
anciens états de retour au poste et d'attente. La limite territoriale du poste
initial ne bloque plus cette poursuite. Après perte de vue, la recherche
continue d'utiliser la dernière position connue et le cycle existant.

Les murs, les angles fermés, les portées de perception et d'attaque, les
obstacles, les zones protégées, les récupérations, les étourdissements et les
tactiques de mêlée ou de tir restent applicables. Les alliés, les créatures
contrôlées et les populations historiques sans ces identifiants gardent leur
politique actuelle. Aucun changement de dégâts, PV, précision, armure, XP,
nombre, placement, récompense ou dessin n'accompagne cette correction.

## Compatibilité

La correction utilise les identifiants déjà enregistrés sur les acteurs.
Elle s'applique aussi aux monstres existants d'une partie reprise, sans
recréer les groupes ni remplacer les acteurs. Le format de sauvegarde et
les profils stockés restent identiques ; aucune nouvelle génération n'est
nécessaire. Aucun fichier du joueur n'est modifié par les tests.

## Vérifications

Les tests couvrent les profils réels de mêlée et de tir aux niveaux 1 et 2,
les cinq classes à une, trois et six cases, le maintien du tir pendant
24 tours, ainsi que les quinze membres des groupes de la ville face au
Traqueur à six cases dans une arène dégagée. Les contrôles moteur couvrent
les limites de vue, les murs, les zones protégées, le camouflage actif et la
reprise de quatre combinaisons population/état de retour ou d'attente.

Les commandes et leurs résultats finaux sont consignés dans `verification.json`.
