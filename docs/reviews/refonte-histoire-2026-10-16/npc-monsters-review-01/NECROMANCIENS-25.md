# NECROS-25 — nécromanciens scientifiques et démoniaques

Propositions du 17 octobre 2026. Le joueur demande de garder le lot 116–131
comme candidats, puis de créer des nécromanciens scientifiques chez les
cultistes et démoniaques chez les démons. Les huit nouvelles apparences
132–139 rejoignent le catalogue pour sélection ultérieure.

## Corrections demandées pendant la création

Le joueur refuse le premier scientifique 132 r01 : sa seringue n'est pas
droite et il ne ressemble pas assez à un homme de science. La r02 propose
une vraie blouse clinique à revers, manches et poches, des proportions humaines,
un masque avec lunettes protectrices et une seringue verticale à axe commun.
Les 133 et 134 sont repris dans cette direction clinique en r02 ; leurs
r01 restent des essais antérieurs, sans refus individuel déduit du joueur.
Le 135 en blouse de protection gris bleuté reste en r01.

Pour le 139, le joueur demande de changer le visage, incohérent avec le corps,
et l'orientation du livre. La r02 remplace la plaque ivoire par une tête
organique gris bleuté. Le livre s'ouvre vers le regard du démon, avec une
main adaptée à la perspective pour soutenir la couverture et la reliure.
Le manteau, les bras et les trois appuis tentaculaires gardent leur direction.

Les sources précédentes sont conservées et exclues du catalogue courant.
Aucune de ces corrections ne vaut validation du nouveau rendu.

## Dessins courants et dimensions natives

| Numéro | Code | Nom provisoire | Famille | Corps natif |
| --- | --- | --- | --- | --- |
| 132 | NEC-C01-r02 | Thanatopracteur du culte | Nécromanciens scientifiques | 42 × 64 px |
| 133 | NEC-C02-r02 | Réanimateur galvanique | Nécromanciens scientifiques | 39 × 67 px |
| 134 | NEC-C03-r02 | Conservateur de moelle | Nécromanciens scientifiques | 30 × 62 px |
| 135 | NEC-C04-r01 | Cryothanaturge | Nécromanciens scientifiques | 33 × 66 px |
| 136 | NEC-D01-r01 | Fossoyeur des failles | Nécromanciens démoniaques | 60 × 76 px |
| 137 | NEC-D02-r02 | Fossoyeuse chitineuse | Nécromanciens démoniaques | 44 × 72 px |
| 138 | NEC-D03-r01 | Gardien des chairs mortes | Nécromanciens démoniaques | 74 × 64 px |
| 139 | NEC-D04-r02 | Nécromancien du linceul | Nécromanciens démoniaques | 51 × 82 px |

La base est de 64 px par case. Les figures gardent leurs tailles propres,
leur ombre séparée et le joueur comme référence. A/B sont deux palettes du
même dessin, avec alpha et contours conservés. Aucun zoom n'est produit.
Les accessoires sont dessinés dans ces poses statiques de revue ; les futures
couches modulaires et prises animées ne sont pas validées par cette galerie.

## Planches natives

![Nécromanciens à taille native](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/necros-25/demons-132-135.png)

![Nécromanciens à taille native](C:/Users/User/Desktop/project-RL/artifacts/npc-monsters-review-01/necros-25/demons-136-139.png)

## Sources et périmètre

Création et corrections avec l'outil intégré `image_gen`. Les huit sources
courantes et cinq essais antérieurs sont conservés sous
`assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/`.
Les prompts exacts, références, empreintes et versions sont enregistrés dans
[necromancers-25.json](necromancers-25.json). Le [catalogue complet](CANDIDATS-MONSTRES.md)
réunit maintenant 110 modèles, avec nom, code, statut et image.

```powershell
python tools/art/show_demon_gallery_review.py --review necromancers-25.json --output necros-25
python tools/art/show_monster_catalogue.py
python tools/art/validate_monster_catalogue.py
```

Il s'agit d'apparences proposées. Les noms, compétences de nécromancie,
invocations, nombres de serviteurs, effets, statistiques, animations et
rencontres ne sont pas définis par ces dessins. Aucun nouvel acteur n'est
raccordé au jeu ; les anciennes validations et les suppressions restent intactes.


## Révision 26 — reliquaire du 137

Le joueur demande de remplacer le livre du 137 par un artefact sordide. La r02
propose un noyau de chair desséchée, retenu dans du métal noirci et deux
crochets d’os. Il confirme les sept autres pour leur inscription dans la liste.
Voir `NEC-D02-ARTEFACT-26.md` et `NEC-D02-artifact-26.json`. La r01 du 137
reste conservée hors de la fiche courante. Aucun pouvoir nouveau n’est défini.
