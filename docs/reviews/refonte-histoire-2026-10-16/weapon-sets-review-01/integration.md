# Intégration de WEAPON-SETS-01

Le visuel est accepté le 18 octobre 2026. Le joueur valide séparément les
bonus et compétences accordées par le seul ensemble actif ; voir `decision.json`.

Le jeu possède quatre emplacements logiques, regroupés en deux paires libres.
Une arme à deux mains est conservée dans un seul emplacement réel et réserve
virtuellement la main voisine. Les autres objets restent dans le sac lorsqu’ils
sont remplacés. Les deux paires peuvent recevoir une arme à deux mains, deux
armes à une main, une seule arme ou aucune.

La révision acceptée WEAPON-SETS-01-GHOST-02 remplace le symbole de liaison et
« Deux mains » dans la seconde case par l’image exacte de l’arme, dessinée à
30 % d’opacité. La première case conserve son image opaque. Le dessin, la
palette, l’échelle et l’orientation restent identiques ; aucun deuxième objet
n’est créé. Cette indication visuelle ne change pas les interactions.

Le bouton de l’inventaire et celui de la barre commandent le même état moteur.
Les raccourcis des deux mains agissent dans l’ensemble visible ; l’ancien
troisième raccourci d’arme permute les ensembles. Les affectations configurées
restent conservées. La permutation n’utilise aucun tour ; équiper garde son coût
actuel. Les attaques et compétences d’arme ne cherchent pas dans la réserve.

Les bonus numériques, ressources et compétences accordées suivent l’ensemble
actif. Les compétences simplement attribuées à la barre restent mémorisées
lorsque leur source passe en réserve, sans devenir utilisables ni apprises.
La permutation n’accorde aucun soin ou remplissage de ressource.

Les nouvelles parties utilisent la génération 154. À la reprise effective d’une
partie du camp de cryogénie possédant les trois anciens emplacements de mains, une commande de
conversion est enregistrée après vérification de l’ancien état. Les deux armes
à une main forment I et l’ancienne arme à deux mains forme II ; l’ensemble choisi
est conservé. La génération du monde reste historique. La commande de conversion
permet le rejeu sans dépendre uniquement d’un cache de sauvegarde.

Les instantanés des parties à deux ensembles utilisent RLWS 16 avec un octet
d’ensemble actif à la fin de l’état de jeu. La lecture et l’encodage des états
historiques restent disponibles ; les anciens états sans ensembles omettent cet
octet et conservent RLWS 15. La conversion précède le point de récupération
durable et la consommation de la suspension dans la procédure de reprise.

Les générations antérieures au camp gardent leur configuration historique.

Le code de dessin du prototype a été remplacé par `src/weapon_sets_app.rs`.
Le mode de capture utilise désormais l’équipement réel. Aucune ressource du
jeu ne dépend des PNG ou de la page temporaire sous `artifacts/`.

Les contrôles exécutés sont conservés dans `integration-checks.json` une fois
les vérifications terminées.
