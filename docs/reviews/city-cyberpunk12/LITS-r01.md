# Proportions des lits · r01

Le [lit r05 prévalidé](LIT-ACCEPTANCE-r05.json) sert de repère à 64 px par
case. Les deux logements de la première ville reçoivent chacun un cadre
individuel adapté à leur décor. Le lit de la maison d’arrivée garde son métal
usé et celui du logement intermédiaire s’accorde avec le mobilier et le
carrelage de la pièce. Le raccord se limite à l’ancienne empreinte du lit.

Le quartier nord utilise le même dessin de lit dans les logements, le
dortoir des pompiers et les variantes de cellules des deux sous-sols. L’aperçu
`north_city_review` conditionne désormais ce dessin dans une emprise maximale
de 44 × 96 px, avec le pied au même point d’ancrage. Les placements et les
emprises de collision sont ceux des plans existants. L’option `--capture-lits`
cadre les chambres, le dortoir et les cellules à 64 px par case. L’option
`--lits-avant-r01` retrouve les proportions précédentes pour comparaison.

Le lit médical du camp mesure déjà 34 × 72 px, conformément à l’[audit
d’échelle](../refonte-histoire-2026-10-16/art-finish-completion-06/AUDIT_MOBILIER.md).

Les retouches de la première ville utilisent ImageGen intégré. Le
[manifeste](../../../assets/source-art/city-cyberpunk12/lits-r01/manifest.json)
associe les couches à leurs images sources et aux prompts [de la maison
d’arrivée](LITS-r01-ARRIVEE-PROMPT.txt) et [du logement
intermédiaire](LITS-r01-INTERMEDIAIRE-PROMPT.txt). Le joueur a prévalidé les
adaptations le 6 novembre 2026. Elles sont intégrées dans les deux versions
jouables de la première ville et dans l’aperçu du quartier nord.
[Accord et empreintes](ACCORD-LITS-r01.json)

Les deux fonds jouables correspondent au rendu validé sous les masques du
lit. Les quatre captures par défaut du quartier nord correspondent à leurs
comparatifs approuvés. Les tests de circulation, de porte, de sauvegarde et
de sélection des sous-sols passent.
