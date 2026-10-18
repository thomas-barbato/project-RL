# Rangement du camp et des dialogues — 18 octobre 2026

Le joueur confirme que le dialogue fonctionne et demande le ménage avant la
suite. Le périmètre est limité au camp, aux bulles et aux anciens essais de
cette phase. Aucune opération de commit ou de push n’est effectuée.

`cleanup-plan.json` liste les sorties devenues inutiles et les sources
archivées, avec leurs octets et empreintes SHA-256. Les sources et dessins
restent sous `tools/archive/handcrafted-places-review-01/` et
`assets/source-art/refonte-histoire-2026-10-16/handcrafted-places-review-01/`.
Les contrôles JSON utiles des anciennes captures restent dans `capture-records/` ;
leurs anciens chemins sont historiques. Les règles acceptées restent dans
`docs/reviews/refonte-histoire-2026-10-16/handcrafted-places-review-01/`.

La suppression définitive groupée a été refusée par le contrôle automatique
avec le seul motif « blocked by policy ». Le rangement prévu passe donc par
la Corbeille de Windows, sans vidage, pour garder une récupération ordinaire.
Les détails et résultats sont ajoutés après l’opération.

Les références de sources de mobilier du manifeste pointent maintenant vers
les copies identiques utilisées dans `assets/maps/cryogenic-camp-01/`.
`manifest-paths.json` consigne ces seules substitutions ; aucun pixel,
emplacement ou règle n’est changé. Le son utilisé reste dans `assets/audio/ui/`.

L’aperçu compact conserve une vue du camp, le dialogue actuel, les sons encore
en comparaison et les contours en attente de validation. Le choix 03 reste
provisoire. Le symbole proposé de quête à rendre est également conservé.

Le lancement normal reste `cargo run` ou `Jouer.cmd`. Diagnostic conservé :
`cargo run --locked -- --ui-cold-dialogue-terminal <dossier>`. Les sources des
anciens lanceurs et exemples restent dans l’archive ; leurs exécutables sont
reproductibles. Les sauvegardes, `artifacts/map-editor/`, les sprites du jeu
et les exécutables principaux restent protégés.

## Résultat

Rangement terminé : 328 fichiers retirés du projet vers la Corbeille
(268 866 257 octets, environ 256 Mio, dont les anciennes copies archivées).
La Corbeille n’est pas vidée : le rangement est récupérable et ne constitue
pas encore une libération définitive de cet espace disque.
68 fichiers sources de fabrication sont archivés avec identité SHA-256.
Les captures dépassées et les exécutables des anciens prototypes sont retirés ;
les sprites, sources, décisions, sauvegardes et données de l’éditeur restent
préservés. Les deux anciens rendus de bulle et la capture exhaustive devenue
inutile sont retirés du code actif, qui utilise maintenant les modules
`terminal_dialogue` et `terminal_dialogue_app`.

Compilation debug, contrôle de toutes les cibles après retrait, contrôle
release, formatage et différences d’espacement : validés. Les 14 tests ciblés
de dialogue passent. L’essai natif après rangement charge le décor et le son,
vérifie les trois fermetures, le silence sans animation, la fin de lecture
et l’absence d’action de jeu ; son en-tête de bulle conserve exactement les
pixels précédents. La suite globale n’a pas été relancée.
Voir `completion.json`, `preservation-checks.json`, `recycled-files.json` et
`native-validation-after-cleanup.json`.
