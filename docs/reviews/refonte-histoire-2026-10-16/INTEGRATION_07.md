# Intégration du HUD et des familiers — HUD-07 / UX-PETS-06

Le 16 octobre 2026, le joueur autorise l'adaptation après les corrections
suivantes : classement des familiers par glisser-déposer, titre sans nombre,
bouton de repli « − » et davantage de surface pour le plateau de jeu.
Les deux habillages sont retenus, Métal sombre par défaut, avec les vrais dessins
de compétences et de statuts. Le bouton Familiers reste après Techniques.

Les effets et bonus affichés doivent venir du moteur. Aucun nouveau buff,
ordre, coût, durée, jet aléatoire, soin ou comportement n'est créé par le HUD.
Le classement est un état de présentation distinct de l'ordre des acteurs.
Les gestes de fenêtre ne dépensent aucun tour. Les commandes de combat,
d'ordre et de destruction conservent leurs validations et coûts existants.

Les copies de l'état de travail avant cette intégration sont conservées dans
artifacts/interface-native-07/before/. Elles incluent les modifications
préexistantes et ne représentent pas la version Git de base.

## Adaptation native

La fenêtre commune utilise les acteurs réels, leur identité visuelle existante,
leurs PV, la batterie et la liaison des drones, ainsi que l’échéance enregistrée
des réanimés. Les statuts viennent des instances actives. DP02 et O03 sont
projetés depuis leurs rangs effectifs ; les valeurs des protections et
malédictions utilisent le rang et la puissance de l’application enregistrée.
Les icônes compactes existantes gardent leurs couleurs dans les deux styles.
Quand un statut ne possède pas de dessin compact, son glyphe existant est réutilisé.

Le titre est simplement « Familiers ». Les boutons de repli et de fermeture sont
dessinés par des traits afin de rester lisibles avec les deux polices. Les six
points de la poignée permettent d’identifier les lignes que l’on peut déplacer.
Le classement conserve les survivants et ajoute les nouveaux compagnons à la fin.
Il est enregistré séparément dans les métadonnées JSON de suspension, avec une
valeur vide pour les anciennes sauvegardes ; le format binaire du monde reste intact.
La fenêtre conserve son classement entre fermeture et réouverture.

Les quatre ordres s’appliquent au groupe de drones par défaut. Le bouton à une
ou deux silhouettes choisit les ordres individuels ou communs déjà présentés
dans UX-PETS-05. F3 garde le cycle commun. Les réanimés restent autonomes ; aucun
sélecteur de comportement ne leur est attribué. Un ordre individuel vérifie la
même propriété, disponibilité et liaison que l’ordre commun et se rejoue depuis
le journal. Les réanimés ne bloquent plus les ordres des drones du groupe.
« Détruire » demande une deuxième confirmation et utilise la commande existante.
Fermer ou replier annule cette confirmation.

Les jauges Drones et Réanimés affichent séparément leurs limites réelles. La
fenêtre, les gestes de souris et les ordres de comportement n’ajoutent aucun
coût ; les commandes de destruction gardent le coût et les refus du moteur.

## Plateau et habillages

Les panneaux latéraux mesurent 222 et 236 pixels logiques. L’ancienne bande de
drones et sa réservation de 64 pixels logiques sous le plateau sont retirées.
Le plateau utilise tout le rectangle central disponible, avec sa caméra et ses
zones de clic calculées depuis le même agencement. La barre de six compétences
et le changement d’arme restent centrés sous le plateau, sans fond englobant.
Les commandes occupent deux rangées discrètes ; Familiers suit Techniques.

Les coordonnées sont placées à côté du lieu. Le compteur global de tours et le
nombre de tours dans le message d’attente sont supprimés. Les durées utiles aux
effets et actions restent présentes. Les effets du personnage sont affichés en
colonnes verticales d’icônes ; les noms et valeurs sont dans les infobulles.
À exactement un tour restant, l’opacité varie doucement entre 0,4 et 1, sur une
période de 1,3 seconde. Avec les animations réduites, seul le contour reste actif.

Métal sombre est la valeur par défaut, avec un grain discret calculé et mis en
cache indépendamment du hasard du jeu. Terminal PC utilise une police monospace,
des tons verts et des lignes de balayage fixes. Les deux habillages se choisissent
dans Options → Affichage → Habillage de l’interface, avec le mécanisme existant
d’application, confirmation, annulation et sauvegarde. Les anciens réglages
chargent Métal sombre sans perdre leurs autres préférences. Les choix QHD et 4K
sont ajoutés à la liste des résolutions fenêtrées.

## Vérification

Le diagnostic isolé `--ui-cold-native-hud` utilise trois drones et un vrai
réanimé, effectue un glisser-déposer par les entrées natives et vérifie que
l’état moteur reste identique. Il produit des captures des deux habillages en
1920 × 1080, 2560 × 1440 et 3840 × 2160, plus une partie sur un niveau généré.
La scène de laboratoire dispose d’une énergie augmentée uniquement pour cumuler
les effets de contrôle ; aucun contenu de partie ni sauvegarde utilisateur ne
reçoit ces valeurs. Les captures sont sous artifacts/interface-native-07/captures/.

Les compilations debug et release avec `--locked` réussissent. Les 59 contrôles
ciblés passent : familiers (6), affichage et options (16), caméra et projection
(20), barre de compétences et armes (14), groupe mixte et projections du moteur
(1), métriques et cache de texte (2). `cargo fmt --all -- --check` et
`git diff --check` passent également.

Après la remarque du joueur sur le centrage vertical, le lieu et les
coordonnées, l’itinéraire, les en-têtes des modules et le titre Familiers
utilisent les limites réelles de la police active. Dix-huit mesures des pixels
dans les captures natives des deux styles et trois résolutions vérifient ces
centres, avec une tolérance de deux pixels logiques pour la rastérisation.
Les encadrements et la surface jouable ne changent pas pendant cette correction.

La comparaison de géométrie avec les sources copiées avant intégration donne,
en présence de familiers : +23,71 % de surface jouable en Full HD, +23,38 %
en QHD et +20,02 % en 4K. Les dimensions de vision et les règles de perception
restent celles du moteur.

La suite binaire générale n’est pas validée : elle a été interrompue après
plus de 940 résultats, avec plusieurs échecs de génération, de reprise et d’UX.
L’assertion de largeur liée à cette adaptation est corrigée et les tests de
projection passent. Les échecs des jauges compactes et du raccourci de
comparaison d’inventaire ont été reproduits ; leurs chemins étaient présents
dans la copie de l’état de travail antérieur. Ils ne sont pas corrigés ici.
Le détail de la validation présente et les empreintes des exécutables sont dans
[VALIDATION_07.json](VALIDATION_07.json). Aucune sauvegarde utilisateur n’est
chargée ou modifiée par le diagnostic, aucun dessin de personnage ou de monstre
n’est créé, et aucun commit ni envoi Git n’est effectué.
