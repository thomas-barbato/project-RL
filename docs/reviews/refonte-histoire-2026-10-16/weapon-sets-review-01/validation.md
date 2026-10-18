# Vérification finale — deux ensembles d’armes

18 octobre 2026. Le visuel accepté est intégré au lancement normal.

- Compilation debug : réussie, sans avertissement.
- Contrôle release : réussi, sans avertissement.
- Suite complète du moteur : **977 réussites, aucun échec**.
- Vérifications ciblées du client : **34 réussites**, plus **5 tests propres aux
  ensembles**, tous réussis.
- Captures natives du rendu intégré : **16 images à 1920 × 1080**, vérifiées,
  avec inspection des deux thèmes, des deux prises et de la barre de compétences.
- Formatage et contrôle des espaces du diff : réussis.

Les tests vérifient la propriété des objets, les deux armes à une main et les
prises à deux mains, l’absence de consommation de tour lors d’une permutation,
les bonus et compétences de l’ensemble actif, le refus des attaques depuis la
réserve, les boutons et raccourcis, l’absence de clic sur l’ancien troisième
bouton, les reprises par instantané et par rejeu, ainsi que la conversion des
parties du camp de génération 153. Les contrôles de rendu ne consomment aucun
objet et ne produisent aucune sauvegarde de joueur.

## Limite de la suite générale du client

Un premier passage complet a donné **926 réussites, 64 échecs et 15 tests
ignorés**. Les 64 cas en échec ont ensuite été exécutés sur une copie isolée de
la révision précédente `83929fc1c45e3334c4e17d470e6e805204f75536` : **50 échouent
déjà sur cette révision**, tandis que 14 y réussissent. Le nom de l’exécutable
de comparaison et son argument de liaison sont les seules adaptations du
dispositif de compilation de cette copie ; son code de jeu vient de Git.

Les 14 écarts ont été corrigés ou leurs hypothèses de test adaptées aux règles
acceptées. Ils font tous partie des 34 vérifications ciblées finales réussies.
Les adaptations concernent notamment les quatre mains possibles, l’ensemble
inscrit dans l’état moteur, les armes de départ et les règles réellement
projetées dans les instantanés. Les scènes de diagnostic sélectionnent aussi
l’ensemble dans le moteur. Les générations d’avant le camp gardent leurs règles
historiques, tandis que la génération 153 est convertie à la reprise effective.

La suite complète du client n’a pas été relancée après ces corrections. Ce
bilan distingue donc les vérifications ciblées réussies des 50 échecs antérieurs
confirmés, et ne présente pas la suite générale comme entièrement verte. Les
noms et les signatures de ces échecs sont conservés dans `suite-comparison.json`.

## Révision de la seconde main — WEAPON-SETS-01-GHOST-02

Le joueur accepte l’image exacte de l’arme en semi-transparence dans la
seconde main. L’opacité de cette copie d’affichage est de 30 % ; la source,
la palette et la taille restent celles de la main principale. Le fond rouge
envisagé n’est pas retenu. Le moteur, les objets et les interactions ne
changent pas. Les cinq tests d’ensembles sont relancés et réussissent.
Les nouvelles captures et contrôles de cette révision figurent dans
`ghost-hand-checks.json` après leur vérification.

## Rangement

Le prototype ne possède plus de dessin ou d’état de permutation fictif. Son code
de dessin est repris dans le module actif ; la capture appelle l’interface et
le moteur réels. La suppression de la copie de comparaison, de son archive,
de son exécutable propre et des sorties temporaires de diagnostic a été
refusée par le contrôle automatique (« blocked by policy »). Ces fichiers
restent donc présents ; aucun nettoyage de ces éléments n’est déclaré terminé.
Les sources du jeu, dessins, sauvegardes et données de l’éditeur de cartes
restent conservés. L’aperçu courant est réutilisé.
