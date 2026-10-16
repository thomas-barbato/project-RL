# Épée longue industrielle — aperçu avant validation

L’utilisateur trouve la hache portée tordue et souhaite voir une véritable épée,
avec des proportions de lame longue plutôt que de couteau ou de poignard.
Sa dernière précision distingue une épée à une main et une épée géante portée
à deux mains. La garde de l’épée à une main doit être nettement plus petite.
Les deux dessins d’épée ont été acceptés par l’utilisateur. Sa dernière remarque
demande que changer d’armure conserve la même taille de personnage.

## Dessin proposé

- Lame droite et longue, pointe centrée, acier sombre abîmé et tranchants clairs.
- Garde réduite, poignée noire, plaques rouges usées et petit indicateur ambré.
- Poignée, centre de la garde et lame suivent le même axe.
- Essai sur BRÈCHE, Cerbère, VIGIE et Fantôme, sur les deux côtés de prise.
- Épée géante : lame plus massive et longue, deux mains fermées sur le même
  manche allongé, pose différente des armes à feu.

Le premier essai porté avait une lame trop grande qui réduisait les personnages
à 32 pixels. Il est conservé dans `personnages-epee-longue-v1-source.png`.
L’essai suivant était trop court et sa garde trop large ; il est conservé dans
`personnages-epee-longue-v2-source.png`. La proposition à une main suivante
allonge sa lame et réduit sa garde selon la dernière remarque de l’utilisateur.

## Aperçus et sources

- `epee-longue.png` : dessin isolé et réduction à 32 pixels.
- `comparatif-epee-longue-portee.png` : comparaison actuelle/proposition et taille réelle.
- `comparatif-une-deux-mains.png` : épée à une main et épée géante à deux mains,
  personnages agrandis et réduction avec corps de 30 pixels.
- `tailles-communes.png` : quatre poses côte à côte, même hauteur et même
  ligne des pieds, quelle que soit l’armure ou la longueur de l’arme.
- `body-scale.json` : mesures du corps et des pieds après normalisation.
- `matraque-sans-garde.png` : garde retirée de la proposition de matraque.
- `epee-longue-source.png` et `personnages-epee-longue-source.png` : sources imagegen.
- `personnages-epee-geante-source.png` : essai de la véritable prise à deux mains.
- `PROMPTS.md` : prompts exacts, mode intégré imagegen.
- `build_previews.py` : recadrage, nettoyage alpha, redimensionnement et présentation.

La source de matraque corrigée se trouve dans
`../melee-brutale-propositions-2026-10-05/matraque-sans-garde-source.png`.
Le tableau des neuf armes utilise maintenant ce dessin sans garde.

## État

Aperçus uniquement ; les deux épées sont validées et le cadrage des corps vient
d’être corrigé à la suite de la dernière remarque. La hache n’est pas
intégrée ; les packers ont été remis à leur état précédent. Aucune texture active,
aucun nom, aucune statistique ni règle du jeu n’a été remplacé.

Vérification : les 215 fichiers actifs de la référence sont identiques
(`preservation.json`). Les deux packers actifs passent `--check` : atlas des tenues
et atlas cyberpunk reproductibles. `git diff --check` passe également.
Il n’y a pas de nouveau rendu exécuté dans le client : les réductions sont des
aperçus. Le corps mesure 120 pixels dans un cadre source de 256 pixels, puis
30 pixels dans la réduction de 64 pixels. Ce cadre plus grand réserve de la place
à la lame au-dessus de la tête sans rétrécir le personnage. Les pieds ont tous
la même ligne de base. Sur les quatre poses comparées, les mesures du corps
donnent exactement 120 pixels et les pieds arrivent tous à y=251.

L’ancien redimensionnement utilisait la boîte englobante du personnage avec
son arme. Le nouveau cadrage repère les bottes et le haut du corps dans une
bande centrale, puis applique une échelle commune au dessin complet. L’arme
et les mains restent intégrées au dessin ; aucune pièce n’est ajoutée par-dessus.

Lors de l’intégration, une pose de mêlée à deux mains devra être ajoutée à côté
de la pose actuelle à une main : aujourd’hui, `weapon_hold()` du packer classe
toutes les armes de mêlée dans `Side`. La prise de mêlée à deux mains devra
utiliser une information explicite par modèle, sans être confondue avec la pose
de fusil ni déduite de la portée d’attaque. Cette attribution n’a pas été modifiée
pendant la préparation des dessins.
