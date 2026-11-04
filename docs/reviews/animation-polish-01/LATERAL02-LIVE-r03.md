# LATERAL02-r03 — essai jouable de la Traqueuse

Le 2 novembre 2026, le joueur demande : « bon je sais pas, il faut qu eje vois
en jeu :) ». Cet accord autorise un essai jouable du candidat 03 présenté ;
il ne constitue pas une validation esthétique définitive ni un accord pour
modifier le Bastion ou les autres tenues.

## Lancement et périmètre

`Essai marche Traqueuse.cmd` appelle le vrai `Jouer.cmd` avec
`--essai-marche-traqueuse`. Une session neuve du camp démarre avec la Traqueuse
en armure Cerbère, sans arme, dans l’interface et avec le contrôleur du jeu.
Les déplacements, collisions et événements passent par les commandes normales.
La préparation utilise les attributs recommandés de la classe et l’équipement
initial normal du moteur. Aucune fonction réservée au Debug n’est nécessaire.

Le candidat concerne exclusivement le corps 5, l’armure 101, la palette d’origine,
sans arme active : les six phases gauche/droite et leur vrai repos latéral.
Le dos, la face, l’arrêt frontal et le rendu des autres configurations restent
ceux du jeu. Les armes et leurs prises ne sont pas validées par cet essai.
Le lancement habituel sans cet argument conserve son comportement précédent.

Les préférences de commandes sont lues pour garder les touches du joueur.
Les chemins de session, de commandes et d’affichage de l’essai sont séparés,
sous `ProjectRL/essai-marche-traqueuse/`. Les sauvegardes habituelles ne sont
ni chargées ni remplacées ; la récupération automatique est désactivée dans
cette session temporaire, qui se ferme sans sauvegarder une campagne.
Le rendu est à 64 px, 400 ms par pas, en fenêtre 1280 × 800.

## Ressource et rendu

`assets/animations/lateral-03/traqueuse-armor3.png` est la copie exacte de
`r03/candidate-03-atlas.png`, sans nouveau dessin. L’atlas est embarqué et préparé
avant l’affichage du jeu, via le chargeur commun ; le téléversement de sa texture
se fait sur le thread graphique. Le chemin de dessin sélectionne une case
préparée de l’atlas, sans lecture ni décompression de fichier. Une seule texture
et un atlas CPU de taille fixe sont conservés.

Le sélecteur est limité à cet essai et à la configuration présentée. Il exclut
les autres classes, armures, palettes et équipements armés. Aucun système de
casque indépendant ni ancien expérimental WALK04 n’est activé.

## Validation

- Tests ciblés de sélection du rendu, six phases et repos distinct.
- Test du véritable `AsciiApp` : deux déplacements réels, poses 0–2 puis 3–5,
  retour à la position initiale et arrêt frontal après 400 ms.
- Test existant des marches latérales normales : 200 cycles conservés.
- Capture native du vrai écran de jeu, avec audit de l’appel effectif au rendu
  du candidat pendant les six phases et absence de cet appel au repos frontal.
- Compilation Release et vérification à travers le lanceur depuis un autre
  répertoire ; empreintes des fichiers utilisateur contrôlées séparément.

Les preuves et le bilan de lancement sont dans
`artifacts/animation-polish-01/lateral-identity-02/live/`.
Le statut demeure un essai autorisé, en attente du jugement du joueur sur le rendu.
