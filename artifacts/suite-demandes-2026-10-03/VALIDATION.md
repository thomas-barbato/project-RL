# Revue des demandes — 3 octobre 2026

## Intégré

Les modules du joueur et des NPC possèdent quatre orientations. La pince et
l'émetteur suivent le déplacement ou la direction dominante de l'attaque.
Pour une cible diagonale, l'axe dominant est retenu ; une égalité conserve la
priorité horizontale. Un événement sans déplacement n'impose pas une direction.

L'atlas de présentation passe de 180 × 20 à 180 × 80 pixels : les 36 dessins
sont préparés une fois, sans rotation ni nouvelle allocation à chaque image.
Les quatre dessins conservent les pixels entiers et le filtre au plus proche.
La direction graphique reste indépendante de la direction logique employée
pour les interactions et des structures de sauvegarde.

Deux interrogations de visibilité du moteur construisaient tout un champ de
vision pour vérifier uniquement la position du joueur. Elles utilisent
maintenant la primitive existante `is_tile_visible`, avec les mêmes règles de
rayon, portes, angles et furtivité. Aucun attribut, dégât, délai de commande,
tirage du hasard ou paramètre de génération n'est modifié.

## Mesures actuelles

Diagnostics natifs debug, même graine et même parcours. Les opérations `wait`
ci-dessous incluent la résolution de la commande et la capture des événements,
sans rendu ni récupération automatique. Huit observations par état : il s'agit
de mesures ciblées, pas d'une certification de fluidité sur toutes les cartes.

| Lieu | Avant, médiane | Après, médiane |
| --- | ---: | ---: |
| Recyclage | 4,34 ms | 4,46 ms |
| Ville | 5,35 ms | 1,77 ms |
| Surface | 6,48 ms | 2,34 ms |
| Profondeur 1 | 1,72 ms | 1,91 ms |

Les empreintes du moteur et des octets de son instantané restent identiques
aux quatre points de comparaison, après 8, 287, 296 et 375 commandes. Les
variations en recyclage et en profondeur ne montrent pas de gain mesurable.

Le diagnostic de rendu conserve un coût médian proche de 9,2 ms pour le monde
immobile en debug. Le rendu n'est pas la cible des deux remplacements de
visibilité. Les premiers affichages sont sensiblement plus coûteux. Les séries
de rendu ne constituent pas un test du déplacement avec sauvegarde active.

Le point de récupération reste synchrone toutes les cinq commandes. Le
diagnostic actuel mesure 85–129 ms de médiane selon le lieu avant cette
intervention, avec des variations plus élevées dans la seconde série. Il peut
donc contribuer aux pauses ressenties pendant une marche continue. Sa fréquence,
son alternance de fichiers et ses garanties de reprise sont conservées.

### Suite technique proposée

Préparer les points périodiques sur un travailleur à partir d'un état immuable,
en gardant une écriture atomique et les deux fichiers de secours. Borner à un
travail en cours, empêcher une écriture ancienne après la mort, le changement de
partie ou la suspension, et attendre explicitement sa fin lors d'une fermeture.
Mesurer d'abord le coût de capture de l'état ; déplacer seulement l'écriture
disque ne suffirait pas, car l'encodage et les empreintes coûtent aussi du temps.
Cette réorganisation n'est pas intégrée dans cette intervention.

## Combat : diagnostic et proposition

Les probes emploient les deux rôles réellement générés dans le biome
`human_habitat`, avec les règles actuelles et le joueur de départ. Sol ouvert,
un adversaire isolé, huit attentes, graine de combat 42. Ce scénario n'est ni un
parcours complet ni un audit de tous les comportements du bestiaire.

| Rôle | Attaques | Touches | PV retirés |
| --- | ---: | ---: | ---: |
| Mêlée | 8 | 7 | 14 |
| Tir, portée 4 | 8 | 7 | 0 |

Le déclenchement de ces attaques fonctionne. Le projectile de surface possède
un dégât brut de 1 sans pénétration ; l'armure initiale l'absorbe. Les ennemis
placés dans la zone industrielle de l'enquête possèdent actuellement tous une
portée de 1, y compris la sentinelle immobile. Ce contenu n'assure donc pas une
présence à distance dans cette zone.

**Proposition à valider :** porter le projectile ordinaire de surface de 1 à
3 dégâts bruts, en conservant sa portée, sa précision et sa pénétration nulle.
La résolution réelle donne alors 2 PV par touche avec 1 d'armure, 1 PV avec
2 d'armure, et zéro avec 3 d'armure. L'armure garde donc son utilité. Ces valeurs
sont éprouvées seulement dans une probe ; les tables de rencontres du jeu ne
sont pas modifiées.

Proposer ensuite une sentinelle à tir annoncé dans la zone industrielle,
portée indicative 5 : elle vise une case, l'annonce pendant un tour, puis tire
sur cette case. Le déplacement, la rupture de ligne de vue et l'armure doivent
rester des réponses utiles. Son comportement réutiliserait le rôle existant
`TelegraphedShooter`. Aucun accroissement général des PV ou des effectifs.
Les valeurs finales exigent un essai avec plusieurs profils du joueur et ses
armes de départ, les couverts, les portes et les routes de repli.

## Objectifs : diagnostic et implantation proposée

Une nouvelle partie ordinaire est initialisée avec `INITIAL_SEED`. Changer la
graine modifie déjà le relais : les huit graines de la probe donnent huit
positions différentes. Le choix de l'emplacement prend cependant le premier
candidat valide trié par distance et coordonnées. Dans les huit cas, Milo est
à une seule case du registre.

**Proposition à valider :** une graine différente à chaque nouvelle partie,
conservée dans sa sauvegarde ; un choix déterministe parmi les emplacements
valides à partir de cette graine ; deux sites réservés avant les rencontres.
Le registre reste dans une salle technique, Milo dans une autre pièce reliée,
avec au moins 24 pas de trajet praticable entre les deux comme valeur d'essai.
La distance est mesurée sur les chemins, pas seulement entre coordonnées.

Les deux solutions donnent la même information et la récompense existante une
seule fois. Une indication au départ permet de chercher le témoin ou le registre.
Les deux sites et la prochaine descente doivent rester accessibles, y compris
après installation des décors et des acteurs. Les anciennes parties conservent
leur implantation et leur journal. La modification nécessiterait une nouvelle
génération ; elle n'est pas activée avec les orientations graphiques.

## Ambiance proposée

L'aperçu séparé `ambiance-lumineuse.html` montre des lampes blanches ancrées aux
murs, un balisage ambre et des voyants verts sur les appareils d'une seule case.
Il reprend les modules approuvés et des accessoires en vue de dessus.

Les halos éclairent seulement le sol proche, restent derrière les acteurs et
ne traversent pas la cloison dans cet exemple. Les lampes restent fixes ; seuls
quelques voyants peuvent varier lentement. Le mode de mouvement réduit fige
l'animation. Le noir reste dominant et les couleurs ne suivent pas le thème
de l'interface. Aucun tuyau, grand objet ou obstacle supplémentaire.

Une intégration devra consulter uniquement les cases perçues et supprimer
l'animation dans la mémoire et hors écran. Les halos n'étendent pas le champ de
vision et ne servent pas seuls à indiquer un danger. Cache de géométrie et
mesure du coût par image avant extension à de nombreuses sources.

## Interactions en ville — propositions seulement

Les services déjà présents, commerce, clinique et amélioration d'équipement,
restent les points de départ. Les nouvelles interactions proposées ont chacune
un bénéfice distinct et restent facultatives.

| Interaction | Intérêt pour le joueur | Limites proposées |
| --- | --- | --- |
| Renseignements sur les routes | Obtenir un repère fiable vers un accès profond et connaître les dangers généraux de deux itinéraires afin de choisir où explorer. | Une information locale par destination, acquise une fois ; aucun dévoilement des ennemis cachés ni accès automatique à toute la carte. Premier renseignement gratuit, coût d'informations complémentaires à déterminer après essai. |
| Banc d'essai | Comparer deux équipements contre un mannequin doté d'une protection annoncée et comprendre une attaque absorbée, sans devoir risquer sa partie pour apprendre. | Simulation séparée des événements de la partie, pas de butin ni d'XP, mêmes formules de combat ; résultats limités aux protections montrées. Aucun combat d'essai ne consomme les ressources réelles. |
| Consigne de la couche | Déposer une arme ou une armure utile, libérer de la place et préparer un équipement différent pour une autre branche de la couche. | Stockage limité, exemple d'essai : six objets non empilables. Contenu persistant dans cette partie seulement ; une descente irréversible rend la consigne précédente inaccessible, avec avertissement avant le passage. Pas de transport entre couches ou entre parties. |

Priorité suggérée : renseignements, puis banc d'essai. La consigne vient
ensuite si elle apporte un vrai choix plutôt que des allers-retours obligatoires.
Les prix et capacités indiqués restent des valeurs d'essai. Aucune fabrication
d'objet ni recette de transformation de matière n'est ajoutée.

## Progression vers l'évasion — proposition seulement

Boucle proposée : explorer la couche, trouver ses accès profonds, choisir un
itinéraire en fonction des ressources et des dangers, se préparer en ville si
utile, puis franchir un passage irréversible. Une quête locale peut donner une
information, du matériel ou une autre route ; la progression ne dépend pas
d'un nombre de quêtes terminées.

Une première tranche jouable à approuver serait : départ existant, deux voies
spatialement distinctes vers une même descente, une branche facultative avec
un service ou une ressource utile, puis une couche suivante déjà sans retour.
Le joueur peut avancer sans accepter l'enquête d'Elias. Aucun nouveau peuple,
nom de personnage ou compteur global d'alarme n'est fixé par cette proposition.

La victoire finale correspondrait au franchissement d'un véritable passage
d'évasion, rendu accessible par des actions dans le monde, avec plusieurs
solutions préparées et une résolution enregistrée. Le simple compteur de
profondeur ne déclencherait pas la victoire. Le lieu, les conditions finales et
le récit de cette sortie restent à soumettre ; ils ne sont pas implémentés.

L'introduction et son texte restent reportés conformément à la décision du
joueur. Ces propositions clarifient les prochains choix sans ajouter de prologue.

## Validation

- `cargo build --locked` et `cargo check --locked --all-targets` : réussis.
- `cargo fmt --all -- --check` et `git diff --check` sur les sources suivies
  concernées : réussis.
- Bibliothèque entière : **793 tests réussis**.
- Vue terminale : **34 tests réussis**.
- Orientations du joueur et des NPC : **4 tests réussis**. Déplacements clavier,
  commandes de marche à la souris, orientation des attaques, visibilité et
  absence de mutation de la simulation ou du journal par le cache graphique.
- Reprise et conservation des octets historiques : **2 tests réussis**.
- Probes de revue : **3 tests réussis**, sortie conservée dans
  `gameplay-review.log` et `gameplay-review.json`.
- Total : **836 tests distincts réussis**. La suite complète du client n'a pas
  été relancée ; ses échecs antérieurs figurent dans
  `../neon-integration-2026-10-03/VALIDATION.md`.
- L'avertissement préexistant sur `wall_joins` demeure.

Captures natives relues : `gallery/cold-start.png`,
`npc-up-20/cold-start.png` et `npc-down-24/cold-start.png`. La galerie contient
les trois rôles, trois poses et quatre directions ; les scènes en jeu emploient
respectivement le violet à 20 pixels et le bleu à 24 pixels. Les processus de
diagnostic ont quitté sans erreur et leurs fichiers sont isolés des parties
et réglages de l'utilisateur.

L'aperçu d'éclairage est contrôlé à 1280 et 320 pixels de largeur. Les modes
éteint, fixe et dynamique fonctionnent et la console ne rapporte pas d'erreur.
Il s'agit d'un aperçu séparé dessiné en code, pas d'une capture du jeu avec un
nouvel éclairage intégré.

`before/` préserve les sources avant cette intervention et `integration.patch`
compare ces copies aux seules modifications de cette intervention.
Aucun commit ni push.
