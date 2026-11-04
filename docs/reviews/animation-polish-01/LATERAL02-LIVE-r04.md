# LATERAL02-r04 — cinq armures de la Traqueuse

Le 2 novembre 2026, après l’essai natif du cycle 03, le joueur indique que la
marche semble fonctionner et demande de l’adapter à toutes les armures. Il
signale aussi une différence d’identité entre l’arrêt et les déplacements
latéraux. Le lot couvre les cinq silhouettes d’armure de la Traqueuse ; le
Bastion reste inchangé.

## Présentation et périmètre

01 : tenue de base ; 02 : légère ; 03 : intermédiaire ; 04 : lourde ;
05 : très lourde. Le cycle latéral de la lourde est repris pixel pour pixel du
candidat 03 testé. Les quatre autres sont des figures complètes, avec les mêmes
appuis compacts. Aucun déplacement de bandes de jambes ni greffe de tête.
Les sources sont dans `assets/source-art/animation-polish-01/lateral-identity-02/r04/`.
Le mode de production et les quatre prompts exacts sont conservés dans
`LATERAL02-PROMPT-r04.md` (outil intégré image_gen, édition avec deux références).

Les faces, dos et repos frontaux reprennent les dessins canoniques VERT18 de
chaque armure, sans nouveau dessin. Le raccord utilise le même profil d’armure
et un point de contact au sol commun. Certaines armures intermédiaires n’ont
pas de famille VERT18 `one` : le rendu sans arme retombait alors sur une ancienne
image frontale avec une autre tête. L’essai emploie explicitement le corps et
les mains de la famille `ranged` canonique, sans ajouter d’arme.

## Essai jouable

`Essai marche Traqueuse.cmd` passe toujours par le vrai `Jouer.cmd`. La session
reste isolée des sauvegardes et préférences normales. F6 parcourt les cinq
tenues à travers la commande normale d’apparence du torse, sans modifier les
statistiques de l’armure réellement équipée ni avancer le temps.

Les ressources embarquées `assets/animations/lateral-04/` sont préparées avant
l’affichage. Le cache est borné aux cinq atlas de 672 × 384 ; le dessin choisit
une case préparée, avec filtrage nearest, sans chargement ni décompression.
Le portrait utilise le même repos frontal que la carte.

Ce lot reste un essai de la Traqueuse sans arme, palette d’origine. Le lancement
normal et les autres classes conservent leur rendu. Les prises d’armes et les
autres palettes ne sont pas couvertes. Les quatre déclinaisons sont à juger
dans cet essai ; le retour positif sur le cycle 03 ne vaut pas approbation
esthétique de chaque nouveau dessin.

## Vérification

Le bilan d’exécution est conservé dans
`artifacts/animation-polish-01/lateral-identity-02/live-r04/validation.json`.
Les captures natives auditent le profil et la case effectivement dessinés au
repos et pendant les déplacements, pour éviter de valider seulement un
sélecteur théorique. Les sauvegardes et préférences utilisateur sont contrôlées
par empreinte avant et après le lancement réel.
