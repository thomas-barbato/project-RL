# Anémone des caves

27 septembre 2026 — génération 111.

## Identité et habitat

Le nom **Anémone des caves** a été explicitement validé après remise en
question d'« Agrippeur ». Il remplace cette appellation pour l'ancienne fiche
« Hymèle à vrilles », sans renommer les autres espèces. Corps radial fixé
au sol, vrilles souples : il s'agit d'un organisme vivant, pas d'un robot.

- Biome de recherche, profondeurs 2 à 4, exclusivement dans l'eau peu profonde.
- Une tentative de rencontre solitaire par région ; aucun ajout si aucun
  habitat accessible n'est disponible, ni dans les villes.
- Au moins 20 cases de distance de Manhattan des passages ; les cases occupées,
  lieux et butins existants restent réservés.
- Niveau de référence 8, bande de catalogue 8–12. Pas encore de niveau individuel
  aléatoire ni d'ajustement au niveau du joueur.
- Valeurs d'essai : 18 PV, 1 d'armure, 3 dégâts cinétiques, 1 de pénétration,
  14 XP de base. Corps physiquement fixé au sol, sans poursuite ni patrouille.

L'ajout n'invente pas de jardins techniques, de nouvelle faction ou de quête.
La génération utilise le flux indépendant de faune après les autres placements.

## Prise annoncée

Une cible perçue au contact déclenche la préparation. Les huit cases voisines
sont annoncées, sauf obstacles, coins fermés et cases protégées. La prévisualisation
ne révèle aucun acteur ou terrain invisible. Reculer hors de cette couronne
avant la prochaine action de l'Anémone évite la prise.

À sa prochaine action, l'Anémone frappe les occupants de cette zone, chacun
au plus une fois. Les règles ordinaires de précision et d'esquive s'appliquent.
Un coup réussi sur une cible survivante à locomotion compatible applique
`core:locomotion_hindered` : les déplacements coûtent au moins 2 UT, sans
interdire de marcher ou d'attaquer. Un coup entièrement absorbé par l'armure
peut entraver, puisqu'il a bien touché. Aucun second jet de stabilité n'est
ajouté à cette prise naturelle.

L'entrave réutilise sa durée existante de deux phases : la phase du coup compte
dans cette durée. Elle ne se cumule pas et ne se renouvelle pas. À expiration,
la protection locomotrice existante dure une phase. Il s'agit d'un état bref
sur la victime, pas d'un lien permanent à maintenir entre deux acteurs.

Après le coup, même dans le vide, l'Anémone récupère pendant trois occasions
d'action. Elle ne suit pas le joueur pendant ce temps, ni ensuite. Une longue
action du joueur peut faire avancer plusieurs phases ennemies. Une préparation
déplacée ou dont la visée est désormais bloquée est annulée.

## Présentation, butin et sauvegardes

Glyphe `e`, nom dans le ciblage et la légende, contact hostile dans CAPTEURS.
L'annonce « Vrilles déployées · éloignez-vous » précède la récupération.
Une cible entravée porte le badge de ralentissement existant et affiche
l'état ENTRAVE dans ses informations.

Pas d'arme cachée pour fabriquer cet effet, pas d'équipement humain lâché.
Les restes animaux utiles attendent la définition des ressources et de leur
usage ; aucune recette d'artisanat n'est introduite.

Identifiant stable `core:cave_anemone`, famille de travail `core:radial_fauna`.
Le comportement `telegraphed_grasper` et la zone `Adjacent` sont ajoutés en fin
des énumérations sérialisées. L'annonce réutilise `Aiming` ; les statuts et
la récupération existaient déjà. Le format du cache moteur reste v6.

Les générations 110 et antérieures retirent seulement cette espèce du profil
de recherche avant génération et empreinte. Elles gardent leurs populations,
y compris les Vers cuirassés en 110. Une nouvelle partie est nécessaire pour
rencontrer naturellement l'Anémone.

Diagnostics isolés : `--ui-cold-cave-anemone` (annonce),
`--ui-cold-cave-anemone-grasp` (prise réelle et badge). Ils ne modifient pas
les sauvegardes du joueur.

## Vérifications du lot

- 716 tests moteur réussis, dont annonce, dégâts uniques, esquive, récupération,
  absence de cumul/renouvellement, protection temporaire et déplacement ralenti
  réellement possible. Couvert, cases protégées, déplacement de la préparation,
  soins humanoïdes exclus et simulation hors zone sont aussi couverts.
- Suite complète client : 350 réussis, un test manuel de suspension externe
  ignoré. Après simplification des textes du journal, les trois tests ciblés
  de l'Anémone passent, dont le nouveau contrôle des messages français et de
  la disparition du badge.
- Placement déterministe sur 32 graines ; comparaison de régions complètes
  avant/après et absence dans les villes. Reprises 109, 110 et 111 par cache
  et rejeu ; empreinte 110 historique inchangée.
- Annonce et entrave/récupération reprises par instantané, avec comparaison
  des événements et états des quatre actions suivantes.
- Captures natives inspectées : annonce, prise et légende. Le texte technique
  initial a été corrigé et la capture de prise refaite. Contraste validé.
- Formatage, compilation, vérification de toutes les cibles et diff contrôlés.
