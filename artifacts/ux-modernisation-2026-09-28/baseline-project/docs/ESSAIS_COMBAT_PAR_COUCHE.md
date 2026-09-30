# Essais de combat par couche

27 septembre 2026 · règles de génération 113 conservées.

Suite de la [première passe d'équilibrage](EQUILIBRAGE_PAR_COUCHE.md).
Cette étape ajoute des mesures reproductibles et des garde-fous de test.
Elle ne modifie ni les statistiques du jeu ni les sauvegardes.

## Ce qui est mesuré

Quatre rencontres déjà approuvées, dans leurs profondeurs autorisées : Ver
cuirassé, Anémone des caves, Hurleur des failles et Sentinelle. Le Ver est
testé séparément en maintenance et en production, avec leurs vrais profils.
Les spécimens viennent des générateurs du catalogue, puis reçoivent le même
ajustement par couche que les populations du jeu.

Pour chaque couple biome/profondeur : 16 graines de variation individuelle,
un couteau et un fusil **sans affixes**, ainsi qu'une veste du même palier.
Cela représente 480 configurations. Le palier est profondeur + 1, plafonné
à 6, comme le catalogue actuel. Ce choix ne garantit pas que le joueur aura
effectivement trouvé ces trois objets à cet instant de sa partie.

Les primaires et les PV du personnage restent ceux du profil de départ,
soit 20 PV dans ces essais. Aucune compétence offensive, potion, progression
de primaires ou amélioration magique n'est supposée acquise.

Le calcul traverse les règles réelles d'impact physique, d'armure, de
pénétration et de résistances. Chaque estimation des dégâts sortants est
comparée à la perte de PV produite par une véritable commande d'attaque.
Les coups ratés et critiques sont désactivés uniquement dans ces fixtures.

**Les nombres ci-dessous sont des coups ordinaires réussis, pas des tours
de combat ni des probabilités de victoire.** Rechargements, mouvements,
récupération, précision, esquive, effets spéciaux, terrain et ennemis voisins
peuvent allonger ou transformer un affrontement. Le sol de mesure est sec ;
l'habitat sert à obtenir le vrai spécimen, pas à simuler toute sa région.

## Résultats

| Rencontre | Couche | PV | Coups au couteau | Coups au fusil | Dégâts reçus avec la veste |
|---|---|---|---|---|---|
| Ver cuirassé | 1 | 19–20 | 4 | 5 | 3 |
| Ver cuirassé | 2 | 24–25 | 4–5 | 4–5 | 2–3 |
| Ver cuirassé, production | 3 | 30–31 | 5 | 5 | 2 |
| Anémone des caves | 2 | 28–29 | 4–5 | 5 | 1 |
| Anémone des caves | 3 | 33–35 | 5 | 5 | 0 |
| Anémone des caves | 4 | 40–42 | 5–6 | 5–6 | 0 |
| Hurleur des failles | 2 | 37–38 | 7 | 7 | 3 |
| Hurleur des failles | 3 | 43–45 | 7 | 7 | 2–3 |
| Hurleur des failles | 4 | 51–54 | 7 | 7 | 2–3 |
| Sentinelle | 3 | 39–40 | 7 | 7 | 7 |
| Sentinelle | 4 | 45–47 | 7 | 6 | 8 |
| Sentinelle | 5 | 52–55 | 6–7 | 6–7 | 9 |
| Sentinelle | 6 | 58–61 | 7 | 7 | 10–11 |

Les valeurs du Ver en production 1–2 sont identiques à celles de maintenance
1–2, mais les deux profils ont été testés. Les fourchettes sont celles des
16 graines de cette arène ; elles ne prétendent pas couvrir tous les tirages.

## Lecture gameplay

- Les armes ordinaires adaptées permettent de vaincre ces rencontres sans
  dépendre d'un affixe rare. Dans cette mesure isolée, la montée des PV ne
  provoque pas une augmentation continue du nombre de coups nécessaires.
- Le Hurleur et la Sentinelle restent plus résistants que le Ver. Leurs
  préparations et récupérations sont importantes pour créer des ouvertures.
- Les vestes absorbent une part croissante des attaques physiques. L'Anémone
  peut ne plus enlever de PV, mais un coup réussi applique toujours sa prise :
  elle reste un obstacle de déplacement, notamment près d'autres dangers.
  Ce comportement était déjà prévu ; aucun dégât minimum artificiel n'est ajouté.
- L'armure physique ne répond pas au tir électrique de la Sentinelle. En
  couche 6, deux tirs ordinaires peuvent tuer le profil initial à 20 PV.
  Ce n'est pas une mesure du personnage réellement arrivé à cette profondeur,
  mais c'est un point de vigilance pour la progression et les résistances.
- Une arme à distance peut nécessiter un rechargement avant les sept touches :
  la colonne « coups » n'efface pas sa capacité de chargeur ni son délai de tir.

Il n'y a donc pas de raison suffisante, sur ces seuls résultats, d'augmenter
globalement les dégâts ou les PV. Les coefficients 113 sont conservés.

## Contre-jeu contrôlé

Un deuxième test laisse réellement frapper une Anémone de couche 4 et une
Sentinelle de couche 6 : dégâts absorbés mais entrave appliquée pour la
première ; dégâts électriques conservés malgré la veste pour la seconde.

Un troisième test couvre quatre graines à la profondeur maximale de chacun
des quatre profils. Le joueur commence dans la zone annoncée avec son
équipement ordinaire. Il se déplace, l'ennemi libère réellement son attaque
et entre en récupération : le joueur reste indemne. Ce contrôle passe par
les cellules utilisées par l'affichage des avertissements, et non par une
copie de leur géométrie. Il couvre une arène ouverte, pas tous les obstacles
possibles d'une région ni une situation où le joueur serait déjà entravé.

Garde-fous larges : chaque arme de référence doit pouvoir blesser l'ennemi,
le budget ne doit pas dépasser dix touches normales et un seul impact
ordinaire ne doit pas tuer le corps initial. Ce ne sont ni des limites
universelles pour les futurs monstres ni un équilibrage définitif.

## Contraste des rencontres de surface

Complément du 27 septembre après validation des repères de rythme du bestiaire.
Deux tests supplémentaires mesurent la surface et le bénéfice d'une meilleure
arme. Aucun coefficient de génération n'a été changé.

Huit graines par espèce, couteau de camp ou fusil de patrouille sans affixes,
profil initial et veste matelassée : 80 configurations. Les meutes sont générées
avec leur minimum réel de deux individus ; un membre est isolé pour la mesure.
Le comportement du groupe entier n'est pas évalué par ce test.

| Rencontre | PV effectifs | Touches au couteau | Touches au fusil |
|---|---|---|---|
| Grignoteur, neutre | 1 | 1 | 1 |
| Mordeur des friches | 1 | 1 | 1 |
| Fouisseur pâle | 8 | 3 | 2 |
| Brise-os | 17–18 | 6 | 5 |

Les faibles PV du Grignoteur et du Mordeur résultent du calcul corporel existant,
notamment de leur Résilience inférieure au neutre et du minimum de 1 PV.
Ce complément n'a pas réduit leurs PV. Ils restent particulièrement vulnérables
aux dégâts de zone : le danger collectif d'une meute reste à éprouver.

Le Dos-rond est également testé : 8 PV, mais il se replie après le premier coup
et gagne de l'armure. Le quotient obtenu avant ce repli (4 touches au couteau,
3 au fusil) **n'est pas une estimation du combat complet**. Le test vérifie le
déclenchement réel de sa protection. Les deux espèces neutres conservent leur
relation initiale, leur IA et l'absence de récompense de défaite.

Sur le même Brise-os, avec le même personnage et la même graine, remplacer le
couteau de camp par un couteau de sapeur fait passer le budget de **6 à 3 touches**.
Le test compare les acteurs ennemis après leur apparition : ils sont identiques.
Il vérifie ensuite les dégâts réels des deux armes. Cela démontre le bénéfice
d'une meilleure base d'arme, pas une promesse sur tous les affixes possibles.

Les tests fixent des budgets propres aux rôles actuels, avec une marge pour
les différences d'armes. Ils n'obligent pas une arme supérieure à respecter
un minimum de touches. Les profils souterrains existants restent testés dans
la matrice précédente ; des petites rencontres profondes demandent encore
la revue puis l'intégration d'autres fiches du catalogue.

## Rejouer les essais

```powershell
cargo test --locked --bin project-rl combat_balance_ -- --nocapture
```

La matrice profonde et le test de surface impriment leurs mesures. Les autres
vérifient le rôle de l'armure, les sorties des attaques annoncées et le bénéfice
d'une meilleure arme. Tout est local aux fixtures,
sans lire ni modifier une partie du joueur. Le module n'est compilé que pour
les tests ; aucun nouveau menu ni réglage de campagne n'est ajouté.

Validation du complément : les **cinq tests `combat_balance_` passent**, dont
les deux nouveaux contrôles de surface et d'amélioration d'arme. Le contrôle
de tous les targets, le formatage et `git diff --check` passent également.
Les trois tests `layer_balance_` avaient été validés lors de la première passe
de mesures. La suite complète n'a pas été relancée pour ce complément limité
aux fixtures de test ; ses anciens totaux ne sont pas une nouvelle exécution.
Aucun commit ni push effectué.

La suite logique reste un essai de progression et de rencontres combinées :
équipements réellement obtenus, munitions, soins, compétences, espace pour
reculer et présence simultanée de plusieurs ennemis. Ces mesures préparent
cet essai, elles ne le remplacent pas.

Une [première passe de rencontres combinées](RENCONTRES_COMBINEES.md) couvre
désormais trois groupes avec leurs profils réels : retraite, repli commun,
couvert, arrivée protégée et restauration des préparations simultanées.
L'essai de progression avec équipement obtenu et ressources finies reste à faire.
