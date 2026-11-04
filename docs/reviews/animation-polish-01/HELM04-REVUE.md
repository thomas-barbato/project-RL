# HELM04 — cinq casques selon les poids visuels d'armure

Les cinq icônes sont acceptées le 31 octobre par « oui ca me va :) ».
L'accord figé est dans `HELM04-ACCEPTANCE.json`. La suite prépare leurs vues
portées communes et leur ajustement à la carrure/armure ; ces nouveaux dessins
ne sont pas acceptés par extension de l'accord sur les icônes.

Le 31 octobre 2026, le joueur demande de produire toute la gamme avant de la
juger, avec autant de modèles de casque que de types d'armure par poids.
La gamme visuelle des tenues utilise cinq degrés : très léger, léger,
intermédiaire, lourd et très lourd. La table de
`class-armors-all-01/REVIEW.md`, les cinq rangs du manifeste de classe et
`src/game_art_class_armor.rs::outfit_index` en conservent les références.
Il s'agit de degrés visuels ; ils ne définissent pas cinq nouvelles tranches
de masse en grammes dans les règles de jeu.

## Règle révisée : même casque dans l'icône et sur le personnage

Le joueur remet ensuite en question le changement de dessin selon la classe.
L'avis retenu est de conserver un dessin commun et reconnaissable par modèle,
fidèle à son icône : mêmes formes, construction, matières, palette, ouvertures,
visière et détails. Seuls les dimensions, les proportions nécessaires au port,
le placement et la projection s'adaptent à la carrure et à l'armure portée.
Face, dos et côtés montrent le même objet. Pour un même équipement, le gabarit
ne varie pas arbitrairement entre les phases ou les angles.

Cette décision remplace la demande de casques redessinés par classe de HELM02
et HELM03. Les règles de dessin propres aux armures de torse restent distinctes.
Les citations et la portée exacte sont dans `HELM04-REGLE.json`.

## Cinq candidats, icônes d'abord

| Repère stable | Poids visuel | Direction de dessin | Provenance |
| --- | --- | --- | --- |
| H04-W01 / 01 | Très léger | Protection ouverte à arceaux, tempes et sangles visibles | Nouveau dessin |
| H04-W02 / 02 | Léger | Coque effilée et optique asymétrique cyan | H03-I02, pixels conservés |
| H04-W03 / 03 | Intermédiaire | Coque organique segmentée à crête courte | H03-I03, pixels conservés |
| H04-W04 / 04 | Lourd | Blindage industriel anguleux cuivre et visière ambre | H03-I01, pixels conservés |
| H04-W05 / 05 | Très lourd | Coque intégrale large, verrou de nuque et visière rouge-orangé | Nouveau dessin |

Les numéros HELM03 restent réservés à leurs recherches initiales. Le classement
ci-dessus emploie les nouveaux identifiants H04-W01–05 ; il ne renumérote pas
leurs sources. Les cinq intitulés sont des repères de revue, pas des noms
d'objets acceptés. Les six objets équipables existants ne sont pas encore
affectés ou fusionnés. Aucun modèle ne devient réservé à une classe.

Les cinq icônes sont présentées ensemble à 64 px, sur fonds clair et sombre,
avec leurs silhouettes et un zoom entier. La protection très légère n'a pas
de visière lumineuse. Les quatre autres conservent un essai optionnel de
clignotement limité au verre coloré, distinct de l'accord sur le dessin.

La fabrication complète est autorisée, la validation des dessins reste à venir.
La priorité demeure les icônes ; les poses portées et le calibrage à la carrure
et à l'armure seront produits après la décision sur cette gamme.

## Sources

Deux nouveaux dessins utilisent **imagegen intégré** sur fond transparent.
Les trois sources HELM03 sont réutilisées sans repeinture. Les prompts et leurs
provenances sont dans `HELM04-PROMPTS.json` et `HELM03-PROMPTS.json`.
Les ressources de ce lot sont conservées dans
`assets/source-art/animation-polish-01/helmet-weight-r01/`.

Les planches reproductibles et le lecteur restent dans l'aperçu courant, sous
`artifacts/animation-polish-01/`. Aucune intégration normale ou dépendance du jeu
aux captures n'est ajoutée.

## Reproduction et contrôles

```powershell
python tools/art/build_animation_polish_review.py
```

La même page ouvre désormais la gamme complète à l'ancre `#gamme-casques` ;
l'ancienne ancre `#nouvelles-icones` reste disponible. Le lecteur remplace
la proposition de trois icônes par les cinq modèles, avec aperçu compact,
fonds clair/sombre, silhouettes et comparaisons fixe/animée.

Les cinq PNG natifs et les trois réutilisations identiques sont vérifiés.
Seuls les pixels des visières changent dans l'animation : 0/39/8/16/21 pixels
pour 01–05 ; l'alpha reste identique. Le WebP natif dure 3,6 secondes.
La syntaxe JavaScript, les imports Python, l'unicité des IDs HTML et les dix
ressources PNG servies en HTTP sont contrôlés. Le bilan est dans
`HELM04-VALIDATION.json`.

Les onglets du navigateur intégré sont visibles dans l'inventaire, mais son
API n'a pas permis d'accéder à l'onglet pour cette révision. Les planches PNG
ont été inspectées directement ; aucun contrôle d'interaction ou capture du
navigateur HELM04 n'est revendiqué. Aucun nouveau contrôle Rust n'est requis
pour ce lecteur de revue sans modification du rendu normal.
