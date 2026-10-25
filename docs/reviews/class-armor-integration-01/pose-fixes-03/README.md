# CLASS-POSE-FIX03 — bras de pistolet et posture de la Métamorphe 01

Le joueur signale les textures incorrectes des bras du Bastion et les manches
en double de l’Arcaniste. Il précise ensuite que la Métamorphe 01 avec casque
ouvert et fusil a une posture différente des autres tenues, et qu’il souhaite
harmoniser cette posture, pas déplacer la case.

Les dix tenues du Bastion et de l’Arcaniste reçoivent des bras repliés peints
séparément. Les masques retirent l’intégralité des anciens avant-bras, mains
et poignets de manches. Les nouveaux bras sont positionnés par rotation rigide,
sans étirement ; les armes conservent leurs dessins et leurs dimensions.
Les doigts sont une petite couche devant la poignée. Un pistolet ne modifie
que son propre côté ; deux pistolets utilisent les deux côtés. Les autres
prises conservent leurs ressources et leur montage précédents.

ME-POST03 reprend uniquement les jambes et les appuis de la Métamorphe 01.
Son buste, sa tête, ses bras et ses points de prise d’armes conservent les pixels
de la référence approuvée. Le casque ouvert conserve le visage de cette classe.
Les sources et références acceptées restent figées ; les corrections disposent
de leurs propres fichiers sous `assets/source-art/class-armor-pose-fixes-03/`.

La page courante présente les cinq stades avec un et deux pistolets et compare
la Métamorphe 01 avant/après aux tenues 02–05 avec le casque ouvert et le fusil.
Les planches utilisent la taille native ; le zoom supplémentaire est explicite.

La demande autorise la correction du montage intégré. Ces corrections restent
à examiner par le joueur : les contrôles techniques ne constituent pas une
nouvelle validation artistique. Aucun contenu, effet, chiffre ou format de
sauvegarde ne change.

`before-cell-hashes.json` fige les 3 100 rendus précédents. Seuls les pistolets
du Bastion et de l’Arcaniste et la posture de la Métamorphe 01 doivent changer.
Les résultats effectifs et captures du moteur sont consignés dans `checks.json`.
