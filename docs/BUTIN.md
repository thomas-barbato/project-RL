# Butin pondéré — premier jalon moteur

Décision confirmée : la profondeur doit favoriser les objets plus puissants, sans supprimer les trouvailles exceptionnelles près de la surface. Un objet associé aux grandes profondeurs pourra donc y avoir un poids très faible plutôt qu'une interdiction systématique. Les probabilités finales, budgets de puissance et affixes ne sont pas encore définis.

Ce jalon tire **des définitions d'objets existantes**. Il ne génère pas encore leurs statistiques et ne déduit pas leur puissance de leur rareté ou de leurs dégâts bruts. Les objets de quête et autres solutions indispensables ne doivent pas dépendre d'un tirage rare ; leur placement garanti reste distinct.

## Fonctionnement

`src/loot/mod.rs` appartient à la bibliothèque sans rendu :

- `LootContext` fournit profondeur, type de carte et source du butin.
- `LootTable` conserve des entrées ordonnées avec un objet, un poids, une plage de profondeur, des sélecteurs et une quantité.
- `LootCatalog` enregistre les tables sous des identifiants stables `paquet:nom` et refuse les doublons.
- `draw` produit des `LootDrop` contenant identifiant d'objet et quantité. Le fournisseur de zone les place ensuite sur les cases prévues, sans modifier leur traversabilité.

Les bornes de profondeur et de quantité sont inclusives. Une profondeur maximale absente n'impose aucune limite haute. Une liste vide de types de carte ou de sources accepte tous les contextes correspondants. Les sélecteurs sont des identifiants exacts, pas des sous-chaînes ; leurs registres sémantiques ne sont pas encore implémentés.

Les poids sont relatifs : la probabilité d'une entrée éligible vaut son poids divisé par la somme des poids éligibles. Un poids nul désactive une entrée. Les lignes qui se recouvrent **s'additionnent**, y compris pour un même objet ; l'ordre n'établit pas de priorité. Les tirages se font avec remise : deux piles peuvent contenir le même objet. S'il n'y a aucune entrée éligible, aucun objet n'est produit et aucun objet de repli hors contexte n'est choisi.

Le moteur utilise les entiers et un tirage sans biais de modulo. Même graine, même table ordonnée et même contexte donnent mêmes résultats et même état RNG final. Les refus, zéro tirage et les contextes vides préservent le RNG. La fixture d'expédition et les régions utilisent un flux séparé pour le butin : changer les tirages ne change ni le terrain, ni les lieux remarquables, ni le flux aléatoire des ennemis.

## Format de contenu

`ContentLoader` charge `loot/*.json5` après résolution de tous les catalogues d'armes et d'objets. Les tables suivent les mêmes contrôles de paquets, namespaces, chemins, taille et champs inconnus que les autres définitions. Un objet inconnu est refusé même dans une entrée désactivée. Les quantités doivent respecter la taille maximale de pile ; une arme a actuellement une limite de un.

Exemple :

```json5
{
  id: "mon_mod:atelier",
  entries: [
    {
      item: "core:repair_patch",
      weight: 999,
      minimum_depth: 0,
      maximum_depth: 2,
      map_kinds: ["core:industrial"],
      sources: ["core:floor"],
      quantity: [1, 1],
    },
    {
      item: "mon_mod:outil_rare",
      weight: 1,
      maximum_depth: 2,
      map_kinds: ["core:industrial"],
      sources: ["core:floor"],
    },
  ],
}
```

Ce mod doit aussi fournir la définition `mon_mod:outil_rare` et déclarer sa dépendance à `core`. Dans ce contexte, la deuxième entrée aurait une probabilité de 1/1000, soit 0,1 % par tirage. C'est un exemple technique, pas un taux final approuvé. D'autres lignes à partir de la profondeur 3 peuvent accroître son poids. Les champs omis prennent profondeur minimale 0, aucune limite haute, sélecteurs universels et quantité `[1, 1]`.

Les tables sont limitées à 1024 entrées, les sélecteurs à 64 valeurs distinctes et chaque requête à 64 tirages. Les plages inversées, quantités nulles, sélecteurs dupliqués et tables sans poids positif sont refusés. Pas encore de tables récursives, d'objets uniques, d'affixes ou de garantie de rareté après plusieurs échecs.

## Preuve jouable et exemple de mod

Les **nouvelles parties** utilisent `core:industrial_floor` pour les deux piles du secteur industriel. La ville conserve ses objets de test fixes. Jusqu'à la profondeur 2, les poids d'essai sont réparation 70, lance-aiguilles 25, plastron rapiécé 12 et lame 5 ; à partir de 3, ils deviennent 45, 50, 20 pour la carapace composite et 5. Cela rend déjà une protection trouvable hors choix de classe, sans prétendre fixer l'équilibrage entre ces objets incomparables.

Les deux biomes régionaux de surface sélectionnent `core:regional_cache`. Chaque région reçoit deux caches accessibles et deux à quatre tirages ; les piles remplissent d'abord ces caches. Depuis la génération v24, le contenu d'un cache placé derrière une entrée verrouillée conserve aussi le propriétaire déclaré par le profil de sécurité régional : le ramasser sans autorisation peut donc déclencher le capteur et la demande de renfort associée, sans modifier le tirage lui-même. En surface, les poids provisoires sont 690 pour une réparation, 300 pour le lance-aiguilles, 130 pour le plastron rapiécé, 9 pour la lame et 1 pour le lance-flammes. Ce dernier reste une exception inférieure à 0,1 % par tirage dans cette nouvelle somme. À partir de la profondeur 1, les poids deviennent 400, 350, 180, 120 pour le plastron, 50 pour la carapace composite et 70 : la distribution évolue bien avec la profondeur. La première région profonde de la version 29, Maintenance ou Production selon la seed, possède trois caches et trois à cinq tirages de ce contexte. Ces valeurs démontrent le schéma, elles ne constituent pas l'équilibrage final.

Le **Plastron rapiécé** fournit actuellement +1 Blindage et peut donc apparaître dès la surface. La **Carapace composite** fournit +2 et entre dans les caches à partir de la profondeur 1 ainsi que dans la table industrielle à partir de la profondeur 3. BRÈCHE reçoit aussi un plastron garanti dans son inventaire initial afin que le système reste testable sans dépendre d'un tirage. Cette attribution de départ n'équipe pas silencieusement la pièce.

`content/core/worlds/expedition.json5` choisit `loot_table` et `loot_draws`. Le secteur fournit automatiquement son type, sa profondeur et la source `core:floor`. Une table absente ou une requête trop grande refuse la création de la partie, sans supprimer de suspension. Une table valide sans entrée éligible produit une zone sans ces piles.

Le paquet `mods/example.arc_arsenal` fournit une table réellement chargeable : `example.arc_arsenal:industrial_floor`. Sa définition `worlds/arc_expedition.json5` la sélectionne dans une expédition alternative également chargée et validée. Le client de test démarre encore explicitement `core:starter_expedition` : il ne propose pas encore de sélecteur d'expédition. La lance du mod a un poids de 1 contre 999 près de la surface, puis 300 contre 700 en profondeur. Cette démonstration n'en fait pas officiellement une arme de haut niveau. Aucun remplacement silencieux de la table `core` n'est autorisé.

Les tirages font partie des définitions persistantes des zones. Les visiter plusieurs fois ne relance pas les dés. Ramassage, inventaire, équipement, consommation et dépôt suivent les commandes et validations existantes.

Les armures et leurs nouvelles lignes de butin appartiennent à la génération **version 36**. Une suspension version 35 conserve le Blindage corporel introduit dans cette version, mais filtre les définitions, emplacements et lignes de butin d'armure avant de vérifier ses empreintes. Reprendre une ancienne partie ne modifie donc ni ses objets déjà tirés ni les probabilités de ses zones ; il faut choisir **Nouvelle partie** pour essayer ces protections.

## Suspension et compatibilité

Les parties versions 3 à 9 possèdent une empreinte dédiée du catalogue de butin en plus des règles et de l'état complet de l'expédition. Modifier les tables, même si un tirage particulier donnerait fortuitement le même résultat, refuse la reprise avant installation et conserve le fichier. La version 4 ajoute de la même manière l'empreinte des définitions d'expédition ; la version 5 y inclut le circuit d'installations, la version 6 ses métadonnées sociales, la version 7 les réactions d'alerte locale, la version 8 les alarmes installées et la version 9 leurs réponses configurées. Ces empreintes ne sont pas des signatures anti-triche.

Les suspensions **version 6** conservent propriété, affiliations et témoins sans recevoir rétroactivement les réactions d'alerte locale. Les suspensions **version 5** conservent le circuit de maintenance sans recevoir rétroactivement propriétaire, affiliations ou témoins. Les suspensions **version 4** calculent leur empreinte avec les définitions d'expédition historiques privées de `hub_facility` ; elles ne reçoivent donc pas rétroactivement les agents ou le régulateur. Les suspensions **version 3** conservent leur catalogue de butin et leur format lors des reprises et nouvelles suspensions ; elles n'exigent pas l'empreinte de monde ajoutée ensuite. Les **versions 2** conservent leur génération fixe, leur historique et leur format. Les **versions 1** restent d'abord vérifiées en mono-carte, puis passent en version 2 comme précédemment. On ne transforme pas silencieusement une partie active en nouveau tirage.

La sauvegarde reste une suspension à reprise unique, pas une liste de points de chargement. L'exécutable précédent a été conservé dans `target/recovery/before-loot-20260910` pour diagnostic ; il ne constitue pas une copie de la sauvegarde.

## Validation

Les tests couvrent le déterminisme, les quantités, frontières de profondeur, types/sources, entrées désactivées, absence de contexte éligible, poids dépassant ensemble `u32`, refus sans consommation RNG, contenu invalide et tables du mod. Un échantillon déterministe de 64 000 tirages par contexte vérifie la trouvaille exceptionnelle à faible profondeur et sa fréquence plus forte en profondeur.

Les tests de l'expédition vérifient aussi l'indépendance terrain/IA, les anciennes suspensions à butin fixe, le refus non destructif d'un catalogue de butin ou de monde modifié et le rejeu des aller-retours avec butin ramassé et mémoire visuelle. Le régulateur de puissance du circuit de maintenance est un placement garanti de type `material`, distinct des tirages de butin : une ressource indispensable à cette preuve ne dépend pas d'une rareté aléatoire. Le joueur peut le ramasser puis le livrer au dépôt qui le demande ; les recettes d'installation refusent une arme ou un consommable employé par erreur comme matériau. La génération reste contrôlée sur 64 graines par le validateur de navigation existant.
