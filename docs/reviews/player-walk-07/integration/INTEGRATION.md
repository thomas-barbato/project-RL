# WALK07 — intégration du24 octobre2026

> Revue ultérieure du24 octobre : le joueur rejette le torse gauche de M100
> dans la révision R07 intégrée ci-dessous. La proposition R08 est préparée
> séparément dans `../m100-torso-r08/`, puis validée par « ok tres bien je
> valide :) ». Elle remplace les sept poses gauches R07 ; la droite reste
> identique. Accord et contrôles : `../m100-torso-r08/acceptance.json` et
> `../m100-torso-r08/installation.json`. Cet historique ne vaut pas validation
> du torse R07.

Le joueur valide les69 autres cycles de la revue groupée et demande de corriger
M100/M104, retirer les fichiers inutiles, puis intégrer au jeu. Son message exact
et la distinction entre acceptation des69 cycles et autorisation des deux
corrections sont conservés dans `authorization.json`.

## Résultat

Les129 silhouettes de base de campagne et le drone canonnier, soit130 cycles
et1820 poses, utilisent leurs dessins entiers dans le rendu normal. La banque
est dans `assets/animations/walk-07/` et figure dans l'inventaire du paquet joueur.
Le lecteur expérimental WALK04 reste désactivé. Les poses de joueur WALK06
déjà intégrées gardent leur fonctionnement et leurs pixels.

M104/MON106 reçoit une réaffectation de ses deux séries directionnelles, sans
retourner ses pixels ni inverser le temps. M100/MON099 reçoit sept nouvelles
poses entières pour la gauche ; ses sept poses droites restent identiques en
RGBA. La génération utilise l'outil intégré, avec source et consigne dans
`generation-r07.json`. Les planches avant correction sont conservées parmi les
sources pour vérifier ces deux invariants.

Le réglage400ms, les choix120/200/400ms et les trois phases par case restent
ceux du contrôleur existant. Le repos distinct est sélectionné à l'arrivée,
sans délai de520ms ajouté au jeu. Les mouvements haut/bas conservent les
dessins de face. Le corps et l'ombre des petites silhouettes partagent un
calage vertical fixe centré, au repos initial comme pendant le cycle.

Le périmètre de rendu animé est celui qui a été présenté : palettes de base
des129 modèles et drone canonnier. Les variantes de couleur non présentées,
les anciennes banques, autres drones, PNJ et formes non revus conservent leur
rendu existant. Les identités, choix de génération, caractéristiques, tours,
déplacements logiques et sauvegardes ne changent pas.

## Vérifications

- `native-checks.json` : tous les atlas installés correspondent exactement aux
  sources sélectionnées ; les1820 poses et le calage fixe sont contrôlés sans
  dépendance à un aperçu ou une capture sous `artifacts/`.
- `engine/` : captures réelles du moteur, commandes de rendu des28 poses des
  deux modèles corrigés, contrôles d'un déplacement logique d'une case,
  maintien de la simulation et absence d'écriture des préférences/campagnes.
- Tests Rust : les tests de banque, directions, repos, corrections, calage,
  durée et séparation du temps de présentation passent. Le filtre `walk`
  donne22 réussites et2 diagnostics ignorés. Il rencontre aussi l'échec déjà
  signalé de `world_context_attack_without_ammo_never_walks`, à
  `src/world_context.rs:418` ; ce module n'est pas modifié par cette intégration.
- La compilation Release et le paquet autonome sont contrôlés. Le bilan
  final du paquet et du nettoyage est conservé dans ce dossier.

Les accords antérieurs, sources et preuves utiles sont conservés. Le nettoyage
porte sur les exports reproductibles devenus inutiles. Les compositions d'armes
de joueur encore utiles à une autre revue et les données de l'éditeur de
cartes sont protégées.
