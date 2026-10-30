# CREATION-UI02-D — étude de l'habillage de création

30 octobre 2026. Étude demandée après l'aperçu compact 02-C. Le joueur préfère
sa disposition, mais trouve l'habillage trop simpliste et peu évocateur d'un
jeu vidéo. Ce retour conserve 02-C comme base de composition ; il ne valide
pas un nouvel habillage ni l'activation normale de cette variante.

La seule modification du jeu autorisée et effectuée pendant cette étude est
le retrait du sous-titre « Un équipement de départ et une façon d'aborder
l'expédition. ». La refonte artistique décrite ci-dessous reste une proposition.

## Diagnostic à partir des captures natives

Les captures examinées sont celles de `compact-02C/metamorphe/` en 1280 × 800
et `compact-02C/arcaniste-small/` en 960 × 540, sous l'aperçu courant. Elles
précèdent le retrait du sous-titre. Aucun nouveau rendu d'habillage n'est
présenté comme déjà intégré.

La lecture et la disposition sont devenues solides : choix de classe stable,
petites cartes, description réservée, actions dégagées. Les mesures natives
de 02-C donnent 16 fiches sans défilement en 1280 × 800, et 15 en 960 × 540 ;
seule Mue prédatrice dépasse dans le petit format. Ce sont les mesures de
la revue précédente, conservées dans `PROPOSITION-02C.json`.

Le manque de caractère vient surtout de l'habillage et de la mise en scène :

| Observation sur 02-C | Conséquence visuelle | Réponse proposée |
| --- | --- | --- |
| Même famille de rectangles arrondis et mêmes aplats presque partout | L'écran évoque un tableau de bord générique | Donner une construction différente au sélecteur, aux logements d'objets et au lecteur |
| Bordures d'un pixel et relief presque absent | Les éléments paraissent posés sur une page | Introduire un léger rebord, une ombre de contact et une lumière de tranche cohérente |
| Personnages réduits à des vignettes de liste | Le joueur choisit surtout un ensemble de textes | Présenter une seule vignette du personnage dans le monde du jeu, dans le vide central disponible |
| Grande zone centrale vide sous les attributs en 1280 × 800 | La fenêtre ne raconte pas le départ en expédition | Utiliser une partie de cette zone pour une scène native sobre |
| Fond noir bleuté uniforme, sans matériau reconnaissable | Peu de lien avec les décors industriels et usés du jeu | Employer une matière mate très légère sur les cadres et des traces d'usure localisées |
| Cyan utilisé pour le nom, les attributs et le grand bouton | Plusieurs éléments se disputent l'attention | Réserver l'accent fort au choix actif et à l'action ; différencier les chiffres des libellés |
| Bouton principal large, plat et très lumineux | Sa forme évoque un bouton de site web | Donner au bouton un logement, une tranche et un état pressé lisible |

Il s'agit d'un jugement artistique sur ces captures, pas d'une mesure objective
de qualité. La simplicité peut être une direction forte lorsqu'elle possède
sa propre grammaire ; ici les composants ont surtout la grammaire d'un outil
de gestion.

## Direction recommandée : préparation d'expédition industrielle

Conserver les trois zones de 02-C et les petites icônes fonctionnelles. Donner
à leur support l'aspect d'un équipement de terrain : métal sombre mat, cadres
à angles légèrement coupés, petits décrochements d'assemblage, lumière froide
sur les arêtes et usure concentrée sur quelques points exposés. Les surfaces
derrière les paragraphes restent calmes pour préserver le texte.

Deux matières discrètes suffisent : une coque et des logements plus creux.
La variation doit suivre une construction compréhensible, avec un même sens
de lumière et une même épaisseur de bord. Les détails doivent avoir une place
précise : fixation, séparation, contact ou état sélectionné. C'est cette
cohérence qui peut donner un résultat moins générique.

Les deux habillages existants, Classique bleu et Terminal PC, sont conservés
comme contrainte de la future maquette. Le travail de matière et de relief
doit fonctionner dans les deux ; cette étude ne crée pas un troisième thème.

## Priorités de la prochaine maquette

1. **Hiérarchie et cadres.** Sélection de classe construite comme un logement
   actif, petites cartes d'objets comme des emplacements, lecteur comme une
   fiche consultée. Quelques angles coupés et une ombre de contact suffisent
   à différencier ces rôles. La taille du texte de description reste 14 px.
2. **Un point d'ancrage dans le jeu.** Dans le vide central du grand format,
   essayer une scène d'environ 192 × 192 px, soit trois cases de 64 px : le
   véritable personnage, un sol et un élément du camp déjà acceptés. Le joueur
   voit ainsi ce qu'il s'apprête à incarner. Les petites icônes restent petites.
   La scène utilise uniquement l'espace libre ; elle se masque quand cet
   espace manque et n'impose aucun nouveau défilement.
3. **Matière et lumière.** Habillage mat, creux légèrement plus sombres,
   arêtes mieux définies et grain presque imperceptible. Une poignée de traces
   d'usure choisies vaut mieux qu'un bruit uniforme sur toute la fenêtre.
4. **Identité des classes.** S'appuyer sur leurs silhouettes, équipements et
   compétences de départ existants : défense pour Bastion, tir pour Traqueuse,
   Braise pour Arcaniste, transformation pour Métamorphe. Le personnage et
   l'icône choisie portent cette différence ; les règles restent identiques.
5. **Typographie et action.** Rendre les nombres d'attribut plus visibles que
   leurs libellés et mieux rythmer les titres. Donner une assise au bouton
   « Commencer la partie » et un état pressé net, avec une place plus discrète
   pour « Retour ». Réemployer les fondus courts de 02-B et respecter le
   réglage de mouvement réduit.

Le retrait du sous-titre permettra également d'essayer un bandeau d'en-tête
moins haut dans cette maquette. La suppression effectuée aujourd'hui ne
déplace pas les contrôles : ce nouveau rythme vertical reste à montrer.

## Références de principe

- [Documentation officielle de création d'Underrail](https://www.stygiansoftware.com/wiki/index.php?title=Character_creation) :
  la sélection d'un portrait fait partie de la création, et la documentation
  décrit plusieurs tailles de portrait ainsi qu'un fond dynamique pour les
  portraits transparents. À retenir pour cette étude : le personnage peut
  être une composante visuelle de la création, au-delà des statistiques.
- [Présentation officielle de Citizen Sleeper par son éditeur](https://www.fellowtraveller.games/citizen-sleeper) :
  la page présente distinctement personnages, classes et iconographie de jeu.
  Ces catégories servent de référence documentaire pour étudier une identité
  cohérente entre personnage et interface.

Ces rapprochements sont des pistes de conception, pas une analyse exhaustive
de ces deux interfaces. Leurs ressources ne sont pas copiées. La tentative
d'ouverture du GIF de classes de Citizen Sleeper a été refusée par le lecteur
web ; aucune observation détaillée de ce GIF n'est revendiquée.

## Cadre technique et critères de revue

La future vignette respecte VIS-COH01-RULE01 : sources acceptées, projection
à axes parallèles, personnage et décor comparés à taille native64. Elle ne
doit pas introduire un nouvel angle de caméra, un agrandissement du sprite
ou une nouvelle pose sans revue. Aucun portrait supplémentaire n'est requis
pour le premier essai.

Appliquer LOAD01-RULE01 aux ressources de présentation : préparation avant
l'ouverture, réutilisation des quatre profils proches et d'un décor commun,
cache borné, aucune lecture ou décompression lourde dans le dessin.

La maquette numérotée suivante devra être revue en 1280 × 800 et 960 × 540,
avec Mue prédatrice comme fiche longue, dans les deux habillages et avec
mouvement réduit. Les clics, le clavier, les noms, les valeurs, la séparation
des effets actuels et du prochain rang, et les sauvegardes restent les mêmes.
Le nombre de fiches sans défilement doit au moins conserver le résultat de
02-C. Une comparaison avec les décors et acteurs du jeu doit accompagner
l'habillage proposé avant son activation normale.

Cette étude précède la maquette native 02-D. Après la demande du joueur
« oui je veux bien :) », elle est réalisée en aperçu. Le premier rendu plaît,
mais ses traits sont jugés trop épais ; la révision r02 allège les contours.
État courant et contrôles : `PROPOSITION-02D.json` et `REVUE-02D.md`.
Après validation de r03, l'habillage est intégré au lancement normal ; voir
`acceptance-02D-r03.json` et `integration-02D-r03.json`.
