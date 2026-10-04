# Rotation des murs

Correction du premier éditeur natif après le signalement d'une orientation ignorée.

## Comportement

- R avec le pinceau Mur tourne le prochain mur et fixe son orientation.
- R sur un mur sélectionné tourne sa pièce actuelle, y compris un angle. Sa forme et son orientation deviennent manuelles et restent stables quand les voisins changent.
- A rétablit les raccords automatiques du mur sélectionné ou des prochains murs.
- La prévisualisation sur la carte, le rendu après placement et la sauvegarde utilisent les mêmes paramètres.
- Un mur automatique isolé utilise une pièce droite, comme sa prévisualisation.
- Copie, déplacement, annulation/rétablissement et fichier JSON conservent la forme manuelle et la rotation. Le champ optionnel `fixed_connections` conserve la compatibilité des cartes de version 1 précédentes, qui restent automatiques.

## Vérification

21 tests de l'exemple `map_editor` passent. Les nouveaux tests couvrent la rotation avant placement, les quatre rotations après placement, la stabilité face aux changements de voisins, la copie, l'annulation/rétablissement, le retour au mode automatique, le format JSON précédent et le rejet des formes invalides.

L'exécutable est recompilé. Les captures natives montrent les quatre orientations des murs droits et des angles dans `captures/wall-rotations.png`. La composition correspondante est enregistrée dans `captures/wall-rotations.json`. Les captures des bâtiments fermés/ouverts et de la scène meublée vérifient également les raccords automatiques existants.

Les images sources et le code de campagne ne sont pas modifiés.
