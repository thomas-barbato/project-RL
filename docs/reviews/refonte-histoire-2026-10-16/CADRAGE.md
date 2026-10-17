# Refonte de l'histoire et de sa présentation

16 octobre 2026 — remplacement documentaire, décisions et propositions.

## CAMP-00 — Nouvelle référence

Le joueur demande de remplacer les anciens fichiers de scénario par son manuscrit,
puis de travailler le départ, l'articulation du jeu, les PNJ, les monstres ordinaires,
élites et boss, les décors et une interface moins chargée. Les noms sont temporaires.

La [base narrative](../../HISTOIRE_CAMPAGNE.md) est une copie exacte du fichier
`C:\Users\User\Desktop\Histoire_Roguelike_Hack_n_Slash_Cyberpunk.md`, version 4.
SHA-256 : `7f92509ac70a6a5a56dc0b758920b979c15cc47fdb825d4c4e56b11a4ba449ca`.
Le fichier d'origine sur le Bureau est conservé.

Cet import autorise le remplacement documentaire. Les mentions de validation du
manuscrit décrivent son statut éditorial ; elles n'autorisent pas séparément
l'intégration de tous ses dialogues, règles, combats ou dessins. Les détails
signalés comme propositions le restent. Aucun nom temporaire n'est figé.

Documents dédiés à l'ancien récit supprimés du projet :

- `Trame narrative et quête principale — Projet Roguelike IA.md` ;
- `docs/EVALUATION_NARRATIVE.md` ;
- `docs/TEXTES_NARRATIFS_PROPOSES.md` ;
- `docs/SUIVI_INTEGRATION_NARRATIVE.md` ;
- `docs/DESCENTE_VERS_LE_NUMERIQUE.md` ;
- `docs/PROPOSITION_PEUPLADES_v0.1.md`, proposition de peuples de la simulation.

Le registre `REMPLACEMENT.json` conserve leurs empreintes et les contrôles.
L'historique Git reste disponible ; aucune copie des anciens scénarios n'est ajoutée.
`MONDE_EXPLORATION_ET_PROGRESSION.md` est réécrit et le contexte de
`REFONTE_AGENT_CRYOGENISE.md` actualisé sans supprimer les accords sur les systèmes.
Les documents mêlant technique et ancien monde sont signalés comme historiques,
et leurs liens narratifs corrigés. Les revues et preuves de fabrication sont
conservées. Les anciens exemples ne définissent plus la nouvelle campagne.

## CAMP-01 — Avis sur l'histoire

La motivation est claire : une personne sacrifiable accepte une mission pour
obtenir sa liberté. Le passé criminel ouvert laisse une place au joueur sans
imposer une innocence cachée ou un statut d'élu.

La cryogénie initiale et les victimes conservées dans l'enfer final créent un écho
fort : commencer captif, traverser des formes d'asservissement et finir libre.
C'est une piste de mise en scène, sans nouvelle révélation proposée.

Les secteurs ont des identités concrètes : commandement, conversion médicale,
surveillance, production, fanatisme puis domination glacée. Ces rôles peuvent
guider formes, matières et comportements. Le froid gagne à rester une rupture
finale, plutôt qu'à uniformiser toute la campagne. Les refuges apportent un
contraste humain et la liberté accordée permet une victoire qui compte.

Le risque principal est le volume : six refuges, environ dix-huit interlocuteurs
utiles, six chefs et vingt-quatre quêtes proposées, avec les décors et ennemis
à produire. Le second est la répétition du schéma mécanisme, ouverture, boss,
ainsi que des sauvetages et récupérations de réserves. La géographie et les
conséquences doivent varier sans multiplier les systèmes.

## CAMP-02 — Moins de quêtes accepté

Le joueur accepte explicitement qu'il y ait moins de quêtes. Les quatre quêtes
par acte sont une base ajustable, pas un quota ni un maximum fixé. Le nombre final
reste ouvert. Une réduction devra garder le fil principal, les informations
indispensables et l'utilité du refuge. Aucun objectif détaillé n'est supprimé
ou fusionné automatiquement par cette décision.

## TRAVEL-01 — Voyage rapide entre camps débloqués

Le principe proposé par le joueur est retenu : voyage rapide entre les camps
déjà débloqués. Un camp non débloqué n'est pas une destination.

L'habillage, le déblocage, l'accès à la commande, les conditions, coûts éventuels
et le fonctionnement à travers le passage démoniaque restent à proposer. Aucun
prix, consommable, délai, recharge ou restriction de combat n'est ajouté.
Le voyage entre camps est distinct des portails bonus et ne rend pas un rapport
obligatoire pour conclure un acte. La fermeture du passage principal et l'éventuelle
exploration après la victoire devront rester cohérentes avec ce réseau.
Le code de déplacement n'est pas modifié dans cette passe.

## PLAN-01 — Ordre de travail du 16 octobre

Le joueur propose de commencer par l'interface, puis les monstres ordinaires,
les élites, les boss, les PNJ de discussion et de services, enfin les décors et
la réalisation du scénario. Cet ordre devient la base de travail courante :

1. Interface : état des lieux, priorités d'allègement, maquette à examiner,
   puis intégration de la proposition retenue.
2. Monstres ordinaires : premières familles du secteur initial et référence
   de style commune, avant d'étendre tout le catalogue.
3. Élites : différenciation lisible des familles concernées.
4. Boss : identité propre et cohérence entre silhouette et combat à concevoir.
5. PNJ de discussion et de services : rôles reconnaissables et cohérence des camps.
6. Décors, cartes et mise en scène : cryoprison, refuges, secteurs, quêtes et
   articulation du voyage rapide.

L'histoire guide dès maintenant le choix des acteurs et leur fonction. La première
maquette d'interface peut utiliser les ressources existantes. La cohérence entre
HUD, acteurs et décors sera contrôlée dans une scène commune avant d'étendre la
production. Ce plan ne valide aucun nouveau dessin, texte ou effet mécanique.

## PLAN-02 — Ordre de réalisation révisé le 17 octobre

Après l'intégration graphique en cours, le joueur propose l'étude de la génération
de Diablo II, la construction de notre monde, son peuplement, la progression,
les compétences des monstres, les boss, les finitions puis les sons. Il accepte
l'ajustement suivant : vérifier tôt une petite tranche jouable réunissant carte,
rencontres, quelques compétences et une première progression d'XP, avant de
développer tout le monde. Les corrections critiques et la première difficulté
sont suivies pendant la réalisation ; l'équilibrage complet vient après les boss.

Le [plan de travail courant](../../PLAN_DE_TRAVAIL_CAMPAGNE.md), PLAN-CAMPAGNE-02,
précise les dépendances, livrables et critères de sortie. Il remplace l'ordre
opérationnel de PLAN-01 pour les travaux à venir, sans modifier les validations
artistiques ou mécaniques déjà obtenues. Les règles nouvelles gardent leur revue.

## START-01 — Première tranche de campagne proposée

Commencer par le réveil, le contrat, la sortie de cryoprison, le premier refuge et
une première sortie de combat, avec peu de texte imposé. Les textes exacts, le plan,
les interactions et les services seront revus avant intégration. Étendre ensuite
le secteur jusqu'à son chef permettra d'éprouver la boucle avant de décliner
les autres actes. Ce n'est pas une validation des quêtes détaillées.

## ART-01 — Direction visuelle à proposer

Préparer une scène commune réunissant décor, joueur, PNJ, ennemis, objet au sol et
interface pour juger l'ensemble avant de produire le catalogue.

Piste : cyberpunk industriel usé, présences humaines reconnaissables et démons aux
matières variées. Donner aux secteurs des formes et matériaux propres. Le trait,
le détail et la stylisation restent à montrer et à choisir.

La [direction artistique actuelle](../../DIRECTION_ARTISTIQUE_TILES.md) garde sa
procédure et ses accords. Depuis le 17 octobre, seul le rendu 64 px est retenu ;
la version 32 px est abandonnée. Montrer les candidats numérotés à taille
native, avec scènes en situation,
couches et points d'attache cohérents.

Proposition : distinguer ordinaire, élite et boss par silhouette, posture et
attributs visibles, au-delà d'une recoloration. Cela ne fixe ni taille, ni
statistiques, ni compétences. Les rôles des PNJ doivent se lire sans accumuler
des badges. Aucun nouveau candidat visuel n'est créé ou intégré ici.

## HUD-01 — Allègement à étudier

Le 16 octobre, le joueur valide le lancement du chantier et ajoute explicitement
la fenêtre des pets, drones et réanimés. Les accords UX-PETS-01 et UX-PETS-02
restent la base de cette fenêtre. L'[audit et la maquette courante](INTERFACE_ET_FAMILIERS.md)
conservent aussi les corrections explicites du joueur.

Le joueur demande de privilégier les résolutions PC Full HD, QHD et 4K :
1920 × 1080, 2560 × 1440 et 3840 × 2160, avec une échelle d'interface adaptée.
Le 1280 × 720 est un contrôle secondaire. Aucun chantier mobile ou tablette
n'est demandé. La lecture de « 2K » comme QHD est explicitée dans la conversation.

Le constat de surcharge vient du joueur. Cette passe n'est pas un audit visuel du
HUD actuel ; les modifications précises devront être jugées sur des captures et
maquettes comparables.

**Correction du joueur — HUD-03 :** alléger visuellement l'interface, en conservant
les informations et commandes présentes. Les propositions HUD-02A et HUD-02B,
qui déplaçaient notamment les défenses et réduisaient les accès visibles, ne
sont pas retenues. Ne pas en déduire une autorisation de masquer d'autres données.

Travailler les cadres, séparations, accents, espacements et la hiérarchie des
titres. Le joueur demande aussi de clarifier le bas de l'écran : distinguer
actions de jeu, compétences rapides, accès aux fenêtres et messages, en gardant
les fonctions et raccourcis existants.

**UX-PETS-04 — choix du joueur :** la fenêtre commune des drones et réanimés
s'ouvre par un bouton avec une icône. Elle peut être fermée au départ puis
déplacée, repliée et fermée une fois ouverte. Son ouverture ne remplace pas
les autres modules du HUD. Le rendu précis reste une proposition à examiner.

**HUD-04 — précisions du joueur :** centrer les compétences sous le plateau,
retirer le compteur global de tours, placer les coordonnées près du nom du lieu,
montrer les buffs/debuffs par leurs icônes avec détails au survol et les faire
clignoter à un tour restant. La disposition verticale est proposée par le joueur.
L'aperçu révisé montre aussi une matière métallique discrète en réponse à sa
demande de textures. L'intensité de cette matière et le rythme du clignotement
restent des propositions visuelles ; aucune règle de durée n'est changée.

**HUD-04B — choix et sauvegarde :** le joueur aime l'agencement et demande de le
garder exactement, avec la barre compétences/changement d'arme seule, sans le
fond pleine largeur du skill-dock. Cette référence métallique est sauvegardée
séparément avant l'essai HUD-05-TERMINAL demandé par le joueur. L'essai reprend
la même disposition dans une ambiance de vieux terminal PC ; le choix entre
les deux habillages reste ouvert.

**HUD-06 — décision du joueur :** garder les deux habillages, Métal sombre
par défaut, avec sélection dans les options. Employer les vraies icônes des
compétences et des statuts. Cette décision remplace l'hésitation précédente.
Le libellé proposé est « Habillage de l’interface », distinct de la couleur
d'accent existante. Le raccordement natif devra préserver les réglages et
charger les anciennes préférences avec le défaut Métal sombre.

**UX-PETS-05 — demande complémentaire :** montrer le bouton Familiers à côté
de Techniques et agrandir la fenêtre pour la lisibilité. Chaque familier doit
montrer ses buffs, passifs ou non, debuffs et durées propres. Afficher uniquement
ce qui le concerne réellement ; ne pas inventer de durée pour un passif ou
un effet permanent. La revue courante propose 400 pixels logiques de large,
une rangée d'icônes d'effets par acteur et le maintien du repli/déplacement/défilement.

Une maquette montrera une scène calme, un combat avec plusieurs effets et une
interaction au refuge, aux tailles de fenêtre retenues. Aucun raccourci ni
élément d'information autre que le compteur global de tours expressément retiré
par le joueur n'est supprimé par ce cadrage.

## REV-01 — Suite et limites

Restent à préciser : volume des actes, lien entre pouvoirs acceptés et condition
de détenu, mort et renouvellement, modalités du voyage rapide, sortie du royaume
et éventuelle exploration après la fin.

Les identifiants servent à reprendre la revue ; les propositions ne deviennent
pas approuvées par leur présence ici. Texte, règles chiffrées et rendu visuel
seront validés séparément. Les dessins seront numérotés et montrés à taille native
avant intégration conformément à `AGENTS.md`.

Cette passe change uniquement la documentation. Départ, camps, quêtes, voyage
rapide et nouveaux graphismes ne sont pas raccordés au jeu. Les modifications
préexistantes sont conservées. Aucun commit, push ou déploiement n'est effectué.
