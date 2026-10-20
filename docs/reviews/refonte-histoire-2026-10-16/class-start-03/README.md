# CLASS-START-03 — apparence de classe et premier raccourci

20 octobre 2026. Le joueur valide la synthèse complète, puis demande :
« ok alors on fait ca :) ».

## Règles acceptées et intégrées

- Le personnage commence sans objet d'armure : aucun torse équipé, aucune
  tenue de classe rangée dans son inventaire. La tenue de départ est son
  apparence de base, sans instance d'objet ni emplacement cosmétique occupé.
- Les cinq modèles de classe déjà acceptés restent ceux du plateau, de la
  création et de l'inspection. Une armure réellement équipée remplace cette
  apparence ; une fois retirée, le modèle de classe réapparaît. Les autres
  pièces et les choix explicites d'apparence conservent leurs règles.
- L'arme de départ doit convenir aux compétences et n'avoir aucun bonus
  d'objet. Les armes ordinaires existantes satisfont cette demande : hachette
  pour le Bastion, pistolet Cogneur P01 pour le Technomancien, fusil de
  patrouille pour le Traqueur, couteau de camp pour l'Arcaniste et le Métamorphe.
  Les sorts, invocations et transformations ne nécessitent pas une autre arme.
  Dégâts de base, attributs, bonus propres aux classes et ressources restent
  identiques ; aucun nouvel effet d'arme n'est ajouté.
- L'active initiale occupe le premier raccourci : B01, D59, T01, FE01 ou M01
  selon la classe. Les cinq autres raccourcis sont vides. Le joueur peut la
  déplacer ou la retirer sans perdre son rang appris.
- Ce remplissage est effectué une seule fois à la création. Vider toute la
  barre est enregistré comme six emplacements vides ; la reprise ne doit
  jamais réinstaller automatiquement le raccourci retiré.

## Compatibilité et ressources

La génération 165 porte ce départ. Les définitions historiques des cinq tenues
et leurs dessins sont conservés, mais elles ne sont plus accordées par les
classes actuelles. L'adaptateur des générations précédentes réinsère leur
ancien objet en première position pour conserver identités d'instances et
ordre du tirage des couleurs. Avant 164, leurs anciennes protections et malus
d'esquive sont également reconstruits. Les sauvegardes existantes ne sont
pas réécrites et la barre qu'elles contiennent est respectée.

Le rendu utilise la sélection d'apparence de classe déjà existante lorsque le
torse est vide. Il ne dépend d'aucune nouvelle image ni d'un fichier temporaire.
Les références FIN-01 et FIN-14 à FIN-17 restent inchangées. Le rééquilibrage
des dégâts, des PV et de l'XP reste un travail séparé.

## Vérifications

Les tests de ce lot vérifient les cinq créations normales, l'absence d'objet
d'armure, l'arme sans bonus, l'utilisation de l'active avec les ressources
initiales, les modèles du plateau et de l'inspection, l'équipement puis le
retrait d'une armure, et le retrait du raccourci par glissement hors de la
barre suivi d'une sauvegarde/reprise. Le bilan d'exécution est conservé dans
`verification.json`.
