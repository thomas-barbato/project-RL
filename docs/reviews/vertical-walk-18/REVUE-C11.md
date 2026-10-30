# VERT18-C11 — manche du couteau dans les doigts

Signalement du 30 octobre 2026 : « on dirait que la lame sort du revers de la
main, alors que normalement la main agrippe le manche et c'est tout ».
Proposition B préparée pour les douze tenues C10 ; **pas d'intégration normale**.

Le gros plan a révélé des attaches héritées des silhouettes C06, ajustées par
une recherche de pixels sombres qui retrouvait parfois le protège-poignet des
nouvelles tenues. Le couteau était alors placé plusieurs pixels au-dessus du
centre des doigts ; le masque de premier plan ne contenait pas les vrais doigts.
Le précédent test de simple contact ne suffisait donc pas à vérifier la prise.

C11-B utilise des repères examinés sur les doigts de chaque tenue, de face et
de dos, puis suivis dans le cycle original. Une main tenant un couteau emploie
son masque de doigts autour de ce nouveau repère. Le manche passe derrière
ces doigts. L'autre main conserve son attache et son masque précédents.
Les parties complémentaires du corps et des doigts se recomposent exactement.

Les pixels du corps, du couteau et du pistolet restent ceux de C10/C06.
La lame reste à 20° vers l'extérieur et vers le bas, règle C08-B. Le pistolet
garde sa position, son échelle et son inclinaison C09-B. Aucun changement de
contenu, d'identité de tenue, de gameplay, de sauvegarde ou de préférence.

Deux études de mains ont été dessinées avec Imagegen intégré, puis conservées
comme essais **non utilisés**. Le correctif montré B utilise les doigts
d'origine ; il corrige les attaches et leur superposition. Prompts et provenance :
`prompts-C11.json`, `generation-C11/`. Sources et géométrie effective :
`assets/source-art/vertical-walk-review-18/knife-contact-C11/`, notamment
`knife-attachments.json` et `composites/`.

## Comparatif et contrôles

L'aperçu `knife-contact-C11/apercu.html` compare A et B sur les douze tenues,
avec deux couteaux 70 ou pistolet 104 + couteau 70, tête sans casque équipé
ou chacun des six casques. Les figures sont natives64 et les détails de mains
sont explicitement agrandis ×6. L'animation peut être arrêtée par phase.

`ANALYSE-C11.json` et `native-checks-C11.json` consignent :

- 3 744 contacts entre pixels du **manche** et doigts contrôlés, sur les douze
  modèles de couteaux/dagues, les deux mains et les treize poses des douze tenues.
- 168 compositions B et 24 comparaisons A dessinées dans le moteur : 2 496
  poses, 768 retours frontaux et 72 planches natives conservées.
- Quatre groupes dans le vrai camp : 624 observations, 48 retours frontaux,
  gain d'acteur identique 1,48 en marche et au repos ; 24 captures natives.
- 5 646 empreintes des banques C06/C08/C09/C10 et des ressources normales C09
  conservées. Aucune ressource de jeu normal remplacée.

Le contrôle humain de la lecture de la prise reste distinct de ces mesures.
Selon AGENTS.md, les nouveaux rendus sont présentés à taille native avant
intégration. Le retour sur C11-B ne constitue pas un accord automatique sur
les poses à deux mains C12, les palettes de torse ou les autres acteurs.
