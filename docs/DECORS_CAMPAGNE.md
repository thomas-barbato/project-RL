# Décors de campagne — production au fil des cartes

17 octobre 2026 · **DECORS-PLAN-01**.

Référence précise acceptée le 18 octobre :
[STYLE-RL-2026-10-18-r01](STYLE_VISUEL_REFERENCE.md). Les nouveaux décors
gardent la projection du camp et de SCENE-U01-07, les masses simplifiées,
les contours affirmés et l’échelle de 64 px. Les prochaines cartes devront
être plus grandes que cette première rue, avec leurs dimensions à présenter.

Le joueur demande : « ca voudra dire qu'il faudra generer de nouveaux décors
aussi :) (tout au long du developpement des cartes de jeu) ».

La production accompagne chaque carte et secteur. Les décors nécessaires au
premier parcours seront créés pendant le prototype ; les cartes suivantes
enrichiront la bibliothèque selon leurs besoins. PM11 apporte ensuite une
passe globale de cohérence et de finition.

## Lieux préparés à la main : présentation préalable obligatoire

Le joueur précise qu’il veut voir ces lieux avant de les valider et confirme
la proposition générale. Chaque lieu reçoit donc un identifiant et une
révision, une vue d’ensemble de son plan et une vue à l’échelle du jeu.
Cela concerne les refuges, lieux importants, futures arènes et autres morceaux
préparés. Le plan et ses décors sont soumis séparément ; un accord sur le
principe de génération ne remplace pas cet examen concret.

`PLACE-CAMP-01 r01` ouvre cette revue avec un schéma d’implantation et le joueur
à taille native. Les surfaces provisoires ne sont pas des décors proposés pour
le jeu. Voir la [revue du premier lieu](reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/README.md).

L’essai moteur r03 a été écarté visuellement : T non peints, faces centrales
retournées et dessin trop éloigné de l’illustration. La revue r04 présente
un nouveau kit dans la même ambiance, avec des T dédiés et une caméra commune.
Ses captures restent à examiner ; les tests de circulation ne valident pas l’art.

Le joueur choisit ensuite précisément l’illustration r02 à quatre pièces et
confirme que son mobilier doit être adapté à la taille du jeu. **r05** montre
ce camp complet dans un prototype moteur séparé : architecture peinte, mobilier
indépendant redimensionné et cinq acteurs acceptés à taille native. La référence
est choisie ; l’adaptation et les nouveaux meubles restent à examiner. Les
quatre pièces sont accessibles dans la simulation. La correction de la porte
de l’essai r04 possède aussi une capture distincte. Voir la même revue pour
les sources, proportions, limites et preuves de circulation.

Le joueur valide ensuite le camp r05, avec une demande d’ajout : la salle de
cryogénie à gauche, point de départ du joueur, avec plusieurs cuves pour les
autres candidats. **r06** présente cette extension, six cuves fermées et une
ouverte. Le camp accepté conserve exactement ses pixels et ses proportions ;
la liaison intérieure et les parcours sont vérifiés dans le moteur. Le dessin
de l’extension est montré dans la revue, sans ajouter de mécanique de réveil.

À la demande du joueur, **r07** place l’officier déjà accepté NPC-01 r03 à
l’entrée de la cryogénie. Il est déplacé depuis le bureau et laisse le passage
libre. Son rôle est d’expliquer le réveil et la mission ; la réplique exacte
DIAL-CRYO-01 reste proposée dans la revue avant activation. Aucun nouveau
dessin, service, combat ou verrou de passage n’est ajouté.

**r08** applique les corrections suivantes : retrait des trois chaises,
trois lits dans l’infirmerie et lampes latérales aux deux ouvertures du bas,
qui restent visuellement dégagées. Les personnages et la grille restent
inchangés. L’éclairage calculé LUM-CAMP-01 et les animations ANIM-CAMP-01 à 03
sont proposés dans la revue, avant activation ; l’éclairage actuel reste peint.

Le joueur autorise ensuite les quatre propositions. **r09** les active dans
le prototype : compensation des halos peints, éclairage 2D des luminaires
sur les surfaces et les personnages, voyants des cuves, écrans et vapeur.
L’occultation par les murs est précalculée pour ce camp fixe puis appliquée
par le rendu GPU. Les animations suivent une horloge visuelle indépendante
des tours. Le plan et les ressources originales restent conservés ; le passage
à toutes les cartes et l’intégration au client de campagne restent à réaliser.

Après renforcement de la lisibilité des trois animations, le joueur valide
explicitement **PLACE-CAMP-01 r09b** dans son ensemble. Le plan, les décors,
le mobilier, la cryogénie, les emplacements des PNJ et l’ambiance deviennent
la référence acceptée du camp de départ. Voir `acceptance-r09b.json` dans la
revue pour l’accord et les empreintes. La génération 153 le raccorde ensuite
au départ des nouveaux personnages ; les règles de services restent celles
du jeu existant. Le premier secteur extérieur reste à construire.

## Première bibliothèque : accès fortifiés

Les lots ci-dessous sont une liste de besoins, sans dessin encore validé.
Les identifiants servent à suivre les futures planches et leurs révisions.

| Lot de préparation | Besoin pour les cartes | Points à montrer en revue |
| --- | --- | --- |
| DEC-01 — Sols et limites | Sols raccordables, murs, coins, extrémités et seuils | Échelle de 64 px, continuité des raccords et contraste avec les acteurs |
| DEC-02 — Ouvertures | Portes et encadrements correspondant aux accès utilisés | Distinction visuelle des états déjà pris en charge ; ouverture réellement lisible |
| DEC-03 — Mobilier courant | Contenants, établis, armoires et équipements de sécurité selon les modules retenus | Silhouette, implantation et distinction entre mobilier et objets récupérables |
| DEC-04 — Repères de parcours | Habillage des accès et du lieu obligatoire, traces d’occupation et marques du culte | Orientation et ambiance sans multiplication de quêtes ou d’interactions |
| DEC-05 — Refuge | Habillage du camp et des emplacements de services réellement conservés | Retrouver les services, lire les sorties et distinguer l’abri des zones hostiles |

Les objets exacts d’un lot seront choisis avec les modules qui en ont besoin.
Le premier ensemble restera assez réduit pour être comparé dans une carte
complète avant de multiplier les variantes. La liste n’autorise pas un nouvel
effet de terrain, une interaction, un stock marchand ou une récompense.

## Cycle de travail pour chaque nouvelle carte

### Méthode de fabrication — décision du 19 octobre 2026

Le joueur demande : « utilise cette methode pour tous les elements visuels
des décors » et « je veux quelque chose de pro ».

La fabrication suit désormais le contrat de
[PRODUCTION_DECORS.md](PRODUCTION_DECORS.md) pour tous les nouveaux décors et
leurs reprises : éléments indépendants dessinés avant assemblage, transparence
propre, échelle native, couches séparées et raccords vérifiés. Une illustration
d'ambiance ne sert plus de banque de fragments de scène. Les ressources
historiques déjà acceptées restent identifiées ; leur présence ne valide pas
la réutilisation d'une mauvaise découpe dans les prochains lots.

1. Définir le rôle de la carte, ses accès et ses emplacements indispensables.
2. Préparer son plan avec les modules disponibles et relever les manques graphiques.
3. Créer les sources des décors manquants dans le style des ressources acceptées.
4. Présenter les candidats numérotés à taille native et une composition sur carte,
   avec les personnages et monstres à leur échelle réelle.
5. Après accord visuel, conditionner les dessins et leurs métadonnées dans `assets/`.
6. Vérifier à nouveau circulation, visibilité, raccords, ombres et objets au sol.
7. Conserver sources et décisions, puis retirer les sorties de fabrication devenues inutiles.

Le travail peut révéler un besoin de changer le plan ou de refaire un morceau
graphique. On revient alors sur l’élément concerné sans considérer le reste
de la carte ou du lot comme automatiquement refusé.

## Grammaire commune

La référence de case est **64 × 64 px**. Un objet haut ou large peut dépasser
visuellement cette case ; son image ne définit pas à elle seule sa collision.
Les nouveaux décors doivent respecter la simplification, la direction de
lumière et la séparation des matériaux de la
[direction artistique](DIRECTION_ARTISTIQUE_TILES.md).

Un module logique n’est pas forcément une seule grande illustration. Sol,
limites, mobilier et effets doivent garder les séparations nécessaires au
rendu et aux règles du jeu. Les variations cosmétiques conservent les accès
et emprises validés ; une modification de circulation fait l’objet d’une
vérification distincte avant de remplacer une variante existante.

Le sol doit laisser ressortir les silhouettes rouges, vertes et bleues, les
corps et le butin. La rouille, les câbles et les motifs restent lisibles sans
remplir chaque case de petits détails. Un décor de fond ne doit pas masquer
un passage ou ressembler à une zone d’effet active par accident.

## Développement des secteurs suivants

Les familles de matériaux évolueront avec les lieux du manuscrit : installations
de conversion, réseau de surveillance, forges, lieux cultuels et plan démoniaque
glacé. Ces intentions guideront les recherches ; le découpage, les dessins,
noms et effets précis gardent leurs validations séparées.

Les éléments communs compatibles seront réutilisés. Chaque secteur recevra
les nouveaux décors qui portent sa fonction et son ambiance, avec validation
sur ses vraies cartes avant extension de sa bibliothèque.
