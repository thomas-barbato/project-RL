# Architecture du moteur

Le moteur de Project RL est une bibliothèque de simulation indépendante de Macroquad. Une interface graphique, un outil de test ou un serveur de validation doivent pouvoir lui envoyer les mêmes commandes et lire les mêmes événements.

## Contrat de simulation

```text
intention du joueur
    → GameCommand
    → validation par GameState
    → mutation déterministe
    → GameEvent
    → présentation future
```

Une commande refusée ne consomme pas de tour. Une commande acceptée passe successivement par la résolution du joueur, des autres acteurs et de l'environnement.

## Extension des règles

Les paramètres susceptibles de varier appartiennent à des structures de règles ou de configuration, pas aux systèmes d'entrée ou de rendu. C'est déjà le cas pour :

- la portée, la métrique et les coins bloquants du champ de vision ;
- les dimensions et contraintes du générateur de salles ;
- les invariants appliqués à une carte générée.

Ces structures sont progressivement prises en charge par le pipeline de contenu. Les règles simples resteront déclaratives. Les changements de comportement qui ne peuvent pas être exprimés avec les primitives du moteur passeront plus tard par une API de script sandboxée et versionnée.

## Paquets de contenu et mods

`ContentLoader` découvre les sous-dossiers de racines fournies par l'application (`content/` et `mods/` dans la preuve actuelle). Il ne contient aucun chemin Windows. Chaque paquet fournit un `manifest.toml` avec un identifiant global, une version sémantique, une contrainte de version du jeu et ses dépendances obligatoires, optionnelles ou incompatibles.

Avant de lire les définitions, le chargeur refuse les identifiants de paquet dupliqués, dépendances absentes, versions incompatibles et cycles. L'ordre obtenu est déterministe : une dépendance précède toujours le paquet qui l'utilise. Le paquet `core` est obligatoire.

Les identifiants de contenu utilisent désormais le type commun `ContentId` sous la forme `namespace:name`. Un fichier ne peut déclarer que des identifiants appartenant au paquet qui le contient, et une collision de catalogue est une erreur explicite. Les fichiers sont limités en taille et leur chemin canonique doit rester dans la racine autorisée du paquet.

Les familles actuellement chargées sont `status/*.json5` et `weapons/*.json5`. Le JSON5 autorise les commentaires et virgules finales, mais refuse les champs inconnus afin que les fautes de frappe ne soient pas silencieuses. `content/core/status/corroded.json5` remplace la définition Rust de la corrosion, tandis que les deux armes initiales proviennent du catalogue externe. Le paquet `mods/example.arc_arsenal` ajoute réellement sa lance électrique au catalogue et à l'inventaire de départ sans recompilation. Les acteurs, capacités et biomes suivront le même pipeline.

## Inventaire et équipement

L'inventaire est une structure de simulation, indépendante de l'affichage. Chaque pile reçoit un `ItemInstanceId` monotone et stable : un tri visuel ou la suppression d'une autre ligne ne peut donc pas transformer une commande en action sur le mauvais objet. L'ajout remplit les piles existantes dans un ordre déterministe, respecte une capacité en emplacements et reste atomique en cas de manque de place. La limite de pile vient de la définition résolue ; les armes utilisent actuellement une limite de un.

L'équipement ne copie pas les statistiques de l'objet. Il conserve la relation entre un identifiant d'emplacement et un `ItemInstanceId`, puis `GameState` résout l'arme dans le `WeaponCatalog` au moment de l'attaque. Une arme ajoutée par un mod emprunte donc exactement le même trajet qu'une arme officielle. Les identifiants et l'ordre des canaux d'attaque appartiennent à `GameRules`, pas à l'interface. Le contenu de départ du sac et l'équipement de départ sont deux listes distinctes : une arme peut ainsi être fournie par un mod et rester rangée jusqu'à ce que le joueur l'équipe.

`GameCommand::EquipWeapon` valide que l'instance se trouve bien dans l'inventaire et correspond à une arme connue. Une réussite consomme un tour et produit `GameEvent::WeaponEquipped`; une erreur ne consomme rien. Une même instance ne peut occuper qu'un seul emplacement.

La vue ASCII ouvre avec `I` un premier écran d'inventaire à deux panneaux : liste et état d'équipement à gauche, profil de combat utile et canaux à droite. Les identifiants internes, clés de traduction et chemins de paquet ne sont pas montrés au joueur. Les touches haut/bas sélectionnent une arme et `1`, `2` ou `3` l'équipent dans le canal correspondant puis l'activent ; le pavé numérique est également pris en charge. Inventaire fermé, `1`, `2` et `3` sélectionnent gratuitement un canal déjà équipé, puis `F` et l'attaque par contact utilisent l'arme active. Cet écran ne possède aucune statistique de combat propre : tout ce qu'il affiche vient de l'inventaire, de l'équipement et des catalogues du moteur.

## Contrat d'une carte générée

Un générateur n'est pas autorisé à déclarer lui-même que son résultat est jouable. Le validateur indépendant vérifie après génération, puis de nouveau après placement du décor :

1. le départ du joueur est praticable ;
2. la sortie est praticable et accessible ;
3. chaque position obligatoire est praticable et accessible ;
4. la bordure extérieure est fermée si la règle l'exige ;
5. toutes les cases praticables appartiennent au même réseau si la règle l'exige.

La connectivité est calculée par flood-fill sur les règles de déplacement réellement fournies au validateur. Un futur décor bloquant pourra donc être inclus grâce à `WalkabilityQuery`, sans réécrire l'algorithme.

## Déterminisme

Toute génération et toute décision aléatoire de gameplay utilisent `GameRng`. Une même graine et les mêmes règles doivent produire le même résultat. Le hasard réservé au rendu utilisera une source séparée afin de ne jamais modifier une partie.

## IA et combat

Les attaques sont décrites par `AttackProfile` et passent toutes par la même résolution des dégâts et résistances. Les acteurs ne possèdent donc pas de classe de combat codée en dur.

Les premiers comportements (`Hunter`, `Sentry`, `Skirmisher`) sont pilotés par `AiProfile`. Ils produisent une intention (`Move`, `Attack`, `Wait`) que `GameState` valide comme toute autre action. L'ajout futur de comportements composés ou scriptés devra préserver cette frontière.

## Présentation ASCII

`src/ascii_app.rs` est un client temporaire du moteur. Il traduit le clavier en `GameCommand` et les états/événements en glyphes, mais ne contient aucune règle de victoire, de combat, de visibilité ou d'IA. Il pourra être remplacé par un rendu à base d'assets sans modifier la simulation.

## Propagation systémique

La primitive `propagate` effectue une propagation déterministe par coût et par étape. Son `PropagationPolicy` décide si un effet traverse une case et à quel coût. Le même algorithme pourra donc servir aux explosions, au feu, au gaz, à l'électricité ou à une règle ajoutée par un mod, sans dupliquer les parcours de grille.

Les capacités sont décrites par `AbilityProfile` et composent des `EffectPrimitive`. La première primitive, `RadialDamage`, utilise cette propagation, applique une réduction configurable selon le coût parcouru puis passe par la résolution centrale des résistances. L'interface ASCII déclenche exactement la même commande que le feront les futures interfaces.

## Progression de la partie

La progression temporaire est portée par `RunProgression`, indépendamment des acteurs, de l'affichage et de `MetaProgression`. Sa courbe utilise des seuils cumulatifs configurables : un mod peut donc remplacer les paliers, les points de compétence gagnés et la réduction appliquée aux menaces triviales sans modifier l'algorithme.

Les gains produisent `ExperienceAwarded`, puis un `LevelGained` pour chaque palier franchi. Une récompense d'exploration, de piratage ou d'objectif pourra fournir une `RewardKey` stable afin de ne payer qu'une fois par partie. Les profils `DefeatReward` marquent aussi l'origine des ennemis : les acteurs invoqués ou fabriqués ne donnent aucun XP par défaut, ce qui ferme les boucles de création/destruction infinies. Leurs taux restent néanmoins des paramètres explicites de `ProgressionRules` pour les mods qui remplacent cette politique.

L'interface ASCII se contente de lire le niveau et l'XP et d'afficher ces événements. Elle ne calcule ni ne modifie la progression.

## États temporaires

Un statut est séparé en deux parties : une `StatusDefinition` immuable enregistrée dans le `StatusCatalog`, puis une `StatusInstance` attachée à l'acteur pendant la simulation. L'instance ne conserve que son identifiant stable, ses charges, sa durée restante et sa source. Une capacité qui référence un identifiant absent du catalogue est refusée avant le début de la partie.

Le modèle réserve les hooks prévus (`TurnStart`, `TurnEnd`, dégâts reçus ou infligés, mouvement et mort) et des primitives génériques. La boucle active pour cette première tranche est `TurnEnd` : sa première primitive inflige un `DamagePacket` plat ou multiplié par les charges. Les durées sont décrémentées après le déclenchement dans un ordre stable par identifiant d'entité puis de statut. Les autres hooks sont définis mais ne sont pas encore raccordés aux événements correspondants.

La vue ASCII fournit une preuve minimale avec `H` : `core:corroded` se cumule jusqu'à trois charges, rafraîchit ses trois tours et inflige des dégâts chimiques en fin de tour. Sa couleur et le journal proviennent des événements `StatusApplied`, `StatusTriggered` et `StatusRemoved`; aucune règle de corrosion n'est codée dans le rendu.
