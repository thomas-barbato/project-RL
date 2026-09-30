# Expédition continue : surface et profondeurs

28 septembre 2026 — génération 122.

## Ce qui est testé

Le nouveau pilote part dans le vrai secteur initial du client, puis utilise les passages ordinaires et leur génération différée. Il ne recrée pas un joueur à chaque région et n'injecte aucun niveau, soin, objet ou bonus à la descente. Les services urbains, les habitants et les rencontres sont installés par le chemin normal du client.

L'échantillon fixé comprend deux graines (0 et 1), deux approches de la surface (directe ou avec détour) et trois styles (mêlée, distance, évitement), soit douze vies indépendantes. La graine 0 suit le trajet souterrain court, la graine 1 le long. Les deux routes souterraines ne sont donc pas croisées avec toutes les graines : ce n'est pas une matrice exhaustive.

- Départ direct : secteur initial `(0,0,0)`, région occidentale `(-1,0,0)`, ville de couche 1 `(-1,0,1)`.
- Détour : deux régions supplémentaires, `(0,1,0)` puis `(-1,1,0)`, avant la même descente.
- Suite : trajet de première couche réservé par la graine, descente vers la couche 2, arrivée à sa ville, puis retour par les régions effectivement traversées **jusqu'à la case de départ**, et pas seulement à la frontière du secteur initial. Un repli précoce peut interrompre cette progression ; une mort termine l'essai.

Tous les liens ordinaires de l'atlas restent disponibles. C'est le pilote qui choisit ces itinéraires : aucun couloir obligatoire n'est ajouté au jeu, aucune quête n'est exigée et aucune sortie n'est déplacée.

## Ressources et décisions

Le kit initial est le cas modeste déjà utilisé dans les diagnostics : couteau de camp, fusil de patrouille, douze munitions, deux soins et 20 PV, au niveau 1 et sans affixe. **Ce kit de test n'est pas le kit de départ définitif du jeu.** Aucun équipement favorable n'est sélectionné après lecture des tirages.

Le pilote cible seulement les hostiles visibles. Il évite les attaques annoncées, utilise un soin à 12 PV ou moins et décide de rentrer à 6 PV ou moins sans soin. En dehors de cette fuite, le style mêlée ou distance privilégie une attaque disponible ; l'évitement privilégie le déplacement. Ce sont des politiques simples, pas un joueur expert.

Lorsqu'aucun hostile n'est visible, il peut rejoindre un objet visible à cinq cases de distance de Manhattan au plus, puis le ramasser. Les objets possédés sont laissés sur place ; un inventaire plein ne donne ni place supplémentaire ni remplacement favorable. Une armure acquise est équipée par une vraie commande si sa protection de base est supérieure à celle portée, sans modifier ses bonus. Les armes de départ ne sont pas remplacées automatiquement. Les points de compétence ne sont pas dépensés, les marchands et soigneurs ne sont pas utilisés. Les bénéfices éventuels des affixes portés restent ceux calculés par le moteur.

**Le terrain et l'itinéraire sont connus du pilote.** Le détour ne constitue pas une exploration exhaustive de la surface. Les réussites ne représentent pas un taux de victoire humain ; les échecs ne justifient pas à eux seuls un affaiblissement général des monstres.

## Contrôles techniques

À chaque transition, le test compare exactement PV, munitions, soins, progression, inventaire avec instances et bonus, et équipement porté. Il sérialise ensuite le monde, le recharge, compare les octets puis continue sur ce monde restauré. Il s'agit du snapshot moteur conservant l'état des régions ; cela ne remplace pas les tests de suspension complète de l'interface.

Le relevé conserve les arrivées, leur tour, l'XP, le niveau, les PV, les réserves, les objets, les victoires, ramassages, équipements, tirs, soins, pics de dégâts et dernière blessure. Une limite de 2000 commandes ou douze attentes consécutives constitue un arrêt technique à examiner, jamais une mort reclassée en succès. Les échecs ne sont pas remplacés par une nouvelle graine.

Commande du diagnostic long :

```powershell
cargo test --locked --bin project-rl continuous_expedition_sample_ -- --ignored --nocapture --test-threads=4
```

Les deux tests ordinaires `continuous_expedition_routes_use_real_open_atlas_links_and_finite_start` et `continuous_expedition_equips_a_found_armor_without_replacing_the_inventory` contrôlent séparément le contrat du pilote. Le second crée volontairement une armure dans son petit test fonctionnel ; cette création n'est jamais effectuée dans les douze parcours mesurés.

Le prétest s'arrêtait dès le retour dans le secteur initial, sans rejoindre le point de départ. Cette limite a été corrigée avant le relevé final : les quatre groupes sont rejoués sur les mêmes graines. Les premiers résultats partiels ne sont pas mélangés à l'échantillon corrigé.

## Relevé final

Les douze vies aboutissent à **sept allers-retours jusqu'au point de départ et cinq morts**. Les six départs directs réussissent ; parmi les six détours, un réussit et cinq meurent. Quatre morts précèdent la première descente, une survient au retour après la ville de couche 2. Toutes se produisent en surface. Aucun arrêt technique ni refus de commande non traité.

| Graine | Surface | Approche | Issue | Tours | Niveau final | PV finaux | Tirs | Soins utilisés | Ramassages |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| 0 | Direct | Mêlée | Aller-retour | 1072 | 1 | 20 | 0 | 0 | 1 |
| 0 | Direct | Distance | Aller-retour | 1080 | 2 | 23 | 10 | 0 | 1 |
| 0 | Direct | Évitement | Aller-retour | 1070 | 1 | 20 | 0 | 0 | 1 |
| 0 | Détour | Mêlée | Mort avant descente | 363 | 1 | 0 | 0 | 2 | 0 |
| 0 | Détour | Distance | Mort au retour | 1085 | 2 | 0 | 12 | 2 | 3 |
| 0 | Détour | Évitement | Mort avant descente | 360 | 1 | 0 | 0 | 2 | 0 |
| 1 | Direct | Mêlée | Aller-retour | 1221 | 1 | 14 | 0 | 1 | 1 |
| 1 | Direct | Distance | Aller-retour | 1237 | 2 | 13 | 12 | 1 | 1 |
| 1 | Direct | Évitement | Aller-retour | 1219 | 1 | 17 | 0 | 2 | 1 |
| 1 | Détour | Mêlée | Mort avant descente | 332 | 2 | 0 | 0 | 2 | 0 |
| 1 | Détour | Distance | Aller-retour | 1586 | 3 | 26 | 12 | 0 | 1 |
| 1 | Détour | Évitement | Mort avant descente | 160 | 1 | 0 | 0 | 2 | 0 |

Les ramassages sont des commandes réussies, pas nécessairement un nombre d'unités d'objet. Une mort conserve zéro PV ; le maximum n'est pas présenté ici lorsque l'acteur a été retiré du monde.

### Première arrivée dans la ville de couche 1

Les quatre vies terminées avant la descente ne figurent pas dans ce tableau. Aucun soin ni rééquipement gratuit n'est effectué pour produire ces états.

| Graine | Surface | Approche | Niveau | XP | PV | Munitions | Soins restants |
|---|---|---|---:|---:|---:|---:|---:|
| 0 | Direct | Mêlée | 1 | 5 | 20/20 | 12 | 2 |
| 0 | Direct | Distance | 1 | 5 | 20/20 | 10 | 2 |
| 0 | Direct | Évitement | 1 | 0 | 20/20 | 12 | 2 |
| 0 | Détour | Distance | 2 | 24 | 9/23 | 0 | 0 |
| 1 | Direct | Mêlée | 1 | 5 | 14/20 | 12 | 1 |
| 1 | Direct | Distance | 2 | 10 | 23/23 | 9 | 2 |
| 1 | Direct | Évitement | 1 | 0 | 14/20 | 12 | 1 |
| 1 | Détour | Distance | 3 | 30 | 26/26 | 0 | 2 |

## Lecture des résultats

Le détour peut produire une progression réelle : sur la graine 1, l'approche à distance entre en couche 1 au niveau 3, avec 26/26 PV, mais sans munition. Elle termine ensuite le retour avec une armure trouvée en couche 2 et ses deux soins encore disponibles. L'approche directe à distance de la même graine entre au niveau 2 avec neuf munitions. Gagner davantage de niveaux et conserver davantage de réserves ne sont donc pas le même avantage.

Sur la graine 0, le détour à distance rapporte deux fusils, dont un avec bonus, et permet d'arriver au niveau 2. Mais l'entrée souterraine se fait à 9/23 PV, sans soin ni munition. Le pilote atteint la ville suivante puis meurt en surface au retour. Il garde ses armes initiales par protocole : **cet échec ne prouve pas que le fusil trouvé aurait été inutile**, puisque ses bonus ne sont pas exploités. Une évaluation des choix d'équipement devra traiter ce cas explicitement.

Le critère de retour corrigé est important : dans le détour en mêlée de la graine 1, le personnage rejoint le secteur initial au tour 235 avec trois PV, mais meurt au tour 332 avant sa case de départ. Une remontée ou un changement de région n'est pas un refuge automatique.

Les morts observées ici se produisent en surface. Plusieurs correspondent à une usure par petits tirs, et l'approche d'évitement de la graine 1 subit aussi jusqu'à dix dégâts après une commande. Ces traces donnent une priorité d'investigation au repli, aux lignes de tir et au choix du couvert en surface. Elles ne démontrent pas une poursuite infinie et ne suffisent pas à prescrire une réduction générale des statistiques ennemies.

Les essais précédents, démarrant directement sous terre, restent séparés : l'historique des commandes, l'état aléatoire, les entrées, les rencontres et les décisions de ramassage ne sont pas identiques. Une différence d'issue ne peut pas être attribuée uniquement aux PV supplémentaires.

## Portée et prochaine décision

La continuité technique est désormais éprouvée avec le client réel et des ressources effectivement gagnées ou dépensées. L'équilibre de campagne reste ouvert : deux graines, pas d'achat de soins, pas de points de compétence dépensés, pas de changement d'arme, et exploration locale très limitée autour du chemin connu.

Avant un réglage global, le prochain travail utile est de reprendre les situations de repli fatales avec des décisions de couverture explicites, puis d'évaluer les armes réellement récupérées et les services disponibles. Cela permettra de distinguer une route trop punitive d'une politique de pilote insuffisante. Cette passe ne modifie ni les monstres, ni le butin, ni les règles de sauvegarde et n'ajoute aucune contrainte de quête.

Suite du diagnostic : [repli à couvert et utilisation des armes trouvées](REPLI_COUVERT_ET_ARMES_TROUVEES.md). Cette comparaison conserve le présent relevé comme référence historique et ne le remplace pas.

## Validation de cette passe

- Quatre groupes longs exécutés explicitement en parallèle, soit les douze parcours finaux : quatre tests réussis en 405,60 secondes.
- Deux nouveaux tests ordinaires de contrat du pilote réussis, ainsi que les deux tests client de progression des PV et le contrôle des kits ordinaires.
- `cargo check --locked --all-targets`, compilation native et contrôle de formatage réussis.
- Contrôle des différences et treize groupes de validation documentaire réussis.
- Pas de suite complète ni de contrôle visuel relancés : les changements de cette passe concernent les diagnostics et leur documentation, pas les règles de combat ou l'interface.
