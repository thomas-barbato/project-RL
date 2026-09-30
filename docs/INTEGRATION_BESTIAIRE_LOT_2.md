# Deuxième lot de bestiaire — intégration 116

27 septembre 2026. Les huit noms du lot ont été validés par le joueur.
Cette tranche termine les cinq rencontres restantes, après Guetteur,
Spectre et Cracheur des mares. Le catalogue compte maintenant 17 identités
jouables, hors rôles génériques et mannequins ; 19 fiches restent à revoir.

## Rencontres

| Rencontre | Milieu | Règle et réponse du joueur |
|---|---|---|
| Écailleux des cavernes | Recherche sèche, couches 2–4 | Oriente ses écailles vers une menace réellement vue. Direction verrouillée pendant trois occasions d'action ; les attaques directes venant de face sont réduites de moitié avant les défenses ordinaires. Les côtés restent ouverts, les dégâts sur la durée ne sont pas réduits par cette orientation. |
| Gueule du vide | Corruption, couches 5–7 | Annonce une couronne à environ deux cases, la frappe au tour suivant, puis annonce/frappe la couronne à trois cases. Le carré central de neuf cases reste sûr. Murs et zones protégées arrêtent l'effet. Trois occasions de récupération après l'expansion. |
| Traînard | Réseaux, couches 4–7 | Une avancée après sa phase lente ; à deux cases ou moins, annonce une frappe rapide sur une case fixe. Le joueur peut l'esquiver. Deux occasions de récupération, puis nouvelle phase lente. Ni double tour caché ni téléportation. |
| Chauve-souris des ruines | Ruines de surface, par paire | Nid partagé, piqué annoncé jusqu'à trois cases, approche réelle de deux pas maximum puis frappe au contact. Repli obligatoire vers le nid, au moins deux occasions ; pas de nouvelle attaque avant son retour. Le vol traverse l'eau profonde, jamais les murs ni les portes fermées. |
| Riveuse | Maintenance et production, couches 1–3 | Neutre, immobile ; annonce un axe de travail local de trois cases maximum, puis rivète et récupère deux occasions. L'interaction habituelle, adjacente, arrête définitivement le chantier, même pendant la préparation. Ne poursuit aucune victime. |

L’Écailleux reste une rosette minérale ; le nom ne crée pas de reptile.
La Gueule du vide n'a ni morsure ni dévoration. Le Traînard reste une anomalie,
pas un humain lent. Aucun nouveau peuple ni système de réputation.

## Génération, puissance et butin

- Tirages de groupes et budgets de faune inchangés. Les nouvelles espèces
  remplacent des tirages possibles ; elles ne s'ajoutent pas systématiquement
  à chaque groupe existant.
- Famille ailée de poids 25 en surface, espèce de niveau de référence 2,
  deux individus ; habitats en ruines et nid commun. Les autres familles
  conservent leurs poids propres.
- Écailleux : famille minérale, niveau de référence 10. Traînard : famille
  des boucles, référence 17. Gueule du vide : famille anormale, référence 21.
- Riveuse : règle de population neutre de poids 25, hors tirage biologique.
  Équilibrage par couche explicitement autorisé pour ce comportement, sans
  augmenter les civils. Aucune armure globale supplémentaire.
- PV et dégâts suivent le modèle 113, indépendamment du niveau du joueur.
  Objectifs de calibration : 1–3 touches réussies pour la chauve-souris,
  3–5 pour les autres avec une arme blanche adaptée, Écailleux pris de côté.
  Ces objectifs ne garantissent pas la difficulté d'une rencontre en groupe.
- Pas de ressources biologiques/minérales/anormales sans usage réel.
  Le nid ne crée aucun objet. La Riveuse a un bras de rivetage et un ensemble
  locomoteur ; seules ses pièces survivantes sont récupérables, avec leur
  durabilité restante. Aucun équipement humain inventé à sa destruction.

## Lisibilité et persistance

- Noms validés, glyphes ASCII distincts, légende et classification CAPTEURS.
  La Riveuse est identifiée comme neutre.
- Prévisualisations tirées des mêmes cellules que l'attaque réelle, sans F2.
  Les cellules cachées et les acteurs non visibles ne sont pas révélés.
- Trait latéral sur l’Écailleux indiquant le côté protégé ; détails de l'état
  au survol/ciblage. Annonces françaises courtes.
- Nid dessiné et mémorisé comme décor, conservé même si les occupants meurent.
- États ajoutés en fin des enums sérialisées ; retrait ciblé des cinq profils
  pour les générations <=115 avant empreinte et reconstruction.
- Les engagements et récupérations continuent dans les zones inactives,
  sans acquérir la position du joueur distant ni publier d'événements cachés.

## Limites explicites

Le rendu reste celui du terminal à glyphes ; ces intégrations ne produisent
pas les futures illustrations de créatures. Le vol n'accorde pas d'immunité
générale aux attaques ou aux effets de sol. Une chauve-souris dont le retour
est bloqué attend/cherche un passage au lieu d'annuler son repli pour attaquer.
Le chantier de la Riveuse est un danger local interrompable, pas une simulation
de construction. Une ancienne partie conserve son ancienne génération :
créer une nouvelle partie pour obtenir les nouvelles tables.

## Vérifications

- 15 tests ciblés ajoutés : comportements, dégâts réels de face/de côté,
  esquive, couvert, protection, arrêt du chantier, récupération avec usure,
  état hors écran, noms, interaction et snapshots/rejeu.
- Génération des cinq profils éprouvée sur 32 graines par profil, avec
  contrôle des habitats, distances aux passages, effectifs et nature du butin.
- Calibration sur huit graines pour chaque profondeur définie, avec couteaux
  sans bonus du palier local : fourchettes de touches visées respectées.
- Les cinq scènes natives de diagnostic ont été capturées et inspectées :
  `target/ui-bestiary-scaled-20260927/cold-start.png`,
  `target/ui-bestiary-maw-20260927/cold-start.png`,
  `target/ui-bestiary-laggard-20260927/cold-start.png`,
  `target/ui-bestiary-bat-20260927/cold-start.png`,
  `target/ui-bestiary-riveter-final-20260927/cold-start.png`.
  Ce sont des arènes isolées utilisant les profils réels, pas une campagne
  complète jouée manuellement.
- Suite globale `cargo test --locked --quiet` : **747 tests moteur et 376 tests
  client réussis**, un test manuel ignoré, aucun échec ; suite client en 347 s.
- `cargo check --locked --all-targets`, compilation native, formatage et
  `git diff --check` réussis.
