# CLASS-MOTION01-r01 — revue des classes hors Traqueuse

**Correction du 2 novembre 2026 :** les 20 configurations du Technomancien
ont été incluses à tort dans cette revue historique. La classe est retirée
de la création et conservée pour les anciennes sauvegardes. La priorité de
reprise indiquée ci-dessous ne s’applique donc pas au roster actuel. La revue
courante utilise `CharacterClassCatalog::iter()` et exclut cette classe.

Demande du 2 novembre 2026 : revérifier toutes les autres classes après l’essai
de la Traqueuse. Cette revue capture le rendu normal de Bastion, Technomancien,
Arcaniste et Métamorphe, sur leurs cinq paliers d’armure. Aucun nouveau dessin
n’est intégré et le cycle de la Traqueuse n’est pas activé dans ce contrôle.

## Périmètre et preuves

80 configurations : quatre classes × cinq armures × quatre équipements
témoins (sans arme, fusil d’assaut, mitrailleuse lourde, hache d’armes).
Chaque configuration inclut un repos frontal, six phases dans chacune des
quatre directions et le retour frontal. Les vues non prises en charge passent
par le même repli frontal que `terminal_view`, sans inventer de dos.

Les 2 000 rendus utilisent `game_art::draw_world_animated` et son ordre de
sélection normal, les ressources préparées de VERT18, la scène du camp et
l’éclairage Discret. Les quatre classes et cinq colonnes sont synchronisées.
Le navigateur affiche les pixels du moteur en taille native avec un zoom entier
optionnel. Les légendes n’altèrent pas les zones capturées.

Le diagnostic sélectionne les phases du rendu ; il ne simule pas une partie
ni les commandes de déplacement. Les données de sauvegarde et les préférences
ne sont pas modifiées. Les autres modèles d’armes, leurs couleurs, les drones
et les transformations de la Métamorphe ne sont pas couverts par ce lot.

## Constats

1. **Technomancien : cycle face/dos absent dans les 20 configurations.**
   Aucun de ses cinq paliers n’est présent dans la banque VERT18 courante.
   Les déplacements verticaux montrent donc un frontal immobile. La silhouette
   mécanique de face et les protections latérales ne correspondent pas bien.

2. **Neuf configurations sans arme n’ont pas de marche verticale :** paliers
   léger, intermédiaire et lourd du Bastion, de l’Arcaniste et de la Métamorphe.
   Les profils correspondants ne proposent pas la famille `one` demandée sans
   arme. Le moteur retombe sur l’ancien frontal. C’est la même catégorie de
   repli que celle rencontrée sur les tenues intermédiaires de la Traqueuse.

3. **Les 160 cycles latéraux ont chacun six images distinctes**, mais ce
   contrôle technique ne valide pas leur identité. Les différences de coiffe,
   de couverture et de coupe restent visibles entre certains frontaux et côtés,
   en particulier l’Arcaniste très lourd et la Métamorphe. Le Bastion conserve
   la démarche déjà jugée acceptable ; les raccords de tenue restent à comparer.

Les 29 configurations non couvertes donnent 58 cycles verticaux immobiles.
Les 51 autres configurations utilisent six phases face/dos. Aucun accord
esthétique global ne découle de ces contrôles.

## Résultat et suite proposée

L’aperçu courant reçoit une section « Revue des classes » avec les cinq armures,
quatre équipements, choix gauche/droite ou haut/bas, pause, phase manuelle,
ralenti et retour au repos. Le bilan détaillé est `CLASS-MOTION01-r01.json`.
Les captures reproductibles sont sous
`artifacts/animation-polish-01/class-motion-01/current/`.

La priorité de reprise est le Technomancien, puis le repli sans arme des neuf
tenues et les raccords d’identité constatés. Il faut présenter chaque reprise
en taille native avant de remplacer le rendu normal. Ce document constitue
un constat de revue, pas une approbation de nouveaux dessins.
