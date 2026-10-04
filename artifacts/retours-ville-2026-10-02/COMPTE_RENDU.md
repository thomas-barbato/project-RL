# Ville intérieure, population, butin et proposition visuelle

Version locale livrée : génération 140, `target/release/project-rl.exe`.

## Résultat

Les nouvelles parties commencent dans la ville intérieure, à côté d'Elias.
Il est visible et accessible avec E dès le premier écran. La ville conserve
son emplacement aléatoire sur l'un des quatre bords. Le recyclage reste
présent dans le monde mais n'est plus le départ des nouvelles parties.

La ville compte cinq habitants mobiles et un soigneur avec sa routine.
Le marchand et les contacts de quête restent à leur poste. Les habitants
utilisent le système existant : ils avancent avec les tours du jeu.

Les modèles nommés d'armes et de protections ont un pool élargi, séparé
du tirage des bonus. Les modèles plus avancés peuvent apparaître jusqu'à
deux couches plus tôt et les modèles précédents restent admissibles.
Les poids continuent à favoriser la force locale. Les porteurs gardent
leur famille d'arme et déposent leur arme réellement équipée. Les caches
humanoïdes peuvent fournir les autres armes et armures ; les animaux et
robots n'obtiennent pas de butin humanoïde.

Le survol et F1 > Symboles décrivent les nouveaux décors. Comptoirs,
étagères, présentoirs et distributeurs extérieurs sont des obstacles sans
service commercial. Les marquages de transport et enseignes sont franchissables.

Pour découvrir le nouveau départ, la population et le pool de modèles,
choisir une nouvelle partie ou un nouvel essai dans `Essayer_expedition.cmd`.
Les anciennes sauvegardes gardent leur carte, leur population et leur pool.
Le nouveau guidage donne les coordonnées de l'entrée aux anciennes parties
et indique Elias lorsque le joueur entre dans la ville.

## Vérification de la sauvegarde signalée

Une copie de `city-test-run.json` a été restaurée sans consommer ou écrire
la sauvegarde utilisateur. Elle était en génération 139. Le joueur se
trouvait en X 51 / Y 25 et n'avait pas atteint l'intérieur.

La porte d'entrée existe réellement en X 129 / Y 54. Elias est présent
en X 180 / Y 60. Le marchand, le soigneur et l'ancien habitant mobile sont
également présents. L'inventaire contient bien trois Veyr R12 : l'ancienne
sélection n'autorisait qu'un modèle de fusil par couche pour ces porteurs.

## Proposition artistique à examiner

[Comparaison native](proposition/cold-start.png) : rendu actuel à gauche,
proposition de terminal industriel à droite. Cette piste n'est pas sélectionnée
par le renderer des parties normales.

- Sol plus sombre et discret, avec une grille logique conservée.
- Machines et mobilier reliés en volumes sur plusieurs cases.
- Mobilier secondaire gris ; couleur réservée surtout aux personnages,
  au butin et aux installations réellement utilisables.
- Pièces composées selon leur fonction : ateliers, étals, réserves, logements.
- Voyants et animations sobres comme étape suivante, après validation.

Références de conception : [Furnishing a/the Dungeon](https://www.gridsagegames.com/blog/2014/07/furnishing-athe-dungeon/)
et [ASCII vs. Tiles](https://www.gridsagegames.com/blog/2015/02/ascii-vs-tiles/),
blog officiel de Cogmind. Les formes du prototype sont dessinées dans le code
du projet ; aucun asset de Cogmind n'est repris.

## Validation

- Départ protégé dans l'intérieur, Elias visible, acceptation réelle de son
  enquête et sauvegarde/reprise : vérifiés sur les quatre orientations.
- Déplacement des cinq habitants et du soigneur : vérifié sur les quatre orientations.
- Diversité des modèles, bonus de 0 à 6, provenance des objets, armes des
  porteurs et remplacement des caches : vérifiés par tirages déterministes.
- Aller-retour réel dans le souterrain avec soins limités, butin conservé,
  guidage et reprise : vérifié.
- La dernière sélection de corrections donne 10 tests réussis ; le diagnostic
  de la copie utilisateur ajoute un test de reprise réussi.
- Vérifications supplémentaires des modèles, ressources des armes, régions,
  circulation et anciennes générations exécutées dans `tests-final.log`.
- Quatre tests historiques d'empreinte 129/130/132/133 restent en échec.
  Leurs valeurs obtenues et attendues correspondent exactement aux échecs
  déjà présents dans `artifacts/interieur-aleatoire-2026-10-02/tests-client-initial.log`.
  Les références historiques n'ont pas été réécrites.
- `cargo check --locked --all-targets`, formatage et `git diff --check` : réussis.
- Builds debug et release : réussis. Deux avertissements release préexistants
  concernent un import et des méthodes inutilisées.
- [Dernière capture native du départ](depart-final/cold-start.png), avec Elias
  adjacent, coordonnées à droite et intitulé « Mégapole de surface » : inspectée.
- La suite complète n'a pas été relancée ; les résultats ci-dessus sont ciblés.

Aucun commit, push ou changement de sauvegarde utilisateur effectué.
