# SYMBOLES-r03 — retrait ciblé des trois barres

Le 5 novembre 2026, le joueur refuse les trois barres de signal répétées
sur le lit, le réfrigérateur et l'évier, jugées sans fonction cohérente.
L'appareil posé sur la table porte le même pictogramme ; il est retiré aussi.
Les cadres, commandes physiques, voyants existants et matériaux sont conservés.

Les quatre champs d'écran sont repris depuis une édition **imagegen intégrée**.
Seuls quatre petits rectangles de l'image native sont appliqués : évier
`[237,106,248,113]`, frigo `[328,83,340,91]`, lit `[450,72,462,81]`,
appareil de table `[222,205,233,212]`. Ils appartiennent à trois couches
indépendantes : `V1-I01-logement-D03-cuisine`, `D05-lit`, `D06-table`.
Les vingt autres couches du lot 02 conservent leurs fichiers et empreintes.

Le fichier complet généré sert uniquement de source de ces petits champs.
Ses autres pixels ne sont pas appliqués. Le logement réel conserve exactement
ses autres pixels, ses murs, son sol, ses raccords et son implantation.
Le petit écran mural de l'étude, au-dessus du frigo, est aussi nettoyé dans
la source ; il était hors du masque appliqué et n'était pas ajouté à la carte.

Source : `assets/source-art/city-cyberpunk12/symboles-r03/V1-I01-sans-barres-source-r03.png`.
Prompt exact : `SYMBOLES-r03-PROMPT.txt`. Provenance, rectangles, empreintes
et sources antérieures : `assets/source-art/city-cyberpunk12/symboles-r03/manifest.json`.
Les fichiers r01 inspectés du lot 02 restent intacts ; les trois couches
modifiées sont des fichiers voisins r03. La retouche de sécurité r02 reste conservée.

La seule capture native renouvelée est celle de l'intérieur du logement,
avec le personnage original 64 × 64. Les autres vues de l'aperçu sont
conservées comme contexte, avec leurs comparatifs précédents. Aucun fichier
actif de carte, de collision, d'égouts ou de sauvegarde n'est modifié.
La préparation des ressources et le cache commun restent utilisés.

Le joueur prévalide l’aperçu r03 le 5 novembre 2026 et trouve le lit un peu
grand. Son accord est enregistré dans `SYMBOLES-ACCEPTANCE-r03.json`.
La variante de taille est présentée dans `LIT-r04.md`.
