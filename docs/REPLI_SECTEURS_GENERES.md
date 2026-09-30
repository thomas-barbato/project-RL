# Repli dans les secteurs générés

27 septembre 2026 · génération 116 inchangée.

Cette passe prolonge la [sortie à ressources finies](EXPEDITION_SURVIE.md)
avec le terrain réel des friches de surface : cartes de 128 × 80 cases,
populations ordinaires, rencontres, faune, sites, installations, butins,
éléments destructibles et sources de renfort. Les équipements portés des
humanoïdes passent aussi par le générateur habituel.

La surface est choisie explicitement pour isoler ce palier ; il ne s'agit pas
d'un tirage de toutes les couches de l'atlas. Les graines 0 à 7 sont toutes
conservées, sans déplacer les ennemis ni filtrer les rencontres difficiles.

## Protocole

Le personnage garde le kit fini précédent : couteau, fusil de patrouille à
12 projectiles, deux soins. Il entre par l'ouest depuis un petit refuge de
test, puis se dirige vers le passage est. Dès qu'un hostile devient visible :

- conduite immédiate : fait demi-tour ;
- conduite différée : poursuit son avancée trois tours avant de revenir.

Chaque carte est jouée une fois par conduite dans une exécution du test,
soit **16 replis**. Une mort arrête l'essai, sans résurrection ni nouveau tirage.
Le trajet et les ressources restent dans un même monde persistant. Les
ennemis peuvent se déplacer, poursuivre et attaquer pendant le retour.

Le pilote évite les cases d'attaque annoncées, ouvre les portes ordinaires,
utilise un soin à 12 PV ou moins et peut frapper au couteau un hostile adjacent
si le chemin est bloqué. Il ne tire pas au fusil et n'attaque pas les neutres.
Il connaît la géométrie de la carte ; ses décisions de contact utilisent la
visibilité réelle, pas la position d'une cible cachée.

Les limites de 600 actions ou de 12 attentes consécutives produisent un résultat
« pilote bloqué », jamais un faux retour réussi. Une commande refusée est
également rapportée. Aucun blocage de ce type n'a été observé dans cet échantillon.

## Résultats

| Conduite | Retours vivants | Morts | PV minimum observés | Soins utilisés |
|---|---:|---:|---:|---:|
| Repli immédiat | 8/8 | 0 | 20 dans chaque essai | 0 |
| Trois tours d'avance supplémentaires | 8/8 | 0 | 11–20 | 0–2 |

Les replis immédiats prennent 11–53 tours depuis le départ du refuge ; les
différés 17–61 tours. Les huit cartes produisent un contact réel sur ce trajet.
Cinq cartes mettent notamment en jeu un comportement mobile parmi chasseur,
chasseur en meute, escarmoucheur ou fouisseur. Le trajet différé de la graine 3
expose à deux hostiles mobiles, contre un au premier contact.

Dans les trajets différés, les minimums de PV sont respectivement
20, 18, 14, 20, 11, 20, 14 et 16 pour les graines 0 à 7. La graine 4 consomme
les deux soins. Les 12 projectiles restent disponibles, puisque ce protocole
mesure la fuite, pas le nettoyage du secteur au fusil.

Personne n'atteint le passage est : le demi-tour est volontairement déclenché
par une rencontre avant cela. Ce résultat ne doit pas être présenté comme
16 secteurs entièrement explorés ou remportés.

## Garde-fous permanents

- Pas de blessure ni de recharge gratuite au passage d'arrivée.
- Au retour au refuge, PV, munitions et soins identiques à leur état avant
  le passage : quitter le combat n'est pas un soin implicite.
- Sur les graines 1 et 4, copie restaurée d'un snapshot vivant pendant le repli :
  mêmes commandes, mêmes événements et même état complet à chaque tour jusqu'au
  retour, notamment après blessure et consommation des soins. Ce contrôle
  parallèle ne remplace jamais un mauvais résultat par celui d'un autre essai.
  Il contrôle la simulation, pas une fermeture/réouverture manuelle du client.
- Contrôle du pilote sur une porte fermée, une case annoncée et un civil
  bloquant un couloir : ouvrir légalement, éviter le danger, ne pas marcher
  sur le personnage.
- Tous les cas et leurs échecs sont imprimés ; les huit replis immédiats de
  référence doivent rester possibles et les blocages du pilote font échouer
  le contrôle, sans exiger l'immortalité dans toute rencontre future.

## Conclusion limitée

Le repli précoce fonctionne sur cet échantillon de terrain généré. Retarder la
décision peut consommer des PV et même l'ensemble des soins, malgré une issue
vivante ici. C'est une différence pertinente dans un jeu à vie unique.

Ce n'est pas une estimation du taux de survie des joueurs : la carte est connue
du pilote, le contact est précoce, huit graines ne couvrent pas les situations
d'encerclement et les autres couches ne sont pas mesurées. Aucun bonus de
survie, changement de population ou ajustement de dégâts/PV n'a été introduit.
Le contrôle ne remplace pas une session jouée avec découverte du terrain.

## Rejouer

```powershell
cargo test --locked --bin project-rl generated_survival -- --nocapture
```

Trois tests dédiés, uniquement dans le binaire de test. Aucun menu, contenu de
campagne ni fichier de partie du joueur n'est modifié. Le kit et les lectures
de réserves sont partagés avec les tests de sortie précédents pour éviter
de mesurer deux équipements de départ différents par accident.

Validation locale : trois nouveaux tests réussis, ainsi que les trois tests
`survival_expedition` précédents après extraction du kit commun. Contrôle de
tous les targets, formatage et `git diff --check` réussis. La suite globale
n'a pas été relancée ; aucun commit ni push effectué.
