# LATERAL02-r02 — marche ciblée de la Traqueuse

Le 2 novembre 2026, le joueur juge la marche peu convaincante, précise
« je parle pas de tous les personnages :) mais d'au moins celle de la
traqueuse :) » et indique « le bastion est ok ».

Cette reprise concerne uniquement la Traqueuse en armure lourde montrée dans
LATERAL02 : corps 5, armure 101, profil `traqueuse-armor3`, tenue 13.
Le retour sur le Bastion conserve son animation actuelle ; il ne valide pas
un nouveau dessin, une autre armure ou un autre lot d’équipement.

## Changement proposé

Le cycle 01 comporte plusieurs variantes de la même jambe en avant. Le nombre
de PNG distincts ne prouve pas deux pas anatomiquement alternés. Le candidat 02
reprend un contact de la jambe proche, le passage puis l’arrivée de la jambe
éloignée, son contact, puis le passage et l’arrivée de la jambe proche.
Une vraie pose de repos reste séparée. Gauche et droite ont chacune six phases.

Les premières générations ne respectaient pas suffisamment le changement
d’appui et ne sont pas utilisées comme cycle final. Des guides de poses
explicites et trois feuilles de **figures entières** produites via l’outil
intégré `image_gen` fournissent les poses retenues. Les membres ne sont pas
découpés, greffés, cisaillés ou déplacés indépendamment. Chaque feuille dispose
d’un calibrage commun de résolution ; les figures entières sont ensuite
alignées et échantillonnées au plus proche à l’échelle de référence de 58 px
de hauteur opaque, sur une grille de jeu de 64 px.

Les sources, guides, atlas, correspondances de poses et consignes sont dans
`assets/source-art/animation-polish-01/lateral-identity-02/r02/`,
`conditioning-r02.json` et `LATERAL02-PROMPT-r02.md`. Les références VERT18
face/dos restent inchangées. Ce candidat est sans arme : aucune prise de fusil
ni compatibilité avec un autre équipement n’est validée ici.

## Comparaison et contrôles

- Compilation Debug verrouillée réussie. Nouvelle capture isolée
  `--ui-cold-lateral-gait` ; l’ancienne capture r01 reste disponible.
- 14 planches moteur : face validée, candidat précédent 01, nouvel essai 02,
  dos validé, dans le camp et sous LIGHT01 Discret. Six images distinctes par
  côté ; références fixes inchangées. Ressources préparées avant le dessin.
- Aperçu natif, avec répartition en deux rangées sur panneau étroit et zoom
  entier ×4 facultatif. La publication ciblée conserve les captures des autres
  revues. Marche continue par défaut, deux pas de 400 ms ; une option distincte
  montre aussi l’arrêt, sans ajouter de délai au jeu.
- Syntaxes Python et JavaScript vérifiées. Contrôles ciblés de format Rust et
  de diff. Inspection des contacts A/B au navigateur, lecture des deux directions,
  pause et mode avec repos vérifiés ; aucune erreur de console relevée.

Ces vérifications ne sont pas un accord esthétique : le rythme, la cohérence
des proportions et des détails entre images restent à juger à taille native.

## État

`awaiting_visual_review`, aucun nouveau dessin intégré au rendu normal.
Le Bastion, les autres classes, le déplacement vertical et les paramètres
des armes sont conservés. Pas de changement de règles, sauvegardes ou préférences.
Selon `AGENTS.md` et le skill de revue de marche, les nouveaux candidats
numérotés sont présentés à taille native avant intégration. L’accord explicite
reste nécessaire pour ce nouveau cycle ; la préparation ne vaut pas intégration.

Aperçu : `animation-polish-01/apercu.html?revision=LATERAL02-r02#identite-laterale`.
