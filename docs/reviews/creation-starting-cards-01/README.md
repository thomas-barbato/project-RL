# CREATION-STARTING-CARDS01 — 26 octobre 2026

Le joueur demande les images de l'équipement et des compétences de départ dans
la partie droite de la création, avec statistiques, descriptions et effets au
survol. Il demande aussi de réorganiser cette partie pour supprimer le grand
espace vide. Le grand personnage retiré et le retrait du Technomancien restent
conservés. Ces demandes autorisent cette intégration d'interface.

La description de classe est suivie de deux groupes de cartes compactes :
équipement puis compétences, avec icônes64px, noms, quantités, type actif/passif
et rang. Les attributs restent visibles en dessous. Le panneau peut défiler dans
les petites tailles d'interface. La fiche au survol occupe la liste de gauche,
au-dessus des boutons de création, et défile à la molette lorsque nécessaire.
Entrer dans la fiche permet de la lire sans perdre le survol ni sélectionner
la classe située derrière. Cliquer une image ne donne ni objet ni compétence.

À sa demande de finition visuelle, la présentation reprend les couleurs et
surfaces du thème actif : noms en gras, icônes natives encadrées, marges régulières,
sous-titres de maniement/quantité, séparateurs discrets entre sections et en-tête
plus marqué dans les fiches. Les cadres ont un peu plus d'air en fenêtre normale
et se resserrent en petite fenêtre, sans réduire les dessins64px. Les boutons
restent séparés des lecteurs et les effets de survol restent cohérents.
Le joueur autorise ensuite l'utilisation de tout l'écran. Le cadre n'est plus
limité à1120×540 : il utilise la hauteur disponible et presque toute la largeur,
avec une limite de lecture de1560px sur les écrans larges. La colonne des classes
reste contenue, les détails se répartissent sur deux colonnes de cartes et les
fiches bénéficient de la hauteur libérée. Les dessins restent en64px natifs.
Les deux colonnes sont équilibrées verticalement en fonction de l'espace libre,
avec les actions ancrées en bas ; elles ne restent pas tassées en haut d'un grand écran.

Un petit état moteur isolé reproduit les règles du véritable nouveau départ,
y compris les réserves communes et le filtrage actuel des anciens objets de
test. Le résultat est mis en cache par classe, attributs et graine. Aucun état
de la partie ouverte, tour, RNG, historique, sauvegarde ou inventaire existant
n'est modifié. Les descriptions d'objets et valeurs de compétences réutilisent
les lecteurs existants et leurs calculs moteur. Les rangs suivants n'affichent
que les valeurs qui changent, avec les autres modificateurs conservés.

Les dessins, descriptions et effets existants sont conservés ; aucune nouvelle
illustration, règle de départ ou mécanique n'est introduite. La fiche de Mue
prédatrice utilisait toutefois des valeurs de base pour la durée et les dégâts
des actions de forme. Un lecteur commun reprend les calculs d'exécution pour
inclure les attributs, le niveau et les bonus de classe/passif, aux rangs actuel
et suivant. L'exécution réutilise ces mêmes calculs, sans changement de formule.

Sources : `src/character_creation_details.rs`, `src/agent_skill_window.rs`,
`src/inventory_terminal.rs` et `src/game/agent_transform_cast.rs`.
Captures isolées : `artifacts/creation-starting-cards-01/`.
Résultats des tests, builds et contrôles visuels : `validation.json`.

Validation finale :40 tests ciblés réussis, contrôle de compilation verrouillée
de toutes les cibles, formatage et diff sans erreur ; Debug et Release reconstruits.
Captures de référence : `release-main/`, `final-wide/`, `final-small/`,
`final-skill-card/`, `final-item-card/`, `final-meta-scroll/` et
`final-ui150-typography/` sous le dossier de captures. Elles couvrent1280×800,
960×540,1920×1080 et l'interface agrandie à150%. Les libellés d'attribut et leur
valeur restent ensemble lors du retour à la ligne. Les itérations précédentes
ne constituent pas l'aperçu de référence. Aucun paquet portable n'est reconstruit.
