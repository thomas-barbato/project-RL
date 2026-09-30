# Miniquad 0.4.11 — correctif local du cache de textures

Source : paquet crates.io `miniquad 0.4.11`, licences MIT / Apache-2.0 conservées.
Les sources des différentes plateformes et les ressources Java/JS sont conservées.

Modification fonctionnelle unique : dans `src/graphics/gl.rs`, `delete_texture`
invalide les liaisons de textures avant la suppression OpenGL. La suppression
d'une texture liée modifie l'état OpenGL ; le cache doit refléter ce changement,
y compris si le pilote réutilise ensuite le même identifiant de texture.

Sans ce correctif, l'agrandissement de l'atlas de police lors de l'ouverture
d'un menu peut produire des rectangles noirs à la place du texte. Reproduction
Windows : démarrage normal, plusieurs images de jeu, puis ouverture d'Échap.
Un préchauffage des caractères réservé aux captures masquait cette régression.

Validation native depuis la racine du projet, dans un nouveau dossier :

```powershell
cargo run -- --ui-cold-pause C:\Temp\rl-cold-pause
cargo run -- --ui-smoke C:\Temp\rl-ui-check
```

Ne pas remplacer par un préchauffage fini des caractères : les textes des mods,
les tailles de police et le DPI peuvent provoquer de nouvelles allocations.
Lors d'une mise à jour de Miniquad, comparer cette modification à l'amont et
rejouer ces diagnostics sans préparation spéciale de la police.
