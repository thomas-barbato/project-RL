# WALK07 — équipement et morphologies

Le joueur demande les autres armes puis les monstres, avec leurs anatomies
réelles, y compris les espèces à plus de deux pattes. Le lot WALK06 intégré
reste conservé. Les nouvelles déclinaisons sont des propositions avant revue.

Pour les armes, fabriquer des cycles entiers sans arme, puis séparer les bras
par propriété de pixels, avec vérification de recomposition exacte. Le corps
entier dessiné fournit déjà toutes les poses de jambes : aucun cisaillement,
déplacement de bandes ou greffe de bas de corps n'est autorisé. Les armes
existantes et leurs palettes restent des objets rigides liés aux prises.
Le rendu normal ne doit pas activer les anciennes animations WALK04.

Un premier atlas groupant quatre tenues a été écarté : il mélangeait deux
niveaux de protection sur une ligne. Source de diagnostic non acceptée :
`C:/Users/User/.codex/generated_images/01a16812-c028-73a3-9d94-d6e3343cb79c/exec-f0058746-a7f6-4546-a90a-02692e470f31.png`.
Les nouveaux dessins utilisent donc une seule identité par atlas, comme WALK06.

Pour le bestiaire, contrôler les supports dans les dessins sources. Les catégories
historiques `quad` et `crawl` donnent un indice et ne valident pas un nombre
de pattes. Conserver les ailes, pinces, tentacules, châssis et formes disjointes.
Une créature sans pieds reçoit un mouvement propre à sa matière et sa forme.
Conserver400ms réglables, trois phases par case, vraie pose de repos à l'arrivée,
et le rendu vertical existant. Aucune règle de jeu ou identité ne change.
