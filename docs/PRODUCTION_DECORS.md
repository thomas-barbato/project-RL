# Fabrication des décors

**DECORS-PRODUCTION-01 — décision du joueur du 19 octobre 2026.**

Cette méthode s'applique à tous les éléments visuels des décors, à mesure de
leur création ou de leur reprise. Elle complète
[STYLE_VISUEL_REFERENCE.md](STYLE_VISUEL_REFERENCE.md) et
[DECORS_CAMPAGNE.md](DECORS_CAMPAGNE.md). Elle ne valide pas les dessins proposés.

## Dessiner les ressources avant de composer la carte

Une illustration sert à choisir la composition et l'ambiance. Elle ne doit
plus être découpée en polygones opaques pour fabriquer des accessoires ou des
bâtiments : ces sélections emportent du sol, des voisins et des contours coupés.

Chaque objet est dessiné complet et isolé, avec de vraies marges transparentes,
puis exporté dans son propre fichier. Les sols sont dessinés comme des surfaces
raccordables. Un atlas destiné au rendu peut regrouper ensuite des ressources
propres ; ce regroupement technique ne remplace pas leur fabrication séparée.

Les ressources acceptées et déjà propres peuvent être réutilisées. Une reprise
ne doit pas imposer une nouvelle apparence à tout le camp ou effacer une
validation antérieure. Les anciennes méthodes restent archivées comme telles.

## Contrat commun

- Même projection orthogonale et faible hauteur apparente que le camp accepté.
- Échelle de 64 px par case ; dimensions physiques définies avant le dessin.
- Contours nets et matériaux simples, lisibles avec les acteurs à taille réelle.
- Murs droits, coins, jonctions et extrémités prévus ensemble, avec mêmes épaisseurs.
- Portes, toits amovibles et objets mobiles séparés des éléments fixes concernés.
- Lentilles et boîtiers dessinés ; émission, halos et éclairage calculés par le moteur.
- Ombres de contact distinctes lorsque nécessaires ; aucun morceau de sol peint
  autour d'un accessoire pour simuler son intégration.
- Dégâts architecturaux dessinés dans la variante du bâtiment concerné : lignes
  interrompues et structure exposée, sans motif générique posé sur un toit intact.
- Source, identifiant, taille native, point d'ancrage et couche enregistrés.
  L'emprise de collision reste une donnée distincte de la boîte de l'image.

## Contrôle du lot

Examiner chaque ressource sur fond clair et sombre, puis sur le sol du jeu.
Vérifier qu'aucune silhouette n'est coupée par un bord d'export et qu'aucun
rectangle de fond, halo peint ou fragment voisin ne demeure. Vérifier les
raccords avec les pièces adjacentes, sans étirer les éléments pour cacher une erreur.

Contrôler aussi les **limites internes de la silhouette**, notamment les angles
rentrants des bâtiments en L et les abords des façades. Une grande marge
transparente ne prouve pas qu'une caisse ou un boîtier est entier : la coupe
peut déjà être dessinée dans la source. Les objets extérieurs doivent être
retirés complètement du bâtiment, puis fabriqués et placés séparément avec
leur base, leurs côtés et leur contour complets. Examiner également les
intérieurs réutilisés ; leur ancien statut ne dispense pas de ce contrôle.

Vérifier les portes dans leurs deux états dans le moteur. Un ancien luminaire
peint ne doit pas laisser de fragment au bord du masque d'ouverture lorsque
la lampe fonctionnelle est déjà placée sur le mur voisin.

Composer ensuite la scène depuis ces ressources. La composition statique peut
être regroupée pour le rendu, mais ses sources, placements et couches doivent
rester disponibles. Contrôler les seuils, les collisions, les passages, le
masquage des toits et le montage physique des luminaires dans le moteur.

Examiner aussi la grille logique sur les intérieurs dessinés. L'alpha
opaque du sol et du mobilier d'un bâtiment complet ne représente pas ses
murs. Vérifier séparément la circulation et les obstacles de vue avec les
deux propriétés déjà distinctes du moteur. Présenter les éventuelles
corrections de perception avant leur activation.

Présenter le lot numéroté et la scène à taille native avant activation des
nouveaux pixels, conformément à AGENTS.md. Les tests de circulation ne valent
pas acceptation artistique. Une anomalie doit être corrigée dans la ressource
ou le placement concerné, sans la cacher sous des gravats ou une autre image.
