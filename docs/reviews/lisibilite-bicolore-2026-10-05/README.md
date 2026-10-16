# Essais bicolores : lisibilité des tenues à 30 pixels

Le 5 octobre 2026, l'utilisateur demande s'il reste possible de gagner en
lisibilité et propose une couleur supplémentaire sur certaines armures pour
éviter un aspect monobloc. Deux propositions ont été dessinées avec l'outil
intégré image_gen à partir des essais précédents, sans remplacement dans le jeu.
Les prompts exacts et les rôles des références sont dans `PROMPTS.md`.

## Direction proposée

- Bastion : casque et torse ivoire ; épaules, avant-bras et genoux bleus ;
  articulations et sous-tenue sombres. Les petits accents jaunes sont retirés.
- Technicien : casque et gilet jaunes ; manches et tissu des jambes
  bleu-violet ; articulations et chaussures sombres.
- Couleur secondaire sur des masses larges, et contours internes simplifiés.
  Le contraste de luminosité entre les matériaux compte aussi pour la lecture.
  Les deux couleurs ont un rôle de construction, plutôt que de petits ornements.
- Une même répartition des matières doit ensuite être gardée dans les huit
  poses de chaque tenue. Ces essais couvrent quatre poses : sans arme, épée
  à une main, pistolet et fusil.

Certaines tenues peuvent rester plus uniformes. Les grandes protections et
les ensembles tissu/protection sont de bons candidats au traitement bicolore.
Les futurs changements de palette peuvent associer une dominante et une
secondaire cohérentes ; cela reste le même objet avec les mêmes statistiques.

## Conséquence pour les variantes de couleurs

Le renderer actuel applique un unique dégradé de tenue à la plupart des pixels
colorés (`src/game_art_palette.rs`, fonction `color` et slot de tenue dans
`figure`). Dessiner des surfaces bleues et jaunes dans la source ne suffit donc
pas à conserver le duo quand une palette différente de l'original est appliquée.

Après validation des dessins, la recoloration devra distinguer matière
principale et matière secondaire, avec un duo déterminé par la palette et le
modèle. Les métaux, la peau, les lumières et les armes gardent leurs matériaux
distincts. Aucune modification de sauvegarde, tirage de couleur ou statistique
n'est implémentée par cette étape d'aperçu.

## Comparaisons et contrôle

- `comparatif-tenues.png` : essai précédent / bicolore, taille native et zoom x2.
- `comparatif-inspection.png` : les deux versions à 128px.
- `capture-outfits-sols.png` : huit paires de figures en rendu Macroquad séparé,
  avec les fonctions actives de sols et sang du jeu. Béton, métal, grille sombre
  et sol ensanglanté. Aucun chargement ni changement de partie.
- `build_previews.py` : crop, normalisation par le packer existant, nettoyage
  alpha et resize nearest seulement, sans peinture ou quantification de couleur.
- `mesures-propositions.json` : les huit corps font 30px, ligne des pieds à 46
  dans le cadre de 48px ; les armes peuvent dépasser la hauteur du corps.
  Les PNG ne sont pas des palettes strictement indexées. Le nombre de teintes
  est enregistré, mais n'est pas utilisé comme une note automatique de lisibilité.
- `preservation.json` : 650 fichiers actifs identiques à la référence prise au
  début de cette étape. Aucun PNG actif, code, contenu, nom, gameplay ou sauvegarde
  changé ; les essais précédents restent disponibles dans leur dossier.

La compilation de l'exécutable de comparaison et la capture native réussissent.
Pas de nouvelle suite de tests du jeu : cette étape ne modifie aucun code actif.
La correction orange et la correction de tache du pistolet de l'étape précédente
restent en place. Aucun commit, push ou déploiement.
