# VERT18-C10 — douze tenues intermédiaires

Proposition du 30 octobre 2026, en attente de validation visuelle. **Non intégrée
au rendu normal.** C09-B reste la version active, avec le couteau C08-B.

La poursuite demandée par le joueur couvre d'abord les douze tenues encore
absentes des quatre classes actuelles : Bastion 01–03, Traqueuse 04–06,
Arcaniste 07–09 et Métamorphe 10–12. Ces numéros de revue correspondent
respectivement aux indices de tenue 1/2/3, 11/12/13, 16/17/18 et 21/22/23.
Les sélecteurs d'armure représentatifs sont 110, 102 et 101.

## Ce qui est montré

- Douze cycles entiers : six phases de face, six phases de dos et un repos
  frontal provenant du même dessin. Comparaison aux huit tenues de base et
  lourdes déjà validées, avec la même échelle native de 64 pixels par case.
- Tête sans casque équipé, puis chacun des six casques partagés C04/C05.
  Les capuches souples des vêtements gardent leur identité ; les sources des
  armures ne servent pas à simuler une tête nue sous un casque déjà dessiné.
- Cinq ports représentatifs : mains libres, pistolet 104, deux couteaux 70,
  deux pistolets 104, pistolet 104 + couteau 70. Les armes restent les dessins
  partagés de C06, avec les angles acceptés C08-B et C09-B, sans redessin.
- Palette d'origine uniquement. Les couleurs de torse, autres armes à une main
  et prises à deux mains ne font pas partie de cette revue de silhouettes.

L'aperçu courant est `armor-C10/apercu.html`, sous le serveur de revue local.
Il présente les douze silhouettes ensemble, les treize poses séparées, les
variantes d'équipement et les captures du vrai rendu du camp. Les images
gardent leur taille native ; les grandes planches se font défiler horizontalement.

## Fabrication et corrections

Les seize figures de chaque source ont été dessinées entières avec **Imagegen
intégré** (`builtin_imagegen`), sur transparence réelle. Les prompts exacts,
références et sorties sont dans `prompts-C10.json` ; chaque génération a sa
provenance et son empreinte dans `generation-C10/`.

Les sources retenues sont celles des champs `selected_output`, à défaut
`output`, des douze jobs principaux. Les essais historiques restent conservés
comme sources, sans être montrés comme candidats courants. L'Arcaniste 07 a
été repris pour sa capuche de dos, puis les trois robes pour préserver leur
longueur et leurs manches d'origine : 07 utilise r03, 08 et 09 utilisent r02.

Le conditionnement applique une échelle uniforme sur X et Y à chaque rangée
de source, calculée à partir de sa pose neutre, avec des appuis stables.
Aucun zoom par phase, aucune greffe de jambes ou de bras. Corps et doigts
sont des masques complémentaires du même dessin ; leur recomposition est
vérifiée à l'octet. Les prises sont ajustées sur les pixels des mains du
nouveau corps ; aucun membre n'est peint dans le dessin partagé d'une arme.

Sources et atlas de préparation :
`assets/source-art/vertical-walk-review-18/armor-C10/`.
Le jeu normal ne lit aucun de ces fichiers et ne dépend pas de `artifacts/`.

## Vérifications

`ANALYSE-C10.json` et `native-checks-C10.json` consignent les résultats :

- 420 compositions préparées : 12 tenues × 7 états de tête × 5 ports d'armes,
  soit 5 460 poses dessinées à taille native dans le moteur ; 1 680 retours
  frontaux contrôlés et 36 planches natives conservées.
- 624 contacts visibles main/arme vérifiés, sans échec. Ce contrôle technique
  complète la revue visuelle de la position de la prise ; il ne la remplace pas.
- Quatre groupes de trois tenues dans le vrai camp : 624 observations de pose,
  48 retours frontaux, même traitement d'acteur et même gain 1,48 pour marche
  et repos, matériaux et éclairage réels. Vingt-quatre captures conservées.
- Lectures et décodages préparés hors dessin ; textures créées sur le thread
  graphique. Le diagnostic utilise un cache de textures borné à 32 Mio.
  L'aperçu utilise aussi un cache borné à 32 Mio et des mini-atlas natifs pour
  les douze vignettes, sans garder douze grands atlas animés en mémoire.
- Compilation Debug verrouillée réussie, scripts Python compilés,
  `git diff --check` sans erreur, avec seulement les avis LF/CRLF existants.
  Aucune suite complète de tests exécutée pour cette préparation visuelle.
- 5 159 empreintes des banques C06, C08, C09 et du jeu C09 préservées.
  Aucune sauvegarde chargée et aucune préférence écrite. Release reste C09.

Le diagnostic de décor a d'abord révélé une commande graphique encore en
attente à la libération d'un matériau entre deux groupes. La fixture attend
désormais la fin de cette image avant de libérer ses ressources. Le contrôle
complet a ensuite été rejoué avec succès ; le rendu normal n'a pas été modifié.

## Validation attendue et suite

Selon AGENTS.md, « Présenter les nouveaux candidats numérotés à taille native
avant leur intégration ». La présente proposition n'est donc pas une activation
des douze tenues et ne vaut pas validation des futures prises à deux mains.

Après accord sur ce lot, préparer les trois familles de prises à deux mains
pour ces mêmes douze tenues, compléter les modèles d'armes et leurs couleurs,
puis consigner séparément leur accord et leur intégration. Les corps de PNJ
supplémentaires, les anciennes tenues de Technomancien et les autres palettes
de torse restent hors périmètre de C10.

## État après C11 et C12

Les reprises C11-BACK-R02 et les trois prises C12 ont été validées puis intégrées
en Debug et Release. Cela couvre deux configurations en ambidextrie et les
74 armes à deux mains sur les douze tenues, selon `integration-C12.json`. Les
autres configurations C10 à une main et les palettes de torse restent distinctes.
