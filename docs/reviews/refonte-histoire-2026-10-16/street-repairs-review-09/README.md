# U01-REPAIRS-09 — collisions, entrepôt et jonction sud

Demande du 20 octobre 2026 : harmoniser le bâtiment en L au toit bleu,
supprimer le long trait noir de la route en Y 14, vérifier les collisions
signalées et reprendre le raccord de l’extension aux lignes Y 48–49.

## Collisions corrigées — COL-U01-09

La fabrication précédente transformait en obstacles des débords de sprites,
gravats périphériques et petits morceaux de silhouettes. L’alpha servait à
déduire des collisions de cases entières ; les empreintes réelles n’étaient
pas conservées séparément. La première correction COL-U01-01 couvre 61 cases
différentes et ne traitait pas ces nouveaux signalements.

Ce lot libère **63 cases supplémentaires**, dont (14,16), (15,40), (2,30),
(2,31), (15,32), (16,32), (17,32) et la continuation du trottoir en (20,49–51).
La carrosserie en (16,33), les bâtiments, portes et limites de ville restent
solides. Le manifeste `assets/maps/prison-street-expanded-01/collision-repairs-r09.json`
conserve les emprises au sol explicites et la liste figée des corrections.
Le disque de contact de 10 px sert au contrôle de fabrication ; il ne
remplace pas le déplacement par cases et n’ajoute aucune statistique au joueur.

La génération **160** applique les deux lots dès la création de la rue.
Les générations antérieures conservent leur carte historique pour la
vérification et le rejeu, puis adoptent les corrections par la commande
existante `RepairStaticZoneFloors`, sans tour et une seule fois par cellule.
Un ancien journal contenant déjà COL-U01-01 n’empêche plus l’adoption du lot
suivant. Les anciennes parties où la rue est encore différée appliquent le
lot à leur première entrée réelle, après vérification de la sauvegarde.
Le rejeu restaure seulement les commandes effectivement enregistrées ; il
n’active jamais une adoption future pendant la vérification historique.

## Nouveaux pixels validés et intégrés — 20 octobre 2026

- **STYLE-U01-09** : édition de l’entrepôt complet et isolé avec imagegen
  intégré. Sa forme en L, la hauteur de ses façades, son extrémité sud fermée,
  la porte de garage et les accessoires de toiture sont conservés. Le bleu
  devient ardoise, les briques et conduits moins saturés, les contours moins
  noirs et uniformes. Export natif de 651 × 403 px.
- **ROAD-U01-09** : la texture extraite emportait un bord de route et des
  grilles. Après rotation pour les branches latérales, cette bande traversait
  la chaussée, notamment en Y 14. Un matériau d’asphalte dessiné séparément
  remplace seulement les bandes de bord, avec raccord de 16 px ; le reste de la
  texture et la géométrie des rues sont conservés.
- **JONC-U01-09** : le fond extérieur de l’ancien bloc est supprimé en suivant
  sa façade ; les haies complètes sont composées après le bloc de départ,
  avec orientations et positions variées. Le composant tronqué du nord-est
  est retiré entièrement et remplacé par une annexe complète réutilisée du
  kit, conditionnée sans étirement dans 224 × 212 px. Les cases pleines
  sous cette annexe sont déjà solides. Son marqueur de porte fermée reprend
  un vrai éclairage rouge du moteur ; les anciennes lentilles retirées ne
  laissent pas de halo. Le départ et les pièces visitables sont conservés.
  Le corps et ses deux décors de sang proches du composant remplacé sont
  déplacés ensemble de 144 px vers l’ouest et 32 px vers le sud, sur la
  chaussée au bord du trottoir, pour rester au sol et ne pas apparaître sur
  le toit de l’annexe.

Le joueur valide le choix 1, avec suppression de la caisse en bois, du baril
bleu et du meuble vert près de l’entrepôt en L. Ces trois modules indépendants
ne sont plus composés dans le décor. À sa demande suivante, leurs cases
reprennent le semi-effacement des joints : les mêmes dalles acceptées
PAV-U01-06B/C, avec la rotation fixe par coordonnées du reste de la rue,
remplacent les joints trop nets en (9–12,7–8). Le sol est composé avant les
bâtiments ; aucune peinture ne recouvre les façades.

La case (11,7), encore bloquée sous le meuble, est libérée. Les autres cases
des trois objets étaient déjà libres. La génération **161** ajoute uniquement
cette correction ; les listes historiques des 61 et 63 cellules, le plan et
les obstructions d’origine restent inchangés pour le rejeu. La reprise des
parties 158, 159 et 160 est contrôlée avec rue visitée ou différée, instantané
et rejeu, puis nouvelle sauvegarde et recharge.

Les sources restent sous `assets/source-art/first-sector-review-01/u01-repairs-r09/`.
Le candidat montré est conservé intact ; `baseline/` fige les quatre fichiers
artistiques précédents et `integrated/` conserve les exports finaux.
Le rendu normal utilise les copies sous `assets/maps/prison-street-expanded-01/`,
y compris les cartes des vrais éclairages. `acceptance.json` enregistre
l’accord, les deux demandes de retouche et les empreintes des fichiers activés.
Le contrôle de différence confirme que les retouches après présentation
restent limitées aux trois objets et à leurs dalles.

## Revue et preuves

La [page de revue courante](../../../../artifacts/street-defects-review-01/index.html)
compare neuf positions dans le moteur, aux dimensions natives : entrepôt,
sol libéré près de l’entrepôt,
route, cratère, voiture, gravats ouest, gravats de la cour, jonction et départ.
Les parties de diagnostic sont isolées et n’écrivent pas les sauvegardes du
joueur. La vue complète du décor retire le masque de vision uniquement pour
la revue ; les scènes de jeu conservent le cercle de six cases.

Les [sources et prompts](../../../../assets/source-art/first-sector-review-01/u01-repairs-r09/generation-receipts.json)
de l’imagegen intégré sont conservés. `imagegen-prompt.json` détaille la
consigne de l’entrepôt. `checks.json` décrit les contrôles exécutés et
`proofs/` conserve les captures présentées et `proofs/integration/` le rendu
normal après activation, avec leurs empreintes séparées.

Scripts reproductibles : `build_street_repairs_09.py` (emprises),
`build_street_repairs_09.py --visual` (composition candidate et lumières),
`show_street_repairs_09.py` (page courante). Les deux anciens composeurs
gardent leur comportement par défaut ; les paramètres de réparation sont
facultatifs et ne réécrivent aucune source historique.
L’activation reproductible utilise `integrate_street_repairs_09.py`, puis
`show_street_repairs_09.py --integrated` publie les preuves du rendu normal.

### Suivi de l’éclairage découvert pendant DEFENSE-U01-10

Le helper partagé conservait les dimensions 48 × 36 de son ancien prototype
pour générer l’irradiance, même avec l’image de 26 × 66 cases. La sortie
intégrée r09 fait donc 384 × 288 échantillons au lieu de 208 × 528 : les sources
et masques existent, mais leur répartition est mal recalée sur le monde.
Les validations de collisions et de sauvegarde ci-dessus restent distinctes
de ce défaut visuel. Le helper utilise désormais les dimensions de l’image.
Le recalage correct est montré dans la proposition DEFENSE-U01-10, avec les
nouveaux feux. Les ressources normales r09 restent figées pendant cette
prévisualisation ; appliquer la correction d’irradiance lors de l’intégration
du rendu accepté, sans modifier collisions, toits ou sauvegardes.
