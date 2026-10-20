# ART-QUALITY-01 — cohérence des corps, personnages, équipements et monstres

19 octobre 2026. Audit demandé après l'intégration de CIVILS-U01-08.
Les constats et recommandations ci-dessous ne sont pas une validation de
nouveaux dessins. Aucun personnage, équipement, monstre ou pouvoir n'est modifié.

## Conclusion

L'impression du joueur est fondée : les nouveaux corps ont une finition plus
naturelle et plus adaptée aux décors usés que plusieurs personnages actuels.
Les plis, les teintes de vêtements, les ombres et les postures sont moins
uniformes. L'écart ne vient cependant pas d'une résolution inférieure pour
tout le bestiaire. Les tenues récentes, les monstres de campagne et beaucoup
d'icônes d'équipement utilisent déjà des sources adaptées au 64 px.

Le problème principal est une **cohabitation de finitions**, aggravée sur
certains chemins historiques par de véritables dessins 32 px agrandis.
Il faut traiter ces deux causes séparément. Agrandir les personnages ou
changer globalement le filtrage ne donnerait pas la finition des nouveaux corps.

## Comparaison contrôlée

La planche `board-native.png` conserve exactement les pixels des zones d'image
de `native-renderer.png`, capture du moteur. Seuls les libellés sont recomposés
pour éviter leur chevauchement. Les vingt échantillons sont cinq corps, quatre
tenues avec armes, un PNJ du camp, cinq monstres, trois icônes d'équipement,
une armure historique et une invocation historique.

Chaque case mesure 64 px. Les corps restent sur leur toile de 80 × 48 px ; les
personnages gardent leurs couches, attaches et ombres habituelles. Les démons
ailés débordent de leur case à leur taille prévue. Aucune silhouette n'est
normalisée à la taille de la plus grande. Un fond identique sans lumière de
scène permet d'isoler les différences présentes dans les dessins. Les icônes
sont montrées dans une case de 64 px, sans prétendre que chaque emplacement de
l'interface mesure exactement cette taille.

La vue `integrated-house-open-native.png` de la revue CIVILS-U01-08 complète la
planche : c'est le rendu normal avec usure, pénombre et lumières du moteur,
identique à la proposition validée. Cette partie technique est créée sans
choix de classe ; sa silhouette ne représente donc pas à elle seule les cinq
classes récentes. La planche montre aussi des tenues de classe actuelles.

L'audit est représentatif, pas une validation individuelle de toutes les
centaines de variantes du bestiaire. Les chargeurs et chemins de composition
ont également été examinés pour distinguer un défaut de dessin d'un défaut
d'échelle. La capture ne charge aucune partie et ne touche à aucune sauvegarde.

## Constats et priorités

| Priorité | Catégorie | Constat | Action recommandée |
|---|---|---|---|
| Haute | Personnage et armures récentes | Les contours épais, les avant-bras très clairs et les grandes surfaces régulières donnent un rendu plus proche d'une figurine ou d'une illustration en aplats. Les corps ont des ombres et plis plus nuancés. Le Bastion et Cerbère rendent l'écart particulièrement visible. | Reprendre la finition d'une tenue pilote : matières mates, ombres par masses, usure située et contours moins uniformes. Garder la silhouette lisible et les attaches. |
| Haute | Armures historiques et invocation | La Brigandine Cognefer et le drone 57 montrent encore de gros pixels issus de sources 32 px. La différence est technique en plus d'être artistique. | Prévoir de véritables remplacements natifs, avec revue ; ne pas les lisser pour masquer le manque d'information. |
| Moyenne | Monstres récents | Les cultistes 164/165, le Possédé 005 et les démons ailés 144/145 sont déjà natifs. Les silhouettes restent identifiables. Les démons ont des volumes propres, des reflets réguliers et des couleurs assez franches, davantage dessinés en aplats que les corps. Les petits cultistes perdent davantage de matière à taille réelle. | Garder les designs, les tailles et les variantes acceptés ; tester une finition moins lisse sur un humain et un démon avant d'étendre la retouche. |
| Moyenne | Armes portées et icônes | Les icônes du fusil et du pistolet restent nettes et détaillées. L'arme portée occupe logiquement moins de pixels ; le pistolet est petit et clair dans la main. Ce n'est pas la preuve d'une mauvaise résolution générale. | Vérifier les matières et le contraste de l'arme portée dans la pénombre, en conservant la taille réelle, la prise et les doigts. Ne pas rendre l'arme aussi grande que son icône. |
| Faible | PNJ du camp | Le PNJ échantillonné est déjà plus sobre et plus proche des corps : tenue sombre, volumes discrets et proportions moins accentuées. | Le garder comme deuxième référence de cohérence. Pas de refonte générale des PNJ justifiée par cet échantillon. |

Confiance élevée sur les différences visibles dans les échantillons et sur
leurs chemins techniques. L'ordre précis des retouches de l'ensemble du
bestiaire reste à établir par famille ; il ne peut être déduit de cinq monstres.

## Ce que vérifie le code actuel

- `src/game_art_worn.rs`, `outfit_index`, `source_resolution` et `draw_key` :
  dix-neuf tenues récentes sélectionnent la composition native 64 lorsque la
  case dépasse 32 px. Leur toile transparente de 128 px ne signifie pas que
  le personnage est agrandi. En revanche, les torses 102, 104 et 113 ne passent
  pas par ces liaisons natives : **Brigandine Cognefer**, **carapace composite**
  (`core:composite_carapace`) et **veste matelassée** gardent un ancien chemin.
  Les anciennes classes/sources de compatibilité peuvent aussi y passer.
- `src/game_art_flat.rs` : corps, manches, armes et doigts composés séparément.
  Les pièces d'équipement secondaires ne sont pas toutes des couches visuelles
  indépendantes : la tenue complète est principalement choisie par le torse.
  Conserver les attaches actuelles ne dispense donc pas de définir les futures
  pièces réellement interchangeables avant une refonte modulaire complète.
- `src/game_art_campaign.rs`, `destination` : les monstres de campagne sont
  placés avec une échelle `case / 64` et un ancrage aux pieds. Le cultiste 164
  occupe réellement 34 × 60 px dans une toile de 128 px ; le Possédé 005,
  62 × 60 px. Ce sont des tailles natives, pas des sprites 32 px doublés.
- `src/game_art_bestiary.rs` conserve une banque historique dont les cellules
  32 px sont dessinées dans la case actuelle. `game_art::agent_drone_visual`
  utilise encore ce chemin pour `agent:d57`. La planche montre cet exemple
  séparément des monstres de campagne.
- `src/game_art_palette.rs` choisit les icônes natives lorsque disponibles,
  et conserve des replis historiques. Le code ne justifie donc ni « toutes
  les armes sont floues », ni « tous les objets sont désormais natifs ».
- Le filtre de dessin est déjà `Nearest` pour les sprites comparés et les
  corps. Les nouveaux corps gardent des transitions de couleur et d'alpha
  dans leurs pixels natifs : `Nearest` ne veut pas dire palette réduite ou
  contours obligatoirement crénelés. Leur conditionnement réduit l'original
  isolé avec Lanczos, sans agrandir un petit ancien sprite.

## Suite recommandée, à proposer avant intégration

1. Préparer un petit lot pilote : une tenue du joueur, une armure portée,
   un fusil avec ses mains, un cultiste et un démon déjà connus. Conserver
   identifiants, silhouettes, couleurs fonctionnelles et règles de combat.
2. Montrer ces essais à 64 px, auprès d'un corps accepté et du PNJ du camp,
   puis sur la carte avec les vraies lumières. Comparer aussi les prises à
   une et deux mains ; une belle pose seule ne valide pas toutes les armes.
3. Après accord sur la finition, traiter les ressources historiques encore
   agrandies, puis harmoniser les autres familles progressivement. Vérifier
   les couches et les variantes de couleur à chaque lot.

Je recommande de viser une cohérence de matières et de volumes, sans copier
toute la densité de détail des cadavres sur chaque acteur. Un corps immobile
peut porter des détails que le joueur doit ignorer pour lire un ennemi en
combat. Les silhouettes actuelles ont cette qualité de lisibilité à préserver.
Les morts de combat et leurs animations restent un chantier séparé de ces
décors statiques ; le lot civil ne valide aucune nouvelle règle de mort.

## Règles et preuves conservées

Les règles du lot accepté sont inscrites dans
`docs/DIRECTION_ARTISTIQUE_TILES.md`, section **Référence de finition
CIVILS-U01-08**, et rappelées dans `docs/STYLE_VISUEL_REFERENCE.md`.
Elles fixent l'échelle, les dessins entiers et transparents, les contours
lisibles, les matières usées, les décors de sang séparés et la revue en scène.
Les recommandations de cet audit ne les remplacent pas par une refonte déjà
approuvée des personnages.

`art-before-audit.json` conserve les empreintes des ressources examinées ;
`checks.json` vérifie qu'elles restent identiques et que la planche ne modifie
aucun pixel de ses vingt zones d'image. `native-renderer.png` est la capture
brute et `board-native.png` sa présentation lisible à la même échelle.

Reproduction :

```text
cargo build --locked
target/debug/project-rl.exe --ui-cold-art-quality-audit artifacts/art-quality-audit-01
python tools/reviews/show_art_quality_audit.py
```
