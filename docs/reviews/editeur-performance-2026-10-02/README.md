# Grande carte : fluidité et ouverture dans l'éditeur

Les lanceurs utilisent maintenant l'exécutable optimisé `target/release/examples/map_editor.exe`. `Essayer_grande_carte.cmd` ouvre directement la carte modifiable, près du départ en ville. Depuis l'éditeur : Cartes (F6), sélectionner `grande-ville-et-nature-2026-10-02`, puis Ouvrir. Charger (Ctrl+O) permet aussi de saisir le chemin complet du JSON.

## Cause et correction

Le cache de peinture était limité à 1 024 cases, alors que la vue au zoom 16 en montre environ 2 600. Il évincait les cases par coordonnées, provoquant des recompositions répétées aussi bien au zoom éloigné que lors d'un déplacement vers l'ouest. La capacité suit désormais le nombre maximal de cases visibles, avec une marge de 512 ; l'éviction utilise l'accès le plus ancien. Les entrées et leurs métadonnées restent bornées. L'invalidation locale après peinture est conservée.

La miniature dessinait chaque sol et chaque objet à chaque image. Son rendu reste maintenant en cache jusqu'à une modification du document ou de ses dimensions d'affichage ; le repère de caméra continue de se déplacer. Les traits de gomme sont également ignorés lorsqu'ils rencontrent une case de peinture entièrement transparente, opération sans effet sur le résultat visuel.

## Mesures natives

Même carte 160 × 112, fenêtre 1 360 × 840. Le diagnostic mesure le rendu, la présentation GPU et l'attente de la prochaine image. Les plages ci-dessous excluent la première image de chaque vue ; elles proviennent de `baseline-debug/performance.json` et `validation-release/performance.json`. Le gain combine la correction des caches et l'utilisation du mode optimisé. Le dossier `corrected-debug` isole le premier effet des caches, avant l'optimisation des gommes.

| Vue | Avant, version debug | Après, version optimisée |
| --- | ---: | ---: |
| Ville, zoom 16 | 4 092–4 192 ms/image | 9,5–9,9 ms/image |
| Nature, zoom 16 | 2 113–2 191 ms/image | 13,3–15,3 ms/image |
| Défilement, zoom 32 | 1 140–1 220 ms/image | 7,0–10,5 ms/image |

Les vues fixes au zoom 16 ne recomposent plus aucune case après la première image, contre 2 165 cases par image en ville et 1 447 en nature auparavant. Le défilement recompose seulement les cases entrant dans le cache (0 à 20 dans ce parcours), contre environ 660 à 680 auparavant. Une pose de sol prend 15,6 ms, huit points de pinceau 0,06 ms et l'entrée en mode Tester 15,5 ms. Le déplacement du joueur est également mesuré avec suivi de caméra.

Des pauses de préparation restent possibles au premier affichage : environ 92 ms pour la vue éloignée en ville, 293 ms en nature dans la validation finale, ainsi que des pointes au démarrage dues au rendu initial. Le passage ultérieur dans `import-bibliotheque` confirme les vues fixes à environ 10–13 ms ; ses premières images présentent aussi des pointes initiales. Les mesures décrivent cet ordinateur et ces parcours, sans garantir une cadence constante sur tous les matériels.

## Vérification

- 55 tests de l'exemple `map_editor` et 28 de `textured_surface_preview` réussis, dont les nouveaux contrôles de cache sur plus de 1 024 cases, d'éviction lors d'un déplacement vers l'ouest et de capacité bornée.
- Formatage, `cargo check --locked --examples`, compilation optimisée et contrôle des espaces du diff réussis.
- Les images de scène avant/après la première correction sont identiques ; les différences sont confinées à la miniature, désormais rendue dans une texture.
- Chargement réel du JSON par l'éditeur et présence dans Cartes vérifiés. Le document de l'aperçu de bibliothèque est exactement égal au document chargé. La capture `import-bibliotheque/cartes.png` montre le fichier sélectionné et son aperçu natif.
- Le placement, le pinceau et les déplacements de test sont annulés avant les captures. Aucun diagnostic n'enregistre la carte, les préférences ou une récupération automatique.

La carte conserve son empreinte SHA-256 : `B39D3B3B98F76B3958B394E6A0538EA1FB858CD854A2B528815CA0B8779D150C`. Les données, la composition et les collisions n'ont pas été modifiées.

Pour reproduire sans enregistrer la carte, choisir un dossier neuf :

```powershell
./target/release/examples/map_editor.exe --map artifacts/map-editor/maps/grande-ville-et-nature-2026-10-02.json --capture-performance DOSSIER_NEUF
```

Le dossier reçoit les chronométrages JSON et les captures ville, nature et bibliothèque. `avant/` conserve les sources initiales ; les mesures intermédiaires restent archivées séparément.
