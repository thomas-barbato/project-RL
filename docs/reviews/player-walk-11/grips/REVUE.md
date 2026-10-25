# WALK11-GRIP-r03 · Prises des armes

Correction technique validée et autorisée pour le jeu normal le25 octobre2026.
L'accord et les empreintes sont conservés dans ../acceptance.json.

Le masque protégeant les jambes rendait des pixels de gant au corps. Les bras
se déplaçaient ensuite vers les prises tandis que ces morceaux restaient près des
cuisses. La séparation tient maintenant compte des mains complètes, des contours
visibles et des petites pièces de manche isolées. Un point de raccord spécifique
conserve la plaque de cuisse du Bastion, tenue3, gauche, phase3, dans le corps.
R03 protège aussi celle de la tenue5, gauche, dans ses sept poses : elle faussait
le repère de la main et laissait un fragment orange près de la prise.
Les mains restent à leur taille native et rejoignent les prises sans s'allonger.
Les armes à une main suivent l'orientation gauche/droite.

Les350 dessins sans arme sont recomposés à l'identique, avec700 mains présentes.
Les sources dessinées, armures, armes partagées et règles de déplacement ne sont
pas redessinées. Les images de corps sont séparées en couches avec leurs propres
pixels ; aucun nouveau membre ni morceau de jambe n'est ajouté.

Le contrôle de contact couvre2690 composites : les25 tenues avec la hache67 sur
leurs14 poses ; les25 tenues avec une paire d'armes sur4 poses ; tous les autres
modèles d'arme sur les cinq premières tenues de classe, dans quatre poses.
Le contrôle r03 exige que chaque îlot de main d'au moins deux pixels à alpha96
recouvre un pixel d'arme à alpha64, sans espace. La tolérance d'un pixel diagonal
de r02 est supprimée.
Le contrôle final passe :2690 compositions, aucun échec. Le test de miroir et
de quart de tour conserve exactement le pixel de prise demandé. Le test de portée
confirme aussi le retour au rendu normal après fermeture de la revue.
Ce contrôle est complété par la revue visuelle native ; il ne remplace pas
l'examen artistique ni un produit exhaustif de toutes les combinaisons.

Les premiers essais de découpe et leurs échecs sont conservés comme diagnostics,
notamment contact-first.json et after/contact.json (r02). La mesure courante est
r03/after/contact.json.

R03 corrige les centres de pixels des miroirs, rotations et repères de prise
provenant du rendu de face. La prise de la hache67 suit le centre du manche fin,
plutôt que son contour inférieur. Les deux pistolets s'orientent correctement,
même lorsque le dessin de face de la main gauche était déjà tourné à gauche.

Les56 petits îlots inspectés près des poignets ont été rattachés à leurs bras
(298 pixels avec leurs contours transparents). Les masques rigides de main
conservent les petits morceaux de gant, plutôt que leur seul îlot principal.
La main est dessinée une seule fois au-dessus de l'arme : le double passage
précédent rendait ses contours translucides plus visibles. Le packeur vérifie
la recomposition RGBA exacte des350 figures et la protection des jambes.
