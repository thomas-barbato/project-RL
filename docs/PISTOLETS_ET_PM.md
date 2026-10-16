# Pistolets et pistolets mitrailleurs

5 octobre 2026. Génération 146. Équilibrage initial à éprouver en partie.

Deux nouvelles familles utilisent les dessins industriels déjà retenus. Chaque
famille comporte six modèles. Les couleurs sont les sept variantes cosmétiques
de génération 145 : elles ne changent ni le nom ni les statistiques du modèle.

| Niveau | Pistolet | Dégâts par tir | PM | Dégâts par projectile |
|---|---|---:|---|---:|
| 1 | Cogneur P-01 | 4 | Crache-fer PM-01 | 2 |
| 2 | Cogneur P-02 | 6 | Crache-fer PM-02 | 3 |
| 3 | Cogneur P-03 | 8 | Crache-fer PM-03 | 4 |
| 4 | Cogneur P-04 | 10 | Crache-fer PM-04 | 5 |
| 5 | Cogneur P-05 | 12 | Crache-fer PM-05 | 6 |
| 6 | Cogneur P-06 | 14 | Crache-fer PM-06 | 7 |

Le pistolet tire un projectile, coûte une munition, porte à cinq cases selon la
distance euclidienne et apporte +5 en précision. Sa masse est de 1,2 kg.
Le PM tire une rafale de trois projectiles, coûte trois munitions, porte à quatre
cases et applique -10 en précision. Sa masse est de 1,8 kg. Les dégâts sont
perçants et restent soumis aux règles normales de précision, protection et
effets. Un mur bloque le tir. Le coût de toute la rafale est payé à son départ.

Ces armes occupent une seule main, gauche ou droite. L'autre main peut porter
une arme de mêlée ou de tir. Une arme à deux mains reste équipée dans son slot ;
le joueur choisit la paire de mains ou cette arme. Une attaque emploie seulement
l'arme sélectionnée. Le rendu utilise la pose commune de pistolet levé ; les
illustrations de terrain et d'inventaire distinguent le pistolet et le PM.

Les douze modèles rejoignent les récompenses aléatoires des monstres, les caches
humanoïdes, le stock blanc et les paris enchantés des marchands d'équipement.
Les stocks gardent leurs règles de niveau et les caches leur choix par famille.
Les niveaux voisins suivent les règles de génération 140. Les ennemis conservent
leur ancien équipement de combat et les classes leur équipement de départ.

Prix de base blancs par niveau : pistolets 65 / 100 / 150 / 220 / 320 / 470 ;
PM 95 / 140 / 210 / 305 / 440 / 640. Les bonus et règles commerciales habituels
appliquent ensuite leurs multiplicateurs.

Les parties de génération 145 ou antérieures gardent leur catalogue et leur
hasard historiques. Les nouvelles familles sont disponibles dans une nouvelle
partie de génération 146. Le format de suspension et le cache RLWS v12 restent
inchangés. Les sauvegardes de référence 143, 144 et 145 sont archivées dans
`artifacts/pistolets-pm-integration-2026-10-05/avant` pour vérifier leurs empreintes
et leur reprise.

Le contenu natif est dans `content/core/weapons`, `visuals`,
`equipment_loot/one_hand_firearms.json5` et `locales/fr_one_hand_firearms.json5`.
Les index graphiques nouveaux sont ajoutés en fin de catalogue pour conserver
les index historiques. Aucun dessin de personnage n'est modifié à cette étape.
