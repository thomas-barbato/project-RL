# Stabilité avant les premières cartes — 18 octobre 2026

## Résultat et périmètre

Les cinquante échecs d’application antérieurs à la refonte des ensembles ont
été reproduits puis classés. **Vingt-neuf réussissent maintenant ; vingt et un
restent en échec et sont nommés dans `triage.json`.** La préparation du plan et
des décors continue ; aucune nouvelle carte de campagne n’est activée.

La suite moteur complète réussit : **977 tests, zéro échec**. La suite client
complète, exécutée après les principales corrections, donne **967 réussites,
24 échecs et 15 ignorés**. Après les derniers ajustements de fixtures, quarante
cas ciblés donnent **19 réussites et 21 échecs** : ils comprennent les 24 échecs
précédents, les tests du camp actuel, les ensembles d’armes et les cinq tests
de reprise historique. La suite complète n’a pas été relancée après ces derniers
ajustements ; ces nombres ne sont pas présentés comme une suite entièrement verte.

## Défauts de compatibilité corrigés

**Empreintes historiques des cartes.** L’ajout d’une table d’obstacles temporaires,
bien qu’exclue de la sérialisation, avait ajouté `obstructions: {}` au Debug de
toutes les cartes. Les empreintes de plusieurs générations anciennes dérivent
du Debug complet de l’état. Une table vide rétablit désormais la représentation
historique. Une table non vide reste dans l’empreinte.

Pour les sauvegardes produites pendant la période intermédiaire, le chargeur
compare aussi la représentation complète qui contenait cette table vide.
Le drapeau de formatage est local au thread et rétabli par garde ; il ne touche
aucun acteur, obstacle, règle, tirage ou tour. Le chargeur accepte uniquement
l’une des deux empreintes complètes exactes. Les erreurs et refus d’intégrité
restent présents. Aucun nombre attendu dans les fixtures n’a été remplacé.

**Ancien cache agent v14.** Le test reproduisant l’ancien moteur doit reconstruire
les obstacles statiques du camp comme le client réel. Le remplacement vérifié
de son catalogue par les nouvelles définitions doit aussi les réinstaller après
le round-trip moteur, car ces obstacles ne sont pas sérialisés. La reprise,
la conservation des octets moteur et les refus de règles ou d’état falsifiés
sont testés. Un test supplémentaire vérifie l’encodage intermédiaire, par cache
et par rejeu, et son refus lorsque l’empreinte est falsifiée.

## Fixtures et propositions historiques

Douze tests de l’ancienne revue graphique de souffle cherchaient un paramètre
supprimé du catalogue actuel après le passage au couloir. Cette revue isolée
conserve sa portée historique de six cases, calculée au rang onze d’après le
catalogue de la révision `40a6ee38`, au lieu de lire le catalogue vivant. Le
Souffle actuel et ses règles acceptées restent inchangés.

Les instances créées par les tests ont désormais une adresse temporaire propre,
afin que la présence d’une vraie partie dans le profil utilisateur ne bloque
pas une création de personnage et que les tests ne dépendent pas de ce profil.
Les adresses du jeu normal restent identiques. Les fixtures d’armes à une main
conservent explicitement le format Debug intermédiaire dans lequel leurs fichiers
de référence ont été enregistrés ; leurs valeurs chiffrées restent intactes.

## Échecs encore ouverts

Le registre distingue notamment : quatre tests d’attaque avec l’ancien raccourci,
un cas mêlant l’action contextuelle et une attaque explicite sans munitions,
un scénario guidé qui suit encore la vieille ville, un ancien retour entre
couches désormais irréversibles, un pilote de survie qui peut boucler sur son
trajet, une hypothèse de nombre de modèles de butin, une ancienne classe et
onze scénarios historiques de ville ou d’interface à examiner.

Leur présence sur la révision antérieure est établie. Cela ne prouve pas que
tous sont de simples tests périmés : les scénarios de trajet et d’interaction
doivent être confrontés à leurs règles avant de changer leurs attentes. Les
tests du retour au camp actuel, de ses services, de sa sauvegarde et des deux
ensembles réussissent dans la vérification ciblée finale.

## Contrôles finaux

Compilation debug et contrôle release réussis, sans avertissement. Formatage et
espaces du diff contrôlés. Le lancement natif isolé `--ui-cold-weapon-sets` produit
seize captures à 1920 × 1080 avec les deux thèmes : chaque dessin conserve
l’instantané moteur et les six instances d’inventaire, sans sauvegarde du joueur.
Les PNG courants de la revue sont réutilisés.

Les données brutes sont temporairement dans `artifacts/pre-map-stability-01/`.
Leurs empreintes, commandes, cas et résultats durables sont dans `triage.json`
et `final-focused-results.json`.
