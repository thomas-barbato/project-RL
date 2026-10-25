# ARCANISTE-SAME-FACE03 — un même visage sous les coiffes

22 octobre 2026. Le joueur constate que le visage ne semble pas identique
sur les cinq tenues. La comparaison confirme des différences de mâchoire,
d’orientation et de proportions, particulièrement sur le 05 de profil.
Le dessin des coiffes avait laissé redessiner le visage avec elles.

La correction prend le visage du 01 comme référence commune. Le 01 reste
intégralement identique. Les têtes des 02–05 sont ajustées à ce visage, avec
la même direction de regard et la même expression. Les cinq corps, matières,
couleurs, colliers de nuque, mains et pieds restent exactement conservés.

Pendant la correction, le joueur précise : « ca me dérange pas si le visage
est un peu masqué par la capuche tu sais ». Une capuche peut cacher le front,
les tempes ou une partie des joues. Cette possibilité est conservée ; la
coiffe n’est pas dégagée artificiellement pour exposer toute la tête.

## Candidats et fabrication

Les candidats `AR-SF03-P01-r01` à `AR-SF03-P05-r01` gardent les numéros 01–05
et la progression tête nue, bandeau, capuche, capuche bordée, diadème.
Ils attendent leur propre revue. Aucune acceptation de nouveaux pixels ni
intégration n’est déduite de ce retour.

Les quatre corrections de tête passent par l’outil intégré `image_gen` avec
deux entrées : la tenue à corriger et un recadrage du visage du 01. Leurs
prompts exacts, rôles et empreintes sont conservés dans `prompts.json`.
Les sorties brutes restent disponibles dans le dossier source.

Pour empêcher une nouvelle dérive à la taille de jeu, le visage natif du 01
est conservé comme une couche commune et réutilisé dans un masque explicite
des traits centraux exposés : yeux, nez, bouche et bas du visage. Tous les
pixels de ce masque sont strictement identiques sur les cinq variantes.
Le front, les tempes, les bords des joues, le bandeau et les cheveux restent
hors de ce transfert pour préserver les parties cachées par la coiffe.
Cette preuve porte sur le masque facial
documenté, pas sur l’ensemble des pixels de la tête et de sa coiffe.

Les grandes sources conservent les têtes lisses corrigées par image_gen à
partir de la référence du 01. Aucun collage de visage ne recouvre leurs
bordures de capuche ou leur diadème. Le masque facial source est conservé
comme repère documentaire et n’est pas appliqué aux grandes illustrations.
Les sources ne sont pas des agrandissements des petits sprites. Le natif
reste la référence exacte aux limites de masque ; sa couche centrale ne
subit pas cinq réductions indépendantes. Aucun code ne dessine ou ne
recolore de nouveaux traits.

Les corps ne sont pas régénérés. Tous les pixels natifs hors du masque de
tête et toutes les lignes depuis y = 11 restent identiques à la revue
précédente. Les sources sont également identiques hors du masque de tête.
Chaque toile fait 64 × 64 ; les appuis restent à y = 61. Le cadrage du corps
est hérité et n’est pas recalculé à partir des nouvelles coiffes.

## Présentation et portée

L’aperçu présente les cinq tenues sur trois fonds, les visages avant/après,
les coiffes au zoom entier et les quatre classes dans la même grille native.
La rue est un montage sur une capture existante ; aucun contrôle d’éclairage
dans le moteur n’est prétendu. Les couches tête/corps se recomposent exactement
et restent des couches d’étude.

Les quinze compositions acceptées des trois autres classes restent figées.
Les dessins et preuves antérieurs restent conservés par empreinte. Casques
équipables indépendants, prises d’armes, variantes de couleur, animations et
intégration restent différés. Aucun effet, règle, objet ou texte joueur ne
change avec cette correction.

Vérification de présentation : les cinq images de la section sont chargées
et affichées à leur taille naturelle dans le navigateur intégré. La capture
de contrôle est conservée dans l’aperçu. Le contrôle technique ne vaut pas
acceptation visuelle des têtes corrigées.
