# Marche validée — WALK12 et arrivée de face

Accord du25 octobre2026, ressources exactes : `acceptance.json`.

Chaque déplacement logique couvre une case de64px. L'animation dure400ms
par défaut, avec le choix existant de120/200/400ms. Un pas joue trois phases
et le déplacement suivant les trois autres, pour alterner les appuis. La pause
de lecture des anciennes démonstrations n'ajoute aucun délai en jeu.

À l'arrivée, **tout acteur** — joueur, monstre ou PNJ — reprend son véritable
dessin statique habituel avec son équipement. Le lecteur ne conserve ni le
dernier pas ni un repos de trois quarts. La direction latérale mémorisée pour
le prochain mouvement ne constitue pas une orientation affichée au repos.

Les25 tenues du joueur utilisent les300 figures entières validées WALK12.
Les jambes gardent leurs protections. Un même équipement conserve la même
prise des bras, mains et objets pendant les six phases, avec seulement la
petite translation entière du torse. Les armes restent des dessins communs
indépendants. Leur géométrie, leurs matières et les points de prise sont conservés.
L'échelle XY est uniforme pour l'ensemble des douze figures d'une tenue.

Les casques équipés suivent la tête réellement dessinée dans chaque phase,
dans les deux orientations. Le centre du torse et la hauteur de tête de la vue
frontale ne servent pas de points d'attache pendant la marche. Le placement
est calculé avant d'ajouter les autres couches portées ; changer d'arme ne doit
pas déplacer le casque. La coque s'oriente vers le côté de marche et suit
l'inclinaison de la tête de chaque phase. Les ouvertures laissent apparaître
le visage déjà dessiné dans cette orientation, sans lui substituer un visage
frontal. Le retour de face reprend le composite frontal habituel.
Correction et contrôles des25 tenues et six casques : `helmet-attachment-01/`.

Un nouveau dessin reste une figure complète et cohérente, avec tête, torse,
genoux et pieds orientés ensemble. Aucune greffe de jambe, aucun cisaillement
ni déplacement de bandes ne remplace une nouvelle phase dessinée. Les nouvelles
extensions se comparent à taille native64 et conservent leur revue explicite.

Le dessin vertical du joueur reste celui déjà accepté. Les monstres emploient
leurs cycles propres à leur morphologie ; les quadrupèdes, êtres à nombreuses
pattes et formes sans jambes ne reçoivent pas la démarche humaine. Les variantes
de couleur conservent leurs identités et leur emprise. Les petits monstres
restent centrés verticalement. La banque rejetée WALK04 et les neuf propositions
de formes/drones WALK08 ne sont pas activées par cet accord.

## Bras et prises — ARM14, 27 octobre 2026

Après comparaison du rendu actuel et du candidat ARM14, le joueur autorise sa
généralisation à tous les personnages. Les bras stables d'un équipement doivent
provenir du **même cycle et de la même projection que le corps**. Ne pas plaquer
des manches ou des gants d'une ancienne pose de repos plus volumineuse sur un
corps de marche plus fin. La largeur des manches ne se corrige pas en étirant
leur portée jusqu'à l'arme.

Pour WALK12, choisir une pose compacte complète parmi les six poses de la tenue
et de la direction, puis garder ses deux bras, ses gants et leurs repères pendant
le cycle. Les vrais dessins d'armes restent indépendants ; leurs prises suivent
ces repères. Le petit déplacement du torse translate l'assemblage entier.
Les six dessins du corps et des jambes restent inchangés. Le retour de face,
le mouvement vertical et les casques conservent leurs règles acceptées.

Accord et contrôles : `../walk-arm-projection-14/acceptance.json` et
`../walk-arm-projection-14/integration-validation.json`.
