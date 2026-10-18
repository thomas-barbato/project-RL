# CAMP-NPC-01 r01 — PNJ du camp

18 octobre 2026. Demande du joueur : rétablir les pictogrammes au-dessus des
PNJ, permettre de revenir au camp et ajouter leur animation de repos.

## Retour au camp — correction intégrée

L'ancienne règle de descente irréversible refusait le passage réciproque.
La liaison entre `core:cryogenic_camp` à profondeur 0 et son secteur adjacent
à profondeur 1 est désormais réversible. Les anciennes cartes et les passages
plus profonds gardent leurs règles. Le format et la génération des sauvegardes
ne changent pas : les parties du camp déjà commencées bénéficient du correctif.

Le retour retrouve la même carte, les mêmes identités de PNJ et les mêmes
services. Aucun coût, voyage rapide, nouveau dialogue ou service n'est ajouté.
Le secteur industriel existant reste un raccord provisoire.

## Pictogrammes et respiration — aperçu à valider

Cette révision ne modifie pas les PNG acceptés. Elle reste activée uniquement
par `--ui-cold-camp-npc-review`, jamais par le démarrage normal du client.

- **CAMP-NPC-01 / NPC-01, officier :** bulle de dialogue, sans faux marqueur de
  quête disponible.
- **CAMP-NPC-02 / NPC-02, armurier :** pictogramme d'arme déjà présent dans l'UI.
- **CAMP-NPC-03 / NPC-03, médecin :** croix de soins, issue de l'icône d'ajout.
- **CAMP-NPC-04 / NPC-04, technicien :** pictogramme de réglage déjà présent
  dans l'UI, pour l'amélioration d'équipement existante.

Les pictogrammes de 18 px restent au-dessus du personnage visible, sans libellé
permanent, et suivent son déplacement affiché. Aucun signal n'est ajouté pour
un PNJ hors de la visibilité réelle du joueur.

La respiration reprend la cadence humanoïde déjà acceptée : deux poses de
1,5 seconde chacune, soit un cycle de 3 secondes. Le haut du corps descend
de 2 px natifs ; les jambes, les pieds et l'ombre restent fixes. Le bas fixe
recouvre la jonction à la taille, avec un chevauchement de 2 px. La
première pose est le dessin accepté exact. Portraits, inspection, mouvements
réduits et déplacement affiché gardent la pose fixe. Aucun tour n'est consommé.

La revue montre chaque PNJ dans le client réel, à côté du joueur, à 64 px par
case sans agrandissement. Les images de revue sont dans le sous-dossier
`camp-npc-review-01/` de la galerie courante. Les captures complètes sont
accessibles depuis chaque candidat.

L'accord sur le plan r09b et les dessins statiques reste intact. L'accord
visuel sur ces pictogrammes et cette respiration doit être explicite avant
leur activation dans le client normal, conformément à `AGENTS.md` :
« présenter […] tous les effets mécaniques ou visuels nouveaux ou modifiés
avant leur intégration ».

## Vérification

Les six tests ciblés du camp passent, dont le parcours sortie → sauvegarde à
l'extérieur → reprise → retour → sauvegarde au camp → reprise → nouvelle
sortie. Ils contrôlent la conservation du camp, des PNJ et des obstructions.
Le contrôle des marqueurs vérifie la visibilité réelle et l'isolation du mode
de revue. Deux tests supplémentaires vérifient la cadence de respiration et
la restauration du mode de revue après son utilisation.

Les vérifications GPU et les contrôles de non-régression des anciens passages
sont consignés dans `camp-npc-review-01.json` une fois exécutés. La suite globale
du client conserve ses 28 échecs antérieurement recensés ; elle n'est pas
déclarée entièrement verte par ces contrôles ciblés.
