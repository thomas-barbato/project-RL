# Lumières locales — intégration et validation

## Rendu intégré

Les lampes murales existantes projettent un halo blanc court sur leur face
ouverte. Les serveurs et terminaux actifs diffusent un vert doux ; les contrôles
prêts possèdent une lumière ambre. Les couleurs appartiennent au décor et
restent indépendantes du thème de l'interface. Seuls les voyants des serveurs
et des terminaux actifs varient lentement, sans extinction ni clignotement brutal.
Les terminaux hors ligne et les consoles inactives ne créent aucun halo.

L'éclairage est attaché aux objets déjà présents, avec une portée de 1,5 à
2,3 cases. Il passe derrière les acteurs et sous le masque de vision arrondi.
Seuls une source actuellement visible et un sol actuellement visible peuvent
être éclairés. Les murs, portes fermées, cellules inconnues et angles opaques
bloquent la diffusion. Une lampe murale éclaire uniquement le côté où sa
bande lumineuse est dessinée. Le mode de mouvement réduit fixe son intensité.

La géométrie est mise en cache en dehors des données de partie. Une modification
de la caméra, de la perception, d'une porte ou d'un décor perçu la reconstruit.
Le halo utilise une texture de 64 × 64 pixels préparée au démarrage, puis des
quads découpés aux cellules autorisées. Le dessin des murs et accessoires est
conservé ; les quatre orientations des modules restent celles déjà approuvées.

## Mesure native

Exécutable debug, fenêtre 1280 × 800, cellules de 20 pixels. Deux séries dans
un ordre inversé, 120 images par mode et 10 images de chauffe exclues des
statistiques. Les modes éteint, fixe et dynamique partagent le même exécutable.
Les horloges de diagnostic sont absentes du rendu release.

Dans l'atelier, 11 sources et 70 quads de sol : le CPU du calcul et de la
préparation du dessin des lumières atteint 0,090 et
0,091 ms de médiane selon la série. Ces mesures
n'isolent pas le temps GPU. Le premier recalcul de l'atelier prend 0,204 ms.
Les variations du rendu complet entre séries ne permettent pas de promettre
un gain de fréquence d'affichage. Le recyclage comporte deux sources sans sol
éclairable dans ce champ initial ; il mesure surtout le coût de consultation.

Une marche de 80 pas aller-retour actualise la visibilité et les souvenirs,
puis recalcule la géométrie à chaque pas. Le CPU des lumières prend
0,194 ms de médiane et 0,297 ms au 95e
percentile. Le rendu complet est de 7,006 ms sans éclairage et
7,285 ms avec éclairage dans ce parcours. Ce test n'active pas les
points de récupération automatiques : il isole le rendu pendant le déplacement.

Les empreintes de simulation et les octets des instantanés moteur et vue sont
identiques avant/après les séries immobiles. Après les 80 déplacements, les
octets moteur et vue sont aussi identiques entre les parcours éteint et dynamique.
Le cache d'éclairage n'entre pas dans les sauvegardes.

Le contrôle GPU compare les images réellement rendues : aucune lumière dans
la mémoire ou l'inconnu ; aucun halo sur un mur opaque non émetteur ; masque
éteint entièrement noir ; pixels différents pour les voyants dynamiques et
strictement identiques aux deux instants testés en mode de mouvement réduit.
Les résultats et images sont dans `performance-verified/`.

## Pauses pendant la marche

Le nouveau diagnostic d'actions, réalisé avant l'éclairage et conservé dans
`baseline-actions/`, confirme un coût de récupération synchrone de 99 à 190 ms
de médiane selon les quatre lieux. Les attentes, sans rendu ni récupération,
prennent environ 1,8 à 5,8 ms. Les points de récupération restent déclenchés
toutes les cinq commandes et peuvent contribuer aux pauses ressenties.

L'encodage, les empreintes et l'écriture participent au coût ; déporter seulement
l'accès au disque ne suffirait pas. La réorganisation sur un travailleur exige
une capture immuable, une écriture atomique, une borne sur les travaux en cours
et une attente lors de la fermeture. Cette intégration d'éclairage conserve
les garanties actuelles de reprise et ne réalise pas cette réorganisation.

## Contrôles

- `cargo build --locked` : réussi.
- `cargo check --locked --all-targets` : réussi.
- `cargo test --locked --bin project-rl terminal_view` : 40 tests réussis,
  dont les six tests d'éclairage (portes, angles, mémoire, invalidation du cache,
  sources inactives et mouvement réduit).
- Compatibilité des octets historiques et restauration sans rejouer le journal :
  deux tests réussis. Total : 42 tests ciblés réussis.
- `cargo check --locked --release --bin project-rl` : réussi. Ce contrôle ne
  constitue pas une mesure runtime release ; les mesures ci-dessus sont debug.
- `cargo fmt --all -- --check` et `git diff --check` sur les sources suivies
  concernées : réussis. Les journaux sont conservés dans ce dossier.
- La suite complète du client n'est pas relancée ; ses échecs antérieurs sont
  documentés dans `../neon-integration-2026-10-03/VALIDATION.md`.
- Le contrôle release signale des imports ou méthodes inutilisés dans
  `arsenal_app.rs`, `ascii_app.rs` et `wall_joins`, en dehors de ce changement.

Captures finales natives relues : `captures-final/violet-20/cold-start.png`,
`captures-final/blue-24/cold-start.png`, `captures-final/city-24/cold-start.png`,
ainsi que les modes éteint et mouvement réduit. Les cinq diagnostics quittent
avec le code 0. Les anciennes captures et séries de préparation restent
conservées séparément. Tous les diagnostics emploient des fichiers et réglages
isolés de ceux du joueur.

Les réglages de combat, la distribution des objectifs et les interactions de
ville restent les propositions de `../suite-demandes-2026-10-03/VALIDATION.md`.
L'introduction reste reportée. `before/` et `integration.patch` permettent de
relire les seuls changements de cette intervention, sans écraser les autres
modifications déjà présentes. Aucun commit ni push.
