# ARCANISTE-MAGE-HEADS02 — coiffes de mage

22 octobre 2026. Le joueur apprécie les tenues de l’Arcaniste, mais demande
des têtes évoquant davantage un magicien de jeu vidéo. Les anciens casques
ne conviennent pas. Ce retour conserve la direction des vêtements ; il ne
valide pas de nouveaux pixels de tête ni une intégration au jeu.

## Périmètre de la reprise

Les cinq corps de `arcaniste-channeler-wave-01/` sont conservés exactement à
taille native. Les versions 01 et 02 restent intégralement identiques. Seules
les têtes des 03–05 changent : capuche prune, capuche à bordure ivoire, puis
capuche et diadème de mage. Le visage humain clair reste visible.

Le manteau prune, la céramique claire et les masses cuivrées des avant-bras
ne changent pas. Les capuches doivent rester compactes, proches des épaules,
sans agrandir la tête ni allonger la nuque. Le visage et la coiffe doivent
rester lisibles dans une case de 64 × 64, avec de grandes masses simples.

Les candidats `AR-MH02-P01-r01` à `AR-MH02-P05-r01` gardent les numéros 01–05.
Les anciens identifiants sont conservés dans le manifeste pour retrouver
chaque tenue. Les nouvelles têtes attendent leur propre revue visuelle.

## Fabrication et vérification

Les trois dessins de tête sont des éditions effectuées par l’outil intégré
`image_gen`. Les prompts exacts, rôles des références et empreintes sont dans
`prompts.json`. Les sorties brutes sont conservées dans le dossier source.
Le script ne peint et ne recolore aucune matière.

Le cadrage de chaque original reste fixe. Seuls les pixels contenus dans un
masque de tête explicite sont transférés vers l’original. Tous les pixels
natifs hors de ce masque, ainsi que toutes les lignes à partir de y = 11,
restent strictement identiques. Les versions 01 et 02 ne sont pas régénérées.
Le masque source est conservé avec la composition haute résolution ; la
composition native reste la référence exacte à sa limite de masque.

Les appuis restent à y = 61. La pointe de la capuche du 03 atteint y = 1,
soit 61 px de silhouette solide contre 60 px pour les quatre autres tenues ;
le corps, le visage et le raccord de nuque ne sont pas étirés. Le champ de
cadrage hérité décrit la projection de l’ancien corps ; les boîtes natives
actuelles sont mesurées séparément dans le manifeste.

Les couches d’étude tête/corps se recomposent exactement. Elles ne constituent
pas encore un masque de remplacement de casque équipé validé. La tête montrée
est celle fournie par le torse, conformément à la règle déjà acceptée.

Les planches présentent les cinq tenues sur fond sombre, clair et sur un
montage de rue, ainsi que les têtes avant/après et les quatre classes à même
grille native. Les zooms entiers sont en Nearest. Le montage de rue ne prouve
pas un contrôle sous l’éclairage du moteur.

Les quinze compositions acceptées du Bastion, du Technomancien et de la
Traqueuse restent figées. Les sources et captures précédentes sont protégées
par empreinte. Casques équipables indépendants, variantes de couleur, prises
d’armes, animations et intégration restent différés. Aucun texte joueur,
objet, compétence, effet ou règle de jeu n’est modifié.

Contrôle de présentation : les cinq images de la section sont chargées et
affichées à leur dimension naturelle dans le navigateur intégré. La capture
est conservée dans l’aperçu. Les nouvelles têtes restent en attente de l’avis
du joueur ; les contrôles techniques ne constituent pas une validation visuelle.
