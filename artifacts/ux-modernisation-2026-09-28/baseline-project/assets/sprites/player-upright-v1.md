# Joueur debout — direction approuvée le 3 octobre 2026

`player-upright-v1.png` est la source RGBA transparente embarquée dans le jeu.
Les trois poses regardent à droite : mains vides, mêlée, distance. La gauche
utilise un retournement horizontal ; le personnage reste debout.

Création originale avec l'outil imagegen, à partir de la planche de joueur
approuvée par l'utilisateur. Aucun élément de Cogmind n'est repris comme asset.

Consigne de dérivation : conserver le même explorateur humain, sa silhouette
debout de trois-quarts, son armure ivoire/grise, sa visière latérale cyan et ses
pieds clairs. Fournir une bande transparente de trois poses à droite, sans
texte ni décor : mains vides, petite lame tenue en main, arme compacte à deux
mains. Simplifier les détails pour une lecture dans une petite case de jeu.

`src/terminal_player.rs` prépare une seule fois un atlas de 72 × 44 pixels,
avec des rendus natifs distincts à 20 et 24 pixels.
L'échantillonnage conserve les proportions et aligne les pieds entre les poses.
Trois valeurs neutres et un accent cyan assurent le contraste sur le sol sombre.
Le filtrage reste au plus proche ; aucune rotation ni teinte du thème d'interface
n'est appliquée. Le PNG source reste intact.

Cette intégration concerne le joueur. Les apparences des NPC restent celles du
rendu existant et attendent une proposition distincte.
