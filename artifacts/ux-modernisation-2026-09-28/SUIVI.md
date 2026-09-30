# Modernisation UX — 28 septembre 2026

La refonte est appliquée dans le projet local, en Rust/Macroquad. Elle reprend les améliorations de confort de [l'audit initial](C:/Users/User/Desktop/project-RL/artifacts/ux-audit-2026-09-28/RAPPORT_UX.md) et unifie la présentation des écrans. Les retours sonores restent une piste ultérieure, comme prévu dans l'audit.

## Changements réalisés

| Parcours | Résultat |
| --- | --- |
| Accueil | Action principale visible au repos, boutons et surfaces harmonisés, motif existant conservé. |
| Création | Démarrer directement avec un profil recommandé ; personnalisation séparée, points disponibles expliqués, boutons impossibles désactivés, répartition préservée lors d'un retour. |
| Sécurité | Confirmation avant de recommencer une partie active, aucune mise en équipement au survol, défilement marchand sans achat. |
| Présentation | Surfaces sombres, turquoise pour les actions et sélections, rouge pour les actions destructrices, focus clair, contraste renforcé et composants communs. |
| F1 | Onglets Commandes essentielles, Symboles et Règles utiles ; touches lues dans la configuration réelle à chaque affichage, avec prise en compte des réattributions, de la disposition du clavier et des boutons de souris. |
| Lecture | Fiches de personnage, objets, techniques, quêtes et archives défilantes ; textes complets consultables. |
| HUD | Actions cliquables, points de compétence visibles, inspection détaillée en petit format, historique des messages observés regroupé par tour. |
| Inventaire | Emplacements plus explicites, comparaison volontaire d'une arme avec le canal choisi, fiche complète, actions adaptées à l'objet. |
| Commerce | Fiche avant achat, raisons des refus accessibles, sélection au clavier et à la molette ; les bonus inconnus restent masqués. |
| Compétences | Filtre Disponibles, coût réel d'apprentissage, montée de niveau signalée sans ouverture forcée ; menu rapide avec ressources et blocages connus. |
| Quêtes | Quêtes actives avant les terminées, objectif suivi choisi par le joueur et conservé lors de la suspension. |
| Dialogues et services | Panneaux adaptés au contenu, lecture intégrale du dialogue, PV après soin affichés à la clinique. |
| Compagnons et laboratoire | Ordre actif lisible, commande reconfigurable, menu d'accès aux essais et à la remise à zéro. |
| Fin de partie et reprise | Bilan avec progression et derniers événements, relance, choix d'un autre profil, historique et accueil ; diagnostic accessible en cas d'échec de reprise. |

L'historique conserve au plus 600 messages observés. Les nouveaux champs de suspension sont facultatifs ; les anciennes sauvegardes restent lisibles. Les informations révélées restent soumises aux règles existantes de perception.

Les nouveaux raccourcis proposés par défaut sont F3 pour les ordres des compagnons, F4 pour les détails, F5 pour le laboratoire et V pour l'historique. Lors de la migration d'anciennes préférences, une touche libre est choisie si la touche proposée est déjà occupée. F1 affiche les affectations réellement retenues.

## Correction des défauts visuels signalés

Les traits décoratifs verticaux ajoutés aux cartes sélectionnées, aux réglages et aux listes ont été retirés. Une carte sélectionnée garde son contour, sans seconde barre parallèle.

Les lignes de compétences ont été agrandies et leurs deux textes placés dans des zones intérieures mesurées : le sous-titre reste sur le fond de sélection. Les libellés des boutons sont ajustés à leur largeur et hauteur utiles. Dans un filtre étroit, le nom complet prime sur l'icône redondante.

![Personnalisation des attributs corrigée](C:/Users/User/Desktop/project-RL/artifacts/ux-modernisation-2026-09-28/correctifs-visuels/character-attributes-small/cold-start.png)

![Compétences corrigées en petit format](C:/Users/User/Desktop/project-RL/artifacts/ux-modernisation-2026-09-28/correctifs-visuels/skills-small/cold-start.png)

## Vérification

Compilation et formatage validés. Les 765 tests de bibliothèque ont réussi. Les 56 vérifications ciblées de la refonte ont réussi, dont 10 nouveaux tests UX couvrant les raccourcis réattribués, la confirmation de relance, le survol, la création, la lecture, la suspension, les quêtes et le bilan. Après les corrections visuelles, les 5 tests de disposition ont également réussi ; les résultats supplémentaires sont conservés dans les journaux de ce dossier.

Les captures couvrent 960 × 540, 1280 × 800 et un grand format réellement obtenu de 1920 × 1061, ainsi que l'interface agrandie et le contraste renforcé. Les écrans concernés ont été inspectés visuellement. L'index [captures-validation.json](C:/Users/User/Desktop/project-RL/artifacts/ux-modernisation-2026-09-28/captures-validation.json) privilégie la capture la plus récente de chaque scène. Le contrôle automatique recherche seulement la présence de texte dans certaines zones ; il ne constitue pas une mesure normalisée de contraste.

Deux tests de combat échouent également avec les fichiers d'interface antérieurs à cette refonte : `combat_balance_surface_roles_remain_distinct_with_starting_white_weapons` et `combat_balance_white_equipment_matrix_matches_real_impacts`. Ils attendent une attaque acceptée alors que le moteur répond `InsufficientMatter { required: 1, available: 0 }`. Voir [la vérification de référence](C:/Users/User/Desktop/project-RL/artifacts/ux-modernisation-2026-09-28/VERIFICATION_BASELINE.md). La suite complète n'est donc pas annoncée entièrement verte.

Le travail préexistant est conservé ; les fichiers d'interface initiaux et le diff initial sont archivés dans `baseline/`. Aucun commit, push ou déploiement.

