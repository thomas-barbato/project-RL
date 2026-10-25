# État actuel : essais rejetés et désactivés

Le joueur a rejeté le grand écart commun aux poses déformées, puis le montage
Traqueuse à jambes redessinées greffées sur un buste fixe (« non ca ne va pas du
tout ca »). Aucune de ces variantes n’est validée. Le rendu jouable les désactive
explicitement avec `REVIEW_ACCEPTED = false` dans `src/game_art_walk.rs` ; le rendu
antérieur des personnages et équipements est restauré. Les préférences de durée
restent disponibles. Les sources et preuves sont conservées pour éviter de
confondre un essai rejeté avec un candidat accepté.

La référence retenue est toujours le canvas `data-scene="1"`, désigné également
par le joueur comme les animations « 040 et 041 ». Un cycle entier cohérent doit
être revu avant une nouvelle activation. Les validations techniques ci-dessous
concernent des essais retirés ; elles ne constituent pas une validation visuelle.

---

# WALK02-FIX02 — correction de la marche sélectionnée

Le joueur a désigné explicitement le canvas `data-scene="1"`, « Animation 2,
marche à gauche et à droite du Bastion à taille native » du fichier joint
`player-walk.html`. L’option Sites est abandonnée. L’accord d’intégration du
23 octobre reste applicable ; les 400 ms attendent son essai en jeu.

## Référence et défaut corrigé

L’atlas encodé dans le fichier joint est identique octet pour octet à
`artifacts/player-walk-review-01/bastion-walk-native64.png`. Une copie durable
est conservée ici sous `reference-canvas-02.png`. La référence sélectionne les
rangées SE et SW, six poses, trois poses par déplacement et alternance du pied
d’appui entre deux déplacements. La demande ultérieure conserve la face pour
le haut et le bas. Il ne s’agit pas de sélectionner l’animation frontale 01.

WALK04-INT01 ne reproduisait pas cette marche : les jambes des configurations
peintes bougeaient trop peu ; les autres armes (dont la hachette de départ)
retombaient sur un corps frontal entier presque immobile. Cette intégration
a été rejetée par le joueur. Ses anciennes captures ne valident pas la marche.

## Rendu actif corrigé

- Une action reste une case de 64 px. Aucun coût, collision, statistique,
  identifiant ou format de sauvegarde ne change.
- Trois poses articulées par case, six poses sur deux pas, appuis alternés,
  cuisses, genoux et pieds mobiles. Le passage reçoit un léger mouvement vertical.
- Deux corps réellement dessinés en trois quarts à gauche et à droite ; le haut
  et le bas utilisent le corps de face. Les poses adaptées à chaque tenue ne
  recopient pas l’armure du Bastion sur les autres classes.
- Les 25 tenues actuelles et les 19 tenues historiques ont des corps et membres
  attachés séparément. Leurs armes réelles, couleurs d’armes, casques et paires
  passent par le compositeur existant : les six équipements du prototype ne
  remplacent plus les autres modèles. La hachette est le modèle réel 67.
- Le modèle d’arme conserve l’angle et l’échelle de son rendu de production.
  Seuls des déplacements, rotations et réflexions rigides de l’objet entier
  sont autorisés. Aucun masque de jambe ne touche une hampe ou une lame.
- Chaque bras et sa main viennent du dessin de la tenue dans cette orientation.
  L’épaule sert d’attache ; les doigts sont composés après l’arme.
- Les six acteurs de service à équipement modulaire ont également trois vues
  et des armes attachées. Les autres personnages, formes, drones et monstres
  conservent leur identité et variante dans les atlas de créatures recalculés.
- Les 400 ms restent le défaut, avec choix 120 / 200 / 400 ms. Les commandes
  rapides ne constituent pas une file visuelle ; les animations réduites
  désactivent la marche. Portraits et inventaire restent hors du contexte animé.

## Ressources et reproduction

Les ressources d’exécution sont uniquement sous `assets/animations/walk-04/` :
659 atlas et 182 rigs directionnels. Chargement à la demande et caches bornés.
La distribution référence aussi `rigs/manifest.json`. Les fichiers HTML, Sites,
`artifacts/` et les dessins sources ne sont pas nécessaires au jeu.

Après le conditionnement des dessins, `tools/art/integrate_player_walk_04.py`
installe aussi les rigs et les cycles corrigés. Les scripts autonomes sont
`build_walk_rigs_02.py` et `rebuild_walk_cycle_02.py`. La pose de repos sert de
source immuable à chaque reconstruction : pas de déformation cumulative.

## Vérification

Les résultats à jour figurent dans `runtime-verification.json`. Le diagnostic
`--ui-cold-walk-integration artifacts/player-walk-fix-02/runtime` utilise le vrai
contrôleur et le vrai rendu : deux commandes opposées d’une case, captures à
30 Hz, hachette de départ, galerie des 25 tenues et six équipements sur sols
clair et sombre, scène de monstres et menu des durées. Les données de campagne
et préférences du joueur ne sont pas écrites.

La compilation et les contrôles de pixels ne remplacent pas l’avis du joueur
sur le résultat artistique ou sur les 400 ms.
