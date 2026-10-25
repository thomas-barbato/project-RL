# WALK09 — démarche commune, acceptée le25 octobre2026

Le joueur précise que les personnages doivent partager une démarche cohérente,
et signale des protections de jambes absentes dans certaines poses. Le nouvel
essai reprend un unique cycle de référence du Bastion pour les cinq classes
et leurs25 tenues. Les350 figures sont entières, avec un vrai repos et six
phases par direction ; aucun déplacement de bandes ou greffe de jambes.

Les orientations sont préparées séparément à partir du même gabarit. Le
conditionnement remet les figures complètes au gabarit de leur tenue, avec une
très petite variation verticale commune. Les pixels des protections centrales
des jambes appartiennent au corps : bouger un bras pour tenir une arme ne peut
plus les emporter. La recomposition des parties reproduit exactement chaque
figure source. Le joueur accepte ce résultat pour le moment ; les sources restent disponibles pour toute retouche ultérieure.

Une correction complémentaire restitue les plaques cuivrées des cuisses du
Bastion5 dans ses phases de marche (`armor-repairs.json`). Les sources
initiales sont conservées pour comparaison. Les armes sont les dessins
partagés existants, composés avec les prises réelles.

## Examiner le résultat

`Voir_animations.cmd` ouvre le catalogue courant en vue « Cote a cote ». Les
cinq personnages sont synchronisés dans les deux sens, avec la même tenue de
progression et la même arme. Choisir les cinq niveaux d'armure pour comparer
les25 tenues. « Image + » permet d'examiner la même phase pour tous.

« Nouvel essai » sélectionne WALK09 ; « Precedent » conserve la comparaison
avec WALK08. Quitter « Cote a cote » pour examiner et valider une configuration
individuelle. La vue de groupe n'enregistre aucune validation collective.
Les nouveaux cas portent `COMMON09-r01` et n'héritent pas des accords anciens.

Chaque animation parcourt une seule case64px, en400ms par défaut, avec les
choix120/200/400ms. Elle revient au repos à l'arrivée. Les mouvements haut/bas,
les monstres et les formes ne sont pas remplacés par ce lot. Le lecteur normal utilise désormais les25 tenues WALK09 avec l’équipement réel.

## Sources et preuves

- Génération intégrée, avec références et consignes exactes :
  `direction-jobs-v1.json`, `direction-jobs.json`, `generation-*.json`,
  `armor-repairs.json`.
- Sources : `assets/source-art/player-walk-review-09/`.
- Ressources candidates : `assets/animations/walk-09/player/`.
- Conditionnement, vérifications et limites : `conditioning.json`, `checks.json`.
- Planches des cinq familles : `*-all-poses-x2.png`.
- Animations de comparaison sans armes : `common-outfit-1.gif` à
  `common-outfit-5.gif`. Le catalogue utilise les armes équipées réelles.

Statut : accepté explicitement dans la conversation, puis intégré au lecteur normal.
`acceptance.json` conserve la citation, le périmètre et les empreintes des ressources.
Les variantes de monstres et les nouveaux cycles de formes restent en revue séparée.
L’audit demandé des cultistes est consigné avec les preuves d’intégration.

Revue ultérieure du25 octobre : le joueur signale un écart de gabarit entre
la face et les déplacements latéraux. L'audit confirme une hauteur et des
proportions différentes ; le maintien de la hauteur dans un cycle ne suffit
pas à assurer cette continuité. Un premier candidat du Bastion est préparé
sans remplacer le jeu : `../player-walk-10/REVUE.md`.
