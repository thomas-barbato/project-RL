# CLASS-ARMORS-ALL02 — coiffes et proportions

21 octobre 2026 — **reprise des numéros 06–25, à comparer en 64 × 64**.
Les cinq Bastion 01–05 restent les compositions acceptées exactes.

## Demande et décision

Après la première planche commune, le joueur n’apprécie pas le Technomancien,
la Traqueuse ni le casque de l’Arcaniste. Il relève l’absence de vrais casques
sur la Traqueuse, propose l’inspiration **tireuse d’élite ou ninja**, juge la tête
de l’Arcaniste trop grande et constate des nuques trop longues en général.

Pour le Technomancien, la clarification est explicite : **« Prêtre-machine
sombre, visage mécanique et manteau sobre »**. Le témoin turquoise auparavant
accepté reste archivé avec son accord exact ; il n’est plus la proposition
courante à décliner. Cette nouvelle direction et ses cinq modèles attendent revue.

Les numéros 01–25 sont conservés. Cette reprise ne vaut aucune approbation
automatique des nouveaux dessins. Les variantes de couleur restent reportées,
et l’intégration attend toujours les armures, casques équipables et prises d’armes.

## Modifications par classe

- **01–05 / Bastion** : audit du raccord accepté, déjà court. Pixels, corps et
  têtes conservés exactement ; aucun redessin ou changement de cadrage.
- **06–10 / Technomancien** : cinq nouvelles tenues sobres de prêtre-machine,
  tissus gris sombres, mécanique cuivrée et protections cendrées ; cinq coiffes
  compactes à visage mécanique fermé, de plus en plus protectrices.
- **11–15 / Traqueuse** : cinq nouvelles tenues de reconnaissance, avec formes
  ajustées, drapés courts et protections mobiles ; cinq véritables coiffes,
  du passe-montagne masqué au casque de reconnaissance fermé.
- **16–20 / Arcaniste** : corps d’armure sources conservés, mais cadrés à partir
  des épaules pour corriger la nuque. Cinq nouvelles coiffes bordeaux, ivoire
  et cuivre, proches du crâne, assorties aux matières des tenues.
- **21–25 / Métamorphe** : dessins sources et tête conservés ; seul le cadrage
  du corps et son raccord sous la tête changent pour rapprocher les épaules.

Les personnages restent humains au repos, avec les identités de classe et les
augmentations de référence. Les coiffes fermées les masquent sans modifier leur
anatomie ou créer un nouveau bonus. La Traqueuse et la Métamorphe restent des femmes.

## Cause du raccord et correction

La première fabrication calait le haut opaque de chaque module sans tête,
y compris son cou, sous une tête qui contenait déjà son propre cou. Le domaine
du corps était ensuite coupé horizontalement sur toute la largeur de la case.
Cette méthode plaçait les épaules trop bas et pouvait supprimer des pixels
de col ou d’épaule pourtant situés à côté du visage.

La reprise cadre les épaules plus haut depuis les sources haute définition,
en gardant une réduction isotrope et les pieds à Y=61. Le corps conserve ses
pixels de haut d’épaule derrière la silhouette de tête. Cette dernière assure
l’occlusion ; une ligne horizontale n’efface plus toutes les épaules. La case
reste de 64 × 64 ; aucun ancien sprite 64 px n’est agrandi.

Les protections ouvertes sont composées sur la tête de base conservée. Une
coiffe entièrement fermée remplace visuellement cette tête complète, ce qui
évite les cheveux ou augmentations dépassant d’un casque censé les couvrir.
Le corps, la tête et la protection de tête restent des calques explicites.
Leur réassemblage doit reproduire exactement les pixels RGBA de la composition.

Les dimensions de cadrage sont des paramètres de revue, pas de nouvelles
collisions, statistiques, dimensions de case ou règles de jeu.

## Sources et présentation

L’outil intégré **image_gen** produit dix nouveaux corps et quinze nouvelles
protections de tête. `prompts.json` conserve les textes exacts, les références,
la réponse du joueur et les cadrages. Les sources vont dans
`assets/source-art/class-readability-01/class-armors-all-02/`.
Les sources de l’Arcaniste et de la Métamorphe sont réutilisées sans modification.
Les accords et la première planche restent conservés, avec vérification de
leurs empreintes avant et après la préparation.

`tools/art/prepare_all_class_armors_02.py` prépare les calques et la galerie
courante. La planche 01–25, les trois fonds, les zooms de chaque classe et les
comparaisons avant/reprise permettent la revue humaine.
Une planche supplémentaire compare les quinze protections de tête à un zoom
entier ×6, en conservant le numéro de chaque tenue.
L’ancre courante
`#all-armors-native64` mène à cette nouvelle planche, présentée sans réduction CSS.
Le montage sur sol de rue ne constitue pas une validation dans l’éclairage moteur.

`candidates.json` contient les sources, identités, calques et empreintes ;
`ui-checks.json` et `validation.json` conservent les contrôles réalisés.
Aucun fichier Rust, catalogue d’objets, sauvegarde ou ressource active n’est modifié.
