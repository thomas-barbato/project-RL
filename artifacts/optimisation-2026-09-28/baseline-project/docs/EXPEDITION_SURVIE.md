# Sortie courte : ressources finies et vie unique

27 septembre 2026 · génération 116 inchangée.

## Contrainte confirmée

Le joueur n'a **qu'une vie**. La mort termine la partie ; recommencer signifie
un nouveau personnage et une nouvelle partie, pas reprendre le même personnage
avec ses acquis. Une suspension conserve une partie vivante, ce n'est pas un
point de résurrection.

L'équilibrage doit donc permettre d'apprendre des signaux lisibles et de
renoncer à une sortie. Il ne doit pas supposer des morts répétées pour réussir
la même rencontre avec le même personnage. Cela n'interdit pas les rencontres
dangereuses et ne garantit pas la survie dans toutes les situations.

## Parcours contrôlé

Suite des [rencontres combinées](RENCONTRES_COMBINEES.md). Trois secteurs
persistent dans le même `WorldState` : un départ sans ennemi, puis deux salles
de surface contenant chacune deux Chauves-souris des ruines et un Cracheur des
mares. Leurs profils proviennent du générateur et du catalogue actuels.

Le personnage part avec un couteau de camp, un fusil de patrouille contenant
12 projectiles et deux soins existants de 6 PV. Ce kit limité remplace celui,
plus généreux, du diagnostic général ; il ne modifie pas le départ en campagne.
Les jets de toucher, défenses, gains d'expérience et récupérations restent actifs.

Une cache humaine du premier secteur contient une veste tirée par le véritable
générateur d'équipement : poids de profondeur, possibilité de bonus et valeurs
aléatoires conservés. Seule la famille est imposée pour éprouver une armure.
Il faut marcher jusqu'à la cache, ramasser l'objet et l'équiper avant d'affronter
le second groupe. Le test vérifie que ses bonus ne sont pas retirés au sort à
la récupération. Aucun équipement humain ne tombe d'une chauve-souris.

L'objectif de la fixture est d'atteindre le fond du second secteur puis de
revenir au départ. Ce n'est pas une nouvelle quête de campagne. Les changements
de secteur ne doivent rendre ni PV, ni munitions, ni consommables.

## Mesures

Les graines **0 à 15 sont toutes exécutées**, une fois par conduite dans chaque
exécution du test. Pas de sauvegarde rechargée après un mauvais jet, pas de
recherche d'une graine gagnante, pas d'échec écarté du rapport.

Deux conduites automatiques utilisent le même parcours et les mêmes tirages :

- **Prudente** : quitte les cases annoncées, soigne à 12 PV ou moins, revient
  à 8 PV ou moins sans soin, et renonce après 40 actions sans progrès dans un
  combat. Cette dernière limite borne le pilote de test, pas l'IA du jeu.
- **Offensive** : attaque sans tenir compte des annonces, sans soin ni décision
  de renoncer. Elle continue néanmoins à se déplacer et à combattre normalement.

Les deux passent au couteau une fois le fusil vide. Une mort arrête immédiatement
l'essai ; un blocage au-delà de 400 commandes échoue au lieu de disparaître des
mesures.

| Conduite | Retour vivant | Objectif atteint | PV au retour | Projectiles restants | Soins restants |
|---|---:|---:|---:|---:|---:|
| Prudente | 16/16 | 14/16 | 20 | 0 | 2 |
| Offensive | 16/16 | 16/16 | 11–20 | 0 | 2 |

Les deux renoncements prudents concernent les graines 10 et 11. Le personnage
conserve sa vie et la veste récupérée ; le test ne transforme pas ces replis
en objectifs réussis. Les parcours prudents durent 60–85 tours, les offensifs
60–76 tours. Chaque aller-retour effectue quatre passages entre secteurs.

## Soins, persistance et mort

Deux scénarios complémentaires évitent de conclure sur les soins à partir
de trajets qui n'en consomment pas :

1. Le personnage subit une vraie attaque jusqu'à être blessé, utilise un soin
   de 6 PV, puis se replie. Il ne lui reste qu'un soin. PV et réserves sont
   identiques avant/après le passage et après restauration du snapshot vivant.
2. Le personnage reste exposé jusqu'à sa mort. Attendre, marcher, ramasser,
   emprunter le passage ou utiliser un soin sont alors refusés, sans avancer
   le temps. Une nouvelle instance de partie repart au secteur initial avec
   les seules réserves de départ, sans exploration ni butin hérités.

Le test client existant `a_finished_run_cannot_create_a_resurrection_checkpoint`
couvre séparément le refus de créer une suspension après la mort.

## Lecture des résultats

**Le fusil seul ne suffit pas à cette sortie** : les 12 projectiles sont dépensés
dans les 32 essais. Le couteau permet ici de poursuivre ; cela justifie de
surveiller l'autonomie à distance, pas d'ajouter immédiatement des munitions
ou de modifier leur capacité.

Les salles sont ouvertes, les ennemis fragiles et leurs attaques annoncées.
Le pilote connaît le plan. La conduite offensive survit elle aussi : cette
fixture n'établit ni une difficulté satisfaisante, ni un taux de survie réel
pour un joueur. Elle ne justifie aucune hausse de PV ou de dégâts ennemis.

La cache de veste est garantie **dans ce test seulement**. La disponibilité
réelle des caches, la progression des compétences, les couloirs, les poursuites
de mêlée et les rencontres des autres couches restent à mesurer. La prochaine
passe doit porter sur des secteurs générés et des profils mobiles, sans
réinitialiser les réserves entre rencontres.

Une [première mesure du repli sur secteurs générés](REPLI_SECTEURS_GENERES.md)
couvre maintenant huit cartes de surface, leurs populations complètes et deux
moments de décision. Elle mesure la fuite au premier contact, pas la conquête
des secteurs ni l'exploration de toutes les couches.

## Rejouer

```powershell
cargo test --locked --bin project-rl survival_expedition -- --nocapture
cargo test --locked --bin project-rl a_finished_run_cannot_create_a_resurrection_checkpoint
```

Ces fixtures sont compilées uniquement pour les tests. Aucun nouveau contenu
de campagne, menu, statistique, règle de butin ou format de sauvegarde n'est
introduit ; aucun fichier de partie du joueur n'est utilisé.

Validation locale : les trois nouveaux tests `survival_expedition` passent
(32 parcours, soin/repli/restauration, mort et nouvelle instance), ainsi que
les deux tests clients existants sur l'absence de point de résurrection et
le redémarrage depuis le menu. Le contrôle de tous les targets, le formatage
et `git diff --check` passent. La suite globale n'a pas été relancée pour
cette passe limitée aux tests. Aucun commit ni push.
